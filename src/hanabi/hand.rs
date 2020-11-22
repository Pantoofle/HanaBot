use crate::hanabi::card;

pub struct Hand {
    pub cards: Vec<card::Card>,
}

impl Hand {
    pub fn new() -> Hand {
        Hand { cards: vec![] }
    }

    pub fn add_card(&mut self, card: card::Card) {
        self.cards.push(card);
    }

    // Add the hint to all cards it applies to in the hand
    pub fn give_hint(&mut self, hint: card::Hint) {
        for card in &mut self.cards {
            if hint.applies_to(&card) {
                card.add_hint(hint);
            }
        }
    }
}
