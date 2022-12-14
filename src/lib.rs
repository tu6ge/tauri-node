mod utils;

use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

use web_sys::console;
use std::env;
use wasmedge_wasi_helper::wasmedge_wasi_helper::_initialize;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

// #[wasm_bindgen]
// pub fn tauri(args: js_sys::Array) {
//     set_panic_hook();

//     let args: Vec<_> = args.to_vec().iter().map(|x| x.as_string().expect("arg is not string type")).collect();

//     let res = format!("arg {:?}", args).into();
//     console::log_1(&res);

//     tauri_cli::run(args, None)
// }


#[wasm_bindgen]
pub fn print_env() -> i32 {
  _initialize();
  set_panic_hook();
  
  println!("The env vars are as follows.");
  for (key, value) in env::vars() {
    println!("{}: {}", key, value);
  }

  println!("The args are as follows.");
  for argument in env::args() {
    println!("{}", argument);
  }

  match env::var("PATH") {
    Ok(path) => println!("PATH: {}", path),
    Err(e) => println!("Couldn't read PATH ({})", e),
  };

  return 0;
}
