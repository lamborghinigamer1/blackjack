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

    fn validate_value(&mut self) -> bool {
        let valid_values = [
            "Ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King",
        ];
        for value in valid_values {
            if value == self.value {
                self.value = value.chars().next().unwrap().to_string();
                if self.value == "1" {
                    self.value = "10".to_string();
                }
                return true;
            }
        }
        panic!("Invalid value: {}", self.value);
    }

    pub fn show(&self) -> String {
        let show = format!("{} {}", self.suit, self.value);
        show
    }

    pub fn score(&self, highace: bool) -> i32 {
        match self.value.as_str() {
            "A" => {
                if highace {
                    11
                } else {
                    1
                }
            }
            "B" | "Q" | "J" | "K" => 10,
            _ => match self.value.parse::<i32>() {
                Ok(parsedvalue) => parsedvalue,
                Err(_) => 0,
            },
        }
    }
}
