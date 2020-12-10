pub mod prelude;

mod admin;
use admin::*;

mod minecraft_server;
use minecraft_server::*;

mod steel_cut_kawaii;
use steel_cut_kawaii::*;

use serenity::framework::standard::macros::group;

#[group]
#[commands(addcom)]
struct Admin;

#[group]
#[commands(about, donate, goal, goals, minecraft, shop, stream)]
struct MuffetBot;

mod socials;
use socials::*;

/// any new socials functions should preferrably go here, though they will still work if added to MuffetBot
#[group]
#[commands(email, patreon, twitter, venmo, youtube)]
struct Socials;
