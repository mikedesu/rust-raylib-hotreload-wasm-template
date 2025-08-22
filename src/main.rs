use game::*;

use raylib_wasm::prelude::*;

#[cfg(feature = "native")]
use libloading::{Library, Symbol};

#[cfg(all(feature = "web", not(target_arch = "wasm32")))]
compile_error!("`web` feature only works with wasm32 targets");

#[cfg(all(feature = "native", target_arch = "wasm32"))]
compile_error!("to compile for WASM do `--features=web --no-default-features`");

#[cfg(all(not(feature = "native"), not(target_arch = "wasm32")))]
compile_error!("to compile for native just do `cargo build`");

#[cfg(feature = "native")]
const fn get_game_path() -> &'static str {
    #[cfg(target_os = "linux")] {
        if cfg!(debug_assertions) { concat!("./target/debug/deps/libgame.so") }
        else { concat!("./target/release/deps/libgame.so") }
    }
    #[cfg(target_os = "windows")] {
        if cfg!(debug_assertions) { ".\\target\\debug\\deps\\libgame.dll"}
        else { ".\\target\\release\\deps\\libgame.dll" }
    }
    #[cfg(not(any(target_os = "windows", target_os = "linux")))] {
        if cfg!(debug_assertions) { "./target/debug/deps/libgame.dylib" }
        else { "./target/release/deps/libgame.dylib" }
    }
}

#[cfg(feature = "native")]
const GAME_PATH: &str = get_game_path();

#[inline]
#[cfg(feature = "native")]
unsafe fn load_lib(file_path: &str) -> Library {
    Library::new(file_path).expect("failed to load the library")
}

#[inline]
#[cfg(feature = "native")]
unsafe fn load_fn_unchecked<'lib, T: 'lib>(
    lib: &'lib Library,
    symbol: &str
) -> Symbol::<'lib, T> {
    match lib.get(symbol.as_bytes()) {
        Ok(ok) => ok,
        Err(e) => panic!("[error loading fn {symbol}] {e}")
    }
}

unsafe fn start() {
    #[cfg(feature = "native")]
    let mut lib = load_lib(GAME_PATH);

    #[cfg(feature = "native")]
    let mut game_frame = load_fn_unchecked::<GameFrame>(
        &lib,
        "game_frame"
    );

    let mut state = game_init();
    while !WindowShouldClose() {
        #[cfg(feature = "native")]
        if IsKeyPressed(KeyboardKey::R) {
            drop(game_frame);
            drop(lib);
            lib = load_lib(GAME_PATH);
            println!("[HOTRELOADING!]");
            game_frame = load_fn_unchecked(&lib, "game_frame");
        }

        game_frame(&mut state);
    }
}

fn main() {
    unsafe { start() }
}
