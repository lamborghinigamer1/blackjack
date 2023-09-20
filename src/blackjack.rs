use crate::{card::Card, player::Player};

pub struct Blackjack;

impl Blackjack {
    pub fn score_hand_int(&mut self, hand: &Vec<Card>) -> i32 {
        let mut score = 0;
        for card in hand {
            if score < 11 {
                score += card.score(true);
            } else {
                score += card.score(false);
            }
        }
        score
    }
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
    pub fn check_final_scores(&mut self, players: &Vec<Player>) -> Vec<String> {
        let mut finalscore = Vec::new();
        let dealer = players.last().unwrap();
        let dealerscore = self.score_hand_int(dealer.hand());
        for player in players {
            if player.name() != "Dealer" {
                let mut playerscore = 0;
                playerscore += self.score_hand_int(player.hand());
                match (playerscore, dealerscore) {
                    (x, y) if x == y => {
                        finalscore.push(format!("{} tied with the Dealer", player.name()))
                    }
                    (x, y) if x <= 21 && (y > 21 || x > y) => {
                        finalscore.push(format!("{} wins from the Dealer", player.name()))
                    }
                    _ => finalscore.push(format!("{} loses from the Dealer", player.name())),
                }
            }
        }
        finalscore
    }
}
