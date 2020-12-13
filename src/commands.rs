pub mod prelude;

mod admin;
use admin::*;

mod builtin;
use builtin::*;

use serenity::framework::standard::macros::group;

#[group]
#[owners_only]
#[help_available(false)]
#[commands(addcom, color, rmcom, set_help)]
struct Admin;

#[group]
#[commands(about, goal, goals, minecraft, shop)]
struct Kawaii;
