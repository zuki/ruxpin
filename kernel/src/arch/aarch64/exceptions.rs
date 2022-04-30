
use core::arch::asm;

use crate::irqs;
use crate::printkln;


pub type IrqFlags = u64;

pub unsafe fn enable_irq(flags: IrqFlags) {
    asm!(
        "msr    DAIF, {:x}",
        in(reg) flags
    );
}

pub unsafe fn disable_irq() -> IrqFlags {
    let mut flags;
    asm!(
        "mrs    {:x}, DAIF",
        "msr    DAIFset, #0xf",
        out(reg) flags,
    );
    flags
}

#[allow(dead_code)]
pub fn enable_all_irq() {
    unsafe {
        asm!("msr    DAIFclr, #0xf");
    }
}

#[allow(dead_code)]
pub fn disable_all_irq() {
    unsafe {
        asm!("msr    DAIFset, #0xf");
    }
}


#[no_mangle]
pub extern "C" fn fatal_error(elr: u64, esr: u64, far: u64) -> ! {
    printkln!("Fatal Error: ESR: {:#x}, FAR: {:#x}, ELR: {:#x}", esr, far, elr);
    loop {}
}

#[no_mangle]
extern "C" fn handle_user_exception(_context: u64, elr: u64, esr: u64, far: u64, _sp: u64) {
    //printkln!("Handle an exception of {:x} for sp {:x}", esr, sp);

    match esr >> 26 {
        // SVC from Aarch64
        0b010101 => {
            use crate::api::handle_syscall;
            handle_syscall();
        },

        // Instruction or Data Abort from lower EL
        0b100000 | 0b100100 => {
            if esr & 0b111100 == 0b001000 {
                printkln!("Instruction or Data Abort caused by Access Flag at address {:x} (allocating new page)", far);
                use crate::proc::process::page_fault_handler;
                page_fault_handler(far);
            } else {
                fatal_error(elr, esr, far);
            }
        },

        _ => {
            fatal_error(elr, esr, far);
        }
    }

    enable_all_irq();
    crate::tasklets::run_tasklets();
    disable_all_irq();
}

#[no_mangle]
extern "C" fn handle_kernel_exception(_context: u64, elr: u64, esr: u64, far: u64) {
    printkln!("Handle a kernel exception of {:x} for far {:x} at {:x}", esr, far, elr);

    match esr >> 26 {
        // Instruction or Data Abort from lower EL
        0b100000 | 0b100100 | 0b100101 => {
            if esr & 0b111100 == 0b001000 {
                printkln!("Instruction or Data Abort caused by Access Flag at address {:x} (allocating new page)", far);
                use crate::proc::process::page_fault_handler;
                page_fault_handler(far);
            } else {
                fatal_error(elr, esr, far);
            }
        },

        _ => {
            fatal_error(elr, esr, far);
        }
    }
}

#[no_mangle]
extern "C" fn handle_irq(_context: u64, _elr: u64, _esr: u64, _far: u64, _sp: u64) {
    //printkln!("Handle an irq of {:x} for sp {:x}", _esr, _sp);

    irqs::handle_irqs();

    enable_all_irq();
    crate::tasklets::run_tasklets();
    disable_all_irq();
}

