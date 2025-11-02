use std::{thread, time::Duration};
use ft6x36::{Dimension, Ft6x36, GestureId, TouchType};
use linux_embedded_hal::I2cdev;
use embedded_hal::i2c::{I2c, SevenBitAddress};
use uinput::device::Builder;
use uinput::event::absolute::{Absolute, Position};
use uinput::event::controller::{Digi, Mouse};
use uinput::event::Controller;
use uinput::event::Event::Absolute as AbsEvent;
use uinput::Event;



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut i2c = I2cdev::new("/dev/i2c-4")?;

    let mut device = Builder::open("/dev/uinput")?
        .name("ft6336u-touch")?
        .event(Event::Controller(Controller::Digi(Digi::Touch)))?
        .event(AbsEvent(Absolute::Position(Position::X)))?
        .max(480)
        .event(AbsEvent(Absolute::Position(Position::Y)))?
        .max(320)
        .event(Event::Controller(Controller::Digi(Digi::DoubleTap)))?
        .create()?;

    let mut ft6336 = Ft6x36::new(i2c, Dimension(480, 320));
    ft6336.init()?;
    match ft6336.get_info() {
        Some(info) => println!("Touch screen info: {info:?}"),
        None => println!("No driver info!"),
    };

    ft6336.set_touch_threshold(20)?;
    ft6336.set_touch_filter_coefficient(20)?;
    ft6336.set_gesture_distance_zoom(25)?;
    ft6336.set_gesture_minimum_angle(30)?;
    ft6336.set_control_mode(0)?;

    let diag = ft6336.get_diagnostics()?;
    println!("{diag:?}");
    let params = ft6336.get_gesture_params()?;
    println!("{params:?}");

    let (mut p1_state, mut p2_state) = (false, false);
    loop {
        let event = ft6336.get_touch_event()?;

        match (event.p1, event.p2, p1_state, p2_state) {
            (None, None, true, true) => {
                println!("Release p1 p2");
                p1_state = false;
                p2_state = false;
            },
            (None, None, true, false) => {
                println!("Release p1");
                p1_state = false;
            },
            (None, None, false, true) => {
                println!("Release p2");
                p2_state = false;
            },
            (None, None, false, false) => (),
            (None, Some(_), true, true) => {
                println!("Release p1 Continue p2");
                p1_state = false;
            },
            (None, Some(_), true, false) => {
                println!("Release p1 Touch p2");
                p1_state = false;
                p2_state = true;
            },
            (None, Some(_), false, true) => {
                println!("Continue p2");
            },
            (None, Some(_), false, false) => {
                println!("Touch p2");
                p2_state = true;
            },
            (Some(_), None, true, true) => {
                println!("Continue p1 Release p2");
                p2_state = false;
            },
            (Some(_), None, true, false) => {
                println!("Continue p1");
            },
            (Some(_), None, false, true) => {
                println!("Touch p1 Release p2");
                p1_state = true;
                p2_state = false;
            },
            (Some(_), None, false, false) => {
                println!("Touch p1");
                p1_state = true;
            },
            (Some(_), Some(_), true, true) => {
                println!("Continue p1 Continue p2");
            },
            (Some(_), Some(_), true, false) => {
                println!("Continue p1 Touch p2");
                p2_state = true;
            },
            (Some(_), Some(_), false, true) => {
                println!("Touch p1 Continue p2");
                p1_state = true;
            },
            (Some(_), Some(_), false, false) => {
                println!("Touch p1 Touch p2");
                p1_state = true;
                p2_state = true;
            },
        };

        // match event.gesture_id {
        //     GestureId::NoGesture => (),
        //     GestureId::MoveUp | GestureId::MoveRight | GestureId::MoveDown | GestureId::MoveLeft => {
        //         println!("MoveUp | MoveRight | MoveDown | MoveLeft: {event:?}");
        //     },
        //     GestureId::ZoomIn => println!("ZoomIn: {event:?}"),
        //     GestureId::ZoomOut => println!("ZoomOut: {event:?}"),
        // };
        // match event.p1 {
        //     Some(p) => match p.touch_type {
        //         TouchType::Press => {
        //             println!("Press p1 {event:?}");
        //             device.send(Event::Controller(Controller::Digi(Digi::Touch)), 1)?
        //         },
        //         TouchType::Release => {
        //             println!("Release p1 {event:?}");
        //             device.send(Event::Controller(Controller::Digi(Digi::Touch)), 0)?;
        //             device.synchronize()?;
        //         },
        //         TouchType::Contact => {
        //             println!("Contact p1 {event:?}")
        //             // println!("Contact");
        //             // device.send(Event::Controller(Controller::Digi(Digi::Touch)), 1)?;
        //             // device.send(AbsEvent(Absolute::Position(Position::X)), p.x as i32)?;
        //             // device.send(AbsEvent(Absolute::Position(Position::Y)), p.y as i32)?;
        //             // device.send(Event::Controller(Controller::Digi(Digi::Touch)), 0)?;
        //             // device.synchronize()?;
        //         },
        //         TouchType::Invalid => println!("Invalid point: {p:?}"),
        //     },
        //     None => (),
        // };
        // match event.p2 {
        //     Some(p) => match p.touch_type {
        //         TouchType::Press => {
        //             println!("Press p2");
        //             device.send(Event::Controller(Controller::Digi(Digi::Touch)), 1)?
        //         },
        //         TouchType::Release => {
        //             println!("Release p2");
        //             device.send(Event::Controller(Controller::Digi(Digi::Touch)), 0)?;
        //             device.synchronize()?;
        //         },
        //         TouchType::Contact => {
        //             println!("Contact p2");
        //             // device.send(Event::Controller(Controller::Digi(Digi::Touch)), 1)?;
        //             // device.send(AbsEvent(Absolute::Position(Position::X)), p.x as i32)?;
        //             // device.send(AbsEvent(Absolute::Position(Position::Y)), p.y as i32)?;
        //             // device.send(Event::Controller(Controller::Digi(Digi::Touch)), 0)?;
        //             // device.synchronize()?;
        //         },
        //         TouchType::Invalid => println!("Invalid point: {p:?}"),
        //     },
        //     None => (),
        // };
    }
    // loop {
    //     let mut buf = [0u8; 7];
    //     i2c.write_read(SevenBitAddress::from_be(0x38), &[0x02], &mut buf)?;
    //
    //     let touches = buf[0] & 0x0F;
    //     if touches > 0 {
    //         let x = (((buf[1] & 0x0F) as u16) << 8) | buf[2] as u16;
    //         let y = (((buf[3] & 0x0F) as u16) << 8) | buf[4] as u16;
    //
    //         // Landscape mode
    //         let (x, y) = (y, 320 - x);
    //
    //         println!("Touch at: x = {x}, y = {y}");
    //
    //         // device.send(Event::Controller(Controller::Mouse(Mouse::Left)), 1)?;
    //         device.send(Event::Controller(Controller::Digi(Digi::Touch)), 1)?;
    //         device.send(AbsEvent(Absolute::Position(Position::X)), x as i32)?;
    //         device.send(AbsEvent(Absolute::Position(Position::Y)), y as i32)?;
    //         device.synchronize()?;
    //
    //         // thread::sleep(Duration::from_millis(200));
    //
    //         // device.send(Event::Controller(Controller::Digi(Digi::Touch)), 1)?;
    //         // device.synchronize()?;
    //         // thread::sleep(Duration::from_millis(200));
    //     }
    //
    //     thread::sleep(Duration::from_millis(16));
    // }
}

