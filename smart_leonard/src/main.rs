#[macro_use]
extern crate maplit;

mod commands;

use anyhow::Result;
use commands::{general::*, smite::*};
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::macros::help;
use serenity::framework::standard::{
    help_commands, macros::group, Args, CommandGroup, CommandResult, HelpOptions, StandardFramework,
};
use serenity::model::{channel::Message, gateway::Ready, id::UserId};
use std::collections::HashSet;

use std::env;

#[group]
#[commands(ping)]
struct General;

#[group]
#[prefix = "smite"]
#[commands(counter)]
struct Smite;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[help]
#[individual_command_tip = "Hello! Here is the list of things I can do for ya! :)\n"]
#[strikethrough_commands_tip_in_dm = ""]
#[strikethrough_commands_tip_in_guild = ""]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .help(&MY_HELP)
        .group(&GENERAL_GROUP)
        .group(&SMITE_GROUP);

    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    client.start().await?;
    Ok(())
}
