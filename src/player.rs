use crate::card::Card;

pub struct Player {
    name: String,
    hand: Vec<Card>,
}

impl Player {
    pub fn new(name: String) -> Player {
        let hand = Vec::new();
        Player { name, hand }
    }
    pub fn name(&self) -> String {
        self.name.to_string()
    }
    pub fn add_card(&mut self, card: Card) {
        self.hand.push(card);
    }
    pub fn show_hand(&self) -> String {
        let mut returnstring = "has ".to_string();
        for card in &self.hand {
            returnstring += &format!("{} ", &card.show());
        }
        returnstring
    }
    pub fn hand(&self) -> &Vec<Card> {
        &self.hand
    }
}
