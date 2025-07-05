use core::mem;
use core::sync::atomic::{AtomicU64, Ordering, fence};

const NULL_EPOCH: u64 = 0;
const SHMSTREAM20: u64 = 0x487312b6b79a9b6d;

#[repr(C,packed)]
pub struct ChannelHdr {
    transport: u64,
    epoch: u64,
    protocol: u64,
    size: u64,
    elements: u64,
    __reserved: u64,
    wsc: u64,
    wc: u64
}

impl ChannelHdr {
    fn set_epoch(&mut self, epoch: u64) {
        fence(Ordering::Acquire);
        self.epoch = epoch;
        fence(Ordering::Release);
    }
}

#[repr(C,packed)]
pub struct Channel<T: Sized> {
    header: ChannelHdr,
    data: T
}

impl<T: Sized> Channel<T> {
    fn deactivate(&mut self) {
        self.header.set_epoch(NULL_EPOCH);
    }

    pub fn init_writer(
        &mut self,
        protocol: u64,
        element_size: u64,
        channel_size: u64,
        epoch: u64
    ) {
        self.deactivate();
        unsafe {
            let _ = mem::replace(self, mem::zeroed());
        }
        let data_size = channel_size - size_of::<ChannelHdr>() as u64;
        self.header.transport = SHMSTREAM20;
        self.header.protocol = protocol;
        self.header.size = element_size;
        self.header.elements = data_size / element_size;
        self.header.wsc = 0;
        self.header.wc = 0;

        self.header.set_epoch(epoch);
    }
}
