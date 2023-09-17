use crate::card::Card;

pub struct Blackjack;

impl Blackjack {
    pub fn score_hand(hand: &Vec<Card>) -> String{
        let mut score = 0;
        for card in hand {
            score += card.score(true);
        }
        format!("Score: {}", score)
    }
}