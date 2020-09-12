use crate::hanabi::card;

struct Hand{
    cards : Vec<card::Card>,
}

impl Hand{
    pub fn give_hint(&self, hint : card::Hint){
        for card in self.cards{
            if hint.applies_to(&card){
                card.add_hint(hint);
            }
        }
    }
}
