use crate::prelude::*;
use crate::utils::config::{get_conf, hot_reload_conf};
use serenity::utils::{content_safe, ContentSafeOptions};

#[derive(Debug)]
enum CommandReloadAction {
    Append,
    Remove,
}

#[derive(Debug)]
enum AllowedReloads {
    Help,
    Commands(CommandReloadAction),
}

#[derive(Debug)]
enum HotReloadError {
    EnvMissing,
    FetchFailed,
    ImproperFormat,
    OperationFailed,
    PermissionDenied,
    WriteFailed,
}

impl std::fmt::Display for HotReloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use HotReloadError::*;
        let msg = match self {
            EnvMissing => "MUFFETBOT_CONFIG env var not found",
            FetchFailed => "Unable to find config file",
            ImproperFormat => "",
            OperationFailed => "Unable to complete operation",
            PermissionDenied => "",
            WriteFailed => "Unable to write new config to file",
        };

        write!(f, "{}", msg)
    }
}

async fn has_permissions(msg: &Message) -> bool {
    msg.author == *crate::OWNER.lock().await
}

async fn try_hot_reload(
    ctx: &Context,
    msg: &Message,
    discrim: AllowedReloads,
) -> anyhow::Result<String, HotReloadError> {
    use HotReloadError::*;
    if !has_permissions(msg).await {
        return Err(PermissionDenied);
    }

    let config_path = match std::env::var("MUFFETBOT_CONFIG") {
        Ok(env) => env,
        _ => return Err(EnvMissing),
    };

    let mut config = match get_conf(&config_path).await {
        Ok(conf) => conf,
        _ => return Err(FetchFailed),
    };

    async fn safeify<S: AsRef<str>>(context: &Context, msg: S) -> String {
        let opts = ContentSafeOptions::new().clean_everyone(false);
        content_safe(&context.cache, msg, &opts).await
    }

    let mut success_msg = String::new();
    let safe_content = safeify(ctx, &msg.content).await;
    match discrim {
        AllowedReloads::Help => {
            config.set_help(safe_content).await;
            success_msg += "Help message successfully changed!";
        }
        AllowedReloads::Commands(action) => {
            use CommandReloadAction::*;
            let mut split_msg = safe_content.trim_start().splitn(2, " ");

            let cmd = split_msg.next();
            if cmd.is_none() {
                return Err(ImproperFormat);
            }
            let cmd = cmd.unwrap();

            match action {
                Append => {
                    let target = split_msg.next();
                    if target.is_none() {
                        return Err(ImproperFormat);
                    }

                    if config.push_command(&cmd, &target.unwrap()).await.is_err() {
                        return Err(OperationFailed);
                    } else {
                        success_msg = format!("added the <{}> command!", cmd);
                    }
                }
                Remove => {
                    if config.pop_command(cmd).await.is_err() {
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

#[command]
async fn addcom(ctx: &Context, msg: &Message) -> CommandResult {
    let result = match try_hot_reload(
        ctx,
        msg,
        AllowedReloads::Commands(CommandReloadAction::Append),
    )
    .await
    {
        Ok(success_msg) => success_msg,
        Err(e) => {
            let description = e.to_string();
            error!("{}", &description);
            description
        }
    };

    announce(ctx, msg, result, &CommandResponse::Dm).await
}

#[command]
async fn rmcom(ctx: &Context, msg: &Message) -> CommandResult {
    let result = match try_hot_reload(
        ctx,
        msg,
        AllowedReloads::Commands(CommandReloadAction::Remove),
    )
    .await
    {
        Ok(success_msg) => success_msg,
        Err(e) => {
            let description = e.to_string();
            error!("{}", &description);
            description
        }
    };

    announce(ctx, msg, result, &CommandResponse::Dm).await
}

#[command]
async fn set_help(ctx: &Context, msg: &Message) -> CommandResult {
    let result = match try_hot_reload(ctx, msg, AllowedReloads::Help).await {
        Ok(success_msg) => success_msg,
        Err(e) => {
            let description = e.to_string();
            error!("{}", &description);
            description
        }
    };

    announce(ctx, msg, result, &CommandResponse::Dm).await
}
