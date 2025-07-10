use core::mem;
use core::sync::atomic::{Ordering, fence};

use static_assertions::assert_eq_size;

const NULL_EPOCH: u64 = 0;
const SHMSTREAM20: u64 = 0x487312b6b79a9b6d;

// Cannot use packed here due to "reference to packed field in unaligned" error.
#[repr(C,align(8))]
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
// Ensure the type is packed by checking its size.
assert_eq_size!(ChannelHdr, [u8;64]);

#[repr(C,align(8))]
pub struct Channel<T: Sized> {
    header: ChannelHdr,
    data: ChannelData<T>
}

impl<'a, T: Sized + Copy> Channel<T> {
    pub unsafe fn from_addr(addr: u64) -> &'a mut Channel<T> {
        unsafe {
            let ptr: *mut Channel<T> = mem::transmute(addr);
            &mut *ptr
        }
    }
    fn deactivate(&mut self) {
        self.set_epoch(NULL_EPOCH);
    }

    fn set_epoch(&mut self, epoch: u64) {
        fence(Ordering::Acquire);
        self.header.epoch = epoch;
        fence(Ordering::Release);
    }

    pub fn init_writer(
        &mut self,
        protocol: u64,
        channel_size: u64,
        epoch: u64
    ) {
        self.deactivate();
        unsafe {
            let _ = mem::replace(self, mem::zeroed());
        }
        let element_size = size_of::<T>() as u64;
        let data_size = channel_size - size_of::<ChannelHdr>() as u64;
        self.header.transport = SHMSTREAM20;
        self.header.protocol = protocol;
        self.header.size = element_size;
        self.header.elements = data_size / element_size;
        self.header.wsc = 0;
        self.header.wc = 0;

        self.set_epoch(epoch);
    }

    pub fn write(&mut self, elem: &T) {
        let mut wc = self.header.wc;
        let pos = wc % self.header.elements;

        wc += 1;

        copy_fence(&wc, &mut self.header.wsc);
        self.data.write_elem(&elem, pos);
        copy_fence(&wc, &mut self.header.wc);
    }
}

#[repr(C)]
struct ChannelData<T: Sized> {
    data: [T;0]
}

impl<T: Sized + Copy> ChannelData<T> {
    fn write_elem(&mut self, elem: &T, offset: u64) {
        unsafe {
            let p = self.data.as_mut_ptr().offset(offset as isize);
            let _ = mem::replace(&mut *p, *elem);
        }
    }
}

fn copy_fence<T: Copy>(src: &T, dst: &mut T) {
    fence(Ordering::Acquire);
    let _ = mem::replace(dst, *src);
    fence(Ordering::Release);
}
