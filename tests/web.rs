//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

use tauri_cli::add;

wasm_bindgen_test_configure!();

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
fn test_add() {
    let val = add(2, 3);
    assert_eq!(val, 5);
}
