use std::{
    collections::HashMap,
    io::{stdin, Write},
    time::Duration,
};

use x11_man::{
    x::{KeyGrabConfig, KeyboardGrabConfig},
    xlib::{Event, TInput, XDisplay},
};

fn main() {
    let mut methods: HashMap<&str, &dyn Func> = HashMap::new();
    methods.insert("keyboard_grabing", &KeyboardGrabing);
    methods.insert("grab_key", &GrabKey);
    methods.insert("quary_keymap", &QuaryKeymap);

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

trait Func {
    fn run(&self);
}

struct QuaryKeymap;
impl Func for QuaryKeymap {
    fn run(&self) {
        let display = XDisplay::new(None);
        loop {
            println!("Keycodes: {:?}", display.quary_keymap().get_keycodes());
            std::thread::sleep(Duration::from_millis(10));
        }
    }
}

struct GrabKey;
impl Func for GrabKey {
    fn run(&self) {
        println!("Is grabing key f!");
        let display = XDisplay::new(None);
        let mut config = KeyGrabConfig::new();
        config.keycode = 41; // means f
        display.grab_key(&config);
        println!("Key f grabed!");

        loop {
            let event = display.next_event();
            match event {
                Event::KeyPress(key) => {
                    println!("F pressed: {}", key.keycode);
                }
                Event::KeyRelease(key) => {
                    println!("F released: {}", key.keycode);
                }
                _ => {}
            }
        }
    }
}

struct KeyboardGrabing;
impl Func for KeyboardGrabing {
    fn run(&self) {
        println!("Is grabing keyboard!");
        let display = XDisplay::new(None);
        let config = KeyboardGrabConfig::new();
        display.grab_keyboard(&config);

        println!("Keyboard grabed!");
        println!("Press esc to exit!");

        'event_loop: loop {
            let event = display.next_event();
            match event {
                Event::KeyPress(key) => {
                    println!("Key Press: {}", key.keycode);
                }
                Event::KeyRelease(key) => {
                    println!("Key Release: {}", key.keycode);
                    if key.keycode == 9 {
                        // 9 means escape
                        break 'event_loop;
                    }
                }
                _ => {}
            }
        }
    }
}
