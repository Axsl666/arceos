use crate::time::Hertz;
use core::ops::Deref;
use embedded_hal::i2c::{ErrorKind, ErrorType, NoAcknowledgeSource, Operation, SevenBitAddress};

/// I2C error
#[derive(Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// Bus error
    Bus,
    /// Arbitration loss
    Arbitration,
    /// No ack received
    Acknowledge,
    /// Overrun/underrun
    Overrun,
    /// Timed out waiting for something.
    Timeout,
    // Pec, // SMBUS mode only
    // Timeout, // SMBUS mode only
    // Alert, // SMBUS mode only
}

impl embedded_hal::i2c::Error for Error {
    fn kind(&self) -> ErrorKind {
        match self {
            Self::Bus => ErrorKind::Bus,
            Self::Arbitration => ErrorKind::ArbitrationLoss,
            Self::Acknowledge => ErrorKind::NoAcknowledge(NoAcknowledgeSource::Unknown),
            Self::Overrun => ErrorKind::Overrun,
            Self::Timeout => ErrorKind::Other,
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum DutyCycle {
    Ratio2to1,
    Ratio16to9,
}

#[derive(PartialEq, Debug)]
pub enum Mode {
    Standard {
        frequency: Hertz,
    },
    Fast {
        frequency: Hertz,
        duty_cycle: DutyCycle,
    },
    FastPlus {
        frequency: Hertz,
        duty_cycle: DutyCycle,
    },
}

impl Mode {
    pub fn standard<F: Into<Hertz>>(frequency: F) -> Self {
        Mode::Standard {
            frequency: frequency.into(),
        }
    }

    pub fn fast<F: Into<Hertz>>(frequency: F, duty_cycle: DutyCycle) -> Self {
        Mode::Fast {
            frequency: frequency.into(),
            duty_cycle,
        }
    }

    pub fn fastplus<F: Into<Hertz>>(frequency: F, duty_cycle: DutyCycle) -> Self {
        Mode::FastPlus {
            frequency: frequency.into(),
            duty_cycle,
        }
    }

    pub fn get_frequency(&self) -> Hertz {
        match *self {
            Mode::Standard { frequency } => frequency,
            Mode::Fast { frequency, .. } => frequency,
            Mode::FastPlus { frequency, .. } => frequency,
        }
    }
}

/// Marker trait for possible SCL pins for an I2C module.
pub trait SclPin<I2C> {}

/// Marker trait for possible SDA pins for an I2C module.
pub trait SdaPin<I2C> {}


/// I2C peripheral operating in master mode
pub struct I2c<I2C, SCLPIN, SDAPIN> {
    i2c: I2C,
    scl_pin: SCLPIN,
    sda_pin: SDAPIN,
    mode: Mode,
    pclk1: u32,
}


/// embedded-hal compatible blocking I2C implementation
///
/// **NOTE**: Before using blocking I2C, you need to enable the DWT cycle counter using the
/// [DWT::enable_cycle_counter] method.
pub struct BlockingI2c<I2C, SCLPIN, SDAPIN> {
    nb: I2c<I2C, SCLPIN, SDAPIN>,
    start_timeout: u32,
    start_retries: u8,
    addr_timeout: u32,
    data_timeout: u32,
}

macro_rules! i2c_impl {
    ($I2Cn:ty, $i2cn:ident, $APBn:ty) => {
        impl<SCLPIN, SDAPIN> I2c<$I2Cn, SCLPIN, SDAPIN> {
            /// Creates a generic I2Cn object on the given pins.
            pub fn $i2cn(
                i2c: $I2Cn,
                scl_pin: SCLPIN,
                sda_pin: SDAPIN,
                mode: Mode,
                clocks: Clocks,
                apb: &mut $APBn,
            ) -> Self
            where
                SCLPIN: SclPin<$I2Cn>,
                SDAPIN: SdaPin<$I2Cn>,
            {
                I2c::<$I2Cn, _, _>::create_internal(i2c, scl_pin, sda_pin, mode, clocks, apb)
            }
        }

        impl<SCLPIN, SDAPIN> BlockingI2c<$I2Cn, SCLPIN, SDAPIN> {
            /// Creates a blocking I2Cn object on the given pins using the embedded-hal `BlockingI2c` trait.
            #[allow(clippy::too_many_arguments)]
            pub fn $i2cn(
                i2c: $I2Cn,
                scl_pin: SCLPIN,
                sda_pin: SDAPIN,
                mode: Mode,
                clocks: Clocks,
                apb: &mut $APBn,
                start_timeout_us: u32,
                start_retries: u8,
                addr_timeout_us: u32,
                data_timeout_us: u32,
            ) -> Self
            where
                SCLPIN: SclPin<$I2Cn>,
                SDAPIN: SdaPin<$I2Cn>,
            {
                BlockingI2c::<$I2Cn, _, _>::create_internal(
                    i2c,
                    scl_pin,
                    sda_pin,
                    mode,
                    clocks,
                    apb,
                    start_timeout_us,
                    start_retries,
                    addr_timeout_us,
                    data_timeout_us,
                )
            }
        }
    };
}
