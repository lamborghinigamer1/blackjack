use crate::card::Card;

pub struct Blackjack;

impl Blackjack {
    pub fn score_hand(&mut self, hand: &Vec<Card>) -> String {
        let mut score = 0;
        let mut cards = 0;
        for card in hand {
            if score < 11 {
                score += card.score(true);
            } else {
                score += card.score(false);
            }
            cards += 1;
        }
        match (score, cards) {
            (score, _) if score == 21 => {
                if cards == 2 {
                    "Blackjack!".to_string()
                } else {
                    "Twenty one!".to_string()
                }
            }
            (score, _) if score > 21 => "Bust!".to_string(),
            (_, 5) => "Five Card Charlie.".to_string(),
            (score, _) => format!("score is: {}", score),
        }
    }
}
