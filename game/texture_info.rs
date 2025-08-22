use raylib_wasm::prelude::Texture;

pub struct TextureInfo {
    pub txid: i32,
    pub width: i32,
    pub height: i32,
    pub frame_width: i32,
    pub frame_height: i32,
    pub num_frames: i32,
    pub num_contexts: i32,
    pub tx: Texture,
}
