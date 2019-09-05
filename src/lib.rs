use serial::SystemPort;
use std::ffi::OsStr;
use std::time::Duration;
use std::io::{Write, Read};
use err_derive::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(display = "Error while opening serial port: {}", _0)]
    Serial(#[error(cause)] serial::Error),
    #[error(display = "Error communicating with serial port: {}", _0)]
    IO(#[error(cause)] std::io::Error),
    #[error(display = "Invalid CRC value when reading for over 8 tries")]
    CRC,
}

impl From<serial::Error> for Error {
    fn from(err: serial::Error) -> Self {
        Error::Serial(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IO(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

/// mh-z19 CO₂ sensor
///
/// ## Usage
///
/// ```
/// use mhz19::MHZ19;
///
/// fn main() {
///     let mut mhz19 = MHZ19::open("/dev/ttyUSB0").unwrap();
///     println("CO₂ readout: {} ppm", mhz19.read().unwrap());
/// }
/// ```
pub struct MHZ19 {
    port: SystemPort
}

/// Supported measure ranges
pub enum Range {
    Range2000 = 2000,
    Range5000 = 5000,
}

enum Command {
    Read = 0x86,
    Zero = 0x87,
    Span = 0x88,
    ABC = 0x79,
    Range = 0x99
}

const READ_WAIT: Duration = Duration::from_millis(100);

impl MHZ19 {
    /// Connect to the mh-z19 at the specified serial port
    pub fn open<T: AsRef<OsStr> + ?Sized>(port: &T) -> Result<Self> {
        Ok(MHZ19 {
            port: serial::open(port)?
        })
    }

    /// Read the CO2 value from the meter as ppm
    ///
    /// ## Blocking
    ///
    /// This command will wait for 100ms between sending the read command and getting the response
    /// during this the thread is blocked
    ///
    /// If the crc check of the response fails the method will retry up to 8 times
    pub fn read(&mut self) -> Result<u16> {
        let command = MHZ19::generate_command(Command::Read, 0, 0);
        let mut buffer = [0; 9];
        let mut crc_err_count = 0;

        loop {
            self.port.write(&command)?;
            std::thread::sleep(READ_WAIT);
            self.port.read(&mut buffer)?;
            let crc = MHZ19::crc8(&buffer);
            if crc != buffer[8] {
                crc_err_count += 1;
                // flush
                let _ = self.port.read(&mut buffer);
                if crc_err_count > 8 {
                    return Err(Error::CRC);
                }
            } else {
                crc_err_count = 0;
                if buffer[0] == 0xff && buffer[1] == 0x86 {
                    return Ok(u16::from_be_bytes([buffer[2], buffer[3]]));
                }
            }
            std::thread::sleep(READ_WAIT);
        }
    }

    /// Tell the mh-z19 to zero-point calibrate
    ///
    /// Sensor should be at 400ppm when calibrating
    pub fn zero_calibrate(&mut self) -> Result<()> {
        self.port.write(&MHZ19::generate_command(Command::Zero, 0, 0))?;
        Ok(())
    }

    /// Tell the mh-z19 to span-point calibrate
    ///
    /// Sensor should be at target level when calibrating
    pub fn span_calibrate(&mut self, value: u16) -> Result<()> {
        let value_bytes = value.to_be_bytes();
        self.port.write(&MHZ19::generate_command(Command::Span, value_bytes[0], value_bytes[1]))?;
        Ok(())
    }

    /// Enable or disable automatic baseline correction
    ///
    /// Automatic baseline correction will automatically adjust the baseline value every 24h after power on
    /// to the "standard" value of 400ppm based on the lowest values measured each cycle.
    ///
    /// This is suitable for situations like home or office buildings where no people are present for
    /// multiple hours each day allowing the CO₂ values to come down to outside levels
    ///
    /// For units produced after 2015 this should be enabled by default
    pub fn enable_abc(&mut self, enable: bool) -> Result<()> {
        self.port.write(&MHZ19::generate_command(Command::ABC, if enable { 0xa0 } else { 0x00 }, 0))?;
        Ok(())
    }

    /// Set the detection range for the sensor
    ///
    /// A range of 2000ppm or 5000ppm is supported
    pub fn set_range(&mut self, range: Range) -> Result<()> {
        let value_bytes = (range as u16).to_be_bytes();
        self.port.write(&MHZ19::generate_command(Command::Range, value_bytes[0], value_bytes[1]))?;
        Ok(())
    }

    fn generate_command(command: Command, data1: u8, data2: u8) -> [u8; 9] {
        let mut command = [0xff, 0x01, command as u8, data1, data2, 0x00, 0x00, 0x00, 0x00];
        command[8] = MHZ19::crc8(&command);
        command
    }

    fn crc8(data: &[u8]) -> u8 {
        let mut crc: u8 = 0;
        for i in 1..8 {
            crc = crc.wrapping_add(data[i]);
        }
        crc = !crc;
        crc.wrapping_add(1)
    }
}
