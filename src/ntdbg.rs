use windows_sys::{
    core::GUID,
    Win32::{
        Foundation::{HANDLE, NTSTATUS},
        Storage::FileSystem::{STANDARD_RIGHTS_REQUIRED, SYNCHRONIZE},
        System::{
            Diagnostics::{
                Debug::{DEBUG_EVENT, EXCEPTION_RECORD},
                Etw::EVENT_FILTER_DESCRIPTOR,
            },
            WindowsProgramming::OBJECT_ATTRIBUTES,
        },
    },
};

use crate::{
    ctypes::{__uint64, c_char, c_uchar, c_ulong, c_void},
    ntapi_base::{CLIENT_ID, PCLIENT_ID},
    windows_local::shared::ntdef::LARGE_INTEGER,
};

EXTERN! {extern "system" {
    fn DbgUserBreakPoint();
    fn DbgBreakPoint();
    fn DbgBreakPointWithStatus(
        Status: c_ulong,
    );
}}
pub const DBG_STATUS_CONTROL_C: u32 = 1;
pub const DBG_STATUS_SYSRQ: u32 = 2;
pub const DBG_STATUS_BUGCHECK_FIRST: u32 = 3;
pub const DBG_STATUS_BUGCHECK_SECOND: u32 = 4;
pub const DBG_STATUS_FATAL: u32 = 5;
pub const DBG_STATUS_DEBUG_CONTROL: u32 = 6;
pub const DBG_STATUS_WORKER: u32 = 7;
EXTERN! {extern "C" {
    fn DbgPrint(
        Format: *const c_char,
        ...
    ) -> c_ulong;
    fn DbgPrintEx(
        ComponentId: c_ulong,
        Level: c_ulong,
        Format: *const c_char,
        ...
    ) -> c_ulong;
}}
EXTERN! {extern "system" {
    fn vDbgPrintEx(
        ComponentId: c_ulong,
        Level: c_ulong,
        Format: *const c_char,
        arglist: *mut c_char,
    ) -> c_ulong;
    fn vDbgPrintExWithPrefix(
        Prefix: *mut c_char,
        ComponentId: c_ulong,
        Level: c_ulong,
        Format: *const c_char,
        arglist: *mut c_char,
    ) -> c_ulong;
    fn DbgQueryDebugFilterState(
        ComponentId: c_ulong,
        Level: c_ulong,
    ) -> NTSTATUS;
    fn DbgSetDebugFilterState(
        ComponentId: c_ulong,
        Level: c_ulong,
        State: c_uchar,
    ) -> NTSTATUS;
    fn DbgPrompt(
        Prompt: *const c_char,
        Response: *mut c_char,
        Length: c_ulong,
    ) -> c_ulong;
}}
STRUCT! {struct DBGKM_EXCEPTION {
    ExceptionRecord: EXCEPTION_RECORD,
    FirstChance: c_ulong,
}}
pub type PDBGKM_EXCEPTION = *mut DBGKM_EXCEPTION;
STRUCT! {struct DBGKM_CREATE_THREAD {
    SubSystemKey: c_ulong,
    StartAddress: *mut c_void,
}}
pub type PDBGKM_CREATE_THREAD = *mut DBGKM_CREATE_THREAD;
STRUCT! {struct DBGKM_CREATE_PROCESS {
    SubSystemKey: c_ulong,
    FileHandle: HANDLE,
    BaseOfImage: *mut c_void,
    DebugInfoFileOffset: c_ulong,
    DebugInfoSize: c_ulong,
    InitialThread: DBGKM_CREATE_THREAD,
}}
pub type PDBGKM_CREATE_PROCESS = *mut DBGKM_CREATE_PROCESS;
STRUCT! {struct DBGKM_EXIT_THREAD {
    ExitStatus: NTSTATUS,
}}
pub type PDBGKM_EXIT_THREAD = *mut DBGKM_EXIT_THREAD;
STRUCT! {struct DBGKM_EXIT_PROCESS {
    ExitStatus: NTSTATUS,
}}
pub type PDBGKM_EXIT_PROCESS = *mut DBGKM_EXIT_PROCESS;
STRUCT! {struct DBGKM_LOAD_DLL {
    FileHandle: HANDLE,
    BaseOfDll: *mut c_void,
    DebugInfoFileOffset: c_ulong,
    DebugInfoSize: c_ulong,
    NamePointer: *mut c_void,
}}
pub type PDBGKM_LOAD_DLL = *mut DBGKM_LOAD_DLL;
STRUCT! {struct DBGKM_UNLOAD_DLL {
    BaseAddress: *mut c_void,
}}
pub type PDBGKM_UNLOAD_DLL = *mut DBGKM_UNLOAD_DLL;
ENUM! {enum DBG_STATE {
    DbgIdle = 0,
    DbgReplyPending = 1,
    DbgCreateThreadStateChange = 2,
    DbgCreateProcessStateChange = 3,
    DbgExitThreadStateChange = 4,
    DbgExitProcessStateChange = 5,
    DbgExceptionStateChange = 6,
    DbgBreakpointStateChange = 7,
    DbgSingleStepStateChange = 8,
    DbgLoadDllStateChange = 9,
    DbgUnloadDllStateChange = 10,
}}
pub type PDBG_STATE = *mut DBG_STATE;
STRUCT! {struct DBGUI_CREATE_THREAD {
    HandleToThread: HANDLE,
    NewThread: DBGKM_CREATE_THREAD,
}}
pub type PDBGUI_CREATE_THREAD = *mut DBGUI_CREATE_THREAD;
STRUCT! {struct DBGUI_CREATE_PROCESS {
    HandleToProcess: HANDLE,
    HandleToThread: HANDLE,
    NewProcess: DBGKM_CREATE_PROCESS,
}}
UNION! {union DBGUI_WAIT_STATE_CHANGE_StateInfo {
    Exception: DBGKM_EXCEPTION,
    CreateThread: DBGUI_CREATE_THREAD,
    CreateProcessInfo: DBGUI_CREATE_PROCESS,
    ExitThread: DBGKM_EXIT_THREAD,
    ExitProcess: DBGKM_EXIT_PROCESS,
    LoadDll: DBGKM_LOAD_DLL,
    UnloadDll: DBGKM_UNLOAD_DLL,
}}
pub type PDBGUI_CREATE_PROCESS = *mut DBGUI_CREATE_PROCESS;
STRUCT! {struct DBGUI_WAIT_STATE_CHANGE {
    NewState: DBG_STATE,
    AppClientId: CLIENT_ID,
    StateInfo: DBGUI_WAIT_STATE_CHANGE_StateInfo,
}}
pub type PDBGUI_WAIT_STATE_CHANGE = *mut DBGUI_WAIT_STATE_CHANGE;
pub const DEBUG_READ_EVENT: c_ulong = 0x0001;
pub const DEBUG_PROCESS_ASSIGN: c_ulong = 0x0002;
pub const DEBUG_SET_INFORMATION: c_ulong = 0x0004;
pub const DEBUG_QUERY_INFORMATION: c_ulong = 0x0008;
pub const DEBUG_ALL_ACCESS: c_ulong = STANDARD_RIGHTS_REQUIRED
    | SYNCHRONIZE
    | DEBUG_READ_EVENT
    | DEBUG_PROCESS_ASSIGN
    | DEBUG_SET_INFORMATION
    | DEBUG_QUERY_INFORMATION;
pub const DEBUG_KILL_ON_CLOSE: u32 = 0x1;
ENUM! {enum DEBUGOBJECTINFOCLASS {
    DebugObjectUnusedInformation = 0,
    DebugObjectKillProcessOnExitInformation = 1,
    MaxDebugObjectInfoClass = 2,
}}
pub type PDEBUGOBJECTINFOCLASS = *mut DEBUGOBJECTINFOCLASS;
EXTERN! {extern "system" {
    fn NtCreateDebugObject(
        DebugObjectHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        Flags: c_ulong,
    ) -> NTSTATUS;
    fn NtDebugActiveProcess(
        ProcessHandle: HANDLE,
        DebugObjectHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtDebugContinue(
        DebugObjectHandle: HANDLE,
        ClientId: PCLIENT_ID,
        ContinueStatus: NTSTATUS,
    ) -> NTSTATUS;
    fn NtRemoveProcessDebug(
        ProcessHandle: HANDLE,
        DebugObjectHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtSetInformationDebugObject(
        DebugObjectHandle: HANDLE,
        DebugObjectInformationClass: DEBUGOBJECTINFOCLASS,
        DebugInformation: *mut c_void,
        DebugInformationLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtWaitForDebugEvent(
        DebugObjectHandle: HANDLE,
        Alertable: c_uchar,
        Timeout: *mut LARGE_INTEGER,
        WaitStateChange: *mut c_void,
    ) -> NTSTATUS;
    fn DbgUiConnectToDbg() -> NTSTATUS;
    fn DbgUiGetThreadDebugObject() -> HANDLE;
    fn DbgUiSetThreadDebugObject(
        DebugObject: HANDLE,
    );
    fn DbgUiWaitStateChange(
        StateChange: PDBGUI_WAIT_STATE_CHANGE,
        Timeout: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn DbgUiContinue(
        AppClientId: PCLIENT_ID,
        ContinueStatus: NTSTATUS,
    ) -> NTSTATUS;
    fn DbgUiStopDebugging(
        Process: HANDLE,
    ) -> NTSTATUS;
    fn DbgUiDebugActiveProcess(
        Process: HANDLE,
    ) -> NTSTATUS;
    fn DbgUiRemoteBreakin(
        Context: *mut c_void,
    );
    fn DbgUiIssueRemoteBreakin(
        Process: HANDLE,
    ) -> NTSTATUS;
    fn DbgUiConvertStateChangeStructure(
        StateChange: PDBGUI_WAIT_STATE_CHANGE,
        DebugEvent: *mut DEBUG_EVENT,
    ) -> NTSTATUS;
}}
FN! {stdcall PENABLECALLBACK(
    SourceId: *const GUID,
    IsEnabled: c_ulong,
    Level: c_uchar,
    MatchAnyKeyword: __uint64,
    MatchAllKeyword: __uint64,
    FilterData: *mut EVENT_FILTER_DESCRIPTOR,
    CallbackContext: *mut c_void,
) -> ()}
pub type REGHANDLE = __uint64;
pub type PREGHANDLE = *mut __uint64;
EXTERN! {extern "system" {
    fn EtwEventRegister(
        ProviderId: *const GUID,
        EnableCallback: PENABLECALLBACK,
        CallbackContext: *mut c_void,
        RegHandle: PREGHANDLE,
    ) -> NTSTATUS;
}}
