use super::constants::KeyClass;
use super::constants::_deviceKeyPress;
use super::constants::_deviceKeyRelease;
use super::structs::*;
use super::types::*;

#[link(name = "X11", kind = "dylib")]
extern "C" {
    pub fn XOpenDisplay(display: *const i8) -> *mut Display;
    pub fn XCreateSimpleWindow(
        display: *mut Display,
        parent: Window,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        border_width: u32,
        black_pixel: u64,
        white_pixel: u64,
    ) -> Window;
    pub fn XRootWindow(display: *mut Display, screen_number: i32) -> Window;
    pub fn XSelectInput(display: *mut Display, w: Window, event_mask: i64) -> i32;
    pub fn XMapWindow(display: *mut Display, w: Window) -> i32;
    pub fn XNextEvent(display: *mut Display, event: *mut XEvent) -> i32;
    pub fn XFillRectangle(
        display: *mut Display,
        drawable: Drawable,
        gc: *mut GC,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) -> i32;
    pub fn XDrawString(
        display: *mut Display,
        drawable: Drawable,
        gc: *mut GC,
        x: i32,
        y: i32,
        string: *const i8,
        length: i32,
    ) -> i32;
    pub fn XLookupKeysym(key_event: *const XKeyEvent, index: i32) -> KeySym;
    pub fn XLookupString(
        key_event: *const XKeyEvent,
        buffer_return: *mut i8,
        bytes_buffer: i32,
        keysym_return: *mut KeySym,
        status_in_out: *mut XComposeStatus,
    ) -> i32;
    pub fn XStoreName(display: *mut Display, w: Window, window_name: *const i8);
    pub fn XGetInputFocus(display: *mut Display, window: *mut Window, revert: *mut i32) -> i32;
    pub fn XQueryPointer(
        display: *mut Display,
        window: Window,
        root: *mut Window,
        child: *mut Window,
        root_x: *mut i32,
        root_y: *mut i32,
        child_x: *mut i32,
        child_y: *mut i32,
        mask: *mut u32,
    ) -> bool;
    /// `keys` - Need to be `[0i8; 32]`
    pub fn XQueryKeymap(display: *mut Display, keys: *mut i8) -> i32;
    pub fn XGrabKey(
        display: *mut Display,
        keycode: i32,
        modifiers: u32,
        grab_window: Window,
        owner_events: bool,
        pointer_mode: i32,
        keyboard_mode: i32,
    ) -> i32;
    pub fn XGrabButton(
        display: *mut Display,
        button: u32,
        modifiers: u32,
        grab_window: Window,
        owner_events: bool,
        event_mask: u32,
        pointer_mode: i32,
        keyboard_mode: i32,
        confine_to: Window,
        cursor: u32,
    ) -> i32;
    pub fn XGrabPointer(
        display: *mut Display,
        grab_window: Window,
        owner_events: bool,
        event_mask: u32,
        pointer_mode: i32,
        keyboard_mode: i32,
        confine_to: Window,
        curosr: u32,
        time: Time,
    ) -> i32;
    pub fn XGrabKeyboard(
        display: *mut Display,
        window: Window,
        owner_events: bool,
        pointer_mode: i32,
        keyboard_mode: i32,
        time: Time,
    ) -> i32;

}

#[link(name = "Xi", kind = "dylib")]
extern "C" {
    pub fn XListInputDevices(display: *mut Display, ndevices: *mut i32) -> *mut XDeviceInfo;
    pub fn XOpenDevice(display: *mut Display, device_id: XID) -> *mut XDevice;
    pub fn XCloseDevice(display: *mut Display, device: *mut XDevice) -> i32;
    pub fn XSelectExtensionEvent(
        display: *mut Display,
        window: Window,
        event_class: *mut XEventClass,
        count: i32,
    ) -> i32;
}

pub unsafe fn find_type_and_class(
    device: *mut XDevice,
    _type: &mut i32,
    class: &mut XEventClass,
    class_id: i32,
    offset: i32,
) {
    *_type = 0;
    *class = 0;

    let mut i: i32 = 0;

    loop {
        if i >= (*device).num_classes {
            break;
        }
        let ip = (*device).classes.offset(i as isize);
        if (*ip).input_class == class_id as u8 {
            *_type = (*ip).event_type_base as i32 + offset;
            *class = (*device).device_id << 8 | *_type as u64;
        }
        i += 1;
    }
}

pub unsafe fn device_key_press(device: *mut XDevice, _type: &mut i32, class: &mut XEventClass) {
    find_type_and_class(device, _type, class, KeyClass, _deviceKeyPress);
}

pub unsafe fn device_key_release(device: *mut XDevice, _type: &mut i32, class: &mut XEventClass) {
    find_type_and_class(device, _type, class, KeyClass, _deviceKeyRelease);
}
