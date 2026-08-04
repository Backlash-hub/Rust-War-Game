
//Building blocks of Cards 

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

impl Suit {
    pub const ALL: [Suit; 4] = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rank {
    Value(u8),
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    pub const ALL: [Rank; 13] = [
        Rank::Value(2), Rank::Value(3), Rank::Value(4), Rank::Value(5),
        Rank::Value(6), Rank::Value(7), Rank::Value(8), Rank::Value(9),
        Rank::Value(10), Rank::Jack, Rank::Queen, Rank::King, Rank::Ace,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new () -> Self {
        let mut cards = Vec::new();
        for suit in Suit::ALL {
            for rank in Rank::ALL {
                cards.push(Card {rank, suit });
            }
        }
    Deck { cards }
    }   
}