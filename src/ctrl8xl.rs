use core::fmt;
use embedded_hal::blocking::i2c::Write;

use crate::Register;

/// The CTRL8_XL register. Accelerometer control register 8 (r/w).
///
/// Accelerometer High-pass and Low-pass filter configuration.
pub struct Ctrl8Xl {
    pub address: u8,
    value: u8,
}

impl fmt::Display for Ctrl8Xl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Binary for Ctrl8Xl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:b}", self.value)
    }
}

impl fmt::LowerHex for Ctrl8Xl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.value, f)
    }
}

/// Sub-address of the register.
pub const ADDR: u8 = 0x17u8;

/// LPF2 on 6D function selection. Refer to Figure 23. Default value: 0 (0: ODR/2 low-pass filtered data sent to 6D interrupt function; 1: LPF2 output data sent to 6D interrupt function)
const LOW_PASS_ON_6D: u8 = 0;

/// Accelerometer slope filter / high-pass filter selection. Refer to Figure 23.
const HP_SLOPE_XL_EN: u8 = 2;

/// Enables accelerometer LPF2 and HPF fast-settling mode. The filter sets the second samples after writing this bit. Active only during device exit from power- down mode. Default value: 0 (0: disabled, 1: enabled)
const FASTSETTL_MODE_XL: u8 = 3;

/// Enables accelerometer high-pass filter reference mode (valid for high-pass path HP_SLOPE_XL_EN bit must be ‘1’). Default value: 0 (0: disabled, 1: enabled(1))
const HP_REF_MODE_XL: u8 = 4;

/// Accelerometer LPF2 and HP filter configuration and cutoff setting. Refer to Table 61.
const HPCF_XL_MASK: u8 = 0b111;
const HPCF_XL_OFFSET: u8 = 5;

/// Accelerometer LPF2 and HP filter configuration and cutoff setting. Refer to Table 61.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum HPCF_Xl {
    ODR_SLOPE_4,
    ODR_10,
    ODR_20,
    ODR_45,
    ODR_100,
    ODR_200,
    ODR_400,
    ODR_800,
}


impl Register for Ctrl8Xl {}

impl Ctrl8Xl {
    pub fn new(value: u8, address: u8) -> Self {
        Ctrl8Xl { value, address }
    }

    /// HP or LPF2 cut-off fraction of ODR: ODR / hpcf.
    pub fn hpcf(&self) -> f32 {
        match (self.value >> HPCF_XL_OFFSET) & HPCF_XL_MASK {
            0 => 4.0,
            1 => 10.,
            2 => 20.,
            3 => 45.,
            4 => 100.,
            5 => 200.,
            6 => 400.,
            7 => 800.0,
            _ => panic!("Unreachable"),
        }
    }

    pub fn set_hpcf<I2C>(
        &mut self,
        i2c: &mut I2C,
        value: HPCF_Xl,
    ) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.value &= !(HPCF_XL_MASK << HPCF_XL_OFFSET);
        self.value |= (value as u8) << HPCF_XL_OFFSET;
        self.write(i2c, self.address, ADDR, self.value)
    }

    pub fn hp_slope_xl_en(&mut self) -> bool {
        self.value & (1 << HP_SLOPE_XL_EN) != 0
    }

    pub fn set_hp_slope_xl_en<I2C>(&mut self, i2c: &mut I2C, value: bool) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.value &= !(1 << HP_SLOPE_XL_EN);
        self.value |= (value as u8) << HP_SLOPE_XL_EN;
        self.write(i2c, self.address, ADDR, self.value)
    }

    pub fn low_pass_on_6d(&mut self) -> bool {
        self.value & (1 << LOW_PASS_ON_6D) != 0
    }

    pub fn set_low_pass_on_6d<I2C>(&mut self, i2c: &mut I2C, value: bool) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.value &= !(1 << LOW_PASS_ON_6D);
        self.value |= (value as u8) << LOW_PASS_ON_6D;
        self.write(i2c, self.address, ADDR, self.value)
    }

    pub fn fastsettl_mode(&mut self) -> bool {
        self.value & (1 << LOW_PASS_ON_6D) != 0
    }

    pub fn set_fastsettl_mode<I2C>(&mut self, i2c: &mut I2C, value: bool) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.value &= !(1 << FASTSETTL_MODE_XL);
        self.value |= (value as u8) << FASTSETTL_MODE_XL;
        self.write(i2c, self.address, ADDR, self.value)
    }

    pub fn hp_ref_mode(&mut self) -> bool {
        self.value & (1 << LOW_PASS_ON_6D) != 0
    }

    pub fn set_hp_ref_mode<I2C>(&mut self, i2c: &mut I2C, value: bool) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.value &= !(1 << HP_REF_MODE_XL);
        self.value |= (value as u8) << HP_REF_MODE_XL;
        self.write(i2c, self.address, ADDR, self.value)
    }
}

