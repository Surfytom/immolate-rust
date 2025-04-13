use crate::game::state;
use crate::game::random as game_random;
use crate::random;

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum CommonJokers {
    JOKER,
    GREEDY_JOKER,
    LUSTY_JOKER,
    WRATHFUL_JOKER,
    GLUTTENOUS_JOKER,
    JOLLY,
    ZANY,
    MAD,
    CRAZY,
    DROLL,
    SLY,
    WILY,
    CLEVER,
    DEVIOUS,
    CRAFTY,
    HALF,
    CREDIT_CARD,
    BANNER,
    MYSTIC_SUMMIT,
    EIGHT_BALL,
    MISPRINT,
    RAISED_FIST,
    CHAOS,
    SCARY_FACE,
    ABSTRACT,
    DELAYED_GRAT,
    GROS_MICHEL,
    EVEN_STEVEN,
    ODD_TODD,
    SCHOLAR,
    BUSINESS,
    SUPERNOVA,
    RIDE_THE_BUS,
    EGG,
    RUNNER,
    ICE_CREAM,
    SPLASH,
    BLUE_JOKER,
    FACELESS,
    GREEN_JOKER,
    SUPERPOSITION,
    TODO_LIST,
    CAVENDISH,
    RED_CARD,
    SQUARE,
    RIFF_RAFF,
    PHOTOGRAPH,
    RESERVED_PARKING,
    MAIL,
    HALLUCINATION,
    FORTUNE_TELLER,
    JUGGLER,
    DRUNKARD,
    GOLDEN,
    POPCORN,
    WALKIE_TALKIE,
    SMILEY,
    TICKET,
    SWASHBUCKLER,
    HANGING_CHAD,
    SHOOT_THE_MOON
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum UncommonJokers {
    STENCIL,
    FOUR_FINGERS,
    MIME,
    CEREMONIAL,
    MARBLE,
    LOYALTY_CARD,
    DUSK,
    FIBONACCI,
    STEEL_JOKER,
    HACK,
    PAREIDOLIA,
    SPACE,
    BURGLAR,
    BLACKBOARD,
    SIXTH_SENSE,
    CONSTELLATION,
    HIKER,
    CARD_SHARP,
    MADNESS,
    SEANCE,
    VAMPIRE,
    SHORTCUT,
    HOLOGRAM,
    CLOUD_9,
    ROCKET,
    MIDAS_MASK,
    LUCHADOR,
    GIFT,
    TURTLE_BEAN,
    EROSION,
    TO_THE_MOON,
    STONE,
    LUCKY_CAT,
    BULL,
    DIET_COLA,
    TRADING,
    FLASH,
    TROUSERS,
    RAMEN,
    SELZER,
    CASTLE,
    MR_BONES,
    ACROBAT,
    SOCK_AND_BUSKIN,
    TROUBADOUR,
    CERTIFICATE,
    SMEARED,
    THROWBACK,
    ROUGH_GEM,
    BLOODSTONE,
    ARROWHEAD,
    ONYX_AGATE,
    GLASS,
    RING_MASTER,
    FLOWER_POT,
    MERRY_ANDY,
    OOPS,
    IDOL,
    SEEING_DOUBLE,
    MATADOR,
    SATELLITE,
    CARTOMANCER,
    ASTRONOMER,
    BOOTSTRAPS
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum RareJokers {
    DNA,
    VAGABOND,
    BARON,
    OBELISK,
    BASEBALL,
    ANCIENT,
    CAMPFIRE,
    BLUEPRINT,
    WEE,
    HIT_THE_ROAD,
    DUO,
    TRIO,
    FAMILY,
    ORDER,
    TRIBE,
    STUNTMAN,
    INVISIBLE,
    BRAINSTORM,
    DRIVERS_LICENSE,
    BURNT
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum LegendaryJokers {
    CAINO,
    TRIBOULET,
    YORICK,
    CHICOT,
    PERKEO
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
pub enum Rarity {
    COMMON,
    UNCOMMON,
    RARE,
    LEGENDARY
}

#[derive(Debug)]
pub enum JokerName {
    Common(CommonJokers),
    Uncommon(UncommonJokers),
    Rare(RareJokers),
    Legendary(LegendaryJokers)
}

#[derive(Debug)]
pub struct Joker {
    pub rarity: Rarity,
    pub joker: JokerName
}

impl Joker {
    pub fn new(joker: JokerName, rarity: Rarity) -> Self {
        Joker { rarity, joker }
    }

    pub fn random(random_state: &mut state::RandomState, rarity: Rarity, ante: u32, key: &str) -> Joker {
        let (key, random_rarity, min, max) = Joker::get_pool(random_state, rarity, ante, key);

        let mut state = random::random_state_from_seed(random_state.get_node(&key));

        let joker_number = random::random_int(&mut state, min, max) as u32;

        let joker_variant = match random_rarity {
            Rarity::COMMON => JokerName::Common(CommonJokers::from_u32(joker_number)),
            Rarity::UNCOMMON => JokerName::Uncommon(UncommonJokers::from_u32(joker_number)),
            Rarity::RARE => JokerName::Rare(RareJokers::from_u32(joker_number)),
            Rarity::LEGENDARY => JokerName::Legendary(LegendaryJokers::from_u32(joker_number))
        };

        Joker::new(joker_variant, random_rarity)
    }

    pub fn get_pool(random_state: &mut state::RandomState, rarity: Rarity, ante: u32, key: &str) -> (String, Rarity, f64, f64) {

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

        let pool_key = game_random::concat_strings(&["Joker", &Joker::get_rarity_string(&random_rarity), key, &ante.to_string(), &random_state.seed]);
        let (min, max) = Joker::get_min_max(&random_rarity);

        (pool_key, random_rarity, min, max)
    }

    pub fn get_min_max(rarity: &Rarity) -> (f64, f64) {
        match rarity {
            Rarity::COMMON => (
                CommonJokers::JOKER as u8 as f64,
                CommonJokers::SHOOT_THE_MOON as u8 as f64
            ),
            Rarity::UNCOMMON => (
                UncommonJokers::STENCIL as u8 as f64,
                UncommonJokers::BOOTSTRAPS as u8 as f64
            ),
            Rarity::RARE => (
                RareJokers::DNA as u8 as f64,
                RareJokers::BURNT as u8 as f64
            ),
            Rarity::LEGENDARY => (
                LegendaryJokers::CAINO as u8 as f64,
                LegendaryJokers::PERKEO as u8 as f64
            )
        }
    }

    pub fn get_rarity_string(rarity: &Rarity) -> String {
        match rarity {
            Rarity::COMMON => "1".to_string(),
            Rarity::UNCOMMON => "2".to_string(),
            Rarity::RARE => "3".to_string(),
            Rarity::LEGENDARY => "4".to_string()
        }
    }
}

impl CommonJokers {
    fn from_u32(value: u32) -> CommonJokers {
        match value {
            0 => CommonJokers::JOKER,
            1 => CommonJokers::GREEDY_JOKER,
            2 => CommonJokers::LUSTY_JOKER,
            3 => CommonJokers::WRATHFUL_JOKER,
            4 => CommonJokers::GLUTTENOUS_JOKER,
            5 => CommonJokers::JOLLY,
            6 => CommonJokers::ZANY,
            7 => CommonJokers::MAD,
            8 => CommonJokers::CRAZY,
            9 => CommonJokers::DROLL,
            10 => CommonJokers::SLY,
            11 => CommonJokers::WILY,
            12 => CommonJokers::CLEVER,
            13 => CommonJokers::DEVIOUS,
            14 => CommonJokers::CRAFTY,
            15 => CommonJokers::HALF,
            16 => CommonJokers::CREDIT_CARD,
            17 => CommonJokers::BANNER,
            18 => CommonJokers::MYSTIC_SUMMIT,
            19 => CommonJokers::EIGHT_BALL,
            20 => CommonJokers::MISPRINT,
            21 => CommonJokers::RAISED_FIST,
            22 => CommonJokers::CHAOS,
            23 => CommonJokers::SCARY_FACE,
            24 => CommonJokers::ABSTRACT,
            25 => CommonJokers::DELAYED_GRAT,
            26 => CommonJokers::GROS_MICHEL,
            27 => CommonJokers::EVEN_STEVEN,
            28 => CommonJokers::ODD_TODD,
            29 => CommonJokers::SCHOLAR,
            30 => CommonJokers::BUSINESS,
            31 => CommonJokers::SUPERNOVA,
            32 => CommonJokers::RIDE_THE_BUS,
            33 => CommonJokers::EGG,
            34 => CommonJokers::RUNNER,
            35 => CommonJokers::ICE_CREAM,
            36 => CommonJokers::SPLASH,
            37 => CommonJokers::BLUE_JOKER,
            38 => CommonJokers::FACELESS,
            39 => CommonJokers::GREEN_JOKER,
            40 => CommonJokers::SUPERPOSITION,
            41 => CommonJokers::TODO_LIST,
            42 => CommonJokers::CAVENDISH,
            43 => CommonJokers::RED_CARD,
            44 => CommonJokers::SQUARE,
            45 => CommonJokers::RIFF_RAFF,
            46 => CommonJokers::PHOTOGRAPH,
            47 => CommonJokers::RESERVED_PARKING,
            48 => CommonJokers::MAIL,
            49 => CommonJokers::HALLUCINATION,
            50 => CommonJokers::FORTUNE_TELLER,
            51 => CommonJokers::JUGGLER,
            52 => CommonJokers::DRUNKARD,
            53 => CommonJokers::GOLDEN,
            54 => CommonJokers::POPCORN,
            55 => CommonJokers::WALKIE_TALKIE,
            56 => CommonJokers::SMILEY,
            57 => CommonJokers::TICKET,
            58 => CommonJokers::SWASHBUCKLER,
            59 => CommonJokers::HANGING_CHAD,
            60 => CommonJokers::SHOOT_THE_MOON,
            _ => CommonJokers::JOKER, // Default case
        }
    }
}

impl UncommonJokers {
    fn from_u32(value: u32) -> UncommonJokers {
        match value {
            0 => UncommonJokers::STENCIL,
            1 => UncommonJokers::FOUR_FINGERS,
            2 => UncommonJokers::MIME,
            3 => UncommonJokers::CEREMONIAL,
            4 => UncommonJokers::MARBLE,
            5 => UncommonJokers::LOYALTY_CARD,
            6 => UncommonJokers::DUSK,
            7 => UncommonJokers::FIBONACCI,
            8 => UncommonJokers::STEEL_JOKER,
            9 => UncommonJokers::HACK,
            10 => UncommonJokers::PAREIDOLIA,
            11 => UncommonJokers::SPACE,
            12 => UncommonJokers::BURGLAR,
            13 => UncommonJokers::BLACKBOARD,
            14 => UncommonJokers::SIXTH_SENSE,
            15 => UncommonJokers::CONSTELLATION,
            16 => UncommonJokers::HIKER,
            17 => UncommonJokers::CARD_SHARP,
            18 => UncommonJokers::MADNESS,
            19 => UncommonJokers::SEANCE,
            20 => UncommonJokers::VAMPIRE,
            21 => UncommonJokers::SHORTCUT,
            22 => UncommonJokers::HOLOGRAM,
            23 => UncommonJokers::CLOUD_9,
            24 => UncommonJokers::ROCKET,
            25 => UncommonJokers::MIDAS_MASK,
            26 => UncommonJokers::LUCHADOR,
            27 => UncommonJokers::GIFT,
            28 => UncommonJokers::TURTLE_BEAN,
            29 => UncommonJokers::EROSION,
            30 => UncommonJokers::TO_THE_MOON,
            31 => UncommonJokers::STONE,
            32 => UncommonJokers::LUCKY_CAT,
            33 => UncommonJokers::BULL,
            34 => UncommonJokers::DIET_COLA,
            35 => UncommonJokers::TRADING,
            36 => UncommonJokers::FLASH,
            37 => UncommonJokers::TROUSERS,
            38 => UncommonJokers::RAMEN,
            39 => UncommonJokers::SELZER,
            40 => UncommonJokers::CASTLE,
            41 => UncommonJokers::MR_BONES,
            42 => UncommonJokers::ACROBAT,
            43 => UncommonJokers::SOCK_AND_BUSKIN,
            44 => UncommonJokers::TROUBADOUR,
            45 => UncommonJokers::CERTIFICATE,
            46 => UncommonJokers::SMEARED,
            47 => UncommonJokers::THROWBACK,
            48 => UncommonJokers::ROUGH_GEM,
            49 => UncommonJokers::BLOODSTONE,
            50 => UncommonJokers::ARROWHEAD,
            51 => UncommonJokers::ONYX_AGATE,
            52 => UncommonJokers::GLASS,
            53 => UncommonJokers::RING_MASTER,
            54 => UncommonJokers::FLOWER_POT,
            55 => UncommonJokers::MERRY_ANDY,
            56 => UncommonJokers::OOPS,
            57 => UncommonJokers::IDOL,
            58 => UncommonJokers::SEEING_DOUBLE,
            59 => UncommonJokers::MATADOR,
            60 => UncommonJokers::SATELLITE,
            61 => UncommonJokers::CARTOMANCER,
            62 => UncommonJokers::ASTRONOMER,
            63 => UncommonJokers::BOOTSTRAPS,
            _ => UncommonJokers::STENCIL, // Default case
        }
    }
}

impl RareJokers {
    fn from_u32(value: u32) -> RareJokers {
        match value {
            0 => RareJokers::DNA,
            1 => RareJokers::VAGABOND,
            2 => RareJokers::BARON,
            3 => RareJokers::OBELISK,
            4 => RareJokers::BASEBALL,
            5 => RareJokers::ANCIENT,
            6 => RareJokers::CAMPFIRE,
            7 => RareJokers::BLUEPRINT,
            8 => RareJokers::WEE,
            9 => RareJokers::HIT_THE_ROAD,
            10 => RareJokers::DUO,
            11 => RareJokers::TRIO,
            12 => RareJokers::FAMILY,
            13 => RareJokers::ORDER,
            14 => RareJokers::TRIBE,
            15 => RareJokers::STUNTMAN,
            16 => RareJokers::INVISIBLE,
            17 => RareJokers::BRAINSTORM,
            18 => RareJokers::DRIVERS_LICENSE,
            19 => RareJokers::BURNT,
            _ => RareJokers::DNA, // Default case
        }
    }
}

impl LegendaryJokers {
    fn from_u32(value: u32) -> LegendaryJokers {
        match value {
            0 => LegendaryJokers::CAINO,
            1 => LegendaryJokers::TRIBOULET,
            2 => LegendaryJokers::YORICK,
            3 => LegendaryJokers::CHICOT,
            4 => LegendaryJokers::PERKEO,
            _ => LegendaryJokers::CAINO, // Default case
        }
    }
}