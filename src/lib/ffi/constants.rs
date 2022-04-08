pub const NoEventMask: i64 = 0;
pub const KeyPressMask: i64 = 1;
pub const KeyReleaseMask: i64 = 2;
pub const ButtonPressMask: i64 = 4;
pub const ButtonReleaseMask: i64 = 8;
pub const EnterWindowMask: i64 = 16;
pub const LeaveWindowMask: i64 = 32;
pub const PointerMotionMask: i64 = 64;
pub const PointerMotionHintMask: i64 = 128;
pub const Button1MotionMask: i64 = 256;
pub const Button2MotionMask: i64 = 512;
pub const Button3MotionMask: i64 = 1024;
pub const Button4MotionMask: i64 = 2048;
pub const Button5MotionMask: i64 = 4096;
pub const ButtonMotionMask: i64 = 8192;
pub const KeymapStateMask: i64 = 16384;
pub const ExposureMask: i64 = 32768;
pub const VisibilityChangeMask: i64 = 65536;
pub const StructureNotifyMask: i64 = 131072;
pub const ResizeRedirectMask: i64 = 262144;
pub const SubstructureNotifyMask: i64 = 524288;
pub const SubstructureRedirectMask: i64 = 1048576;
pub const FocusChangeMask: i64 = 2097152;
pub const PropertyChangeMask: i64 = 4194304;
pub const ColormapChangeMask: i64 = 8388608;
pub const OwnerGrabButtonMask: i64 = 16777216;

pub const KeyPress: i32 = 2;
pub const KeyRelease: i32 = 3;
pub const ButtonPress: i32 = 4;
pub const ButtonRelease: i32 = 5;
pub const MotionNotify: i32 = 6;
pub const EnterNotify: i32 = 7;
pub const LeaveNotify: i32 = 8;
pub const FocusIn: i32 = 9;
pub const FocusOut: i32 = 10;
pub const KeymapNotify: i32 = 11;
pub const Expose: i32 = 12;
pub const GraphicsExpose: i32 = 13;
pub const NoExpose: i32 = 14;
pub const VisibilityNotify: i32 = 15;
pub const CreateNotify: i32 = 16;
pub const DestroyNotify: i32 = 17;
pub const UnmapNotify: i32 = 18;
pub const MapNotify: i32 = 19;
pub const MapRequest: i32 = 20;
pub const ReparentNotify: i32 = 21;
pub const ConfigureNotify: i32 = 22;
pub const ConfigureRequest: i32 = 23;
pub const GravityNotify: i32 = 24;
pub const ResizeRequest: i32 = 25;
pub const CirculateNotify: i32 = 26;
pub const CirculateRequest: i32 = 27;
pub const PropertyNotify: i32 = 28;
pub const SelectionClear: i32 = 29;
pub const SelectionRequest: i32 = 30;
pub const SelectionNotify: i32 = 31;
pub const ColormapNotify: i32 = 32;
pub const ClientMessage: i32 = 33;
pub const MappingNotify: i32 = 34;
pub const GenericEvent: i32 = 35;
pub const LASTEvent: i32 = 36;

pub const ShiftMask: i32 = 1;
pub const LockMask: i32 = 2;
pub const ControlMask: i32 = 4;
pub const Mod1Mask: i32 = 8;
pub const Mod2Mask: i32 = 16;
pub const Mod3Mask: i32 = 32;
pub const Mod4Mask: i32 = 64;
pub const Mod5Mask: i32 = 128;

pub const Button1Mask: i32 = 256;
pub const Button2Mask: i32 = 512;
pub const Button3Mask: i32 = 1024;
pub const Button4Mask: i32 = 2048;
pub const Button5Mask: i32 = 4096;
pub const AnyModifier: i32 = 32768;

pub const Button1: u32 = 1;
pub const Button2: u32 = 2;
pub const Button3: u32 = 3;
pub const Button4: u32 = 4;
pub const Button5: u32 = 5;
pub const AnyButton: u32 = 0;

pub const DvAccelNum: i64 = 1;
pub const DvAccelDenom: i64 = 2;
pub const DvThreshold: i64 = 4;

pub const DvKeyClickPrecent: i64 = 1;
pub const DvPrecent: i64 = 2;
pub const DvPitch: i64 = 4;
pub const DvDuration: i64 = 8;
pub const DvLed: i64 = 16;
pub const DvLedMode: i64 = 32;
pub const DvKey: i64 = 64;
pub const DvAutoRepeatMode: i64 = 128;

pub const DvString: i64 = 1;

pub const DvIntiger: i64 = 1;

pub const DeviceMode: i64 = 1;
pub const Relative: i32 = 0;
pub const Absolute: i32 = 1;

pub const ProximityState: i64 = 2;
pub const InProximity: i64 = 0;
pub const OutOfProximity: i64 = 2;

pub const AddToList: i32 = 0;
pub const DeleteFromList: i32 = 1;

pub const KeyClass: i32 = 0;
pub const ButtonClass: i32 = 1;
pub const ValuatorClass: i32 = 2;
pub const FeedbackClass: i32 = 3;
pub const ProximityClass: i32 = 4;
pub const FocusClass: i32 = 5;
pub const OtherClass: i32 = 6;
pub const AttachClass: i32 = 7;

pub const _devicePointerMonitorHint: i32 = 0;
pub const _deviceButton1Motion: i32 = 1;
pub const _deviceButton2Motion: i32 = 2;
pub const _deviceButton3Motion: i32 = 3;
pub const _deviceButton4Motion: i32 = 4;
pub const _deviceButton5Motion: i32 = 5;
pub const _deviceButtonMotion: i32 = 6;
pub const _deviceButtonGrab: i32 = 7;
pub const _deviceOwnerGrabButton: i32 = 8;
pub const _noExtensionEvent: i32 = 9;

pub const _devicePresence: i32 = 0;

pub const _deviceEnter: i32 = 0;
pub const _deviceLeave: i32 = 1;

pub const DeviceAdded: i32 = 0;
pub const DeviceRemoved: i32 = 1;
pub const DeviceEnabled: i32 = 2;
pub const DeviceDisabled: i32 = 3;
pub const DeviceUnrecoverable: i32 = 4;
pub const DeviceControlChanged: i32 = 5;

pub const XI_BadDevice: i32 = 0;
pub const XI_BadEvent: i32 = 1;
pub const XI_BadMode: i32 = 2;
pub const XI_DeviceBusy: i32 = 3;
pub const XI_BadClass: i32 = 4;

pub const _deviceKeyPress: i32 = 0;
pub const _deviceKeyRelease: i32 = 1;

pub const _deviceButtonPress: i32 = 0;
pub const _deviceButtonRelease: i32 = 1;

pub const PointerRoot: i64 = 1;

pub const GrabModeSync: i32 = 0;
pub const GrabModeAsync: i32 = 1;

pub const GrabSuccess: i32 = 0;
pub const AlreadyGrabbed: i32 = 1;
pub const GrabInvalidTime: i32 = 2;
pub const GrabNotViewable: i32 = 3;
pub const GrabFrozen: i32 = 4;

pub const AsyncPointer: i32 = 0;
pub const SyncPointer: i32 = 1;
pub const ReplayPointer: i32 = 2;
pub const AsyncKeyboard: i32 = 3;
pub const SyncKeyboard: i32 = 4;
pub const ReplayKeyboard: i32 = 5;
pub const AsyncBoth: i32 = 6;
pub const SyncBoth: i32 = 7;
