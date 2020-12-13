use crate::prelude::*;
use crate::utils::config::{get_conf, hot_reload_conf};

#[derive(Debug)]
enum CommandReloadAction {
    Append,
    Remove,
}

#[derive(Debug)]
enum AllowedReloads {
    Help,
    Color,
    Commands(CommandReloadAction),
}

#[derive(Debug)]
enum HotReloadError {
    EnvMissing,
    FetchFailed,
    ImproperFormat,
    OperationFailed,
    WriteFailed,
}

impl std::fmt::Display for HotReloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use HotReloadError::*;
        let msg = match self {
            EnvMissing => "MUFFETBOT_CONFIG env var not found",
            FetchFailed => "Unable to find config file",
            ImproperFormat => {
                "Improper formatting for command. Use the `help` command for more info."
            }
            OperationFailed => "Unable to complete operation",
            WriteFailed => "Unable to write new config to file",
        };

        write!(f, "{}", msg)
    }
}

use strum::IntoEnumIterator;
async fn try_hot_reload(mut args: Args, discrim: AllowedReloads) -> Result<String, HotReloadError> {
    use HotReloadError::*;
    let config_path = match std::env::var("MUFFETBOT_CONFIG") {
        Ok(env) => env,
        _ => return Err(EnvMissing),
    };

    let mut config = match get_conf(&config_path).await {
        Ok(conf) => conf,
        _ => return Err(FetchFailed),
    };

    let mut success_msg = String::new();
    match discrim {
        AllowedReloads::Color => {
            use crate::utils::config::Color;

            if let Some(color) = args.remains() {
                let color = color.trim().to_lowercase();
                let mut valid = false;

                for colour in Color::iter() {
                    if colour.as_ref() == color {
                        config.set_color(colour).await;
                        success_msg += "Color successfully changed!";
                        valid = true;
                        break;
                    }
                }

                if !valid {
                    success_msg += "**Sorry, that's not a valid color!**\n";
                    for color in Color::iter() {
                        success_msg += &format!("> *{}*\n", color.as_ref());
                    }
                    return Ok(success_msg);
                }
            } else {
                for color in Color::iter() {
                    success_msg += &format!("> *{}*\n", color.as_ref());
                }
                return Ok(success_msg);
            }
        }
        AllowedReloads::Help => {
            success_msg += match args.remains() {
                Some(help) => {
                    config.set_help(help.to_string()).await;
                    "Help message successfully changed!"
                }
                None => return Err(ImproperFormat),
            };
        }
        AllowedReloads::Commands(action) => {
            let cmd = match args.single::<String>() {
                Ok(cmd) => cmd,
                _ => return Err(ImproperFormat),
            };

            match action {
                CommandReloadAction::Append => match args.remains() {
                    Some(target) => {
                        if config.push_command(&cmd, target).await.is_err() {
                            return Err(OperationFailed);
                        } else {
                            success_msg = format!("added the <{}> command!", cmd);
                        }
                    }
                    None => return Err(ImproperFormat),
                },
                CommandReloadAction::Remove => {
                    if config.pop_command(cmd.as_ref()).await.is_err() {
                        return Err(OperationFailed);
                    } else {
                        success_msg = format!("removed the <{}> command!", cmd);
                    }
                }
            }
        }
    }

    match hot_reload_conf(config_path, config).await {
        Ok(_) => Ok(success_msg),
        Err(_) => Err(WriteFailed),
    }
}

#[instrument]
#[command]
#[owners_only]
#[delimiters(" ")]
#[description = "add a command"]
#[usage = "`!addcom ig <command trigger> <command value>`"]
#[example = "`!addcom ig https://www.instagram.com/me`"]
async fn addcom(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let result =
        match try_hot_reload(args, AllowedReloads::Commands(CommandReloadAction::Append)).await {
            Ok(success_msg) => success_msg,
            Err(e) => {
                let description = e.to_string();
                info!("{}", &description);
                description
            }
        };

    announce(ctx, msg, result, &CommandResponse::DmOwner).await
}

#[instrument]
#[command]
#[owners_only]
#[description = "remove a command"]
#[usage = "`!rmcom <command trigger>`"]
#[example = "`!rmcom ig`"]
async fn rmcom(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let result =
        match try_hot_reload(args, AllowedReloads::Commands(CommandReloadAction::Remove)).await {
            Ok(success_msg) => success_msg,
            Err(e) => {
                let description = e.to_string();
                info!("{}", &description);
                description
            }
        };

    announce(ctx, msg, result, &CommandResponse::DmOwner).await
}

#[instrument]
#[command]
#[owners_only]
#[description = "change the message that displays before the help command"]
#[usage = "`!set_help <new help message>`"]
#[example = "`!set_help this is my new help message`"]
async fn set_help(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let result = match try_hot_reload(args, AllowedReloads::Help).await {
        Ok(success_msg) => success_msg,
        Err(e) => {
            let description = e.to_string();
            info!("{}", &description);
            description
        }
    };

    announce(ctx, msg, result, &CommandResponse::DmOwner).await
}

#[instrument]
#[command]
#[owners_only]
#[description = "change the highlight color for the bot's responses"]
#[usage = "`!color <new color>`"]
#[example = "`!color rohrkatze-blue`"]
async fn color(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let result = match try_hot_reload(args, AllowedReloads::Color).await {
        Ok(succes_msg) => succes_msg,
        Err(e) => {
            let description = e.to_string();
            info!("{}", &description);
            description
        }
    };

    announce(ctx, msg, result, &CommandResponse::DmOwner).await
}
