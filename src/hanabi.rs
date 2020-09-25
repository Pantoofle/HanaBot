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
    pub fn new() -> Hanabi<'static>{
        Hanabi{
            playground : playground::Playground::new(),
            players : vec![],
            status : Status::Building,
        }
    }

    pub fn add_player(&mut self, player : player::Player){
        self.players.push(player);
    }
}
