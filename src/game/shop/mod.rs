pub mod shop_data;
use crate::game::voucher;
use crate::game::pack::*;
use crate::game::state;
use crate::game::random;
use crate::game::tarot::*;
use crate::game::joker::*;
use crate::game::planet::*;
use crate::game::card::*;
use crate::game::spectral::*;
use crate::game::random::Random;

#[derive(Debug)]
pub struct Shop {
    shop_size: usize,
    voucher: voucher::Voucher,
    packs: [Pack; 2],
    cards: Vec<CardType>,
    rates: [shop_data::Rate; 5],
    total_rate: f64
}

impl Shop {

    pub fn default() -> Self {
        Shop {
            shop_size: 2,
            voucher: voucher::Voucher::from_number(0),
            packs: [WEIGHTED_PACKS[0].clone(), WEIGHTED_PACKS[0].clone()],
            cards: Vec::new(),
            rates: shop_data::BASE_RATES.clone(),
            total_rate: shop_data::BASE_RATE_TOTAL.clone()
        }
    }

    pub fn random(&mut self, game_state: &mut state::State) {
        self.random_packs(game_state);
        self.random_voucher(game_state);
        self.random_cards(game_state);
    }

    fn random_packs(&mut self, game_state: &mut state::State) {
        for pack in self.packs.iter_mut() {
            *pack = Pack::get_random_pack(game_state);
        }
    }

    fn random_voucher(&mut self, game_state: &mut state::State) {
        self.voucher = game_state.vouchers.random(&mut game_state.random_state, false);
    }

    fn random_cards(&mut self, game_state: &mut state::State) {
        let mut shop_cards: Vec<CardType> = Vec::with_capacity(self.shop_size);
        for _i in 0..self.shop_size {
            shop_cards.push(self.random_card(game_state));
        }
        self.cards = shop_cards
    }

    fn random_card(&mut self, game_state: &mut state::State) -> CardType {
        let seed = random::concat_strings(&["cdt", &game_state.random_state.ante.to_string(), &game_state.random_state.seed]);
        let dbl = game_state.random_state.random_double(&seed);

        let polled_rate = dbl * self.total_rate;
        let mut check_rate: f64 = 0.0;

        for rate in self.rates {
            check_rate += rate.rate;

            if check_rate > polled_rate {
                return self.get_random_card(&mut game_state.random_state, rate.card_type, Rarity::COMMON);
            }
        }

        self.get_random_card(&mut game_state.random_state, self.rates[0].card_type, Rarity::COMMON)
    }


    fn get_random_card(&self, random_state: &mut state::RandomState, card_type: PackCardType, rarity: Rarity) -> CardType {
        println!("{:?}", card_type);
        match card_type {
            PackCardType::Tarot => CardType::Tarot(Tarot::get_random(random_state)),
            PackCardType::Joker => CardType::Joker(Joker::get_random(random_state, rarity, shop_data::SHOP_KEY)),
            PackCardType::Spectral => CardType::Spectral(Spectral::get_random(random_state)),
            PackCardType::Planet => CardType::Planet(Planet::get_random(random_state)),
            PackCardType::Card => CardType::Card(Card::get_random(random_state, shop_data::SHOP_KEY)),
            _ => panic!("No card type found from opening pack!!!")
        }
    }

    pub fn set_shop_size(&mut self, amount: usize) {
        self.shop_size = amount;
    }

    pub fn set_spectral_rate(&mut self, amount: f64) {
        self.rates[4].rate = amount;
        self.total_rate += amount;
    }

    pub fn set_card_rate(&mut self, amount: f64) {
        self.rates[3].rate = amount;
        self.total_rate += amount;
    }
}