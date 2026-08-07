
use crate::cards::{Card, Deck};

pub struct Player {
    pub name: String,
    pub hand: Vec<Card>,
    pub winning_pile: Vec<Card>,
}

impl Player {
    pub fn new(name: &str, hand: Vec<Card>, winning_pile: Vec<Card>) -> Self {
        Player {
            name: name.to_string(),
            hand,
            winning_pile,
        }
    }
}

pub fn new_players(player_name: &str, mut deck: Deck) -> (Player, Player) {
    let half = deck.cards.len() / 2;
    let hand2= deck.cards.split_off(half);
    let hand1 = deck.cards;
    let winning_pile_1 = Vec::new();
    let winning_pile_2 = Vec::new();

    let player1 = Player::new(&player_name, hand1, winning_pile_1);
    let player2 = Player::new("Computer", hand2, winning_pile_2);
    
    (player1, player2)
}