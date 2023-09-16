mod card;

fn main() {
    let card = card::Card::new("Hearts".to_string(), "Ace".to_string());
    println!("{}", card.show());
}
