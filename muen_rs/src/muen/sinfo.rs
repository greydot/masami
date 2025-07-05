// Subject info module

use core::cmp::*;
use core::ffi::c_char;
use core::mem::{transmute, size_of};
use core::str;

const MUEN_SINFO_MAGIC: u64 = 0x03006f666e69756d;
const MAX_RESOURCE_COUNT: usize = 255;
const MAX_NAME_SIZE: usize = 63;
const HASH_SIZE: usize = 32;

// Resource name
#[repr(C,packed)]
#[derive(Clone,Copy)]
pub struct MuenName {
    pub len: u8,
    pub data: [c_char; MAX_NAME_SIZE],
    null: u8
}

impl MuenName {
    fn as_str(&self) -> &str {
        unsafe {
            let sz = min(self.len as usize, MAX_NAME_SIZE);
            let b: &[u8; MAX_NAME_SIZE] = transmute(&self.data);
            str::from_raw_parts(b.as_ptr(), sz)
        }
    }
    fn matches(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

// Memory kind
#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Clone,Copy)]
enum MuenMemoryKind {
    MUEN_MEM_SUBJ = 0,
    MUEN_MEM_SUBJ_INFO,
    MUEN_MEM_SUBJ_BIN,
    MUEN_MEM_SUBJ_ZP,
    MUEN_MEM_SUBJ_INITRD,
    MUEN_MEM_SUBJ_CHANNEL,
    MUEN_MEM_SUBJ_STATE,
    MUEN_MEM_SUBJ_TIMED_EVT,
    MUEN_MEM_SUBJ_INTRS,
    MUEN_MEM_SUBJ_SCHEDINFO,
    MUEN_MEM_SUBJ_BIOS,
    MUEN_MEM_SUBJ_ACPI_RSDP,
    MUEN_MEM_SUBJ_ACPI_XSDT,
    MUEN_MEM_SUBJ_ACPI_FADT,
    MUEN_MEM_SUBJ_ACPI_DSDT,
    MUEN_MEM_SUBJ_DEVICE,
    MUEN_MEM_SUBJ_SOLO5_BOOT_INFO,
    MUEN_MEM_SUBJ_CRASH_AUDIT,
    MUEN_MEM_KRNL_IFACE
}

// Known memory contents
#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Clone,Copy)]
enum MuenContentKind{
    MUEN_CONTENT_UNINITIALIZED = 0,
    MUEN_CONTENT_FILL,
    MUEN_CONTENT_FILE
}

// Memory region description
#[repr(C, packed)]
#[derive(Clone,Copy)]
pub struct MuenMemRegion {
    kind: MuenMemoryKind,
    content: MuenContentKind,
    flags: u8,
    pattern: u16,
    padding1: [c_char;3],
    address: u64,
    size: u64,
    hash: [u8; HASH_SIZE]
}

const MAX_VARIANT_SZ: usize = size_of::<MuenMemRegion>();

const MUEN_DEVICE_SZ: usize = 7;
// PCI device description
#[repr(C,packed)]
#[derive(Clone,Copy)]
pub struct MuenDevice {
    sid: u16,
    irte_start: u16,
    irq_start: u8,
    ir_count: u8,
    flags: u8,
    padding: [u8; MAX_VARIANT_SZ - MUEN_DEVICE_SZ]
}

const MUEN_DEVMEM_SZ: usize = 16 + 1;
// Device MMIO region description
#[repr(C,packed)]
#[derive(Clone,Copy)]
pub struct MuenDevmem {
    pub flags: u8,
    padding1: [u8;7],
    pub addr: u64,
    pub size: u64,
    padding2: [u8; MAX_VARIANT_SZ - (MUEN_DEVMEM_SZ + 7)]
}

// Currently known resource types
#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Clone,Copy,PartialEq, Eq, PartialOrd, Ord)]
pub enum MuenResourceKind {
    MUEN_RES_NONE = 0,
    MUEN_RES_MEMORY,
    MUEN_RES_EVENT,
    MUEN_RES_VECTOR,
    MUEN_RES_DEVICE,
    MUEN_RES_DEVMEM
}

#[repr(C,packed)]
#[derive(Clone,Copy)]
pub union MuenResourceData {
    pub mem: MuenMemRegion,
    pub dev: MuenDevice,
    pub devmem: MuenDevmem,
    pub num: u8
}

#[repr(C,packed)]
#[derive(Clone,Copy)]
pub struct MuenResource {
    pub kind: MuenResourceKind,
    pub name: MuenName,
    padding: [u8; 3],
    pub data: MuenResourceData
}

#[repr(C,packed)]
pub struct SubjectInfo {
    magic: u64,
    tsc_khz: u32,
    name: MuenName,
    res_count: u16,
    padding: [u8;1],
    resources: [MuenResource; MAX_RESOURCE_COUNT]
}

impl SubjectInfo {
    pub const fn init() -> &'static SubjectInfo {
        let ptr = 0xe00000000 as *mut SubjectInfo;
        unsafe { ptr.as_ref().unwrap() }
    }

    pub fn check_magic(&self) -> bool {
        self.magic == MUEN_SINFO_MAGIC
    }

    pub fn name(&self) -> &str {
        unsafe {
            // explicit &[i8] -> &[u8]
            let conv = |a: &[i8; MAX_NAME_SIZE]| transmute(a);
            let uname: &[u8;MAX_NAME_SIZE] = conv(&self.name.data);
            str::from_utf8_unchecked(uname)
        }
    }

    pub fn tsc_khz(&self) -> u32 {
        self.tsc_khz
    }

    pub fn get_resource(&self, name: &str, kind: MuenResourceKind) -> Option<MuenResource> {
        for r in self.resources {
            let k = r.kind; // avoiding error[E0793]: reference to packed field is unaligned
            if r.name.matches(name) && k == kind {
                return Some(r)
            }
        }
        return None
    }
}
