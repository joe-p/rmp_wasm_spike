#![cfg_attr(not(test), no_std)]
extern crate alloc;

#[cfg_attr(not(test), panic_handler)]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

use alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

mod allocator {
    use dlmalloc::GlobalDlmalloc;

    #[global_allocator]
    static GLOBAL: GlobalDlmalloc = GlobalDlmalloc;
}

#[wasm_bindgen]
pub struct Person {
    name: String,
    favorite_numbers: Vec<u64>,
}

impl From<&[u8]> for Person {
    fn from(value: &[u8]) -> Self {
        let value = value.to_owned();
        let mut value = value.as_slice();
        rmpv::decode::read_value_ref_with_max_depth(&mut value, 2)
            .unwrap()
            .to_owned()
            .try_into()
            .unwrap()
    }
}

impl From<Person> for Vec<u8> {
    fn from(person: Person) -> Self {
        let mut bytes: &mut [u8] = Default::default();
        rmpv::encode::write_value(&mut bytes, &person.into()).unwrap();
        bytes.into()
    }
}

impl From<Person> for rmpv::Value {
    fn from(person: Person) -> Self {
        let name = rmpv::Value::from(person.name);
        let favorite_numbers =
            rmpv::Value::from_iter(person.favorite_numbers.iter().map(|x| x.to_owned()));

        let value_map: Vec<(rmpv::Value, rmpv::Value)> =
            vec![("n".into(), name), ("fn".into(), favorite_numbers)];

        value_map.into()
    }
}

impl TryFrom<rmpv::Value> for Person {
    type Error = String;
    fn try_from(mp_value: rmpv::Value) -> Result<Self, String> {
        let mut person = Person {
            name: String::default(),
            favorite_numbers: Vec::default(),
        };
        mp_value
            .as_map()
            .ok_or("not a map")?
            .iter()
            .for_each(|(key, value)| {
                if let Some(key_str) = key.as_str() {
                    match key_str {
                        "n" => person.name = value.as_str().unwrap().to_string(),
                        "fn" => {
                            person.favorite_numbers = value
                                .as_array()
                                .unwrap()
                                .iter()
                                .map(|v| v.as_u64().unwrap())
                                .collect()
                        }
                        _ => {}
                    }
                }
            });

        Ok(person)
    }
}

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn encode_person(person: Person) -> Vec<u8> {
    person.into()
}

#[wasm_bindgen]
pub fn decode_person(bytes: &[u8]) -> Person {
    bytes.into()
}
