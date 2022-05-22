use std::fmt::{Display, Formatter};
use std::time::Duration;

use crate::x::{
    ButtonGrabConfig, KeyGrabConfig, KeyboardGrabConfig, PointerGrabConfig, SelectInputConfig,
};

use super::x::GrabMode;

use super::ffi::{x, xlib};

#[derive(Debug, Copy, Clone)]
pub struct Screen {
    pub _s: *mut xlib::Screen,
}

unsafe impl Sync for Screen {}
unsafe impl Send for Screen {}

impl Screen {
    pub unsafe fn new(screen: *mut xlib::Screen) -> Self {
        Self { _s: screen }
    }
}

pub trait TInput {
    fn grab_key(&self, config: &KeyGrabConfig);
    fn grab_button(&self, config: &ButtonGrabConfig);
    fn grab_pointer(&self, config: &PointerGrabConfig);
    fn grab_keyboard(&self, config: &KeyboardGrabConfig);
    fn ungrab_key(&self, config: &KeyGrabConfig);
    fn ungrab_button(&self, config: &ButtonGrabConfig);
    fn ungrab_pointer(&self, config: &PointerGrabConfig);
    fn ungrab_keyboard(&self, config: &KeyboardGrabConfig);
    fn select_input(&self, config: &SelectInputConfig);
    fn next_event(&self) -> Event;
    fn try_next_event(&self) -> Option<Event>;
}

pub struct GC {
    pub _gc: *mut xlib::GC,
    pub _gc_values: xlib::XGCValues,
}

#[derive(Debug, Copy, Clone)]
pub struct XDisplay {
    pub _d: *mut xlib::Display,
    pub keyboard_grabmode: GrabMode,
    pub pointer_grabmode: GrabMode,
    pub owner_events: bool,
    pub time: u64,
}

unsafe impl Sync for XDisplay {}
unsafe impl Send for XDisplay {}

impl Default for XDisplay {
    fn default() -> Self {
        Self {
            _d: std::ptr::null_mut(),
            keyboard_grabmode: GrabMode::Async,
            pointer_grabmode: GrabMode::Async,
            owner_events: true,
            time: 0,
        }
    }
}

impl XDisplay {
    /// Open display or X11 session
    pub fn new(number: Option<i8>) -> Self {
        let display = match number {
            Some(number) => unsafe { xlib::XOpenDisplay(&number) },
            None => unsafe { xlib::XOpenDisplay(std::ptr::null()) },
        };

        Self {
            _d: display,
            ..Default::default()
        }
    }

    /// To create from unsafe ffi::xlib::Display
    ///
    /// Don't use this only if neseccary
    pub unsafe fn from_row(row: *mut xlib::Display) -> Self {
        Self {
            _d: row,
            ..Default::default()
        }
    }

    /// Get default screen number
    pub fn default_screen(&self) -> i32 {
        unsafe { (*self._d).default_screen }
    }

    /// Get screens
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

    /// Get screens length
    pub fn screens_len(&self) -> i32 {
        unsafe { (*self._d).nscreens }
    }

    pub fn get_default_screen(&self) -> Screen {
        self.screens()[self.default_screen() as usize].clone()
    }

    /// Get root window or main window
    pub fn root_window(&self, screen_number: i32) -> x::Window {
        unsafe { xlib::XRootWindow(self._d, screen_number) }
    }

    /// Select events mask from window
    pub fn select_input(&self, window: x::Window, mask: i64) -> i32 {
        unsafe { xlib::XSelectInput(self._d, window, mask) }
    }

    /// Set window title
    pub fn store_name(&self, window: x::Window, name: &str) {
        unsafe {
            xlib::XStoreName(self._d, window, name.as_ptr() as *const i8);
        }
    }

    /// Get focused window
    pub fn get_input_focus(&self) -> (x::Window, i32) {
        let mut window = 0;
        let mut revert = 0;

        unsafe { xlib::XGetInputFocus(self._d, &mut window, &mut revert) };

        (window, revert)
    }

    /// Returns the mouse position and button state.
    pub fn quary_pointer(&self, window: x::Window) -> QuaryPointer {
        let mut root = 0;
        let mut child = 0;

        let mut root_x = 0;
        let mut root_y = 0;
        let mut child_x = 0;
        let mut child_y = 0;
        let mut mask = 0;

        unsafe {
            xlib::XQueryPointer(
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

    /// Return all pressed keys
    pub fn quary_keymap(&self) -> QuaryKeymap {
        let mut keys = [0u8; 32];

        unsafe { xlib::XQueryKeymap(self._d, &mut keys) };

        QuaryKeymap { keys }
    }

    pub fn ungrap_key(&self, window: x::Window, keycode: u32, modifiers: u32) -> i32 {
        unsafe { xlib::XUngrabKey(self._d, keycode as i32, modifiers, window) }
    }

    pub fn ungrab_button(&self, window: x::Window, button: u32, modifiers: u32) {
        unsafe {
            xlib::XUngrabButton(self._d, button, modifiers, window);
        }
    }

    pub fn ungrab_pointer(&self) {
        unsafe {
            xlib::XUngrabPointer(self._d, self.time);
        }
    }

    pub fn ungrab_keyboard(&self) {
        unsafe {
            xlib::XUngrabKeyboard(self._d, self.time);
        }
    }
}

impl TInput for XDisplay {
    fn grab_key(&self, config: &KeyGrabConfig) {
        unsafe {
            xlib::XGrabKey(
                self._d,
                config.keycode,
                config.modifiers,
                self.root_window(self.default_screen()),
                config.owner_events as i32,
                config.pointer_grab_mode as i32,
                config.keyboard_grab_mode as i32,
            );
        }
    }

    fn grab_button(&self, config: &ButtonGrabConfig) {
        unsafe {
            xlib::XGrabButton(
                self._d,
                config.button,
                config.modifiers,
                self.root_window(self.default_screen()),
                config.owner_events as i32,
                config.as_mask(),
                config.pointer_grab_mode as i32,
                config.keyboard_grab_mode as i32,
                config.confine_to,
                config.cursor,
            );
        }
    }

    fn grab_pointer(&self, config: &PointerGrabConfig) {
        unsafe {
            xlib::XGrabPointer(
                self._d,
                self.root_window(self.default_screen()),
                config.owner_events as i32,
                config.as_mask(),
                config.pointer_grab_mode as i32,
                config.keyboard_grab_mode as i32,
                config.confine_to,
                config.cursor,
                config.time,
            );
        }
    }

    fn grab_keyboard(&self, config: &KeyboardGrabConfig) {
        unsafe {
            xlib::XGrabKeyboard(
                self._d,
                self.root_window(self.default_screen()),
                config.owner_events as i32,
                config.pointer_grab_mode as i32,
                config.keyboard_grab_mode as i32,
                config.time,
            );
        }
    }

    fn select_input(&self, config: &SelectInputConfig) {
        unsafe {
            xlib::XSelectInput(
                self._d,
                self.root_window(self.default_screen()),
                config.get_row(),
            );
        }
    }

    fn ungrab_key(&self, config: &KeyGrabConfig) {
        unsafe {
            xlib::XUngrabKey(
                self._d,
                config.keycode,
                config.modifiers,
                self.root_window(self.default_screen()),
            );
        }
    }

    fn ungrab_button(&self, config: &ButtonGrabConfig) {
        unsafe {
            xlib::XUngrabButton(
                self._d,
                config.button,
                config.modifiers,
                self.root_window(self.default_screen()),
            );
        }
    }

    fn ungrab_pointer(&self, config: &PointerGrabConfig) {
        unsafe {
            xlib::XUngrabPointer(self._d, config.time);
        }
    }

    fn ungrab_keyboard(&self, config: &KeyboardGrabConfig) {
        unsafe {
            xlib::XUngrabKeyboard(self._d, config.time);
        }
    }

    fn next_event(&self) -> Event {
        let mut event = xlib::XEvent { _type: 0 };
        unsafe {
            xlib::XNextEvent(self._d, &mut event);
            Event::from_raw(&event)
        }
    }

    fn try_next_event(&self) -> Option<Event> {
        let mut event = xlib::XEvent { _type: 0 };
        unsafe {
            let res = xlib::XCheckMaskEvent(self._d, -1, &mut event);
            if res == 0 {
                None
            } else {
                Some(Event::from_raw(&event))
            }
        }
    }
}

pub struct QuaryPointer {
    pub root: x::Window,
    pub child: x::Window,
    pub root_x: i32,
    pub root_y: i32,
    pub child_x: i32,
    pub child_y: i32,
    pub mask: u32,
}

pub struct QuaryKeymap {
    pub keys: [u8; 32],
}

impl QuaryKeymap {
    pub fn get_keycodes(&self) -> Vec<u64> {
        let mut data = Vec::new();
        for i in 0..32u64 {
            let mut x = 0u64;
            while x < 8 {
                if self.keys[i as usize] & (1 << x) != 0 {
                    data.push(i * 8 + x);
                }
                x += 1;
            }
        }
        println!("Data: {:?}", data);
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

#[derive(Debug, Copy, Clone)]
pub struct Keycode(pub u32);
impl Display for Keycode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Keycode: {}", self.0)
    }
}
impl PartialEq for Keycode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl PartialEq<u32> for Keycode {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}
impl PartialEq<i32> for Keycode {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other as u32
    }
}

#[derive(Debug, Clone)]
pub struct KeyEvent {
    pub serial: u64,
    pub send_event: bool,
    pub display: XDisplay,
    pub window: u64,
    pub root: u64,
    pub subwindow: u64,
    pub time: u64,
    pub x: i32,
    pub y: i32,
    pub x_root: i32,
    pub y_root: i32,
    pub keycode: Keycode,
    pub same_screen: bool,
}

#[derive(Debug, Clone)]
pub struct ButtonEvent {
    pub serial: u64,
    pub send_event: bool,
    pub display: XDisplay,
    pub window: u64,
    pub root: u64,
    pub subwindow: u64,
    pub time: u64,
    pub x: i32,
    pub y: i32,
    pub x_root: i32,
    pub y_root: i32,
    pub state: u32,
    pub button: u32,
    pub same_screen: bool,
}

#[derive(Debug, Clone)]
pub struct MotionEvent {
    pub serial: u64,
    pub send_event: bool,
    pub display: XDisplay,
    pub window: u64,
    pub root: u64,
    pub subwindow: u64,
    pub time: u64,
    pub x: i32,
    pub y: i32,
    pub x_root: i32,
    pub y_root: i32,
    pub is_hint: bool,
    pub state: u32,
    pub same_screen: bool,
}

#[derive(Debug, Clone)]
pub struct CrossingEvent {
    pub serial: u64,
    pub send_event: bool,
    pub display: XDisplay,
    pub window: u64,
    pub root: u64,
    pub subwindow: u64,
    pub time: u64,
    pub x: i32,
    pub y: i32,
    pub x_root: i32,
    pub y_root: i32,
    pub mode: i32,
    pub detail: i32,
    pub same_screen: bool,
    pub focus: bool,
    pub state: u32,
}

#[derive(Debug, Clone)]
pub struct FocusEvent {
    pub serial: u64,
    pub send_event: bool,
    pub display: XDisplay,
    pub window: u64,
    pub mode: i32,
    pub detail: i32,
}

#[derive(Debug, Clone)]
pub struct KeymapEvent {
    pub serial: u64,
    pub send_event: bool,
    pub display: XDisplay,
    pub window: u64,
    pub key_vector: [u8; 32],
}

#[derive(Debug, Clone)]
pub struct ExposeEvent {
    pub serial: u64,
    pub send_event: bool,
    pub display: XDisplay,
    pub window: u64,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub count: u32,
}

#[derive(Debug, Clone)]
pub struct GraphicsExposeEvent {
    pub serial: u64,
    pub send_event: bool,
    pub display: XDisplay,
    pub drawable: u64,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub count: u32,
    pub major_code: i32,
    pub minor_code: i32,
}

#[derive(Debug, Clone)]
pub struct NoExposeEvent {
    pub serial: u64,
    pub send_event: bool,
    pub display: XDisplay,
    pub drawable: u64,
    pub major_code: i32,
    pub minor_code: i32,
}

#[derive(Debug, Clone)]
pub struct VisibilityEvent {
    pub serial: u64,
    pub send_event: bool,
    pub display: XDisplay,
    pub window: u64,
    pub state: i32,
}

#[derive(Debug, Clone)]
pub struct CreateWindowEvent {
    pub serial: u64,
    pub send_event: bool,
    pub display: XDisplay,
    pub parent: u64,
    pub window: u64,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub border_width: u32,
    pub override_redirect: bool,
}

#[derive(Debug, Clone)]
pub struct DestroyWindowEvent {
    pub serial: u64,
    pub send_event: bool,
    pub display: XDisplay,
    pub event: u64,
    pub window: u64,
}

#[derive(Debug, Clone)]
pub struct UnmapEvent {
    pub serial: u64,
    pub send_event: bool,
    pub display: XDisplay,
    pub event: u64,
    pub window: u64,
    pub from_configure: bool,
}

#[derive(Debug, Clone)]
pub struct MapEvent {
    pub serial: u64,
    pub send_event: bool,
    pub display: XDisplay,
    pub event: u64,
    pub window: u64,
    pub override_redirect: bool,
}

#[derive(Debug, Clone)]
pub struct MapRequestEvent {
    pub serial: u64,
    pub send_event: bool,
    pub display: XDisplay,
    pub parent: u64,
    pub window: u64,
}

#[derive(Debug, Clone)]
pub struct ReparentEvent {
    pub serial: u64,
    pub send_event: bool,
    pub display: XDisplay,
    pub event: u64,
    pub window: u64,
    pub parent: u64,
    pub x: i32,
    pub y: i32,
    pub override_redirect: bool,
}

#[derive(Debug, Clone)]
pub struct ConfigureEvent {
    pub serial: u64,
    pub send_event: bool,
    pub display: XDisplay,
    pub event: u64,
    pub window: u64,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub border_width: u32,
    pub above: u64,
    pub override_redirect: bool,
}

#[derive(Debug, Clone)]
pub struct ConfigureRequestEvent {
    pub serial: u64,
    pub send_event: bool,
    pub display: XDisplay,
    pub parent: u64,
    pub window: u64,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub border_width: u32,
    pub above: u64,
    pub detail: i32,
    pub value_mask: u64,
}

#[derive(Debug, Clone)]
pub struct GravityEvent {
    pub serial: u64,
    pub send_event: bool,
    pub display: XDisplay,
    pub event: u64,
    pub window: u64,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone)]
pub struct ResizeRequestEvent {
    pub serial: u64,
    pub send_event: bool,
    pub display: XDisplay,
    pub window: u64,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone)]
pub struct CirculateEvent {
    pub serial: u64,
    pub send_event: bool,
    pub display: XDisplay,
    pub event: u64,
    pub window: u64,
    pub place: i32,
}

#[derive(Debug, Clone)]
pub struct CirculateRequestEvent {
    pub serial: u64,
    pub send_event: bool,
    pub display: XDisplay,
    pub parent: u64,
    pub window: u64,
    pub place: i32,
}

#[derive(Debug, Clone)]
pub struct PropertyEvent {
    pub serial: u64,
    pub send_event: bool,
    pub display: XDisplay,
    pub window: u64,
    pub atom: u64,
    pub time: u64,
    pub state: i32,
}

#[derive(Debug, Clone)]
pub struct SelectionClearEvent {
    pub serial: u64,
    pub send_event: bool,
    pub display: XDisplay,
    pub window: u64,
    pub selection: u64,
    pub time: u64,
}

#[derive(Debug, Clone)]
pub struct SelectionRequestEvent {
    pub serial: u64,
    pub send_event: bool,
    pub display: XDisplay,
    pub owner: u64,
    pub requestor: u64,
    pub selection: u64,
    pub target: u64,
    pub property: u64,
    pub time: u64,
}

#[derive(Debug, Clone)]
pub struct SelectionEvent {
    pub serial: u64,
    pub send_event: bool,
    pub display: XDisplay,
    pub requestor: u64,
    pub selection: u64,
    pub target: u64,
    pub property: u64,
    pub time: u64,
}

#[derive(Debug, Clone)]
pub struct ColormapEvent {
    pub serial: u64,
    pub send_event: bool,
    pub display: XDisplay,
    pub window: u64,
    pub colormap: u64,
    pub c_new: bool,
    pub state: i32,
}

#[derive(Debug, Clone)]
pub struct ClientMessageEvent {
    pub serial: u64,
    pub send_event: bool,
    pub display: XDisplay,
    pub window: u64,
    pub message_type: u64,
    pub format: i32,
    pub data: [i8; 20],
}

#[derive(Debug, Clone)]
pub struct MappingNotifyEvent {
    pub serial: u64,
    pub send_event: bool,
    pub display: XDisplay,
    pub window: u64,
    pub request: i32,
    pub first_keycode: i32,
    pub count: i32,
}

#[derive(Debug, Clone)]
pub struct GenericEvent {
    pub serial: u64,
    pub send_event: bool,
    pub display: XDisplay,
    pub extension: i32,
    pub evtype: i32,
}

#[derive(Debug, Clone)]
pub enum Event {
    KeyPress(KeyEvent),
    KeyRelease(KeyEvent),
    ButtonPress(ButtonEvent),
    ButtonRelease(ButtonEvent),
    MotionNotify(MotionEvent),
    EnterNotify(CrossingEvent),
    LeaveNotify(CrossingEvent),
    FocusIn(FocusEvent),
    FocusOut(FocusEvent),
    KeymapNotify(KeymapEvent),
    Expose(ExposeEvent),
    GraphicsExpose(GraphicsExposeEvent),
    NoExpose(NoExposeEvent),
    VisibilityNotify(VisibilityEvent),
    CreateNotify(CreateWindowEvent),
    DestroyNotify(DestroyWindowEvent),
    UnmapNotify(UnmapEvent),
    MapNotify(MapEvent),
    MapRequest(MapRequestEvent),
    ReparentNotify(ReparentEvent),
    ConfigureNotify(ConfigureEvent),
    ConfigureRequest(ConfigureRequestEvent),
    GravityNotify(GravityEvent),
    ResizeRequest(ResizeRequestEvent),
    CirculateNotify(CirculateEvent),
    CirculateRequest(CirculateRequestEvent),
    PropertyNotify(PropertyEvent),
    SelectionClear(SelectionClearEvent),
    SelectionRequest(SelectionRequestEvent),
    SelectionNotify(SelectionEvent),
    ColormapNotify(ColormapEvent),
    ClientMessage(ClientMessageEvent),
    MappingNotify(MappingNotifyEvent),
    GenericEvent(GenericEvent),
    LASTEvent,
    Unknown,
}

impl Event {
    pub unsafe fn from_raw(event: &xlib::XEvent) -> Self {
        match event._type {
            x::KeyPress => Event::KeyPress(KeyEvent {
                serial: event.xkey.serial,
                send_event: event.xkey.send_event > 0,
                display: XDisplay::from_row(event.xkey.display),
                window: event.xkey.window,
                root: event.xkey.root,
                subwindow: event.xkey.subwindow,
                time: event.xkey.time,
                x: event.xkey.x,
                y: event.xkey.y,
                x_root: event.xkey.x_root,
                y_root: event.xkey.y_root,
                keycode: Keycode(event.xkey.keycode),
                same_screen: event.xkey.same_screen > 0,
            }),
            x::KeyRelease => Event::KeyRelease(KeyEvent {
                serial: event.xkey.serial,
                send_event: event.xkey.send_event > 0,
                display: XDisplay::from_row(event.xkey.display),
                window: event.xkey.window,
                root: event.xkey.root,
                subwindow: event.xkey.subwindow,
                time: event.xkey.time,
                x: event.xkey.x,
                y: event.xkey.y,
                x_root: event.xkey.x_root,
                y_root: event.xkey.y_root,
                keycode: Keycode(event.xkey.keycode),
                same_screen: event.xkey.same_screen > 0,
            }),
            x::ButtonPress => Event::ButtonPress(ButtonEvent {
                serial: event.xbutton.serial,
                send_event: event.xbutton.send_event > 0,
                display: XDisplay::from_row(event.xbutton.display),
                window: event.xbutton.window,
                root: event.xbutton.root,
                subwindow: event.xbutton.subwindow,
                time: event.xbutton.time,
                x: event.xbutton.x,
                y: event.xbutton.y,
                x_root: event.xbutton.x_root,
                y_root: event.xbutton.y_root,
                state: event.xbutton.state,
                button: event.xbutton.button,
                same_screen: event.xbutton.same_screen > 0,
            }),
            x::ButtonRelease => Event::ButtonRelease(ButtonEvent {
                serial: event.xbutton.serial,
                send_event: event.xbutton.send_event > 0,
                display: XDisplay::from_row(event.xbutton.display),
                window: event.xbutton.window,
                root: event.xbutton.root,
                subwindow: event.xbutton.subwindow,
                time: event.xbutton.time,
                x: event.xbutton.x,
                y: event.xbutton.y,
                x_root: event.xbutton.x_root,
                y_root: event.xbutton.y_root,
                state: event.xbutton.state,
                button: event.xbutton.button,
                same_screen: event.xbutton.same_screen > 0,
            }),
            x::MotionNotify => Event::MotionNotify(MotionEvent {
                serial: event.xmotion.serial,
                send_event: event.xmotion.send_event > 0,
                display: XDisplay::from_row(event.xmotion.display),
                window: event.xmotion.window,
                root: event.xmotion.root,
                subwindow: event.xmotion.subwindow,
                time: event.xmotion.time,
                x: event.xmotion.x,
                y: event.xmotion.y,
                x_root: event.xmotion.x_root,
                y_root: event.xmotion.y_root,
                is_hint: event.xmotion.is_hint > 0,
                state: event.xmotion.state,
                same_screen: event.xmotion.same_screen > 0,
            }),
            x::EnterNotify => Event::EnterNotify(CrossingEvent {
                serial: event.xcrossing.serial,
                send_event: event.xcrossing.send_event > 0,
                display: XDisplay::from_row(event.xcrossing.display),
                window: event.xcrossing.window,
                root: event.xcrossing.root,
                subwindow: event.xcrossing.subwindow,
                time: event.xcrossing.time,
                x: event.xcrossing.x,
                y: event.xcrossing.y,
                x_root: event.xcrossing.x_root,
                y_root: event.xcrossing.y_root,
                mode: event.xcrossing.mode,
                detail: event.xcrossing.detail,
                same_screen: event.xcrossing.same_screen > 0,
                focus: event.xcrossing.focus > 0,
                state: event.xcrossing.state,
            }),
            x::LeaveNotify => Event::LeaveNotify(CrossingEvent {
                serial: event.xcrossing.serial,
                send_event: event.xcrossing.send_event > 0,
                display: XDisplay::from_row(event.xcrossing.display),
                window: event.xcrossing.window,
                root: event.xcrossing.root,
                subwindow: event.xcrossing.subwindow,
                time: event.xcrossing.time,
                x: event.xcrossing.x,
                y: event.xcrossing.y,
                x_root: event.xcrossing.x_root,
                y_root: event.xcrossing.y_root,
                mode: event.xcrossing.mode,
                detail: event.xcrossing.detail,
                same_screen: event.xcrossing.same_screen > 0,
                focus: event.xcrossing.focus > 0,
                state: event.xcrossing.state,
            }),
            x::FocusIn => Event::FocusIn(FocusEvent {
                serial: event.xfocus.serial,
                send_event: event.xfocus.send_event > 0,
                display: XDisplay::from_row(event.xfocus.display),
                window: event.xfocus.window,
                mode: event.xfocus.mode,
                detail: event.xfocus.detail,
            }),
            x::FocusOut => Event::FocusOut(FocusEvent {
                serial: event.xfocus.serial,
                send_event: event.xfocus.send_event > 0,
                display: XDisplay::from_row(event.xfocus.display),
                window: event.xfocus.window,
                mode: event.xfocus.mode,
                detail: event.xfocus.detail,
            }),
            x::KeymapNotify => Event::KeymapNotify(KeymapEvent {
                serial: event.xkeymap.serial,
                send_event: event.xkeymap.send_event > 0,
                display: XDisplay::from_row(event.xkeymap.display),
                window: event.xkeymap.window,
                key_vector: event.xkeymap.key_vector,
            }),
            x::Expose => Event::Expose(ExposeEvent {
                serial: event.xexpose.serial,
                send_event: event.xexpose.send_event > 0,
                display: XDisplay::from_row(event.xexpose.display),
                window: event.xexpose.window,
                x: event.xexpose.x,
                y: event.xexpose.y,
                width: event.xexpose.width as u32,
                height: event.xexpose.height as u32,
                count: event.xexpose.count as u32,
            }),
            x::GraphicsExpose => Event::GraphicsExpose(GraphicsExposeEvent {
                serial: event.xgraphicsexpose.serial,
                send_event: event.xgraphicsexpose.send_event > 0,
                display: XDisplay::from_row(event.xgraphicsexpose.display),
                drawable: event.xgraphicsexpose.drawable,
                x: event.xgraphicsexpose.x,
                y: event.xgraphicsexpose.y,
                width: event.xgraphicsexpose.width as u32,
                height: event.xgraphicsexpose.height as u32,
                count: event.xgraphicsexpose.count as u32,
                major_code: event.xgraphicsexpose.major_code,
                minor_code: event.xgraphicsexpose.minor_code,
            }),
            x::NoExpose => Event::NoExpose(NoExposeEvent {
                serial: event.xnoexpose.serial,
                send_event: event.xnoexpose.send_event > 0,
                display: XDisplay::from_row(event.xnoexpose.display),
                drawable: event.xnoexpose.drawable,
                major_code: event.xnoexpose.major_code,
                minor_code: event.xnoexpose.minor_code,
            }),
            x::VisibilityNotify => Event::VisibilityNotify(VisibilityEvent {
                serial: event.xvisibility.serial,
                send_event: event.xvisibility.send_event > 0,
                display: XDisplay::from_row(event.xvisibility.display),
                window: event.xvisibility.window,
                state: event.xvisibility.state,
            }),
            x::CreateNotify => Event::CreateNotify(CreateWindowEvent {
                serial: event.xcreatewindow.serial,
                send_event: event.xcreatewindow.send_event > 0,
                display: XDisplay::from_row(event.xcreatewindow.display),
                parent: event.xcreatewindow.parent,
                window: event.xcreatewindow.window,
                x: event.xcreatewindow.x,
                y: event.xcreatewindow.y,
                width: event.xcreatewindow.width as u32,
                height: event.xcreatewindow.height as u32,
                border_width: event.xcreatewindow.border_width as u32,
                override_redirect: event.xcreatewindow.override_redirect > 0,
            }),
            x::DestroyNotify => Event::DestroyNotify(DestroyWindowEvent {
                serial: event.xdestroywindow.serial,
                send_event: event.xdestroywindow.send_event > 0,
                display: XDisplay::from_row(event.xdestroywindow.display),
                event: event.xdestroywindow.event,
                window: event.xdestroywindow.window,
            }),
            x::UnmapNotify => Event::UnmapNotify(UnmapEvent {
                serial: event.xunmap.serial,
                send_event: event.xunmap.send_event > 0,
                display: XDisplay::from_row(event.xunmap.display),
                event: event.xunmap.event,
                window: event.xunmap.window,
                from_configure: event.xunmap.from_configure > 0,
            }),
            x::MapNotify => Event::MapNotify(MapEvent {
                serial: event.xmap.serial,
                send_event: event.xmap.send_event > 0,
                display: XDisplay::from_row(event.xmap.display),
                event: event.xmap.event,
                window: event.xmap.window,
                override_redirect: event.xmap.override_redirect > 0,
            }),
            x::MapRequest => Event::MapRequest(MapRequestEvent {
                serial: event.xmaprequest.serial,
                send_event: event.xmaprequest.send_event > 0,
                display: XDisplay::from_row(event.xmaprequest.display),
                parent: event.xmaprequest.parent,
                window: event.xmaprequest.window,
            }),
            x::ReparentNotify => Event::ReparentNotify(ReparentEvent {
                serial: event.xreparent.serial,
                send_event: event.xreparent.send_event > 0,
                display: XDisplay::from_row(event.xreparent.display),
                event: event.xreparent.event,
                window: event.xreparent.window,
                parent: event.xreparent.parent,
                x: event.xreparent.x,
                y: event.xreparent.y,
                override_redirect: event.xreparent.override_redirect > 0,
            }),
            x::ConfigureNotify => Event::ConfigureNotify(ConfigureEvent {
                serial: event.xconfigure.serial,
                send_event: event.xconfigure.send_event > 0,
                display: XDisplay::from_row(event.xconfigure.display),
                event: event.xconfigure.event,
                window: event.xconfigure.window,
                x: event.xconfigure.x,
                y: event.xconfigure.y,
                width: event.xconfigure.width as u32,
                height: event.xconfigure.height as u32,
                border_width: event.xconfigure.border_width as u32,
                above: event.xconfigure.above,
                override_redirect: event.xconfigure.override_redirect > 0,
            }),
            x::ConfigureRequest => Event::ConfigureRequest(ConfigureRequestEvent {
                serial: event.xconfigurerequest.serial,
                send_event: event.xconfigurerequest.send_event > 0,
                display: XDisplay::from_row(event.xconfigurerequest.display),
                parent: event.xconfigurerequest.parent,
                window: event.xconfigurerequest.window,
                x: event.xconfigurerequest.x,
                y: event.xconfigurerequest.y,
                width: event.xconfigurerequest.width as u32,
                height: event.xconfigurerequest.height as u32,
                border_width: event.xconfigurerequest.border_width as u32,
                above: event.xconfigurerequest.above,
                detail: event.xconfigurerequest.detail,
                value_mask: event.xconfigurerequest.value_mask,
            }),
            x::GravityNotify => Event::GravityNotify(GravityEvent {
                serial: event.xgravity.serial,
                send_event: event.xgravity.send_event > 0,
                display: XDisplay::from_row(event.xgravity.display),
                event: event.xgravity.event,
                window: event.xgravity.window,
                x: event.xgravity.x,
                y: event.xgravity.y,
            }),
            x::ResizeRequest => Event::ResizeRequest(ResizeRequestEvent {
                serial: event.xresizerequest.serial,
                send_event: event.xresizerequest.send_event > 0,
                display: XDisplay::from_row(event.xresizerequest.display),
                window: event.xresizerequest.window,
                width: event.xresizerequest.width as u32,
                height: event.xresizerequest.height as u32,
            }),
            x::CirculateNotify => Event::CirculateNotify(CirculateEvent {
                serial: event.xcirculate.serial,
                send_event: event.xcirculate.send_event > 0,
                display: XDisplay::from_row(event.xcirculate.display),
                event: event.xcirculate.event,
                window: event.xcirculate.window,
                place: event.xcirculate.place,
            }),
            x::CirculateRequest => Event::CirculateRequest(CirculateRequestEvent {
                serial: event.xcirculaterequest.serial,
                send_event: event.xcirculaterequest.send_event > 0,
                display: XDisplay::from_row(event.xcirculaterequest.display),
                parent: event.xcirculaterequest.parent,
                window: event.xcirculaterequest.window,
                place: event.xcirculaterequest.place,
            }),
            x::PropertyNotify => Event::PropertyNotify(PropertyEvent {
                serial: event.xproperty.serial,
                send_event: event.xproperty.send_event > 0,
                display: XDisplay::from_row(event.xproperty.display),
                window: event.xproperty.window,
                atom: event.xproperty.atom,
                time: event.xproperty.time,
                state: event.xproperty.state,
            }),
            x::SelectionClear => Event::SelectionClear(SelectionClearEvent {
                serial: event.xselectionclear.serial,
                send_event: event.xselectionclear.send_event > 0,
                display: XDisplay::from_row(event.xselectionclear.display),
                window: event.xselectionclear.window,
                selection: event.xselectionclear.selection,
                time: event.xselectionclear.time,
            }),
            x::SelectionRequest => Event::SelectionRequest(SelectionRequestEvent {
                serial: event.xselectionrequest.serial,
                send_event: event.xselectionrequest.send_event > 0,
                display: XDisplay::from_row(event.xselectionrequest.display),
                owner: event.xselectionrequest.owner,
                requestor: event.xselectionrequest.requestor,
                selection: event.xselectionrequest.selection,
                target: event.xselectionrequest.target,
                property: event.xselectionrequest.property,
                time: event.xselectionrequest.time,
            }),
            x::SelectionNotify => Event::SelectionNotify(SelectionEvent {
                serial: event.xselection.serial,
                send_event: event.xselection.send_event > 0,
                display: XDisplay::from_row(event.xselection.display),
                requestor: event.xselection.requestor,
                selection: event.xselection.selection,
                target: event.xselection.target,
                property: event.xselection.property,
                time: event.xselection.time,
            }),
            x::ColormapNotify => Event::ColormapNotify(ColormapEvent {
                serial: event.xcolormap.serial,
                send_event: event.xcolormap.send_event > 0,
                display: XDisplay::from_row(event.xcolormap.display),
                window: event.xcolormap.window,
                colormap: event.xcolormap.colormap,
                c_new: event.xcolormap.new > 0,
                state: event.xcolormap.state,
            }),
            x::ClientMessage => Event::ClientMessage(ClientMessageEvent {
                serial: event.xclient.serial,
                send_event: event.xclient.send_event > 0,
                display: XDisplay::from_row(event.xclient.display),
                window: event.xclient.window,
                message_type: event.xclient.message_type,
                format: event.xclient.format,
                data: event.xclient.data.b,
            }),
            x::MappingNotify => Event::MappingNotify(MappingNotifyEvent {
                serial: event.xmapping.serial,
                send_event: event.xmapping.send_event > 0,
                display: XDisplay::from_row(event.xmapping.display),
                window: event.xmapping.window,
                request: event.xmapping.request,
                first_keycode: event.xmapping.first_keycode,
                count: event.xmapping.count,
            }),
            x::GenericEvent => Event::GenericEvent(GenericEvent {
                serial: event.xgeneric.serial,
                send_event: event.xgeneric.send_event > 0,
                display: XDisplay::from_row(event.xgeneric.display),
                extension: event.xgeneric.extension,
                evtype: event.xgeneric.evtype,
            }),
            x::LASTEvent => Event::LASTEvent,

            _ => Event::Unknown,
        }
    }
}
