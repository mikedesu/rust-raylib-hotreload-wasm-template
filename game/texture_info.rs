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

pub fn new_texture_info(
    txid: i32,
    w: i32,
    h: i32,
    fw: i32,
    fh: i32,
    nf: i32,
    nc: i32,
    tx: Texture,
) -> TextureInfo {
    TextureInfo {
        txid: txid,
        width: w,
        height: h,
        frame_width: fw,
        frame_height: fh,
        num_frames: nf,
        num_contexts: nc,
        tx: tx,
    }
}
