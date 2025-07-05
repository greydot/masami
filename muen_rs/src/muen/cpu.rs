use core::arch::asm;
use x86_64::addr::*;
use x86_64::structures::idt::{InterruptDescriptorTable,
                              InterruptStackFrame,
                              PageFaultErrorCode};
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable};
use x86_64::structures::tss::TaskStateSegment;

const STACK_SIZE: usize = 4096;

static mut INTR_STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
static mut TRAP_STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
static mut NMI_STACK:  [u8; STACK_SIZE] = [0; STACK_SIZE];

#[allow(static_mut_refs, unsafe_op_in_unsafe_fn)]
unsafe fn stack_to_virt(s: &[u8]) -> VirtAddr {
    let p = s.as_ptr().offset(STACK_SIZE as isize).addr() as u64;
    VirtAddr::new(p)
}

static mut TSS: TaskStateSegment = TaskStateSegment::new();
static mut GDT: GlobalDescriptorTable = GlobalDescriptorTable::new();

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

//const GDT_TSS_LO: usize = 3;
//const GDT_TSS_HI: usize = 4;

#[allow(static_mut_refs, unsafe_op_in_unsafe_fn)]
pub unsafe fn cpu_init() {
    GDT.append(Descriptor::kernel_code_segment());
    GDT.append(Descriptor::kernel_data_segment());

    TSS.interrupt_stack_table[0] = stack_to_virt(&INTR_STACK);
    TSS.interrupt_stack_table[1] = stack_to_virt(&TRAP_STACK);
    TSS.interrupt_stack_table[2] = stack_to_virt(& NMI_STACK);

    GDT.append(Descriptor::tss_segment(&TSS));
    GDT.load();

    // Replace it with set_general_handler! when
    // https://github.com/rust-osdev/x86_64/issues/553 is fixed
    for i in 0..31 {
        IDT[i].set_handler_fn(interrupt_hdl);
    }
    IDT.invalid_tss.set_handler_fn(interrupt_code);
    IDT.segment_not_present.set_handler_fn(interrupt_code);
    IDT.stack_segment_fault.set_handler_fn(interrupt_code);
    IDT.general_protection_fault.set_handler_fn(interrupt_code);
    IDT.cp_protection_exception.set_handler_fn(interrupt_code);
    IDT.vmm_communication_exception.set_handler_fn(interrupt_code);
    IDT.security_exception.set_handler_fn(interrupt_code);
    IDT.page_fault.set_handler_fn(page_fault_handler);
//    IDT.double_fault.set_handler_addr(interrupt_code_d.to_virt_addr());

    IDT.load();
}

extern "x86-interrupt"
fn interrupt_hdl(f: InterruptStackFrame) {
    unsafe { f.iretq(); }
}

extern "x86-interrupt"
fn interrupt_code(f: InterruptStackFrame, _code: u64) {
    unsafe { f.iretq(); }
}

extern "x86-interrupt"
fn page_fault_handler(f: InterruptStackFrame, _code: PageFaultErrorCode) {
    unsafe { f.iretq(); }
}

#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn cpu_halt() -> ! {
    asm!("cli", "hlt");
    loop {}
}
