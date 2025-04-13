mod random;
mod game;
use crate::game::state;
use crate::game::pack;
use crate::game::joker;

fn main() {

    // let hashed_seed = random::seed_from_string("BB");
    // let mut starting_state = random::random_state_from_seed(hashed_seed);
    // println!("starting state: {:?}", starting_state);

    // for _i in 0..10 {    
    //     println!("random int: {}", random::random_int(&mut starting_state, 0.0, 10.0));
    // }

    let mut game_state = state::State::new("ABC");
    
    let joker = joker::Joker::random(&mut game_state.random_state, joker::Rarity::COMMON, game_state.ante, "key");

    println!("Joker: {:?}", joker);
}