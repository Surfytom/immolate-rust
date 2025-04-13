
use crate::game::state::RandomState;
use crate::game::random;
use crate::game::spectral::Spectral;
mod planet_data;

#[derive(Debug)]
pub struct Planet {
    name: &'static str
}

impl random::Random for Planet {
    const CARD_TYPE: &'static str = "Planet";
    const APPEND_KEY: &'static str = "pl1";
    type Item = Planet;

    fn before_get_random(random_state: &mut RandomState) -> Option<Self::Item> {
        if random_state.roll_for_soul(Self::CARD_TYPE, Self::APPEND_KEY) { 
            return Some(Planet::new(Spectral::get_soul()));
        }

        if random_state.roll_for_soul(Self::CARD_TYPE, Self::APPEND_KEY) { 
            return Some(Planet::new(Spectral::get_black_hole())); 
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
        Planet::new(planet_data::PLANET_CARDS[index])
    }

    fn get_max() -> f64 {
        (planet_data::PLANET_CARDS.len() - 1) as f64
    }
}

impl Planet {
    pub fn new(name: &'static str) -> Self {
        Planet { name }
    }
}