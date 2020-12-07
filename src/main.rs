/// SEE https://docs.rs/serenity/0.8.7/serenity/ for documentation on Discord API for Rust
/// SEE https://github.com/serenity-rs/serenity/tree/current/examples for examples
mod commands;
mod utils;

pub(crate) use commands::prelude;
use utils::boiler::*;
pub(crate) use utils::net::Links;

use serenity::prelude::*;
use serenity::{framework::standard::StandardFramework, http::Http, model::user::User};
use std::{collections::HashSet, env, sync::Arc};

use once_cell::sync::Lazy;

static OWNER: Lazy<Mutex<User>> = Lazy::new(|| Mutex::default());

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("unable to fetch token from env");
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
    let framework = StandardFramework::new()
        .configure(|c| {
            c.with_whitespace(true)
                .on_mention(Some(bot_id))
                .prefix("!")
                .delimiters(vec![", ", ","])
                .owners(owners)
        })
        .unrecognised_command(unknown_command)
        .help(&MUFFET_HELP)
        .group(&commands::MUFFETBOT_GROUP)
        .group(&commands::SOCIALS_GROUP);
    let mut client = serenity::client::Client::builder(&token)
        .event_handler(Handler::new())
        .framework(framework)
        .await
        .expect("Err creating client");
    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }
    if let Err(e) = client.start().await {
        println!("Client error: {:#?}", e);
    }
}
