use raylib_wasm::{KeyboardKey as KEY, *};

// use crate::state::State; // doesn't work
// use super::state::State;

const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;

const TARGET_WIDTH: i32 = 640;
const TARGET_HEIGHT: i32 = 360;

const TARGET_SRC: Rectangle = Rectangle {
    x: 0.0,
    y: 0.0,
    width: 640.0,
    height: 360.0,
};
const TARGET_DST: Rectangle = Rectangle {
    x: 0.0,
    y: 0.0,
    width: 1280.0,
    height: 720.0,
};

const SPEED_DEFAULT: f32 = 850.0;
const SPEED_BOOSTED: f32 = 1550.0;

#[no_mangle]
pub unsafe fn game_init() -> State {
    SetTargetFPS(144);
    init_window(WINDOW_WIDTH, WINDOW_HEIGHT, "game");

    State {
        rect: Rectangle {
            x: (WINDOW_WIDTH as f32 - 100.0) / 2.0,
            y: (WINDOW_HEIGHT as f32 - 100.0) / 2.0,
            width: 100.0,
            height: 100.0,
        },
        speed: 850.0,
        mouse_pos: Vector2 { x: 0.0, y: 0.0 },
        target: LoadRenderTexture(TARGET_WIDTH, TARGET_HEIGHT),
    }
}

unsafe fn handle_keys(state: &mut State) {
    if IsKeyDown(KEY::Space) {
        state.speed = SPEED_BOOSTED
    }
    if !IsKeyDown(KEY::Space) {
        state.speed = SPEED_DEFAULT
    }

    let dt = GetFrameTime();
    if IsKeyDown(KEY::W) {
        state.rect.y -= dt * state.speed
    }
    if IsKeyDown(KEY::A) {
        state.rect.x -= dt * state.speed
    }
    if IsKeyDown(KEY::S) {
        state.rect.y += dt * state.speed
    }
    if IsKeyDown(KEY::D) {
        state.rect.x += dt * state.speed
    }
}

unsafe fn handle_mouse(state: &mut State) {
    state.mouse_pos = GetMousePosition();
}

pub type GameFrame = unsafe fn(state: &mut State);

#[no_mangle]
pub unsafe fn game_frame(state: &mut State) {
    handle_keys(state);
    handle_mouse(state);

    BeginDrawing();
    {
        BeginTextureMode(state.target);
        {
            ClearBackground(DARKGREEN);
            draw_text("hello world", 250, 500, 50, RAYWHITE);
            DrawRectangleRec(state.rect, RAYWHITE);
            DrawFPS(WINDOW_WIDTH - 100, 10);
            let rect_pos = format! {
                "rect: [{x}, {y}]",
                x = state.rect.x.round(),
                y = state.rect.y.round()
            };
            draw_text(&rect_pos, 10, 10, 20, RAYWHITE);
            let mouse_pos = format! {
                "mouse: [{x}, {y}]",
                x = state.mouse_pos.x.round(),
                y = state.mouse_pos.y.round()
            };
            draw_text(&mouse_pos, 10, 30, 20, RAYWHITE);
            DrawCircle(
                state.mouse_pos.x as i32,
                state.mouse_pos.y as i32,
                10.0,
                RAYWHITE,
            );
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

    EndDrawing();
}

#[no_mangle]
pub unsafe fn game_over() {
    CloseWindow();
}
