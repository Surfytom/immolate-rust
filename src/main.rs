mod random;
mod game;
use crate::game::state;
use crate::game::pack;
use crate::game::joker;
use crate::game::tarot;
use crate::game::voucher;
use crate::game::item;

fn main() {

    // let hashed_seed = random::seed_from_string("BB");
    // let mut starting_state = random::random_state_from_seed(hashed_seed);
    // println!("starting state: {:?}", starting_state);

    let mut game_state = state::State::new("ABC");

    let mut random_pack = item::Pack::get_random_pack(&mut game_state.random_state, true);

    println!("First Pack: {:?}", random_pack);

    let cards = random_pack.open(&mut game_state.random_state);

    println!("Pack Cards: {:?}", cards);

    let random_pack_2 = item::Pack::get_random_pack(&mut game_state.random_state, false);

    println!("Second Pack: {:?}", random_pack_2);
}