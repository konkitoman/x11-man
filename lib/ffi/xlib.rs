use super::x::*;

pub const X_HAVE_UTF8_STRING: i32 = 1;

pub type XPointer = *mut i8;

pub type Bool = i32;
pub type Status = i32;
pub const True: Bool = 1;
pub const False: Bool = 0;

pub const QueuedAlready: i32 = 0;
pub const QueuedAfterReading: i32 = 1;
pub const QueuedAfterFlush: i32 = 2;

// Todo: Copy from X11/Xlib.h 91 - 144

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XExtData {
    pub number: i32,
    pub next: *mut XExtData,
    pub free_private: *mut extern "C" fn(extensions: *mut XExtData) -> i32,
    pub private_data: XPointer,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XExtCodes {
    pub extension: i32,
    pub major_opcode: i32,
    pub first_event: i32,
    pub first_error: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XPixmapFormatValues {
    pub depth: i32,
    pub bits_per_pixel: i32,
    pub scanline_pad: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XGCValues {
    pub function: i32,
    pub plane_mask: u64,
    pub foreground: u64,
    pub background: u64,
    pub line_width: i32,
    pub line_style: i32,
    pub cap_style: i32,
    pub join_style: i32,
    pub fill_style: i32,
    pub fill_rule: i32,
    pub arc_mode: i32,
    pub title: Pixmap,
    pub stipple: Pixmap,
    pub ts_x_origin: i32,
    pub ts_y_origin: i32,
    pub font: Font,
    pub subwindow_mode: i32,
    pub graphics_exposures: Bool,
    pub clip_x_origin: i32,
    pub clip_y_origin: i32,
    pub clip_mask: Pixmap,
    pub dash_offset: i32,
    pub dashes: i8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _XGC {
    pub ext_data: *mut XExtData,
    pub gid: GContext,
}

pub type GC = *mut _XGC;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Visual {
    ext_data: *mut XExtData,
    visualid: VisualID,
    class: i32,
    red_mask: u64,
    green_mask: u64,
    blue_mask: u64,
    bits_per_rgb: i32,
    map_entries: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Depth {
    depth: i32,
    nvisuals: i32,
    visuals: *mut Visual,
}

//pub struct _XDisplay;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Screen {
    ext_data: *mut XExtData,
    display: *mut Display,
    root: Window,
    width: i32,
    height: i32,
    mwidth: i32,
    mheight: i32,
    ndepths: i32,
    depths: *mut Depth,
    root_depth: i32,
    root_visual: *mut Visual,
    default_gc: GC,
    cmap: Colormap,
    white_pixel: u64,
    black_pixel: u64,
    max_maps: i32,
    min_maps: i32,
    backing_store: i32,
    save_unders: Bool,
    root_input_mask: i64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScreenFormat {
    pub ext_data: *mut XExtData,
    pub depth: i32,
    pub bits_per_pixel: i32,
    pub scanline_pad: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XSetWindowAttributes {
    pub background_pixmap: Pixmap,
    pub background_pixel: u64,
    pub border_pixmap: Pixmap,
    pub border_pixel: u64,
    pub bit_gravity: i32,
    pub win_gravity: i32,
    pub backing_store: i32,
    pub backing_planes: u64,
    pub backing_pixel: u64,
    pub save_under: Bool,
    pub event_mask: i64,
    pub do_not_propagate_mask: i64,
    pub override_redirect: Bool,
    pub colormap: Colormap,
    pub cursor: Cursor,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XWindowAttributes {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub border_width: i32,
    pub depth: i32,
    pub visual: *mut Visual,
    pub root: Window,
    pub class: i32,
    pub bit_gravity: i32,
    pub win_gravity: i32,
    pub backing_store: i32,
    pub backing_planes: u64,
    pub backing_pixel: u64,
    pub save_under: Bool,
    pub colormap: Colormap,
    pub map_installed: Bool,
    pub map_state: i32,
    pub all_event_masks: i64,
    pub your_event_mask: i64,
    pub do_not_propagate_mask: i64,
    pub override_redirect: Bool,
    pub screen: *mut Screen,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XHostAddress {
    pub family: i32,
    pub length: i32,
    pub address: *mut u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XServerInterpretedAddress {
    pub typelength: i32,
    pub valuelength: i32,
    pub typ: *mut i8,
    pub value: *mut i8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XImage {
    pub width: i32,
    pub height: i32,
    pub xoffset: i32,
    pub format: i32,
    pub data: *mut i8,
    pub byte_order: i32,
    pub bitmap_unit: i32,
    pub bitmap_bit_order: i32,
    pub bitmap_pad: i32,
    pub depth: i32,
    pub bytes_per_line: i32,
    pub bits_per_pixel: i32,
    pub red_mask: u64,
    pub green_mask: u64,
    pub blue_mask: u64,
    pub obdata: XPointer,
    create_image: *mut extern "C" fn(
        display: *mut Display,
        visual: *mut Visual,
        depth: i32,
        data: *mut i8,
        width: i32,
        height: i32,
        bitmap_pad: i32,
        bytes_per_line: i32,
    ) -> *mut XImage,
    destroy_image: *mut extern "C" fn(*mut XImage) -> i32,
    get_pixel: *mut extern "C" fn(*mut XImage, i32, i32) -> u64,
    put_pixel: *mut extern "C" fn(*mut XImage, i32, i32, u64) -> i32,
    sub_image: *mut extern "C" fn(*mut XImage, i32, i32, u32, u32) -> *mut XImage,
    add_pixel: *mut extern "C" fn(*mut XImage, i64) -> i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XWindowChanges {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub border_width: i32,
    pub sibling: Window,
    pub stack_mode: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XColor {
    pub pixel: u64,
    pub red: u16,
    pub green: u16,
    pub blue: u16,
    pub flags: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XSegment {
    pub x1: i16,
    pub y1: i16,
    pub x2: i16,
    pub y2: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XPoint {
    pub x: i16,
    pub y: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XRectangle {
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XArc {
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub angle1: i16,
    pub angle2: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XKeyboardControl {
    pub key_click_percent: i32,
    pub bell_percent: i32,
    pub bell_pitch: i32,
    pub bell_duration: i32,
    pub led: i32,
    pub led_mode: i32,
    pub key: i32,
    pub auto_repeat_mode: i32,
}

#[repr(C)]
#[derive(Debug)]
pub struct XKeyboardState {
    pub key_click_percent: i32,
    pub bell_percent: i32,
    pub bell_pitch: u32,
    pub bell_duration: u32,
    pub led_mask: u64,
    pub global_auto_repeat: i32,
    pub auto_repeats: [i8],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XTimeCoord {
    pub time: Time,
    pub x: i16,
    pub y: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XModifierKeymap {
    pub max_keypermod: i32,
    pub modifiermap: *mut KeyCode,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _XPrivate;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _XrmHashBucketRec;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
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
    pub private16: u64,
    pub min_keycode: i32,
    pub max_keycode: i32,
    pub private17: XPointer,
    pub private18: XPointer,
    pub private19: i32,
    pub xdefaults: *mut i8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XKeyEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: Bool,
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
    pub same_screen: Bool,
}

pub type XKeyPressedEvent = XKeyEvent;
pub type XKeyReleasedEvent = XKeyEvent;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XButtonEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: Bool,
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
    pub same_screen: Bool,
}

pub type XButtonPressedEvent = XButtonEvent;
pub type XButtonReleasedEvent = XButtonEvent;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XMotionEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: Bool,
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
    pub same_screen: Bool,
}

pub type XPointerMovedEvent = XMotionEvent;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XCrossingEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: Bool,
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
    pub same_screen: Bool,
    pub focus: Bool,
    pub state: u32,
}

pub type XEnterWindowEvent = XCrossingEvent;
pub type XLeaveWindowEvent = XCrossingEvent;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XFocusChangeEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub mode: i32,
    pub detail: i32,
}

pub type XFocusInEvent = XFocusChangeEvent;
pub type XFocusOutEvent = XFocusChangeEvent;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XKeymapEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub key_vector: [u8; 32],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XExposeEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub count: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XGraphicsExposeEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: Bool,
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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XNoExposeEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: Bool,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub major_code: i32,
    pub minor_code: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XVisibilityEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub state: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XCreateWindowEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: Bool,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub border_width: i32,
    pub override_redirect: Bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XDestroyWindowEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: Bool,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XUnmapEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: Bool,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub from_configure: Bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XMapEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: Bool,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub override_redirect: Bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XMapRequestEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: Bool,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XReparentEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: Bool,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub parent: Window,
    pub x: i32,
    pub y: i32,
    pub override_redirect: Bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XConfigureEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: Bool,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub border_width: i32,
    pub above: Window,
    pub override_redirect: Bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XGravityEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: Bool,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XResizeRequestEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub width: i32,
    pub height: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XConfigureRequestEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: Bool,
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
    pub value_mask: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XCirculateEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: Bool,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub place: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XCirculateRequestEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: Bool,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub place: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XPropertyEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub atom: Atom,
    pub time: Time,
    pub state: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XSelectionClearEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub selection: Atom,
    pub time: Time,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XSelectionRequestEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: Bool,
    pub display: *mut Display,
    pub owner: Window,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XSelectionEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: Bool,
    pub display: *mut Display,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XColormapEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub colormap: Colormap,
    pub new: Bool,
    pub state: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union _XClientMessageEventData {
    pub b: [i8; 20],
    pub s: [i16; 10],
    pub l: [i32; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XClientMessageEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub message_type: Atom,
    pub format: i32,
    pub data: _XClientMessageEventData,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XMappingEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub request: i32,
    pub first_keycode: i32,
    pub count: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XErrorEvent {
    pub _type: i32,
    pub display: *mut Display,
    pub resourceid: XID,
    pub serial: u64,
    pub error_code: u8,
    pub request_code: u8,
    pub minor_code: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XAnyEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XGenericEvent {
    pub _type: i32,
    pub serial: u64,
    pub send_event: Bool,
    pub display: *mut Display,
    pub extension: i32,
    pub evtype: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XGenericEventCookie {
    pub _type: i32,
    pub serial: u64,
    pub send_event: Bool,
    pub display: *mut Display,
    pub extension: i32,
    pub evtype: i32,
    pub cookie: u32,
    pub data: *mut i8,
}

#[repr(C)]
#[derive(Copy, Clone)]
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

// #define XAllocID(dpy) ((*((_XPrivDisplay)(dpy))->resource_alloc)((dpy)))

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XCharStruct {
    pub lbearing: i16,
    pub rbearing: i16,
    pub width: i16,
    pub ascent: i16,
    pub descent: i16,
    pub attributes: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XFontProp {
    pub name: Atom,
    pub card32: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XFontStruct {
    pub ext_data: *mut XExtData,
    pub fid: Font,
    pub direction: u32,
    pub min_char_or_byte2: u32,
    pub max_char_or_byte2: u32,
    pub min_byte1: u32,
    pub max_byte1: u32,
    pub all_chars_exist: Bool,
    pub default_char: u32,
    pub n_properties: i32,
    pub properties: *mut XFontProp,
    pub min_bounds: XCharStruct,
    pub max_bounds: XCharStruct,
    pub per_char: *mut XCharStruct,
    pub ascent: i32,
    pub descent: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XTextItem {
    pub chars: *mut i8,
    pub nchars: i32,
    pub delta: i32,
    pub font: Font,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XChar2b {
    pub byte1: u8,
    pub byte2: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XTextItem16 {
    pub chars: *mut XChar2b,
    pub nchars: i32,
    pub delta: i32,
    pub font: Font,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union XEDataObject {
    pub gc: GC,
    pub visual: *mut Visual,
    pub screen: *mut Screen,
    pub pixmap_format: *mut ScreenFormat,
    pub font: *mut XFontStruct,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XFontSetExtents {
    pub max_ink_extent: XRectangle,
    pub max_logical_extent: XRectangle,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _XOM;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _XOC;

pub type XOM = *mut _XOM;
pub type XOC = *mut _XOC;
pub type XFontSet = *mut _XOC;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XmbTextItem {
    pub chars: *mut i8,
    pub nchars: i32,
    pub delta: i32,
    pub font_set: XFontSet,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XwcTextItem {
    pub chars: *mut u16,
    pub nchars: i32,
    pub delta: i32,
    pub font_set: XFontSet,
}

pub const XNRequiredCharSet: &str = "requiredCharSet";
pub const XNQueryOrientation: &str = "queryOrientation";
pub const XNBaseFontName: &str = "baseFontName";
pub const XNOMAutomatic: &str = "omAutomatic";
pub const XNMissingCharSet: &str = "missingCharSet";
pub const XNDefaultString: &str = "defaultString";
pub const XNOrientation: &str = "orientation";
pub const XNDirectionalDependentDrawing: &str = "directionalDependentDrawing";
pub const XNContextualDrawing: &str = "contextualDrawing";
pub const XNFontInfo: &str = "fontInfo";

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XOMCharSetList {
    pub charset_count: i32,
    pub charset_list: *mut *mut i8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum XOrientation {
    XOMOrientation_LTR_TTB,
    XOMOrientation_RTL_TTB,
    XOMOrientation_TTB_LTR,
    XOMOrientation_TTB_RTL,
    XOMOrientation_Context,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XOMFontInfo {
    pub num_font: i32,
    pub font_struct_list: *mut *mut XFontStruct,
    pub font_name_list: *mut *mut i8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _XIM;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _XIC;

pub type XIM = *mut _XIM;
pub type XIC = *mut _XIC;

pub type XIMProc = extern "C" fn(XIM, XPointer, XPointer);
pub type XICProc = extern "C" fn(XIC, XPointer, XPointer) -> Bool;
pub type XIDProc = extern "C" fn(*mut Display, XPointer, XPointer);

pub type XIMStyle = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XIMStyles {
    pub count_styles: u16,
    pub supported_styles: *mut XIMStyle,
}

pub const XIMPreeditArea: i64 = 0x0001;
pub const XIMPreeditCallbacks: i64 = 0x0002;
pub const XIMPreeditPosition: i64 = 0x0004;
pub const XIMPreeditNothing: i64 = 0x0008;
pub const XIMPreeditNone: i64 = 0x0010;
pub const XIMStatusArea: i64 = 0x0100;
pub const XIMStatusCallbacks: i64 = 0x0200;
pub const XIMStatusNothing: i64 = 0x0400;
pub const XIMStatusNone: i64 = 0x0800;

pub const XNVaNestedList: &str = "XNVaNestedList";
pub const XNQueryInputStyle: &str = "queryInputStyle";
pub const XNClientWindow: &str = "clientWindow";
pub const XNInputStyle: &str = "inputStyle";
pub const XNFocusWindow: &str = "focusWindow";
pub const XNResourceName: &str = "resourceName";
pub const XNResourceClass: &str = "resourceClass";
pub const XNGeometryCallback: &str = "geometryCallback";
pub const XNDestroyCallback: &str = "destroyCallback";
pub const XNFilterEvents: &str = "filterEvents";
pub const XNPreeditStartCallback: &str = "preeditStartCallback";
pub const XNPreeditDoneCallback: &str = "preeditDoneCallback";
pub const XNPreeditDrawCallback: &str = "preeditDrawCallback";
pub const XNPreeditCaretCallback: &str = "preeditCaretCallback";
pub const XNPreeditStateNotifyCallback: &str = "preeditStateNotifyCallback";
pub const XNPreeditAttributes: &str = "preeditAttributes";
pub const XNStatusStartCallback: &str = "statusStartCallback";
pub const XNStatusDoneCallback: &str = "statusDoneCallback";
pub const XNStatusDrawCallback: &str = "statusDrawCallback";
pub const XNStatusAttributes: &str = "statusAttributes";
pub const XNArea: &str = "area";
pub const XNAreaNeeded: &str = "areaNeeded";
pub const XNSpotLocation: &str = "spotLocation";
pub const XNColormap: &str = "colorMap";
pub const XNStdColormap: &str = "stdColorMap";
pub const XNForeground: &str = "foreground";
pub const XNBackground: &str = "background";
pub const XNBackgroundPixmap: &str = "backgroundPixmap";
pub const XNFontSet: &str = "fontSet";
pub const XNLineSpace: &str = "lineSpace";
pub const XNCursor: &str = "cursor";

pub const XNQueryIMValuesList: &str = "queryIMValuesList";
pub const XNQueryICValuesList: &str = "queryICValuesList";
pub const XNVisiblePosition: &str = "visiblePosition";
pub const XNR6PreeditCallback: &str = "r6PreeditCallback";
pub const XNStringConversionCallback: &str = "stringConversionCallback";
pub const XNStringConversion: &str = "stringConversion";
pub const XNResetState: &str = "resetState";
pub const XNHotKey: &str = "hotKey";
pub const XNHotKeyState: &str = "hotKeyState";
pub const XNPreeditState: &str = "preeditState";
pub const XNSeparatorofNestedList: &str = "separatorofNestedList";

pub const XBufferOverflow: i32 = -1;
pub const XLookupNone: i32 = 1;
pub const XLookupChars: i32 = 2;
pub const XLookupKeySym: i32 = 3;
pub const XLookupBoth: i32 = 4;

pub type XVaNestedList = *mut i8;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XIMCallback {
    pub client_data: XPointer,
    pub callback: XIMProc,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XICCallback {
    pub client_data: XPointer,
    pub callback: XICProc,
}

pub type XIMFeedback = u64;

pub const XIMReverse: i64 = 1;
pub const XIMUnderline: i64 = 1 << 1;
pub const XIMHighlight: i64 = 1 << 2;
pub const XIMPrimary: i64 = 1 << 5;
pub const XIMSecondary: i64 = 1 << 6;
pub const XIMTertiary: i64 = 1 << 7;
pub const XIMVisibleToForward: i64 = 1 << 8;
pub const XIMVisibleToBackword: i64 = 1 << 9;
pub const XIMVisibleToCenter: i64 = 1 << 10;

#[repr(C)]
#[derive(Copy, Clone)]
pub union _XIMTextString {
    multi_byte: *mut i8,
    wide_char: *mut i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XIMText {
    pub length: u16,
    pub feedback: *mut XIMFeedback,
    pub encoding_is_wchar: Bool,
    pub string: _XIMTextString,
}

pub type XIMPreeditState = u64;

pub const XIMPreeditUnKnown: i64 = 0;
pub const XIMPreeditEnable: i64 = 1;
pub const XIMPreeditDisable: i64 = 1 << 1;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XIMPreeditStateNotifyCallbackStruct {
    pub state: XIMPreeditState,
}

pub type XIMResetState = u64;

pub const XIMInitialState: i64 = 1;
pub const XIMPreserveState: i64 = 1 << 1;

pub type XIMStringConversionFeedback = u64;

pub const XIMStringConversionLeftEdge: i64 = 0x00000001;
pub const XIMStringConversionRightEdge: i64 = 0x00000002;
pub const XIMStringConversionTopEdge: i64 = 0x00000004;
pub const XIMStringConversionBottomEdge: i64 = 0x00000008;
pub const XIMStringConversionConcealed: i64 = 0x00000010;
pub const XIMStringConversionWrapped: i64 = 0x00000020;

#[repr(C)]
#[derive(Copy, Clone)]
pub union _XIMStringConversionTextString {
    mbs: *mut i8,
    wcs: *mut i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XIMStringConversionText {
    pub length: u16,
    pub feedback: *mut XIMStringConversionFeedback,
    pub encoding_is_wchar: Bool,
    pub string: _XIMStringConversionTextString,
}

pub type XIMStringConversionPosition = u16;
pub type XIMStringConversionType = u16;

pub const XIMStringConversionBuffer: i32 = 0x0001;
pub const XIMStringConversionLine: i32 = 0x0002;
pub const XIMStringConversionWord: i32 = 0x0003;
pub const XIMStringConversionChar: i32 = 0x0004;

pub type XIMStringConversionOperation = u16;

pub const XIMStringConversionSubtitution: i32 = 0x0001;
pub const XIMStringConversionRetrieval: i32 = 0x0002;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum XIMCaretDirection {
    XIMForwardChar,
    XIMBackwardChar,
    XIMForwardWord,
    XIMBackwardWord,
    XIMCaretUp,
    XIMCaretDown,
    XIMNextLine,
    XIMPreviousLine,
    XIMLineStart,
    XIMLineEnd,
    XIMAbsolutePosition,
    XIMDontChange,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _XIMStringConversionCallbackStruct {
    pub position: XIMStringConversionPosition,
    pub direction: XIMCaretDirection,
    pub operation: XIMStringConversionOperation,
    pub factor: u16,
    pub text: *mut XIMStringConversionText,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XIMPreeditDrawCallbackStruct {
    pub caret: i32,
    pub chg_first: i32,
    pub chg_length: i32,
    pub text: *mut XIMText,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum XIMCaretStyle {
    XIMIsInvisible,
    XIMIsPrimary,
    XIMIsSecondary,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XIMPreeditCaretCallbackStruct {
    pub position: i32,
    pub direction: XIMCaretDirection,
    pub style: XIMCaretStyle,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum XIMStatusDataType {
    XIMTextType,
    XIMBitmapType,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union _XIMStatusDrawCallbackStructData {
    text: *mut XIMText,
    bitmap: Pixmap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XIMStatusDrawCallbackStruct {
    pub type_: XIMStatusDataType,
    pub data: _XIMStatusDrawCallbackStructData,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XIMHotKeyTrigger {
    pub keysym: KeySym,
    pub modifier: i32,
    pub modifier_mask: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XIMHotKeyTriggers {
    pub num_hot_key: i32,
    pub key: *mut XIMHotKeyTrigger,
}

pub type XIMHotKeyState = u64;

pub const XIMHotKeyStateON: i64 = 1;
pub const XIMHotKeyStateOFF: i64 = 2;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XIMValuesList {
    pub count_values: i32,
    pub supported_values: *mut *mut i8,
}

#[link(name = "X11", kind = "dylib")]
extern "C" {
    pub fn _Xmblen(str: *mut i8, len: i32) -> i32;

    pub static _Xdebug: i32;

    pub fn XLoadQueryFont(display: *mut Display, name: *const i8) -> *mut XFontStruct;
    pub fn XQueryFont(display: *mut Display, font_ID: Font) -> *mut XFontStruct;

    pub fn XGetMotionEvents(
        display: *mut Display,
        window: Window,
        start: Time,
        stop: Time,
        nevents_return: *mut i32,
    ) -> *mut XTimeCoord;

    pub fn XDeleteModifiermapEntry(
        modmap: *mut XModifierKeymap,
        keycode_entry: KeyCode,
        modifier: i32,
    ) -> *mut XModifierKeymap;
    pub fn XGetModifierMapping(display: *mut Display) -> *mut XModifierKeymap;
    pub fn XInsertModifiermapEntry(
        modmap: *mut XModifierKeymap,
        keycode_entry: KeyCode,
        modifier: i32,
    ) -> *mut XModifierKeymap;
    pub fn XNewModifiermap(num_modifier_keys: i32) -> *mut XModifierKeymap;

    pub fn XCreateImage(
        display: *mut Display,
        visual: *mut Visual,
        depth: i32,
        format: i32,
        offset: i32,
        data: *mut i8,
        width: u32,
        height: u32,
        bitmap_pad: i32,
        bytes_per_line: i32,
    ) -> *mut XImage;
    pub fn XInitImage(image: *mut XImage) -> Status;
    pub fn XGetImage(
        display: *mut Display,
        drawable: Drawable,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        plane_mask: u64,
        format: i32,
    ) -> *mut XImage;
    pub fn XGetSubImage(
        display: *mut Display,
        drawable: Drawable,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        plane_mask: u64,
        format: i32,
        dest_image: *mut XImage,
        dest_x: i32,
        dest_y: i32,
    ) -> *mut XImage;

    pub fn XOpenDisplay(display_name: *const i8) -> *mut Display;

    pub fn XrmInitialize();

    pub fn XFetchBytes(display: *mut Display, nbytes_return: *mut i32) -> *mut i8;
    pub fn XFetchBuffer(display: *mut Display, nbytes_return: *mut i32, buffer: i32) -> *mut i8;
    pub fn XGetAtomName(display: *mut Display, atom: Atom) -> *mut i8;
    pub fn XGetAtomNames(
        display: *mut Display,
        atoms: *mut Atom,
        count: i32,
        names_return: *mut *mut i8,
    ) -> i32;
    pub fn XGetDefault(display: *mut Display, program: *const i8, option: *const i8) -> *mut i8;
    pub fn XDisplayName(string: *const i8) -> *mut i8;
    pub fn XKeysymToString(keysym: KeySym) -> *mut i8;
    pub fn XSynchronize(display: *mut Display, onoff: Bool) -> Status;
    pub fn XSetAfterFunction(
        display: *mut Display,
        after_func: unsafe extern "C" fn(display: *mut Display),
    ) -> unsafe extern "C" fn(display: *mut Display);
    pub fn XInternAtom(display: *mut Display, atom_name: *const i8, only_if_exists: Bool) -> Atom;
    pub fn XCopyColormapAndFree(display: *mut Display, colormap: Colormap) -> Colormap;
    pub fn XCreateColormap(
        display: *mut Display,
        window: Window,
        visual: *mut Visual,
        alloc: i32,
    ) -> Colormap;
    pub fn XCreatePixmapCursor(
        display: *mut Display,
        source: Pixmap,
        mask: Pixmap,
        foreground_color: *mut XColor,
        background_color: *mut XColor,
        x: u32,
        y: u32,
    ) -> Cursor;
    pub fn XCreateGlyphCursor(
        display: *mut Display,
        source_font: Font,
        mask_font: Font,
        source_char: u32,
        mask_char: u32,
        foreground_color: *const XColor,
        background_color: *const XColor,
    ) -> Cursor;
    pub fn XCreateFontCursor(display: *mut Display, shape: u32) -> Cursor;
    pub fn XLoadFont(display: *mut Display, name: *const i8) -> Font;
    pub fn XCreateGC(
        display: *mut Display,
        drawable: Drawable,
        valuemask: u32,
        values: *const XGCValues,
    ) -> GC;
    pub fn XGContextFromGC(gc: GC) -> GContext;
    pub fn XFlushGC(display: *mut Display, gc: GC);
    pub fn XCreatePixmap(
        display: *mut Display,
        drawable: Drawable,
        width: u32,
        height: u32,
        depth: u32,
    ) -> Pixmap;
    pub fn XCreateBitmapFromData(
        display: *mut Display,
        ddrawable: Drawable,
        data: *const i8,
        width: u32,
        height: u32,
    ) -> Pixmap;
    pub fn XCreatePixmapFromBitmapData(
        display: *mut Display,
        ddrawable: Drawable,
        data: *mut i8,
        width: u32,
        height: u32,
        fg: u64,
        bg: u64,
        depth: u32,
    ) -> Pixmap;
    pub fn XCreateSimpleWindow(
        display: *mut Display,
        parent: Window,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        border_width: u32,
        border: u64,
        background: u64,
    ) -> Window;
    pub fn XCreateWindow(
        display: *mut Display,
        parent: Window,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        border_width: u32,
        depth: i32,
        class: u32,
        visual: *mut Visual,
        valuemask: u64,
        attributes: *mut XSetWindowAttributes,
    ) -> Window;
    pub fn XListInstalledColormaps(
        display: *mut Display,
        window: Window,
        num_return: *mut i32,
    ) -> *mut Colormap;
    pub fn XListFonts(
        display: *mut Display,
        pattern: *const i8,
        maxnames: i32,
        actual_count_return: *mut i32,
    ) -> *mut *mut i8;
    pub fn XListFontsWithInfo(
        display: *mut Display,
        pattern: *const i8,
        maxnames: i32,
        count_return: *mut i32,
        info_return: *mut *mut XFontStruct,
    ) -> *mut *mut i8;
    pub fn XGetFontPath(display: *mut Display, npaths_return: *mut i32) -> *mut *mut i8;
    pub fn XListExtensions(display: *mut Display, nextensions_return: *mut i32) -> *mut *mut i8;
    pub fn XListProperties(
        display: *mut Display,
        window: Window,
        num_prop_return: *mut i32,
    ) -> *mut Atom;
    pub fn XListHosts(
        display: *mut Display,
        nhosts_return: *mut i32,
        state_return: *mut Bool,
    ) -> *mut XHostAddress;
    pub fn XKeycodeToKeysym(display: *mut Display, keycode: KeyCode, index: i32) -> KeySym;
    pub fn XLookupKeysym(key_event: *mut XKeyEvent, index: i32) -> KeySym;
    pub fn XGetKeyboardMapping(
        display: *mut Display,
        first_keycode: KeyCode,
        keycode_count: i32,
        keysyms_per_keycode_return: *mut i32,
    ) -> *mut KeySym;
    pub fn XStringToKeysym(string: *const i8) -> KeySym;
    pub fn XMaxRequestSize(display: *mut Display) -> i64;
    pub fn XExtendedMaxRequestSize(display: *mut Display) -> i64;
    pub fn XResourceManagerString(display: *mut Display) -> *mut i8;
    pub fn XScreenResourceString(screen: *mut Screen) -> *mut i8;
    pub fn XDisplayMotionBufferSize(display: *mut Display) -> u64;
    pub fn XVisualIDFromVisual(visual: *mut Visual) -> VisualID;

    pub fn XInitThreads() -> Status;
    pub fn XLockDisplay(display: *mut Display);
    pub fn XUnlockDisplay(display: *mut Display);
    pub fn XInitExtension(display: *mut Display, name: *const i8) -> XExtCodes;
    pub fn XAddExtension(display: *mut Display) -> XExtCodes;
    pub fn XFindOnExtensionList(structure: *mut *mut XExtData, number: i32) -> *mut XExtData;
    pub fn XEHeadOfExtensionList(object: XEDataObject) -> *mut *mut XExtData;
    pub fn XRootWindow(display: *mut Display, screen_number: i32) -> Window;
    pub fn XDefaultRootWindow(display: *mut Display) -> Window;
    pub fn XRootWindowOfScreen(screen: *mut Screen) -> Window;
    pub fn XDefaultVisual(display: *mut Display, screen_number: i32) -> *mut Visual;
    pub fn XDefaultVisualOfScreen(screen: *mut Screen) -> *mut Visual;
    pub fn XDefaultGC(display: *mut Display, screen_number: i32) -> GC;
    pub fn XDefaultGCOfScreen(screen: *mut Screen) -> GC;
    pub fn XBlackPixel(display: *mut Display, screen_number: i32) -> u64;
    pub fn XWhitePixel(display: *mut Display, screen_number: i32) -> u64;
    pub fn XAllPlanes() -> u64;
    pub fn XBlackPixelOfScreen(screen: *mut Screen) -> u64;
    pub fn XWhitePixelOfScreen(screen: *mut Screen) -> u64;
    pub fn XNextRequest(display: *mut Display) -> u64;
    pub fn XLastKnownRequestProcessed(display: *mut Display) -> u64;
    pub fn XServerVendor(display: *mut Display) -> *mut i8;
    pub fn XDisplayString(display: *mut Display) -> *mut i8;
    pub fn XDefaultColormap(display: *mut Display, screen_number: i32) -> Colormap;
    pub fn XDefaultColormapOfScreen(screen: *mut Screen) -> Colormap;
    pub fn XDisplayOfScreen(screen: *mut Screen) -> *mut Display;
    pub fn XScreenOfDisplay(display: *mut Display, screen_number: i32) -> *mut Screen;
    pub fn XDefaultScreenOfDisplay(display: *mut Display) -> *mut Screen;
    pub fn XEventMaskOfScreen(screen: *mut Screen) -> i64;
    pub fn XScreenNumberOfScreen(screen: *mut Screen) -> i32;
}

pub type XErrorHandler = extern "C" fn(display: *mut Display, error_event: *mut XErrorEvent) -> i32;
pub type XIOErrorHandler = extern "C" fn(display: *mut Display) -> i32;
pub type XIOErrorExitHandler = extern "C" fn(display: *mut Display, user_data: *mut i8);

#[link(name = "X11", kind = "dylib")]
extern "C" {
    pub fn XSetErrorHandler(handler: XErrorHandler) -> XErrorHandler;
    pub fn XSetIOErrorHandler(handler: XIOErrorHandler) -> XIOErrorHandler;
    pub fn XSetIOErrorExitHandler(
        display: *mut Display,
        handler: XIOErrorExitHandler,
        user_data: *mut i8,
    );
}

#[link(name = "X11", kind = "dylib")]
extern "C" {
    pub fn XListPixmapFormats(
        display: *mut Display,
        count_return: *mut i32,
    ) -> *mut XPixmapFormatValues;
    pub fn XListDepths(
        display: *mut Display,
        screen_number: i32,
        count_return: *mut i32,
    ) -> *mut i32;
    pub fn XReconfigureWMWindow(
        display: *mut Display,
        window: Window,
        screen_number: i32,
        mask: u32,
        window_changes: *mut XWindowChanges,
    ) -> Status;
    pub fn XGetWMProtocols(
        display: *mut Display,
        window: Window,
        protocols_return: *mut *mut Atom,
        count_return: *mut i32,
    ) -> Status;
    pub fn XSetWMProtocols(
        display: *mut Display,
        window: Window,
        protocols: *mut Atom,
        count: i32,
    ) -> Status;
    pub fn XIconifyWindow(display: *mut Display, window: Window, screen_number: i32) -> Status;
    pub fn XWithdrawWindow(display: *mut Display, window: Window, screen_number: i32) -> Status;
    pub fn XGetCommand(
        display: *mut Display,
        window: Window,
        argv_return: *mut *mut *mut i8,
        argc_return: *mut i32,
    ) -> Status;
    pub fn XGetWMColormapWindows(
        display: *mut Display,
        window: Window,
        colormap_windows_return: *mut *mut Window,
        count_return: *mut i32,
    ) -> Status;
    pub fn XSetWMColormapWindows(
        display: *mut Display,
        window: Window,
        colormap_windows: *mut Window,
        count: i32,
    ) -> Status;
    pub fn XFreeStringList(list: *mut *mut i8);
    pub fn XSetTransientForHint(display: *mut Display, window: Window, parent: Window) -> i32;
    pub fn XActivateScreenSaver(display: *mut Display) -> i32;
    pub fn XAddHost(display: *mut Display, host: *mut XHostAddress) -> i32;
    pub fn XAddHosts(display: *mut Display, hosts: *mut XHostAddress, num_hosts: i32) -> i32;
    pub fn XAddToExtensionList(structure: *mut *mut XExtData, ext_data: *mut XExtData) -> i32;
    pub fn XAddToSaveSet(display: *mut Display, window: Window) -> i32;
    pub fn XAllocColor(display: *mut Display, colormap: Colormap, color: *mut XColor) -> Status;
    pub fn XAllocColorCells(
        display: *mut Display,
        colormap: Colormap,
        config: Bool,
        plane_masks_return: *mut u64,
        nplanes: u32,
        pixels_return: *mut u64,
        npixels: u32,
    ) -> Status;
    pub fn XAllocColorPlanes(
        display: *mut Display,
        colormap: Colormap,
        config: Bool,
        pixels_return: *mut u64,
        ncolors: i32,
        nreds: i32,
        ngreens: i32,
        nblues: i32,
        rmask_return: *mut u64,
        gmask_return: *mut u64,
        bmask_return: *mut u64,
    ) -> Status;
    pub fn XAllocNamedColor(
        display: *mut Display,
        colormap: Colormap,
        color_name: *const i8,
        exact_def_return: *mut XColor,
        screen_def_return: *mut XColor,
    ) -> Status;
    pub fn XAllowEvents(display: *mut Display, event_mode: i32, time: Time) -> i32;
    pub fn XAutoRepeatOff(display: *mut Display) -> i32;
    pub fn XAutoRepeatOn(display: *mut Display) -> i32;
    pub fn XBell(display: *mut Display, percent: i32) -> i32;
    pub fn XBitmapBitOrder(display: *mut Display) -> i32;
    pub fn XBitmapPad(display: *mut Display) -> i32;
    pub fn XBitmapUnit(display: *mut Display) -> i32;
    pub fn XCellsOfScreen(screen: *mut Screen) -> u32;
    pub fn XChangeActivePointerGrab(
        display: *mut Display,
        event_mask: u32,
        cursor: Cursor,
        time: Time,
    ) -> i32;
    pub fn XChangeGC(display: *mut Display, gc: GC, valuemask: u32, values: *mut XGCValues) -> i32;
    pub fn XChangeKeyboardControl(
        display: *mut Display,
        value_mask: u32,
        values: *mut XKeyboardControl,
    ) -> i32;
    pub fn XChangeKeyboardMapping(
        display: *mut Display,
        first_keycode: i32,
        keysyms_per_keycode: i32,
        keysyms: *mut KeySym,
        num_codes: i32,
    ) -> i32;
    pub fn XChangePointerControl(
        display: *mut Display,
        do_accel: Bool,
        do_threshold: Bool,
        acceleration_numerator: i32,
        acceleration_denominator: i32,
        threshold: i32,
    ) -> i32;
    pub fn XChangeProperty(
        display: *mut Display,
        window: Window,
        property: Atom,
        _type: Atom,
        format: i32,
        mode: i32,
        data: *const u8,
        nelements: u32,
    ) -> i32;
    pub fn XChangeSaveSet(display: *mut Display, window: Window, change_mode: i32) -> i32;
    pub fn XChangeWindowAttributes(
        display: *mut Display,
        window: Window,
        valuemask: u32,
        attributes: *mut XSetWindowAttributes,
    ) -> i32;
    pub fn XCheckIfEvent(
        display: *mut Display,
        event_return: *mut XEvent,
        predicate: *mut unsafe extern "C" fn(*mut Display, *mut XEvent, XPointer) -> Bool,
        arg: XPointer,
    ) -> Bool;
    pub fn XCheckMaskEvent(
        display: *mut Display,
        event_mask: i64,
        event_return: *mut XEvent,
    ) -> Bool;
    pub fn XCheckTypedEvent(
        display: *mut Display,
        event_type: i32,
        event_return: *mut XEvent,
    ) -> Bool;
    pub fn XCheckTypedWindowEvent(
        display: *mut Display,
        window: Window,
        event_type: i32,
        event_return: *mut XEvent,
    ) -> Bool;
    pub fn XCheckWindowEvent(
        display: *mut Display,
        window: Window,
        event_mask: i64,
        event_return: *mut XEvent,
    ) -> Bool;
    pub fn XCirculateSubwindows(display: *mut Display, window: Window, direction: i32) -> i32;
    pub fn XCirculateSubwindowsDown(display: *mut Display, window: Window) -> i32;
    pub fn XCirculateSubwindowsUp(display: *mut Display, window: Window) -> i32;
    pub fn XClearArea(
        display: *mut Display,
        window: Window,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        exposures: Bool,
    ) -> i32;
    pub fn XClearWindow(display: *mut Display, window: Window) -> i32;
    pub fn XCloseDisplay(display: *mut Display) -> i32;
    pub fn XConfigureWindow(
        display: *mut Display,
        window: Window,
        value_mask: u32,
        values: *mut XWindowChanges,
    ) -> i32;
    pub fn XConnectionNumber(display: *mut Display) -> i32;
    pub fn XConvertSelection(
        display: *mut Display,
        selection: Atom,
        target: Atom,
        property: Atom,
        requestor: Window,
        time: Time,
    ) -> i32;
    pub fn XCopyArea(
        display: *mut Display,
        src: Drawable,
        dest: Drawable,
        gc: GC,
        src_x: i32,
        src_y: i32,
        width: u32,
        height: u32,
        dest_x: i32,
        dest_y: i32,
    ) -> i32;
    pub fn XCopyGC(display: *mut Display, src: GC, valuemask: u64, dest: GC) -> i32;
    pub fn XCopyPlane(
        display: *mut Display,
        src: Drawable,
        dest: Drawable,
        gc: GC,
        src_x: i32,
        src_y: i32,
        width: u32,
        height: u32,
        dest_x: i32,
        dest_y: i32,
        plane: u32,
    ) -> i32;
    pub fn XDefaultDepth(display: *mut Display, screen_number: i32) -> i32;
    pub fn XDefaultDepthOfScreen(screen: *mut Screen) -> i32;
    pub fn XDefaultScreen(display: *mut Display) -> i32;
    pub fn XDefineCursor(display: *mut Display, window: Window, cursor: Cursor) -> i32;
    pub fn XDeleteProperty(display: *mut Display, window: Window, property: Atom) -> i32;
    pub fn XDestroyWindow(display: *mut Display, window: Window) -> i32;
    pub fn XDestroySubwindows(display: *mut Display, window: Window) -> i32;
    pub fn XDoesBackingStore(screen: *mut Screen) -> i32;
    pub fn XDoesSaveUnders(screen: *mut Screen) -> Bool;
    pub fn XDisableAccessControl(display: *mut Display) -> i32;
    pub fn XDisplayCells(display: *mut Display, screen_number: i32) -> i32;
    pub fn XDisplayHeight(display: *mut Display, screen_number: i32) -> i32;
    pub fn XDisplayHeightMM(display: *mut Display, screen_number: i32) -> i32;
    pub fn XDisplayKeycodes(
        display: *mut Display,
        min_keycodes_return: *mut i32,
        max_keycodes_return: *mut i32,
    ) -> i32;
    pub fn XDisplayPlanes(display: *mut Display, screen_number: i32) -> i32;
    pub fn XDisplayWidth(display: *mut Display, screen_number: i32) -> i32;
    pub fn XDisplayWidthMM(display: *mut Display, screen_number: i32) -> i32;
    pub fn XDrawArc(
        display: *mut Display,
        drawable: Drawable,
        gc: GC,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        angle1: i32,
        angle2: i32,
    ) -> i32;
    pub fn XDrawArcs(
        display: *mut Display,
        drawable: Drawable,
        gc: GC,
        arcs: *mut XArc,
        narcs: i32,
    ) -> i32;
    pub fn XDrawImageString(
        display: *mut Display,
        drawable: Drawable,
        gc: GC,
        x: i32,
        y: i32,
        string: *const i8,
        length: i32,
    ) -> i32;
    pub fn XDrawImageString16(
        display: *mut Display,
        drawable: Drawable,
        gc: GC,
        x: i32,
        y: i32,
        string: *const XChar2b,
        length: i32,
    ) -> i32;
    pub fn XDrawLine(
        display: *mut Display,
        drawable: Drawable,
        gc: GC,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> i32;
    pub fn XDrawLines(
        display: *mut Display,
        drawable: Drawable,
        gc: GC,
        points: *mut XPoint,
        npoints: i32,
        mode: i32,
    ) -> i32;
    pub fn XDrawPoint(display: *mut Display, drawable: Drawable, gc: GC, x: i32, y: i32) -> i32;
    pub fn XDrawPoints(
        display: *mut Display,
        drawable: Drawable,
        gc: GC,
        points: *mut XPoint,
        npoints: i32,
        mode: i32,
    ) -> i32;
    pub fn XDrawRectangle(
        display: *mut Display,
        drawable: Drawable,
        gc: GC,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) -> i32;
    pub fn XDrawRectangles(
        display: *mut Display,
        drawable: Drawable,
        gc: GC,
        rectangles: *mut XRectangle,
        nrectangles: i32,
    ) -> i32;
    pub fn XDrawSegments(
        display: *mut Display,
        drawable: Drawable,
        gc: GC,
        segments: *mut XSegment,
        nsegments: i32,
    ) -> i32;
    pub fn XDrawString(
        display: *mut Display,
        drawable: Drawable,
        gc: GC,
        x: i32,
        y: i32,
        string: *const i8,
        length: i32,
    ) -> i32;
    pub fn XDrawString16(
        display: *mut Display,
        drawable: Drawable,
        gc: GC,
        x: i32,
        y: i32,
        string: *const XChar2b,
        length: i32,
    ) -> i32;
    pub fn XDrawText(
        display: *mut Display,
        drawable: Drawable,
        gc: GC,
        x: i32,
        y: i32,
        strings: *mut XTextItem,
        nitems: i32,
    ) -> i32;
    pub fn XDrawText16(
        display: *mut Display,
        drawable: Drawable,
        gc: GC,
        x: i32,
        y: i32,
        items: *mut XTextItem16,
        nitems: i32,
    ) -> i32;
    pub fn XEnableAccessControl(display: *mut Display) -> i32;
    pub fn XEventsQueued(display: *mut Display, mode: i32) -> i32;
    pub fn XFetchName(display: *mut Display, window: Window, name_return: *mut *mut i8) -> i32;
    pub fn XFillArc(
        display: *mut Display,
        drawable: Drawable,
        gc: GC,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        angle1: i32,
        angle2: i32,
    ) -> i32;
    pub fn XFillArcs(
        display: *mut Display,
        drawable: Drawable,
        gc: GC,
        arcs: *mut XArc,
        narcs: i32,
    ) -> i32;
    pub fn XFillPolygon(
        display: *mut Display,
        drawable: Drawable,
        gc: GC,
        points: *mut XPoint,
        npoints: i32,
        shape: i32,
        mode: i32,
    ) -> i32;
    pub fn XFillRectangle(
        display: *mut Display,
        drawable: Drawable,
        gc: GC,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) -> i32;
    pub fn XFillRectangles(
        display: *mut Display,
        drawable: Drawable,
        gc: GC,
        rectangles: *mut XRectangle,
        nrectangles: i32,
    ) -> i32;
    pub fn XFlush(display: *mut Display) -> i32;
    pub fn XForceScreenSaver(display: *mut Display, mode: i32) -> i32;
    pub fn XFree(data: *mut i8);
    pub fn XFreeColormap(display: *mut Display, colormap: Colormap) -> i32;
    pub fn XFreeColors(
        display: *mut Display,
        colormap: Colormap,
        pixels: *mut u32,
        npixels: i32,
        planes: u32,
    ) -> i32;
    pub fn XFreeCursor(display: *mut Display, cursor: Cursor) -> i32;
    pub fn XFreeExtensionList(list: *mut *mut i8) -> i32;
    pub fn XFreeFont(display: *mut Display, font_struct: *mut XFontStruct) -> i32;
    pub fn XFreeFontInfo(
        names: *mut *mut i8,
        free_info: *mut XFontStruct,
        actual_count: i32,
    ) -> i32;
    pub fn XFreeFontNames(list: *mut *mut i8) -> i32;
    pub fn XFreeFontPath(list: *mut *mut i8) -> i32;
    pub fn XFreeGC(display: *mut Display, gc: GC) -> i32;
    pub fn XFreeModifiermap(modmap: *mut XModifierKeymap) -> i32;
    pub fn XFreePixmap(display: *mut Display, pixmap: Pixmap) -> i32;
    pub fn XGeometry(
        display: *mut Display,
        screen: i32,
        position: *const i8,
        default_position: *const i8,
        bwidth: u32,
        fwidth: u32,
        fheight: u32,
        xadder: i32,
        yadder: i32,
        x_return: *mut i32,
        y_return: *mut i32,
        width_return: *mut i32,
        height_return: *mut i32,
    ) -> i32;
    pub fn XGetErrorDatabaseText(
        display: *mut Display,
        name: *const i8,
        message: *const i8,
        default_string: *const i8,
        buffer_return: *mut i8,
        length: i32,
    ) -> i32;
    pub fn XGetErrorText(
        display: *mut Display,
        code: i32,
        buffer_return: *mut i8,
        length: i32,
    ) -> i32;
    pub fn XGetFontProperty(
        font_struct: *mut XFontStruct,
        atom: Atom,
        value_return: *mut u32,
    ) -> i32;
    pub fn XGetGCValues(
        display: *mut Display,
        gc: GC,
        valuemask: u32,
        values_return: *mut XGCValues,
    ) -> i32;
    pub fn XGetGeometry(
        display: *mut Display,
        d: Drawable,
        root_return: *mut Window,
        x_return: *mut i32,
        y_return: *mut i32,
        width_return: *mut u32,
        height_return: *mut u32,
        border_width_return: *mut u32,
        depth_return: *mut u32,
    ) -> i32;
    pub fn XGetIconName(
        display: *mut Display,
        window: Window,
        icon_name_return: *mut *mut i8,
    ) -> i32;
    pub fn XGetInputFocus(
        display: *mut Display,
        focus_return: *mut Window,
        revert_to_return: *mut i32,
    ) -> i32;
    pub fn XGetKeyboardControl(display: *mut Display, values_return: *mut XKeyboardState) -> i32;
    pub fn XGetPointerControl(
        display: *mut Display,
        accel_numerator_return: *mut i32,
        accel_denominator_return: *mut i32,
        threshold_return: *mut i32,
    ) -> i32;
    pub fn XGetPointerMapping(display: *mut Display, map_return: *mut u8, nmap: i32) -> i32;
    pub fn XGetScreenSaver(
        display: *mut Display,
        timeout_return: *mut i32,
        interval_return: *mut i32,
        prefer_blanking_return: *mut i32,
        allow_exposures_return: *mut i32,
    ) -> i32;
    pub fn XGetTransientForHint(
        display: *mut Display,
        window: Window,
        prop_window_return: *mut Window,
    ) -> i32;
    pub fn XGetWindowProperty(
        display: *mut Display,
        window: Window,
        atom: Atom,
        long_offset: i64,
        long_length: i64,
        delete: Bool,
        req_type: Atom,
        actual_type_return: *mut Atom,
        actual_format_return: *mut i32,
        nitems_return: *mut u64,
        bytes_after_return: *mut u64,
        prop_return: *mut *mut u8,
    ) -> Status;
    pub fn XGetWindowAttributes(
        display: *mut Display,
        window: Window,
        window_attributes_return: *mut XWindowAttributes,
    ) -> i32;
    pub fn XGrabButton(
        display: *mut Display,
        button: u32,
        modifiers: u32,
        grab_window: Window,
        owner_events: Bool,
        event_mask: u32,
        pointer_mode: i32,
        keyboard_mode: i32,
        confine_to: Window,
        cursor: Cursor,
    ) -> i32;
    pub fn XGrabKey(
        display: *mut Display,
        keycode: i32,
        modifiers: u32,
        grab_window: Window,
        owner_events: Bool,
        pointer_mode: i32,
        keyboard_mode: i32,
    ) -> i32;
    pub fn XGrabKeyboard(
        display: *mut Display,
        grab_window: Window,
        owner_events: Bool,
        pointer_mode: i32,
        keyboard_mode: i32,
        time: Time,
    ) -> i32;
    pub fn XGrabPointer(
        display: *mut Display,
        grab_window: Window,
        owner_events: Bool,
        event_mask: u32,
        pointer_mode: i32,
        keyboard_mode: i32,
        confine_to: Window,
        cursor: Cursor,
        time: Time,
    ) -> i32;
    pub fn XGrabServer(display: *mut Display) -> i32;
    pub fn XHeightMMOfScreen(screen: *mut Screen) -> u32;
    pub fn XHeightOfScreen(screen: *mut Screen) -> u32;
    pub fn XIfEvent(
        display: *mut Display,
        event_return: *mut XEvent,
        predicate: *mut unsafe extern "C" fn(*mut Display, *mut XEvent, *mut XPointer) -> Bool,
        arg: XPointer,
    ) -> i32;
    pub fn XImageByteOrder(display: *mut Display) -> i32;
    pub fn XInstallColormap(display: *mut Display, colormap: Colormap) -> i32;
    pub fn XKeysymToKeycode(display: *mut Display, keysym: KeySym) -> KeyCode;
    pub fn XKillClient(display: *mut Display, resource: XID) -> i32;
    pub fn XLookupColor(
        display: *mut Display,
        colormap: Colormap,
        color_name: *const i8,
        exact_def_return: *mut XColor,
        screen_def_return: *mut XColor,
    ) -> Status;
    pub fn XLowerWindow(display: *mut Display, window: Window) -> i32;
    pub fn XMapRaised(display: *mut Display, window: Window) -> i32;
    pub fn XMapSubwindows(display: *mut Display, window: Window) -> i32;
    pub fn XMapWindow(display: *mut Display, window: Window) -> i32;
    pub fn XMaskEvent(display: *mut Display, event_mask: i64, event_return: *mut XEvent) -> i32;
    pub fn XMaxCmapsOfScreen(screen: *mut Screen) -> i32;
    pub fn XMinCmapsOfScreen(screen: *mut Screen) -> i32;
    pub fn XMoveResizeWindow(
        display: *mut Display,
        window: Window,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) -> i32;
    pub fn XMoveWindow(display: *mut Display, window: Window, x: i32, y: i32) -> i32;
    pub fn XNextEvent(display: *mut Display, event_return: *mut XEvent) -> i32;
    pub fn XNoOp(display: *mut Display) -> i32;
    pub fn XParseColor(
        display: *mut Display,
        colormap: Colormap,
        spec: *const i8,
        exact_def_return: *mut XColor,
    ) -> Status;
    pub fn XParseGeometry(
        string: *const i8,
        x_return: *mut i32,
        y_return: *mut i32,
        width_return: *mut u32,
        height_return: *mut u32,
    ) -> i32;
    pub fn XPeekEvent(display: *mut Display, event_return: *mut XEvent) -> i32;
    pub fn XPeekIfEvent(
        display: *mut Display,
        event_return: *mut XEvent,
        predicate: *mut unsafe extern "C" fn(*mut Display, *mut XEvent, *mut XPointer) -> Bool,
        arg: XPointer,
    ) -> i32;
    pub fn XPending(display: *mut Display) -> i32;
    pub fn XPlanesOfScreen(screen: *mut Screen) -> u32;
    pub fn XProtocolRevision(display: *mut Display) -> i32;
    pub fn XProtocolVersion(display: *mut Display) -> i32;
    pub fn XPutBackEvent(display: *mut Display, event: *mut XEvent) -> i32;
    pub fn XPutImage(
        display: *mut Display,
        d: Drawable,
        gc: GC,
        image: *mut XImage,
        src_x: i32,
        src_y: i32,
        dest_x: i32,
        dest_y: i32,
        width: u32,
        height: u32,
    ) -> i32;
    pub fn XQLength(display: *mut Display) -> i32;
    pub fn XQueryBestCursor(
        display: *mut Display,
        drawable: Drawable,
        width: u32,
        height: u32,
        width_return: *mut u32,
        height_return: *mut u32,
    ) -> Cursor;
    pub fn XQueryBestSize(
        display: *mut Display,
        class: i32,
        drawable: Drawable,
        width: u32,
        height: u32,
        width_return: *mut u32,
        height_return: *mut u32,
    ) -> u32;
    pub fn XQueryBestStipple(
        display: *mut Display,
        drawable: Drawable,
        width: u32,
        height: u32,
        width_return: *mut u32,
        height_return: *mut u32,
    ) -> Pixmap;
    pub fn XQueryBestTile(
        display: *mut Display,
        drawable: Drawable,
        width: u32,
        height: u32,
        width_return: *mut u32,
        height_return: *mut u32,
    ) -> Pixmap;
    pub fn XQueryColor(
        display: *mut Display,
        colormap: Colormap,
        def_in_out: *mut XColor,
    ) -> Status;
    pub fn XQueryColors(
        display: *mut Display,
        colormap: Colormap,
        defs_in_out: *mut XColor,
        ncolors: i32,
    ) -> Status;
    pub fn XQueryExtension(
        display: *mut Display,
        name: *const i8,
        major_opcode_return: *mut i32,
        first_event_return: *mut i32,
        first_error_return: *mut i32,
    ) -> Bool;
    pub fn XQueryKeymap(display: *mut Display, keys_return: &mut [u8; 32]) -> i32;
    pub fn XQueryPointer(
        display: *mut Display,
        w: Window,
        root_return: *mut Window,
        child_return: *mut Window,
        root_x_return: *mut i32,
        root_y_return: *mut i32,
        win_x_return: *mut i32,
        win_y_return: *mut i32,
        keys_buttons_return: *mut u32,
    ) -> Bool;
    pub fn XQueryTextExtents(
        display: *mut Display,
        font_ID: XID,
        string: *const i8,
        nchars: i32,
        direction_return: *mut i32,
        font_ascent_return: *mut i32,
        font_descent_return: *mut i32,
        overall_return: *mut XCharStruct,
    ) -> i32;
    pub fn XQueryTextExtents16(
        display: *mut Display,
        font_ID: XID,
        string: *const XChar2b,
        nchars: i32,
        direction_return: *mut i32,
        font_ascent_return: *mut i32,
        font_descent_return: *mut i32,
        overall_return: *mut XCharStruct,
    ) -> i32;
    pub fn XQueryTree(
        display: *mut Display,
        window: Window,
        root_return: *mut Window,
        parent_return: *mut Window,
        children_return: *mut *mut Window,
        nchildren_return: *mut u32,
    ) -> Window;
    pub fn XRaiseWindow(display: *mut Display, window: Window) -> i32;
    pub fn XReadBitmapFile(
        display: *mut Display,
        d: Drawable,
        filename: *const i8,
        width_return: *mut u32,
        height_return: *mut u32,
        bitmap_return: *mut Pixmap,
        x_hot_return: *mut i32,
        y_hot_return: *mut i32,
    ) -> Status;
    pub fn XReadBitmapFileData(
        filename: *const i8,
        width_return: *mut u32,
        height_return: *mut u32,
        bitmap_return: *mut *mut u8,
        x_hot_return: *mut i32,
        y_hot_return: *mut i32,
    ) -> Status;
    pub fn XRebindKeysym(
        display: *mut Display,
        keysym: KeySym,
        list: *mut KeySym,
        mod_count: i32,
        string: *const u8,
        bytes_string: i32,
    ) -> Status;
    pub fn XRecolorCursor(
        display: *mut Display,
        cursor: Cursor,
        foreground_color: *mut XColor,
        background_color: *mut XColor,
    ) -> i32;
    pub fn XRefreshKeyboardMapping(event_map: *mut XMappingEvent) -> i32;
    pub fn XRemoveFromSaveSet(display: *mut Display, w: Window) -> i32;
    pub fn XRemoveHost(display: *mut Display, host: XHostAddress) -> i32;
    pub fn XRemoveHosts(display: *mut Display, hosts: *mut XHostAddress, nhosts: i32) -> i32;
    pub fn XReparentWindow(
        display: *mut Display,
        wwindow: Window,
        parent: Window,
        x: i32,
        y: i32,
    ) -> i32;
    pub fn XResetScreenSaver(display: *mut Display) -> i32;
    pub fn XResizeWindow(display: *mut Display, window: Window, width: u32, height: u32) -> i32;
    pub fn XRestackWindows(display: *mut Display, windows: *mut Window, nwindows: i32) -> i32;
    pub fn XRotateBuffers(display: *mut Display, rotate: i32) -> i32;
    pub fn XRotateWindowProperties(
        display: *mut Display,
        window: Window,
        props: *mut Atom,
        nprops: i32,
        delta: i32,
    ) -> i32;
    pub fn XScreenCount(display: *mut Display) -> i32;
    pub fn XSelectInput(display: *mut Display, window: Window, event_mask: i64) -> i32;
    pub fn XSendEvent(
        display: *mut Display,
        window: Window,
        propagate: Bool,
        event_mask: i64,
        event_send: *mut XEvent,
    ) -> Status;
    pub fn XSetAccessControl(display: *mut Display, mode: i32) -> i32;
    pub fn XSetArcMode(display: *mut Display, GC: GC, arc_mode: i32) -> i32;
    pub fn XSetBackground(display: *mut Display, GC: GC, background: u64) -> i32;
    pub fn XSetClipMask(display: *mut Display, GC: GC, pixmap: Pixmap) -> i32;
    pub fn XSetClipOrigin(
        display: *mut Display,
        GC: GC,
        clip_x_origin: i32,
        clip_y_origin: i32,
    ) -> i32;
    pub fn XSetClipRectangles(
        display: *mut Display,
        GC: GC,
        clip_x_origin: i32,
        clip_y_origin: i32,
        rectangles: *mut XRectangle,
        n: i32,
        ordering: i32,
    ) -> i32;
    pub fn XSetCloseDownMode(display: *mut Display, close_mode: i32) -> i32;
    pub fn XSetCommand(display: *mut Display, window: Window, argv: *mut *mut i8, argc: i32)
        -> i32;
    pub fn XSetDashes(
        display: *mut Display,
        GC: GC,
        dash_offset: i32,
        dash_list: *const i8,
        dashes_length: i32,
    );
    pub fn XSetFillRule(display: *mut Display, GC: GC, fill_rule: i32) -> i32;
    pub fn XSetFillStyle(display: *mut Display, GC: GC, fill_style: i32) -> i32;
    pub fn XSetFont(display: *mut Display, GC: GC, font: Font) -> i32;
    pub fn XSetFontPath(display: *mut Display, directories: *mut *mut i8, ndirs: i32) -> i32;
    pub fn XSetForeground(display: *mut Display, GC: GC, foreground: u64) -> i32;
    pub fn XSetFunction(display: *mut Display, GC: GC, function: i32) -> i32;
    pub fn XSetGraphicsExposures(display: *mut Display, GC: GC, graphics_exposures: Bool) -> i32;
    pub fn XSetIconName(display: *mut Display, w: Window, icon_name: *const i8) -> i32;
    pub fn XSetInputFocus(display: *mut Display, focus: Window, revert_to: i32, time: Time) -> i32;
    pub fn XSetLineAttributes(
        display: *mut Display,
        GC: GC,
        line_width: u32,
        line_style: i32,
        cap_style: i32,
        join_style: i32,
    ) -> i32;
    pub fn XSetModifierMapping(display: *mut Display, modmap: *mut XModifierKeymap) -> i32;
    pub fn XSetPlaneMask(display: *mut Display, GC: GC, plane_mask: u64) -> i32;
    pub fn XSetPointerMapping(display: *mut Display, map: *const u8, nmap: i32) -> i32;
    pub fn XSetScreenSaver(
        display: *mut Display,
        timeout: i32,
        interval: i32,
        prefer_blanking: i32,
        allow_exposures: i32,
    ) -> i32;
    pub fn XSetSelectionOwner(
        display: *mut Display,
        selection: Atom,
        owner: Window,
        time: Time,
    ) -> Status;
    pub fn XSetState(
        display: *mut Display,
        GC: GC,
        foreground: u64,
        background: u64,
        function: i32,
        plane_mask: u64,
    ) -> i32;
    pub fn XSetStipple(display: *mut Display, GC: GC, stipple: Pixmap) -> i32;
    pub fn XSetSubwindowMode(display: *mut Display, GC: GC, subwindow_mode: i32) -> i32;
    pub fn XSetTSOrigin(display: *mut Display, GC: GC, ts_x_origin: i32, ts_y_origin: i32) -> i32;
    pub fn XSetTile(display: *mut Display, GC: GC, tile: Pixmap) -> i32;
    pub fn XSetWindowBackground(display: *mut Display, window: Window, background: u64) -> i32;
    pub fn XSetWindowBackgroundPixmap(
        display: *mut Display,
        window: Window,
        background_pixmap: Pixmap,
    ) -> i32;
    pub fn XSetWindowBorder(display: *mut Display, window: Window, border_pixel: u32) -> i32;
    pub fn XSetWindowBorderPixmap(
        display: *mut Display,
        window: Window,
        border_pixmap: Pixmap,
    ) -> i32;
    pub fn XSetWindowBorderWidth(display: *mut Display, window: Window, width: u32) -> i32;
    pub fn XSetWindowColormap(display: *mut Display, window: Window, colormap: Colormap) -> i32;
    pub fn XStoreBuffer(display: *mut Display, bytes: *const i8, nbytes: i32, buffer: i32) -> i32;
    pub fn XStoreBytes(display: *mut Display, bytes: *const i8, nbytes: i32) -> i32;
    pub fn XStoreColor(display: *mut Display, colormap: Colormap, color: *mut XColor) -> i32;
    pub fn XStoreColors(
        display: *mut Display,
        colormap: Colormap,
        color: *mut XColor,
        ncolors: i32,
    ) -> i32;
    pub fn XStoreName(display: *mut Display, window: Window, window_name: *const i8) -> i32;
    pub fn XStoreNamedColor(
        display: *mut Display,
        colormap: Colormap,
        color: *const i8,
        pixel: u64,
        flags: i32,
    ) -> Status;
    pub fn XSync(display: *mut Display, discard: Bool) -> i32;
    pub fn XTextExtents(
        font: Font,
        string: *const i8,
        mchars: i32,
        direction_return: *mut i32,
        font_ascent_return: *mut i32,
        font_descent_return: *mut i32,
        overall_return: *mut XCharStruct,
    );
    pub fn XTextExtents16(
        font: Font,
        string: *const XChar2b,
        mchars: i32,
        direction_return: *mut i32,
        font_ascent_return: *mut i32,
        font_descent_return: *mut i32,
        overall_return: *mut XCharStruct,
    );
    pub fn XTextWidth(font: Font, string: *const i8, count: i32) -> u32;
    pub fn XTextWidth16(font: Font, string: *const XChar2b, count: i32) -> u32;
    pub fn XTranslateCoordinates(
        display: *mut Display,
        src_w: Window,
        dest_w: Window,
        src_x: i32,
        src_y: i32,
        dest_x_return: *mut i32,
        dest_y_return: *mut i32,
        child_return: *mut Window,
    ) -> Bool;
    pub fn XUndefineCursor(display: *mut Display, window: Window) -> i32;
    pub fn XUngrabButton(
        display: *mut Display,
        button: u32,
        modifiers: u32,
        grab_window: Window,
    ) -> i32;
    pub fn XUngrabKey(
        display: *mut Display,
        keycode: i32,
        modifiers: u32,
        grab_window: Window,
    ) -> i32;
    pub fn XUngrabKeyboard(display: *mut Display, time: Time) -> i32;
    pub fn XUngrabPointer(display: *mut Display, time: Time) -> i32;
    pub fn XUngrabServer(display: *mut Display) -> i32;
    pub fn XUninstallColormap(display: *mut Display, colormap: Colormap) -> i32;
    pub fn XUnloadFont(display: *mut Display, font: Font) -> i32;
    pub fn XUnmapSubwindows(display: *mut Display, window: Window) -> i32;
    pub fn XUnmapWindow(display: *mut Display, window: Window) -> i32;
    pub fn XVendorRelease(display: *mut Display) -> i32;
    pub fn XWarpPointer(
        display: *mut Display,
        src_w: Window,
        dest_w: Window,
        src_x: i32,
        src_y: i32,
        src_width: u32,
        src_height: u32,
        dest_x: i32,
        dest_y: i32,
    ) -> i32;
    pub fn XWidthMMOfScreen(screen: *mut Screen) -> u32;
    pub fn XWidthOfScreen(screen: *mut Screen) -> u32;
    pub fn XWindowEvent(
        display: *mut Display,
        window: Window,
        event_mask: i64,
        event_return: *mut XEvent,
    ) -> i32;
    pub fn XWriteBitmapFile(
        display: *mut Display,
        filename: *const i8,
        bitmap: *const Pixmap,
        width: u32,
        height: u32,
        x_hot: i32,
        y_hot: i32,
    ) -> i32;
    pub fn XSupportsLocale() -> Bool;
    pub fn XSetLocaleModifiers(modifier_list: *mut i8) -> *mut i8;
    pub fn XOpenOM(
        display: *mut Display,
        rdb: *mut _XrmHashBucketRec,
        res_name: *const i8,
        res_class: *const i8,
    ) -> XOM;
    pub fn XCloseOM(display: *mut Display, om: XOM) -> Status;
    pub fn XSetOMValues(om: XOM, ...) -> *mut i8;
    pub fn XGetOMValues(om: XOM, ...) -> *mut i8;
    pub fn XDisplayOfOM(om: XOM) -> *mut Display;
    pub fn XLocaleOfOM(om: XOM) -> *mut i8;
    pub fn XDestroyOC(oc: XOC);
    pub fn XOMOfOC(oc: XOC) -> XOM;
    pub fn XSetOCValues(oc: XOC, ...) -> *mut i8;
    pub fn XGetOCValues(oc: XOC, ...) -> *mut i8;
    pub fn XCreateFontSet(
        display: *mut Display,
        base_font_name_list: *const i8,
        missing_charset_list: *mut *mut *mut i8,
        missing_charset_count: *mut i32,
        def_string: *mut *mut i8,
    ) -> XFontSet;
    pub fn XFreeFontSet(display: *mut Display, font_set: XFontSet);
    pub fn XFontsOfFontSet(
        font_set: XFontSet,
        font_struct_list_return: *mut *mut XFontStruct,
        font_name_list_return: *mut *mut *mut i8,
    ) -> i32;
    pub fn XBaseFontNameListOfFontSet(font_set: XFontSet) -> *mut i8;
    pub fn XLocaleOfFontSet(font_set: XFontSet) -> *mut i8;
    pub fn XContextDependentDrawing(font_set: XFontSet) -> Bool;
    pub fn XDirectionalDependentDrawing(font_set: XFontSet) -> Bool;
    pub fn XContextualDrawing(font_set: XFontSet) -> Bool;
    pub fn XExtentsOfFontSet(font_set: XFontSet) -> *mut XFontSetExtents;
    pub fn XmbTextEscapement(font_set: XFontSet, text: *const i8, text_length: i32) -> i32;
    pub fn XwcTextEscapement(font_set: XFontSet, text: *const i16, text_length: i32) -> i32;
    pub fn Xutf8TextEscapement(font_set: XFontSet, text: *const i8, text_length: i32) -> i32;
    pub fn XmbTextExtents(
        font_set: XFontSet,
        text: *const i8,
        text_length: i32,
        overall_ink_return: *mut XRectangle,
        overall_logical_return: *mut XRectangle,
    );
    pub fn XwcTextExtents(
        font_set: XFontSet,
        text: *const i16,
        text_length: i32,
        overall_ink_return: *mut XRectangle,
        overall_logical_return: *mut XRectangle,
    );
    pub fn Xutf8TextExtents(
        font_set: XFontSet,
        text: *const i8,
        text_length: i32,
        overall_ink_return: *mut XRectangle,
        overall_logical_return: *mut XRectangle,
    );
    pub fn XmbTextPerCharExtents(
        font_set: XFontSet,
        text: *const i8,
        bytes_text: i32,
        ink_extents_buffer: *mut XRectangle,
        logical_extents_buffer: *mut XRectangle,
        buffer_size: i32,
        num_chars: *mut i32,
        overall_ink_return: *mut XRectangle,
        overall_logical_return: *mut XRectangle,
    ) -> Status;
    pub fn XwcTextPerCharExtents(
        font_set: XFontSet,
        text: *const i16,
        wchars_text: i32,
        ink_extents_buffer: *mut XRectangle,
        logical_extents_buffer: *mut XRectangle,
        buffer_size: i32,
        num_chars: *mut i32,
        overall_ink_return: *mut XRectangle,
        overall_logical_return: *mut XRectangle,
    ) -> Status;
    pub fn Xutf8TextPerCharExtents(
        font_set: XFontSet,
        text: *const i8,
        bytes_text: i32,
        ink_extents_buffer: *mut XRectangle,
        logical_extents_buffer: *mut XRectangle,
        buffer_size: i32,
        num_chars: *mut i32,
        overall_ink_return: *mut XRectangle,
        overall_logical_return: *mut XRectangle,
    ) -> Status;
    pub fn XmbDrawText(
        display: *mut Display,
        drawable: Drawable,
        gc: GC,
        x: i32,
        y: i32,
        text_items: *const XmbTextItem,
        text_length: i32,
    );
    pub fn XwcDrawText(
        display: *mut Display,
        drawable: Drawable,
        gc: GC,
        x: i32,
        y: i32,
        text_items: *const XwcTextItem,
        text_length: i32,
    );
    pub fn Xutf8DrawText(
        display: *mut Display,
        drawable: Drawable,
        gc: GC,
        x: i32,
        y: i32,
        text_items: *const XmbTextItem,
        text_length: i32,
    );
    pub fn XmbDrawString(
        display: *mut Display,
        drawable: Drawable,
        font_set: XFontSet,
        gc: GC,
        x: i32,
        y: i32,
        string: *const i8,
        length: i32,
    );
    pub fn XwcDrawString(
        display: *mut Display,
        drawable: Drawable,
        font_set: XFontSet,
        gc: GC,
        x: i32,
        y: i32,
        string: *const i16,
        length: i32,
    );
    pub fn Xutf8DrawString(
        display: *mut Display,
        drawable: Drawable,
        font_set: XFontSet,
        gc: GC,
        x: i32,
        y: i32,
        string: *const i8,
        length: i32,
    );
    pub fn XmbDrawImageString(
        display: *mut Display,
        drawable: Drawable,
        font_set: XFontSet,
        gc: GC,
        x: i32,
        y: i32,
        string: *const i8,
        length: i32,
    );
    pub fn XwcDrawImageString(
        display: *mut Display,
        drawable: Drawable,
        font_set: XFontSet,
        gc: GC,
        x: i32,
        y: i32,
        string: *const i16,
        length: i32,
    );
    pub fn Xutf8DrawImageString(
        display: *mut Display,
        drawable: Drawable,
        font_set: XFontSet,
        gc: GC,
        x: i32,
        y: i32,
        string: *const i8,
        length: i32,
    );
    pub fn XOpenIM(
        display: *mut Display,
        rdb: _XrmHashBucketRec,
        res_name: *const i8,
        res_class: *const i8,
    ) -> XIM;
    pub fn XCloseIM(im: XIM) -> Status;
    pub fn XGetIMValues(im: XIM, ...) -> *mut i8;
    pub fn XSetIMValues(im: XIM, ...) -> *mut i8;
    pub fn XDisplayOfIM(im: XIM) -> *mut Display;
    pub fn XLocaleOfIM(im: XIM) -> *mut i8;
    pub fn XCreateIC(im: XIM, ...) -> XIC;
    pub fn XDestroyIC(ic: XIC);
    pub fn XSetICFocus(ic: XIC);
    pub fn XUnsetICFocus(ic: XIC);
    pub fn XwcResetIC(ic: XIC) -> *mut i16;
    pub fn XmbResetIC(ic: XIC) -> *mut i8;
    pub fn Xutf8ResetIC(ic: XIC) -> *mut i8;
    pub fn XSetICValues(ic: XIC, ...) -> *mut i8;
    pub fn XGetICValues(ic: XIC, ...) -> *mut i8;
    pub fn XIMOfIC(ic: XIC) -> XIM;
    pub fn XFilterEvent(event: *mut XEvent, window: Window) -> Bool;
    pub fn XmbLookupString(
        ic: XIC,
        event: *mut XKeyPressedEvent,
        buffer_return: *mut i8,
        bytes_buffer: i32,
        keysym_return: *mut KeySym,
        status_return: *mut Status,
    ) -> i32;
    pub fn XwcLookupString(
        ic: XIC,
        event: *mut XKeyPressedEvent,
        buffer_return: *mut i16,
        wchars_buffer: i32,
        keysym_return: *mut KeySym,
        status_return: *mut Status,
    ) -> i32;
    pub fn Xutf8LookupString(
        ic: XIC,
        event: *mut XKeyPressedEvent,
        buffer_return: *mut i8,
        bytes_buffer: i32,
        keysym_return: *mut KeySym,
        status_return: *mut Status,
    ) -> i32;
    pub fn XVaCreateNestedList(unused: i32, ...) -> XVaNestedList;
    pub fn XRegisterIMInstantiateCallback(
        display: *mut Display,
        rdb: _XrmHashBucketRec,
        res_name: *const i8,
        res_class: *const i8,
        call_back: XIMProc,
        client_data: XPointer,
    ) -> Status;
    pub fn XUnregisterIMInstantiateCallback(
        display: *mut Display,
        rdb: _XrmHashBucketRec,
        res_name: *const i8,
        res_class: *const i8,
        call_back: XIMProc,
        client_data: XPointer,
    ) -> Status;
}

pub type XConnectionWatchProc = *mut fn(
    display: *mut Display,
    client_data: XPointer,
    fd: i32,
    opening: Bool,
    watch_data: *mut XPointer,
);

#[link(name = "X11", kind = "dylib")]
extern "C" {
    pub fn XInternalConnectionNumbers(
        display: *mut Display,
        fd_return: *mut *mut i32,
        count_return: *mut i32,
    ) -> Status;
    pub fn XProcessInternalConnection(display: *mut Display, fd: i32);
    pub fn XAddConnectionWatch(
        display: *mut Display,
        call_back: XConnectionWatchProc,
        client_data: XPointer,
    ) -> Status;
    pub fn XRemoveConnectionWatch(
        display: *mut Display,
        call_back: XConnectionWatchProc,
        client_data: XPointer,
    ) -> Status;
    pub fn XSetAuthorization(
        name: *mut i8,
        name_length: i32,
        data: *mut i8,
        data_length: i32,
    ) -> Status;
    pub fn _Xmbtowc(wstr: *mut i16, str: *mut i8, len: i32) -> i32;
    pub fn _Xwctomb(str: *mut i8, wstr: *mut i16, len: i32) -> i32;
    pub fn XGetEventData(display: *mut Display, cookie: XGenericEventCookie) -> Bool;
    pub fn XFreeEventData(display: *mut Display, cookie: XGenericEventCookie);

}
