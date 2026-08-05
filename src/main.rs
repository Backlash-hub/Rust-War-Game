mod cards;
mod player;

use cards::Deck;

fn main() {
    let deck = Deck::new();
    for card in &deck.cards {
        println!("{:?}", card);
    };
}
