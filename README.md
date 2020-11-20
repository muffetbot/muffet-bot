# muffet-bot


#### basic announcements
*simple_announcements.rs* in *src/commands* will announce basic text messages to the channel. To create a basic announcement:
> **In src/commands/simple_announcements.rs:**<br>
> Create a new function - the name of the function will be the command<br>
> *i.e.* `async fn new_name( ...`<br>
> Change the value of the announcement variable to your announcement<br>
> *i.e.* `let announcement = r#"new text"#;`<br>
> **In src/commands.rs:**<br>
> Add the name of the function to the list in the `#[commands(... , new_name)]` macro

<br>

#### social media links

*socials.rs* uses the **Links** enum from *utils/net.rs*. To add a new link:
> **In src/utils/net.rs:**<br>
> Add a new descriptive enum field in Links<br>
> Insert a new hash for the url inside the `static URLS` declaration<br>
> **In src/commands/socials.rs:**<br>
> Create a new function - the name of the function will be the command<br>
> *i.e.* `async fn new_name( ...`<br>
> Make sure the name of your new enum entry matches the enum in the function<br>
> *i.e.* `open_in_browser(NewName, ctx, msg).await`<br>
> **In src/commands.rs:**<br>
> Add the name of the function to the list in the `#[commands(... , new_name)]` macro

<br>

#### for both, make sure the `#[command]` macro is over the function