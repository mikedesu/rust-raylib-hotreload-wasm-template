use crate::Sprite;
use crate::TextureInfo;
use raylib_wasm::prelude::RenderTexture2D;
use raylib_wasm::prelude::Vector2;
use std::collections::HashMap;

pub struct State {
    pub mouse_pos: Vector2,
    pub target: RenderTexture2D,
    pub textures: HashMap<i32, TextureInfo>,
    pub sprites: HashMap<i32, Sprite>,
    pub frame_count: i32,
}
