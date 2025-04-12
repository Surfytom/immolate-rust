static DEBUG: bool = false;

pub union DoubleLong {
    pub d: f64,
    pub ul: u64
}

struct LuaRandom {
    state: [u64; 4],
    out: DoubleLong
}

pub fn fract(double: f64) -> f64 {
    double - double.floor()
}

pub fn round_double(double: &f64, round_to: u32) -> f64 {
    let base: i64 = 10;
    let pow = base.pow(round_to) as f64;
    println!("power: {:.25}", pow);
    let t = double * pow;
    println!("t: {:.25}", t);

    let mut g = 0.0;
    if t > t.floor() + 0.5 {
        g = t.ceil();
    } else {
        g = t.floor();
    }

    println!("rounded int: {:.25}", g);
    println!("rounded double: {:.25}", g / pow);

    g / pow
}

pub fn seed_from_string(s: &str) -> f64 {
    let mut num: f64 = 1.0;

    s.chars().rev().enumerate().for_each(| (i, c) | { 
        let reverse_index: f64 = ((s.len() - 1) - i) as f64;

        let int_part: i64 = ((1.1239285023 / num) * (c as u32 as f64) * 3.14159265358979323846 + 3.14159265358979323846 * (reverse_index + 1.0)) as i64;
        let fract_part: f64 = fract(fract(1.1239285023 / num * (c as u32 as f64) * 3.14159265358979323846) + fract(3.14159265358979323846 * (reverse_index + 1.0))); 
        num = fract((int_part as f64) + fract_part);

        if DEBUG {
            println!("char {}: {}", i, c);
            let x1: f64 = 1.1239285469055175781250000 / num;
            println!("x1: {:.25}", x1);
            let x2 = x1 * (c as u8 as f64);
            println!("x2: {:.25}", x2);
            let x3 = x2 * 3.14159265358979323846 + 3.14159265358979323846;
            println!("x3: {:.25}", x3);
            let x4 = x3 * (reverse_index + 1.0);
            println!("x4: {:.25}", x4);
            let x4b = x4 as i64;
            println!("x4b: {}", x4b);

            println!("int part: {:?}", int_part);
            println!("fract part: {:.25}", fract_part);
            println!("num: {:.25}", num);
        }
    }); 

    num
}

fn randomise_state_step(state: &mut [u64; 4], z: &mut u64, dl: &mut DoubleLong, i: usize, k: i32, q: i32, s: i32) {
	*z = state[i];

	let x1: u64 = *z << q;
	let x2: u64 = x1 ^ *z;
	let x3: u64 = x2 >> (k - s);
	let x4: u64 = (-1i64 << (64 - k)) as u64;
	let x5: u64 = *z & x4;
	let x6: u64 = x5 << s;
	let x7: u64 = x3 ^ x6;

	unsafe { dl.ul ^= x7 };
	state[i] = x7;
    *z = x7;
}

fn randomise_state(state: &mut [u64; 4]) -> DoubleLong {

    let mut z: u64 = 0;
    let mut r = DoubleLong { d: 0.0 };

    randomise_state_step(state, &mut z, &mut r, 0, 63, 31, 18);
    randomise_state_step(state, &mut z, &mut r, 1, 58, 19, 28);
	randomise_state_step(state, &mut z, &mut r, 2, 55, 24, 7);
	randomise_state_step(state, &mut z, &mut r, 3, 47, 21, 8);

    r
}

pub fn random_state_from_seed(seed: f64) -> [u64; 4] {
    let mut mut_seed = seed.clone();
    let mut r: u32 = 0x11090601;

    let mut u = DoubleLong { d: 0.0 };
    let mut state: [u64; 4] = [0, 0, 0, 0];

    state = state.map(| _num | {
        let m = 1u32 << (r & 255u32);
        u.d = 0.0;
        r = r.rotate_right(8);

        mut_seed *= 3.14159265358979323846;
		mut_seed += 2.7182818284590452354;

        u.d = mut_seed;

        if (unsafe { u.ul } < (m as u64)) { unsafe { u.ul += m as u64 }; }

        unsafe { u.ul }
    });

    println!("state: {:?}", state);

    for _i in 0..10 {
        randomise_state(&mut state);
    }

    state
}

fn random_unsigned_int(state: &mut [u64; 4]) -> DoubleLong {
    let mut dl = randomise_state(state);

    unsafe { dl.ul = (dl.ul & 4503599627370495) | 4607182418800017408 };

    dl
}

pub fn random_double(state: &mut [u64; 4]) -> DoubleLong {
    let mut r: DoubleLong = random_unsigned_int(state);

    unsafe { r.d = r.d - 1.0 };
    
    r
}

pub fn random_int(state: &mut [u64; 4], min: f64, max: f64) -> i64 {
    let r = random_double(state);

    unsafe { ((r.d * (max - min + 1.0)) + min) as i64 }
}