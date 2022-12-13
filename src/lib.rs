mod utils;

use tauri_cli::info::{self, Options};
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

use web_sys::console;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

// use std::env::args_os;
// use std::ffi::OsStr;
// use std::path::Path;
// use std::process::exit;

#[wasm_bindgen]
pub fn tauri(args: js_sys::Array) {
    set_panic_hook();

    let args: Vec<_> = args.to_vec().iter().map(|x| x.to_owned().as_string().expect("arg is not string type")).collect();

    tauri_cli::run(args, Some("cargo tauri".into()))
}

// #[wasm_bindgen]
// pub fn info() {
//     set_panic_hook();
//     let res = tauri_cli::helpers::command_env(false);
//     //let res = info::command(Options{});
//     let res: JsValue = format!("help return {:?}", res).into();
//     console::log_1(&res);
// }
