use std::os::raw::{c_int, c_void};

pub type LamePtr = *mut c_void;

#[link(name="mp3lame")]
extern "C" {
    pub fn lame_init() -> LamePtr;
    pub fn lame_close(ptr: LamePtr) -> c_int;
    pub fn lame_set_in_samplerate(ptr: LamePtr, samplerate: c_int) -> c_int;
    pub fn lame_get_in_samplerate(ptr: LamePtr) -> c_int;
    pub fn lame_set_num_channels(ptr: LamePtr, channels: c_int) -> c_int;
    pub fn lame_get_num_channels(ptr: LamePtr) -> c_int;
    pub fn lame_set_quality(ptr: LamePtr, quality: c_int) -> c_int;
    pub fn lame_get_quality(ptr: LamePtr) -> c_int;
    pub fn lame_set_brate(ptr: LamePtr, quality: c_int) -> c_int;
    pub fn lame_get_brate(ptr: LamePtr) -> c_int;
    pub fn lame_set_error_protection(ptr: LamePtr, protection: c_int) -> c_int;
    pub fn lame_get_error_protection(ptr: LamePtr) -> c_int;
    pub fn lame_init_params(ptr: LamePtr) -> c_int;
    pub fn lame_encode_buffer(ptr: LamePtr,
        pcm_l: *const i16, pcm_r: *const i16, pcm_numsamples: c_int,
        mp3buf: *mut u8, mp3buf_size: c_int) -> c_int;
    #[allow(dead_code)]
    pub fn lame_encode_buffer_ieee_float(ptr: LamePtr,
                              pcm_l: *const f32, pcm_r: *const f32, pcm_numsamples: c_int,
                              mp3buf: *mut u8, mp3buf_size: c_int) -> c_int;
    pub fn lame_encode_flush(ptr: LamePtr, mp3buf: *mut u8, size: c_int) -> c_int;
    pub fn lame_encode_flush_nogap(ptr: LamePtr, mp3buf: *mut u8, size: c_int) -> c_int;
}
