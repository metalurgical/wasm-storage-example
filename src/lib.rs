mod utils;

use cryptoxide::digest::Digest;
use cryptoxide::sha3::Keccak256;
use gloo_storage::{LocalStorage, Storage};
use wasm_bindgen::prelude::*;
use serde::{Serialize,Deserialize};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const APP_NAME: &str = "storage_example";

#[wasm_bindgen]
#[derive(Serialize,Deserialize)]
struct Item {
    key: String,
    data: String,
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn get_string(key: String) -> Result<String, JsValue> {
    let res = LocalStorage::get(key).map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(res)
}

#[wasm_bindgen]
pub fn delete_string(key: String) {
    LocalStorage::delete(key)
}

#[wasm_bindgen]
pub fn set_string(input: String) -> Result<JsValue, JsValue> {
    let mut unhashed_key = APP_NAME.to_string();
    unhashed_key.push_str(&input);
    let mut ctx = Keccak256::new();
    ctx.input(unhashed_key.as_bytes());
    let key = ctx.result_str();
    let data = Item {
        key: key.clone(),
        data: input.clone(),
    };
    LocalStorage::set(key, input).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let serialized = JsValue::from_serde(&data).map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(serialized)
}

#[wasm_bindgen]
pub fn update_string(key: String, input: String) -> Result<(), JsValue> {
    LocalStorage::set(key, input).map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(())
}

#[wasm_bindgen]
pub fn clear_storage() {
    LocalStorage::clear()
}

#[cfg(test)]
mod test {
    use crate::{clear_storage, delete_string, get_string, Item, set_string};
    use gloo_storage::{LocalStorage, Storage};
    use wasm_bindgen::JsValue;
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    wasm_bindgen_test_configure!(run_in_browser);
    //e.g run: wasm-pack test --chrome

    #[test]
    fn get() {
        let value = "value";
        let item = JsValue::into_serde::<Item>( &set_string(value.to_string()).unwrap()).unwrap();
        let  key = item.key;
        let obtained_value: String = get_string(key.into()).unwrap();
        assert_eq!(value, obtained_value)
    }

    #[test]
    fn set_and_length() {
        clear_storage();
        assert_eq!(LocalStorage::length(), 0);
        let _ = set_string("value".to_string()).unwrap();
        assert_eq!(LocalStorage::length(), 1);
        clear_storage();
        assert_eq!(LocalStorage::length(), 0);
    }

    #[test]
    fn delete() {
        let value = "value";
        let item = JsValue::into_serde::<Item>( &set_string(value.to_string()).unwrap()).unwrap();
        let  key = item.key;
        assert_eq!(LocalStorage::length(), 1);
        delete_string(key);
        assert_eq!(LocalStorage::length(), 0);
    }
}
