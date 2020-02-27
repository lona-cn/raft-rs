extern crate rand;
use base64_url::*;
use num_bigint::{BigInt, RandBigInt, ToBigInt};
use num_traits::{Zero,cast::ToPrimitive};
use rand::{random, thread_rng, Rng};

pub fn rand_string(n: usize) -> String {
    let mut b = vec![0u8; 2 * n];
    let mut rng = rand::thread_rng();
    for x in b.iter_mut() {
        *x = rng.gen();
    }
    let output = String::new();
    let res = base64_url::encode_and_push_to_string(&b, &output);
    return (&res[0..n]).parse().unwrap();
}

fn make_seed() -> i64 {
    if let Some(max) = ToBigInt::to_bigint(&((1 as i64) << 62)) {
        let mut rng = rand::thread_rng();
        let low: BigInt = Zero::zero();
        let bigx = rng.gen_bigint_range(&low, &max);
        if let Some(num) = bigx.to_i64() {
            num
        } else {
            0
        }
    } else {
        0
    }
}

#[test]
fn test_rand_string() {
    println!("{}", rand_string(10));
}

#[test]
fn test_make_seed() {
    println!("{}", make_seed());
}
