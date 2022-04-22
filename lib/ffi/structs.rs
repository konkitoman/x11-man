use super::types::*;

#[derive(Debug)]
#[repr(C)]
pub struct XExtData {
    pub number: i32,
    pub next: *mut XExtData,
    pub free_private: *mut extern "C" fn(*mut XExtData) -> i32,
    pub private_data: i8,
}

#[derive(Debug)]
#[repr(C)]
pub struct Visual {
    pub ext_data: *mut XExtData,
    pub visualid: VisualID,
    pub class: i32,
    pub red_mask: u64,
    pub green_mask: u64,
    pub blue_mask: u64,
    pub bits_per_rgb: i32,
    pub map_entries: i32,
}

#[derive(Debug)]
#[repr(C)]
pub struct Depth {
    pub depth: i32,
    pub nvisuals: i32,
    pub visuals: *mut Visual,
}

#[derive(Debug)]
#[repr(C)]
pub struct GC {
    pub ext_data: *mut XExtData,
    pub gid: GContext,
}

#[derive(Debug)]
#[repr(C)]
pub struct Screen {
    pub ext_data: *mut XExtData,
    pub display: *mut Display,
    pub root: Window,
    pub width: i32,
    pub height: i32,
    pub mwidth: i32,
    pub mheight: i32,
    pub ndepths: i32,
    pub depths: *mut Depth,
    pub root_depth: i32,
    pub root_visual: *mut Visual,
    pub default_gc: *mut GC,
    pub cmap: Colormap,
    pub white_pixel: u64,
    pub black_pixel: u64,
    pub max_maps: i32,
    pub min_maps: i32,
    pub backing_store: i32,
    pub save_unders: bool,
    pub root_input_mask: i64,
}

#[derive(Debug)]
#[repr(C)]
pub struct ScreenFormat {
    pub ext_data: *mut XExtData,
    pub depth: i32,
    pub bits_per_pixel: i32,
    pub scanline_pad: i32,
}
#[derive(Debug)]
#[repr(C)]
pub struct Display {
    pub ext_data: *mut XExtData,
    pub private1: *mut _XPrivate,
    pub fd: i32,
    pub private2: i32,
    pub proto_major_version: i32,
    pub proto_minor_version: i32,
    pub vendor: *mut i8,
    pub private3: XID,
    pub private4: XID,
    pub private5: XID,
    pub private6: i32,
    pub resource_alloc: *mut extern "C" fn(*mut Display) -> XID,
    pub byte_order: i32,
    pub bitmap_unit: i32,
    pub bitmap_pad: i32,
    pub bitmap_bit_order: i32,
    pub nformats: i32,
    pub pixmap_format: *mut ScreenFormat,
    pub private8: i32,
    pub release: i32,
    pub private9: *mut _XPrivate,
    pub private10: *mut _XPrivate,
    pub qlen: i32,
    pub last_request_read: u64,
    pub request: u64,
    pub private11: XPointer,
    pub private12: XPointer,
    pub private13: XPointer,
    pub private14: XPointer,
    pub max_request_size: u32,
    pub db: *mut _XrmHashBucketRec,
    pub private15: *mut extern "C" fn(*mut Display) -> i32,
    pub display_name: *mut i8,
    pub default_screen: i32,
    pub nscreens: i32,
    pub screens: *mut Screen,
    pub motion_buffer: u64,
    pub private16: *mut i8,
    pub min_keycode: i32,
    pub max_keycode: i32,
    pub private17: XPointer,
    pub private18: XPointer,
    pub private19: i32,
    pub xdefaults: *mut i8,
}
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct XInputClassInfo {
    pub input_class: u8,
    pub event_type_base: u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct XDevice {
    pub device_id: XID,
    pub num_classes: i32,
    pub classes: *mut XInputClassInfo,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct XComposeStatus {
    pub compose_ptr: *mut i8,
    pub chars_matched: i32,
}

// X11 events
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XAnyEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut Display,
    pub window: Window,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XKeyEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: i32,
    pub y: i32,
    pub x_root: i32,
    pub y_root: i32,
    pub state: u32,
    pub keycode: u32,
    pub same_screen: bool,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XButtonEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: i32,
    pub y: i32,
    pub x_root: i32,
    pub y_root: i32,
    pub state: u32,
    pub button: u32,
    pub same_screen: bool,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XMotionEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: i32,
    pub y: i32,
    pub x_root: i32,
    pub y_root: i32,
    pub state: u32,
    pub is_hint: i8,
    pub same_screen: bool,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XCrossingEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
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

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XFocusChangeEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut Display,
    pub window: Window,
    pub mode: i32,
    pub detail: i32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XExposeEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut Display,
    pub window: Window,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub count: i32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XGraphicsExposeEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub count: i32,
    pub major_code: i32,
    pub minor_code: i32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XNoExposeEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub major_code: i32,
    pub minor_code: i32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XVisibilityEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut Display,
    pub window: Window,
    pub state: i32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XCreateWindowEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub border_width: i32,
    pub override_redirect: bool,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XDestroyWindowEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XUnmapEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub from_configure: bool,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XMapEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub override_redirect: bool,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XMapRequestEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XReparentEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub parent: Window,
    pub x: i32,
    pub y: i32,
    pub override_redirect: bool,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XConfigureEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub border_width: i32,
    pub above: Window,
    pub override_redirect: bool,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XGravityEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XResizeRequestEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut Display,
    pub window: Window,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XConfigureRequestEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub border_width: i32,
    pub above: Window,
    pub detail: i32,
    pub value_mask: u32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XCirculateEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub place: i32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XCirculateRequestEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub place: i32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XPropertyEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut Display,
    pub window: Window,
    pub atom: Atom,
    pub time: Time,
    pub state: i32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XSelectionClearEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut Display,
    pub window: Window,
    pub selection: Atom,
    pub time: Time,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XSelectionRequestEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut Display,
    pub owner: Window,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XSelectionEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut Display,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XColormapEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut Display,
    pub window: Window,
    pub colormap: Colormap,
    pub c_new: bool,
    pub state: i32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XClientMessageEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut Display,
    pub window: Window,
    pub message_type: Atom,
    pub format: i32,
    pub data: [i64; 5],
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XMappingEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut Display,
    pub window: Window,
    pub request: i32,
    pub first_keycode: u8,
    pub count: u8,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XErrorEvent {
    pub _type: i32,
    pub display: *mut Display,
    pub resourceid: XID,
    pub serial: u64,
    pub error_code: i32,
    pub request_code: i32,
    pub minor_code: i32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XKeymapEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut Display,
    pub window: Window,
    pub key_vector: [u8; 32],
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XGenericEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut Display,
    pub extension: i32,
    pub evtype: i32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XGenericEventCookie {
    pub _type: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut Display,
    pub extension: i32,
    pub evtype: i32,
    pub cookie: u32,
    pub data: *mut u8,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union XEvent {
    pub _type: i32,
    pub xany: XAnyEvent,
    pub xkey: XKeyEvent,
    pub xbutton: XButtonEvent,
    pub xmotion: XMotionEvent,
    pub xcrossing: XCrossingEvent,
    pub xfocus: XFocusChangeEvent,
    pub xexpose: XExposeEvent,
    pub xgraphicsexpose: XGraphicsExposeEvent,
    pub xnoexpose: XNoExposeEvent,
    pub xvisibility: XVisibilityEvent,
    pub xcreatewindow: XCreateWindowEvent,
    pub xdestroywindow: XDestroyWindowEvent,
    pub xunmap: XUnmapEvent,
    pub xmap: XMapEvent,
    pub xmaprequest: XMapRequestEvent,
    pub xreparent: XReparentEvent,
    pub xconfigure: XConfigureEvent,
    pub xgravity: XGravityEvent,
    pub xresizerequest: XResizeRequestEvent,
    pub xconfigurerequest: XConfigureRequestEvent,
    pub xcirculate: XCirculateEvent,
    pub xcirculaterequest: XCirculateRequestEvent,
    pub xproperty: XPropertyEvent,
    pub xselectionclear: XSelectionClearEvent,
    pub xselectionrequest: XSelectionRequestEvent,
    pub xselection: XSelectionEvent,
    pub xcolormap: XColormapEvent,
    pub xclient: XClientMessageEvent,
    pub xmapping: XMappingEvent,
    pub xerror: XErrorEvent,
    pub xkeymap: XKeymapEvent,
    pub xgeneric: XGenericEvent,
    pub xgenericcookie: XGenericEventCookie,
    pub pad: [i64; 24],
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XAnyClassInfo {
    pub class: XID,
    pub length: i32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XDeviceInfo {
    pub id: XID,
    pub _type: Atom,
    pub name: *mut i8,
    pub num_classes: i32,
    pub _use: i32,
    pub inputclassinfo: *mut XAnyClassInfo,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XDeviceKeyEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut Display,
    pub window: Window,
    pub deviceid: XID,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: i32,
    pub y: i32,
    pub x_root: i32,
    pub y_root: i32,
    pub state: u32,
    pub keycode: u32,
    pub same_screen: bool,
    pub device_state: u32,
    pub axes_count: u8,
    pub first_axis: u8,
    pub axis_data: [i32; 6],
}

impl XDeviceKeyEvent {
    pub unsafe fn from(event: *mut XEvent) -> *mut XDeviceKeyEvent {
        event as *mut XDeviceKeyEvent
    }
    pub unsafe fn as_key_event(&self) -> XKeyEvent {
        XKeyEvent {
            _type: self._type,
            serial: self.serial,
            send_event: self.send_event,
            display: self.display,
            window: self.window,
            root: self.root,
            subwindow: self.subwindow,
            time: self.time,
            x: self.x,
            y: self.y,
            x_root: self.x_root,
            y_root: self.y_root,
            state: self.state,
            keycode: self.keycode,
            same_screen: self.same_screen,
        }
    }
}
