use wasm_bindgen_test::*;

use pouch::*;

#[wasm_bindgen_test]
fn it_works() {
    assert_eq!(get_version(), "0.0.3-alpha");
}
