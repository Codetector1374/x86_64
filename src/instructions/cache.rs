
//! Cache related instructions

/// Invalid all data cache (Flush)
#[inline]
pub fn wbinvd() {
    #[cfg(feature = "inline_asm")]
    unsafe {
        asm!("wbinvd" :::: "volatile");
    }
    #[cfg(not(feature = "inline_asm"))]
    unsafe {
        panic!("unsupport");
    }
}
