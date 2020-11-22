use crate::hanabi::hand;
use serenity::client::Context;
use serenity::model::channel::{ChannelType, PermissionOverwrite, PermissionOverwriteType};
use serenity::model::id::{ChannelId, GuildId, RoleId, UserId};
use serenity::model::permissions::Permissions;
use serenity::model::user::User;
use serenity::Result;

pub struct Player {
    pub hand: hand::Hand,
    pub name: String,
    pub channel: Option<ChannelId>,
    pub user: UserId,
}

impl Player {
    pub fn new(user: &User) -> Player {
        Player {
            hand: hand::Hand::new(),
            name: user.name.clone(),
            channel: None,
            user: user.id,
        }
    }

    // Creates a new text channel that will be used to communicate with the player
    pub async fn init_channel(&mut self, ctx: &Context, guild: GuildId) -> Result<ChannelId> {
        let mut allow = Permissions::empty();
        allow.insert(Permissions::READ_MESSAGES);
        allow.insert(Permissions::SEND_MESSAGES);

        let everyone_id = RoleId::from(*guild.as_u64());
        let perms = vec![
            PermissionOverwrite {
                allow,
                deny: Permissions::all(),
                kind: PermissionOverwriteType::Member(self.user),
            },
            PermissionOverwrite {
                allow: Permissions::empty(),
                deny: Permissions::all(),
                kind: PermissionOverwriteType::Role(everyone_id),
            },
        ];

        let ch = guild
            .create_channel(&ctx, |c| {
                c.name(&self.name)
                    .kind(ChannelType::Text)
                    .category(779078429881532437)
                    .permissions(perms)
            })
            .await;

        match ch {
            Err(e) => Err(e),
            Ok(channel) => {
                self.channel = Some(channel.id);
                Ok(channel.id)
            }
        }
    }

    pub fn format_hand(&self) -> String {
        let mut s: String = String::new();
        for card in &self.hand.cards {
            s.push_str(card.color.to_string());
        }
        s.push_str("\n");
        for card in &self.hand.cards {
            s.push_str(card.value_to_string());
        }
        s
    }
}
