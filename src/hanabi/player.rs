use crate::hanabi::hand;
use serenity::model::id::{ChannelId, UserId};
use serenity::model::user::User;

pub struct Player{
    hand: hand::Hand,
    name: String,
    channel: ChannelId,
    user: UserId,
}

impl Player{
    pub fn new(user : &User, chan : ChannelId) -> Player{
        Player{
            hand : hand::Hand::new(),
            name : user.name.clone(),
            channel : chan,
            user : user.id,
        }
    }
}
