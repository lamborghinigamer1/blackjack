use crate::card::Card;

pub struct Blackjack;

impl Blackjack {
    pub fn score_hand(&mut self, hand: &Vec<Card>) -> String {
        let mut score = 0;
        let mut cards = 0;
        for card in hand {
            score += card.score(true);
            cards += 1;
        }
        match (score, cards) {
            (score, _) if score == 21 => "Black Jack!".to_string(),
            (score, _) if score > 21 => "Bust!".to_string(),
            (_, 5) => "Five Card Charlie.".to_string(),
            (score, _) => format!("Score: {}", score),
        }
    }
}
