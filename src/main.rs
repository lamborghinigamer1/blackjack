mod blackjack;
mod card;
mod dealer;
mod deck;
mod player;

fn main() {
    println!("Welcome to blackjack");
    println!("How many players will be playing?");
    let mut dealer = dealer::Dealer::new(blackjack::Blackjack, deck::Deck::new());
    let mut playersamount = String::new();
    std::io::stdin().read_line(&mut playersamount).unwrap();

    let players_amount: i32 = match playersamount.trim().parse() {
        Ok(n) => n,
        Err(e) => panic!("Invalid number: {}", e),
    };
    for _ in 0..players_amount {
        let mut playername = String::new();
        println!("Player's name:");
        loop {
            std::io::stdin().read_line(&mut playername).unwrap();
            if playername.trim().is_empty() || playername.trim() == "Dealer" {
                println!("Please provide a valid name");
            }
            else {
                break;
            }
        }
        dealer.add_player(player::Player::new(playername.trim().to_string()));
    }

    dealer.play_game();
}
