unsafe extern "C" {
    unsafe static _sidata: u32;
    unsafe static mut _sdata: u32;
    unsafe static mut _edata: u32;
    unsafe static mut _sbss: u32;
    unsafe static mut _ebss: u32;
}

#[unsafe(link_section = ".isr_vector.reset")]
#[used]
static RESET_VECTOR: unsafe extern "C" fn() -> ! = reset_handler;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reset_handler() -> ! {
    unsafe {
        let mut src = &raw const _sidata;
        let mut dst = &raw mut _sdata;
        let end = &raw mut _edata;
        while dst < end {
            dst.write(src.read());
            dst = dst.add(1);
            src = src.add(1);
        }

        let mut dst = &raw mut _sbss;
        let end = &raw mut _ebss;
        while dst < end {
            dst.write(0);
            dst = dst.add(1);
        }
    }

    let _ = crate::main();

    loop {
        core::hint::spin_loop()
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop()
    }
}
