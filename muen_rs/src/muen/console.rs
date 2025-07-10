use super::channel::Channel;
use super::sched::*;
use super::sinfo::*;

use core::ffi::c_char;

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
