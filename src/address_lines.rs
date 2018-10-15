use crate::OutputPinBool;
use hal::hal::digital::OutputPin;
use stm32f4xx_hal as hal;

pin_block! {
    AddressLines: [impl OutputPin] = [
        a0: A0, a1: A1, a2: A2, a3: A3, a4: A4,
        a5: A5, a6: A6, a7: A7, a8: A8, a9: A9,
        a10: A10, a11: A11, a12: A12, a13: A13,
        a14: A14, a15: A15, a16: A16, a17: A17,
        a18: A18
    ];
    write(u32) {
        a0  = 0;
        a1  = 1;
        a2  = 2;
        a3  = 3;
        a4  = 4;
        a5  = 5;
        a6  = 6;
        a7  = 7;
        a8  = 8;
        a9  = 9;
        a10 = 10;
        a11 = 11;
        a12 = 12;
        a13 = 13;
        a14 = 14;
        a15 = 15;
        a16 = 16;
        a17 = 17;
        a18 = 18;
    }
}