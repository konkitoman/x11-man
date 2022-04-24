pub const X_PROTOCOL: u32 = 11;
pub const X_PROTOCOL_REVISION: u32 = 0;

pub type XID = u64;
pub type Mask = u64;
pub type Atom = u64;
pub type VisualID = u64;
pub type Time = u64;

pub type Window = XID;
pub type Drawable = XID;
pub type Font = XID;
pub type Pixmap = XID;
pub type Cursor = XID;
pub type Colormap = XID;
pub type GContext = XID;
pub type KeySym = XID;

pub type KeyCode = u8;

pub const None: u64 = 0;
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

pub const ShiftMask: u32 = 1 << 0;
pub const LockMask: u32 = 1 << 1;
pub const ControlMask: u32 = 1 << 2;
/// Alt
pub const Mod1Mask: u32 = 1 << 3;
/// numlock
pub const Mod2Mask: u32 = 1 << 4;
pub const Mod3Mask: u32 = 1 << 5;
/// Tux/Windows key
pub const Mod4Mask: u32 = 1 << 6;
pub const Mod5Mask: u32 = 1 << 7;

pub const ShiftMapIndex: u32 = 0;
pub const LockMapIndex: u32 = 1;
pub const ControlMapIndex: u32 = 2;
pub const Mod1MapIndex: u32 = 3;
pub const Mod2MapIndex: u32 = 4;
pub const Mod3MapIndex: u32 = 5;
pub const Mod4MapIndex: u32 = 6;
pub const Mod5MapIndex: u32 = 7;

pub const Button1Mask: u32 = 1 << 8;
pub const Button2Mask: u32 = 1 << 9;
pub const Button3Mask: u32 = 1 << 10;
pub const Button4Mask: u32 = 1 << 11;
pub const Button5Mask: u32 = 1 << 12;
pub const AnyModifier: u32 = 1 << 15;

pub const Button1: u32 = 1;
pub const Button2: u32 = 2;
pub const Button3: u32 = 3;
pub const Button4: u32 = 4;
pub const Button5: u32 = 5;

pub const NotifyNormal: u32 = 0;
pub const NotifyGrab: u32 = 1;
pub const NotifyUngrab: u32 = 2;
pub const NotifyWhileGrabbed: u32 = 3;
pub const NotifyHint: u32 = 1;

pub const NotifyAncestor: u32 = 0;
pub const NotifyVirtual: u32 = 1;
pub const NotifyInferior: u32 = 2;
pub const NotifyNonlinear: u32 = 3;
pub const NotifyNonlinearVirtual: u32 = 4;
pub const NotifyPointer: u32 = 5;
pub const NotifyPointerRoot: u32 = 6;
pub const NotifyDetailNone: u32 = 7;

pub const VisibilityUnovscured: u32 = 0;
pub const VisibilityPartiallyObscured: u32 = 1;
pub const VisibilityFullyObscured: u32 = 2;

pub const PlaceOnTop: u32 = 0;
pub const PlaceOnBottom: u32 = 1;

pub const FamilyInternet: u32 = 0;
pub const FamilyDECnet: u32 = 1;
pub const FamilyChaos: u32 = 2;
pub const FamilyInternet6: u32 = 3;

pub const FamilyServerInterpreted: u32 = 5;

pub const PropertyNewValue: u32 = 0;
pub const PropertyDelete: u32 = 1;

pub const ColormapUninstalled: u32 = 0;
pub const ColormapInstalled: u32 = 1;

pub const GrabModeSync: u32 = 0;
pub const GrabModeAsync: u32 = 1;

pub const GrabSuccess: u32 = 0;
pub const AlreadyGrabbed: u32 = 1;
pub const GrabInvalidTime: u32 = 2;
pub const GrabNotViewable: u32 = 3;
pub const GrabFrozen: u32 = 4;

pub const AsyncPointer: u32 = 0;
pub const SyncPointer: u32 = 1;
pub const ReplayPointer: u32 = 2;
pub const AsyncKeyboard: u32 = 3;
pub const SyncKeyboard: u32 = 4;
pub const ReplayKeyboard: u32 = 5;
pub const AsyncBoth: u32 = 6;
pub const SyncBoth: u32 = 7;

pub const RevertToNone: u32 = None as u32;
pub const RevertToPointerRoot: u32 = PointerRoot as u32;
pub const RevertToParent: u32 = 2;

pub const Success: u32 = 0;
pub const BadRequest: u32 = 1;
pub const BadValue: u32 = 2;
pub const BadWindow: u32 = 3;
pub const BadPixmap: u32 = 4;
pub const BadAtom: u32 = 5;
pub const BadCursor: u32 = 6;
pub const BadFont: u32 = 7;
pub const BadMatch: u32 = 8;
pub const BadDrawable: u32 = 9;
pub const BadAccess: u32 = 10;
pub const BadAlloc: u32 = 11;
pub const BadColor: u32 = 12;
pub const BadGC: u32 = 13;
pub const BadIDChoice: u32 = 14;
pub const BadName: u32 = 15;
pub const BadLength: u32 = 16;
pub const BadImplementation: u32 = 17;

pub const FirstExtensionError: u32 = 128;
pub const LastExtensionError: u32 = 255;

pub const InputOutput: u32 = 1;
pub const InputOnly: u32 = 2;

pub const CWBackPixmap: u32 = 1 << 0;
pub const CWBackPixel: u32 = 1 << 1;
pub const CWBorderPixmap: u32 = 1 << 2;
pub const CWBorderPixel: u32 = 1 << 3;
pub const CWBitGravity: u32 = 1 << 4;
pub const CWWinGravity: u32 = 1 << 5;
pub const CWBackingStore: u32 = 1 << 6;
pub const CWBackingPlanes: u32 = 1 << 7;
pub const CWBackingPixel: u32 = 1 << 8;
pub const CWOverrideRedirect: u32 = 1 << 9;
pub const CWSaveUnder: u32 = 1 << 10;
pub const CWEventMask: u32 = 1 << 11;
pub const CWDontPropagate: u32 = 1 << 12;
pub const CWColormap: u32 = 1 << 13;
pub const CWCursor: u32 = 1 << 14;

pub const CWX: u32 = 1 << 0;
pub const CWY: u32 = 1 << 1;
pub const CWWidth: u32 = 1 << 2;
pub const CWHeight: u32 = 1 << 3;
pub const CWBorderWidth: u32 = 1 << 4;
pub const CWSibling: u32 = 1 << 5;
pub const CWStackMode: u32 = 1 << 6;

pub const ForgetGravity: u32 = 0;
pub const NorthWestGravity: u32 = 1;
pub const NorthGravity: u32 = 2;
pub const NorthEastGravity: u32 = 3;
pub const WestGravity: u32 = 4;
pub const CenterGravity: u32 = 5;
pub const EastGravity: u32 = 6;
pub const SouthWestGravity: u32 = 7;
pub const SouthGravity: u32 = 8;
pub const SouthEastGravity: u32 = 9;
pub const StaticGravity: u32 = 10;

pub const UnmapGrab: u32 = 0;

pub const NotUseful: u32 = 0;
pub const WhenMapped: u32 = 1;
pub const Always: u32 = 2;

pub const IsUnmapped: u32 = 0;
pub const IsUnviewable: u32 = 1;
pub const IsViewable: u32 = 2;

pub const SetModeInsert: u32 = 0;
pub const SetModeDelete: u32 = 1;

pub const DestroyAll: u32 = 0;
pub const RetrainPermanent: u32 = 1;
pub const RetainTemporary: u32 = 2;

pub const Above: u32 = 0;
pub const Below: u32 = 1;
pub const TopIf: u32 = 2;
pub const BottomIf: u32 = 3;
pub const Opposite: u32 = 4;

pub const RaiseLowest: u32 = 0;
pub const LowerHighest: u32 = 1;

pub const PropModeReplace: u32 = 0;
pub const PropModePrepend: u32 = 1;
pub const PropModeAppend: u32 = 2;

pub const GXclear: u32 = 0x0;
pub const GXand: u32 = 0x1;
pub const GXandReverse: u32 = 0x2;
pub const GXcopy: u32 = 0x3;
pub const GXandInverted: u32 = 0x4;
pub const GXnoop: u32 = 0x5;
pub const GXxor: u32 = 0x6;
pub const GXor: u32 = 0x7;
pub const GXnor: u32 = 0x8;
pub const GXequiv: u32 = 0x9;
pub const GXinvert: u32 = 0xa;
pub const GXorReverse: u32 = 0xb;
pub const GXcopyInverted: u32 = 0xc;
pub const GXorInverted: u32 = 0xd;
pub const GXnand: u32 = 0xe;
pub const GXset: u32 = 0xf;

pub const LineSolid: u32 = 0;
pub const LineOnOffDash: u32 = 1;
pub const LineDoubleDash: u32 = 2;

pub const CapNotLast: u32 = 0;
pub const CapButt: u32 = 1;
pub const CapRound: u32 = 2;
pub const CapProjecting: u32 = 3;

pub const JoinMiter: u32 = 0;
pub const JoinRound: u32 = 1;
pub const JoinBevel: u32 = 2;

pub const FillSolid: u32 = 0;
pub const FillTiled: u32 = 1;
pub const FillStippled: u32 = 2;
pub const FillOpaqueStippled: u32 = 3;

pub const EvenOddRule: u32 = 0;
pub const WindingRule: u32 = 1;

pub const ClipByChildren: u32 = 0;
pub const IncludeInferiors: u32 = 1;

pub const Unsorted: u32 = 0;
pub const YSorted: u32 = 1;
pub const YXSorted: u32 = 2;
pub const YXBanded: u32 = 3;

pub const CoordModeOrigin: u32 = 0;
pub const CoordModePrevious: u32 = 1;

pub const Complex: u32 = 0;
pub const Nonconvex: u32 = 1;
pub const Convex: u32 = 2;

pub const ArcChord: u32 = 0;
pub const ArcPieSlice: u32 = 1;

pub const GCFunction: u32 = 1 << 0;
pub const GCPlaneMask: u32 = 1 << 1;
pub const GCForeground: u32 = 1 << 2;
pub const GCBackground: u32 = 1 << 3;
pub const GCLineWidth: u32 = 1 << 4;
pub const GCLineStyle: u32 = 1 << 5;
pub const GCCapStyle: u32 = 1 << 6;
pub const GCJoinStyle: u32 = 1 << 7;
pub const GCFillStyle: u32 = 1 << 8;
pub const GCFillRule: u32 = 1 << 9;
pub const GCTile: u32 = 1 << 10;
pub const GCStipple: u32 = 1 << 11;
pub const GCTileStipXOrigin: u32 = 1 << 12;
pub const GCTileStipYOrigin: u32 = 1 << 13;
pub const GCFont: u32 = 1 << 14;
pub const GCSubwindowMode: u32 = 1 << 15;
pub const GCGraphicsExposures: u32 = 1 << 16;
pub const GCClipXOrigin: u32 = 1 << 17;
pub const GCClipYOrigin: u32 = 1 << 18;
pub const GCClipMask: u32 = 1 << 19;
pub const GCDashOffset: u32 = 1 << 20;
pub const GCDashList: u32 = 1 << 21;
pub const GCArcMode: u32 = 1 << 22;

pub const GCLastBit: u32 = 22;

pub const FontLeftToRight: u32 = 0;
pub const FontRightToLeft: u32 = 1;

pub const FontChange: u32 = 255;

pub const XYBitmap: u32 = 0;
pub const XYPixmap: u32 = 1;
pub const ZPixmap: u32 = 2;

pub const AllocNone: u32 = 0;
pub const AllocAll: u32 = 1;

pub const DoRed: u32 = 1 << 0;
pub const DoGreen: u32 = 1 << 1;
pub const DoBlue: u32 = 1 << 2;

pub const CursorShape: u32 = 0;
pub const TileShape: u32 = 1;
pub const StippleShape: u32 = 2;

pub const AutoRepeatModeOff: u32 = 0;
pub const AutoRepeatModeOn: u32 = 1;
pub const AutoRepeatModeDefault: u32 = 2;

pub const LedModeOff: u32 = 0;
pub const LedModeOn: u32 = 1;

pub const KBKeyClickPercent: u64 = 1 << 0;
pub const KBBellPercent: u64 = 1 << 1;
pub const KBBellPitch: u64 = 1 << 2;
pub const KBBellDuration: u64 = 1 << 3;
pub const KBLed: u64 = 1 << 4;
pub const KBLedMode: u64 = 1 << 5;
pub const KBKey: u64 = 1 << 6;
pub const KBAutoRepeatMode: u64 = 1 << 7;

pub const MappingSuccess: u32 = 0;
pub const MappingBusy: u32 = 1;
pub const MappingFailed: u32 = 2;

pub const MappingModifier: u32 = 0;
pub const MappingKeyboard: u32 = 1;
pub const MappingPointer: u32 = 2;

pub const DontPreferBlanking: u32 = 0;
pub const PreferBlanking: u32 = 1;
pub const DefaultBlanking: u32 = 2;

pub const DisableScreenSaver: u32 = 0;
pub const DisableScreenInterval: u32 = 1;

pub const DontAllowExposures: u32 = 0;
pub const AllowExposures: u32 = 1;
pub const DefaultExposures: u32 = 2;

pub const ScreenSaverReset: u32 = 0;
pub const ScreenSaverActive: u32 = 1;

pub const HostInsert: u32 = 0;
pub const HostDelete: u32 = 1;

pub const EnableAccess: u32 = 1;
pub const DisableAccess: u32 = 0;

pub const StaticGray: u32 = 0;
pub const GrayScale: u32 = 1;
pub const StaticColor: u32 = 2;
pub const PseudoColor: u32 = 3;
pub const TrueColor: u32 = 4;
pub const DirectColor: u32 = 5;

pub const LSBFirst: u32 = 0;
pub const MSBFirst: u32 = 1;
