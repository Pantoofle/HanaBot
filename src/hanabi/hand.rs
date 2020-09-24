use crate::hanabi::card;

pub struct Hand{
    cards : Vec<card::Card>,
}

impl Hand{
    pub fn new() -> Hand{
        Hand{cards : vec![]}
    }

    pub fn give_hint(&mut self, hint : card::Hint){
        for card in &mut self.cards{
            if hint.applies_to(&card){
                card.add_hint(hint);
            }
        }
    }
}
