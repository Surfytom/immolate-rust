use crate::game::state;
use crate::game::random as game_random;
use crate::random;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
pub enum Packs {
    ARCANA_PACK,
    JUMBO_ARCANA_PACK,
    MEGA_ARCANA_PACK,
    CELESTIAL_PACK,
    JUMBO_CELESTIAL_PACK,
    MEGA_CELESTIAL_PACK,
    STANDARD_PACK,
    JUMBO_STANDARD_PACK,
    MEGA_STANDARD_PACK,
    BUFFOON_PACK,
    JUMBO_BUFFOON_PACK,
    MEGA_BUFFOON_PACK,
    SPECTRAL_PACK,
    JUMBO_SPECTRAL_PACK,
    MEGA_SPECTRAL_PACK,
}

#[derive(Clone, Copy)]
pub struct Pack {
    pub pack: Packs,
    weight: f64,
    size: i32,
    type_key: &'static str,
    key: &'static str
}

static WEIGHTED_PACKS: [Pack; 15] = [
    Pack { pack: Packs::ARCANA_PACK, weight: 4.0, size: 3, type_key: "Tarot", key: "ar1" },
    Pack { pack: Packs::JUMBO_ARCANA_PACK, weight: 2.0, size: 5, type_key: "Tarot", key: "ar1" },
    Pack { pack: Packs::MEGA_ARCANA_PACK, weight: 0.5, size: 5, type_key: "Tarot", key: "ar1" },
    Pack { pack: Packs::CELESTIAL_PACK, weight: 4.0, size: 3, type_key: "Planet", key: "pl1" },
    Pack { pack: Packs::JUMBO_CELESTIAL_PACK, weight: 2.0, size: 5, type_key: "Planet", key: "pl1" },
    Pack { pack: Packs::MEGA_CELESTIAL_PACK, weight: 0.5, size: 5, type_key: "Planet", key: "pl1" },
    Pack { pack: Packs::STANDARD_PACK, weight: 4.0, size: 3, type_key: "stdset", key: "sta" },
    Pack { pack: Packs::JUMBO_STANDARD_PACK, weight: 2.0, size: 5, type_key: "stdset", key: "sta" },
    Pack { pack: Packs::MEGA_STANDARD_PACK, weight: 0.5, size: 5, type_key: "stdset", key: "sta" },
    Pack { pack: Packs::BUFFOON_PACK, weight: 1.2, size: 2, type_key: "Joker", key: "buf" },
    Pack { pack: Packs::JUMBO_BUFFOON_PACK, weight: 0.6, size: 4, type_key: "Joker", key: "buf" },
    Pack { pack: Packs::MEGA_BUFFOON_PACK, weight: 0.15, size: 4, type_key: "Joker", key: "buf" },
    Pack { pack: Packs::SPECTRAL_PACK, weight: 0.6, size: 2, type_key: "Spectral", key: "spe" },
    Pack { pack: Packs::JUMBO_SPECTRAL_PACK, weight: 0.3, size: 4, type_key: "Spectral", key: "spe" },
    Pack { pack: Packs::MEGA_SPECTRAL_PACK, weight: 0.07, size: 4, type_key: "Spectral", key: "spe" },
];

static TOTAL_WEIGHT: f64 = 22.42;

fn get_min_max() -> (Packs, Packs) {
    (Packs::ARCANA_PACK, Packs::MEGA_SPECTRAL_PACK)
}

pub fn get_random_pack(random_state: &mut state::RandomState, ante: u32, first_pack: bool) -> Pack {
    if first_pack {
        return WEIGHTED_PACKS[Packs::BUFFOON_PACK as usize];
    } else {

        let combined_seed = game_random::concat_strings(&["shop_pack", &ante.to_string(), &random_state.seed]);

        let mut state = random::random_state_from_seed(random_state.get_node(&combined_seed));
        let mut dl = random::random_double(&mut state);

        unsafe { dl.d *= TOTAL_WEIGHT };

        let mut accumulated_weight = 0.0;

        if let Some(pack) = WEIGHTED_PACKS.iter().enumerate().find(|(_, p)| {
            accumulated_weight += p.weight;
            accumulated_weight >= unsafe { dl.d }
        }) {    
            *pack.1
        } else {
            WEIGHTED_PACKS[0]
        }
    }
}