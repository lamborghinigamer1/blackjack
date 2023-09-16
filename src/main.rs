mod card;
mod deck;
mod player;

fn main() {
    let mut deck = deck::Deck::new();
    let mut player = player::Player::new("Juan".to_string());
    player.add_card(deck.draw_card());
    player.add_card(deck.draw_card());
    println!("{} {}", player.name(), player.show_hand());
}
