use std::io;
use crate::player::new_players;
use crate::cards::Deck;

pub fn play_game() {
    println!("Welcome to War");
    let deck = Deck::new();

    println!("Player Name: ");

    let mut player1_name = String::new();

    io::stdin().read_line( & mut player1_name ).expect("Failed to read line");
    let player_name = player1_name.trim();

    if player_name.is_empty() {
        return;
    }

    let (mut player1, mut player2) = new_players(player_name, deck);

    let card1 = player1.hand.swap_remove(0);
    let card2 = player2.hand.swap_remove(0);

    if card1.rank > card2.rank {
        println!("{} wins the round!", player1.name);
        player1.winning_pile.push(card1);
        player1.winning_pile.push(card2);
    } else if card2.rank > card1.rank {
        println!("{} wins the round!", player2.name);
        player2.winning_pile.push(card1);
        player2.winning_pile.push(card2);
    } else {
        println!("It's a tie!");
    }

    println!("Player1 = {}", player1.name);
    for card in &player1.winning_pile {
        println!("Winning Pile = {:?}", card);
    }
    println!("");
    println!("Player2 = {}", player2.name);
    for card in &player2.winning_pile {
        println!("Winning Pile = {:?}", card);
    }
}