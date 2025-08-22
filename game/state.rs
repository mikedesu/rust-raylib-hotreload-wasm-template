use raylib_wasm::RenderTexture2D;
use raylib_wasm::Vector2;
use std::collections::HashMap;

use texture_info::TextureInfo;

pub struct State {
    pub mouse_pos: Vector2,
    pub target: RenderTexture2D,
    pub textures: HashMap<i32, TextureInfo>,
}
