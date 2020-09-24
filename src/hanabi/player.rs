use crate::hanabi::hand;
use serenity::model::user::User;

pub struct Player{
    hand: hand::Hand,
    user: User,
}

impl Player{
    pub fn new(user: User) -> Player{
        Player{
            hand : hand::Hand::new(),
            user,
        }
    }
}
