use std::collections::HashMap;
use crate::game::deck;
use crate::lua_random;
use crate::game::voucher;
use crate::game::random as game_random;
use crate::game::deck::*;

struct NodeMap (HashMap<String, f64>);

impl NodeMap {
    pub fn new() -> Self {
        NodeMap (HashMap::new())
    }

    fn insert(&mut self, key: &str, value: f64) {
        self.0.insert(key.to_string(), value);
    }

    fn get(&self, key: &str) -> Option<&f64> {
        self.0.get(key)
    }

    pub fn random(&mut self, id: &str, hashed_seed: f64) -> f64 {

        let node_value;

        if let Some(value) = self.get(id) {
            node_value = *value;
        } else {
            node_value = lua_random::seed_from_string(id);
        }

        println!("Node id: {}", id);
        println!("Hashed Seed Value: {:.25}", hashed_seed);
        println!("Node Value: {:.25}", node_value);

        let x1 = lua_random::fract(node_value * 1.72431234 + 2.134453429141);
        let x2 = lua_random::round_double(&x1, 13);

        println!("x1: {:.25}", x1);
        println!("x2: {:.25}", x2);

        self.insert(id, x2);

        println!("added: {:.25}", x2 + hashed_seed);
        println!("added and divided: {:.25}", (x2 + hashed_seed) / 2.0);

        (x2 + hashed_seed) / 2.0
    }
}

pub struct RandomState {
    pub seed: String,
    pub hashed_seed: f64,
    pub nodes: NodeMap,
    pub ante: u32
}

impl RandomState {
    pub fn new(seed: &str) -> Self {
        RandomState { 
            seed: seed.to_string(), 
            hashed_seed: lua_random::seed_from_string(seed), 
            nodes: NodeMap::new(),
            ante: 1
        }
    }

    pub fn get_node(&mut self, id: &str) -> f64 {
        self.nodes.random(id, self.hashed_seed)
    }

    pub fn random_int(&mut self, min: f64, max: f64, seed: &str) -> i64 {
        let mut state = lua_random::random_state_from_seed(self.get_node(seed));
        lua_random::random_int(&mut state, min, max)
    }

    pub fn random_usize(&mut self, min: f64, max: f64, seed: &str) -> usize {
        self.random_int(min, max, seed) as usize
    }

    pub fn random_double(&mut self, seed: &str) -> f64 {
        let mut state = lua_random::random_state_from_seed(self.get_node(seed));
        unsafe { lua_random::random_double(&mut state).d }
    }

    pub fn roll_for_soul(&mut self, type_str: &str, seed: &str) -> bool {
        let state_seed = game_random::concat_strings(&["soul_", type_str, &self.ante.to_string(), seed]);
        self.random_double(&state_seed) > 0.997
    }
}

pub struct State {
    pub random_state: RandomState,
    pub deck: deck::Deck,
    pub vouchers: voucher::VoucherArray,
    pub gold: u64,
    pub joker_amount: u64,
    is_first_pacK: bool
}

impl State {
    pub fn new(seed: &str, deck_type: deck::deck_data::Decks) -> State {
        let mut state = State { 
            random_state: RandomState::new(seed),
            deck: Deck::new(),
            vouchers: voucher::VoucherArray::new(),
            gold: 4,
            joker_amount: 5,
            is_first_pacK: true
        };

        state.deck = deck::Deck::new_with_state(deck_type, &mut state);

        state
    }

    pub fn next_ante(&mut self) {
        self.random_state.ante += 1;
    }

    pub fn set_gold(&mut self, amount: u64) {
        self.gold = amount;
    }

    pub fn set_joker_amount(&mut self, amount: u64) {
        self.joker_amount = amount;
    }

    pub fn is_first_pacK(&mut self) -> bool {
        let value = self.is_first_pacK;
        self.is_first_pacK = !self.is_first_pacK && true;
        value
    }
}