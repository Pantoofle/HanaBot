use strum::EnumIter;

#[derive(Clone, Copy, EnumIter, PartialEq, Eq, Hash, Debug)]
pub enum Color{
    Red,
    Blue,
    Green,
    White,
    Yellow,
    All,
}

#[derive(Clone, Copy)]
pub enum Hint{
    Color(Color),
    Value(u8),
}

impl Hint{
    // Returns true if the hint applies to a given card
    pub fn applies_to(&self, card:&Card) -> bool{
        match self{
            Hint::Color(c) if *c == card.color => true,
            Hint::Value(v) if *v == card.value => true,
            _ => false,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Card{
    pub value: u8,
    pub color: Color,
    pub hints: (Option<Hint>, Option<Hint>),
}

impl Card{
    // Add an hint to the card
    pub fn add_hint(&mut self, hint:Hint) {
        match hint{
            Hint::Color(c) => {
                match self.hints.0{
                    Some(Hint::Color(c2)) if c2 == c => {
                        self.hints.0 = Some(Hint::Color(Color::All));
                    },
                    _ => (),
                }
            }
            Hint::Value(v) => {
                self.hints.1 = Some(Hint::Value(v));
            }

        }

    }
}
