use wasm_bindgen_test::*;

use pouch::*;

#[wasm_bindgen_test]
fn it_works() {
    assert_eq!(get_pouchdb_version(), "1.0.0");
}
