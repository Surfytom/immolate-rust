use crate::game::pack::PackCardType;

pub static SHOP_KEY: &'static str = "sho";

#[derive(Debug, Clone, Copy)]
pub struct Rate {
    pub rate: f64,
    pub card_type: PackCardType
}

pub static BASE_RATE_TOTAL: f64 = 28.0;

pub static BASE_RATES: [Rate; 5] = [
    Rate { rate: 20.0, card_type: PackCardType::Joker },
    Rate { rate: 4.0, card_type: PackCardType::Tarot },
    Rate { rate: 4.0, card_type: PackCardType::Planet },
    Rate { rate: 0.0, card_type: PackCardType::Card },
    Rate { rate: 0.0, card_type: PackCardType::Spectral }
];