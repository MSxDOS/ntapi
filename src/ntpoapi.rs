use windows_sys::Win32::{
    Foundation::{HANDLE, NTSTATUS, UNICODE_STRING},
    System::Power::{
        DEVICE_POWER_STATE, EXECUTION_STATE, LATENCY_TIME, POWER_ACTION,
        POWER_INFORMATION_LEVEL, SYSTEM_POWER_STATE,
    },
};

use crate::ctypes::{c_long, c_uchar, c_ulong, c_ushort, c_void};

UNION! {union POWER_STATE {
    SystemState: SYSTEM_POWER_STATE,
    DeviceState: DEVICE_POWER_STATE,
}}
pub type PPOWER_STATE = *mut POWER_STATE;
ENUM! {enum POWER_STATE_TYPE {
    SystemPowerState = 0,
    DevicePowerState = 1,
}}
pub type PPOWER_STATE_TYPE = *mut POWER_STATE_TYPE;
STRUCT! {struct SYSTEM_POWER_STATE_CONTEXT {
    ContextAsUlong: c_ulong,
}}
BITFIELD! {SYSTEM_POWER_STATE_CONTEXT ContextAsUlong: c_ulong [
    Reserved1 set_Reserved1[0..8],
    TargetSystemState set_TargetSystemState[8..12],
    EffectiveSystemState set_EffectiveSystemState[12..16],
    CurrentSystemState set_CurrentSystemState[16..20],
    IgnoreHibernationPath set_IgnoreHibernationPath[20..21],
    PseudoTransition set_PseudoTransition[21..22],
    Reserved2 set_Reserved2[22..32],
]}
pub type PSYSTEM_POWER_STATE_CONTEXT = *mut SYSTEM_POWER_STATE_CONTEXT;
STRUCT! {struct COUNTED_REASON_CONTEXT_u_s {
    ResourceFileName: UNICODE_STRING,
    ResourceReasonId: c_ushort,
    StringCount: c_ulong,
    ReasonStrings: *mut UNICODE_STRING,
}}
UNION! {union COUNTED_REASON_CONTEXT_u {
    s: COUNTED_REASON_CONTEXT_u_s,
    SimpleString: UNICODE_STRING,
}}
STRUCT! {struct COUNTED_REASON_CONTEXT {
    Version: c_ulong,
    Flags: c_ulong,
    u: COUNTED_REASON_CONTEXT_u,
}}
pub type PCOUNTED_REASON_CONTEXT = *mut COUNTED_REASON_CONTEXT;
ENUM! {enum POWER_STATE_HANDLER_TYPE {
    PowerStateSleeping1 = 0,
    PowerStateSleeping2 = 1,
    PowerStateSleeping3 = 2,
    PowerStateSleeping4 = 3,
    PowerStateShutdownOff = 4,
    PowerStateShutdownReset = 5,
    PowerStateSleeping4Firmware = 6,
    PowerStateMaximum = 7,
}}
pub type PPOWER_STATE_HANDLER_TYPE = *mut POWER_STATE_HANDLER_TYPE;
FN! {stdcall PENTER_STATE_SYSTEM_HANDLER(
    SystemContext: *mut c_void,
) -> NTSTATUS}
FN! {stdcall PENTER_STATE_HANDLER(
    Context: *mut c_void,
    SystemHandler: PENTER_STATE_SYSTEM_HANDLER,
    SystemContext: *mut c_void,
    NumberProcessors: c_long,
    Number: *mut c_long,
) -> NTSTATUS}
STRUCT! {struct POWER_STATE_HANDLER {
    Type: POWER_STATE_HANDLER_TYPE,
    RtcWake: c_uchar,
    Spare: [c_uchar; 3],
    Handler: PENTER_STATE_HANDLER,
    Context: *mut c_void,
}}
pub type PPOWER_STATE_HANDLER = *mut POWER_STATE_HANDLER;
FN! {stdcall PENTER_STATE_NOTIFY_HANDLER(
    State: POWER_STATE_HANDLER_TYPE,
    Context: *mut c_void,
    Entering: c_uchar,
) -> NTSTATUS}
STRUCT! {struct POWER_STATE_NOTIFY_HANDLER {
    Handler: PENTER_STATE_NOTIFY_HANDLER,
    Context: *mut c_void,
}}
pub type PPOWER_STATE_NOTIFY_HANDLER = *mut POWER_STATE_NOTIFY_HANDLER;
STRUCT! {struct PROCESSOR_POWER_INFORMATION {
    Number: c_ulong,
    MaxMhz: c_ulong,
    CurrentMhz: c_ulong,
    MhzLimit: c_ulong,
    MaxIdleState: c_ulong,
    CurrentIdleState: c_ulong,
}}
pub type PPROCESSOR_POWER_INFORMATION = *mut PROCESSOR_POWER_INFORMATION;
STRUCT! {struct SYSTEM_POWER_INFORMATION {
    MaxIdlenessAllowed: c_ulong,
    Idleness: c_ulong,
    TimeRemaining: c_ulong,
    CoolingMode: c_uchar,
}}
pub type PSYSTEM_POWER_INFORMATION = *mut SYSTEM_POWER_INFORMATION;
EXTERN! {extern "system" {
    fn NtPowerInformation(
        InformationLevel: POWER_INFORMATION_LEVEL,
        InputBuffer: *mut c_void,
        InputBufferLength: c_ulong,
        OutputBuffer: *mut c_void,
        OutputBufferLength: c_ulong,
    ) -> NTSTATUS;
    fn NtSetThreadExecutionState(
        NewFlags: EXECUTION_STATE,
        PreviousFlags: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtRequestWakeupLatency(
        latency: LATENCY_TIME,
    ) -> NTSTATUS;
    fn NtInitiatePowerAction(
        SystemAction: POWER_ACTION,
        LightestSystemState: SYSTEM_POWER_STATE,
        Flags: c_ulong,
        Asynchronous: c_uchar,
    ) -> NTSTATUS;
    fn NtSetSystemPowerState(
        SystemAction: POWER_ACTION,
        LightestSystemState: SYSTEM_POWER_STATE,
        Flags: c_ulong,
    ) -> NTSTATUS;
    fn NtGetDevicePowerState(
        Device: HANDLE,
        State: *mut DEVICE_POWER_STATE,
    ) -> NTSTATUS;
    fn NtIsSystemResumeAutomatic() -> c_uchar;
}}
