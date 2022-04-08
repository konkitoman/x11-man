use std::time::Duration;

use x11_man::ffi::constants::{Button1, Button5Mask, ButtonPress, ControlMask, ShiftMask};
use x11_man::ffi::functions::XGrabButton;
use x11_man::ffi::{constants::GrabModeAsync, functions::XGrabKey};
use x11_man::ffi::{
    constants::{
        Button1Mask, Button1MotionMask, Button2MotionMask, ButtonMotionMask, ButtonPressMask,
        ButtonReleaseMask, FocusChangeMask, FocusOut, KeyRelease, KeyReleaseMask,
        PointerMotionMask, PointerRoot,
    },
    functions::{
        XCreateSimpleWindow, XGetInputFocus, XListInputDevices, XOpenDisplay, XQueryPointer,
        XRootWindow, XStoreName,
    },
    types::{Window, XEventClass},
};

use x11_man::ffi::{
    constants::{Expose, ExposureMask, KeyPress, KeyPressMask},
    functions::{
        DeviceKeyPress, DeviceKeyRelease, XDrawString, XFillRectangle, XLookupString, XMapWindow,
        XNextEvent, XOpenDevice, XQueryKeymap, XSelectExtensionEvent, XSelectInput,
    },
    structs::{XComposeStatus, XDeviceKeyEvent, XEvent},
    types::KeySym,
};

fn main() {
    safe_m_k();
    unsafe {
        let display = xlib::Display::new(None);
        let root = display.root_window(0);
        display.grab_key(
            41,
            0,
            root,
            true,
            GrabMode::GrabModeAsync,
            GrabMode::GrabModeAsync,
        );

        loop {
            let event = display.next_event();
            if event._type == ButtonPress {
                println!("Button: {}", event.xbutton.button);
            }
            println!("{}", event._type);
        }
    }
}

use x11_man::x::GrabMode;
use x11_man::xlib;
fn safe_m_k() {
    let display = xlib::Display::new(None);
    let root = display.root_window(0);

    loop {
        let pointer = display.quary_pointer(root);
        let keymap = display.quary_keymap();

        println!("Root X: {}, Y: {}", pointer.root_x, pointer.root_y);
        println!("Child X: {}, Y: {}", pointer.child_x, pointer.child_y);
        println!("Mask: {}", pointer.mask);

        println!("Keys: {:?}", keymap.get_keycodes());
        std::thread::sleep(Duration::from_millis(100));
    }
}

/// Get mouse and keyboard global state
unsafe fn get_m_k_global() {
    let display = XOpenDisplay(std::ptr::null());
    let root = XRootWindow(display, (*display).default_screen);

    loop {
        let mut root_window = 0;
        let mut child_window = 0;
        let mut root_x = 0;
        let mut root_y = 0;
        let mut child_x = 0;
        let mut child_y = 0;
        let mut mask = 0;

        XQueryPointer(
            display,
            root,
            &mut root_window,
            &mut child_window,
            &mut root_x,
            &mut root_y,
            &mut child_x,
            &mut child_y,
            &mut mask,
        );
        let mut keys = [0i8; 32];

        XQueryKeymap(display, &mut keys as *mut i8);

        println!("Root: X: {}, Y: {}", root_x, root_y);
        println!("Child: X: {}, Y: {}", child_x, child_y);
        println!("Keys: {:?}", get_keycode(&keys));

        std::thread::sleep(Duration::from_millis(1));
    }
}

pub unsafe fn get_keycode(data: &[i8; 32]) -> Vec<usize> {
    let mut result = Vec::new();
    for i in 0..32 {
        let mut j = 0;
        while (j < 8) {
            if data[i] & (1 << j) != 0 {
                result.push(i * 8 + j);
            }
            j += 1;
        }
    }
    result
}

unsafe fn get_fucused_window_input() {
    let display = XOpenDisplay(std::ptr::null());
    let root = XRootWindow(display, (*display).default_screen);
    let mut focused: Window = 0;
    let mut revert_to = 0;

    XGetInputFocus(display, &mut focused, &mut revert_to);
    let masks = KeyPressMask | KeyReleaseMask | FocusChangeMask;
    XSelectInput(display, focused, masks);

    let mut event = XEvent { _type: 0 };
    loop {
        XNextEvent(display, &mut event);
        if event._type == FocusOut {
            if focused != root {
                XSelectInput(display, focused, 0);
            }
            XGetInputFocus(display, &mut focused, &mut revert_to);
            if focused == PointerRoot as u64 {
                focused = root;
            }
            XSelectInput(display, focused, masks);
        } else if event._type == KeyPress {
            println!("Key Down: {}", event.xkey.keycode);
        } else if event._type == KeyRelease {
            println!("Key Up: {}", event.xkey.keycode);
        }
    }
}

unsafe fn device_get_input() {
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
    let message = "Hello World".as_ptr() as *const i8;
    let display = XOpenDisplay(0 as *const i8);

    let root = x11_man::ffi::functions::XRootWindow(display, (*display).default_screen);
    let screen = &std::slice::from_raw_parts((*display).screens, (*display).nscreens as usize)
        [(*display).default_screen as usize];
    let gc = screen.default_gc;
    let black = screen.black_pixel;
    let white = screen.white_pixel;
    let window = x11_man::ffi::functions::XCreateSimpleWindow(
        display, root, 10, 10, 100, 100, 1, black, white,
    );
    XStoreName(
        display,
        window,
        "Rust X11 Lib Window\0".as_ptr() as *const i8,
    );

    XSelectInput(
        display,
        window,
        ExposureMask | KeyPressMask | PointerMotionMask,
    );
    XMapWindow(display, window);
    let mut event = XEvent { _type: 0 };
    loop {
        XNextEvent(display, &mut event);
        if event._type == Expose {
            XFillRectangle(display, window, gc, 20, 20, 10, 10);
            XDrawString(display, window, gc, 20, 20, message, 11);
        } else if event._type == KeyPress {
            let mut char: i8 = 0;
            XLookupString(
                &event.xkey,
                &mut char,
                255,
                0 as *mut KeySym,
                0 as *mut XComposeStatus,
            );
            let char = char::from_u32(char as u32).unwrap();
            println!("Key: {}", char);
        } else {
            println!("Type: {}", event._type);
        }
    }
}
