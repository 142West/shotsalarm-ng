use core::slice::SlicePattern;
use std::thread::sleep;

use rppal::i2c::I2c;
use crate::error::*;

type Error = Box<dyn std::error::Error>;
type I2CResult = Result<(), rppal::i2c::Error>;

const DEFAULT_I2C_ADDRESS: u16 = 0x72;
const MAX_ROWS: u8 = 4;
const MAX_COLS: u8 = 20;

const SPECIAL_COMMAND: u8 = 254;
const SETTING_COMMAND: u8 = 0x7C;

// Define command characters
// taken from https://github.com/fourstix/Sparkfun_CircuitPython_SerLCD/blob/master/sparkfun_serlcd.py#L87

const CLEAR_COMMAND: u8 = 0x2D;
const CONTRAST_COMMAND: u8 = 0x18;
const ADDRESS_COMMAND: u8 = 0x19;
const BACKLIGHT_RGB_COMMAND: u8 = 0x2B;
const ENABLE_SYSMSG_DISPLAY_COMMAND: u8 = 0x2E;
const DISABLE_SYSMSG_DISPLAY_COMMAND: u8 = 0x2F;
const ENABLE_SPLASH_COMMAND: u8 = 0x30;
const DISABLE_SPLASH_COMMAND: u8 = 0x31;
const SAVE_DISPLAY_AS_SPLASH: u8 = 0x0A;
const SHOW_VERSION_COMMAND: u8 = 0x2C;
const RESET_COMMAND: u8 = 0x08;
const HOME_CURSOR_COMMAND: u8 = 0x02;
const SET_ENTRY_MODE_COMMAND: u8 = 0x04;
const CONTROL_DISPLAY_COMMAND: u8 = 0x08;
const CURSOR_SHIFT_COMMAND: u8 = 0x10;
const ENTRY_RIGHT_COMMAND: u8 = 0x00;
const ENTRY_LEFT_COMMAND: u8 = 0x02;

const DISPLAY_ON_FLAG: u8 = 0x04;
const DISPLAY_OFF_FLAG: u8 = 0x00;
const CURSOR_ON_FLAG: u8 = 0x02;
const CURSOR_OFF_FLAG: u8 = 0x00;
const LCD_BLINK_ON_FLAG: u8 = 0x01;
const LCD_BLINK_OFF_FLAG: u8 = 0x00;

fn map_range(value: u8, in_min: u8, in_max: u8, out_min: u8, out_max: u8) -> u8 {
    return (value - in_min) * (out_max - out_min) / (in_max - in_min) + out_min;
}

struct LcdConn {
    lcd_address: u16,
    connection: I2c,
}

impl LcdConn {
    pub fn try_init() -> Result<Self, Error> {
        let conn = I2c::new()?;
        Ok(Self { lcd_address: DEFAULT_I2C_ADDRESS, connection: conn })
    }

    pub fn begin(&mut self) -> I2CResult {

        Ok(())
    }

    fn write_bytes(&mut self, buffer: &[u8]) -> Result<usize, rppal::i2c::Error> {
        self.connection.write(buffer)
    }

    fn command(&mut self, command: u8) -> Result<(), rppal::i2c::Error> {
        
        let mut data: Vec<u8> = Vec::new();
        data.push(SETTING_COMMAND);
        data.push(command & 0xFF);
        self.write_bytes(data.as_slice())?;

        sleep(std::time::Duration::from_millis(10)); // wait a tiny bit
        Ok(())
    }

    fn special_command(&mut self, command: u8, count: Option<u8>) -> I2CResult {
        let mut data: Vec<u8> = Vec::new();
        data.push(SPECIAL_COMMAND);
        for _ in 1..=count.unwrap_or(1) {
            data.push(command & 0xFF);
        }

        self.write_bytes(data.as_slice())?;
        Ok(())
    }

    pub fn clear(&mut self) -> I2CResult {
        self.command(CLEAR_COMMAND)?;
        Ok(())
    }

    pub fn home(&mut self) -> I2CResult {
        self.special_command(HOME_CURSOR_COMMAND, None)
    }

    pub fn write_string(&mut self, message: String) -> I2CResult {
        self.write_bytes(message.as_bytes())?;
        Ok(())
    }

    pub fn set_backlight_rgb(&mut self, red: u8, green: u8, blue: u8) -> I2CResult {
        let r = 128 + map_range(red, 0, 255, 0, 29);
        let g = 158 + map_range(green, 0, 255, 0, 29);
        let b = 188 + map_range(blue, 0, 255, 0, 29);

        let mut data: Vec<u8> = Vec::new();

        Ok(())
    }

}
