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
        println!("Player's name:");
        let mut playername = String::new();
        std::io::stdin().read_line(&mut playername).unwrap();
        dealer.add_player(player::Player::new(playername.trim().to_string()));
    }

    dealer.play_game();
}
