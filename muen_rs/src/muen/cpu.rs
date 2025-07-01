use x86_64::addr::*;
use x86_64::structures::{gdt::{Descriptor, GlobalDescriptorTable}, tss::TaskStateSegment};

const STACK_SIZE: usize = 4096;

static mut INTR_STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
static mut TRAP_STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
static mut NMI_STACK:  [u8; STACK_SIZE] = [0; STACK_SIZE];

static mut GDT: GlobalDescriptorTable = GlobalDescriptorTable::new();

const GDT_TSS_LO: usize = 3;
const GDT_TSS_HI: usize = 4;

#[allow(static_mut_refs, unsafe_op_in_unsafe_fn)]
pub unsafe fn cpu_init() {
    // GDT
    GDT.append(Descriptor::kernel_code_segment());
    GDT.append(Descriptor::kernel_data_segment());
    GDT.load_unsafe();
    // GDT

    let mut tss_lo = TaskStateSegment::new();
    tss_lo.interrupt_stack_table[0] = VirtAddr::new(INTR_STACK.as_mut_ptr().offset(STACK_SIZE as isize).addr() as u64);
    tss_lo.interrupt_stack_table[1] = VirtAddr::new(TRAP_STACK.as_mut_ptr().offset(STACK_SIZE as isize).addr() as u64);
    tss_lo.interrupt_stack_table[2] = VirtAddr::new( NMI_STACK.as_mut_ptr().offset(STACK_SIZE as isize).addr() as u64);
}
