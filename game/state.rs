//use raylib_wasm::Rectangle;
use raylib_wasm::RenderTexture2D;
use raylib_wasm::Texture;
use raylib_wasm::Vector2;

pub struct State {
    pub mouse_pos: Vector2,
    pub target: RenderTexture2D,
    pub tx: Texture,
}
