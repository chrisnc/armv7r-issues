//! An example program for QEMU's Armv8-R Virtual Machine
//!
//! Written by Jonathan Pallant at Ferrous Systems
//!
//! Copyright (c) Ferrous Systems, 2024

#![no_std]
#![no_main]

/// The entry-point to the Rust application.
///
/// It is called by the start-up code below.
#[no_mangle]
pub extern "C" fn kmain() {
    panic!("I am a panic");
}

/// Called when the application raises an unrecoverable `panic!`.
///
/// Exits QEMU using a semihosting breakpoint.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    const SYS_REPORTEXC: u32 = 0x18;
    loop {
        // Exit, using semihosting
        unsafe {
            core::arch::asm!(
                "svc 0x123456",
                in("r0") SYS_REPORTEXC,
                in("r1") 0x20026
            )
        }
    }
}

core::arch::global_asm!(
    r#"

.section .text.startup
.global _start
.code 32
.align 0

_start:
    // Set stack pointer
    ldr r3, =stack_top
    mov sp, r3
    // Allow VFP coprocessor access
    mrc p15, 0, r0, c1, c0, 2
    orr r0, r0, #0xF00000
    mcr p15, 0, r0, c1, c0, 2
    // Enable VFP
    mov r0, #0x40000000
    vmsr fpexc, r0
    // Jump to application
    bl kmain
    // In case the application returns, loop forever
    b .

"#
);

// End of file
