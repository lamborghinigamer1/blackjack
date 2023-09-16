pub struct Card {
    suit: String,
    value: String,
}

impl Card {
    pub fn new(suit: String, value: String) -> Card {
        let mut card = Card { suit, value };
        card.validate_suit();
        card.validate_value();
        card
    }

    fn validate_suit(&mut self) -> bool {
        let valid_suit = ["Hearts", "Spades", "Diamonds", "Clubs"];
        let changesuit = ["♥", "♠", "♦", "♣"];
        let mut index = 0;
        for suit in valid_suit {
            if suit == self.suit {
                self.suit = changesuit[index].to_string();
                return true;
            }
            index += 1;
        }
        panic!("Invalid suit: {}", self.suit);
    }
    fn validate_value(&self) {
        let valid_values = [
            "Ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King",
        ];
        if !valid_values.contains(&self.value.as_str()) {
            panic!("Invalid value: {}", self.value);
        }
    }
    pub fn show(&self) -> String {
        let show = format!("{} {}", self.suit, self.value);
        show
    }
}
