use super::types::*;

#[derive(Debug)]
#[repr(C)]
pub struct XExtData {
    pub number: libc::c_int,
    pub next: *mut XExtData,
    pub free_private: *mut fn(*mut XExtData) -> libc::c_int,
    pub private_data: *mut libc::c_char,
}

#[derive(Debug)]
#[repr(C)]
pub struct Visual {
    pub ext_data: *mut XExtData,
    pub visualid: VisualID,
    pub class: libc::c_int,
    pub red_mask: libc::c_ulong,
    pub green_mask: libc::c_ulong,
    pub blue_mask: libc::c_ulong,
    pub bits_per_rgb: libc::c_int,
    pub map_entries: libc::c_int,
}

#[derive(Debug)]
#[repr(C)]
pub struct Depth {
    pub depth: libc::c_int,
    pub nvisuals: libc::c_int,
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
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub mwidth: libc::c_int,
    pub mheight: libc::c_int,
    pub ndepths: libc::c_int,
    pub depths: *mut Depth,
    pub root_depth: libc::c_int,
    pub root_visual: *mut Visual,
    pub default_gc: *mut GC,
    pub cmap: Colormap,
    pub white_pixel: libc::c_ulong,
    pub black_pixel: libc::c_ulong,
    pub max_maps: libc::c_int,
    pub min_maps: libc::c_int,
    pub backing_store: libc::c_int,
    pub save_unders: bool,
    pub root_input_mask: libc::c_long,
}

#[derive(Debug)]
#[repr(C)]
pub struct ScreenFormat {
    pub ext_data: *mut XExtData,
    pub depth: libc::c_int,
    pub bits_per_pixel: libc::c_int,
    pub scanline_pad: libc::c_int,
}
#[derive(Debug)]
#[repr(C)]
pub struct Display {
    pub ext_data: *mut XExtData,
    pub private1: *mut _XPrivate,
    pub fd: libc::c_int,
    pub private2: libc::c_int,
    pub proto_major_version: libc::c_int,
    pub proto_minor_version: libc::c_int,
    pub vendor: *mut libc::c_char,
    pub private3: XID,
    pub private4: XID,
    pub private5: XID,
    pub private6: libc::c_int,
    pub resource_alloc: *mut fn(*mut Display) -> XID,
    pub byte_order: libc::c_int,
    pub bitmap_unit: libc::c_int,
    pub bitmap_pad: libc::c_int,
    pub bitmap_bit_order: libc::c_int,
    pub nformats: libc::c_int,
    pub pixmap_format: *mut ScreenFormat,
    pub private8: libc::c_int,
    pub release: libc::c_int,
    pub private9: *mut _XPrivate,
    pub private10: *mut _XPrivate,
    pub qlen: libc::c_int,
    pub last_request_read: libc::c_ulong,
    pub request: libc::c_ulong,
    pub private11: XPointer,
    pub private12: XPointer,
    pub private13: XPointer,
    pub private14: XPointer,
    pub max_request_size: libc::c_uint,
    pub db: *mut _XrmHashBucketRec,
    pub private15: *mut fn(*mut Display) -> libc::c_int,
    pub display_name: *mut libc::c_char,
    pub default_screen: libc::c_int,
    pub nscreens: libc::c_int,
    pub screens: *mut Screen,
    pub motion_buffer: libc::c_ulong,
    pub private16: *mut libc::c_char,
    pub min_keycode: libc::c_int,
    pub max_keycode: libc::c_int,
    pub private17: XPointer,
    pub private18: XPointer,
    pub private19: libc::c_int,
    pub xdefaults: *mut libc::c_char,
}
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct XInputClassInfo {
    pub input_class: libc::c_uchar,
    pub event_type_base: libc::c_uchar,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct XDevice {
    pub device_id: XID,
    pub num_classes: libc::c_int,
    pub classes: *mut XInputClassInfo,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct XComposeStatus {
    pub compose_ptr: *mut libc::c_char,
    pub chars_matched: libc::c_int,
}

// X11 events
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XAnyEvent {
    pub _type: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: bool,
    pub display: *mut Display,
    pub window: Window,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XKeyEvent {
    pub _type: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: bool,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub state: libc::c_uint,
    pub keycode: libc::c_uint,
    pub same_screen: bool,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XButtonEvent {
    pub _type: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: bool,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub state: libc::c_uint,
    pub button: libc::c_uint,
    pub same_screen: bool,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XMotionEvent {
    pub _type: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: bool,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub state: libc::c_uint,
    pub is_hint: libc::c_char,
    pub same_screen: bool,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XCrossingEvent {
    pub _type: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: bool,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub mode: libc::c_int,
    pub detail: libc::c_int,
    pub same_screen: bool,
    pub focus: bool,
    pub state: libc::c_uint,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XFocusChangeEvent {
    pub _type: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: bool,
    pub display: *mut Display,
    pub window: Window,
    pub mode: libc::c_int,
    pub detail: libc::c_int,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XExposeEvent {
    pub _type: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: bool,
    pub display: *mut Display,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub count: libc::c_int,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XGraphicsExposeEvent {
    pub _type: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: bool,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub count: libc::c_int,
    pub major_code: libc::c_int,
    pub minor_code: libc::c_int,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XNoExposeEvent {
    pub _type: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: bool,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub major_code: libc::c_int,
    pub minor_code: libc::c_int,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XVisibilityEvent {
    pub _type: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: bool,
    pub display: *mut Display,
    pub window: Window,
    pub state: libc::c_int,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XCreateWindowEvent {
    pub _type: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: bool,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub override_redirect: bool,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XDestroyWindowEvent {
    pub _type: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: bool,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XUnmapEvent {
    pub _type: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: bool,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub from_configure: bool,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XMapEvent {
    pub _type: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: bool,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub override_redirect: bool,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XMapRequestEvent {
    pub _type: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: bool,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XReparentEvent {
    pub _type: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: bool,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub parent: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub override_redirect: bool,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XConfigureEvent {
    pub _type: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: bool,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub above: Window,
    pub override_redirect: bool,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XGravityEvent {
    pub _type: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: bool,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XResizeRequestEvent {
    pub _type: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: bool,
    pub display: *mut Display,
    pub window: Window,
    pub width: libc::c_int,
    pub height: libc::c_int,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XConfigureRequestEvent {
    pub _type: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: bool,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub above: Window,
    pub detail: libc::c_int,
    pub value_mask: libc::c_uint,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XCirculateEvent {
    pub _type: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: bool,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub place: libc::c_int,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XCirculateRequestEvent {
    pub _type: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: bool,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub place: libc::c_int,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XPropertyEvent {
    pub _type: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: bool,
    pub display: *mut Display,
    pub window: Window,
    pub atom: Atom,
    pub time: Time,
    pub state: libc::c_int,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XSelectionClearEvent {
    pub _type: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: bool,
    pub display: *mut Display,
    pub window: Window,
    pub selection: Atom,
    pub time: Time,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XSelectionRequestEvent {
    pub _type: libc::c_int,
    pub serial: libc::c_ulong,
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
    pub _type: libc::c_int,
    pub serial: libc::c_ulong,
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
    pub _type: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: bool,
    pub display: *mut Display,
    pub window: Window,
    pub colormap: Colormap,
    pub c_new: bool,
    pub state: libc::c_int,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XClientMessageEvent {
    pub _type: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: bool,
    pub display: *mut Display,
    pub window: Window,
    pub message_type: Atom,
    pub format: libc::c_int,
    pub data: [libc::c_long; 5],
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XMappingEvent {
    pub _type: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: bool,
    pub display: *mut Display,
    pub window: Window,
    pub request: libc::c_int,
    pub first_keycode: libc::c_uchar,
    pub count: libc::c_uchar,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XErrorEvent {
    pub _type: libc::c_int,
    pub display: *mut Display,
    pub resourceid: XID,
    pub serial: libc::c_ulong,
    pub error_code: libc::c_int,
    pub request_code: libc::c_int,
    pub minor_code: libc::c_int,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XKeymapEvent {
    pub _type: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: bool,
    pub display: *mut Display,
    pub window: Window,
    pub key_vector: [libc::c_uchar; 32],
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XGenericEvent {
    pub _type: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: bool,
    pub display: *mut Display,
    pub extension: libc::c_int,
    pub evtype: libc::c_int,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XGenericEventCookie {
    pub _type: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: bool,
    pub display: *mut Display,
    pub extension: libc::c_int,
    pub evtype: libc::c_int,
    pub cookie: libc::c_uint,
    pub data: *mut libc::c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union XEvent {
    pub _type: libc::c_int,
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
    pub pad: [libc::c_long; 24],
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XAnyClassInfo {
    pub class: XID,
    pub length: libc::c_int,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XDeviceInfo {
    pub id: XID,
    pub _type: Atom,
    pub name: *mut libc::c_char,
    pub num_classes: libc::c_int,
    pub _use: libc::c_int,
    pub inputclassinfo: *mut XAnyClassInfo,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XDeviceKeyEvent {
    pub _type: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: bool,
    pub display: *mut Display,
    pub window: Window,
    pub deviceid: XID,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub state: libc::c_uint,
    pub keycode: libc::c_uint,
    pub same_screen: bool,
    pub device_state: libc::c_uint,
    pub axes_count: libc::c_uchar,
    pub first_axis: libc::c_uchar,
    pub axis_data: [libc::c_int; 6],
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
