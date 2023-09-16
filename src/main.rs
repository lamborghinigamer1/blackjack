mod card;
mod deck;

fn main() {
    let mut deck = deck::Deck::new();
    let card = deck.draw_card().unwrap();
    println!("{}", card.show());
}
