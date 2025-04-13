use crate::game::state::RandomState;
use crate::game::random;
use std::fmt;
mod joker_data;

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
pub enum Rarity {
    COMMON,
    UNCOMMON,
    RARE,
    LEGENDARY
}

impl Rarity {
    fn as_usize(&self) -> usize {
        match self {
            Rarity::COMMON => 0,
            Rarity::UNCOMMON => 1,
            Rarity::RARE => 2,
            Rarity::LEGENDARY => 3,
        }
    }

    fn as_string(&self) -> &str {
        match self {
            Rarity::COMMON => "1",
            Rarity::UNCOMMON => "2",
            Rarity::RARE => "3",
            Rarity::LEGENDARY => "4",
        }
    }

    fn get_max(&self) -> f64 {
        (joker_data::JOKER_ARRAYS[self.as_usize()].len() - 1) as f64
    }

    
    pub fn debug_string(&self) -> &'static str {
        match self {
            Rarity::COMMON => "Common",
            Rarity::UNCOMMON => "Uncommon",
            Rarity::RARE => "Rare",
            Rarity::LEGENDARY => "Legendary"
        }
    }
}

pub struct Joker {
    pub name: &'static str,
    pub rarity: Rarity
}

impl Joker {
    pub fn new(name: &'static str, rarity: Rarity) -> Self {
        Joker { name, rarity }
    }

    pub fn from_number(num: usize, rarity: Rarity) -> Self {
        Joker { name: joker_data::JOKER_ARRAYS[rarity.as_usize()][num], rarity }
    }

    pub fn get_random(random_state: &mut RandomState, rarity: Rarity, key: &str) -> Joker {
        let (seed, random_rarity) = Joker::get_pool(random_state, rarity, random_state.ante, key);
        let joker_number = random_state.random_usize(0.0, random_rarity.get_max(), &seed);

        Joker::from_number(joker_number, random_rarity)
    }

    pub fn get_pool(random_state: &mut RandomState, rarity: Rarity, ante: u32, key: &str) -> (String, Rarity) {

        let mut random_rarity = Rarity::LEGENDARY;

        if rarity == Rarity::RARE {
            random_rarity = Rarity::RARE;
        }

        // Common here means just roll for a joker like normal
        if rarity == Rarity::COMMON {

            let seed = random::concat_strings(&["rarity", &ante.to_string(), key, &random_state.seed]);
            let db = random_state.random_double(&seed);

            if db > 0.95 {
                random_rarity = Rarity::RARE;
            } 
            else if db > 0.7 {
                random_rarity = Rarity::UNCOMMON;
            }
            else {
                random_rarity = Rarity::COMMON;
            }
        }

        let pool_key = random::concat_strings(&["Joker", random_rarity.as_string(), key, &ante.to_string(), &random_state.seed]);

        (pool_key, random_rarity)
    }
}

impl fmt::Debug for Rarity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.debug_string())
    }
}

impl fmt::Debug for Joker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} ({:?})", self.name, self.rarity)
    }
}