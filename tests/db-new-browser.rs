wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
use wasm_bindgen_test::*;

use pouch::*;

#[wasm_bindgen_test]
fn test_get_version() {
    assert_eq!(get_version(), "0.0.4-alpha");
}
