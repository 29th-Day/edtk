#![no_std]
#![no_main]

mod rt;

const RCC_AHB1ENR: *mut u32 = 0x4002_3830 as *mut u32;
const GPIOD_MODER: *mut u32 = 0x4002_0C00 as *mut u32;
const GPIOD_BSRR: *mut u32 = 0x4002_0C18 as *mut u32;

const GPIODEN: u32 = 1 << 3;

const GPIO_12_MODE_OUTPUT: u32 = 0b01 << 24;
const GPIO_12_SET: u32 = 1 << 12;
const GPIO_12_RESET: u32 = 1 << 28;

fn delay() {
    for _ in 0..1_000_000 {
        core::hint::spin_loop();
    }
}

fn main() {
    unsafe {
        RCC_AHB1ENR.write_volatile(RCC_AHB1ENR.read_volatile() | GPIODEN);
        GPIOD_MODER.write_volatile(GPIOD_MODER.read_volatile() | GPIO_12_MODE_OUTPUT);

        loop {
            GPIOD_BSRR.write_volatile(GPIO_12_SET);
            delay();

            GPIOD_BSRR.write_volatile(GPIO_12_RESET);
            delay();
        }
    }
}
