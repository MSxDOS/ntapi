use windows_sys::Win32::{
    Foundation::{HANDLE, NTSTATUS, UNICODE_STRING},
    System::{
        Kernel::{LIST_ENTRY, RTL_BALANCED_NODE, SINGLE_LIST_ENTRY, STRING},
        SystemServices::{
            IMAGE_BASE_RELOCATION, IMAGE_IMPORT_DESCRIPTOR,
            IMAGE_RESOURCE_DATA_ENTRY, IMAGE_RESOURCE_DIRECTORY,
            IMAGE_RESOURCE_DIRECTORY_ENTRY, IMAGE_RESOURCE_DIRECTORY_STRING,
        },
        WindowsProgramming::{
            IMAGE_DELAYLOAD_DESCRIPTOR, IMAGE_THUNK_DATA64, OBJECT_ATTRIBUTES,
        },
    },
};

use crate::{
    ctypes::{
        __int64, c_char, c_long, c_uchar, c_ulong, c_ushort, c_void, wchar_t,
    },
    windows_local::{
        shared::ntdef::LARGE_INTEGER, um::winnt::ACTIVATION_CONTEXT,
    },
};

FN! {stdcall PLDR_INIT_ROUTINE(
    DllHandle: *mut c_void,
    Reason: c_ulong,
    Context: *mut c_void,
) -> c_uchar}
STRUCT! {struct LDR_SERVICE_TAG_RECORD {
    Next: *mut LDR_SERVICE_TAG_RECORD,
    ServiceTag: c_ulong,
}}
pub type PLDR_SERVICE_TAG_RECORD = *mut LDR_SERVICE_TAG_RECORD;
STRUCT! {struct LDRP_CSLIST {
    Tail: *mut SINGLE_LIST_ENTRY,
}}
pub type PLDRP_CSLIST = *mut LDRP_CSLIST;
ENUM! {enum LDR_DDAG_STATE {
    LdrModulesMerged = -5i32 as u32,
    LdrModulesInitError = -4i32 as u32,
    LdrModulesSnapError = -3i32 as u32,
    LdrModulesUnloaded = -2i32 as u32,
    LdrModulesUnloading = -1i32 as u32,
    LdrModulesPlaceHolder = 0,
    LdrModulesMapping = 1,
    LdrModulesMapped = 2,
    LdrModulesWaitingForDependencies = 3,
    LdrModulesSnapping = 4,
    LdrModulesSnapped = 5,
    LdrModulesCondensed = 6,
    LdrModulesReadyToInit = 7,
    LdrModulesInitializing = 8,
    LdrModulesReadyToRun = 9,
}}
UNION! {union LDR_DDAG_NODE_u {
    Dependencies: LDRP_CSLIST,
    RemovalLink: SINGLE_LIST_ENTRY,
}}
STRUCT! {struct LDR_DDAG_NODE {
    Modules: LIST_ENTRY,
    ServiceTagList: PLDR_SERVICE_TAG_RECORD,
    LoadCount: c_ulong,
    LoadWhileUnloadingCount: c_ulong,
    LowestLink: c_ulong,
    u: LDR_DDAG_NODE_u,
    IncomingDependencies: LDRP_CSLIST,
    State: LDR_DDAG_STATE,
    CondenseLink: SINGLE_LIST_ENTRY,
    PreorderNumber: c_ulong,
}}
pub type PLDR_DDAG_NODE = *mut LDR_DDAG_NODE;
STRUCT! {struct LDR_DEPENDENCY_RECORD {
    DependencyLink: SINGLE_LIST_ENTRY,
    DependencyNode: PLDR_DDAG_NODE,
    IncomingDependencyLink: SINGLE_LIST_ENTRY,
    IncomingDependencyNode: PLDR_DDAG_NODE,
}}
pub type PLDR_DEPENDENCY_RECORD = *mut LDR_DEPENDENCY_RECORD;
ENUM! {enum LDR_DLL_LOAD_REASON {
    LoadReasonStaticDependency = 0,
    LoadReasonStaticForwarderDependency = 1,
    LoadReasonDynamicForwarderDependency = 2,
    LoadReasonDelayloadDependency = 3,
    LoadReasonDynamicLoad = 4,
    LoadReasonAsImageLoad = 5,
    LoadReasonAsDataLoad = 6,
    LoadReasonEnclavePrimary = 7,
    LoadReasonEnclaveDependency = 8,
    LoadReasonUnknown = -1i32 as u32,
}}
pub type PLDR_DLL_LOAD_REASON = *mut LDR_DLL_LOAD_REASON;
pub const LDRP_PACKAGED_BINARY: c_ulong = 0x00000001;
pub const LDRP_STATIC_LINK: c_ulong = 0x00000002;
pub const LDRP_IMAGE_DLL: c_ulong = 0x00000004;
pub const LDRP_LOAD_IN_PROGRESS: c_ulong = 0x00001000;
pub const LDRP_UNLOAD_IN_PROGRESS: c_ulong = 0x00002000;
pub const LDRP_ENTRY_PROCESSED: c_ulong = 0x00004000;
pub const LDRP_ENTRY_INSERTED: c_ulong = 0x00008000;
pub const LDRP_CURRENT_LOAD: c_ulong = 0x00010000;
pub const LDRP_FAILED_BUILTIN_LOAD: c_ulong = 0x00020000;
pub const LDRP_DONT_CALL_FOR_THREADS: c_ulong = 0x00040000;
pub const LDRP_PROCESS_ATTACH_CALLED: c_ulong = 0x00080000;
pub const LDRP_DEBUG_SYMBOLS_LOADED: c_ulong = 0x00100000;
pub const LDRP_IMAGE_NOT_AT_BASE: c_ulong = 0x00200000;
pub const LDRP_COR_IMAGE: c_ulong = 0x00400000;
pub const LDRP_DONT_RELOCATE: c_ulong = 0x00800000;
pub const LDRP_SYSTEM_MAPPED: c_ulong = 0x01000000;
pub const LDRP_IMAGE_VERIFYING: c_ulong = 0x02000000;
pub const LDRP_DRIVER_DEPENDENT_DLL: c_ulong = 0x04000000;
pub const LDRP_ENTRY_NATIVE: c_ulong = 0x08000000;
pub const LDRP_REDIRECTED: c_ulong = 0x10000000;
pub const LDRP_NON_PAGED_DEBUG_INFO: c_ulong = 0x20000000;
pub const LDRP_MM_LOADED: c_ulong = 0x40000000;
pub const LDRP_COMPAT_DATABASE_PROCESSED: c_ulong = 0x80000000;
STRUCT! {struct LDRP_LOAD_CONTEXT {
    BaseDllName: UNICODE_STRING,
    somestruct: *mut c_void,
    Flags: c_ulong,
    pstatus: *mut NTSTATUS,
    ParentEntry: *mut LDR_DATA_TABLE_ENTRY,
    Entry: *mut LDR_DATA_TABLE_ENTRY,
    WorkQueueListEntry: LIST_ENTRY,
    ReplacedEntry: *mut LDR_DATA_TABLE_ENTRY,
    pvImports: *mut *mut LDR_DATA_TABLE_ENTRY,
    ImportDllCount: c_ulong,
    TaskCount: c_long,
    pvIAT: *mut c_void,
    SizeOfIAT: c_ulong,
    CurrentDll: c_ulong,
    piid: *mut IMAGE_IMPORT_DESCRIPTOR,
    OriginalIATProtect: c_ulong,
    GuardCFCheckFunctionPointer: *mut c_void,
    pGuardCFCheckFunctionPointer: *mut *mut c_void,
}}
UNION! {union LDR_DATA_TABLE_ENTRY_u1 {
    InInitializationOrderLinks: LIST_ENTRY,
    InProgressLinks: LIST_ENTRY,
}}
UNION! {union LDR_DATA_TABLE_ENTRY_u2 {
    FlagGroup: [c_uchar; 4],
    Flags: c_ulong,
}}
STRUCT! {struct LDR_DATA_TABLE_ENTRY {
    InLoadOrderLinks: LIST_ENTRY,
    InMemoryOrderLinks: LIST_ENTRY,
    u1: LDR_DATA_TABLE_ENTRY_u1,
    DllBase: *mut c_void,
    EntryPoint: PLDR_INIT_ROUTINE,
    SizeOfImage: c_ulong,
    FullDllName: UNICODE_STRING,
    BaseDllName: UNICODE_STRING,
    u2: LDR_DATA_TABLE_ENTRY_u2,
    ObsoleteLoadCount: c_ushort,
    TlsIndex: c_ushort,
    HashLinks: LIST_ENTRY,
    TimeDateStamp: c_ulong,
    EntryPointActivationContext: *mut ACTIVATION_CONTEXT,
    Lock: *mut c_void,
    DdagNode: PLDR_DDAG_NODE,
    NodeModuleLink: LIST_ENTRY,
    LoadContext: *mut LDRP_LOAD_CONTEXT,
    ParentDllBase: *mut c_void,
    SwitchBackContext: *mut c_void,
    BaseAddressIndexNode: RTL_BALANCED_NODE,
    MappingInfoIndexNode: RTL_BALANCED_NODE,
    OriginalBase: usize,
    LoadTime: LARGE_INTEGER,
    BaseNameHashValue: c_ulong,
    LoadReason: LDR_DLL_LOAD_REASON,
    ImplicitPathOptions: c_ulong,
    ReferenceCount: c_ulong,
    DependentLoadFlags: c_ulong,
    SigningLevel: c_uchar,
}}
BITFIELD! {unsafe LDR_DATA_TABLE_ENTRY_u2 Flags: c_ulong [
    PackagedBinary set_PackagedBinary[0..1],
    MarkedForRemoval set_MarkedForRemoval[1..2],
    ImageDll set_ImageDll[2..3],
    LoadNotificationsSent set_LoadNotificationsSent[3..4],
    TelemetryEntryProcessed set_TelemetryEntryProcessed[4..5],
    ProcessStaticImport set_ProcessStaticImport[5..6],
    InLegacyLists set_InLegacyLists[6..7],
    InIndexes set_InIndexes[7..8],
    ShimDll set_ShimDll[8..9],
    InExceptionTable set_InExceptionTable[9..10],
    ReservedFlags1 set_ReservedFlags1[10..12],
    LoadInProgress set_LoadInProgress[12..13],
    LoadConfigProcessed set_LoadConfigProcessed[13..14],
    EntryProcessed set_EntryProcessed[14..15],
    ProtectDelayLoad set_ProtectDelayLoad[15..16],
    ReservedFlags3 set_ReservedFlags3[16..18],
    DontCallForThreads set_DontCallForThreads[18..19],
    ProcessAttachCalled set_ProcessAttachCalled[19..20],
    ProcessAttachFailed set_ProcessAttachFailed[20..21],
    CorDeferredValidate set_CorDeferredValidate[21..22],
    CorImage set_CorImage[22..23],
    DontRelocate set_DontRelocate[23..24],
    CorILOnly set_CorILOnly[24..25],
    ReservedFlags5 set_ReservedFlags5[25..28],
    Redirected set_Redirected[28..29],
    ReservedFlags6 set_ReservedFlags6[29..31],
    CompatDatabaseProcessed set_CompatDatabaseProcessed[31..32],
]}
pub type PLDR_DATA_TABLE_ENTRY = *mut LDR_DATA_TABLE_ENTRY;
#[inline]
pub const fn LDR_IS_DATAFILE(DllHandle: usize) -> bool {
    DllHandle & 1 != 0
}
#[inline]
pub const fn LDR_IS_IMAGEMAPPING(DllHandle: usize) -> bool {
    DllHandle & 2 != 0
}
#[inline]
pub const fn LDR_IS_RESOURCE(DllHandle: usize) -> bool {
    LDR_IS_IMAGEMAPPING(DllHandle) || LDR_IS_DATAFILE(DllHandle)
}
EXTERN! {extern "system" {
    fn LdrLoadDll(
        DllPath: *mut wchar_t,
        DllCharacteristics: *mut c_ulong,
        DllName: *mut UNICODE_STRING,
        DllHandle: *mut *mut c_void,
    ) -> NTSTATUS;
    fn LdrUnloadDll(
        DllHandle: *mut c_void,
    ) -> NTSTATUS;
    fn LdrGetDllHandle(
        DllPath: *mut wchar_t,
        DllCharacteristics: *mut c_ulong,
        DllName: *mut UNICODE_STRING,
        DllHandle: *mut *mut c_void,
    ) -> NTSTATUS;
}}
pub const LDR_GET_DLL_HANDLE_EX_UNCHANGED_REFCOUNT: c_ulong = 0x00000001;
pub const LDR_GET_DLL_HANDLE_EX_PIN: c_ulong = 0x00000002;
EXTERN! {extern "system" {
    fn LdrGetDllHandleEx(
        Flags: c_ulong,
        DllPath: *mut wchar_t,
        DllCharacteristics: *mut c_ulong,
        DllName: *mut UNICODE_STRING,
        DllHandle: *mut *mut c_void,
    ) -> NTSTATUS;
    fn LdrGetDllHandleByMapping(
        BaseAddress: *mut c_void,
        DllHandle: *mut *mut c_void,
    ) -> NTSTATUS;
    fn LdrGetDllHandleByName(
        BaseDllName: *mut UNICODE_STRING,
        FullDllName: *mut UNICODE_STRING,
        DllHandle: *mut *mut c_void,
    ) -> NTSTATUS;
    fn LdrGetDllFullName(
        DllHandle: *mut c_void,
        FullDllName: *mut UNICODE_STRING,
    ) -> NTSTATUS;
    fn LdrGetDllDirectory(
        DllDirectory: *mut UNICODE_STRING,
    ) -> NTSTATUS;
    fn LdrSetDllDirectory(
        DllDirectory: *mut UNICODE_STRING,
    ) -> NTSTATUS;
}}
pub const LDR_ADDREF_DLL_PIN: c_ulong = 0x00000001;
EXTERN! {extern "system" {
    fn LdrAddRefDll(
        Flags: c_ulong,
        DllHandle: *mut c_void,
    ) -> NTSTATUS;
    fn LdrGetProcedureAddress(
        DllHandle: *mut c_void,
        ProcedureName: *mut STRING,
        ProcedureNumber: c_ulong,
        ProcedureAddress: *mut *mut c_void,
    ) -> NTSTATUS;
}}
pub const LDR_GET_PROCEDURE_ADDRESS_DONT_RECORD_FORWARDER: c_ulong = 0x00000001;
EXTERN! {extern "system" {
    fn LdrGetProcedureAddressEx(
        DllHandle: *mut c_void,
        ProcedureName: *mut STRING,
        ProcedureNumber: c_ulong,
        ProcedureAddress: *mut *mut c_void,
        Flags: c_ulong,
    ) -> NTSTATUS;
    fn LdrGetKnownDllSectionHandle(
        DllName: *const wchar_t,
        KnownDlls32: c_uchar,
        Section: *mut HANDLE,
    ) -> NTSTATUS;
    fn LdrGetProcedureAddressForCaller(
        DllHandle: *mut c_void,
        ProcedureName: *mut STRING,
        ProcedureNumber: c_ulong,
        ProcedureAddress: *mut *mut c_void,
        Flags: c_ulong,
        Callback: *mut *mut c_void,
    ) -> NTSTATUS;
}}
pub const LDR_LOCK_LOADER_LOCK_FLAG_RAISE_ON_ERRORS: c_ulong = 0x00000001;
pub const LDR_LOCK_LOADER_LOCK_FLAG_TRY_ONLY: c_ulong = 0x00000002;
pub const LDR_LOCK_LOADER_LOCK_DISPOSITION_INVALID: c_ulong = 0;
pub const LDR_LOCK_LOADER_LOCK_DISPOSITION_LOCK_ACQUIRED: c_ulong = 1;
pub const LDR_LOCK_LOADER_LOCK_DISPOSITION_LOCK_NOT_ACQUIRED: c_ulong = 2;
EXTERN! {extern "system" {
    fn LdrLockLoaderLock(
        Flags: c_ulong,
        Disposition: *mut c_ulong,
        Cookie: *mut *mut c_void,
    ) -> NTSTATUS;
}}
pub const LDR_UNLOCK_LOADER_LOCK_FLAG_RAISE_ON_ERRORS: c_ulong = 0x00000001;
EXTERN! {extern "system" {
    fn LdrUnlockLoaderLock(
        Flags: c_ulong,
        Cookie: *mut c_void,
    ) -> NTSTATUS;
    fn LdrRelocateImage(
        NewBase: *mut c_void,
        LoaderName: *mut c_char,
        Success: NTSTATUS,
        Conflict: NTSTATUS,
        Invalid: NTSTATUS,
    ) -> NTSTATUS;
    fn LdrRelocateImageWithBias(
        NewBase: *mut c_void,
        Bias: __int64,
        LoaderName: *mut c_char,
        Success: NTSTATUS,
        Conflict: NTSTATUS,
        Invalid: NTSTATUS,
    ) -> NTSTATUS;
    fn LdrProcessRelocationBlock(
        VA: usize,
        SizeOfBlock: c_ulong,
        NextOffset: *mut c_ushort,
        Diff: isize,
    ) -> *mut IMAGE_BASE_RELOCATION;
    fn LdrVerifyMappedImageMatchesChecksum(
        BaseAddress: *mut c_void,
        NumberOfBytes: usize,
        FileLength: c_ulong,
    ) -> c_uchar;
}}
FN! {stdcall PLDR_IMPORT_MODULE_CALLBACK(
    Parameter: *mut c_void,
    ModuleName: *mut c_char,
) -> ()}
EXTERN! {extern "system" {
    fn LdrVerifyImageMatchesChecksum(
        ImageFileHandle: HANDLE,
        ImportCallbackRoutine: PLDR_IMPORT_MODULE_CALLBACK,
        ImportCallbackParameter: *mut c_void,
        ImageCharacteristics: *mut c_ushort,
    ) -> NTSTATUS;
}}
STRUCT! {struct LDR_IMPORT_CALLBACK_INFO {
    ImportCallbackRoutine: PLDR_IMPORT_MODULE_CALLBACK,
    ImportCallbackParameter: *mut c_void,
}}
pub type PLDR_IMPORT_CALLBACK_INFO = *mut LDR_IMPORT_CALLBACK_INFO;
STRUCT! {struct LDR_SECTION_INFO {
    SectionHandle: HANDLE,
    DesiredAccess: c_ulong,
    ObjA: *mut OBJECT_ATTRIBUTES,
    SectionPageProtection: c_ulong,
    AllocationAttributes: c_ulong,
}}
pub type PLDR_SECTION_INFO = *mut LDR_SECTION_INFO;
STRUCT! {struct LDR_VERIFY_IMAGE_INFO {
    Size: c_ulong,
    Flags: c_ulong,
    CallbackInfo: LDR_IMPORT_CALLBACK_INFO,
    SectionInfo: LDR_SECTION_INFO,
    ImageCharacteristics: c_ushort,
}}
pub type PLDR_VERIFY_IMAGE_INFO = *mut LDR_VERIFY_IMAGE_INFO;
EXTERN! {extern "system" {
    fn LdrVerifyImageMatchesChecksumEx(
        ImageFileHandle: HANDLE,
        VerifyInfo: PLDR_VERIFY_IMAGE_INFO,
    ) -> NTSTATUS;
    fn LdrQueryModuleServiceTags(
        DllHandle: *mut c_void,
        ServiceTagBuffer: *mut c_ulong,
        BufferSize: *mut c_ulong,
    ) -> NTSTATUS;
}}
pub const LDR_DLL_NOTIFICATION_REASON_LOADED: c_ulong = 1;
pub const LDR_DLL_NOTIFICATION_REASON_UNLOADED: c_ulong = 2;
STRUCT! {struct LDR_DLL_LOADED_NOTIFICATION_DATA {
    Flags: c_ulong,
    FullDllName: *mut UNICODE_STRING,
    BaseDllName: *mut UNICODE_STRING,
    DllBase: *mut c_void,
    SizeOfImage: c_ulong,
}}
pub type PLDR_DLL_LOADED_NOTIFICATION_DATA =
    *mut LDR_DLL_LOADED_NOTIFICATION_DATA;
STRUCT! {struct LDR_DLL_UNLOADED_NOTIFICATION_DATA {
    Flags: c_ulong,
    FullDllName: *const UNICODE_STRING,
    BaseDllName: *const UNICODE_STRING,
    DllBase: *mut c_void,
    SizeOfImage: c_ulong,
}}
pub type PLDR_DLL_UNLOADED_NOTIFICATION_DATA =
    *mut LDR_DLL_UNLOADED_NOTIFICATION_DATA;
UNION! {union LDR_DLL_NOTIFICATION_DATA {
    Loaded: LDR_DLL_LOADED_NOTIFICATION_DATA,
    Unloaded: LDR_DLL_UNLOADED_NOTIFICATION_DATA,
}}
pub type PLDR_DLL_NOTIFICATION_DATA = *mut LDR_DLL_NOTIFICATION_DATA;
FN! {stdcall PLDR_DLL_NOTIFICATION_FUNCTION(
    NotificationReason: c_ulong,
    NotificationData: PLDR_DLL_NOTIFICATION_DATA,
    Context: *mut c_void,
) -> ()}
EXTERN! {extern "system" {
    fn LdrRegisterDllNotification(
        Flags: c_ulong,
        NotificationFunction: PLDR_DLL_NOTIFICATION_FUNCTION,
        Context: *mut c_void,
        Cookie: *mut *mut c_void,
    ) -> NTSTATUS;
    fn LdrUnregisterDllNotification(
        Cookie: *mut c_void,
    ) -> NTSTATUS;
}}
STRUCT! {struct PS_MITIGATION_OPTIONS_MAP {
    Map: [usize; 2],
}}
pub type PPS_MITIGATION_OPTIONS_MAP = *mut PS_MITIGATION_OPTIONS_MAP;
STRUCT! {struct PS_MITIGATION_AUDIT_OPTIONS_MAP {
    Map: [usize; 2],
}}
pub type PPS_MITIGATION_AUDIT_OPTIONS_MAP =
    *mut PS_MITIGATION_AUDIT_OPTIONS_MAP;
STRUCT! {struct PS_SYSTEM_DLL_INIT_BLOCK {
    Size: c_ulong,
    SystemDllWowRelocation: usize,
    SystemDllNativeRelocation: usize,
    Wow64SharedInformation: [usize; 16],
    RngData: c_ulong,
    Flags: c_ulong,
    MitigationOptionsMap: PS_MITIGATION_OPTIONS_MAP,
    CfgBitMap: usize,
    CfgBitMapSize: usize,
    Wow64CfgBitMap: usize,
    Wow64CfgBitMapSize: usize,
    MitigationAuditOptionsMap: PS_MITIGATION_AUDIT_OPTIONS_MAP,
}}
BITFIELD! {PS_SYSTEM_DLL_INIT_BLOCK Flags: c_ulong [
    CfgOverride set_CfgOverride[0..1],
    Reserved set_Reserved[1..32],
]}
pub type PPS_SYSTEM_DLL_INIT_BLOCK = *mut PS_SYSTEM_DLL_INIT_BLOCK;
EXTERN! {extern "system" {
    fn LdrSystemDllInitBlock() -> PPS_SYSTEM_DLL_INIT_BLOCK;
    fn LdrAddLoadAsDataTable(
        Module: *mut c_void,
        FilePath: *mut wchar_t,
        Size: usize,
        Handle: HANDLE,
    ) -> NTSTATUS;
    fn LdrRemoveLoadAsDataTable(
        InitModule: *mut c_void,
        BaseModule: *mut *mut c_void,
        Size: *mut usize,
        Flags: c_ulong,
    ) -> NTSTATUS;
    fn LdrGetFileNameFromLoadAsDataTable(
        Module: *mut c_void,
        pFileNamePrt: *mut *mut c_void,
    ) -> NTSTATUS;
    fn LdrDisableThreadCalloutsForDll(
        DllImageBase: *mut c_void,
    ) -> NTSTATUS;
    fn LdrAccessResource(
        DllHandle: *mut c_void,
        ResourceDataEntry: *mut IMAGE_RESOURCE_DATA_ENTRY,
        ResourceBuffer: *mut *mut c_void,
        ResourceLength: *mut c_ulong,
    ) -> NTSTATUS;
}}
STRUCT! {struct LDR_RESOURCE_INFO {
    Type: usize,
    Name: usize,
    Language: usize,
}}
pub type PLDR_RESOURCE_INFO = *mut LDR_RESOURCE_INFO;
pub const RESOURCE_TYPE_LEVEL: c_ulong = 0;
pub const RESOURCE_NAME_LEVEL: c_ulong = 1;
pub const RESOURCE_LANGUAGE_LEVEL: c_ulong = 2;
pub const RESOURCE_DATA_LEVEL: c_ulong = 3;
EXTERN! {extern "system" {
    fn LdrFindResource_U(
        DllHandle: *mut c_void,
        ResourceInfo: PLDR_RESOURCE_INFO,
        Level: c_ulong,
        ResourceDataEntry: *mut *mut IMAGE_RESOURCE_DATA_ENTRY,
    ) -> NTSTATUS;
    fn LdrFindResourceDirectory_U(
        DllHandle: *mut c_void,
        ResourceInfo: PLDR_RESOURCE_INFO,
        Level: c_ulong,
        ResourceDirectory: *mut *mut IMAGE_RESOURCE_DIRECTORY,
    ) -> NTSTATUS;
}}
STRUCT! {struct LDR_ENUM_RESOURCE_ENTRY_Path_s {
    Id: c_ushort,
    NameIsPresent: c_ushort,
}}
UNION! {union LDR_ENUM_RESOURCE_ENTRY_Path {
    NameOrId: usize,
    Name: *mut IMAGE_RESOURCE_DIRECTORY_STRING,
    s: LDR_ENUM_RESOURCE_ENTRY_Path_s,
}}
STRUCT! {struct LDR_ENUM_RESOURCE_ENTRY {
    Path: [LDR_ENUM_RESOURCE_ENTRY_Path; 3],
    Data: *mut c_void,
    Size: c_ulong,
    Reserved: c_ulong,
}}
pub type PLDR_ENUM_RESOURCE_ENTRY = *mut LDR_ENUM_RESOURCE_ENTRY;
#[inline]
pub unsafe fn NAME_FROM_RESOURCE_ENTRY(
    RootDirectory: *mut IMAGE_RESOURCE_DIRECTORY,
    Entry: &IMAGE_RESOURCE_DIRECTORY_ENTRY,
) -> usize {
    // if Entry.u.s().NameIsString() != 0 {
    //     return RootDirectory as usize + Entry.u.s().NameOffset() as usize;
    // }
    // *Entry.u.Id() as usize
    if Entry.Anonymous1.Name == 0 {
        Entry.Anonymous1.Id as usize
    } else {
        RootDirectory as usize + Entry.Anonymous2.OffsetToData as usize
    }
}
EXTERN! {extern "system" {
    fn LdrEnumResources(
        DllHandle: *mut c_void,
        ResourceInfo: PLDR_RESOURCE_INFO,
        Level: c_ulong,
        ResourceCount: *mut c_ulong,
        Resources: PLDR_ENUM_RESOURCE_ENTRY,
    ) -> NTSTATUS;
    fn LdrFindEntryForAddress(
        DllHandle: *mut c_void,
        Entry: *mut PLDR_DATA_TABLE_ENTRY,
    ) -> NTSTATUS;
}}
STRUCT! {struct RTL_PROCESS_MODULE_INFORMATION {
    Section: HANDLE,
    MappedBase: *mut c_void,
    ImageBase: *mut c_void,
    ImageSize: c_ulong,
    Flags: c_ulong,
    LoadOrderIndex: c_ushort,
    InitOrderIndex: c_ushort,
    LoadCount: c_ushort,
    OffsetToFileName: c_ushort,
    FullPathName: [c_uchar; 256],
}}
pub type PRTL_PROCESS_MODULE_INFORMATION = *mut RTL_PROCESS_MODULE_INFORMATION;
STRUCT! {struct RTL_PROCESS_MODULES {
    NumberOfModules: c_ulong,
    Modules: [RTL_PROCESS_MODULE_INFORMATION; 1],
}}
pub type PRTL_PROCESS_MODULES = *mut RTL_PROCESS_MODULES;
STRUCT! {struct RTL_PROCESS_MODULE_INFORMATION_EX {
    NextOffset: c_ushort,
    BaseInfo: RTL_PROCESS_MODULE_INFORMATION,
    ImageChecksum: c_ulong,
    TimeDateStamp: c_ulong,
    DefaultBase: *mut c_void,
}}
pub type PRTL_PROCESS_MODULE_INFORMATION_EX =
    *mut RTL_PROCESS_MODULE_INFORMATION_EX;
EXTERN! {extern "system" {
    fn LdrQueryProcessModuleInformation(
        ModuleInformation: PRTL_PROCESS_MODULES,
        Size: c_ulong,
        ReturnedSize: *mut c_ulong,
    ) -> NTSTATUS;
}}
FN! {stdcall PLDR_ENUM_CALLBACK(
    ModuleInformation: PLDR_DATA_TABLE_ENTRY,
    Parameter: *mut c_void,
    Stop: *mut c_uchar,
) -> ()}
EXTERN! {extern "system" {
    fn LdrEnumerateLoadedModules(
        ReservedFlag: c_uchar,
        EnumProc: PLDR_ENUM_CALLBACK,
        Context: *mut c_void,
    ) -> NTSTATUS;
    fn LdrOpenImageFileOptionsKey(
        SubKey: *mut UNICODE_STRING,
        Wow64: c_uchar,
        NewKeyHandle: *mut HANDLE,
    ) -> NTSTATUS;
    fn LdrQueryImageFileKeyOption(
        KeyHandle: HANDLE,
        ValueName: *const wchar_t,
        Type: c_ulong,
        Buffer: *mut c_void,
        BufferSize: c_ulong,
        ReturnedLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn LdrQueryImageFileExecutionOptions(
        SubKey: *mut UNICODE_STRING,
        ValueName: *const wchar_t,
        ValueSize: c_ulong,
        Buffer: *mut c_void,
        BufferSize: c_ulong,
        ReturnedLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn LdrQueryImageFileExecutionOptionsEx(
        SubKey: *mut UNICODE_STRING,
        ValueName: *const wchar_t,
        Type: c_ulong,
        Buffer: *mut c_void,
        BufferSize: c_ulong,
        ReturnedLength: *mut c_ulong,
        Wow64: c_uchar,
    ) -> NTSTATUS;
}}
UNION! {union DELAYLOAD_PROC_DESCRIPTOR_Description {
    Name: *const c_char,
    Ordinal: c_ulong,
}}
STRUCT! {struct DELAYLOAD_PROC_DESCRIPTOR {
    ImportDescribedByName: c_ulong,
    Description: DELAYLOAD_PROC_DESCRIPTOR_Description,
}}
pub type PDELAYLOAD_PROC_DESCRIPTOR = *mut DELAYLOAD_PROC_DESCRIPTOR;
STRUCT! {struct DELAYLOAD_INFO {
    Size: c_ulong,
    DelayloadDescriptor: *const IMAGE_DELAYLOAD_DESCRIPTOR,
    ThunkAddress: *mut IMAGE_THUNK_DATA64,
    TargetDllName: *const c_char,
    TargetApiDescriptor: DELAYLOAD_PROC_DESCRIPTOR,
    TargetModuleBase: *mut c_void,
    Unused: *mut c_void,
    LastError: c_ulong,
}}
pub type PDELAYLOAD_INFO = *mut DELAYLOAD_INFO;
FN! {stdcall PDELAYLOAD_FAILURE_DLL_CALLBACK(
    NotificationReason: c_ulong,
    DelayloadInfo: PDELAYLOAD_INFO,
) -> *mut c_void}
FN! {stdcall PDELAYLOAD_FAILURE_SYSTEM_ROUTINE(
    DllName: *const c_char,
    ProcName: *const c_char,
) -> *mut c_void}
EXTERN! {extern "system" {
    fn LdrResolveDelayLoadedAPI(
        ParentModuleBase: *mut c_void,
        DelayloadDescriptor: *const IMAGE_DELAYLOAD_DESCRIPTOR,
        FailureDllHook: PDELAYLOAD_FAILURE_DLL_CALLBACK,
        FailureSystemHook: PDELAYLOAD_FAILURE_SYSTEM_ROUTINE,
        ThunkAddress: *mut IMAGE_THUNK_DATA64,
        Flags: c_ulong,
    ) -> *mut c_void;
    fn LdrResolveDelayLoadsFromDll(
        ParentBase: *mut c_void,
        TargetDllName: *const c_char,
        Flags: c_ulong,
    ) -> NTSTATUS;
    fn LdrSetDefaultDllDirectories(
        DirectoryFlags: c_ulong,
    ) -> NTSTATUS;
    fn LdrShutdownProcess() -> NTSTATUS;
    fn LdrShutdownThread() -> NTSTATUS;
    fn LdrSetImplicitPathOptions(
        ImplicitPathOptions: c_ulong,
    ) -> NTSTATUS;
    fn LdrControlFlowGuardEnforced() -> c_uchar;
}}
