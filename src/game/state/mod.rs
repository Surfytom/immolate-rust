use std::collections::HashMap;
use crate::game::deck;
use crate::random;

struct NodeMap (HashMap<String, f64>);

impl NodeMap {
    pub fn new() -> Self {
        Self (HashMap::new())
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
    pub nodes: NodeMap
}

impl RandomState {
    pub fn new(seed: &str) -> Self {
        RandomState { 
            seed: seed.to_string(), 
            hashed_seed: random::seed_from_string(seed), 
            nodes: NodeMap::new() 
        }
    }

    pub fn get_node(&mut self, id: &str) -> f64 {
        self.nodes.random(id, self.hashed_seed)
    }
}

pub struct State {
    pub random_state: RandomState,
    pub deck: deck::Deck,
    pub ante: u32
}

impl State {
    pub fn new(seed: &str) -> State {
        State { 
            random_state: RandomState::new(seed),
            deck: deck::Deck::new(),
            ante: 1
        }
    }
}