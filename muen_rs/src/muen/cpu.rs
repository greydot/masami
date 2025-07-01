use core::arch::asm;

const GDT_SIZE: usize = 5;

#[repr(C,packed)]
struct gdt_ptr {
    limit: u16,
    base:  u64
}

#[allow(dead_code,non_camel_case_types)]
enum GDT_ENTRY {
    NULL = 0,
    CODE = 1,
    DATA = 2,
    TSS_LO = 3,
    TSS_HI = 4,
//    TSS = GDT_ENTRIES::TSS_LO as isize
}

#[allow(dead_code,non_camel_case_types)]
const GDT_ENTRY_TSS: usize = GDT_ENTRY::TSS_LO as usize;

const fn gdt_desc_offset(e: GDT_ENTRY) -> u64 {
    e as u64 * 0x8
}

const GDT_CODE: u64 = 0x00af99000000ffff;
const GDT_DATA: u64 = 0x00cf93000000ffff;

static mut CPU_GDT64: [u64;GDT_SIZE] = [0;GDT_SIZE];

#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn gdt_init() {
    CPU_GDT64[GDT_ENTRY::CODE as usize] = GDT_CODE;
    CPU_GDT64[GDT_ENTRY::DATA as usize] = GDT_DATA;

    let gdtptr = gdt_ptr {
        limit: size_of::<[u64;GDT_SIZE]>() as u16 - 1,
        base: &raw const CPU_GDT64 as u64
    };

    asm!(
        "lgdt [{g}]",
        "push {c}",
        "push 2f",
        "ret",
        "2:",
        "mov rax, {d}",
        "mov ss, eax",
        "mov ds, eax",
        "mov es, eax",
        g = in(reg) &gdtptr,
        c = const gdt_desc_offset(GDT_ENTRY::CODE),
        d = const gdt_desc_offset(GDT_ENTRY::DATA)
    )
}
