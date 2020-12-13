use anyhow::Result;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CommandResponse {
    Channel,
    Dm,
    Embed,
    #[serde(rename = "dm owner")]
    DmOwner,
    Reply,
}

impl Default for CommandResponse {
    fn default() -> Self {
        Self::Channel
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
struct Command {
    admin: Option<bool>,
    name: String,
    color: Option<Color>,
    help: Option<String>,
    #[serde(rename = "path")]
    url_path: Option<String>,
    response_type: Option<CommandResponse>,
    target: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "muffetbot")]
pub struct Config {
    help_color: Option<Color>,
    discord_token: String,
    log_path: String,
    help_message: Option<String>,
    command_prefix: Option<String>,
    site_url: Option<String>,
    #[serde(rename = "command")]
    commands: Option<Vec<Command>>,
}

#[derive(Debug)]
pub struct CommandData {
    admin: bool,
    color: Color,
    help: String,
    response_type: CommandResponse,
    trigger: String,
    value: String,
}

impl Default for CommandData {
    fn default() -> Self {
        Self {
            help: "No help available for this command".to_owned(),
            ..Default::default()
        }
    }
}

impl CommandData {
    pub fn restricted(&self) -> bool {
        self.admin
    }

    pub fn get_color(&self) -> Color {
        self.color.clone()
    }

    pub fn get_help(&self) -> &str {
        &self.help
    }

    pub fn get_response_type(&self) -> &CommandResponse {
        &self.response_type
    }

    pub fn get_trigger(&self) -> &str {
        &self.trigger
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }
}

#[derive(Debug, Default)]
pub struct ConfigData {
    pub commands: Vec<CommandData>,
    pub help_color: Color,
    pub help_message: String,
    pub site_url: String,
}

impl Config {
    /// Consumes Config which has private, optional members for serialization
    /// and returns ConfigData struct which has public, non-optional members.
    pub async fn data(self) -> ConfigData {
        ConfigData {
            help_color: {
                match self.help_color {
                    Some(color) => color,
                    None => Color::Kerbal,
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
                        let admin = cmd.admin.unwrap_or(false);
                        let help = cmd
                            .help
                            .unwrap_or("No help available for this command".to_string());
                        let response_type = cmd.response_type.unwrap_or_default();
                        let color = cmd.color.unwrap_or_default();
                        let value = if let Some(target) = cmd.target {
                            target
                        } else if let Some(mut url_path) = cmd.url_path {
                            if !url_path.starts_with("/") {
                                url_path = String::from("/") + url_path.as_ref();
                            }

                            let path = match &self.site_url {
                                Some(base_url) => base_url.clone() + url_path.as_ref(),
                                None => url_path,
                            };
                            path
                        } else {
                            continue;
                        };

                        commands.push(CommandData {
                            admin,
                            color,
                            response_type,
                            trigger: cmd.name.trim().to_lowercase(),
                            value,
                            help,
                        });
                    }
                }
                commands
            },
            site_url: {
                match self.site_url {
                    Some(url) => url,
                    None => String::new(),
                }
            },
        }
    }

    /// Getter for path to the logs directory.
    /// Cannot be hot reloaded.
    pub fn get_log_path(&self) -> &str {
        &self.log_path
    }

    /// Getter for alternate command prefix.
    /// Cannot be hot reloaded.
    pub fn get_command_prefix(&self) -> String {
        match &self.command_prefix {
            Some(prefix) => prefix.to_string(),
            None => String::from("!"),
        }
    }

    /// Getter for discord token (Panics if no token).
    /// Cannot be hot reloaded.
    pub fn get_token(&self) -> String {
        self.discord_token.clone()
    }

    /// Changes global help message -
    /// Suuports hot reload.assert_eq!
    pub async fn set_help(&mut self, new_message: String) {
        self.help_message = Some(new_message);
    }

    pub async fn set_color(&mut self, new_color: Color) {
        self.help_color = Some(new_color);
    }

    /// Attempts to append command to config -
    /// Supports hot reload.
    pub async fn push_command(
        &mut self,
        command_name: &str,
        command_target: &str,
    ) -> Result<(), ()> {
        if self.command_exists(command_name) {
            return Err(());
        }

        let mut cmds = match &self.commands {
            Some(c) => c.clone(),
            None => Vec::new(),
        };
        cmds.push(Command {
            admin: Some(false),
            color: Some(Color::default()),
            name: command_name.to_owned(),
            help: None,
            response_type: Some(CommandResponse::default()),
            target: Some(command_target.to_owned()),
            url_path: None,
        });
        self.commands = Some(cmds);

        Ok(())
    }

    fn command_exists(&self, command_name: &str) -> bool {
        if let Some(cmds) = &self.commands {
            for cmd in cmds {
                if cmd.name == command_name {
                    return true;
                }
            }
        }
        false
    }

    /// Attempts to remove existing command -
    /// Supports hot reaload.
    pub async fn pop_command(&mut self, command_name: &str) -> Result<(), ()> {
        if self.command_exists(command_name) {
            let mut cmds = self.commands.clone().unwrap();
            cmds.retain(|c| c.name != command_name);

            self.commands = Some(cmds);
            Ok(())
        } else {
            Err(())
        }
    }
}

use std::fs::read_to_string;
use std::path::{Path, PathBuf};
/// Attempts to fetch config asynchronously
pub async fn get_conf<P: AsRef<Path>>(config_path: P) -> Result<Config> {
    let conf_file = read_to_string(config_path)?;
    let conf_toml: Config = toml::from_str(&conf_file)?;

    Ok(conf_toml)
}

/// Attempts to write new config to file asynchronously
pub async fn hot_reload_conf<P: AsRef<Path>>(config_path: P, new_config: Config) -> Result<()> {
    std::fs::write(&config_path, toml::to_string(&new_config)?)?;

    let reloaded_data = new_config.data().await;
    *crate::CONFIG.lock().await = reloaded_data;
    Ok(())
}

/// Basic Q&A CLI
fn query_stdin(question: &str, sanitize: bool) -> Option<String> {
    println!("\n{}", question);
    let mut answer_buffer = String::new();
    if std::io::stdin().read_line(&mut answer_buffer).is_err() {
        println!("Sorry, something went wrong. Please answer again.");
        return query_stdin(question, sanitize);
    }

    if answer_buffer.trim() == "" {
        None
    } else {
        if sanitize {
            Some(answer_buffer.replace("\n", ""))
        } else {
            Some(answer_buffer)
        }
    }
}

// pub enum InitError {
//
// }

/// writes MUFFETBOT_CONFIG to ~/.profile
fn set_muffetbot_env<S>(config_path: S, unix_home: &PathBuf) -> Result<()>
where
    S: std::fmt::Display + AsRef<std::ffi::OsStr>,
{
    std::env::set_var("MUFFETBOT_CONFIG", &config_path);

    let mut unix_profile = unix_home.clone();
    unix_profile.push(".profile");
    if !unix_profile.exists() || !unix_profile.is_file() {
        std::fs::File::create(&unix_profile)?;
    }
    let profile_str = read_to_string(&unix_profile)?;

    let mut profile_lines = profile_str.split("\n").collect::<Vec<&str>>();

    profile_lines.retain(|ln| !ln.trim_start().starts_with("export MUFFETBOT_CONFIG"));
    let mut profile_string = profile_lines.join("\n");
    profile_string.push_str(&format!("\nexport MUFFETBOT_CONFIG=\"{}\"", config_path));

    std::fs::write(unix_profile, profile_string)
        .expect("Write failed. Please set $MUFFETBOT_CONFIG manually");
    Ok(())
}

use std::fs::create_dir_all;
/// Initial configuration walkthrough
pub fn init() -> Result<String> {
    let unix_home = home::home_dir().expect("Unable to locate system home.");
    let discord_token = query_stdin("Please enter your server discord token", true)
        .expect("Discord token is required!");

    let config_path = match query_stdin(
        "By default the muffetbot config will be stored in ~/.config/muffetbot.toml\n\
        If you want to choose a different path, please enter it now or leave this empty.",
        true,
    ) {
        Some(query) => PathBuf::from(query),
        None => {
            let mut config_path = unix_home.clone();
            config_path.push(".config/muffetbot.toml");
            config_path
        }
    };

    let site_url = query_stdin(
        "Do you have a website? Please enter its url or leave blank.",
        true,
    );
    let help_message = query_stdin(
        "Enter the description you want to show when the bot's !help command is triggered",
        false,
    );
    let log_path = match query_stdin(
        "By default the muffetbot logs will be stored in ~/.local/share/muffetbot-logs\n\
        If you want to choose a different path, please enter it now or leave this empty.",
        true,
    ) {
        Some(path) => PathBuf::from(path),
        None => {
            let mut log_path = unix_home.clone();
            log_path.push(".local/share/muffetbot-logs");
            log_path
        }
    };
    create_dir_all(&log_path)?;

    let prefix = match query_stdin(
        "Do you wish to override the command prefix?\n\
        Enter your preference or leave the default value `!`",
        true,
    ) {
        Some(pre) => pre,
        None => "!".to_owned(),
    };

    let new_config = Config {
        help_color: Some(Color::BlitzBlue),
        commands: None,
        command_prefix: Some(prefix),
        discord_token,
        help_message,
        log_path: log_path.to_string_lossy().to_string(),
        site_url,
    };

    let conf_path = Path::new(&config_path);
    if let Some(parent) = conf_path.parent() {
        if !parent.exists() {
            create_dir_all(parent)?;
        }
    }

    std::fs::write(&config_path, toml::to_string(&new_config)?)?;
    let config_path = config_path.to_string_lossy().to_string();
    set_muffetbot_env(&config_path, &unix_home)?;
    Ok(config_path)
}

use strum::*;

#[derive(AsRefStr, Clone, Debug, Deserialize, EnumIter, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab_case")]
pub enum Color {
    BlitzBlue = 0x6FC6E2,
    Blue = 0x3498DB,
    Blurple = 0x7289DA,
    DarkBlue = 0x206694,
    DarkGold = 0xC27C0E,
    DarkGreen = 0x1F8B4C,
    DarkGrey = 0x607D8B,
    DarkMagenta = 0xAD1457,
    DarkOrange = 0xA84300,
    DarkPurple = 0x71368A,
    DarkRed = 0x992D22,
    DarkTeal = 0x11806A,
    DarkerGrey = 0x546E7A,
    FabledPink = 0xFAB1ED,
    FadedPurple = 0x8882C4,
    Fooyoo = 0x11CA80,
    Gold = 0xF1C40F,
    Kerbal = 0xBADA55,
    LightGrey = 0x979C9F,
    LighterGrey = 0x95A5A6,
    Magenta = 0xE91E63,
    MeibePink = 0xE68397,
    Orange = 0xE67E22,
    Purple = 0x9B59B6,
    Red = 0xE74C3C,
    RohrkatzeBlue = 0x7596FF,
    Rosewater = 0xF6DBD8,
    Teal = 0x1ABC9C,
}

impl Default for Color {
    fn default() -> Self {
        Self::BlitzBlue
    }
}

use serenity::utils::Colour;

impl Into<Colour> for Color {
    fn into(self) -> Colour {
        Colour::new(self as u32)
    }
}
