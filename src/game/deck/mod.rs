#[allow(non_camel_case_types)]
enum Suites {
    HEARTS,
    CLUBS,
    SPADES,
    DIAMONDS
}

#[allow(non_camel_case_types)]
enum Ranks {
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    TEN,
    JACK,
    QUEEN,
    KING,
    ACE
}

pub struct Card {
    suite: Suites,
    rank: Ranks
}

pub struct Deck {
    cards: Vec<Card>,
    hand_size: u32,
    discard_amount: u32,
    hand_amount: u32,
    hand: Vec<Card>
}

impl Deck {
    pub fn new() -> Deck {
        Deck {
            cards: Vec::new(),
            hand_size: 0,
            discard_amount: 0,
            hand_amount: 0,
            hand: Vec::new()
        }
    }
}

#[allow(non_camel_case_types)]
pub enum Hands {
    FLUSH_FIVE,
    FLUSH_HOUSE,
    FIVE_OF_A_KIND,
    STRAIGHT_FLUSH,
    FOUR_OF_A_KIND,
    FULL_HOUSE,
    FLUSH,
    STRAIGHT,
    THREE_OF_A_KIND,
    TWO_PAIR,
    PAIR,
    HIGH_CARD
}