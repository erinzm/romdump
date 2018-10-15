#![feature(uniform_paths)]
#![no_std]
#![no_main]

extern crate panic_semihosting;
// extern crate panic_halt;
// extern crate stm32f4xx_hal;
use stm32f4xx_hal as hal;

mod address_lines;

use cortex_m_rt::entry;
use nb::block;
use hal::hal::digital::OutputPin;
use hal::prelude::*;
use hal::serial::{config::Config, Serial};
use hal::stm32;

use core::fmt::Write;
use cortex_m_semihosting::{debug, hio};

use address_lines::AddressLines;

trait OutputPinBool: OutputPin {
    fn set_value(&mut self, val: bool) {
        if val {
            self.set_high()
        } else {
            self.set_low()
        }
    }
}

impl<T> OutputPinBool for T where T: OutputPin {}


#[entry]
fn main() -> ! {
    let mut stdout = hio::hstdout().unwrap();

    let p = hal::stm32::Peripherals::take().unwrap();

    let gpioa = p.GPIOA.split();
    let gpiob = p.GPIOB.split();
    let gpioc = p.GPIOC.split();
    let rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    let txp = gpioa.pa2.into_alternate_af7();
    let rxp = gpioa.pa3.into_alternate_af7();

    let serial = Serial::usart2(
        p.USART2,
        (txp, rxp),
        Config::default().baudrate(115_200.bps()),
        clocks,
    )
    .unwrap();

    // Separate out the sender and receiver of the serial port
    let (mut tx, mut rx_) = serial.split();

    // ---- pins for ROM ----
    let mut address_lines = AddressLines::new(
        gpioa.pa5.into_push_pull_output(),
        gpioa.pa6.into_push_pull_output(),
        gpioa.pa7.into_push_pull_output(),
        gpiob.pb6.into_push_pull_output(),
        gpioc.pc7.into_push_pull_output(),
        gpioa.pa9.into_push_pull_output(),
        gpioa.pa8.into_push_pull_output(),
        gpiob.pb10.into_push_pull_output(),
    );

    address_lines.write(0xaa);

    loop {}
}
