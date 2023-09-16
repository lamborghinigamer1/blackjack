pub struct Card {
    suit: String,
    value: String,
}

impl Card {
    pub fn new(suit: String, value: String) -> Card {
        let card = Card { suit, value };
        card.validate_suit();
        card.validate_value();
        card
    }

    fn validate_suit(&self) {
        let valid_suits = ["Hearts", "Spades", "Diamonds", "Clubs"];
        if !valid_suits.contains(&self.suit.as_str()) {
            panic!("Invalid suit: {}", self.suit);
        }
    }
    fn validate_value(&self){
        let valid_values = ["Ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King"];
        if !valid_values.contains(&self.value.as_str()) {
            panic!("Invalid value: {}", self.value);
        }
    }
    pub fn show(&self) -> String
    {
        let show = format!("{} {}", self.suit, self.value);
        show
    }
}
