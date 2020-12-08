pub use crate::utils::scraper::SteelCutter;
pub use log::*;
pub use serenity::prelude::*;
use serenity::utils::{content_safe, ContentSafeOptions};
pub use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

/// announces given message to entire thread. `announcement` can be any type that implements as_ref for string slice
pub async fn announce<S>(ctx: &Context, msg: &Message, announcement: S) -> CommandResult
where
    S: AsRef<str>,
{
    let content = content_safe(&ctx.cache, announcement, &ContentSafeOptions::default()).await;

    if let Err(e) = msg.channel_id.say(&ctx.http, &content).await {
        eprintln!("Error sending message: {:#?}", e);
    }

    Ok(())
}

#[macro_export]
macro_rules! commandify {
    ($cmd_name:ident, $cmd_target:expr) => {{
        use paste::*;
        paste! {
            #[command]
            pub async fn [<get_ $cmd_name>](ctx: &Context, msg: &Message) -> CommandResult {
                let target = $cmd_target.to_string();
                announce(ctx, msg, target).await
            }
        }
    }};
}

// #[macro_export]
// macro_rules! groupify {
//     ($grp_name:ident, $cmd_name:ident, $cmd_target:expr) => {{
//         use paste::*;
//         use serenity::framework::standard::macros::group;

//         paste! {
//             #[group]
//             #[commands(commandify!([<get_$cmd_name>]))]
//             struct $grp_name;
//         }

//         $grp_name
//     }};
// }
