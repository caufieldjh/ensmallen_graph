use super::types::*;
use ::core::cmp::Ordering;


// WARNING
// the initializzation only works in release mode because of
// overflow checking. We could consider to disable it in the Cargo.toml.
// the suggested method to initialize the seed of xorshiro
fn splitmix64(x: u64) -> u64 {
    // http://prng.di.unimi.it/splitmix64.c
	let mut z: u64 = x + 0x9e3779b97f4a7c15;
	z = (z ^ (z >> 30)) * 0xbf58476d1ce4e5b9;
	z = (z ^ (z >> 27)) * 0x94d049bb133111eb;
	z ^ (z >> 31)
}

pub fn initialize_seed(start_seed: u64) {
    // method suggested here
    // http://prng.di.unimi.it/
    // We suggest to use a SplitMix64 to initialize the state of our generators
    // starting from a 64-bit seed, as research has shown that initialization 
    // must be performed with a generator radically different in nature from 
    // the one initialized to avoid correlation on similar seeds.
    let mut s = start_seed;
    unsafe{
        GLOBAL_SEED[0] = splitmix64(s);
        s += 0x9e3779b97f4a7c15;
        GLOBAL_SEED[1] = splitmix64(s);
        s += 0x9e3779b97f4a7c15;
        GLOBAL_SEED[2] = splitmix64(s);
        s += 0x9e3779b97f4a7c15;
        GLOBAL_SEED[3] = splitmix64(s);
    }
}


// global static seed, this could be moved inside a struct
// WARNING
// the current implementation is not thread safe because we
// mutate a shared state between threads without any locks.
// This should not create any problem since we do not need
// a strong PRNG so for speed sake it's intentionally let
// this way.
// The only real problem could be that we lose determinism
static mut GLOBAL_SEED: [u64; 4] = [6591408588322595484, 5451729388608518856, 8913376598984957243, 17912695770704705270];


#[inline(always)]
fn rotl(x : u64, k: u64) -> u64{
	(x << k) | (x >> (64 - k))
}

#[inline(always)]
pub fn xorshiro256plus() -> f64{
    // based on xorshiro256+ which seems to be the fastest floating point generator
    // http://prng.di.unimi.it/xoshiro256plus.c
    // the conversion from u64 to f64 is taken from:
    // http://prng.di.unimi.it/
    // the informations about the structure of a f64 was taken from IEEE 754
    // https://standards.ieee.org/content/ieee-standards/en/standard/754-2019.html
    // https://en.wikipedia.org/wiki/Double-precision_floating-point_format
    // if this is still a bottleneck we can consider to implement
    // http://prng.di.unimi.it/xoshiro256+-vect-speed.c
    // which exploits avx to generate in parallel 8 random numbers and fill a 
    // cache with it
    unsafe {
        // normal xorshiro implementation
        let (result, _): (u64, bool) = GLOBAL_SEED[0].overflowing_add(GLOBAL_SEED[3]);

        let t: u64 = GLOBAL_SEED[1] << 17;

        GLOBAL_SEED[2] ^= GLOBAL_SEED[0];
        GLOBAL_SEED[3] ^= GLOBAL_SEED[1];
        GLOBAL_SEED[1] ^= GLOBAL_SEED[2];
        GLOBAL_SEED[0] ^= GLOBAL_SEED[3];

        GLOBAL_SEED[2] ^= t;

        GLOBAL_SEED[3] = rotl(GLOBAL_SEED[3], 45);
        // method proposed by vigna on http://prng.di.unimi.it/ 
        (result >> 11) as f64 * 2.0f64.powf(-53.0)
    }
}

pub fn sample(weights: &[WeightT]) -> usize {
    let mut cumulative_sum: Vec<f64> = Vec::with_capacity(weights.len());
    let mut total_weight = 0f64;
    for w in weights {
        total_weight += w;
        cumulative_sum.push(total_weight);
    }

    let rnd: f64 = xorshiro256plus() * cumulative_sum[cumulative_sum.len() - 1];

    // Find the first item which has a weight *higher* than the chosen weight.
    cumulative_sum.binary_search_by(
        |w|
            if *w <= rnd { 
                Ordering::Less 
            } else { 
                Ordering::Greater 
            }
        ).unwrap_err()
}