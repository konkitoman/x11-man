use super::constants::KeyClass;
use super::constants::_deviceKeyPress;
use super::constants::_deviceKeyRelease;
use super::structs::*;
use super::types::*;

#[link(name = "X11", kind = "dylib")]
extern "C" {
    pub fn XOpenDisplay(display: *const libc::c_char) -> *mut Display;
    pub fn XCreateSimpleWindow(
        display: *mut Display,
        parent: Window,
        x: libc::c_int,
        y: libc::c_int,
        width: libc::c_uint,
        height: libc::c_uint,
        border_width: libc::c_uint,
        black_pixel: libc::c_ulong,
        white_pixel: libc::c_ulong,
    ) -> Window;
    pub fn XRootWindow(display: *mut Display, screen_number: libc::c_int) -> Window;
    pub fn XSelectInput(display: *mut Display, w: Window, event_mask: libc::c_long) -> libc::c_int;
    pub fn XMapWindow(display: *mut Display, w: Window) -> libc::c_int;
    pub fn XNextEvent(display: *mut Display, event: *mut XEvent) -> libc::c_int;
    pub fn XFillRectangle(
        display: *mut Display,
        drawable: Drawable,
        gc: *mut GC,
        x: libc::c_int,
        y: libc::c_int,
        width: libc::c_uint,
        height: libc::c_uint,
    ) -> libc::c_int;
    pub fn XDrawString(
        display: *mut Display,
        drawable: Drawable,
        gc: *mut GC,
        x: libc::c_int,
        y: libc::c_int,
        string: *const libc::c_char,
        length: libc::c_int,
    ) -> libc::c_int;
    pub fn XLookupKeysym(key_event: *const XKeyEvent, index: libc::c_int) -> KeySym;
    pub fn XLookupString(
        key_event: *const XKeyEvent,
        buffer_return: *mut libc::c_char,
        bytes_buffer: libc::c_int,
        keysym_return: *mut KeySym,
        status_in_out: *mut XComposeStatus,
    ) -> libc::c_int;
    pub fn XStoreName(display: *mut Display, w: Window, window_name: *const libc::c_char);
}

#[link(name = "Xi", kind = "dylib")]
extern "C" {
    pub fn XListInputDevices(display: *mut Display, ndevices: *mut libc::c_int)
        -> *mut XDeviceInfo;
    pub fn XOpenDevice(display: *mut Display, device_id: XID) -> *mut XDevice;
    pub fn XCloseDevice(display: *mut Display, device: *mut XDevice) -> libc::c_int;
    pub fn XSelectExtensionEvent(
        display: *mut Display,
        window: Window,
        event_class: *mut XEventClass,
        count: libc::c_int,
    ) -> libc::c_int;
}

pub unsafe fn FindTypeAndClass(
    device: *mut XDevice,
    _type: &mut i32,
    class: &mut XEventClass,
    class_id: i32,
    offset: i32,
) {
    *_type = 0;
    *class = 0;

    let mut i: i32 = 0;
    let mut ip = (*device).classes;

    loop {
        if i >= (*device).num_classes {
            break;
        }
        let mut ip = ip.offset(i as isize);
        if (*ip).input_class == class_id as u8 {
            *_type = (*ip).event_type_base as i32 + offset;
            *class = (*device).device_id << 8 | *_type as u64;
        }
        i += 1;
    }
}

pub unsafe fn DeviceKeyPress(device: *mut XDevice, _type: &mut i32, class: &mut XEventClass) {
    FindTypeAndClass(device, _type, class, KeyClass, _deviceKeyPress);
}

pub unsafe fn DeviceKeyRelease(device: *mut XDevice, _type: &mut i32, class: &mut XEventClass) {
    FindTypeAndClass(device, _type, class, KeyClass, _deviceKeyRelease);
}
