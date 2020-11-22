use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::channel::Message;
use serenity::model::id::ChannelId;

// use std::env;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;
use tokio::sync::Mutex;
pub mod hanabi;

lazy_static! {
    static ref TABLES: Mutex<HashMap<usize, hanabi::Hanabi>> = Mutex::new(HashMap::new());
    static ref CHAN_TO_TABLEID: Mutex<HashMap<ChannelId, usize>> = Mutex::new(HashMap::new());
    static ref ID: Mutex<usize> = Mutex::new(0);
}

#[group]
#[commands(ping, play, deal, start)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from a file
    let token = fs::read_to_string("DISCORD_TOKEN").expect("Error when reading the token");
    let my_global = 5;

    let mut client = Client::new(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;
    Ok(())
}

#[command]
async fn play(ctx: &Context, msg: &Message) -> CommandResult {
    let game = hanabi::Hanabi::new(ctx, msg).await;
    let mut id = ID.lock().await;

    for p in &game.players {
        CHAN_TO_TABLEID.lock().await.insert(p.channel.unwrap(), *id);
    }

    TABLES.lock().await.insert(*id, game);
    *id += 1;

    Ok(())
}

#[command]
async fn deal(ctx: &Context, msg: &Message) -> CommandResult {
    if let Some(id) = CHAN_TO_TABLEID.lock().await.get(&msg.channel_id) {
        if let Some(game) = TABLES.lock().await.get_mut(id) {
            println!("Dealing...");
            game.deal(ctx).await.expect("Could not deal");
        } else {
            msg.channel_id
                .say(
                    &ctx,
                    "Tu es pas dans un channel de jeu. Tu peux pas faire ça",
                )
                .await
                .expect("Could not send error message");
        }
    };

    Ok(())
}

#[command]
async fn start(ctx: &Context, msg: &Message) -> CommandResult {
    let mut game = hanabi::Hanabi::new(ctx, msg).await;
    Ok(())
}
