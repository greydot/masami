use super::channel::Channel;
use super::sched::*;
use super::sinfo::*;

use core::ffi::{c_char, c_size_t};
use core::slice;

const DEBUGLOG_PROTO: u64 = 0xf00789094b6f70cf;
const MSG_SIZE:usize = 56;

#[repr(C,packed)]
#[derive(Clone,Copy)]
struct Msg {
    timestamp: u64,
    data: [c_char;56]
}

impl Msg {
    pub fn new() -> Msg {
        Msg{ timestamp: 0, data: [0;MSG_SIZE] }
    }
}

pub struct Console<'a> {
    channel: &'a mut Channel<Msg>,
    buf: Msg,
    pos: usize
}

static mut CONSOLE: Option<Console<'static>> = None;

impl<'a> Console<'a> {
    pub fn init(sinfo: &'a SubjectInfo) -> Option<Console<'a>> {
        let res = sinfo.get_resource("debuglog", MuenResourceKind::MUEN_RES_MEMORY)?;
        let sched = SchedulingInfo::init();
        let epoch: u64 = sched.tsc_schedule_start;

        let channel: &'a mut Channel<Msg> = unsafe {
            Channel::from_addr(res.data.mem.address)
        };
        let mem_size = unsafe { res.data.mem.size };

        channel.init_writer(DEBUGLOG_PROTO, mem_size, epoch);

        Some(Console{channel, buf: Msg::new(), pos: 0})
    }

    #[allow(static_mut_refs)]
    pub fn get() -> &'static mut Console<'static> {
        unsafe {
            CONSOLE.get_or_insert(
                Console::init(SubjectInfo::init()).expect("Failed to initialise console")
            )
        }
    }

    pub fn flush(&mut self) {
        let sched = SchedulingInfo::init();
        self.buf.timestamp = sched.tsc_schedule_start;
        self.channel.write(&self.buf);
        self.buf.data = [0;MSG_SIZE];
        self.pos = 0;
    }

    pub fn write(&mut self, s: &str) {
        for c in s.as_bytes() {
            if *c as char != '\x0d' {
                self.buf.data[self.pos] = *c as c_char;
                if self.pos == 55 {
                    self.flush();
                } else {
                    self.pos += 1;
                }
            }
        }
    }
}

pub fn write_line(s: &str) {
    let mut console = Console::get();
    console.write(s)
}

pub extern "C"
fn c_write_line(s: *const c_char, sz: c_size_t) {
    let s = unsafe {
        let p = s as *const u8;
        let sl = slice::from_raw_parts(p, sz);
        str::from_utf8_unchecked(sl)
    };
    write_line(s);
}
