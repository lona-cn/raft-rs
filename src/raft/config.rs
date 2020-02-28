extern crate rand;
use base64_url::*;
use chrono::{DateTime, Local};
use num_bigint::{BigInt, RandBigInt, ToBigInt};
use num_traits::{cast::ToPrimitive, Zero};
use rand::{random, thread_rng, Rng};
use std::collections::HashMap;
use std::sync::Mutex;
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

/*
type config struct {
    mu        sync.Mutex
    t         *testing.T
    net       *labrpc.Network
    n         int
    rafts     []*Raft
    applyErr  []string // from apply channel readers
    connected []bool   // whether each server is on the net
    saved     []*Persister
    endnames  [][]string            // the port file names each sends to
    logs      []map[int]interface{} // copy of each server's committed entries
    start     time.Time             // time at which make_config() was called
    // begin()/end() statistics
    t0        time.Time // time at which test_test.go called cfg.begin()
    rpcs0     int       // rpcTotal() at start of test
    cmds0     int       // number of agreements
    bytes0    int64
    maxIndex  int
    maxIndex0 int
}
*/
pub struct Raft(i32);
pub struct Persister(i32);
pub struct Config {
    mu:          Box<Mutex<u32>>,
    n:           i32,
    rafts:       Vec<Box<Raft>>,
    apply_err:   Vec<String>,
    connected:   Vec<bool>,
    saved:       Vec<Box<Persister>>,
    end_name:    Vec<Vec<String>>,
    logs:        Vec<HashMap<i32, i32>>, // copy of each server's committed entries
    start:       chrono::DateTime<Local>, // time at which make_config() was called
    t0:          chrono::DateTime<Local>,
    rpcs0:       i32,
    cmds0:       i32,
    bytes0:      i64,
    max_index:   i64,
    max_index_0: i64,
}

#[test]
fn test_rand_string() {
    println!("{}", rand_string(10));
}

#[test]
fn test_make_seed() {
    println!("{}", make_seed());
}
