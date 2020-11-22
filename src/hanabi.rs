use serenity::client::Context;
use serenity::futures::StreamExt;
use serenity::model::{channel::Message, id::UserId};
use std::collections::HashMap;
use std::result::Result;

pub mod card;
pub mod hand;
pub mod player;
pub mod playground;

type CommandResult<T> = Result<(), T>;

pub enum Status {
    Building,
    Starting,
    Turn(usize),
    Ended,
}

pub struct Settings {
    nb_players: usize,
    with_multicolor: bool,
    with_black: bool,
    hand_size: usize,
}

impl Settings {
    pub fn default() -> Settings {
        Settings {
            nb_players: 0,
            with_multicolor: false,
            with_black: false,
            hand_size: 5,
        }
    }
}

pub struct Hanabi {
    playground: playground::Playground,
    pub players: Vec<player::Player>,
    status: Status,
    settings: Settings,
    userid_to_playerid: HashMap<UserId, usize>,
}

impl Hanabi {
    pub async fn new(ctx: &Context, msg: &Message) -> Hanabi {
        let mut game = Hanabi {
            playground: playground::Playground::new(false, false),
            players: vec![],
            status: Status::Building,
            settings: Settings::default(),
            userid_to_playerid: HashMap::new(),
        };

        // Parse the start message to get the list of players
        for user in &msg.mentions {
            let mut p = player::Player::new(&user);
            println!("Creating player channel");
            p.init_channel(&ctx, msg.guild_id.expect("Message not from a guild"))
                .await
                .expect("Error when creating the channel");
            game.add_player(p, user.id);
        }

        // Display the actual settings
        game.print_setup_message(ctx).await;
        game
    }

    pub async fn deal(&mut self, ctx: &Context) -> CommandResult<&str> {
        // When the settings are set, call deal to deal the hands
        if let Status::Building = self.status {
            self.status = Status::Starting;
            // Dealing the cards
            println!("Drawing the cards");
            for player in self.players.iter_mut() {
                for _ in 1..=self.settings.hand_size {
                    player.hand.add_card(self.playground.draw().unwrap())
                }
            }

            // Print the hands
            println!("Printing the hands");
            for p_id in 0..=self.settings.nb_players - 1 {
                self.print_player_pov(ctx, p_id).await;
            }

            Ok(())
        } else {
            Err("Not ready to deal yet")
        }
    }

    pub fn start(&mut self, msg: &Message) -> CommandResult<&str> {
        if let Status::Starting = self.status {
            // Start the game, the player that called !start starts
            if let Some(id) = self.userid_to_playerid.get(&msg.author.id) {
                self.status = Status::Turn(id.to_owned());
                Ok(())
            } else {
                Err("User not in my list of players")
            }
        } else {
            Err("We are not ready to start yet")
        }
    }

    pub fn add_player(&mut self, player: player::Player, user_id: UserId) {
        self.userid_to_playerid
            .insert(user_id, self.settings.nb_players);
        self.players.push(player);
        self.settings.nb_players += 1;
    }

    pub async fn print_setup_message(&self, ctx: &Context) {
        // Send the setting message for each player in the game
        for p in &self.players {
            if let Some(chan_id) = p.channel {
                chan_id
                    .say(&ctx, self.format_settings_message())
                    .await
                    .expect("Could not send the settings message");

                chan_id
                    .say(&ctx, self.format_player_order())
                    .await
                    .expect("Could not send player list");
            }
        }
    }

    fn format_settings_message(&self) -> String {
        format!(
            "Paramètres du serveur :
         - Nombre de joueurs : {}
         - Cartes multicolores : {}
         - Cartes noires : {}",
            self.settings.nb_players, self.settings.with_multicolor, self.settings.with_black
        )
    }

    fn format_player_order(&self) -> String {
        let mut s: String = "Liste des joueurs : ".to_owned();
        for p in &self.players {
            s.push_str(&p.name);
            s.push_str(", ");
        }
        s
    }

    pub async fn print_player_pov(&self, ctx: &Context, p_id: usize) {
        let mut hands: Vec<(String, String)> = vec![];
        let mut id = p_id.clone();
        // Iterate over other players to generate the hands
        loop {
            id = (id + 1) % self.settings.nb_players;
            if id == p_id {
                break;
            }

            let player = self.players.get(id).expect("Error finding player");
            hands.push((player.name.to_owned(), player.format_hand()));
        }

        // Print the hand
        let player = self.players.get(p_id).expect("Error finding player");
        for (name, hand) in hands {
            if let Some(channel) = player.channel {
                channel
                    .say(&ctx, name)
                    .await
                    .expect("Could not send message");
                channel
                    .say(&ctx, hand)
                    .await
                    .expect("Could not send message");
            }
        }
    }

    pub async fn clear_channels(&self, ctx: &Context) {
        for p in &self.players {
            let mut messages = p.channel.unwrap().messages_iter(&ctx).boxed();
            while let Some(Ok(message)) = messages.next().await {
                message
                    .delete(&ctx)
                    .await
                    .expect("Error while deleting message");
            }
        }
    }
}
