use x86_64::addr::*;
use x86_64::structures::{gdt::{Descriptor, GlobalDescriptorTable}, tss::TaskStateSegment};

const STACK_SIZE: usize = 4096;

static mut INTR_STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
static mut TRAP_STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
static mut NMI_STACK:  [u8; STACK_SIZE] = [0; STACK_SIZE];

#[allow(static_mut_refs, unsafe_op_in_unsafe_fn)]
unsafe fn stack_to_virt(s: &[u8]) -> VirtAddr {
    let p = s.as_ptr().offset(STACK_SIZE as isize).addr() as u64;
    VirtAddr::new(p)
}

static mut TSS_LO: TaskStateSegment = TaskStateSegment::new();
static mut GDT: GlobalDescriptorTable = GlobalDescriptorTable::new();

//const GDT_TSS_LO: usize = 3;
//const GDT_TSS_HI: usize = 4;

#[allow(static_mut_refs, unsafe_op_in_unsafe_fn)]
pub unsafe fn cpu_init() {
    GDT.append(Descriptor::kernel_code_segment());
    GDT.append(Descriptor::kernel_data_segment());

    TSS_LO.interrupt_stack_table[0] = stack_to_virt(&INTR_STACK);
    TSS_LO.interrupt_stack_table[1] = stack_to_virt(&TRAP_STACK);
    TSS_LO.interrupt_stack_table[2] = stack_to_virt(& NMI_STACK);

    GDT.append(Descriptor::tss_segment(&TSS_LO));
    GDT.load();
}
