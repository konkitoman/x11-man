pub const NoEventMask: libc::c_long = 0;
pub const KeyPressMask: libc::c_long = 1;
pub const KeyReleaseMask: libc::c_long = 2;
pub const ButtonPressMask: libc::c_long = 4;
pub const ButtonReleaseMask: libc::c_long = 8;
pub const EnterWindowMask: libc::c_long = 16;
pub const LeaveWindowMask: libc::c_long = 32;
pub const PointerMotionMask: libc::c_long = 64;
pub const PointerMotionHintMask: libc::c_long = 128;
pub const Button1MotionMask: libc::c_long = 256;
pub const Button2MotionMask: libc::c_long = 512;
pub const Button3MotionMask: libc::c_long = 1024;
pub const Button4MotionMask: libc::c_long = 2048;
pub const Button5MotionMask: libc::c_long = 4096;
pub const ButtonMotionMask: libc::c_long = 8192;
pub const KeymapStateMask: libc::c_long = 16384;
pub const ExposureMask: libc::c_long = 32768;
pub const VisibilityChangeMask: libc::c_long = 65536;
pub const StructureNotifyMask: libc::c_long = 131072;
pub const ResizeRedirectMask: libc::c_long = 262144;
pub const SubstructureNotifyMask: libc::c_long = 524288;
pub const SubstructureRedirectMask: libc::c_long = 1048576;
pub const FocusChangeMask: libc::c_long = 2097152;
pub const PropertyChangeMask: libc::c_long = 4194304;
pub const ColormapChangeMask: libc::c_long = 8388608;
pub const OwnerGrabButtonMask: libc::c_long = 16777216;

pub const KeyPress: libc::c_int = 2;
pub const KeyRelease: libc::c_int = 3;
pub const ButtonPress: libc::c_int = 4;
pub const ButtonRelease: libc::c_int = 5;
pub const MotionNotify: libc::c_int = 6;
pub const EnterNotify: libc::c_int = 7;
pub const LeaveNotify: libc::c_int = 8;
pub const FocusIn: libc::c_int = 9;
pub const FocusOut: libc::c_int = 10;
pub const KeymapNotify: libc::c_int = 11;
pub const Expose: libc::c_int = 12;
pub const GraphicsExpose: libc::c_int = 13;
pub const NoExpose: libc::c_int = 14;
pub const VisibilityNotify: libc::c_int = 15;
pub const CreateNotify: libc::c_int = 16;
pub const DestroyNotify: libc::c_int = 17;
pub const UnmapNotify: libc::c_int = 18;
pub const MapNotify: libc::c_int = 19;
pub const MapRequest: libc::c_int = 20;
pub const ReparentNotify: libc::c_int = 21;
pub const ConfigureNotify: libc::c_int = 22;
pub const ConfigureRequest: libc::c_int = 23;
pub const GravityNotify: libc::c_int = 24;
pub const ResizeRequest: libc::c_int = 25;
pub const CirculateNotify: libc::c_int = 26;
pub const CirculateRequest: libc::c_int = 27;
pub const PropertyNotify: libc::c_int = 28;
pub const SelectionClear: libc::c_int = 29;
pub const SelectionRequest: libc::c_int = 30;
pub const SelectionNotify: libc::c_int = 31;
pub const ColormapNotify: libc::c_int = 32;
pub const ClientMessage: libc::c_int = 33;
pub const MappingNotify: libc::c_int = 34;
pub const GenericEvent: libc::c_int = 35;
pub const LASTEvent: libc::c_int = 36;

pub const ShiftMask: libc::c_int = 1;
pub const LockMask: libc::c_int = 2;
pub const ControlMask: libc::c_int = 4;
pub const Mod1Mask: libc::c_int = 8;
pub const Mod2Mask: libc::c_int = 16;
pub const Mod3Mask: libc::c_int = 32;
pub const Mod4Mask: libc::c_int = 64;
pub const Mod5Mask: libc::c_int = 128;

pub const Button1Mask: libc::c_int = 256;
pub const Button2Mask: libc::c_int = 512;
pub const Button3Mask: libc::c_int = 1024;
pub const Button4Mask: libc::c_int = 2048;
pub const Button5Mask: libc::c_int = 4096;
pub const AnyModifier: libc::c_int = 32768;

pub const Button1: libc::c_uint = 1;
pub const Button2: libc::c_uint = 2;
pub const Button3: libc::c_uint = 3;
pub const Button4: libc::c_uint = 4;
pub const Button5: libc::c_uint = 5;
pub const AnyButton: libc::c_uint = 0;

pub const DvAccelNum: libc::c_long = 1;
pub const DvAccelDenom: libc::c_long = 2;
pub const DvThreshold: libc::c_long = 4;

pub const DvKeyClickPrecent: libc::c_long = 1;
pub const DvPrecent: libc::c_long = 2;
pub const DvPitch: libc::c_long = 4;
pub const DvDuration: libc::c_long = 8;
pub const DvLed: libc::c_long = 16;
pub const DvLedMode: libc::c_long = 32;
pub const DvKey: libc::c_long = 64;
pub const DvAutoRepeatMode: libc::c_long = 128;

pub const DvString: libc::c_long = 1;

pub const DvIntiger: libc::c_long = 1;

pub const DeviceMode: libc::c_long = 1;
pub const Relative: libc::c_int = 0;
pub const Absolute: libc::c_int = 1;

pub const ProximityState: libc::c_long = 2;
pub const InProximity: libc::c_long = 0;
pub const OutOfProximity: libc::c_long = 2;

pub const AddToList: libc::c_int = 0;
pub const DeleteFromList: libc::c_int = 1;

pub const KeyClass: libc::c_int = 0;
pub const ButtonClass: libc::c_int = 1;
pub const ValuatorClass: libc::c_int = 2;
pub const FeedbackClass: libc::c_int = 3;
pub const ProximityClass: libc::c_int = 4;
pub const FocusClass: libc::c_int = 5;
pub const OtherClass: libc::c_int = 6;
pub const AttachClass: libc::c_int = 7;

pub const _devicePointerMonitorHint: libc::c_int = 0;
pub const _deviceButton1Motion: libc::c_int = 1;
pub const _deviceButton2Motion: libc::c_int = 2;
pub const _deviceButton3Motion: libc::c_int = 3;
pub const _deviceButton4Motion: libc::c_int = 4;
pub const _deviceButton5Motion: libc::c_int = 5;
pub const _deviceButtonMotion: libc::c_int = 6;
pub const _deviceButtonGrab: libc::c_int = 7;
pub const _deviceOwnerGrabButton: libc::c_int = 8;
pub const _noExtensionEvent: libc::c_int = 9;

pub const _devicePresence: libc::c_int = 0;

pub const _deviceEnter: libc::c_int = 0;
pub const _deviceLeave: libc::c_int = 1;

pub const DeviceAdded: libc::c_int = 0;
pub const DeviceRemoved: libc::c_int = 1;
pub const DeviceEnabled: libc::c_int = 2;
pub const DeviceDisabled: libc::c_int = 3;
pub const DeviceUnrecoverable: libc::c_int = 4;
pub const DeviceControlChanged: libc::c_int = 5;

pub const XI_BadDevice: libc::c_int = 0;
pub const XI_BadEvent: libc::c_int = 1;
pub const XI_BadMode: libc::c_int = 2;
pub const XI_DeviceBusy: libc::c_int = 3;
pub const XI_BadClass: libc::c_int = 4;

pub const _deviceKeyPress: libc::c_int = 0;
pub const _deviceKeyRelease: libc::c_int = 1;

pub const _deviceButtonPress: libc::c_int = 0;
pub const _deviceButtonRelease: libc::c_int = 1;
