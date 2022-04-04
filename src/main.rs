mod lib;

use std::ffi::CString;

use lib::ffi::{
    functions::{XListInputDevices, XOpenDisplay, XRootWindow, XStoreName},
    types::XEventClass,
};

use crate::lib::ffi::{
    constants::{Expose, ExposureMask, KeyPress, KeyPressMask},
    functions::{
        DeviceKeyPress, DeviceKeyRelease, XDrawString, XFillRectangle, XLookupString, XMapWindow,
        XNextEvent, XOpenDevice, XSelectExtensionEvent, XSelectInput,
    },
    structs::{XComposeStatus, XDeviceKeyEvent, XEvent},
    types::KeySym,
};

fn main() {
    unsafe {
        let display = XOpenDisplay(std::ptr::null());
        let root_window = XRootWindow(display, (*display).default_screen);

        let mut number_dev = 0;
        let devices = XListInputDevices(display, &mut number_dev);

        let array = std::slice::from_raw_parts(devices, number_dev as usize);
        for (index, device) in array.iter().enumerate() {
            println!("{}: {}", index, get_string(device.name));
        }
        let stdin = std::io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input: usize = input.trim().parse().unwrap();
        let device_info = array[input];

        println!("Device: {}", get_string(device_info.name));

        let device = XOpenDevice(display, device_info.id);

        let mut event_list: &mut [XEventClass] = &mut [0; 7];
        let mut number = 0;
        let mut o1 = -1;
        let mut o2 = -1;

        DeviceKeyPress(device, &mut o1, &mut event_list[number]);
        number += 1;
        DeviceKeyRelease(device, &mut o2, &mut event_list[number]);
        number += 1;

        if XSelectExtensionEvent(
            display,
            root_window,
            &mut event_list[0] as *mut XEventClass,
            number as i32,
        ) > 0
        {
            println!("error selecting extension events");
        }

        let mut event = XEvent { _type: 0 };

        loop {
            XNextEvent(display, &mut event);
            if event._type == o1 {
                let key = XDeviceKeyEvent::from(&mut event);
                let mut c_key = 0;
                XLookupString(
                    &(*key).as_key_event(),
                    &mut c_key,
                    255,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                );

                println!("Pressed: {}", char::from_u32(c_key as u32).unwrap());
            } else if event._type == o2 {
                let key = XDeviceKeyEvent::from(&mut event);
                let mut c_key = 0;
                XLookupString(
                    &(*key).as_key_event(),
                    &mut c_key,
                    255,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                );

                println!("Released: {}", char::from_u32(c_key as u32).unwrap());
            }
        }
    }
}

pub unsafe fn get_string(beagining: *mut i8) -> String {
    let mut data = String::new();
    let mut index: usize = 0;
    loop {
        let c = *beagining.offset(index as isize);
        if c == 0 {
            break;
        }
        data.push(char::from_u32(c as u32).unwrap());
        index += 1;
    }

    data
}

pub unsafe fn create_window() {
    let message = "Hello World".as_ptr() as *const libc::c_char;
    let display = XOpenDisplay(0 as *const libc::c_char);

    let root = lib::ffi::functions::XRootWindow(display, (*display).default_screen);
    let screen = &std::slice::from_raw_parts((*display).screens, (*display).nscreens as usize)
        [(*display).default_screen as usize];
    let gc = screen.default_gc;
    let black = screen.black_pixel;
    let white = screen.white_pixel;
    let window =
        lib::ffi::functions::XCreateSimpleWindow(display, root, 10, 10, 100, 100, 1, black, white);
    XStoreName(
        display,
        window,
        "Rust X11 Lib Window\0".as_ptr() as *const libc::c_char,
    );

    XSelectInput(display, window, ExposureMask | KeyPressMask);
    XMapWindow(display, window);
    let mut event = XEvent { _type: 0 };
    loop {
        XNextEvent(display, &mut event);
        if event._type == Expose {
            XFillRectangle(display, window, gc, 20, 20, 10, 10);
            XDrawString(display, window, gc, 20, 20, message, 11);
        } else if event._type == KeyPress {
            let mut char: libc::c_char = 0;
            XLookupString(
                &event.xkey,
                &mut char,
                255,
                0 as *mut KeySym,
                0 as *mut XComposeStatus,
            );
            let char = char::from_u32(char as u32).unwrap();
            println!("Key: {}", char);
        }
    }
}
