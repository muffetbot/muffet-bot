use crate::prelude::*;

/*
* template function:
*
*   #[command]
*   async fn func_name(ctx: &Context, msg: &Message) -> CommandResult {
*       let announcement = r#"new message"#;   // raw string (r#""#) makes it easier to retain proper formatting
*       announce(ctx, msg, announcement).await
*   }
*/

#[command]
async fn pobox(ctx: &Context, msg: &Message) -> CommandResult {
    let announcement = r#"
PO Box 000000
City, State 181818"#;
    announce(ctx, msg, announcement).await
}
