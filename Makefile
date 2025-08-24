

all: desktop 


check: game/game.rs game/state.rs src/main.rs 
	cargo check

desktop: game/game.rs game/state.rs game/texture_info.rs src/main.rs 
	cargo run --features=native

web: game/game.rs game/state.rs game/texture_info.rs src/main.rs 
	cargo build --target=wasm32-unknown-unknown --features=web --no-default-features
	python -m http.server


clean:
	cargo clean
