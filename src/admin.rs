mod commands;
use commands::*;

use serenity::framework::standard::macros::group;

#[group]
#[owners_only]
#[help_available(false)]
#[commands(addcom, color, rmcom, set_help)]
struct Admin;
