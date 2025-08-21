

all: desktop 


check: game/game.rs game/state.rs src/main.rs 
	cargo check

desktop: game/game.rs game/state.rs src/main.rs 
	cargo run --features=native




clean:
	cargo clean
