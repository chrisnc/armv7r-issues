# Ferrocene for 32-bit Arm Cortex-R bare-metal Demo

This demo application for Armv7-R demonstrates a compilation problem in
`--release` mode.

`cargo run` will work but `cargo run --release` gives this error:

```text
   Compiling qemu-armv7r v0.1.0 (/Users/jonathan/Documents/ferrous-systems/qemu-armv7r)
error: <inline asm>:18:5: instruction requires: VFP2
    vmsr fpexc, r0
    ^

error: could not compile `qemu-armv7r` (bin "qemu-armv7r")
```

This doesn't make sense as in an `eabihf` target, the FPU is mandatory, and this instruction is given in the Armv7-A and Armv7-R Architecture Specification as the one to use to copy from ARM core registers to either Advanced SIMD or Floating-point Extension System Registers. Also, it works fine in debug profile?!

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](./LICENSE-MIT) or
<http://opensource.org/licenses/MIT>) at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
