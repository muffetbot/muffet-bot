use anyhow::Result;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
struct Command {
    name: String,
    #[serde(rename = "path")]
    url_path: Option<String>,
    target: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "muffetbot")]
pub struct Config {
    bot_alias: Option<String>,
    discord_token: String,
    log_path: String,
    help_message: Option<String>,
    site_url: Option<String>,
    #[serde(rename = "command")]
    commands: Option<Vec<Command>>,
}

#[derive(Default)]
pub struct ConfigData {
    pub bot_alias: String,
    pub commands: Vec<(String, String)>,
    pub help_message: String,
}

impl Config {
    pub async fn data(self) -> ConfigData {
        ConfigData {
            bot_alias: {
                match self.bot_alias {
                    Some(alias) => alias,
                    None => String::from("muffetbot"),
                }
            },
            help_message: {
                match self.help_message {
                    Some(help) => help,
                    None => String::new(),
                }
            },
            commands: {
                let mut commands = Vec::new();
                if let Some(conf_commands) = self.commands {
                    for cmd in conf_commands {
                        if let Some(target) = cmd.target {
                            commands.push((cmd.name.trim().to_lowercase(), target));
                        } else if let Some(mut url_path) = cmd.url_path {
                            if !url_path.starts_with("/") {
                                url_path = String::from("/") + url_path.as_ref();
                            }

                            let path = match &self.site_url {
                                Some(base_url) => base_url.clone() + url_path.as_ref(),
                                None => url_path,
                            };
                            commands.push((cmd.name.trim().to_lowercase(), path));
                        }
                    }
                }
                commands
            },
        }
    }

    /// Getter for path to the logs directory.
    /// Changing the log_path requires bot restart, no hot-reload at runtime.
    pub fn get_log_path(&self) -> String {
        self.log_path.clone()
    }

    /// Getter for discord token (Panics if no token).
    /// Changing the log_path requires bot restart, no hot-reload at runtime.
    pub fn get_token(&self) -> String {
        self.discord_token.clone()
    }

    pub async fn set_help(&mut self, new_message: String) {
        self.help_message = Some(new_message);
    }

    pub async fn push_command(&mut self, command_name: &str, command_target: &str) -> Result<()> {
        let new_command = Command {
            name: command_name.to_owned(),
            target: Some(command_target.to_owned()),
            url_path: None,
        };

        match &self.commands {
            Some(mut cmds) => {
                for cmd in cmds {
                    if cmd.name == new_command.name {
                        anyhow::bail!("{} already exists!", cmd.name)
                    }
                }
                cmds.push(new_command);
            }
            None => self.commands = Some(vec![new_command]),
        }

        Ok(())
    }

    fn command_exists(&self, command_name: &str) -> bool {
        if let Some(cmds) = self.commands {
            for cmd in cmds {
                if cmd.name == command_name {
                    return true;
                }
            }
        }
        false
    }

    pub async fn pop_command(&mut self, command_name: &str) -> Result<()> {
        if self.command_exists(command_name) {
            let mut cmds = self.commands.clone().unwrap();
            cmds.retain(|c| c.name != command_name);

            self.commands = Some(cmds);
            Ok(())
        } else {
            anyhow::bail!("Command does not exist!")
        }
    }
}

use std::fs::read_to_string;
use std::path::Path;
pub async fn get_conf<P: AsRef<Path>>(config_path: P) -> Result<Config> {
    let conf_file = read_to_string(config_path)?;
    let conf_toml: Config = toml::from_str(&conf_file)?;

    Ok(conf_toml)
}

pub async fn hot_reload_conf<P: AsRef<Path>>(config_path: P, new_config: Config) -> Result<()> {
    std::fs::write(&config_path, toml::to_string(&new_config)?)?;

    let reloaded_data = new_config.data().await;
    *crate::CONFIG.lock().await = reloaded_data;
    Ok(())
}

fn query_stdin(question: &str) -> Option<String> {
    println!("{}", question);
    let mut answer_buffer = String::new();
    if std::io::stdin().read_line(&mut answer_buffer).is_err() {
        println!("Sorry, something went wrong. Please answer again.");
        return query_stdin(question);
    }
    if answer_buffer.trim() == "" {
        None
    } else {
        Some(answer_buffer)
    }
}

fn set_muffetbot_env<S: AsRef<str>>(config_path: S) -> Result<()> {
    let unix_profile = std::fs::canonicalize("~/.profile")?;
    let profile_str = read_to_string(&unix_profile)?;

    let mut profile_lines = profile_str.split("\n").collect::<Vec<&str>>();

    profile_lines.retain(|ln| !ln.trim_start().starts_with("export MUFFETBOT_CONFIG"));
    let mut profile_string = profile_lines.join("\n");
    profile_string.push_str(&format!(
        "\nexport MUFFETBOT_CONFIG=\"{}\"",
        config_path.as_ref()
    ));

    std::fs::write(unix_profile, profile_string)?;
    Ok(())
}

use std::fs::create_dir_all;

pub fn init() -> Result<()> {
    let bot_alias = query_stdin(
        "Do you want to rename muffetbot?\
        Enter an alias for muffetbot or leave this empty.",
    );

    let discord_token =
        query_stdin("Please enter your server discord token").expect("Discord token is required!");

    let config_path = match query_stdin(
        "By default the muffetbot config will be stored in ~/.config/muffetbot.toml\
        If you want to choose a different path, please enter it now or leave this empty.",
    ) {
        Some(query) => query,
        None => "~/.config/muffetbot.toml".to_owned(),
    };

    let site_url = query_stdin("Do you have a website? Please enter its url or leave blank.");
    let help_message = query_stdin(
        "Enter the description you want to show when the bot's !help command is triggered",
    );
    let log_path = match query_stdin(
        "By default the muffetbot logs will be stored in ~/.local/share/muffetbot-logs\
        If you want to choose a different path, please enter it now or leave this empty.",
    ) {
        Some(path) => path,
        None => "~/.local/share/muffetbot-logs".to_owned(),
    };
    create_dir_all(&log_path)?;

    let new_config = Config {
        bot_alias,
        commands: None,
        discord_token,
        help_message,
        log_path,
        site_url,
    };

    let conf_path = std::path::Path::new(&config_path);
    if let Some(parent) = conf_path.parent() {
        if !parent.exists() {
            create_dir_all(parent)?;
        }
    }

    std::fs::write(&config_path, toml::to_string(&new_config)?)?;
    set_muffetbot_env(config_path)?;
    Ok(())
}
