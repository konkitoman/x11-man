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
