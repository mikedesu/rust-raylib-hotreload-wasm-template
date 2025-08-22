//use raylib_wasm::{KeyboardKey as KEY, *};
//use raylib_wasm::KeyboardKey as KEY;
use raylib_wasm::*;

// use crate::state::State; // doesn't work
// use super::state::State; // doesn't work
// use state::State; // doesn't work

mod state;
use state::State;
use crate::texture_info::TextureInfo;

// need to import texture_info.rs's TextureInfo struct
// it is located in ./state/texture_info.rs
//use texture_info::TextureInfo; // unresolved import, use of undeclared crate or module
//mod texture_info; // unresolved module, cant find module file: texture_info.rs, or
// texture_info/mod.rs

use std::collections::HashMap;

const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;

const TARGET_WIDTH: i32 = 640;
const TARGET_HEIGHT: i32 = 360;

const ORIGIN: Vector2 = Vector2 { x: 0.0, y: 0.0 };

const TARGET_SRC: Rectangle = Rectangle {
    x: 0.0,
    y: 0.0,
    width: TARGET_WIDTH as f32,
    height: -TARGET_HEIGHT as f32,
};

const TARGET_DST: Rectangle = Rectangle {
    x: 0.0,
    y: 0.0,
    width: WINDOW_WIDTH as f32,
    height: WINDOW_HEIGHT as f32,
};

const TARGET_FPS: i32 = 60;

#[no_mangle]
pub unsafe fn game_init() -> State {
    SetTargetFPS(TARGET_FPS);

    init_window(WINDOW_WIDTH, WINDOW_HEIGHT, "game");

    let mut textures: HashMap<i32, TextureInfo> = HashMap::new();

    // game/game.rs|43 col 5-54 error| cannot assign to data in an index of `HashMap<i32, raylib_wasm::Texture>` trait `IndexMut` is required to modify indexed content, but it is not implemented for `HashMap<i32, raylib_wasm::Texture>`
    //textures[&0] = load_texture("img/human_idle.png");

    textures.insert(0, load_texture("img/human_idle.png"));

    State {
        mouse_pos: Vector2 { x: 0.0, y: 0.0 },
        target: LoadRenderTexture(TARGET_WIDTH, TARGET_HEIGHT),
        textures: textures, //tx: load_texture("img/human_idle.png"),
    }
}

unsafe fn handle_keys(_state: &mut State) {
    //if IsKeyDown(KEY::Space) {
    //    state.speed = SPEED_BOOSTED
    //}
    //if !IsKeyDown(KEY::Space) {
    //    state.speed = SPEED_DEFAULT
    //}

    //let dt = GetFrameTime();
    //if IsKeyDown(KEY::W) {
    //    state.rect.y -= dt * state.speed
    //}
    //if IsKeyDown(KEY::A) {
    //    state.rect.x -= dt * state.speed
    //}
    //if IsKeyDown(KEY::S) {
    //    state.rect.y += dt * state.speed
    //}
    //if IsKeyDown(KEY::D) {
    //    state.rect.x += dt * state.speed
    //}
}

unsafe fn handle_mouse(state: &mut State) {
    state.mouse_pos = GetMousePosition();
}

unsafe fn handle_drawing(state: &mut State) {
    BeginDrawing();
    {
        BeginTextureMode(state.target);
        {
            ClearBackground(BLACK);
            //DrawRectangleRec(state.rect, RAYWHITE);

            let c: Color = Color {
                r: 0x66 as u8,
                g: 0x66 as u8,
                b: 0x66 as u8,
                a: 255,
            };

            //let text: str = "evildojo666"; // this doesnt work
            // declare a new string that says "evildojo666"
            let text: &str = "evildojo666";
            // why does this work?
            // because the string is a string slice, which is a reference to a string literal

            let fontsize: i32 = 50;
            let m: i32 = measure_text(text, fontsize as usize);
            let x: i32 = TARGET_WIDTH / 2 - m / 2;
            let y: i32 = TARGET_HEIGHT / 2 - fontsize / 2;

            draw_text(text, x, y, fontsize as usize, c);

            //DrawTexturePro(
            //    *state.textures.get(&0).unwrap(),
            //    Rectangle {
            //        x: 0.0,
            //        y: 0.0,
            //        width: 32.0,
            //        height: 32.0,
            //    },
            //    Rectangle {
            //        x: 0.0,
            //        y: 0.0,
            //        width: TARGET_WIDTH as f32,
            //        height: TARGET_HEIGHT as f32,
            //    },
            //    ORIGIN,
            //    0.0,
            //    WHITE,
            //);

            //let rect_pos = format! {
            //    "rect: [{x}, {y}]",
            //    x = state.rect.x.round(),
            //    y = state.rect.y.round()
            //};
            //draw_text(&rect_pos, 10, 10, 20, RAYWHITE);
            //let mouse_pos = format! {
            //    "mouse: [{x}, {y}]",
            //    x = state.mouse_pos.x.round(),
            //    y = state.mouse_pos.y.round()
            //};
            //draw_text(&mouse_pos, 10, 30, 20, RAYWHITE);
            //let mx = (state.mouse_pos.x / 2.0) as i32;
            //let my = (state.mouse_pos.y / 2.0) as i32;
            //DrawCircle(mx, my, 10.0, RAYWHITE);
        }

        EndTextureMode();
    }

    DrawTexturePro(
        state.target.texture,
        TARGET_SRC,
        TARGET_DST,
        Vector2 { x: 0.0, y: 0.0 },
        0.0,
        WHITE,
    );

    DrawFPS(5, 5);

    EndDrawing();
}

pub type GameFrame = unsafe fn(state: &mut State);

#[no_mangle]
pub unsafe fn game_frame(state: &mut State) {
    handle_keys(state);
    handle_mouse(state);
    handle_drawing(state);
}

#[no_mangle]
pub unsafe fn game_over() {
    CloseWindow();
}
