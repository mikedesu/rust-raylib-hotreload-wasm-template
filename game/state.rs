use raylib_wasm::prelude::*;

pub struct State {
    pub rect: Rectangle,
    pub speed: f32,
    pub mouse_pos: Vector2,
    pub target: RenderTexture2D,
}
