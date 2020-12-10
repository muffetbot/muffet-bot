use crate::prelude::*;
use crate::utils::config::{get_conf, hot_reload_conf};
use serenity::utils::{content_safe, ContentSafeOptions};

async fn has_permissions(msg: &Message) -> bool {
    let owner = &crate::OWNER.lock().await.clone();
    &msg.author == owner
}

#[command]
async fn addcom(ctx: &Context, msg: &Message) -> CommandResult {
    if has_permissions(msg).await {
        let config_path = std::env::var("MUFFETBOT_CONFIG")?;
        let mut config = get_conf(&config_path).await?;

        let new_command = msg.content.split_whitespace().collect::<Vec<&str>>();
        if new_command.len() > 2 {
            return announce(ctx, msg, "usage: !addcom <command name> <command target>").await;
        }

        let (cmd, target) = (
            content_safe(&ctx.cache, &new_command[0], &ContentSafeOptions::default()).await,
            content_safe(&ctx.cache, &new_command[1], &ContentSafeOptions::default()).await,
        );
        if let Err(e) = config.push_command(&cmd, &target).await {
            return announce(ctx, msg, e.to_string()).await;
        }
        hot_reload_conf(config_path, config).await?;
        announce(ctx, msg, format!("added the <{}> command!", cmd)).await
    } else {
        announce(ctx, msg, "You do not have permission to use this command").await
    }
}

#[command]
async fn rmcom(ctx: &Context, msg: &Message) -> CommandResult {
    if has_permissions(msg).await {
        let config_path = std::env::var("MUFFETBOT_CONFIG")?;
        let mut config = get_conf(&config_path).await?;

        let to_rm = msg.content.split_whitespace().collect::<Vec<&str>>();
        if to_rm.len() > 1 {
            return announce(ctx, msg, "usage: !rmcom <command name>").await;
        }

        let target = &to_rm[0];
        if let Err(e) = config.pop_command(target).await {
            return announce(ctx, msg, e.to_string()).await;
        }
        hot_reload_conf(config_path, config).await?;
        announce(ctx, msg, "command deleted!").await
    } else {
        announce(ctx, msg, "You do not have permission to use this command").await
    }
}

#[command]
async fn set_help(ctx: &Context, msg: &Message) -> CommandResult {
    if has_permissions(msg).await {
        let config_path = std::env::var("MUFFETBOT_CONFIG")?;
        let mut config = get_conf(&config_path).await?;

        let new_help = content_safe(&ctx.cache, &msg.content, &ContentSafeOptions::default()).await;
        config.set_help(new_help).await;

        hot_reload_conf(config_path, config).await?;
        announce(ctx, msg, "Help message changed!").await
    } else {
        announce(ctx, msg, "You do not have permission to use this command").await
    }
}
