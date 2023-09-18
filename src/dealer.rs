use crate::{
    blackjack::Blackjack,
    deck::Deck,
    player::{self, Player},
};

pub struct Dealer {
    blackjack: Blackjack,
    deck: Deck,
    players: Vec<Player>,
}

impl Dealer {
    pub fn new(blackjack: Blackjack, deck: Deck) -> Self {
        let players = Vec::new();
        Dealer {
            blackjack,
            deck,
            players,
        }
    }
    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn play_game(&mut self) {
        self.add_player(player::Player::new("Dealer".to_string()));

        for player in &mut self.players {
            player.add_card(self.deck.draw_card());
            player.add_card(self.deck.draw_card());
            println!(
                "{} {} -> {}",
                player.name(),
                player.show_hand(),
                self.blackjack.score_hand(&player.hand())
            );
        }
        for player in &mut self.players {
            if player.name() != "Dealer" {
                loop {
                    let mut anothercard = String::new();
                    let mut playerscore = self.blackjack.score_hand(&player.hand());

                    if playerscore == "Bust!"
                        || playerscore == "Twenty one!"
                        || playerscore == "Blackjack!"
                        || playerscore == "Five Card Charlie."
                    {
                        break;
                    }

                    println!(
                        "{} your {}, would you like another card? (y/n)",
                        player.name(),
                        playerscore
                    );
                    std::io::stdin().read_line(&mut anothercard).unwrap();

                    if anothercard.to_lowercase().trim() == "y" {
                        player.add_card(self.deck.draw_card());
                        playerscore = self.blackjack.score_hand(&player.hand());
                        println!(
                            "{} drew {}",
                            player.name(),
                            &player.hand().last().unwrap().show()
                        );
                        println!(
                            "{} {} -> {}",
                            player.name(),
                            player.show_hand(),
                            playerscore
                        );
                    } else {
                        break;
                    }
                }
            } else {
                println!(
                    "{} {} -> {}",
                    player.name(),
                    player.show_hand(),
                    self.blackjack.score_hand(&player.hand())
                );
                loop {
                    let mut dealerscore = 0;
                    let mut amountcard = 0;

                    for card in player.hand() {
                        if dealerscore < 11 {
                            dealerscore += card.score(true)
                        } else {
                            dealerscore += card.score(false);
                        }
                        amountcard += 1;
                    }
                    if dealerscore > 17 || amountcard == 5 {
                        break;
                    } else {
                        player.add_card(self.deck.draw_card());
                        println!(
                            "{} drew {}",
                            player.name(),
                            &player.hand().last().unwrap().show()
                        );
                        println!(
                            "{} {} -> {}",
                            player.name(),
                            player.show_hand(),
                            self.blackjack.score_hand(&player.hand()).trim()
                        );
                    }
                }
            }
        }
    }
}
