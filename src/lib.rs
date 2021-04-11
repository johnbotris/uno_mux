#![no_std]

use avr_hal_generic::hal::{
    adc::Channel,
    digital::v2::{InputPin, OutputPin},
};

use ufmt::{uDebug, uDisplay, uWrite, uwrite, Formatter};

use u4::U4;

/// A wrapper for using avr hal devices on a 16 channel analogue multiplexer CD74HC4067 or any other compatible mux
pub struct Multiplexer<S0, S1, S2, S3, IO, EN> {
    select0: S0,
    select1: S1,
    select2: S2,
    select3: S3,
    io: IO,
    enable: EN,
}

impl<S0, S1, S2, S3, IO, EN> Multiplexer<S0, S1, S2, S3, IO, EN> {
    /// Create a new Multiplexer
    ///
    /// ```
    /// let dp = arduino_uno::Peripherals::take().unwrap();
    /// let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);
    ///
    /// // Create a new mux
    /// // the enable pin is not bound
    /// let mut mux = Multiplexer::new(
    ///     pins.d2.into_output(&mut pins.ddr),
    ///     pins.d3.into_output(&mut pins.ddr),
    ///     pins.d4.into_output(&mut pins.ddr),
    ///     pins.d5.into_output(&mut pins.ddr),
    ///     pins.a0.into_output(&mut pins.ddr),
    ///     ()
    /// );
    /// ```
    pub fn new(select0: S0, select1: S1, select2: S2, select3: S3, io: IO, enable: EN) -> Self {
        Self {
            select0,
            select1,
            select2,
            select3,
            io,
            enable,
        }
    }

    /// Select a channel
    pub fn select(
        &mut self,
        selection: U4,
    ) -> Result<(), MultiplexSelectionError<S0::Error, S1::Error, S2::Error, S3::Error>>
    where
        S0: OutputPin,
        S1: OutputPin,
        S2: OutputPin,
        S3: OutputPin,
    {
        let selection: u16 = selection.into();
        set_pin(&mut self.select0, (selection >> 0 & 1) != 0)
            .map_err(MultiplexSelectionError::Select0)?;

        set_pin(&mut self.select1, (selection >> 1 & 1) != 0)
            .map_err(MultiplexSelectionError::Select1)?;

        set_pin(&mut self.select2, (selection >> 2 & 1) != 0)
            .map_err(MultiplexSelectionError::Select2)?;

        set_pin(&mut self.select3, (selection >> 3 & 1) != 0)
            .map_err(MultiplexSelectionError::Select3)?;

        Ok(())
    }

    /// Enable standby for selection pins. EN pin is enable low
    pub fn enable(&mut self) -> Result<(), EN::Error>
    where
        EN: OutputPin,
    {
        self.enable.set_low()
    }

    /// Disable selection pins. EN pin is disable high
    pub fn disable(&mut self) -> Result<(), EN::Error>
    where
        EN: OutputPin,
    {
        self.enable.set_high()
    }
}

fn set_pin<PIN: OutputPin>(pin: &mut PIN, on: bool) -> Result<(), PIN::Error> {
    if on {
        pin.set_high()
    } else {
        pin.set_low()
    }
}

impl<S0, S1, S2, S3, IO, EN> OutputPin for Multiplexer<S0, S1, S2, S3, IO, EN>
where
    IO: OutputPin,
{
    type Error = IO::Error;

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.io.set_high()
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.io.set_low()
    }
}

impl<S0, S1, S2, S3, IO, EN> InputPin for Multiplexer<S0, S1, S2, S3, IO, EN>
where
    IO: InputPin,
{
    type Error = IO::Error;

    fn is_high(&self) -> Result<bool, Self::Error> {
        self.io.is_high()
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        self.io.is_high()
    }
}

impl<ADC, S0, S1, S2, S3, IO, EN> Channel<ADC> for Multiplexer<S0, S1, S2, S3, IO, EN>
where
    IO: Channel<ADC>,
{
    type ID = IO::ID;

    fn channel() -> Self::ID {
        IO::channel()
    }
}

pub enum MultiplexSelectionError<E0, E1, E2, E3> {
    Select0(E0),
    Select1(E1),
    Select2(E2),
    Select3(E3),
}

impl<E0, E1, E2, E3> uDebug for MultiplexSelectionError<E0, E1, E2, E3>
where
    E0: uDebug,
    E1: uDebug,
    E2: uDebug,
    E3: uDebug,
{
    fn fmt<W: ?Sized>(&self, f: &mut Formatter<W>) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        use MultiplexSelectionError::*;
        match self {
            Select0(e) => uwrite!(f, "Select0({:?})", e),
            Select1(e) => uwrite!(f, "Select1({:?})", e),
            Select2(e) => uwrite!(f, "Select2({:?})", e),
            Select3(e) => uwrite!(f, "Select3({:?})", e),
        }
    }
}

impl<E0, E1, E2, E3> uDisplay for MultiplexSelectionError<E0, E1, E2, E3>
where
    E0: uDisplay,
    E1: uDisplay,
    E2: uDisplay,
    E3: uDisplay,
{
    fn fmt<W: ?Sized>(&self, f: &mut Formatter<W>) -> Result<(), W::Error>
    where
        W: uWrite,
    {
        use MultiplexSelectionError::*;
        match self {
            Select0(e) => uwrite!(f, "Select0({})", e),
            Select1(e) => uwrite!(f, "Select1({})", e),
            Select2(e) => uwrite!(f, "Select2({})", e),
            Select3(e) => uwrite!(f, "Select3({})", e),
        }
    }
}

pub mod u4 {
    use core::convert::TryFrom;
    use ufmt::{uDisplay, uWrite, uwrite, Formatter};

    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
    pub struct U4(u16);

    impl U4 {
        pub const MAX: U4 = U4(15);

        pub const ZERO: U4 = U4(0);
        pub const ONE: U4 = U4(1);
        pub const TWO: U4 = U4(2);
        pub const THREE: U4 = U4(3);
        pub const FOUR: U4 = U4(4);
        pub const FIVE: U4 = U4(5);
        pub const SIX: U4 = U4(6);
        pub const SEVEN: U4 = U4(7);
        pub const EIGHT: U4 = U4(8);
        pub const NINE: U4 = U4(9);
        pub const TEN: U4 = U4(10);
        pub const ELEVEN: U4 = U4(11);
        pub const TWELVE: U4 = U4(12);
        pub const THIRTEEN: U4 = U4(13);
        pub const FOURTEEN: U4 = U4(14);
        pub const FIFTEEN: U4 = U4(15);

        pub fn truncated(val: u16) -> U4 {
            U4(val % U4::MAX.0)
        }
    }

    impl From<U4> for u16 {
        fn from(u4: U4) -> u16 {
            u4.0
        }
    }

    impl TryFrom<u16> for U4 {
        type Error = ();
        fn try_from(val: u16) -> Result<U4, Self::Error> {
            if val <= U4::MAX.0 {
                Ok(U4(val))
            } else {
                Err(())
            }
        }
    }

    impl uDisplay for U4 {
        fn fmt<W: ?Sized>(&self, f: &mut Formatter<W>) -> Result<(), W::Error>
        where
            W: uWrite,
        {
            uwrite!(f, "{}u4", self.0)
        }
    }
}
