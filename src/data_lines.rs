use hal::hal::digital::InputPin;
use stm32f4xx_hal as hal;

pin_block! {
    DataLines: [impl InputPin] = [
        a0: A0, a1: A1, a2: A2, a3: A3,
        a4: A4, a5: A5, a6: A6, a7: A7
    ];
    read -> u32 {
        a0 -> 0;
        a1 -> 1;
        a2 -> 2;
        a3 -> 3;
        a4 -> 4;
        a5 -> 5;
        a6 -> 6;
        a7 -> 7;
    }
}
