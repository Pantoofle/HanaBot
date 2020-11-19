use std::collections::HashMap;
use strum::IntoEnumIterator;

use crate::hanabi::card;

struct Stack{
    color : card::Color,
    value : u8,
}

pub struct Playground{
    color_stacks : HashMap<card::Color, Stack>,
    hint_tokens : u8,
    error_tokens : u8,
    discard_pile : Vec<card::Card>,
    draw_pile : Vec<card::Card>,
}

impl Playground{
    pub fn new() -> Playground{
        let mut p = Playground{
            color_stacks : HashMap::new(),
            hint_tokens : 8,
            error_tokens : 3,
            discard_pile : Vec::<card::Card>::new(),
            draw_pile : Vec::<card::Card>::new(),
        };

        // Generate the different color stacks
        for c in card::Color::iter(){
            p.color_stacks.insert(c.clone(), Stack{color : c, value : 0});
        }

        // Populate the draw pile
        p.draw_pile = Playground::full_deck();
        p
    }

    pub fn can_play_card(&self, card : &card::Card) -> bool{
        let stack = self.color_stacks.get(&card.color).unwrap();
        stack.value + 1 == card.value
    }

    fn full_deck() -> Vec<card::Card>{
        let mut deck = Vec::<card::Card>::new();
        let card_nb = vec![3,2,2,2,1];

        for c in card::Color::iter(){
            for (card_nb, nb_copies) in card_nb.iter().enumerate(){
                let copies = {
                    if c == card::Color::All { 1 }
                    else { *nb_copies }
                };

                for _ in 1..=copies{
                    deck.push(card::Card {color : c, value : (card_nb+1) as u8, hints : (None, None)})
                }
            }
        }

        deck
    }
}
