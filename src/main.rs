/// SEE https://docs.rs/serenity/0.9.2/serenity/ for documentation on Discord API for Rust
/// SEE https://github.com/serenity-rs/serenity/tree/current/examples for examples
mod admin;
mod mods;
mod utils;

use utils::discord::*;
pub(crate) use utils::prelude;

use crate::prelude::*;
use serenity::{framework::standard::StandardFramework, http::Http, model::user::User};
use std::{collections::HashSet, env, sync::Arc};

use once_cell::sync::Lazy;
static OWNER: Lazy<Mutex<User>> = Lazy::new(|| Mutex::default());

use utils::config::{get_conf, ConfigData};
static CONFIG: Lazy<Mutex<ConfigData>> = Lazy::new(|| Mutex::default());

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // try getting $MUFFETBOT_CONFIG env or trigger initial setup if not present
    let mut config_path = match env::var("MUFFETBOT_CONFIG") {
        Ok(path) => path,
        _ => match utils::config::init() {
            Ok(mb_env) => mb_env,
            Err(e) => {
                eprintln!("{}", e.as_ref());
                anyhow::bail!("init error")
            }
        },
    };

    if !std::path::Path::new(&config_path).exists() {
        eprintln!("$MUFFETBOT_CONFIG points to a non-existing path");
        config_path = match utils::config::init() {
            Ok(mb_env) => mb_env,
            Err(e) => {
                eprintln!("{}", e.as_ref());
                anyhow::bail!("init error")
            }
        }
    }

    // getting config data from file at $MUFFETBOT_CONFIG path
    let config = get_conf(&config_path).await?;
    let prefix = config.get_command_prefix();
    let token = config.get_token();

    // creating log file subscriber
    let file_appender = tracing_appender::rolling::daily(config.get_log_path(), "MBOT");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // initiating tracing logger
    tracing_subscriber::fmt().with_writer(non_blocking).init();

    // setting initial global config object
    let config_data = config.data().await;
    *CONFIG.lock().await = config_data;

    info!("Hydrated global config");

    // setting up connection with Discord server
    let http = Http::new_with_token(&token);
    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
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

    // initiating serenity framework
    let framework = StandardFramework::new()
        .configure(|c| {
            c.with_whitespace(false)
                .on_mention(Some(bot_id))
                .prefix(&prefix)
                .owners(owners)
        })
        .unrecognised_command(unknown_command)
        .help(&MUFFET_HELP)
        .group(&admin::ADMIN_GROUP)
        .group(&mods::commands::CUSTOMCOMMANDS_GROUP);

    // setting up client to subscribe to Discord events
    let mut client = serenity::client::Client::builder(&token)
        .framework(framework)
        .await
        .expect("Err creating client");
    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    // starting bot
    if let Err(e) = client.start().await {
        info!("Client error: {:#?}", e);
    } else {
        info!("Client lives!")
    }
    Ok(())
}
