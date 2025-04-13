use crate::game::state;
use crate::game::random as game_random;
use crate::random;

pub static COMMON_JOKERS: [&'static str; 61] = [
    "JOKER",
    "GREEDY_JOKER",
    "LUSTY_JOKER",
    "WRATHFUL_JOKER",
    "GLUTTENOUS_JOKER",
    "JOLLY",
    "ZANY",
    "MAD",
    "CRAZY",
    "DROLL",
    "SLY",
    "WILY",
    "CLEVER",
    "DEVIOUS",
    "CRAFTY",
    "HALF",
    "CREDIT_CARD",
    "BANNER",
    "MYSTIC_SUMMIT",
    "EIGHT_BALL",
    "MISPRINT",
    "RAISED_FIST",
    "CHAOS",
    "SCARY_FACE",
    "ABSTRACT",
    "DELAYED_GRAT",
    "GROS_MICHEL",
    "EVEN_STEVEN",
    "ODD_TODD",
    "SCHOLAR",
    "BUSINESS",
    "SUPERNOVA",
    "RIDE_THE_BUS",
    "EGG",
    "RUNNER",
    "ICE_CREAM",
    "SPLASH",
    "BLUE_JOKER",
    "FACELESS",
    "GREEN_JOKER",
    "SUPERPOSITION",
    "TODO_LIST",
    "CAVENDISH",
    "RED_CARD",
    "SQUARE",
    "RIFF_RAFF",
    "PHOTOGRAPH",
    "RESERVED_PARKING",
    "MAIL",
    "HALLUCINATION",
    "FORTUNE_TELLER",
    "JUGGLER",
    "DRUNKARD",
    "GOLDEN",
    "POPCORN",
    "WALKIE_TALKIE",
    "SMILEY",
    "TICKET",
    "SWASHBUCKLER",
    "HANGING_CHAD",
    "SHOOT_THE_MOON"
];

pub static UNCOMMON_JOKERS: [&'static str; 64] = [
    "STENCIL",
    "FOUR_FINGERS",
    "MIME",
    "CEREMONIAL",
    "MARBLE",
    "LOYALTY_CARD",
    "DUSK",
    "FIBONACCI",
    "STEEL_JOKER",
    "HACK",
    "PAREIDOLIA",
    "SPACE",
    "BURGLAR",
    "BLACKBOARD",
    "SIXTH_SENSE",
    "CONSTELLATION",
    "HIKER",
    "CARD_SHARP",
    "MADNESS",
    "SEANCE",
    "VAMPIRE",
    "SHORTCUT",
    "HOLOGRAM",
    "CLOUD_9",
    "ROCKET",
    "MIDAS_MASK",
    "LUCHADOR",
    "GIFT",
    "TURTLE_BEAN",
    "EROSION",
    "TO_THE_MOON",
    "STONE",
    "LUCKY_CAT",
    "BULL",
    "DIET_COLA",
    "TRADING",
    "FLASH",
    "TROUSERS",
    "RAMEN",
    "SELZER",
    "CASTLE",
    "MR_BONES",
    "ACROBAT",
    "SOCK_AND_BUSKIN",
    "TROUBADOUR",
    "CERTIFICATE",
    "SMEARED",
    "THROWBACK",
    "ROUGH_GEM",
    "BLOODSTONE",
    "ARROWHEAD",
    "ONYX_AGATE",
    "GLASS",
    "RING_MASTER",
    "FLOWER_POT",
    "MERRY_ANDY",
    "OOPS",
    "IDOL",
    "SEEING_DOUBLE",
    "MATADOR",
    "SATELLITE",
    "CARTOMANCER",
    "ASTRONOMER",
    "BOOTSTRAPS"
];

pub static RARE_JOKERS: [&'static str; 20] = [
    "DNA",
    "VAGABOND",
    "BARON",
    "OBELISK",
    "BASEBALL",
    "ANCIENT",
    "CAMPFIRE",
    "BLUEPRINT",
    "WEE",
    "HIT_THE_ROAD",
    "DUO",
    "TRIO",
    "FAMILY",
    "ORDER",
    "TRIBE",
    "STUNTMAN",
    "INVISIBLE",
    "BRAINSTORM",
    "DRIVERS_LICENSE",
    "BURNT"
];

pub static LEGENDARY_JOKERS: [&'static str; 5] = [
    "CAINO",
    "TRIBOULET",
    "YORICK",
    "CHICOT",
    "PERKEO"
];

pub static JOKER_ARRAYS: [&'static [&'static str]; 4] = [
    &COMMON_JOKERS,
    &UNCOMMON_JOKERS,
    &RARE_JOKERS,
    &LEGENDARY_JOKERS
];

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
pub enum Rarity {
    COMMON,
    UNCOMMON,
    RARE,
    LEGENDARY
}

impl Rarity {
    fn as_usize(&self) -> usize {
        match self {
            Rarity::COMMON => 0,
            Rarity::UNCOMMON => 1,
            Rarity::RARE => 2,
            Rarity::LEGENDARY => 3,
        }
    }

    fn as_string(&self) -> &str {
        match self {
            Rarity::COMMON => "1",
            Rarity::UNCOMMON => "2",
            Rarity::RARE => "3",
            Rarity::LEGENDARY => "4",
        }
    }

    fn get_max(&self) -> f64 {
        (JOKER_ARRAYS[self.as_usize()].len() - 1) as f64
    }
}


#[derive(Debug)]
pub struct Joker {
    pub rarity: Rarity,
    pub name: &'static str
}

impl Joker {
    pub fn new(name: &'static str, rarity: Rarity) -> Self {
        Joker { name, rarity }
    }

    pub fn from_number(num: usize, rarity: Rarity) -> Self {
        Joker { name: JOKER_ARRAYS[rarity.as_usize()][num], rarity }
    }

    pub fn random(random_state: &mut state::RandomState, rarity: Rarity, ante: u32, key: &str) -> Joker {
        let (key, random_rarity) = Joker::get_pool(random_state, rarity, ante, key);

        let mut state = random::random_state_from_seed(random_state.get_node(&key));

        let joker_number = random::random_int(&mut state, 0.0, random_rarity.get_max()) as usize;

        Joker::from_number(joker_number, random_rarity)
    }

    pub fn get_pool(random_state: &mut state::RandomState, rarity: Rarity, ante: u32, key: &str) -> (String, Rarity) {

        let mut random_rarity = Rarity::LEGENDARY;

        if rarity == Rarity::RARE {
            random_rarity = Rarity::RARE;
        }

        // Common here means just roll for a joker like normal
        if rarity == Rarity::COMMON {

            let seed_string = game_random::concat_strings(&["rarity", &ante.to_string(), key, &random_state.seed]);

            let mut state = random::random_state_from_seed(random_state.get_node(&seed_string));

            let dl = random::random_double(&mut state);
            let d = unsafe { dl.d };

            if (d > 0.95) {
                random_rarity = Rarity::RARE;
            } 
            else if d > 0.7 {
                random_rarity = Rarity::UNCOMMON;
            }
            else {
                random_rarity = Rarity::COMMON;
            }
        }

        let pool_key = game_random::concat_strings(&["Joker", random_rarity.as_string(), key, &ante.to_string(), &random_state.seed]);

        (pool_key, random_rarity)
    }
}