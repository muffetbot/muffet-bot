pub mod prelude;

mod minecraft_server;
use minecraft_server::*;

mod steel_cut_kawaii;
use steel_cut_kawaii::*;

mod simple_announcements;
use simple_announcements::*;

use serenity::framework::standard::macros::group;

#[group]
#[commands(about, donate, goal, goals, minecraft, pobox, shop, stream)]
struct MuffetBot;

mod socials;
use socials::*;

/// any new socials functions should preferrably go here, though they will still work if added to MuffetBot
#[group]
#[commands(email, patreon, twitter, venmo, youtube)]
struct Socials;
