//#[cfg(feature = "native")]
use raylib_wasm::prelude::*;
//use raylib_wasm

mod sprite;
mod state;
mod texture_info;
use crate::sprite::new_sprite;
use crate::sprite::sprite_anim;
use crate::texture_info::new_texture_info;
use sprite::Sprite;
use state::State;
use std::collections::HashMap;
use texture_info::TextureInfo;

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
    let mut sprites: HashMap<i32, Sprite> = HashMap::new();

    let t = load_texture("img/human_idle.png");
    let ti = new_texture_info(0, t.width, t.height, 32, 32, 16, 4, t);
    textures.insert(0, ti);

    let s = new_sprite(0, 32, 32, 16, 4);

    sprites.insert(0, s);

    State {
        mouse_pos: Vector2 { x: 0.0, y: 0.0 },
        target: LoadRenderTexture(TARGET_WIDTH, TARGET_HEIGHT),
        textures: textures, //tx: load_texture("img/human_idle.png"),
        sprites: sprites,
        frame_count: 0,
    }
}

unsafe fn handle_keys(_state: &mut State) {
    //let dt = GetFrameTime();
    //if IsKeyDown(KEY::W) {
    //    state.rect.y -= dt * state.speed
    //}
}

unsafe fn handle_mouse(state: &mut State) {
    state.mouse_pos = GetMousePosition();
}

unsafe fn handle_drawing(state: &mut State) {
    BeginDrawing();
    BeginTextureMode(state.target);
    ClearBackground(BLACK);
    //DrawRectangleRec(state.rect, RAYWHITE);
    let c: Color = Color {
        r: 0x66 as u8,
        g: 0x66 as u8,
        b: 0x66 as u8,
        a: 255,
    };
    let text: &str = "evildojo666";
    // why does this work?
    // because the string is a string slice, which is a reference to a string literal
    let fontsize: i32 = 50;
    let m: i32 = measure_text(text, fontsize as usize);
    let x: i32 = TARGET_WIDTH / 2 - m / 2;
    let y: i32 = TARGET_HEIGHT / 2 - fontsize / 2;
    draw_text(text, x, y, fontsize as usize, c);

    let hero_sprite = state.sprites.get_mut(&0);
    if hero_sprite.is_some() {
        let hs: &mut Sprite = hero_sprite.unwrap();
        let tx = state.textures.get(&hs.txid).unwrap();

        let w = tx.frame_width;
        let h = tx.frame_height;

        let src = Rectangle {
            x: (hs.current_frame * tx.frame_width) as f32,
            y: 0.0,
            width: w as f32,
            height: h as f32,
        };

        let scale = 8.0;
        let dw = tx.frame_width as f32 * scale;
        let dh = tx.frame_height as f32 * scale;
        let dx = TARGET_WIDTH as f32 / 2.0 - dw / 2.0;
        let dy = TARGET_HEIGHT as f32 / 2.0 - dh / 2.0;

        let dst = Rectangle {
            x: dx as f32,
            y: dy as f32,
            width: dw,
            height: dh,
        };

        DrawTexturePro(tx.tx, src, dst, ORIGIN, 0.0, WHITE);

        if state.frame_count % 10 == 0 {
            sprite_anim(hs);
        }

        state.frame_count += 1;
    }

    EndTextureMode();
    DrawTexturePro(
        state.target.texture,
        TARGET_SRC,
        TARGET_DST,
        ORIGIN,
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
