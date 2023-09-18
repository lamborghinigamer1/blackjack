mod blackjack;
mod card;
mod dealer;
mod deck;
mod player;

fn main() {
    let mut dealer = dealer::Dealer::new(blackjack::Blackjack, deck::Deck::new());
    dealer.add_player(player::Player::new("Lambo".to_string()));
    dealer.add_player(player::Player::new("Tijmoe".to_string()));

    dealer.play_game();
}
