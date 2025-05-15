#![cfg_attr(not(test), no_std)]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use serde::{Deserialize, Serialize};

mod allocator {
    use dlmalloc::GlobalDlmalloc;

    #[global_allocator]
    static GLOBAL: GlobalDlmalloc = GlobalDlmalloc;
}

#[wasm_bindgen]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Person {
    name: String,
    favorite_numbers: Vec<u64>,
}

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn encode_person(person: Person) -> Vec<u8> {
    let mut bytes: &mut [u8] = Default::default();
    rmp_serde::encode::write_named(&mut bytes, &person).unwrap();
    bytes.into()
}

#[wasm_bindgen]
pub fn decode_person(bytes: &[u8]) -> Person {
    rmp_serde::decode::from_slice::<Person>(bytes).unwrap()
}
