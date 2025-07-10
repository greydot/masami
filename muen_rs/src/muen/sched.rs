#[repr(C,packed)]
pub struct SchedulingInfo {
    pub tsc_schedule_start: u64,
    pub tsc_schedule_end: u64
}

const SCHED_INFO_ADDR: u64 = 0xe00008000;

impl SchedulingInfo {
    pub fn init() -> &'static SchedulingInfo {
        let ptr = SCHED_INFO_ADDR as *mut SchedulingInfo;
        unsafe {
            ptr.as_ref().unwrap()
        }
    }
}
