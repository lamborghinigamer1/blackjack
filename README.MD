# Blackjack

## What's this?

Blackjack is a card game commonly played in casinos. I decided to make this project in Rust, because I also have a Discord bot written in Rust and just want a simple way to add parameters and change up the game. Another reason is that I'm practicing Object Oriented Programming and just felt like making it in my free time.

## Rules of Black Jack

1. Each player place their bets.

2. The dealer gives every player from the deck 2 cards each containing 2 values. In total from 2 all the way up to 21.

3. The dealer asks each player if they want another card.

4. If a player or the dealer goes over 21, they have gone bust.

5. If a player or the dealer has 5 cards they automatically lose (Five card charlie).

6. When someone wins they receive all the bets, but if it's a tie the one that drew get their bet back.

## How to use?

1. Create a new Cargo project and add add 

```toml
[dependencies]
blackjack = { git = "https://github.com/lamborghinigamer1/blackjack.git" }
```
in your Cargo.toml file

2. To start a new blackjack game in the console just add

```rust
use blackjack::{Dealer, Deck, Player};

fn main() {
    let mut dealer = Dealer::new(blackjack::Blackjack, Deck::new());
    dealer.add_player(Player::new("Player1".to_string()));
    dealer.add_player(Player::new("Player2".to_string()));
    dealer.play_game();
}
```

To your main.rs file (Feel free to add as many players as you want)