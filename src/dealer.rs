use crate::{blackjack::Blackjack, deck::Deck, player::Player};

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
        for player in &mut self.players {
            player.add_card(self.deck.draw_card());
            player.add_card(self.deck.draw_card());
            println!(
                "{} {} {}",
                player.name(),
                player.show_hand(),
                self.blackjack.score_hand(&player.hand())
            );
        }
    }
}
