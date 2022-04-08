pub const ParentRelative: u64 = 1;
pub const CopyFromParent: u64 = 0;

pub const PointerWindow: u64 = 0;
pub const InputFocus: u64 = 1;

pub const PointerRoot: u64 = 1;

pub const AnyPropertyType: u64 = 0;

pub const AnyKey: u64 = 0;
pub const AnyButton: u64 = 0;
pub const AllTemporary: u64 = 0;
pub const CurrentTime: u64 = 0;
pub const NoSymbol: u64 = 0;

pub mod EventMasks {
    pub const NoEventMask: u64 = 0;
    pub const KeyPressMask: u64 = 1 << 0;
    pub const KeyReleaseMask: u64 = 1 << 1;
    pub const ButtonPressMask: u64 = 1 << 2;
    pub const ButtonReleaseMask: u64 = 1 << 3;
    pub const EnterWindowMask: u64 = 1 << 4;
    pub const LeaveWindowMask: u64 = 1 << 5;
    pub const PointerMotionMask: u64 = 1 << 6;
    pub const PointerMotionHintMask: u64 = 1 << 7;
    pub const Button1MotionMask: u64 = 1 << 8;
    pub const Button2MotionMask: u64 = 1 << 9;
    pub const Button3MotionMask: u64 = 1 << 10;
    pub const Button4MotionMask: u64 = 1 << 11;
    pub const Button5MotionMask: u64 = 1 << 12;
    pub const ButtonMotionMask: u64 = 1 << 13;
    pub const KeymapStateMask: u64 = 1 << 14;
    pub const ExposureMask: u64 = 1 << 15;
    pub const VisibilityChangeMask: u64 = 1 << 16;
    pub const StructureNotifyMask: u64 = 1 << 17;
    pub const ResizeRedirectMask: u64 = 1 << 18;
    pub const SubstructureNotifyMask: u64 = 1 << 19;
    pub const SubstructureRedirectMask: u64 = 1 << 20;
    pub const FocusChangeMask: u64 = 1 << 21;
    pub const PropertyChangeMask: u64 = 1 << 22;
    pub const ColormapChangeMask: u64 = 1 << 23;
    pub const OwnerGrabButtonMask: u64 = 1 << 24;
}

pub mod Events {
    pub const KeyPress: u32 = 2;
    pub const KeyRelease: u32 = 3;
    pub const ButtonPress: u32 = 4;
    pub const ButtonRelease: u32 = 5;
    pub const MotionNotify: u32 = 6;
    pub const EnterNotify: u32 = 7;
    pub const LeaveNotify: u32 = 8;
    pub const FocusIn: u32 = 9;
    pub const FocusOut: u32 = 10;
    pub const KeymapNotify: u32 = 11;
    pub const Expose: u32 = 12;
    pub const GraphicsExpose: u32 = 13;
    pub const NoExpose: u32 = 14;
    pub const VisibilityNotify: u32 = 15;
    pub const CreateNotify: u32 = 16;
    pub const DestroyNotify: u32 = 17;
    pub const UnmapNotify: u32 = 18;
    pub const MapNotify: u32 = 19;
    pub const MapRequest: u32 = 20;
    pub const ReparentNotify: u32 = 21;
    pub const ConfigureNotify: u32 = 22;
    pub const ConfigureRequest: u32 = 23;
    pub const GravityNotify: u32 = 24;
    pub const ResizeRequest: u32 = 25;
    pub const CirculateNotify: u32 = 26;
    pub const CirculateRequest: u32 = 27;
    pub const PropertyNotify: u32 = 28;
    pub const SelectionClear: u32 = 29;
    pub const SelectionRequest: u32 = 30;
    pub const SelectionNotify: u32 = 31;
    pub const ColormapNotify: u32 = 32;
    pub const ClientMessage: u32 = 33;
    pub const MappingNotify: u32 = 34;
    pub const GenericEvent: u32 = 35;
    pub const LASTEvent: u32 = 36;
}

pub mod Modifiers {
    pub const ShiftMask: u32 = 1 << 0;
    pub const LockMask: u32 = 1 << 1;
    pub const ControlMask: u32 = 1 << 2;
    pub const Mod1Mask: u32 = 1 << 3;
    pub const Mod2Mask: u32 = 1 << 4;
    pub const Mod3Mask: u32 = 1 << 5;
    pub const Mod4Mask: u32 = 1 << 6;
    pub const Mod5Mask: u32 = 1 << 7;
}

pub mod ModifierMaping {
    pub const ShiftMapIndex: u32 = 0;
    pub const LockMapIndex: u32 = 1;
    pub const ControlMapIndex: u32 = 2;
    pub const Mod1MapIndex: u32 = 3;
    pub const Mod2MapIndex: u32 = 4;
    pub const Mod3MapIndex: u32 = 5;
    pub const Mod4MapIndex: u32 = 6;
    pub const Mod5MapIndex: u32 = 7;
}

pub mod ButtonMasks {
    pub const Button1Mask: u32 = 1 << 8;
    pub const Button2Mask: u32 = 1 << 9;
    pub const Button3Mask: u32 = 1 << 10;
    pub const Button4Mask: u32 = 1 << 11;
    pub const Button5Mask: u32 = 1 << 12;
}

pub const AnyModifier: u32 = 1 << 15;

pub mod Buttons {
    pub const Button1: u32 = 1;
    pub const Button2: u32 = 2;
    pub const Button3: u32 = 3;
    pub const Button4: u32 = 4;
    pub const Button5: u32 = 5;
}

pub mod Notify {
    pub const Normal: u32 = 0;
    pub const Grab: u32 = 1;
    pub const Ungrab: u32 = 2;
    pub const WhileGrabbed: u32 = 3;

    pub const Hint: u32 = 1;

    pub const Ancestor: u32 = 0;
    pub const Virtual: u32 = 1;
    pub const Inferior: u32 = 2;
    pub const Nonlinear: u32 = 3;
    pub const NonlinearVirtual: u32 = 4;
    pub const Pointer: u32 = 5;
    pub const PointerRoot: u32 = 6;
    pub const DetailNone: u32 = 7;

    pub const VisibilityUnobscured: u32 = 0;
    pub const VisibilityPartiallyObscured: u32 = 1;
    pub const VisibilityFullyObscured: u32 = 2;
}

pub const PlaceOnTop: u32 = 0;
pub const PlaceOnBottom: u32 = 1;

pub const FamilyInternet: u32 = 0;
pub const FamilyDECnet: u32 = 1;
pub const FamilyChaos: u32 = 2;
pub const FamilyInternet6: u32 = 6;

pub const FamilyServerInterpreted: u32 = 5;

pub const PropertyNewValue: u32 = 0;
pub const PropertyDelete: u32 = 1;

pub enum GrabMode {
    GrabModeSync = 0,
    GrabModeAsync = 1,
}

pub enum GrabError {
    AlreadyGrabbed = 1,
    InvalidTime = 2,
    NotViewable = 3,
    Frozen = 4,
}

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

pub enum Revert {
    ToNone = 0,
    ToPointerRoot = Notify::PointerRoot as isize,
    ToParent = 2,
}

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

pub const InputOutput: u32 = 1;
pub const InputOnly: u32 = 2;

pub mod WindowAttributes {
    pub const BackPixmap: u64 = 1 << 0;
    pub const BackPixel: u64 = 1 << 1;
    pub const BorderPixmap: u64 = 1 << 2;
    pub const BorderPixel: u64 = 1 << 3;
    pub const BitGravity: u64 = 1 << 4;
    pub const WinGravity: u64 = 1 << 5;
    pub const BackingStore: u64 = 1 << 6;
    pub const BackingPlanes: u64 = 1 << 7;
    pub const BackingPixel: u64 = 1 << 8;
    pub const OverrideRedirect: u64 = 1 << 9;
    pub const SaveUnder: u64 = 1 << 10;
    pub const EventMask: u64 = 1 << 11;
    pub const DontPropagate: u64 = 1 << 12;
    pub const ColorMap: u64 = 1 << 13;
    pub const Cursor: u64 = 1 << 14;
}

pub mod ConfigureWindow {
    pub const X: u32 = 1 << 0;
    pub const Y: u32 = 1 << 1;
    pub const Width: u32 = 1 << 2;
    pub const Height: u32 = 1 << 3;
    pub const BorderWidth: u32 = 1 << 4;
    pub const Sibling: u32 = 1 << 5;
    pub const StackMode: u32 = 1 << 6;
}

pub mod Gravity {
    pub const Forget: u32 = 0;
    pub const NorthWest: u32 = 1;
    pub const North: u32 = 2;
    pub const NorthEast: u32 = 3;
    pub const West: u32 = 4;
    pub const Center: u32 = 5;
    pub const East: u32 = 6;
    pub const SouthWest: u32 = 7;
    pub const South: u32 = 8;
    pub const SouthEast: u32 = 9;
    pub const Static: u32 = 10;
}

pub const NotUseful: u32 = 0;
pub const WhenMapped: u32 = 1;
pub const Always: u32 = 2;

pub const IsUnmapped: u32 = 0;
pub const IsUnviewable: u32 = 1;
pub const IsViewable: u32 = 2;

pub const SetModeInsert: u32 = 0;
pub const SetModeDelete: u32 = 1;

pub const DestroyAll: u32 = 0;
pub const RetainPermanent: u32 = 1;
pub const RetainTemporary: u32 = 2;

pub const Above: u32 = 0;
pub const Below: u32 = 1;
pub const TopIf: u32 = 2;
pub const BottomIf: u32 = 3;
pub const Opposite: u32 = 4;
