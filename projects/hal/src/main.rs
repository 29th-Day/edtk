#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal::{
    pac::{CorePeripherals, Peripherals},
    prelude::*,
};

#[entry]
fn main() -> ! {
    let cp = CorePeripherals::take().unwrap();
    let p = Peripherals::take().unwrap();

    let mut rcc = p.RCC.constrain();
    let gpiod = p.GPIOD.split(&mut rcc);
    let mut led = gpiod.pd12.into_push_pull_output();

    let mut delay = cp.SYST.delay(&rcc.clocks);

    loop {
        led.set_high();
        delay.delay(1.secs());

        led.set_low();
        delay.delay(1.secs());
    }

    // loop {
    //     cortex_m::asm::wfi();
    // }
}

fn delay() {
    for _ in 0..1_000_000 {
        core::hint::spin_loop();
    }
}

#[allow(unused)]
fn pac() -> ! {
    use stm32f4::stm32f405::Peripherals;

    let peripherals = Peripherals::take().unwrap();

    let rcc = &peripherals.RCC;
    let gpiod = &peripherals.GPIOD;

    rcc.ahb1enr().modify(|_, w| w.gpioden().enabled());
    gpiod.moder().modify(|_, w| w.moder12().output());

    loop {
        gpiod.bsrr().write(|w| w.bs12().set_bit());
        delay();

        gpiod.bsrr().write(|w| w.br12().set_bit());
        delay();
    }
}
