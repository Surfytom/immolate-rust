mod lua_random;
mod game;
use crate::game::state;
use crate::game::pack;

fn main() {

    let mut game_state = state::State::new("ABC");

    let mut random_pack = pack::Pack::get_random_pack(&mut game_state.random_state, true);

    println!("First Pack: {:?}", random_pack);

    let cards = random_pack.open(&mut game_state.random_state);

    println!("Pack Cards: {:?}", cards);

    let random_pack_2 = pack::Pack::get_random_pack(&mut game_state.random_state, false);

    println!("Second Pack: {:?}", random_pack_2);
}