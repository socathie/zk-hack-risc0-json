#![no_main]

use json::parse;
use json_core::Outputs;
use risc0_zkvm_guest::{env, sha};

risc0_zkvm_guest::entry!(main);

pub fn main() {
    let data1: String = env::read();
    let data2: String = env::read();

    let sha1 = sha::digest(&data1.as_bytes());
    let sha2 = sha::digest(&data2.as_bytes());
    let data1 = parse(&data1).unwrap();
    let data2 = parse(&data2).unwrap();

    if data1["critical_data"].as_u32().unwrap()!=data2["critical_data"].as_u32().unwrap() {
        panic!();
    }
    let out = Outputs {
        hash1: *sha1,
        hash2: *sha2,
    };
    env::commit(&out);
}
