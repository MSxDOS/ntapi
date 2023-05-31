use windows_sys::{
    core::GUID,
    Win32::{
        Devices::DeviceAndDriverInstallation::PNP_VETO_TYPE,
        Foundation::{HANDLE, NTSTATUS, UNICODE_STRING},
    },
};

use crate::ctypes::{c_ulong, c_void, wchar_t};

ENUM! {enum PLUGPLAY_EVENT_CATEGORY {
    HardwareProfileChangeEvent = 0,
    TargetDeviceChangeEvent = 1,
    DeviceClassChangeEvent = 2,
    CustomDeviceEvent = 3,
    DeviceInstallEvent = 4,
    DeviceArrivalEvent = 5,
    PowerEvent = 6,
    VetoEvent = 7,
    BlockedDriverEvent = 8,
    InvalidIDEvent = 9,
    MaxPlugEventCategory = 10,
}}
pub type PPLUGPLAY_EVENT_CATEGORY = *mut PLUGPLAY_EVENT_CATEGORY;
STRUCT! {struct PLUGPLAY_EVENT_BLOCK_u_DeviceClass {
    ClassGuid: GUID,
    SymbolicLinkName: [wchar_t; 1],
}}
STRUCT! {struct PLUGPLAY_EVENT_BLOCK_u_TargetDevice {
    DeviceIds: [wchar_t; 1],
}}
STRUCT! {struct PLUGPLAY_EVENT_BLOCK_u_InstallDevice {
    DeviceId: [wchar_t; 1],
}}
STRUCT! {struct PLUGPLAY_EVENT_BLOCK_u_CustomNotification {
    NotificationStructure: *mut c_void,
    DeviceIds: [wchar_t; 1],
}}
STRUCT! {struct PLUGPLAY_EVENT_BLOCK_u_ProfileNotification {
    Notification: *mut c_void,
}}
STRUCT! {struct PLUGPLAY_EVENT_BLOCK_u_PowerNotification {
    NotificationCode: c_ulong,
    NotificationData: c_ulong,
}}
STRUCT! {struct PLUGPLAY_EVENT_BLOCK_u_VetoNotification {
    VetoType: PNP_VETO_TYPE,
    DeviceIdVetoNameBuffer: [wchar_t; 1],
}}
STRUCT! {struct PLUGPLAY_EVENT_BLOCK_u_BlockedDriverNotification {
    BlockedDriverGuid: GUID,
}}
STRUCT! {struct PLUGPLAY_EVENT_BLOCK_u_InvalidIDNotification {
    ParentId: [wchar_t; 1],
}}
UNION! {union PLUGPLAY_EVENT_BLOCK_u {
    DeviceClass: PLUGPLAY_EVENT_BLOCK_u_DeviceClass,
    TargetDevice: PLUGPLAY_EVENT_BLOCK_u_TargetDevice,
    InstallDevice: PLUGPLAY_EVENT_BLOCK_u_InstallDevice,
    CustomNotification: PLUGPLAY_EVENT_BLOCK_u_CustomNotification,
    ProfileNotification: PLUGPLAY_EVENT_BLOCK_u_ProfileNotification,
    PowerNotification: PLUGPLAY_EVENT_BLOCK_u_PowerNotification,
    VetoNotification: PLUGPLAY_EVENT_BLOCK_u_VetoNotification,
    BlockedDriverNotification: PLUGPLAY_EVENT_BLOCK_u_BlockedDriverNotification,
    InvalidIDNotification: PLUGPLAY_EVENT_BLOCK_u_InvalidIDNotification,
}}
STRUCT! {struct PLUGPLAY_EVENT_BLOCK {
    EventGuid: GUID,
    EventCategory: PLUGPLAY_EVENT_CATEGORY,
    Result: *mut c_ulong,
    Flags: c_ulong,
    TotalSize: c_ulong,
    DeviceObject: *mut c_void,
    u: PLUGPLAY_EVENT_BLOCK_u,
}}
pub type PPLUGPLAY_EVENT_BLOCK = *mut PLUGPLAY_EVENT_BLOCK;
ENUM! {enum PLUGPLAY_CONTROL_CLASS {
    PlugPlayControlEnumerateDevice = 0,
    PlugPlayControlRegisterNewDevice = 1,
    PlugPlayControlDeregisterDevice = 2,
    PlugPlayControlInitializeDevice = 3,
    PlugPlayControlStartDevice = 4,
    PlugPlayControlUnlockDevice = 5,
    PlugPlayControlQueryAndRemoveDevice = 6,
    PlugPlayControlUserResponse = 7,
    PlugPlayControlGenerateLegacyDevice = 8,
    PlugPlayControlGetInterfaceDeviceList = 9,
    PlugPlayControlProperty = 10,
    PlugPlayControlDeviceClassAssociation = 11,
    PlugPlayControlGetRelatedDevice = 12,
    PlugPlayControlGetInterfaceDeviceAlias = 13,
    PlugPlayControlDeviceStatus = 14,
    PlugPlayControlGetDeviceDepth = 15,
    PlugPlayControlQueryDeviceRelations = 16,
    PlugPlayControlTargetDeviceRelation = 17,
    PlugPlayControlQueryConflictList = 18,
    PlugPlayControlRetrieveDock = 19,
    PlugPlayControlResetDevice = 20,
    PlugPlayControlHaltDevice = 21,
    PlugPlayControlGetBlockedDriverList = 22,
    PlugPlayControlGetDeviceInterfaceEnabled = 23,
    MaxPlugPlayControl = 24,
}}
pub type PPLUGPLAY_CONTROL_CLASS = *mut PLUGPLAY_CONTROL_CLASS;
EXTERN! {extern "system" {
    fn NtGetPlugPlayEvent(
        EventHandle: HANDLE,
        Context: *mut c_void,
        EventBlock: PPLUGPLAY_EVENT_BLOCK,
        EventBufferSize: c_ulong,
    ) -> NTSTATUS;
    fn NtPlugPlayControl(
        PnPControlClass: PLUGPLAY_CONTROL_CLASS,
        PnPControlData: *mut c_void,
        PnPControlDataLength: c_ulong,
    ) -> NTSTATUS;
    fn NtSerializeBoot() -> NTSTATUS;
    fn NtEnableLastKnownGood() -> NTSTATUS;
    fn NtDisableLastKnownGood() -> NTSTATUS;
    fn NtReplacePartitionUnit(
        TargetInstancePath: *mut UNICODE_STRING,
        SpareInstancePath: *mut UNICODE_STRING,
        Flags: c_ulong,
    ) -> NTSTATUS;
}}
