use ft6x36::{Dimension, Ft6x36, TouchPoint, TouchType};
use linux_embedded_hal::I2cdev;
use uinput::device::Builder;
use uinput::event::absolute::{Absolute, Multi};
use uinput::event::controller::Digi;
use uinput::event::Controller;
use uinput::event::Event::Absolute as AbsEvent;



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let i2c = I2cdev::new("/dev/i2c-4")?;

    let mut device = Builder::open("/dev/uinput")?
        .name("ft6336u-touch")?
        .event(AbsEvent(Absolute::Multi(Multi::Slot)))?
        .max(2)
        .event(AbsEvent(Absolute::Multi(Multi::TrackingId)))?
        .event(AbsEvent(Absolute::Multi(Multi::PositionX)))?
        .max(480)
        .event(AbsEvent(Absolute::Multi(Multi::PositionY)))?
        .max(320)
        .event(Controller::Digi(Digi::Touch))?
        .create()?;

    let mut ft6336 = Ft6x36::new(i2c, Dimension(320, 480));
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
        let mut event = ft6336.get_touch_event()?;

        if let Some(p) = event.p1 {
            event.p1 = Some(TouchPoint {
                touch_type: p.touch_type,
                x: p.y,
                y: 320 - p.x,
            });
        }
        if let Some(p) = event.p2 {
            event.p2 = Some(TouchPoint {
                touch_type: p.touch_type,
                x: p.y,
                y: 320 - p.x,
            });
        }

        match (event.p1, event.p2, p1_state, p2_state) {
            (None, None, true, true) => {
                //println!("Release p1 p2");
                device.position(&Multi::Slot, 0)?;
                device.position(&Multi::TrackingId, -1)?;
                device.position(&Multi::Slot, 1)?;
                device.position(&Multi::TrackingId, -1)?;
                device.synchronize()?;

                device.release(&Digi::Touch)?;

                device.synchronize()?;
                p1_state = false;
                p2_state = false;
            },
            (None, None, true, false) => {
                //println!("Release p1");
                device.position(&Multi::Slot, 0)?;
                device.position(&Multi::TrackingId, -1)?;
                device.synchronize()?;
                device.release(&Digi::Touch)?;
                device.synchronize()?;
                p1_state = false;
            },
            (None, None, false, true) => {
                //println!("Release p2");
                device.position(&Multi::Slot, 1)?;
                device.position(&Multi::TrackingId, -1)?;
                device.synchronize()?;
                device.synchronize()?;
                p2_state = false;
            },
            (None, None, false, false) => (),
            (None, Some(p2), true, true) => {
                if p2.touch_type == TouchType::Contact {
                    //println!("Release p1 Continue p2");

                    device.position(&Multi::Slot, 0)?;
                    device.position(&Multi::TrackingId, -1)?;
                    device.position(&Multi::Slot, 1)?;
                    device.position(&Multi::PositionX, p2.x as i32)?;
                    device.position(&Multi::PositionY, p2.y as i32)?;
                    device.synchronize()?;
                } else {
                    //println!("Release p1");
                    device.position(&Multi::Slot, 0)?;
                    device.position(&Multi::TrackingId, -1)?;
                    device.synchronize()?;
                    device.release(&Digi::Touch)?;
                    device.synchronize()?;
                }
                p1_state = false;
            },
            (None, Some(p2), true, false) => {
                //println!("Release p1 Touch p2");

                device.position(&Multi::Slot, 0)?;
                device.position(&Multi::TrackingId, -1)?;
                device.position(&Multi::Slot, 1)?;
                device.position(&Multi::TrackingId, 101)?;
                device.position(&Multi::PositionX, p2.x as i32)?;
                device.position(&Multi::PositionY, p2.y as i32)?;
                device.synchronize()?;

                p1_state = false;
                p2_state = true;
            },
            (None, Some(p2), false, true) => {
                if p2.touch_type == TouchType::Contact {
                    //println!("Continue p2");
                    device.position(&Multi::Slot, 1)?;
                    device.position(&Multi::PositionX, p2.x as i32)?;
                    device.position(&Multi::PositionY, p2.y as i32)?;
                    device.synchronize()?;
                }
            },
            (None, Some(p2), false, false) => {
                //println!("Touch p2");
                device.position(&Multi::Slot, 1)?;
                device.position(&Multi::TrackingId, 101)?;
                device.position(&Multi::PositionX, p2.x as i32)?;
                device.position(&Multi::PositionY, p2.y as i32)?;
                device.synchronize()?;
                p2_state = true;
            },
            (Some(p1), None, true, true) => {
                if p1.touch_type == TouchType::Contact {
                    //println!("Continue p1 Release p2");
                    device.position(&Multi::Slot, 0)?;
                    device.position(&Multi::PositionX, p1.x as i32)?;
                    device.position(&Multi::PositionY, p1.y as i32)?;
                    device.position(&Multi::Slot, 1)?;
                    device.position(&Multi::TrackingId, -1)?;
                    device.synchronize()?;
                } else {
                    //println!("Release p2");
                    device.position(&Multi::Slot, 1)?;
                    device.position(&Multi::TrackingId, -1)?;
                    device.synchronize()?;
                }
                p2_state = false;
            },
            (Some(p1), None, true, false) => {
                if p1.touch_type == TouchType::Contact {
                    //println!("Continue p1");
                    device.position(&Multi::Slot, 0)?;
                    device.position(&Multi::PositionX, p1.x as i32)?;
                    device.position(&Multi::PositionY, p1.y as i32)?;
                }
            },
            (Some(p1), None, false, true) => {
                //println!("Touch p1 Release p2");
                device.press(&Digi::Touch)?;
                device.synchronize()?;

                device.position(&Multi::Slot, 0)?;
                device.position(&Multi::TrackingId, 100)?;
                device.position(&Multi::PositionX, p1.x as i32)?;
                device.position(&Multi::PositionY, p1.y as i32)?;

                device.position(&Multi::Slot, 1)?;
                device.position(&Multi::TrackingId, -1)?;

                device.synchronize()?;

                p1_state = true;
                p2_state = false;
            },
            (Some(p1), None, false, false) => {
                //println!("Touch p1");
                device.press(&Digi::Touch)?;
                device.synchronize()?;

                device.position(&Multi::Slot, 0)?;
                device.position(&Multi::TrackingId, 100)?;
                device.position(&Multi::PositionX, p1.x as i32)?;
                device.position(&Multi::PositionY, p1.y as i32)?;
                device.synchronize()?;
                p1_state = true;
            },
            (Some(p1), Some(p2), true, true) => {
                if p1.touch_type == TouchType::Contact && p2.touch_type == TouchType::Contact {
                    device.position(&Multi::Slot, 0)?;
                    device.position(&Multi::PositionX, p1.x as i32)?;
                    device.position(&Multi::PositionY, p1.y as i32)?;
                    device.position(&Multi::Slot, 1)?;
                    device.position(&Multi::PositionX, p2.x as i32)?;
                    device.position(&Multi::PositionY, p2.y as i32)?;

                    device.synchronize()?;
                    //println!("Continue p1 Continue p2");
                }
            },
            (Some(p1), Some(p2), true, false) => {
                if p1.touch_type == TouchType::Contact {
                    //println!("Continue p1 Touch p2");
                    device.position(&Multi::Slot, 0)?;
                    device.position(&Multi::PositionX, p1.x as i32)?;
                    device.position(&Multi::PositionY, p1.y as i32)?;
                    device.position(&Multi::Slot, 1)?;
                    device.position(&Multi::TrackingId, 101)?;
                    device.position(&Multi::PositionX, p2.x as i32)?;
                    device.position(&Multi::PositionY, p2.y as i32)?;

                    device.synchronize()?;
                } else {
                    //println!("Touch p2");

                    device.position(&Multi::Slot, 1)?;
                    device.position(&Multi::TrackingId, 101)?;
                    device.position(&Multi::PositionX, p2.x as i32)?;
                    device.position(&Multi::PositionY, p2.y as i32)?;
                    device.synchronize()?;
                }
                p2_state = true;
            },
            (Some(p1), Some(p2), false, true) => {
                if p2.touch_type == TouchType::Contact {
                    //println!("Touch p1 Continue p2");

                    device.position(&Multi::Slot, 0)?;
                    device.position(&Multi::TrackingId, 100)?;
                    device.position(&Multi::PositionX, p1.x as i32)?;
                    device.position(&Multi::PositionY, p1.y as i32)?;
                    device.position(&Multi::Slot, 1)?;
                    device.position(&Multi::PositionX, p2.x as i32)?;
                    device.position(&Multi::PositionY, p2.y as i32)?;
                    device.synchronize()?;
                } else {
                    //println!("Touch p1");
                    device.press(&Digi::Touch)?;
                    device.synchronize()?;
                    device.position(&Multi::Slot, 0)?;
                    device.position(&Multi::TrackingId, 100)?;
                    device.position(&Multi::PositionX, p1.x as i32)?;
                    device.position(&Multi::PositionY, p1.y as i32)?;
                    device.synchronize()?;
                }
                p1_state = true;
            },
            (Some(p1), Some(p2), false, false) => {
                //println!("Touch p1 Touch p2");

                device.press(&Digi::Touch)?;
                device.synchronize()?;
                device.position(&Multi::Slot, 0)?;
                device.position(&Multi::TrackingId, 100)?;
                device.position(&Multi::PositionX, p1.x as i32)?;
                device.position(&Multi::PositionY, p1.y as i32)?;
                device.position(&Multi::Slot, 1)?;
                device.position(&Multi::TrackingId, 101)?;
                device.position(&Multi::PositionX, p2.x as i32)?;
                device.position(&Multi::PositionY, p2.y as i32)?;
                device.synchronize()?;

                p1_state = true;
                p2_state = true;
            },
        };
    }
}

