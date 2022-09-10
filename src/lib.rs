#![no_std]

use embedded_hal as hal;
use num_enum::{TryFromPrimitive};
use core::convert::TryFrom;

pub struct BMA4xx<I2C, InterruptPin> {
    i2c: I2C,
    address: I2C_Address,
    interrupt_pin1: Option<InterruptPin>,
    interrupt_pin2: Option<InterruptPin>,
    chip: CHIP,
}

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Copy,Clone)]
pub enum I2C_Address {
    /// https://www.bosch-sensortec.com/media/boschsensortec/downloads/datasheets/bst-bma400-ds000.pdf#page=108
    BMA400 = 0b001_0100,
    /// https://www.bosch-sensortec.com/media/boschsensortec/downloads/datasheets/bst-bma400-ds000.pdf#page=108
    /// chosen by pin-strapping SDO to VDDIO
    BMA400_ALTERNATIVE = 0b001_0101,
    /// https://www.bosch-sensortec.com/media/boschsensortec/downloads/datasheets/bst-bma456-ds000.pdf#page=82
    /// also used by BMA421 and BMA425
    BMA456 = 0b001_1000,
    /// https://www.bosch-sensortec.com/media/boschsensortec/downloads/datasheets/bst-bma456-ds000.pdf#page=82
    /// also used by BMA421 and BMA425
    /// chosen by pin-strapping SDO to VDDIO
    BMA456_ALTERNATIVE = 0b001_1001,
}

/// created per https://github.com/InfiniTimeOrg/InfiniTime/tree/develop/src/drivers
impl<I2C, CommunicationError, InterruptPin> BMA4xx<I2C, InterruptPin>
    where
        I2C: hal::blocking::i2c::Write<Error = CommunicationError>
            + hal::blocking::i2c::Read<Error = CommunicationError>
            + hal::blocking::i2c::WriteRead<Error = CommunicationError>,
        InterruptPin: hal::digital::v2::InputPin,
{
    pub fn new<'a>(i2c: I2C,
               address: I2C_Address,
               interrupt_pin1: Option<InterruptPin>,
               interrupt_pin2: Option<InterruptPin>,
    ) -> Result<Self, Error<CommunicationError>>
    {
        let mut _self = Self{
            i2c,
            address,
            interrupt_pin1,
            interrupt_pin2,
            chip: CHIP::BMA400,
        };

        // verify chip id
        let chip_id = _self.read_register(Registers::CHIP_ID)?;
        match CHIP::try_from(chip_id) {
            Ok(chip)    => _self.chip = chip,
            Err(_) => return Err(Error::UnknownChipId(chip_id))
        }

        Ok(_self)
    }

    fn read_register(&mut self, register: Registers ) -> Result<u8, Error<CommunicationError>> {
        let request = &[register as u8];
        let mut response:[u8;1] = [0;1];
        self.i2c.write_read(self.address as u8, request, &mut response).map_err(Error::I2c)?;
        Ok(response[0])
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Error<CommunicationError> {
    /// Underlying I2C device error
    I2c(CommunicationError),

    /// unrecognized BMA chip id
    UnknownChipId(u8),
    
    /// Device failed to resume from reset
    ResetTimeout
}

#[derive(TryFromPrimitive)]
#[repr(u8)]
enum CHIP {
    /// https://www.bosch-sensortec.com/media/boschsensortec/downloads/datasheets/bst-bma456-ds000.pdf#page=11
    BMA456  = 0x16,
    /// https://www.bosch-sensortec.com/media/boschsensortec/downloads/datasheets/bst-bma400-ds000.pdf#page=14
    BMA400  = 0x90,
    /// TODO find datasheet
    BMA421  = 0x11,
    /// https://datasheet.lcsc.com/lcsc/1912111437_Bosch-Sensortec-BMA425_C437656.pdf#page=11
    BMA425  = 0x13,
}

#[allow(unused)]
#[allow(non_camel_case_types)]
#[repr(u8)]
/// https://www.bosch-sensortec.com/media/boschsensortec/downloads/datasheets/bst-bma400-ds000.pdf#page=50
enum Registers {
    CHIP_ID         = 0x00,
    ERR_REG         = 0x02,
    STATUS          = 0x03,
    DATA_0          = 0x0A,
    DATA_1          = 0x0B,
    DATA_2          = 0x0C,
    DATA_3          = 0x0D,
    DATA_4          = 0x0E,
    DATA_5          = 0x0F,
    DATA_6          = 0x10,
    DATA_7          = 0x11,
    DATA_8          = 0x12,
    DATA_9          = 0x13,
    DATA_10         = 0x14,
    DATA_11         = 0x15,
    DATA_12         = 0x16,
    DATA_13         = 0x17,
    SENSOR_TIME_0   = 0x18,
    SENSOR_TIME_1   = 0x19,
    SENSOR_TIME_2   = 0x1A,
    EVENT           = 0x1B,
    INT_STATUS_0    = 0x1C,
    INT_STATUS_1    = 0x1D,
    STEP_COUNTER_0  = 0x1E,
    STEP_COUNTER_1  = 0x1F,
    STEP_COUNTER_2  = 0x20,
    STEP_COUNTER_3  = 0x21,
    TEMPERATURE     = 0x22,
    FIFO_LENGTH_0   = 0x24,
    FIFO_LENGTH_1   = 0x25,
    FIFO_DATA       = 0x26,
    ACTIVITY_TYPE   = 0x27,
    INTERNAL_STATUS = 0x2A,
    ACC_CONF        = 0x40,
    ACC_RANGE       = 0x41,
    AUX_CONF        = 0x44,
    FIFO_DOWNS      = 0x45,
    FIFO_WTM_0      = 0x46,
    FIFO_WTM_1      = 0x47,
    FIFO_CONFIG_0   = 0x48,
    FIFO_CONFIG_1   = 0x49,
    AUX_DEV_ID      = 0x4B,
    AUX_IF_CONF     = 0x4C,
    AUX_RD_ADDR     = 0x4D,
    AUX_WR_ADDR     = 0x4E,
    AUX_WR_DATA     = 0x4F,
    INT1_IO_CTRL    = 0x53,
    INT2_IO_CTRL    = 0x54,
    INT_LATCH       = 0x55,
    INT1_MAP        = 0x56,
    INT2_MAP        = 0x57,
    INT_MAP_DATA    = 0x58,
    INIT_CTRL       = 0x59,
    FEATURES_IN     = 0x5E,
    INTERNAL_ERROR  = 0x5F,
    NVM_CONF        = 0x6A,
    IF_CONF         = 0x6B,
    ACC_SELF_TEST   = 0x6D,
    NV_CONF         = 0x70,
    OFFSET_0        = 0x71,
    OFFSET_1        = 0x72,
    OFFSET_2        = 0x73,
    PWR_CONF        = 0x7C,
    PWR_CTRL        = 0x7D,
    CMD             = 0x7E,
}
