use core::ffi::c_void;

use super::console::*;
use super::cpu::*;
use super::sinfo::*;

#[unsafe(no_mangle)]
pub unsafe extern "C"
fn _start(_arg: *mut c_void) {
    let sinfo = SubjectInfo::init();
    if !sinfo.check_magic() {
        panic!("INVALID MAGIC NUMBER IN SUBJECT INFO")
    }
    unsafe { cpu_init(); }
    let mut console = Console::init(sinfo).unwrap();
    console.write("Masami initialised");
}
