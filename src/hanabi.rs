pub mod player;
pub mod playground;
pub mod card;
pub mod hand;

use serenity::model::user::User;

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
    pub fn new() -> Hanabi<'static>{
        Hanabi{
            playground : playground::Playground::new(),
            players : vec![],
            status : Status::Building,
        }
    }

    pub fn add_players(&mut self, users : Vec<User>){
        for u in users{
            self.players.push(player::Player::new(u));
        }
    }
}
