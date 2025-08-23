use std::{thread, time::Duration};
use linux_embedded_hal::I2cdev;
use embedded_hal::i2c::{I2c, SevenBitAddress};
use uinput::device::Builder;
use uinput::event::absolute::{Absolute, Position};
use uinput::event::Event::Absolute as AbsEvent;



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut i2c = I2cdev::new("/dev/i2c-1")?;

    let mut device = Builder::default()?
        .name("ft6336u-touch")?
        .event(AbsEvent(Absolute::Position(Position::X)))?
        .event(AbsEvent(Absolute::Position(Position::Y)))?
        .create()?;

    loop {
        let mut buf = [0u8; 7];
        i2c.write_read(SevenBitAddress::from_be(0x38), &[0x02], &mut buf)?;

        let touches = buf[0] & 0x0F;
        if touches > 0 {
            let x = (((buf[1] & 0x0F) as u16) << 8) | buf[2] as u16;
            let y = (((buf[3] & 0x0F) as u16) << 8) | buf[4] as u16;

            println!("Touch at: x = {x}, y = {y}");

            device.send(AbsEvent(Absolute::Position(Position::X)), x as i32)?;
            device.send(AbsEvent(Absolute::Position(Position::Y)), y as i32)?;
            device.synchronize()?;
        }

        thread::sleep(Duration::from_millis(16));
    }
}

