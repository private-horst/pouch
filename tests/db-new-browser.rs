wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
// use wasm_bindgen_test::*;

use pouch::*;

// TODO make them work with 'wasm-pack test --chrome'
// #[wasm_bindgen_test]
async fn _test_get_name() {
    let db_name = "tests_new_node";
    let db = Database::new(db_name);
    let info = db.info().await.unwrap();
    assert_eq!(info.db_name, db_name);
}
