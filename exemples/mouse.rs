use std::{
    collections::HashMap,
    io::{stdin, Write},
};

use x11_man::{
    ffi::x::{
        Button1, Button1Mask, ButtonPressMask, ButtonReleaseMask, ControlMask, LockMask, Mod1Mask,
        Mod2Mask, Mod3Mask, Mod4Mask, Mod5Mask, PointerMotionMask, ShiftMask,
    },
    x::{ButtonGrabConfig, PointerGrabConfig},
    xlib::{Event, TInput, XDisplay},
};

pub fn main() {
    let mut methods: HashMap<&str, &dyn TFunc> = HashMap::new();
    methods.insert("pointer_grabing", &GrabPointer);
    methods.insert("button_grabing", &GrabButton);
    methods.insert("quary_pointer", &QueryPointer);

    for (name, _) in methods.iter() {
        println!("{}", name);
    }

    print!("Enter method name: ");
    let mut stdout = std::io::stdout();
    stdout.flush().unwrap();
    let stdint = stdin();
    let mut method_name = String::new();
    stdint.read_line(&mut method_name).unwrap();
    let method_name = method_name.trim();
    methods[method_name].run();
}

pub trait TFunc {
    fn run(&self);
}

/// # Atention is grabing pointer you can't use pointer any more before is ungrabed
struct GrabPointer;

impl TFunc for GrabPointer {
    fn run(&self) {
        let display = XDisplay::new(None);

        let mut pointer_config = PointerGrabConfig::new();
        pointer_config.all(); // echivalent pointer_config.button_press(true).button_release(true).pointer_motion(true);

        display.grab_pointer(&pointer_config);

        loop {
            let event = display.next_event();
            match event {
                Event::ButtonPress(button) => {
                    println!(
                        "Button pressed: X: {}, Y: {}, button {}",
                        button.x, button.y, button.button
                    );
                }
                Event::ButtonRelease(button) => {
                    println!(
                        "Button released: X: {}, Y: {}, button {}",
                        button.x, button.y, button.button
                    );
                }
                Event::MotionNotify(motion) => {
                    println!("Motion: X: {}, Y: {}", motion.x, motion.y);
                }
                _ => {
                    println!("{:?}", event)
                }
            }
        }

        //display.ungrab_pointer(); for ungrabing
    }
}

pub struct GrabButton;

impl TFunc for GrabButton {
    fn run(&self) {
        let display = XDisplay::new(None);
        let mut config = ButtonGrabConfig::new();
        config.set_modifiers("S-A");
        config.all();

        display.grab_button(&config);

        loop {
            let event = display.next_event();
            match event {
                Event::ButtonPress(button) => {
                    println!(
                        "Button pressed: X: {}, Y: {}, button {}",
                        button.x, button.y, button.button
                    );
                }
                Event::ButtonRelease(button) => {
                    println!(
                        "Button released: X: {}, Y: {}, button {}",
                        button.x, button.y, button.button
                    );
                }
                Event::MotionNotify(motion) => {
                    println!("Motion: X: {}, Y: {}", motion.x, motion.y);
                }
                _ => {
                    println!("{:?}", event);
                }
            }
        }
    }
}

pub struct QueryPointer;

impl TFunc for QueryPointer {
    fn run(&self) {
        let display = XDisplay::new(None);
        let root = display.root_window(display.default_screen());

        loop {
            let pointer = display.quary_pointer(root);
            println!(
                "Pointer: X: {}, Y: {}, Mask: {}",
                pointer.root_x, pointer.root_y, pointer.mask
            );
        }
    }
}
