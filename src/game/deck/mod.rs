pub mod deck_data;
use crate::game::state::*;
use crate::game::card::card_data::CARD_CARDS;

#[derive(Debug)]
pub struct Deck {
    cards: Vec<&'static str>,
    hand_size: u32,
    discard_amount: u32,
    hand_amount: u32,
    hand: Vec<&'static str>
}

impl Deck {
    pub fn new() -> Self {
        Deck { cards: Deck::get_standard_deck(), hand_size: 8, discard_amount: 4, hand_amount: 4, hand: Vec::new() }
    }

    pub fn new_with_state(deck_type: deck_data::Decks, game_state: &mut State) -> Self {
        deck_type.setup(game_state)
    }

    pub fn set_discard(&mut self, amount: u32) {
        self.discard_amount = amount;
    }

    pub fn set_hand_amount(&mut self, amount: u32) {
        self.hand_amount = amount;
    }

    pub fn set_hand_size(&mut self, amount: u32) {
        self.hand_size = amount;
    }

    fn fill_cards(&mut self) {
        self.cards = CARD_CARDS.clone().to_vec();
    }

    fn get_standard_deck() -> Vec<&'static str> {
        CARD_CARDS.clone().to_vec()
    }

    fn filter_deck(&mut self, match_strings: &[&str]) {
        self.cards.retain(| card | match_strings.iter().all(| s |
            !card.contains(&s.to_uppercase())
        ))
    }

    fn double_deck(&mut self) {
        let mut card_copy = self.cards.clone();
        self.cards.append(&mut card_copy);
    }
}
