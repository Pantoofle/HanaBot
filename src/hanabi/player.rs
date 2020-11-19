use std::sync::Arc;
use crate::hanabi::hand;
use serenity::model::id::{ChannelId, UserId, GuildId};
use serenity::model::user::User;
use serenity::model::channel::{PermissionOverwrite, PermissionOverwriteType, ChannelType};
use serenity::model::permissions::Permissions;
use serenity::http::Http;
use serenity::Result;

pub struct Player{
    pub hand: hand::Hand,
    pub name: String,
    pub channel: Option<ChannelId>,
    pub user: UserId,
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

    // Creates a new text channel that will be used to communicate with the player
    pub async fn init_channel(&mut self, http : Arc<Http>, guild : GuildId) -> Result<ChannelId>{
        let mut allow = Permissions::empty();
        allow.insert(Permissions::READ_MESSAGES);
        allow.insert(Permissions::SEND_MESSAGES);

        let perms = vec![PermissionOverwrite{
            allow,
            deny : Permissions::empty(),
            kind : PermissionOverwriteType::Member(self.user),
        }];

        let ch = guild.create_channel(http, |c| c
            .name(&self.name)
            .kind(ChannelType::Text)
            .category(779078429881532437)
            .permissions(perms))
            .await;
                      
        match ch{
            Err(e) => Err(e),
            Ok(channel) => {
                self.channel = Some(channel.id); 
                Ok(channel.id)
            }
        }
    }
}
