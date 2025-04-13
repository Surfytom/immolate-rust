use crate::game::state::RandomState;

pub trait Random {
    const CARD_TYPE: &'static str;
    const APPEND_KEY: &'static str;
    type Item;

    fn before_get_random(random_state: &mut RandomState) -> Option<Self::Item> {
        None
    }
    fn get_random(random_state: &mut RandomState) -> Self::Item;

    fn from_number(index: usize) -> Self::Item;
    fn get_max() -> f64;
}

pub fn concat_strings(strings: &[&str]) -> String {
    strings.join("")
}