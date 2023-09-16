use crate::card::Card;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let valid_suits = ["Hearts", "Spades", "Diamonds", "Clubs"];
        let valid_values = [
            "Ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King",
        ];
        let mut cards = Vec::new();
        for validsuit in valid_suits {
            for validvalue in valid_values {
                cards.push(Card::new(validsuit.to_string(), validvalue.to_string()));
            }
        }
        Deck { cards }
    }
    pub fn draw_card(&mut self) -> Option<Card> {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
        self.cards.pop()
    }
}
