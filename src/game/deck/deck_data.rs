use crate::game::deck::*;
use crate::game::state::*;

#[allow(non_camel_case_types)]
pub enum Hands {
    FLUSH_FIVE,
    FLUSH_HOUSE,
    FIVE_OF_A_KIND,
    STRAIGHT_FLUSH,
    FOUR_OF_A_KIND,
    FULL_HOUSE,
    FLUSH,
    STRAIGHT,
    THREE_OF_A_KIND,
    TWO_PAIR,
    PAIR,
    HIGH_CARD
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Decks {
    RED,
    BLUE,
    YELLOW,
    // GREEN,
    BLACK,
    // MAGIC,
    // NEBULA,
    // GHOST,
    ABAONDONED,
    CHECKERED,
    // ZODIAC,
    PAINTED,
    // ANAGLYPH,
    // PLASMA,
    // ERRATIC
}

impl Decks {
    pub fn setup(&self, game_state: &mut State) -> Deck {

        let mut deck = Deck::new();

        match self {
            Decks::RED => deck.set_discard(deck.discard_amount + 1),
            Decks::BLUE => deck.set_hand_amount(deck.hand_amount + 1),
            Decks::YELLOW => game_state.set_gold(game_state.gold + 10),
            // GREEN
            Decks::BLACK => {
                deck.set_hand_amount(deck.hand_amount - 1);
                game_state.set_joker_amount(game_state.joker_amount + 1);
            },
            // MAGIC
            // NEBULA
            // GHOST
            Decks::ABAONDONED => {
                deck.filter_deck(&["J", "Q", "K"]);
            },
            Decks::CHECKERED => {
                // Might need re-ordering
                deck.filter_deck(&["D", "C"]);
                deck.double_deck();
            }
            // ZODIAC
            Decks::PAINTED => {
                deck.set_hand_size(deck.hand_size + 2);
                game_state.set_joker_amount(game_state.joker_amount - 1);
            }
            //ERRATIC

        }

        println!("Deck after {:?}: {:?}", self, deck);

        deck
    }


}