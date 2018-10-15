use crate::OutputPinBool;
use stm32f4xx_hal as hal;
use hal::hal::digital::OutputPin;

pub struct AddressLines<A1, A2, A3, A4, A5, A6, A7, A8> {
    /// LSB
    a1: A1,
    a2: A2,
    a3: A3,
    a4: A4,
    a5: A5,
    a6: A6,
    a7: A7,
    /// MSB
    a8: A8,
}

impl<A1, A2, A3, A4, A5, A6, A7, A8> AddressLines<A1, A2, A3, A4, A5, A6, A7, A8>
where
    A1: OutputPin,
    A2: OutputPin,
    A3: OutputPin,
    A4: OutputPin,
    A5: OutputPin,
    A6: OutputPin,
    A7: OutputPin,
    A8: OutputPin,
{
    pub fn new(
        a1: A1,
        a2: A2,
        a3: A3,
        a4: A4,
        a5: A5,
        a6: A6,
        a7: A7,
        a8: A8,
    ) -> AddressLines<A1, A2, A3, A4, A5, A6, A7, A8> {
        AddressLines {
            a1: a1,
            a2: a2,
            a3: a3,
            a4: a4,
            a5: a5,
            a6: a6,
            a7: a7,
            a8: a8,
        }
    }

    pub fn write(&mut self, v: u8) {
        self.a1.set_value(v & (1 << 0) != 0);
        self.a2.set_value(v & (1 << 1) != 0);
        self.a3.set_value(v & (1 << 2) != 0);
        self.a4.set_value(v & (1 << 3) != 0);
        self.a5.set_value(v & (1 << 4) != 0);
        self.a6.set_value(v & (1 << 5) != 0);
        self.a7.set_value(v & (1 << 6) != 0);
        self.a8.set_value(v & (1 << 7) != 0);
    }
}
