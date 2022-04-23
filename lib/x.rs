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

pub struct PointerGrabConfig {
    pub button_press: bool,
    pub button_release: bool,
    pub button1: bool,
    pub button2: bool,
    pub button3: bool,
    pub button4: bool,
    pub button5: bool,
    pub pointer_motion: bool,
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
