// #![feature(wrapping_int_impl)]
// use std::num::Wrapping;

union DoubleLong {
    d: f64,
    ul: u64
}

fn get_double(dl: &DoubleLong) -> f64 {
    unsafe { dl.d }
}

fn get_uint(dl: &DoubleLong) -> u64 {
    unsafe { dl.ul }
}

struct LuaRandom {
    state: [u64; 4],
    out: DoubleLong
}

fn fract(double: f64) -> f64 {
    double - double.floor()
}

fn hash_string(s: &str) -> f64 {
    let mut num: f64 = 1.0;

    s.chars().rev().enumerate().for_each(| (i, c) | { 

        let reverse_index: f64 = ((s.len() - 1) - i) as f64;

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

        let int_part: i64 = ((1.1239285023 / num) * (c as u32 as f64) * 3.14159265358979323846 + 3.14159265358979323846 * (reverse_index + 1.0)) as i64;
        println!("int part: {:?}", int_part);

        let fract_part: f64 = fract(fract(1.1239285023 / num * (c as u32 as f64) * 3.14159265358979323846) + fract(3.14159265358979323846 * (reverse_index + 1.0))); 
        println!("fract part: {:.25}", fract_part);

        num = fract((int_part as f64) + fract_part);
        println!("num: {:.25}", num);
    }); 

    return num;
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

fn randomise_state(state: &mut [u64; 4]) {

    let mut z: u64 = 0;
    let mut r = DoubleLong { d: 0.0 };

    randomise_state_step(state, &mut z, &mut r, 0, 63, 31, 18);
    randomise_state_step(state, &mut z, &mut r, 1, 58, 19, 28);
	randomise_state_step(state, &mut z, &mut r, 2, 55, 24, 7);
	randomise_state_step(state, &mut z, &mut r, 3, 47, 21, 8);
}

fn random_state_from_seed(seed: f64) -> [u64; 4] {
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

fn main() {

    let lr = LuaRandom { 
        state: [0, 0, 0, 0],
        out: DoubleLong {
            d: 1.0 
        }
    };

    println!("LuaRandom: state: {:?}, double: {}, uint: {}", lr.state, get_double(&lr.out), get_uint(&lr.out));

    let hashed_seed = hash_string("BB");
    let starting_state = random_state_from_seed(hashed_seed);
    println!("starting state: {:?}", starting_state);
}
