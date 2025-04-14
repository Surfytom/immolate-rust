use crate::game::state::RandomState;
use crate::game::random;
pub mod card_data;

#[derive(Debug)]
pub struct Card {
    pub name: &'static str
}

impl Card {
    pub fn new(name: &'static str) -> Self {
        Card { name }
    }

    pub fn from_number(index: usize) -> Self {
        Card { name: card_data::CARD_CARDS[index] }
    }

    pub fn get_random(random_state: &mut RandomState, key: &str) -> Card {
        let seed = random::concat_strings(&["front", key, &random_state.ante.to_string(), &random_state.seed]);
        let card_number = random_state.random_usize(0.0, Card::get_max(), &seed);

        Card::from_number(card_number)
    }

    pub fn get_max() -> f64 {
        (card_data::CARD_CARDS.len() - 1) as f64
    }
}