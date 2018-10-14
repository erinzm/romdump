#![no_std]
#![no_main]

extern crate panic_semihosting;
extern crate stm32f4xx_hal as hal;

use nb::block;
use cortex_m_rt::entry;
use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::stm32;
use stm32f4xx_hal::serial::{config::Config, Serial};


use core::fmt::Write;

use cortex_m_semihosting::{debug, hio};


#[entry]
fn main() -> ! {
    let mut stdout = hio::hstdout().unwrap();

    let p = hal::stm32::Peripherals::take().unwrap();

    let gpioa = p.GPIOA.split();
    let rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    let txp = gpioa.pa2.into_alternate_af7();
    let rxp = gpioa.pa3.into_alternate_af7();

    let serial = Serial::usart2(
        p.USART2,
        (txp, rxp),
        Config::default().baudrate(115_200.bps()),
        clocks,
    ).unwrap();

    // Separate out the sender and receiver of the serial port
    let (mut tx, mut rx) = serial.split();

    loop {
        // Read character and echo it back
        let received = block!(rx.read()).unwrap();
        block!(tx.write(received)).ok();
    }
}
