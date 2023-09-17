mod blackjack;
mod card;
mod deck;
mod player;

fn main() {
    let mut deck = deck::Deck::new();
    let mut player = player::Player::new("Juan".to_string());
    player.add_card(deck.draw_card());
    player.add_card(deck.draw_card());

    let black_jack = blackjack::Blackjack::score_hand(&player.hand());

    println!("{} {} {}", player.name(), player.show_hand(), black_jack);
}
