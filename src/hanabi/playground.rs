use std::collections::HashMap;
use strum::IntoEnumIterator;

use crate::hanabi::card;

struct Stack{
    color : card::Color,
    value : u8,
}

struct Playground{
    color_stacks : HashMap<card::Color, Stack>,

}

impl Playground{
    pub fn new() -> Playground{
        let p = Playground{
            color_stacks : HashMap::new(),
        };

        // Generate the different color stacks
        for c in card::Color::iter(){
            p.color_stacks.insert(c, Stack{color : c, value : 0});
        }

        p
    }

    pub fn can_play_card(&self, card : &card::Card) -> bool{
        let stack = self.color_stacks.get(&card.color).unwrap();
        stack.value + 1 == card.value
    }
}
