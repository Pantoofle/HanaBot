use std::sync::Arc;
use crate::hanabi::hand;
use serenity::model::id::{ChannelId, UserId, GuildId};
use serenity::model::user::User;
use serenity::model::channel::{PermissionOverwrite, PermissionOverwriteType, ChannelType};
use serenity::model::permissions::Permissions;
use serenity::http::Http;
use serenity::Result;

pub struct Player{
    hand: hand::Hand,
    name: String,
    channel: Option<ChannelId>,
    user: UserId,
}

impl Player{
    pub fn new(user : &User) -> Player{
        Player{
            hand : hand::Hand::new(),
            name : user.name.clone(),
            channel : None,
            user : user.id,
        }
    }

    pub async fn init_channel(&mut self, http : Arc<Http>, guild : GuildId) -> Result<ChannelId>{
        let mut allow = Permissions::empty();
        allow.insert(Permissions::READ_MESSAGES);
        allow.insert(Permissions::SEND_MESSAGES);

        let deny = Permissions::empty();

        let perms = vec![PermissionOverwrite{
            allow,
            deny,
            kind : PermissionOverwriteType::Member(self.user),
        }];

        let ch = guild.create_channel(http, |c| c.name(&self.name).kind(ChannelType::Text).permissions(perms)).await;
        match ch{
            Err(e) => Err(e),
            Ok(channel) => {
                self.channel = Some(channel.id); 
                Ok(channel.id)
            }
        }
    }
}
