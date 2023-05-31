use windows_sys::{
    core::GUID,
    Win32::{
        Foundation::{HANDLE, NTSTATUS, UNICODE_STRING},
        System::{
            Kernel::{LIST_ENTRY, NT_TIB, PROCESSOR_NUMBER, SLIST_HEADER},
            SystemServices::FLS_MAXIMUM_AVAILABLE,
            Threading::RTL_CRITICAL_SECTION,
        },
    },
};

use crate::{
    ctypes::{
        __uint64, c_char, c_long, c_uchar, c_ulong, c_ushort, c_void, wchar_t,
    },
    ntapi_base::CLIENT_ID,
    ntpsapi::{GDI_HANDLE_BUFFER, PPEB_LDR_DATA},
    ntrtl::PRTL_USER_PROCESS_PARAMETERS,
    windows_local::{
        shared::ntdef::ULARGE_INTEGER, um::winnt::ACTIVATION_CONTEXT,
    },
};

STRUCT! {struct RTL_ACTIVATION_CONTEXT_STACK_FRAME {
    Previous: PRTL_ACTIVATION_CONTEXT_STACK_FRAME,
    ActivationContext: *mut ACTIVATION_CONTEXT,
    Flags: c_ulong,
}}
pub type PRTL_ACTIVATION_CONTEXT_STACK_FRAME =
    *mut RTL_ACTIVATION_CONTEXT_STACK_FRAME;
STRUCT! {struct ACTIVATION_CONTEXT_STACK {
    ActiveFrame: *mut RTL_ACTIVATION_CONTEXT_STACK_FRAME,
    FrameListCache: LIST_ENTRY,
    Flags: c_ulong,
    NextCookieSequenceNumber: c_ulong,
    StackId: c_ulong,
}}
pub type PACTIVATION_CONTEXT_STACK = *mut ACTIVATION_CONTEXT_STACK;
STRUCT! {struct API_SET_NAMESPACE {
    Version: c_ulong,
    Size: c_ulong,
    Flags: c_ulong,
    Count: c_ulong,
    EntryOffset: c_ulong,
    HashOffset: c_ulong,
    HashFactor: c_ulong,
}}
pub type PAPI_SET_NAMESPACE = *mut API_SET_NAMESPACE;
STRUCT! {struct API_SET_HASH_ENTRY {
    Hash: c_ulong,
    Index: c_ulong,
}}
pub type PAPI_SET_HASH_ENTRY = *mut API_SET_HASH_ENTRY;
STRUCT! {struct API_SET_NAMESPACE_ENTRY {
    Flags: c_ulong,
    NameOffset: c_ulong,
    NameLength: c_ulong,
    HashedLength: c_ulong,
    ValueOffset: c_ulong,
    ValueCount: c_ulong,
}}
pub type PAPI_SET_NAMESPACE_ENTRY = *mut API_SET_NAMESPACE_ENTRY;
STRUCT! {struct API_SET_VALUE_ENTRY {
    Flags: c_ulong,
    NameOffset: c_ulong,
    NameLength: c_ulong,
    ValueOffset: c_ulong,
    ValueLength: c_ulong,
}}
pub type PAPI_SET_VALUE_ENTRY = *mut API_SET_VALUE_ENTRY;
UNION! {union PEB_u {
    KernelCallbackTable: *mut c_void,
    UserSharedInfoPtr: *mut c_void,
}}
#[repr(C)]
pub struct LEAP_SECOND_DATA([u8; 0]); //fixme
STRUCT! {struct PEB {
    InheritedAddressSpace: c_uchar,
    ReadImageFileExecOptions: c_uchar,
    BeingDebugged: c_uchar,
    BitField: c_uchar,
    Mutant: HANDLE,
    ImageBaseAddress: *mut c_void,
    Ldr: PPEB_LDR_DATA,
    ProcessParameters: PRTL_USER_PROCESS_PARAMETERS,
    SubSystemData: *mut c_void,
    ProcessHeap: *mut c_void,
    FastPebLock: *mut RTL_CRITICAL_SECTION,
    IFEOKey: *mut c_void,
    AtlThunkSListPtr: *mut SLIST_HEADER,
    CrossProcessFlags: c_ulong,
    u: PEB_u,
    SystemReserved: [c_ulong; 1],
    AtlThunkSListPtr32: c_ulong,
    ApiSetMap: PAPI_SET_NAMESPACE,
    TlsExpansionCounter: c_ulong,
    TlsBitmap: *mut c_void,
    TlsBitmapBits: [c_ulong; 2],
    ReadOnlySharedMemoryBase: *mut c_void,
    SharedData: *mut c_void,
    ReadOnlyStaticServerData: *mut *mut c_void,
    AnsiCodePageData: *mut c_void,
    OemCodePageData: *mut c_void,
    UnicodeCaseTableData: *mut c_void,
    NumberOfProcessors: c_ulong,
    NtGlobalFlag: c_ulong,
    CriticalSectionTimeout: ULARGE_INTEGER,
    HeapSegmentReserve: usize,
    HeapSegmentCommit: usize,
    HeapDeCommitTotalFreeThreshold: usize,
    HeapDeCommitFreeBlockThreshold: usize,
    NumberOfHeaps: c_ulong,
    MaximumNumberOfHeaps: c_ulong,
    ProcessHeaps: *mut *mut c_void,
    GdiSharedHandleTable: *mut c_void,
    ProcessStarterHelper: *mut c_void,
    GdiDCAttributeList: c_ulong,
    LoaderLock: *mut RTL_CRITICAL_SECTION,
    OSMajorVersion: c_ulong,
    OSMinorVersion: c_ulong,
    OSBuildNumber: c_ushort,
    OSCSDVersion: c_ushort,
    OSPlatformId: c_ulong,
    ImageSubsystem: c_ulong,
    ImageSubsystemMajorVersion: c_ulong,
    ImageSubsystemMinorVersion: c_ulong,
    ActiveProcessAffinityMask: usize,
    GdiHandleBuffer: GDI_HANDLE_BUFFER,
    PostProcessInitRoutine: *mut c_void,
    TlsExpansionBitmap: *mut c_void,
    TlsExpansionBitmapBits: [c_ulong; 32],
    SessionId: c_ulong,
    AppCompatFlags: ULARGE_INTEGER,
    AppCompatFlagsUser: ULARGE_INTEGER,
    pShimData: *mut c_void,
    AppCompatInfo: *mut c_void,
    CSDVersion: UNICODE_STRING,
    ActivationContextData: *mut c_void,
    ProcessAssemblyStorageMap: *mut c_void,
    SystemDefaultActivationContextData: *mut c_void,
    SystemAssemblyStorageMap: *mut c_void,
    MinimumStackCommit: usize,
    FlsCallback: *mut *mut c_void,
    FlsListHead: LIST_ENTRY,
    FlsBitmap: *mut c_void,
    FlsBitmapBits: [c_ulong; FLS_MAXIMUM_AVAILABLE as usize / c_ulong::BITS as usize],
    FlsHighIndex: c_ulong,
    WerRegistrationData: *mut c_void,
    WerShipAssertPtr: *mut c_void,
    pUnused: *mut c_void,
    pImageHeaderHash: *mut c_void,
    TracingFlags: c_ulong,
    CsrServerReadOnlySharedMemoryBase: __uint64,
    TppWorkerpListLock: *mut RTL_CRITICAL_SECTION,
    TppWorkerpList: LIST_ENTRY,
    WaitOnAddressHashTable: [*mut c_void; 128],
    TelemetryCoverageHeader: *mut c_void,
    CloudFileFlags: c_ulong,
    CloudFileDiagFlags: c_ulong,
    PlaceholderCompatibilityMode: c_char,
    PlaceholderCompatibilityModeReserved: [c_char; 7],
    LeapSecondData: *mut LEAP_SECOND_DATA,
    LeapSecondFlags: c_ulong,
    NtGlobalFlag2: c_ulong,
}}
BITFIELD! {PEB BitField: c_uchar [
    ImageUsesLargePages set_ImageUsesLargePages[0..1],
    IsProtectedProcess set_IsProtectedProcess[1..2],
    IsImageDynamicallyRelocated set_IsImageDynamicallyRelocated[2..3],
    SkipPatchingUser32Forwarders set_SkipPatchingUser32Forwarders[3..4],
    IsPackagedProcess set_IsPackagedProcess[4..5],
    IsAppContainer set_IsAppContainer[5..6],
    IsProtectedProcessLight set_IsProtectedProcessLight[6..7],
    IsLongPathAwareProcess set_IsLongPathAwareProcess[7..8],
]}
BITFIELD! {PEB CrossProcessFlags: c_ulong [
    ProcessInJob set_ProcessInJob[0..1],
    ProcessInitializing set_ProcessInitializing[1..2],
    ProcessUsingVEH set_ProcessUsingVEH[2..3],
    ProcessUsingVCH set_ProcessUsingVCH[3..4],
    ProcessUsingFTH set_ProcessUsingFTH[4..5],
    ProcessPreviouslyThrottled set_ProcessPreviouslyThrottled[5..6],
    ProcessCurrentlyThrottled set_ProcessCurrentlyThrottled[6..7],
    ProcessImagesHotPatched set_ProcessImagesHotPatched[7..8],
    ReservedBits0 set_ReservedBits0[8..32],
]}
BITFIELD! {PEB TracingFlags: c_ulong [
    HeapTracingEnabled set_HeapTracingEnabled[0..1],
    CritSecTracingEnabled set_CritSecTracingEnabled[1..2],
    LibLoaderTracingEnabled set_LibLoaderTracingEnabled[2..3],
    SpareTracingBits set_SpareTracingBits[3..32],
]}
BITFIELD! {PEB LeapSecondFlags: c_ulong [
    SixtySecondEnabled set_SixtySecondEnabled[0..1],
    Reserved set_Reserved[1..32],
]}
pub type PPEB = *mut PEB;
pub const GDI_BATCH_BUFFER_SIZE: usize = 310;
STRUCT! {struct GDI_TEB_BATCH {
    Offset: c_ulong,
    HDC: usize,
    Buffer: [c_ulong; GDI_BATCH_BUFFER_SIZE],
}}
pub type PGDI_TEB_BATCH = *mut GDI_TEB_BATCH;
STRUCT! {struct TEB_ACTIVE_FRAME_CONTEXT {
    Flags: c_ulong,
    FrameName: *mut c_char,
}}
pub type PTEB_ACTIVE_FRAME_CONTEXT = *mut TEB_ACTIVE_FRAME_CONTEXT;
STRUCT! {struct TEB_ACTIVE_FRAME {
    Flags: c_ulong,
    Previous: *mut TEB_ACTIVE_FRAME,
    Context: PTEB_ACTIVE_FRAME_CONTEXT,
}}
pub type PTEB_ACTIVE_FRAME = *mut TEB_ACTIVE_FRAME;
STRUCT! {struct TEB_u_s {
    ReservedPad0: c_uchar,
    ReservedPad1: c_uchar,
    ReservedPad2: c_uchar,
    IdealProcessor: c_uchar,
}}
UNION! {union TEB_u {
    CurrentIdealProcessor: PROCESSOR_NUMBER,
    IdealProcessorValue: c_ulong,
    s: TEB_u_s,
}}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
STRUCT! {struct TEB {
    NtTib: NT_TIB,
    EnvironmentPointer: *mut c_void,
    ClientId: CLIENT_ID,
    ActiveRpcHandle: *mut c_void,
    ThreadLocalStoragePointer: *mut c_void,
    ProcessEnvironmentBlock: PPEB,
    LastErrorValue: c_ulong,
    CountOfOwnedCriticalSections: c_ulong,
    CsrClientThread: *mut c_void,
    Win32ThreadInfo: *mut c_void,
    User32Reserved: [c_ulong; 26],
    UserReserved: [c_ulong; 5],
    WOW32Reserved: *mut c_void,
    CurrentLocale: c_ulong,
    FpSoftwareStatusRegister: c_ulong,
    ReservedForDebuggerInstrumentation: [*mut c_void; 16],
    SystemReserved1: [*mut c_void; 30],
    PlaceholderCompatibilityMode: c_char,
    PlaceholderReserved: [c_char; 11],
    ProxiedProcessId: c_ulong,
    ActivationStack: ACTIVATION_CONTEXT_STACK,
    WorkingOnBehalfTicket: [c_uchar; 8],
    ExceptionCode: NTSTATUS,
    ActivationContextStackPointer: PACTIVATION_CONTEXT_STACK,
    InstrumentationCallbackSp: usize,
    InstrumentationCallbackPreviousPc: usize,
    InstrumentationCallbackPreviousSp: usize,
    TxFsContext: c_ulong,
    InstrumentationCallbackDisabled: c_uchar,
    GdiTebBatch: GDI_TEB_BATCH,
    RealClientId: CLIENT_ID,
    GdiCachedProcessHandle: HANDLE,
    GdiClientPID: c_ulong,
    GdiClientTID: c_ulong,
    GdiThreadLocalInfo: *mut c_void,
    Win32ClientInfo: [usize; 62],
    glDispatchTable: [*mut c_void; 233],
    glReserved1: [usize; 29],
    glReserved2: *mut c_void,
    glSectionInfo: *mut c_void,
    glSection: *mut c_void,
    glTable: *mut c_void,
    glCurrentRC: *mut c_void,
    glContext: *mut c_void,
    LastStatusValue: NTSTATUS,
    StaticUnicodeString: UNICODE_STRING,
    StaticUnicodeBuffer: [wchar_t; 261],
    DeallocationStack: *mut c_void,
    TlsSlots: [*mut c_void; 64],
    TlsLinks: LIST_ENTRY,
    Vdm: *mut c_void,
    ReservedForNtRpc: *mut c_void,
    DbgSsReserved: [*mut c_void; 2],
    HardErrorMode: c_ulong,
    Instrumentation: [*mut c_void; 11],
    ActivityId: GUID,
    SubProcessTag: *mut c_void,
    PerflibData: *mut c_void,
    EtwTraceData: *mut c_void,
    WinSockData: *mut c_void,
    GdiBatchCount: c_ulong,
    u: TEB_u,
    GuaranteedStackBytes: c_ulong,
    ReservedForPerf: *mut c_void,
    ReservedForOle: *mut c_void,
    WaitingOnLoaderLock: c_ulong,
    SavedPriorityState: *mut c_void,
    ReservedForCodeCoverage: usize,
    ThreadPoolData: *mut c_void,
    TlsExpansionSlots: *mut *mut c_void,
    DeallocationBStore: *mut c_void,
    BStoreLimit: *mut c_void,
    MuiGeneration: c_ulong,
    IsImpersonating: c_ulong,
    NlsCache: *mut c_void,
    pShimData: *mut c_void,
    HeapVirtualAffinity: c_ushort,
    LowFragHeapDataSlot: c_ushort,
    CurrentTransactionHandle: HANDLE,
    ActiveFrame: PTEB_ACTIVE_FRAME,
    FlsData: *mut c_void,
    PreferredLanguages: *mut c_void,
    UserPrefLanguages: *mut c_void,
    MergedPrefLanguages: *mut c_void,
    MuiImpersonation: c_ulong,
    CrossTebFlags: c_ushort,
    SameTebFlags: c_ushort,
    TxnScopeEnterCallback: *mut c_void,
    TxnScopeExitCallback: *mut c_void,
    TxnScopeContext: *mut c_void,
    LockCount: c_ulong,
    WowTebOffset: c_long,
    ResourceRetValue: *mut c_void,
    ReservedForWdf: *mut c_void,
    ReservedForCrt: __uint64,
    EffectiveContainerId: GUID,
}}
#[cfg(target_arch = "x86")]
STRUCT! {struct TEB {
    NtTib: NT_TIB,
    EnvironmentPointer: *mut c_void,
    ClientId: CLIENT_ID,
    ActiveRpcHandle: *mut c_void,
    ThreadLocalStoragePointer: *mut c_void,
    ProcessEnvironmentBlock: PPEB,
    LastErrorValue: c_ulong,
    CountOfOwnedCriticalSections: c_ulong,
    CsrClientThread: *mut c_void,
    Win32ThreadInfo: *mut c_void,
    User32Reserved: [c_ulong; 26],
    UserReserved: [c_ulong; 5],
    WOW32Reserved: *mut c_void,
    CurrentLocale: c_ulong,
    FpSoftwareStatusRegister: c_ulong,
    ReservedForDebuggerInstrumentation: [*mut c_void; 16],
    SystemReserved1: [*mut c_void; 26],
    PlaceholderCompatibilityMode: c_char,
    PlaceholderReserved: [c_char; 11],
    ProxiedProcessId: c_ulong,
    ActivationStack: ACTIVATION_CONTEXT_STACK,
    WorkingOnBehalfTicket: [c_uchar; 8],
    ExceptionCode: NTSTATUS,
    ActivationContextStackPointer: PACTIVATION_CONTEXT_STACK,
    InstrumentationCallbackSp: usize,
    InstrumentationCallbackPreviousPc: usize,
    InstrumentationCallbackPreviousSp: usize,
    InstrumentationCallbackDisabled: c_uchar,
    SpareBytes: [c_uchar; 23],
    TxFsContext: c_ulong,
    GdiTebBatch: GDI_TEB_BATCH,
    RealClientId: CLIENT_ID,
    GdiCachedProcessHandle: HANDLE,
    GdiClientPID: c_ulong,
    GdiClientTID: c_ulong,
    GdiThreadLocalInfo: *mut c_void,
    Win32ClientInfo: [usize; 62],
    glDispatchTable: [*mut c_void; 233],
    glReserved1: [usize; 29],
    glReserved2: *mut c_void,
    glSectionInfo: *mut c_void,
    glSection: *mut c_void,
    glTable: *mut c_void,
    glCurrentRC: *mut c_void,
    glContext: *mut c_void,
    LastStatusValue: NTSTATUS,
    StaticUnicodeString: UNICODE_STRING,
    StaticUnicodeBuffer: [wchar_t; 261],
    DeallocationStack: *mut c_void,
    TlsSlots: [*mut c_void; 64],
    TlsLinks: LIST_ENTRY,
    Vdm: *mut c_void,
    ReservedForNtRpc: *mut c_void,
    DbgSsReserved: [*mut c_void; 2],
    HardErrorMode: c_ulong,
    Instrumentation: [*mut c_void; 9],
    ActivityId: GUID,
    SubProcessTag: *mut c_void,
    PerflibData: *mut c_void,
    EtwTraceData: *mut c_void,
    WinSockData: *mut c_void,
    GdiBatchCount: c_ulong,
    u: TEB_u,
    GuaranteedStackBytes: c_ulong,
    ReservedForPerf: *mut c_void,
    ReservedForOle: *mut c_void,
    WaitingOnLoaderLock: c_ulong,
    SavedPriorityState: *mut c_void,
    ReservedForCodeCoverage: usize,
    ThreadPoolData: *mut c_void,
    TlsExpansionSlots: *mut *mut c_void,
    MuiGeneration: c_ulong,
    IsImpersonating: c_ulong,
    NlsCache: *mut c_void,
    pShimData: *mut c_void,
    HeapVirtualAffinity: c_ushort,
    LowFragHeapDataSlot: c_ushort,
    CurrentTransactionHandle: HANDLE,
    ActiveFrame: PTEB_ACTIVE_FRAME,
    FlsData: *mut c_void,
    PreferredLanguages: *mut c_void,
    UserPrefLanguages: *mut c_void,
    MergedPrefLanguages: *mut c_void,
    MuiImpersonation: c_ulong,
    CrossTebFlags: c_ushort,
    SameTebFlags: c_ushort,
    TxnScopeEnterCallback: *mut c_void,
    TxnScopeExitCallback: *mut c_void,
    TxnScopeContext: *mut c_void,
    LockCount: c_ulong,
    WowTebOffset: c_long,
    ResourceRetValue: *mut c_void,
    ReservedForWdf: *mut c_void,
    ReservedForCrt: __uint64,
    EffectiveContainerId: GUID,
}}
BITFIELD! {TEB SameTebFlags: c_ushort [
    SafeThunkCall set_SafeThunkCall[0..1],
    InDebugPrint set_InDebugPrint[1..2],
    HasFiberData set_HasFiberData[2..3],
    SkipThreadAttach set_SkipThreadAttach[3..4],
    WerInShipAssertCode set_WerInShipAssertCode[4..5],
    RanProcessInit set_RanProcessInit[5..6],
    ClonedThread set_ClonedThread[6..7],
    SuppressDebugMsg set_SuppressDebugMsg[7..8],
    DisableUserStackWalk set_DisableUserStackWalk[8..9],
    RtlExceptionAttached set_RtlExceptionAttached[9..10],
    InitialThread set_InitialThread[10..11],
    SessionAware set_SessionAware[11..12],
    LoadOwner set_LoadOwner[12..13],
    LoaderWorker set_LoaderWorker[13..14],
    SkipLoaderInit set_SkipLoaderInit[14..15],
    SpareSameTebBits set_SpareSameTebBits[15..16],
]}
pub type PTEB = *mut TEB;
