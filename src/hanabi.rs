use serenity::model::{channel::Message, user::User};
use serenity::client::Context;
use std::sync::Arc;
use serenity::http::Http;

pub mod player;
pub mod playground;
pub mod card;
pub mod hand;

pub enum Status<'a>{
    Building,
    Turn(&'a player::Player),
    Ended,
}

pub struct Hanabi<'a>{
    playground : playground::Playground,
    players : Vec<player::Player>,
    status : Status<'a>,
}

impl Hanabi<'_>{
    pub async fn new(ctx : &Context, msg : &Message) -> Hanabi<'static>{
        let mut game = Hanabi{
            playground : playground::Playground::new(),
            players : vec![],
            status : Status::Building,
        };

        for user in &msg.mentions{
            let mut p = player::Player::new(&user);
            println!("Creating player channel");
            p.init_channel(ctx.http.clone(), msg.guild_id.expect("Message not from a guild"))
                .await
                .expect("Error when creating the channel");
            game.add_player(p);
        }
        game.print_setup_message(ctx.http.clone()).await;
        game
    }

    pub fn add_player(&mut self, player : player::Player){
        self.players.push(player);
    }

    pub async fn print_setup_message(&self, http : Arc<Http>){
        for p in &self.players{
            if let Some(chan_id) = p.channel{
                chan_id.say(http.clone(), "Bienvenue dans ce chan de Hanabi !")
                    .await
                    .expect("Could not send the welcome message");
            }
        }
    }
}
