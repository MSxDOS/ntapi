use windows_sys::{
    core::GUID,
    Win32::{
        Foundation::{NTSTATUS, UNICODE_STRING},
        System::{
            Kernel::{
                LIST_ENTRY32, PROCESSOR_NUMBER, SINGLE_LIST_ENTRY32, STRING32,
            },
            SystemServices::{FLS_MAXIMUM_AVAILABLE, NT_TIB32},
        },
    },
};

use crate::{
    ctypes::{__uint64, c_char, c_long, c_uchar, c_ulong, c_ushort, wchar_t},
    ntapi_base::CLIENT_ID32,
    ntldr::{LDR_DDAG_STATE, LDR_DLL_LOAD_REASON},
    ntpsapi::GDI_HANDLE_BUFFER32,
    ntrtl::RTL_MAX_DRIVE_LETTERS,
    string::{UTF16Const, UTF8Const},
    windows_local::shared::ntdef::{LARGE_INTEGER, ULARGE_INTEGER},
};

pub const WOW64_SYSTEM_DIRECTORY: UTF8Const = UTF8Const("SysWOW64\0");
/// "SysWOW64"
pub const WOW64_SYSTEM_DIRECTORY_U: UTF16Const = UTF16Const(&[
    0x0053, 0x0079, 0x0073, 0x0057, 0x004F, 0x0057, 0x0036, 0x0034, 0u16,
]);
pub const WOW64_X86_TAG: UTF8Const = UTF8Const(" (x86)\0");
/// " (x86)"
pub const WOW64_X86_TAG_U: UTF16Const =
    UTF16Const(&[0x0020, 0x0028, 0x0078, 0x0038, 0x0036, 0x0029, 0u16]);
ENUM! {enum WOW64_SHARED_INFORMATION {
    SharedNtdll32LdrInitializeThunk = 0,
    SharedNtdll32KiUserExceptionDispatcher = 1,
    SharedNtdll32KiUserApcDispatcher = 2,
    SharedNtdll32KiUserCallbackDispatcher = 3,
    SharedNtdll32ExpInterlockedPopEntrySListFault = 4,
    SharedNtdll32ExpInterlockedPopEntrySListResume = 5,
    SharedNtdll32ExpInterlockedPopEntrySListEnd = 6,
    SharedNtdll32RtlUserThreadStart = 7,
    SharedNtdll32pQueryProcessDebugInformationRemote = 8,
    SharedNtdll32BaseAddress = 9,
    SharedNtdll32LdrSystemDllInitBlock = 10,
    Wow64SharedPageEntriesCount = 11,
}}
STRUCT! {struct RTL_BALANCED_NODE32_u_s {
    Left: c_ulong, // WOW64_POINTER
    Right: c_ulong, // WOW64_POINTER
}}
UNION! {union RTL_BALANCED_NODE32_u {
    Children: [c_ulong; 2], // WOW64_POINTER
    s: RTL_BALANCED_NODE32_u_s,
}}
STRUCT! {struct RTL_BALANCED_NODE32 {
    u: RTL_BALANCED_NODE32_u,
    ParentValue: c_ulong,
}}
pub type PRTL_BALANCED_NODE32 = *mut RTL_BALANCED_NODE32;
STRUCT! {struct RTL_RB_TREE32 {
    Root: c_ulong, // WOW64_POINTER
    Min: c_ulong, // WOW64_POINTER
}}
pub type PRTL_RB_TREE32 = *mut RTL_RB_TREE32;
STRUCT! {struct PEB_LDR_DATA32 {
    Length: c_ulong,
    Initialized: c_uchar,
    SsHandle: c_ulong,
    InLoadOrderModuleList: LIST_ENTRY32,
    InMemoryOrderModuleList: LIST_ENTRY32,
    InInitializationOrderModuleList: LIST_ENTRY32,
    EntryInProgress: c_ulong,
    ShutdownInProgress: c_uchar,
    ShutdownThreadId: c_ulong,
}}
pub type PPEB_LDR_DATA32 = *mut PEB_LDR_DATA32;
STRUCT! {struct LDR_SERVICE_TAG_RECORD32 {
    Next: c_ulong,
    ServiceTag: c_ulong,
}}
pub type PLDR_SERVICE_TAG_RECORD32 = *mut LDR_SERVICE_TAG_RECORD32;
STRUCT! {struct LDRP_CSLIST32 {
    Tail: c_ulong, // WOW64_POINTER
}}
pub type PLDRP_CSLIST32 = *mut LDRP_CSLIST32;
UNION! {union LDR_DDAG_NODE32_u {
    Dependencies: LDRP_CSLIST32,
    RemovalLink: SINGLE_LIST_ENTRY32,
}}
STRUCT! {struct LDR_DDAG_NODE32 {
    Modules: LIST_ENTRY32,
    ServiceTagList: c_ulong, // WOW64_POINTER
    LoadCount: c_ulong,
    LoadWhileUnloadingCount: c_ulong,
    LowestLink: c_ulong,
    u: LDR_DDAG_NODE32_u,
    IncomingDependencies: LDRP_CSLIST32,
    State: LDR_DDAG_STATE,
    CondenseLink: SINGLE_LIST_ENTRY32,
    PreorderNumber: c_ulong,
}}
pub type PLDR_DDAG_NODE32 = *mut LDR_DDAG_NODE32;
pub const LDR_DATA_TABLE_ENTRY_SIZE_WINXP_32: usize = 80;
pub const LDR_DATA_TABLE_ENTRY_SIZE_WIN7_32: usize = 144;
pub const LDR_DATA_TABLE_ENTRY_SIZE_WIN8_32: usize = 152;
UNION! {union LDR_DATA_TABLE_ENTRY32_u1 {
    InInitializationOrderLinks: LIST_ENTRY32,
    InProgressLinks: LIST_ENTRY32,
}}
UNION! {union LDR_DATA_TABLE_ENTRY32_u2 {
    FlagGroup: [c_uchar; 4],
    Flags: c_ulong,
}}
STRUCT! {struct LDR_DATA_TABLE_ENTRY32 {
    InLoadOrderLinks: LIST_ENTRY32,
    InMemoryOrderLinks: LIST_ENTRY32,
    u1: LDR_DATA_TABLE_ENTRY32_u1,
    DllBase: c_ulong, // WOW64_POINTER
    EntryPoint: c_ulong, // WOW64_POINTER
    SizeOfImage: c_ulong,
    FullDllName: STRING32,
    BaseDllName: STRING32,
    u2: LDR_DATA_TABLE_ENTRY32_u2,
    ObsoleteLoadCount: c_ushort,
    TlsIndex: c_ushort,
    HashLinks: LIST_ENTRY32,
    TimeDateStamp: c_ulong,
    EntryPointActivationContext: c_ulong, // WOW64_POINTER
    Lock: c_ulong, // WOW64_POINTER
    DdagNode: c_ulong, // WOW64_POINTER
    NodeModuleLink: LIST_ENTRY32,
    LoadContext: c_ulong, // WOW64_POINTER
    ParentDllBase: c_ulong, // WOW64_POINTER
    SwitchBackContext: c_ulong, // WOW64_POINTER
    BaseAddressIndexNode: RTL_BALANCED_NODE32,
    MappingInfoIndexNode: RTL_BALANCED_NODE32,
    OriginalBase: c_ulong,
    LoadTime: LARGE_INTEGER,
    BaseNameHashValue: c_ulong,
    LoadReason: LDR_DLL_LOAD_REASON,
    ImplicitPathOptions: c_ulong,
    ReferenceCount: c_ulong,
    DependentLoadFlags: c_ulong,
    SigningLevel: c_uchar,
}}
BITFIELD! {unsafe LDR_DATA_TABLE_ENTRY32_u2 Flags: c_ulong [
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
pub type PLDR_DATA_TABLE_ENTRY32 = *mut LDR_DATA_TABLE_ENTRY32;
STRUCT! {struct CURDIR32 {
    DosPath: STRING32,
    Handle: c_ulong, // WOW64_POINTER
}}
pub type PCURDIR32 = *mut CURDIR32;
STRUCT! {struct RTL_DRIVE_LETTER_CURDIR32 {
    Flags: c_ushort,
    Length: c_ushort,
    TimeStamp: c_ulong,
    DosPath: STRING32,
}}
pub type PRTL_DRIVE_LETTER_CURDIR32 = *mut RTL_DRIVE_LETTER_CURDIR32;
STRUCT! {struct RTL_USER_PROCESS_PARAMETERS32 {
    MaximumLength: c_ulong,
    Length: c_ulong,
    Flags: c_ulong,
    DebugFlags: c_ulong,
    ConsoleHandle: c_ulong, // WOW64_POINTER
    ConsoleFlags: c_ulong,
    StandardInput: c_ulong, // WOW64_POINTER
    StandardOutput: c_ulong, // WOW64_POINTER
    StandardError: c_ulong, // WOW64_POINTER
    CurrentDirectory: CURDIR32,
    DllPath: STRING32,
    ImagePathName: STRING32,
    CommandLine: STRING32,
    Environment: c_ulong, // WOW64_POINTER
    StartingX: c_ulong,
    StartingY: c_ulong,
    CountX: c_ulong,
    CountY: c_ulong,
    CountCharsX: c_ulong,
    CountCharsY: c_ulong,
    FillAttribute: c_ulong,
    WindowFlags: c_ulong,
    ShowWindowFlags: c_ulong,
    WindowTitle: STRING32,
    DesktopInfo: STRING32,
    ShellInfo: STRING32,
    RuntimeData: STRING32,
    CurrentDirectories: [RTL_DRIVE_LETTER_CURDIR32; RTL_MAX_DRIVE_LETTERS],
    EnvironmentSize: c_ulong,
    EnvironmentVersion: c_ulong,
    PackageDependencyData: c_ulong, // WOW64_POINTER
    ProcessGroupId: c_ulong,
    LoaderThreads: c_ulong,
}}
pub type PRTL_USER_PROCESS_PARAMETERS32 = *mut RTL_USER_PROCESS_PARAMETERS32;
UNION! {union PEB32_u {
    KernelCallbackTable: c_ulong, // WOW64_POINTER
    UserSharedInfoPtr: c_ulong, // WOW64_POINTER
}}
STRUCT! {struct PEB32 {
    InheritedAddressSpace: c_uchar,
    ReadImageFileExecOptions: c_uchar,
    BeingDebugged: c_uchar,
    BitField: c_uchar,
    Mutant: c_ulong, // WOW64_POINTER
    ImageBaseAddress: c_ulong, // WOW64_POINTER
    Ldr: c_ulong, // WOW64_POINTER
    ProcessParameters: c_ulong, // WOW64_POINTER
    SubSystemData: c_ulong, // WOW64_POINTER
    ProcessHeap: c_ulong, // WOW64_POINTER
    FastPebLock: c_ulong, // WOW64_POINTER
    AtlThunkSListPtr: c_ulong, // WOW64_POINTER
    IFEOKey: c_ulong, // WOW64_POINTER
    CrossProcessFlags: c_ulong,
    u: PEB32_u,
    SystemReserved: [c_ulong; 1],
    AtlThunkSListPtr32: c_ulong,
    ApiSetMap: c_ulong, // WOW64_POINTER
    TlsExpansionCounter: c_ulong,
    TlsBitmap: c_ulong, // WOW64_POINTER
    TlsBitmapBits: [c_ulong; 2],
    ReadOnlySharedMemoryBase: c_ulong, // WOW64_POINTER
    HotpatchInformation: c_ulong, // WOW64_POINTER
    ReadOnlyStaticServerData: c_ulong, // WOW64_POINTER
    AnsiCodePageData: c_ulong, // WOW64_POINTER
    OemCodePageData: c_ulong, // WOW64_POINTER
    UnicodeCaseTableData: c_ulong, // WOW64_POINTER
    NumberOfProcessors: c_ulong,
    NtGlobalFlag: c_ulong,
    CriticalSectionTimeout: LARGE_INTEGER,
    HeapSegmentReserve: c_ulong,
    HeapSegmentCommit: c_ulong,
    HeapDeCommitTotalFreeThreshold: c_ulong,
    HeapDeCommitFreeBlockThreshold: c_ulong,
    NumberOfHeaps: c_ulong,
    MaximumNumberOfHeaps: c_ulong,
    ProcessHeaps: c_ulong, // WOW64_POINTER
    GdiSharedHandleTable: c_ulong, // WOW64_POINTER
    ProcessStarterHelper: c_ulong, // WOW64_POINTER
    GdiDCAttributeList: c_ulong,
    LoaderLock: c_ulong, // WOW64_POINTER
    OSMajorVersion: c_ulong,
    OSMinorVersion: c_ulong,
    OSBuildNumber: c_ushort,
    OSCSDVersion: c_ushort,
    OSPlatformId: c_ulong,
    ImageSubsystem: c_ulong,
    ImageSubsystemMajorVersion: c_ulong,
    ImageSubsystemMinorVersion: c_ulong,
    ActiveProcessAffinityMask: c_ulong,
    GdiHandleBuffer: GDI_HANDLE_BUFFER32,
    PostProcessInitRoutine: c_ulong, // WOW64_POINTER
    TlsExpansionBitmap: c_ulong, // WOW64_POINTER
    TlsExpansionBitmapBits: [c_ulong; 32],
    SessionId: c_ulong,
    AppCompatFlags: ULARGE_INTEGER,
    AppCompatFlagsUser: ULARGE_INTEGER,
    pShimData: c_ulong, // WOW64_POINTER
    AppCompatInfo: c_ulong, // WOW64_POINTER
    CSDVersion: STRING32,
    ActivationContextData: c_ulong, // WOW64_POINTER
    ProcessAssemblyStorageMap: c_ulong, // WOW64_POINTER
    SystemDefaultActivationContextData: c_ulong, // WOW64_POINTER
    SystemAssemblyStorageMap: c_ulong, // WOW64_POINTER
    MinimumStackCommit: c_ulong,
    FlsCallback: c_ulong, // WOW64_POINTER
    FlsListHead: LIST_ENTRY32,
    FlsBitmap: c_ulong, // WOW64_POINTER
    FlsBitmapBits: [c_ulong; FLS_MAXIMUM_AVAILABLE as usize / c_ulong::BITS as usize],
    FlsHighIndex: c_ulong,
    WerRegistrationData: c_ulong, // WOW64_POINTER
    WerShipAssertPtr: c_ulong, // WOW64_POINTER
    pContextData: c_ulong, // WOW64_POINTER
    pImageHeaderHash: c_ulong, // WOW64_POINTER
    TracingFlags: c_ulong,
    CsrServerReadOnlySharedMemoryBase: __uint64,
    TppWorkerpListLock: c_ulong, // WOW64_POINTER
    TppWorkerpList: LIST_ENTRY32,
    WaitOnAddressHashTable: [c_ulong; 128], // WOW64_POINTER
    TelemetryCoverageHeader: c_ulong, // WOW64_POINTER
    CloudFileFlags: c_ulong,
    CloudFileDiagFlags: c_ulong,
    PlaceholderCompatibilityMode: c_char,
    PlaceholderCompatibilityModeReserved: [c_char; 7],
}}
BITFIELD! {PEB32 BitField: c_uchar [
    ImageUsesLargePages set_ImageUsesLargePages[0..1],
    IsProtectedProcess set_IsProtectedProcess[1..2],
    IsImageDynamicallyRelocated set_IsImageDynamicallyRelocated[2..3],
    SkipPatchingUser32Forwarders set_SkipPatchingUser32Forwarders[3..4],
    IsPackagedProcess set_IsPackagedProcess[4..5],
    IsAppContainer set_IsAppContainer[5..6],
    IsProtectedProcessLight set_IsProtectedProcessLight[6..7],
    IsLongPathAwareProcess set_IsLongPathAwareProcess[7..8],
]}
BITFIELD! {PEB32 CrossProcessFlags: c_ulong [
    ProcessInJob set_ProcessInJob[0..1],
    ProcessInitializing set_ProcessInitializing[1..2],
    ProcessUsingVEH set_ProcessUsingVEH[2..3],
    ProcessUsingVCH set_ProcessUsingVCH[3..4],
    ProcessUsingFTH set_ProcessUsingFTH[4..5],
    ReservedBits0 set_ReservedBits0[5..32],
]}
BITFIELD! {PEB32 TracingFlags: c_ulong [
    HeapTracingEnabled set_HeapTracingEnabled[0..1],
    CritSecTracingEnabled set_CritSecTracingEnabled[1..2],
    LibLoaderTracingEnabled set_LibLoaderTracingEnabled[2..3],
    SpareTracingBits set_SpareTracingBits[3..32],
]}
pub type PPEB32 = *mut PEB32;
pub const GDI_BATCH_BUFFER_SIZE: usize = 310;
STRUCT! {struct GDI_TEB_BATCH32 {
    Offset: c_ulong,
    HDC: c_ulong,
    Buffer: [c_ulong; GDI_BATCH_BUFFER_SIZE],
}}
pub type PGDI_TEB_BATCH32 = *mut GDI_TEB_BATCH32;
STRUCT! {struct TEB32_u_s {
    ReservedPad0: c_uchar,
    ReservedPad1: c_uchar,
    ReservedPad2: c_uchar,
    IdealProcessor: c_uchar,
}}
UNION! {union TEB32_u {
    CurrentIdealProcessor: PROCESSOR_NUMBER,
    IdealProcessorValue: c_ulong,
    s: TEB32_u_s,
}}
STRUCT! {struct TEB32 {
    NtTib: NT_TIB32,
    EnvironmentPointer: c_ulong, // WOW64_POINTER
    ClientId: CLIENT_ID32,
    ActiveRpcHandle: c_ulong, // WOW64_POINTER
    ThreadLocalStoragePointer: c_ulong, // WOW64_POINTER
    ProcessEnvironmentBlock: c_ulong, // WOW64_POINTER
    LastErrorValue: c_ulong,
    CountOfOwnedCriticalSections: c_ulong,
    CsrClientThread: c_ulong, // WOW64_POINTER
    Win32ThreadInfo: c_ulong, // WOW64_POINTER
    User32Reserved: [c_ulong; 26],
    UserReserved: [c_ulong; 5],
    WOW32Reserved: c_ulong, // WOW64_POINTER
    CurrentLocale: c_ulong,
    FpSoftwareStatusRegister: c_ulong,
    ReservedForDebuggerInstrumentation: [c_ulong; 16], // WOW64_POINTER
    SystemReserved1: [c_ulong; 36], // WOW64_POINTER
    WorkingOnBehalfTicket: [c_uchar; 8],
    ExceptionCode: NTSTATUS,
    ActivationContextStackPointer: c_ulong, // WOW64_POINTER
    InstrumentationCallbackSp: c_ulong,
    InstrumentationCallbackPreviousPc: c_ulong,
    InstrumentationCallbackPreviousSp: c_ulong,
    InstrumentationCallbackDisabled: c_uchar,
    SpareBytes: [c_uchar; 23],
    TxFsContext: c_ulong,
    GdiTebBatch: GDI_TEB_BATCH32,
    RealClientId: CLIENT_ID32,
    GdiCachedProcessHandle: c_ulong, // WOW64_POINTER
    GdiClientPID: c_ulong,
    GdiClientTID: c_ulong,
    GdiThreadLocalInfo: c_ulong, // WOW64_POINTER
    Win32ClientInfo: [c_ulong; 62],
    glDispatchTable: [c_ulong; 233], // WOW64_POINTER
    glReserved1: [c_ulong; 29], // WOW64_POINTER
    glReserved2: c_ulong, // WOW64_POINTER
    glSectionInfo: c_ulong, // WOW64_POINTER
    glSection: c_ulong, // WOW64_POINTER
    glTable: c_ulong, // WOW64_POINTER
    glCurrentRC: c_ulong, // WOW64_POINTER
    glContext: c_ulong, // WOW64_POINTER
    LastStatusValue: NTSTATUS,
    StaticUnicodeString: STRING32,
    StaticUnicodeBuffer: [wchar_t; 261],
    DeallocationStack: c_ulong, // WOW64_POINTER
    TlsSlots: [c_ulong; 64], // WOW64_POINTER
    TlsLinks: LIST_ENTRY32,
    Vdm: c_ulong, // WOW64_POINTER
    ReservedForNtRpc: c_ulong, // WOW64_POINTER
    DbgSsReserved: [c_ulong; 2], // WOW64_POINTER
    HardErrorMode: c_ulong,
    Instrumentation: [c_ulong; 9], // WOW64_POINTER
    ActivityId: GUID,
    SubProcessTag: c_ulong, // WOW64_POINTER
    PerflibData: c_ulong, // WOW64_POINTER
    EtwTraceData: c_ulong, // WOW64_POINTER
    WinSockData: c_ulong, // WOW64_POINTER
    GdiBatchCount: c_ulong,
    u: TEB32_u,
    GuaranteedStackBytes: c_ulong,
    ReservedForPerf: c_ulong, // WOW64_POINTER
    ReservedForOle: c_ulong, // WOW64_POINTER
    WaitingOnLoaderLock: c_ulong,
    SavedPriorityState: c_ulong, // WOW64_POINTER
    ReservedForCodeCoverage: c_ulong,
    ThreadPoolData: c_ulong, // WOW64_POINTER
    TlsExpansionSlots: c_ulong, // WOW64_POINTER
    MuiGeneration: c_ulong,
    IsImpersonating: c_ulong,
    NlsCache: c_ulong, // WOW64_POINTER
    pShimData: c_ulong, // WOW64_POINTER
    HeapVirtualAffinity: c_ushort,
    LowFragHeapDataSlot: c_ushort,
    CurrentTransactionHandle: c_ulong, // WOW64_POINTER
    ActiveFrame: c_ulong, // WOW64_POINTER
    FlsData: c_ulong, // WOW64_POINTER
    PreferredLanguages: c_ulong, // WOW64_POINTER
    UserPrefLanguages: c_ulong, // WOW64_POINTER
    MergedPrefLanguages: c_ulong, // WOW64_POINTER
    MuiImpersonation: c_ulong,
    CrossTebFlags: c_ushort,
    SameTebFlags: c_ushort,
    TxnScopeEnterCallback: c_ulong, // WOW64_POINTER
    TxnScopeExitCallback: c_ulong, // WOW64_POINTER
    TxnScopeContext: c_ulong, // WOW64_POINTER
    LockCount: c_ulong,
    WowTebOffset: c_long,
    ResourceRetValue: c_ulong, // WOW64_POINTER
    ReservedForWdf: c_ulong, // WOW64_POINTER
    ReservedForCrt: __uint64,
    EffectiveContainerId: GUID,
}}
BITFIELD! {TEB32 SameTebFlags: c_ushort [
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
    SpareSameTebBits set_SpareSameTebBits[14..16],
]}
pub type PTEB32 = *mut TEB32;
#[inline]
pub fn UStr32ToUStr(Destination: &mut UNICODE_STRING, Source: &STRING32) {
    Destination.Length = Source.Length;
    Destination.MaximumLength = Source.MaximumLength;
    Destination.Buffer = Source.Buffer as *mut u16;
}
#[inline]
pub fn UStrToUStr32(Destination: &mut STRING32, Source: &UNICODE_STRING) {
    Destination.Length = Source.Length;
    Destination.MaximumLength = Source.MaximumLength;
    Destination.Buffer = Source.Buffer as u32;
}
