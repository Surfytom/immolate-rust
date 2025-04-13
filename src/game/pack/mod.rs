use crate::game::state::RandomState;
use crate::game::random;
use crate::game::random::Random;

use crate::game::tarot::*;
use crate::game::joker::*;
use crate::game::spectral::*;
use crate::game::planet::*;
use crate::game::card::*;

static BUFFOON_PACK_INDEX: usize = 9;

#[derive(Clone, Copy, Debug)]
pub struct Pack {
    pub name: &'static str,
    weight: f64,
    size: usize,
    card_type: PackCardType,
    key: &'static str
}

static WEIGHTED_PACKS: [Pack; 15] = [
    Pack { name: "ARCANA_PACK", weight: 4.0, size: 3, card_type: PackCardType::Tarot, key: "ar1" },
    Pack { name: "JUMBO_ARCANA_PACK", weight: 2.0, size: 5, card_type: PackCardType::Tarot, key: "ar1" },
    Pack { name: "MEGA_ARCANA_PACK", weight: 0.5, size: 5, card_type: PackCardType::Tarot, key: "ar1" },
    Pack { name: "CELESTIAL_PACK", weight: 4.0, size: 3, card_type: PackCardType::Planet, key: "pl1" },
    Pack { name: "JUMBO_CELESTIAL_PACK", weight: 2.0, size: 5, card_type: PackCardType::Planet, key: "pl1" },
    Pack { name: "MEGA_CELESTIAL_PACK", weight: 0.5, size: 5, card_type: PackCardType::Planet, key: "pl1" },
    Pack { name: "STANDARD_PACK", weight: 4.0, size: 3, card_type: PackCardType::Card, key: "sta" },
    Pack { name: "JUMBO_STANDARD_PACK", weight: 2.0, size: 5, card_type: PackCardType::Card, key: "sta" },
    Pack { name: "MEGA_STANDARD_PACK", weight: 0.5, size: 5, card_type: PackCardType::Card, key: "sta" },
    Pack { name: "BUFFOON_PACK", weight: 1.2, size: 2, card_type: PackCardType::Joker, key: "buf" },
    Pack { name: "JUMBO_BUFFOON_PACK", weight: 0.6, size: 4, card_type: PackCardType::Joker, key: "buf" },
    Pack { name: "MEGA_BUFFOON_PACK", weight: 0.15, size: 4, card_type: PackCardType::Joker, key: "buf" },
    Pack { name: "SPECTRAL_PACK", weight: 0.6, size: 2, card_type: PackCardType::Spectral, key: "spe" },
    Pack { name: "JUMBO_SPECTRAL_PACK", weight: 0.3, size: 4, card_type: PackCardType::Spectral, key: "spe" },
    Pack { name: "MEGA_SPECTRAL_PACK", weight: 0.07, size: 4, card_type: PackCardType::Spectral, key: "spe" }
];

static TOTAL_WEIGHT: f64 = 22.42;

#[derive(Debug, Clone, Copy)]
pub enum PackCardType {
    Tarot,
    Joker,
    Spectral,
    Planet,
    Card
}

#[derive(Debug)]
pub enum CardType {
    Tarot(Tarot),
    Joker(Joker),
    Spectral(Spectral),
    Planet(Planet),
    Card(Card)
}


impl Pack {

    pub fn get_random_pack(random_state: &mut RandomState, first_pack: bool) -> Pack {
        if first_pack {
            return WEIGHTED_PACKS[BUFFOON_PACK_INDEX];
        } else {
    
            let seed = random::concat_strings(&["shop_pack", &random_state.ante.to_string(), &random_state.seed]);
            let mut db = random_state.random_double(&seed);
    
            db *= TOTAL_WEIGHT;
    
            let mut accumulated_weight = 0.0;
    
            if let Some(pack) = WEIGHTED_PACKS.iter().enumerate().find(|(_, p)| {
                accumulated_weight += p.weight;
                accumulated_weight >= db
            }) {    
                *pack.1
            } else {
                WEIGHTED_PACKS[0]
            }
        }
    }

    pub fn open(&mut self, random_state: &mut RandomState) -> Vec<CardType> {

        let mut cards = Vec::with_capacity(self.size);

        for _i in 0..self.size {
            let card = self.get_random_card(random_state, Rarity::COMMON);
            cards.push(card);
        }

        cards
    }

    pub fn get_random_card(&mut self, random_state: &mut RandomState, rarity: Rarity) -> CardType {
        println!("{:?}", self.card_type);
        match self.card_type {
            PackCardType::Tarot => CardType::Tarot(Tarot::get_random(random_state)),
            PackCardType::Joker => CardType::Joker(Joker::get_random(random_state, rarity, self.key)),
            PackCardType::Spectral => CardType::Spectral(Spectral::get_random(random_state)),
            PackCardType::Planet => CardType::Planet(Planet::get_random(random_state)),
            PackCardType::Card => CardType::Card(Card::get_random(random_state, self.key)),
            _ => panic!("No card type found from opening pack!!!")
        }
    }

    pub fn get_card(&mut self, index: usize, rarity: Rarity) -> CardType {
        match self.card_type {
            PackCardType::Tarot => CardType::Tarot(Tarot::from_number(index)),
            PackCardType::Joker => CardType::Joker(Joker::from_number(index, rarity)),
            PackCardType::Spectral => CardType::Spectral(Spectral::from_number(index)),
            PackCardType::Planet => CardType::Planet(Planet::from_number(index)),
            PackCardType::Card => CardType::Card(Card::from_number(index)),
            _ => panic!("No card type found from opening pack!!!")
        }
    }
}