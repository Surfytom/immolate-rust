use crate::game::random;
use crate::game::spectral;
use crate::game::state::RandomState;
mod tarot_data;

#[derive(Debug)]
pub struct Tarot {
    name: &'static str
}

impl random::Random for Tarot {
    const CARD_TYPE: &'static str = "Tarot";
    const APPEND_KEY: &'static str = "ar1";
    type Item = Tarot;

    fn before_get_random(random_state: &mut RandomState) -> Option<Self::Item> {
        if random_state.roll_for_soul(Self::CARD_TYPE, Self::APPEND_KEY) { 
            Some(Tarot::new(spectral::Spectral::get_soul()))
        } else {
            None
        }
    }

    fn get_random(random_state: &mut RandomState) -> Self::Item {
        if let Some(value) = Self::before_get_random(random_state) {
            return value;
        }

        let seed = random::concat_strings(&[Self::CARD_TYPE, Self::APPEND_KEY, &random_state.ante.to_string(), &random_state.seed]);
        let tarot_index = random_state.random_usize(0.0, Self::get_max(), &seed);

        Self::from_number(tarot_index)
    }

    fn from_number(index: usize) -> Self::Item {
        Tarot { name: tarot_data::TAROT_CARDS[index] }
    }

    fn get_max() -> f64 {
        (tarot_data::TAROT_CARDS.len() - 1) as f64
    }
}

impl Tarot {
    pub fn new(name: &'static str) -> Self {
        Tarot { name }
    }
}