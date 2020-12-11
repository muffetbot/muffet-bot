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
target = "https://www.ranoutofideas.com"
# this message will be shown if a user types !help idk
help = "this is the help message for idk"
# possible values are `dm`, `reply`, and `channel`
# this determines the visibility of the bot's reply
# a `reply` setting will mention the user in the channel with the response
response_type = "reply"
```

*setting both the target and path will result in only the target value being displayed*
<br>

**The bot will create a config for you on first run**