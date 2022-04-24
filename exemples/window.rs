use x11_man::{
    ffi,
    x::{SelectInputConfig, SimpleWindow},
    xlib::{Event, TInput, XDisplay},
};

fn main() {
    let mut display = XDisplay::new(None);
    let mut window = SimpleWindow::new(&display, 500, 300);

    unsafe {
        let mut gc_values = ffi::xlib::XGCValues::default();
        gc_values.foreground = 0xffffffff;
        let gc = ffi::xlib::XCreateGC(display._d, window._w, 0, &gc_values);

        let mut select_input = SelectInputConfig::new();
        window.select_input(&select_input);

        loop {
            let event = window.next_event();
            match &event {
                Event::Expose(event) => {
                    ffi::xlib::XDrawString(
                        display._d,
                        window._w,
                        gc,
                        1,
                        10,
                        "Hello World\0".as_ptr() as *const i8,
                        11,
                    );
                }
                _ => {
                    println!("{:?}", event);
                }
            }
        }
    }
}
