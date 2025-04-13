mod voucher_data;
use crate::game::random as game_random;
use crate::game::state;
use crate::random;
use std::fmt;

static TYPE_STR: &'static str = "Voucher";
static TAG_TYPE_STR: &'static str = "Voucher_fromtag";

#[derive(Debug, Clone, Copy)]
pub struct Voucher {
    name: &'static str,
    locked: bool,
    owned: bool
}

impl Voucher {
    pub fn new (name: &'static str, locked: bool, owned: bool) -> Self {
        Voucher { name, locked, owned }
    }

    pub fn from_number(index: usize) -> Self {
        Voucher { name: Voucher::get_voucher(index), locked: false, owned: false }
    }

    pub fn default() -> Self {
        Voucher { name: Voucher::get_voucher(0), locked: false, owned: false }
    }

    pub fn get_voucher(index: usize) -> &'static str {
        voucher_data::VOUCHER_CARDS[index]
    }
}

pub struct VoucherArray ([Voucher; 32]);

impl VoucherArray {
    pub fn new() -> Self {
        let mut arr: [Voucher; 32] = [Voucher::default(); 32];

        for i in 0..voucher_data::VOUCHER_CARDS.len() {
            arr[i] = Voucher::new(Voucher::get_voucher(i), i % 2 != 0, false)
        }

        VoucherArray (arr)
    }

    pub fn random(&mut self, random_state: &mut state::RandomState, from_tag: bool) -> Voucher {
        let mut voucher_index = self.get_random(random_state, from_tag, "");
        let mut random_voucher = self.get(voucher_index);

        let mut resample_count = 1;
        while random_voucher.owned || random_voucher.locked {
            println!("Voucher owned or locked {} | {:?}", voucher_index, random_voucher);
            voucher_index = self.get_random(random_state, from_tag, &game_random::concat_strings(&["_resample", &resample_count.to_string()]));
            random_voucher = self.get(voucher_index);
            resample_count += 1;
        }

        self.get_mut(voucher_index).owned = true;
        println!("Unlocked: {} | {:?}", voucher_index, self.get(voucher_index));

        if voucher_index <= (VoucherArray::get_max() as usize) {
            if self.get(voucher_index + 1).locked {
                self.unlock(voucher_index + 1);
            }
        }

        random_voucher
    }

    fn get_random(&mut self, random_state: &mut state::RandomState, from_tag: bool, resample_key: &str) -> usize {
        let mut type_str = TYPE_STR;
        if from_tag { type_str = TAG_TYPE_STR; }

        let state_seed = game_random::concat_strings(&[type_str, &random_state.ante.to_string(), resample_key, &random_state.seed]);
        let mut state = random::random_state_from_seed(random_state.get_node(&state_seed));

        let voucher_index = random::random_int(&mut state, 0.0, VoucherArray::get_max()) as usize;

        voucher_index
    }

    pub fn get(&mut self, index: usize) -> Voucher {
        self.0[index]
    }

    pub fn get_mut(&mut self, index: usize) -> &mut Voucher {
        &mut self.0[index]
    }

    pub fn unlock(&mut self, index: usize) {
        self.get_mut(index).locked = false
    }

    pub fn get_max() -> f64 {
        (voucher_data::VOUCHER_CARDS.len() - 1) as f64
    }
}

impl fmt::Debug for VoucherArray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "VoucherArray [")?;
        for voucher in &self.0 {
            let locked_str = if voucher.locked { " 🔒" } else { "" };
            let owned_str = if voucher.owned { " ✅" } else { "" };
            writeln!(f, "  - {}{}{}", voucher.name, locked_str, owned_str)?;
        }
        write!(f, "]")
    }
}