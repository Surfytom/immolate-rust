mod lua_random;
mod game;
use crate::game::state;
use crate::game::pack;
use crate::game::deck::deck_data::Decks;
use crate::game::shop;

fn main() {

    let mut game_state = state::State::new("ABC", Decks::RED);

    let mut shop = shop::Shop::default();

    shop.random(&mut game_state);

    println!("shop: {:?}", shop);
    // let random_pack = pack::Pack::get_random_pack(&mut game_state);
    // println!("Pack: {:?}", random_pack);
}