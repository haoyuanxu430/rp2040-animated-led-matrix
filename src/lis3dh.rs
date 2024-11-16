use embedded_hal::blocking::i2c::{Write, WriteRead};

pub struct Lis3dh<I2C> {
    i2c: I2C,
    addr: u8,
}

impl<I2C, E> Lis3dh<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    pub const ADDR: u8 = 0x18;
    
    // Register addresses
    const WHO_AM_I: u8 = 0x0F;
    const CTRL_REG1: u8 = 0x20;
    const CTRL_REG4: u8 = 0x23;
    const TEMP_CFG_REG: u8 = 0x1F;
    const OUT_X_L: u8 = 0x28;
    const OUT_X_H: u8 = 0x29;
    const OUT_Y_L: u8 = 0x2A;
    const OUT_Y_H: u8 = 0x2B;
    const OUT_Z_L: u8 = 0x2C;
    const OUT_Z_H: u8 = 0x2D;

    pub fn new(i2c: I2C) -> Self {
        Self {
            i2c,
            addr: Self::ADDR,
        }
    }

    pub fn init(&mut self) -> Result<(), E> {
        // Normal mode and 1.344kHz data rate
        self.write_reg(Self::CTRL_REG1, 0b10010111)?;
        // Block data update on
        self.write_reg(Self::CTRL_REG4, 0b10000000)?;
        // Auxiliary ADC on
        self.write_reg(Self::TEMP_CFG_REG, 0b11000000)?;
        Ok(())
    }

    pub fn read_accel(&mut self) -> Result<(f32, f32, f32), E> {
        let x = self.read_axis(Self::OUT_X_L, Self::OUT_X_H)?;
        let y = self.read_axis(Self::OUT_Y_L, Self::OUT_Y_H)?;
        let z = self.read_axis(Self::OUT_Z_L, Self::OUT_Z_H)?;
        Ok((x, y, z))
    }

    fn read_axis(&mut self, l_reg: u8, h_reg: u8) -> Result<f32, E> {
        let low = self.read_reg(l_reg)?;
        let high = self.read_reg(h_reg)?;
        let raw = ((high as i16) << 8) | (low as i16);
        Ok(Self::normalize(raw as f32))
    }

    fn normalize(acc: f32) -> f32 {
        let mut val = acc * 0.004 / 64.0;
        if val > 2.0 {
            val -= 4.1;
        }
        val
    }

    fn write_reg(&mut self, reg: u8, value: u8) -> Result<(), E> {
        self.i2c.write(self.addr, &[reg, value])
    }

    fn read_reg(&mut self, reg: u8) -> Result<u8, E> {
        let mut data = [0];
        self.i2c.write_read(self.addr, &[reg], &mut data)?;
        Ok(data[0])
    }
}