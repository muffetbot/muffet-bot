# muffet-bot


#### Basic announcements
Adding announcements is now as simple as adding a new command to the config file.
URL's will automatically be handled and displayed by Discord.
<br>

*set the target value to the announcement you want, which can be plaintext:*
```toml
[[command]]
# command names should be a single word, although you can use underscores instead of spaces
name = "po_box" 
target = """
PO Box 000000
City, State 181818
"""
```

*given a full url the announcement will display whatever is there if it's safe*
```toml
[[command]]
name = "booyaka"
target = "https://duck.com"
```
<br>

*or use a path instead, the given path will be added to the main site url provided*
*obviously `site_url` has to be set for this to work*
```toml
site_url = "https://mysite.com"

[[command]]
name = "about"
path = "/about"

[[command]]
name = "contact"
path = "contact-us"
```

*the `!about` command will display `https://mysite.com/about`*
*the `!contact` command will display `https://mysite.com/contact-us`*
<br>

*the keys `help` and `response_type` are also available for commands*
```toml
[[command]]
name = "idk"
# change the highlight color for this command's help
color = "dark-gold"
target = "https://www.ranoutofideas.com"
# this message will be shown if a user types !help idk
help = "this is the help message for idk"
# possible values are `dm`, `dm owner`, `embed`, `reply`, and `channel`
# this determines the visibility of the bot's reply
# a `reply` setting will mention the user in the channel with the response
# a `dm owner` setting will only work for the server owner, sending them a dm
# an `embed` setting will be prettier and obey the command's color setting, but will not display link previews
response_type = "reply"
```

*setting both the target and path will result in only the target value being displayed*
<br>

**The bot will create a config for you on first run**

*Available colors*
```rust
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
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
```
<br>

##### Will not compile without custom impl of Debug for serenity's Context struct:
> TypeMap does not derive Debug trait
> Required by tracing crate