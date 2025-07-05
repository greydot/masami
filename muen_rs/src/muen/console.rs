use super::sinfo;

use core::ffi::c_char;

const DEBUGLOG_PROTO: u64 = 0xf00789094b6f70cf;

#[repr(C,packed)]
struct Msg {
    timestamp: u64,
    data: [c_char;56]
}

pub struct Console {

}
