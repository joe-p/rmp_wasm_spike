# Results

## Manual rmpv

```rust
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
```

```sh
cargo build --target wasm32-unknown-unknown && twiggy top -n 25 target/wasm32-unknown-unknown/debug/rmpv_wasm_spike.wasm                                                                   
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
 Shallow Bytes │ Shallow % │ Item
───────────────┼───────────┼──────────────────────────────────────────────────────────────────────────────────────────────────────────────
        897359 ┊    38.17% ┊ custom section '.debug_str'
        658988 ┊    28.03% ┊ custom section '.debug_info'
        299358 ┊    12.73% ┊ custom section '.debug_line'
        132176 ┊     5.62% ┊ custom section '.debug_ranges'
         80363 ┊     3.42% ┊ "function names" subsection
         35328 ┊     1.50% ┊ custom section '.debug_abbrev'
         21477 ┊     0.91% ┊ data[0]
         15983 ┊     0.68% ┊ rmpv::decode::value_ref::read_value_ref_inner::h1719196324cae8a1
          5075 ┊     0.22% ┊ rmpv::encode::value::write_value::h4c0958f84b1b1955
          2359 ┊     0.10% ┊ rmp::encode::sint::write_sint::h0f91fcf6e09b8119
          2325 ┊     0.10% ┊ alloc::raw_vec::RawVecInner<A>::grow_amortized::hf324431329d2971d
          2325 ┊     0.10% ┊ alloc::raw_vec::RawVecInner<A>::grow_amortized::hce419f262655f50c
          2179 ┊     0.09% ┊ dlmalloc::dlmalloc::Dlmalloc<A>::malloc::hb4cd449a33575ea6
          2151 ┊     0.09% ┊ dlmalloc::dlmalloc::Dlmalloc<A>::tmalloc_large::h0cb3983f18adef37
          1979 ┊     0.08% ┊ dlmalloc::dlmalloc::Dlmalloc<A>::sys_alloc::h666cb0363b39545b
          1928 ┊     0.08% ┊ <alloc::alloc::Global as core::alloc::Allocator>::shrink::hf4c542e0b208bc57
          1885 ┊     0.08% ┊ rmpv_wasm_spike::<impl core::convert::From<rmpv_wasm_spike::Person> for rmpv::Value>::from::h12d1b61a2c6ed0d4
          1816 ┊     0.08% ┊ dlmalloc::dlmalloc::Dlmalloc<A>::free::hb52dbc0d3c5b98c0
          1762 ┊     0.07% ┊ alloc::raw_vec::RawVecInner<A>::grow_exact::he70c99606e8f21ba
          1760 ┊     0.07% ┊ rmpv::decode::value_ref::read_map_data::h098f491d89c07472
          1728 ┊     0.07% ┊ dlmalloc::dlmalloc::Dlmalloc<A>::memalign::hb8f5009f0ba30c3f
          1693 ┊     0.07% ┊ alloc::alloc::Global::grow_impl::h0a0edb085b03ed8c
          1693 ┊     0.07% ┊ alloc::alloc::Global::grow_impl::hd6fd5c03158bb5e3
          1689 ┊     0.07% ┊ dlmalloc::dlmalloc::Dlmalloc<A>::try_realloc_chunk::h93dc9847165ba5a9
          1492 ┊     0.06% ┊ rmpv::ValueRef::to_owned::h1ad6bd269d351218
        174180 ┊     7.41% ┊ ... and 927 more.
       2351051 ┊   100.00% ┊ Σ [952 Total Rows]


```
