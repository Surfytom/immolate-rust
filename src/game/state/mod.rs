use std::collections::HashMap;
use crate::game::deck;
use crate::random;
use crate::game::voucher;
use crate::game::random as game_random;

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
            node_value = random::seed_from_string(id);
        }

        println!("Node id: {}", id);
        println!("Hashed Seed Value: {:.25}", hashed_seed);
        println!("Node Value: {:.25}", node_value);

        let x1 = random::fract(node_value * 1.72431234 + 2.134453429141);
        println!("x1: {:.25}", x1);
        let x2 = random::round_double(&x1, 13);
        println!("x2: {:.25}", x2);

        self.insert(id, x1);

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
            hashed_seed: random::seed_from_string(seed), 
            nodes: NodeMap::new(),
            ante: 1
        }
    }

    pub fn get_node(&mut self, id: &str) -> f64 {
        self.nodes.random(id, self.hashed_seed)
    }

    pub fn random_int(&mut self, min: f64, max: f64, seed: &str) -> i64 {
        let mut state = random::random_state_from_seed(self.get_node(seed));
        random::random_int(&mut state, min, max)
    }

    pub fn random_usize(&mut self, min: f64, max: f64, seed: &str) -> usize {
        self.random_int(min, max, seed) as usize
    }

    pub fn random_double(&mut self, seed: &str) -> f64 {
        let mut state = random::random_state_from_seed(self.get_node(seed));
        unsafe { random::random_double(&mut state).d }
    }

    pub fn roll_for_soul(&mut self, type_str: &str, seed: &str) -> bool {
        let state_seed = game_random::concat_strings(&["soul_", type_str, &self.ante.to_string(), seed]);
        self.random_double(&state_seed) > 0.997
    }
}

pub struct State {
    pub random_state: RandomState,
    pub deck: deck::Deck,
    pub vouchers: voucher::VoucherArray
}

impl State {
    pub fn new(seed: &str) -> State {
        State { 
            random_state: RandomState::new(seed),
            deck: deck::Deck::new(),
            vouchers: voucher::VoucherArray::new()
        }
    }

    pub fn next_ante(&mut self) {
        self.random_state.ante += 1;
    }
}