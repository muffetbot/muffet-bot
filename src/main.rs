/// SEE https://docs.rs/serenity/0.8.7/serenity/ for documentation on Discord API for Rust
/// SEE https://github.com/serenity-rs/serenity/tree/current/examples for examples
mod commands;
mod utils;

pub(crate) use commands::prelude;
use utils::discord::*;
pub(crate) use utils::scraper::Links;

use crate::prelude::*;
use serenity::{framework::standard::StandardFramework, http::Http, model::user::User};
use std::{collections::HashSet, env, sync::Arc};

use once_cell::sync::Lazy;
static OWNER: Lazy<Mutex<User>> = Lazy::new(|| Mutex::default());

use utils::config::ConfigData;
static CONFIG: Lazy<Mutex<ConfigData>> = Lazy::new(|| Mutex::default());

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config_path = match env::var("MUFFETBOT_CONFIG") {
        Ok(path) => path,
        _ => {
            utils::config::init()?;
            env::var("MUFFETBOT_CONFIG")
                .expect("Please set MUFFETBOT_CONFIG env. Unable to find config file.")
        }
    };

    use utils::{config::get_conf, logger::crate_logger};
    let config = get_conf(&config_path).await?;
    let prefix = config.get_command_prefix();
    let token = config.get_token();
    let _logger = crate_logger(config.get_log_path()).expect("unable to initiate logger");
    let config_data = config.data().await;

    *CONFIG.lock().await = config_data;

    let http = Http::new_with_token(&token);
    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(mut info) => {
            info.name = (&CONFIG).lock().await.bot_alias.clone();
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            *OWNER.lock().await = info.owner.clone();
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(e) => panic!("Could not access the bot id: {:#?}", e),
            }
        }
        Err(e) => panic!("Could not access application info: {:#?}", e),
    };
    let framework = StandardFramework::new()
        .configure(|c| {
            c.with_whitespace(false)
                .on_mention(Some(bot_id))
                .prefix(prefix)
                .delimiters(vec![", ", ","])
                .owners(owners)
        })
        .unrecognised_command(unknown_command)
        .help(&MUFFET_HELP)
        .group(&commands::ADMIN_GROUP)
        .group(&commands::MUFFETBOT_GROUP);
    let mut client = serenity::client::Client::builder(&token)
        .framework(framework)
        .await
        .expect("Err creating client");
    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }
    if let Err(e) = client.start().await {
        error!("Client error: {:#?}", e);
    }

    Ok(())
}
