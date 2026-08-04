mod cards;
use cards::Deck;

fn main() {
    let mut deck = Deck::new();
    for card in &deck.cards {
        println!("{:?}", card);
    };
}
