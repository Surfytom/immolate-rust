use crate::game::state::RandomState;
use crate::game::random;
pub mod spectral_data;

#[derive(Debug)]
pub struct Spectral {
    name: &'static str
}

impl random::Random for Spectral {
    const CARD_TYPE: &'static str = "Spectral";
    const APPEND_KEY: &'static str = "spe";
    type Item = Spectral;

    fn before_get_random(random_state: &mut RandomState) -> Option<Self::Item> {
        if random_state.roll_for_soul(Self::CARD_TYPE, Self::APPEND_KEY) { 
            return Some(Spectral::new(Spectral::get_soul()));
        }

        if random_state.roll_for_soul(Self::CARD_TYPE, Self::APPEND_KEY) { 
            return Some(Spectral::new(Spectral::get_black_hole())); 
        }

        None
    }

    fn get_random(random_state: &mut RandomState) -> Self::Item {
        if let Some(value) = Self::before_get_random(random_state) {
            return value
        }

        let seed = random::concat_strings(&[Self::CARD_TYPE, Self::APPEND_KEY, &random_state.ante.to_string(), &random_state.seed]);
        let tarot_index = random_state.random_usize(0.0, Self::get_max(), &seed);

        Self::from_number(tarot_index)
    }

    fn from_number(index: usize) -> Self {
        Spectral::new(spectral_data::SPECTRAL_CARDS[index])
    }

    fn get_max() -> f64 {
        (spectral_data::SPECTRAL_CARDS.len() - 1) as f64
    }
}

impl Spectral {
    pub fn new(name: &'static str) -> Self {
        Spectral { name }
    }

    pub fn get_soul() -> &'static str {
        spectral_data::SPECTRAL_CARDS[16]
    }

    pub fn get_black_hole() -> &'static str {
        spectral_data::SPECTRAL_CARDS[17]
    }
}