//! Access to various system and model specific registers.

pub mod control;
pub mod model_specific;
pub mod rflags;

/// Gets the current instruction pointer. Note that this is only approximate as it requires a few
/// instructions to execute.
#[cfg(feature = "inline_asm")]
#[inline(always)]
pub fn read_rip() -> u64 {
    let rip: u64;
    unsafe {
        llvm_asm!(
            "lea (%rip), $0"
            : "=r"(rip) ::: "volatile"
        );
    }
    rip
}

/// Get the current rsp value
#[cfg(feature = "inline_asm")]
#[inline(always)]
pub fn read_rsp() -> u64 {
    let rsp: u64;
    unsafe {
       asm!(
       "mov $0, rsp":"=r"(rsp):::"volatile", "intel"
       );
    }
    rsp
}