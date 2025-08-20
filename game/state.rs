use raylib_wasm::Rectangle;
use raylib_wasm::RenderTexture2D;
use raylib_wasm::Vector2;

pub struct State {
    pub rect: Rectangle,
    pub speed: f32,
    pub mouse_pos: Vector2,
    pub target: RenderTexture2D,
}
