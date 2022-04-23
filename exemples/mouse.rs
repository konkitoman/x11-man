use x11_man::{
    x::PointerGrabConfig,
    xlib::{Event, XDisplay},
};

pub fn main() {
    GrabPointer.run();
}

pub trait TFunc {
    fn run(&self);
}

/// # Atention is grabing pointer you can't use pointer any more before is ungrabed
struct GrabPointer;

impl TFunc for GrabPointer {
    fn run(&self) {
        let display = XDisplay::new(None);
        let root = display.root_window(display.default_screen());

        let mut pointer_config = PointerGrabConfig::new();
        pointer_config.all(); // echivalent pointer_config.button_press(true).button_release(true).pointer_motion(true);

        display.grab_pointer(root, &pointer_config, 0, 0);

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

        //display.ungrab_pointer(); for ungrabing
    }
}
