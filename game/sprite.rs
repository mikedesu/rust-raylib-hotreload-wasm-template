//use raylib_wasm::prelude::Rectangle;

pub struct Sprite {
    pub txid: i32,
    pub width: i32,
    pub height: i32,
    pub current_frame: i32,
    pub current_context: i32,
    pub num_frames: i32,
    pub num_contexts: i32,
}

pub fn new_sprite(txid: i32, w: i32, h: i32, nf: i32, nc: i32) -> Sprite {
    Sprite {
        txid: txid,
        width: w,
        height: h,
        current_frame: 0,
        current_context: 0,
        num_frames: nf,
        num_contexts: nc,
    }
}

pub fn sprite_anim(s: &mut Sprite) {
    s.current_frame += 1;
    if s.current_frame >= s.num_frames {
        s.current_frame = 0;
    }
}
