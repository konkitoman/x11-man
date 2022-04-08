use crate::x::GrabMode;

use super::ffi::*;

#[derive(Debug, Copy, Clone)]
pub struct Screen {
    pub _s: *mut structs::Screen,
}

unsafe impl Sync for Screen {}
unsafe impl Send for Screen {}

impl Screen {
    pub unsafe fn new(screen: *mut structs::Screen) -> Self {
        Self { _s: screen }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Display {
    pub _d: *mut structs::Display,
}

unsafe impl Sync for Display {}
unsafe impl Send for Display {}

impl Display {
    /// open display
    pub fn new(number: Option<i8>) -> Self {
        let display = match number {
            Some(number) => unsafe { functions::XOpenDisplay(&number) },
            None => unsafe { functions::XOpenDisplay(std::ptr::null()) },
        };

        Self { _d: display }
    }

    /// get default screen number
    pub fn default_screen(&self) -> i32 {
        unsafe { (*self._d).default_screen }
    }

    /// get screens
    pub fn screens(&self) -> Vec<Screen> {
        let mut data = Vec::new();
        unsafe {
            for i in 0..self.screens_len() {
                let screen = (*self._d).screens.offset(i as isize);
                data.push(Screen::new(screen));
            }
        }
        data
    }

    /// get screens length
    pub fn screens_len(&self) -> i32 {
        unsafe { (*self._d).nscreens }
    }

    pub fn get_default_screen(&self) -> Screen {
        self.screens()[self.default_screen() as usize].clone()
    }

    pub fn root_window(&self, screen_number: i32) -> types::Window {
        unsafe { functions::XRootWindow(self._d, screen_number) }
    }

    pub fn select_input(&self, window: types::Window, mask: i64) -> i32 {
        unsafe { functions::XSelectInput(self._d, window, mask) }
    }

    pub fn store_name(&self, window: types::Window, name: &str) {
        unsafe { functions::XStoreName(self._d, window, name.as_ptr() as *const i8) }
    }

    pub fn get_input_focus(&self) -> (types::Window, i32) {
        let mut window = 0;
        let mut revert = 0;

        unsafe { functions::XGetInputFocus(self._d, &mut window, &mut revert) };

        (window, revert)
    }

    pub fn next_event(&self) -> structs::XEvent {
        let mut event = structs::XEvent { _type: 0 };

        unsafe { functions::XNextEvent(self._d, &mut event) };

        event
    }

    pub fn quary_pointer(&self, window: types::Window) -> QuaryPointer {
        let mut root = 0;
        let mut child = 0;

        let mut root_x = 0;
        let mut root_y = 0;
        let mut child_x = 0;
        let mut child_y = 0;
        let mut mask = 0;

        unsafe {
            functions::XQueryPointer(
                self._d,
                window,
                &mut root,
                &mut child,
                &mut root_x,
                &mut root_y,
                &mut child_x,
                &mut child_y,
                &mut mask,
            )
        };

        QuaryPointer {
            root,
            child,
            root_x,
            root_y,
            child_x,
            child_y,
            mask,
        }
    }

    pub fn quary_keymap(&self) -> QuaryKeymap {
        let mut keys = [0i8; 32];

        unsafe { functions::XQueryKeymap(self._d, &mut keys[0]) };

        QuaryKeymap { keys }
    }

    pub fn grab_key(
        &self,
        keycode: i32,
        modifiers: u32,
        grab_window: types::Window,
        owner_events: bool,
        pointer_mode: GrabMode,
        keyboard_mode: GrabMode,
    ) -> i32 {
        unsafe {
            functions::XGrabKey(
                self._d,
                keycode,
                modifiers,
                grab_window,
                owner_events,
                pointer_mode as i32,
                keyboard_mode as i32,
            )
        }
    }

    pub fn grab_button(
        &self,
        button: u32,
        modifiers: u32,
        grab_window: types::Window,
        owner_events: bool,
        event_mask: u32,
        pointer_mode: GrabMode,
        keyboard_mode: GrabMode,
        confine_to: types::Window,
        cursor: u32,
    ) -> i32 {
        unsafe {
            functions::XGrabButton(
                self._d,
                button,
                modifiers,
                grab_window,
                owner_events,
                event_mask,
                pointer_mode as i32,
                keyboard_mode as i32,
                confine_to,
                cursor,
            )
        }
    }

    pub fn grab_pointer(
        &self,
        grab_window: types::Window,
        owner_events: bool,
        event_mask: u32,
        pointer_mode: GrabMode,
        keyboard_mode: GrabMode,
        confine_to: types::Window,
        cursor: u32,
        time: u64,
    ) -> i32 {
        unsafe {
            functions::XGrabPointer(
                self._d,
                grab_window,
                owner_events,
                event_mask,
                pointer_mode as i32,
                keyboard_mode as i32,
                confine_to,
                cursor,
                time,
            )
        }
    }

    pub fn grab_keyboard(
        &self,
        grab_window: types::Window,
        owner_events: bool,
        pointer_mode: GrabMode,
        keyboard_mode: GrabMode,
        time: u64,
    ) -> i32 {
        unsafe {
            functions::XGrabKeyboard(
                self._d,
                grab_window,
                owner_events,
                pointer_mode as i32,
                keyboard_mode as i32,
                time,
            )
        }
    }
}

pub struct QuaryPointer {
    pub root: types::Window,
    pub child: types::Window,
    pub root_x: i32,
    pub root_y: i32,
    pub child_x: i32,
    pub child_y: i32,
    pub mask: u32,
}

pub struct QuaryKeymap {
    pub keys: [i8; 32],
}

impl QuaryKeymap {
    pub fn get_keycodes(&self) -> Vec<u32> {
        let mut data = Vec::new();
        for i in 0..32u32 {
            let mut x = 0u32;
            while x < 8 {
                if self.keys[i as usize] & (1 << x) != 0 {
                    data.push(i * 8 + x);
                }
                x += 1;
            }
        }
        data
    }
}

pub unsafe fn c_str_to_string(c_str: *mut i8) -> String {
    let mut data = String::new();
    let mut index: usize = 0;
    loop {
        let c_char = *c_str.offset(index as isize);
        if c_char == 0 {
            break;
        }
        data.push(char::from_u32(c_char as u32).unwrap());
        index += 1;
    }

    data
}
