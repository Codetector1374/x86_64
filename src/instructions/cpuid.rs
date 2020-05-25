//! CPUID Instructions

/// Get the MFG String
#[inline]
pub fn mfgid() -> [u8; 12] {
    #[cfg(feature = "inline_asm")]
        let ebx: u32;
    let ecx: u32;
    let edx: u32;
    let mode = 0;
    unsafe {
        asm!("cpuid" :"={ebx}"(ebx), "={ecx}"(ecx), "={edx}"(edx):"rax"(mode)::"volatile");
    }
    #[cfg(not(feature = "inline_asm"))]
        unsafe {
        panic!("unsupport");
    }
    unsafe {
        core::mem::transmute([
            ebx, edx, ecx
        ])
    }
}
