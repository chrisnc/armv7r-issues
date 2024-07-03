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
    loop {
    }
}

// This part fails to compile in `--release` (or with `-Copt-level=1`), unless you
// also set `-Ccodegen-units=1`.
core::arch::global_asm!(
    r#"

.section .text.startup
.global _start

_start:
    lr.w t0, 0(t1)
    j .

"#
);
