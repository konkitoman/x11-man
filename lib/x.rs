use crate::ffi::x;

#[derive(Debug, Copy, Clone)]
pub enum GrabMode {
    Sync = 0,
    Async = 1,
}

#[derive(Debug, Copy, Clone)]
pub enum GrabError {
    AlreadyGrabbed = 1,
    InvalidTime = 2,
    NotViewable = 3,
    Frozen = 4,
}

#[derive(Debug, Copy, Clone)]
pub enum AllowEvents {
    AsyncPointer = 0,
    SyncPointer = 1,
    ReplayPointer = 2,
    AsyncKeyboard = 3,
    SyncKeyboard = 4,
    ReplayKeyboard = 5,
    AsyncBoth = 6,
    SyncBoth = 7,
}

#[derive(Debug, Copy, Clone)]
pub enum Revert {
    ToNone = 0,
    ToPointerRoot = 6 as isize,
    ToParent = 2,
}

#[derive(Debug, Copy, Clone)]
pub enum Errors {
    Success = 0,
    BadRequest = 1,
    BadValue = 2,
    BadWindow = 3,
    BadPixmap = 4,
    BadAtom = 5,
    BadCursor = 6,
    BadFont = 7,
    BadMatch = 8,
    BadDrawable = 9,
    BadAccess = 10,
    BadAlloc = 11,
    BadColor = 12,
    BadGC = 13,
    BadIDChoice = 14,
    BadName = 15,
    BadLength = 16,
    BadImplementation = 17,

    FirstExtensionError = 128,
    LastExtensionError = 255,
}

pub struct SelectInputConfig {}

impl Default for SelectInputConfig {
    fn default() -> Self {
        SelectInputConfig {}
    }
}

impl SelectInputConfig {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get_row(&self) -> i64 {
        0
    }
}

pub struct KeyGrabConfig {
    pub keycode: i32,
    pub modifiers: u32,

    pub owner_events: bool,
    pub keyboard_grab_mode: GrabMode,
    pub pointer_grab_mode: GrabMode,
}

impl Default for KeyGrabConfig {
    fn default() -> Self {
        KeyGrabConfig {
            keycode: 0,
            modifiers: 0,

            owner_events: true,
            keyboard_grab_mode: GrabMode::Async,
            pointer_grab_mode: GrabMode::Async,
        }
    }
}

impl KeyGrabConfig {
    pub fn new() -> Self {
        Default::default()
    }
}

pub struct KeyboardGrabConfig {
    pub owner_events: bool,
    pub keyboard_grab_mode: GrabMode,
    pub pointer_grab_mode: GrabMode,
    pub time: u64,
}

impl Default for KeyboardGrabConfig {
    fn default() -> Self {
        KeyboardGrabConfig {
            owner_events: true,
            keyboard_grab_mode: GrabMode::Async,
            pointer_grab_mode: GrabMode::Async,
            time: 0,
        }
    }
}

impl KeyboardGrabConfig {
    pub fn new() -> Self {
        Default::default()
    }
}

pub struct ButtonGrabConfig {
    pub button: u32,
    pub modifiers: u32,
    pub button_release: bool,
    pub pointer_motion: bool,

    pub confine_to: u64,
    pub cursor: u64,

    pub owner_events: bool,
    pub pointer_grab_mode: GrabMode,
    pub keyboard_grab_mode: GrabMode,
}

impl Default for ButtonGrabConfig {
    fn default() -> Self {
        Self {
            button: 0,
            modifiers: 0,
            button_release: false,
            pointer_motion: false,

            confine_to: 0,
            cursor: 0,

            owner_events: true,
            pointer_grab_mode: GrabMode::Async,
            keyboard_grab_mode: GrabMode::Async,
        }
    }
}

impl ButtonGrabConfig {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the button to grab.
    /// If is 0, is grab all buttons.
    pub fn set_button(&mut self, button: u32) -> &mut Self {
        self.button = button;
        self
    }

    /// S = Shift, C = Ctrl, L = Lock, M = Tux/Win, A = Alt, N = Numpad, \[ = Mod3, \] = Mod5
    ///
    /// Ctrl + Shift = `"C-S"`
    ///
    /// Alt + Shift = `"A-S"`
    pub fn set_modifiers(&mut self, modifiers: &str) -> &mut Self {
        let mut mods = 0u32;
        for c in modifiers.chars() {
            match c {
                'S' => mods |= x::ShiftMask as u32,
                'C' => mods |= x::ControlMask as u32,
                'L' => mods |= x::LockMask as u32,
                'M' => mods |= x::Mod4Mask as u32,
                'A' => mods |= x::Mod1Mask as u32,
                'N' => mods |= x::Mod2Mask as u32,
                '[' => mods |= x::Mod3Mask as u32,
                ']' => mods |= x::Mod5Mask as u32,
                _ => {}
            }
        }
        self.modifiers = mods;
        self
    }

    /// Set if add the button release.event
    pub fn button_release(&mut self, value: bool) -> &mut Self {
        self.button_release = value;
        self
    }

    /// Set if add the pointer motion.event when the button is pressed
    pub fn pointer_motion(&mut self, value: bool) -> &mut Self {
        self.pointer_motion = value;
        self
    }

    /// Set all events when button is pressed
    pub fn all(&mut self) -> &mut Self {
        self.button_release = true;
        self.pointer_motion = true;
        self
    }

    pub fn as_mask(&self) -> u32 {
        let mut mask = 0u32;
        if self.button_release {
            mask |= x::ButtonReleaseMask as u32;
        }
        if self.pointer_motion {
            mask |= x::PointerMotionMask as u32;
        }
        mask
    }
}

pub struct PointerGrabConfig {
    pub button_press: bool,
    pub button_release: bool,
    pub button1: bool,
    pub button2: bool,
    pub button3: bool,
    pub button4: bool,
    pub button5: bool,
    pub pointer_motion: bool,

    pub confine_to: u64,
    pub cursor: u64,

    pub owner_events: bool,
    pub pointer_grab_mode: GrabMode,
    pub keyboard_grab_mode: GrabMode,
    pub time: u64,
}

impl Default for PointerGrabConfig {
    fn default() -> Self {
        Self {
            button_press: false,
            button_release: false,
            button1: false,
            button2: false,
            button3: false,
            button4: false,
            button5: false,
            pointer_motion: false,

            confine_to: 0,
            cursor: 0,

            owner_events: true,
            pointer_grab_mode: GrabMode::Async,
            keyboard_grab_mode: GrabMode::Async,
            time: 0,
        }
    }
}

impl PointerGrabConfig {
    pub fn new() -> Self {
        Self::default()
    }

    /// any button press
    /// event `ButtonPress`
    pub fn button_press(&mut self, value: bool) -> &mut Self {
        self.button_press = value;
        self
    }

    /// any button release
    /// event `ButtonRelease`
    pub fn button_release(&mut self, value: bool) -> &mut Self {
        self.button_release = value;
        self
    }

    /// is only for motion and button 1-5
    /// 1 = left, 2 = middle, 3 = right
    /// event `MotionNotify`
    pub fn button(&mut self, button: u8, value: bool) -> &mut Self {
        match button {
            1 => self.button1 = value,
            2 => self.button2 = value,
            3 => self.button3 = value,
            4 => self.button4 = value,
            5 => self.button5 = value,
            _ => {}
        }
        self
    }

    /// motion
    /// event `MotionNotify`
    pub fn pointer_motion(&mut self, value: bool) -> &mut Self {
        self.pointer_motion = value;
        self
    }

    /// geting all button press as release, motion
    /// events `ButtonPress`, `ButtonRelease`, `MotionNotify`
    pub fn all(&mut self) -> &mut Self {
        self.button_press = true;
        self.button_release = true;
        self.pointer_motion = true;

        self
    }

    /// is used internally
    pub fn as_mask(&self) -> u32 {
        let mut mask = 0;

        if self.button_press {
            mask |= x::ButtonPressMask as u32;
        }
        if self.button_release {
            mask |= x::ButtonReleaseMask as u32;
        }
        if self.button1 {
            mask |= x::Button1Mask as u32;
        }
        if self.button2 {
            mask |= x::Button2Mask as u32;
        }
        if self.button3 {
            mask |= x::Button3Mask as u32;
        }
        if self.button4 {
            mask |= x::Button4Mask as u32;
        }
        if self.button5 {
            mask |= x::Button5Mask as u32;
        }
        if self.pointer_motion {
            mask |= x::PointerMotionMask as u32;
        }

        mask
    }
}
