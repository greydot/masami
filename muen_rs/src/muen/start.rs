use core::ffi::{c_char, c_void, c_size_t};

use super::console::*;
use super::cpu::*;
use super::sinfo::*;

#[repr(C)]
pub struct MasamiInfo {
    console_write: extern "C" fn(s: *const c_char, sz: c_size_t),
}

unsafe extern "C" {
    pub fn masami_main(info: *const MasamiInfo);
}

#[unsafe(no_mangle)]
pub unsafe extern "C"
fn _start(_arg: *mut c_void) {
    masami_init();
    let info = MasamiInfo {
        console_write: c_write_line
    };
    unsafe { masami_main(&info) }
}

pub fn masami_init() {
    cpu_init();
    let sinfo = SubjectInfo::init();
    if !sinfo.check_magic() {
        panic!("INVALID MAGIC NUMBER IN SUBJECT INFO")
    }
    let console = Console::get();
    console.write("Masami initialised");
}
