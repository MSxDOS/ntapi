use crate::{
    ctypes::{__int64, __uint64, c_char, c_ulong, c_void, wchar_t},
    ntexapi::SYSTEM_MEMORY_LIST_INFORMATION,
    ntmmapi::MMPFN_IDENTITY,
};

ENUM! {enum PF_BOOT_PHASE_ID {
    PfKernelInitPhase = 0,
    PfBootDriverInitPhase = 90,
    PfSystemDriverInitPhase = 120,
    PfSessionManagerInitPhase = 150,
    PfSMRegistryInitPhase = 180,
    PfVideoInitPhase = 210,
    PfPostVideoInitPhase = 240,
    PfBootAcceptedRegistryInitPhase = 270,
    PfUserShellReadyPhase = 300,
    PfMaxBootPhaseId = 900,
}}
ENUM! {enum PF_ENABLE_STATUS {
    PfSvNotSpecified = 0,
    PfSvEnabled = 1,
    PfSvDisabled = 2,
    PfSvMaxEnableStatus = 3,
}}
STRUCT! {struct PF_TRACE_LIMITS {
    MaxNumPages: c_ulong,
    MaxNumSections: c_ulong,
    TimerPeriod: __int64,
}}
pub type PPF_TRACE_LIMITS = *mut PF_TRACE_LIMITS;
STRUCT! {struct PF_SYSTEM_PREFETCH_PARAMETERS {
    EnableStatus: [PF_ENABLE_STATUS; 2],
    TraceLimits: [PF_TRACE_LIMITS; 2],
    MaxNumActiveTraces: c_ulong,
    MaxNumSavedTraces: c_ulong,
    RootDirPath: [wchar_t; 32],
    HostingApplicationList: [wchar_t; 128],
}}
pub type PPF_SYSTEM_PREFETCH_PARAMETERS = *mut PF_SYSTEM_PREFETCH_PARAMETERS;
pub const PF_BOOT_CONTROL_VERSION: u32 = 1;
STRUCT! {struct PF_BOOT_CONTROL {
    Version: c_ulong,
    DisableBootPrefetching: c_ulong,
}}
pub type PPF_BOOT_CONTROL = *mut PF_BOOT_CONTROL;
ENUM! {enum PREFETCHER_INFORMATION_CLASS {
    PrefetcherRetrieveTrace = 1,
    PrefetcherSystemParameters = 2,
    PrefetcherBootPhase = 3,
    PrefetcherRetrieveBootLoaderTrace = 4,
    PrefetcherBootControl = 5,
}}
pub const PREFETCHER_INFORMATION_VERSION: c_ulong = 23;
pub const PREFETCHER_INFORMATION_MAGIC: c_ulong = 0x6b756843;
STRUCT! {struct PREFETCHER_INFORMATION {
    Version: c_ulong,
    Magic: c_ulong,
    PrefetcherInformationClass: PREFETCHER_INFORMATION_CLASS,
    PrefetcherInformation: *mut c_void,
    PrefetcherInformationLength: c_ulong,
}}
pub type PPREFETCHER_INFORMATION = *mut PREFETCHER_INFORMATION;
STRUCT! {struct PF_SYSTEM_SUPERFETCH_PARAMETERS {
    EnabledComponents: c_ulong,
    BootID: c_ulong,
    SavedSectInfoTracesMax: c_ulong,
    SavedPageAccessTracesMax: c_ulong,
    ScenarioPrefetchTimeoutStandby: c_ulong,
    ScenarioPrefetchTimeoutHibernate: c_ulong,
}}
pub type PPF_SYSTEM_SUPERFETCH_PARAMETERS =
    *mut PF_SYSTEM_SUPERFETCH_PARAMETERS;
pub const PF_PFN_PRIO_REQUEST_VERSION: u32 = 1;
pub const PF_PFN_PRIO_REQUEST_QUERY_MEMORY_LIST: u32 = 0x1;
pub const PF_PFN_PRIO_REQUEST_VALID_FLAGS: u32 = 0x1;
STRUCT! {struct PF_PFN_PRIO_REQUEST {
    Version: c_ulong,
    RequestFlags: c_ulong,
    PfnCount: usize,
    MemInfo: SYSTEM_MEMORY_LIST_INFORMATION,
    PageData: [MMPFN_IDENTITY; 256],
}}
pub type PPF_PFN_PRIO_REQUEST = *mut PF_PFN_PRIO_REQUEST;
ENUM! {enum PFS_PRIVATE_PAGE_SOURCE_TYPE {
    PfsPrivateSourceKernel = 0,
    PfsPrivateSourceSession = 1,
    PfsPrivateSourceProcess = 2,
    PfsPrivateSourceMax = 3,
}}
UNION! {union PFS_PRIVATE_PAGE_SOURCE_u {
    SessionId: c_ulong,
    ProcessId: c_ulong,
}}
STRUCT! {struct PFS_PRIVATE_PAGE_SOURCE {
    Type: PFS_PRIVATE_PAGE_SOURCE_TYPE,
    u: PFS_PRIVATE_PAGE_SOURCE_u,
    ImagePathHash: c_ulong,
    UniqueProcessHash: usize,
}}
UNION! {union PF_PRIVSOURCE_INFO_u {
    WsSwapPages: usize,
    SessionPagedPoolPages: usize,
    StoreSizePages: usize,
}}
pub type PPFS_PRIVATE_PAGE_SOURCE = *mut PFS_PRIVATE_PAGE_SOURCE;
STRUCT! {struct PF_PRIVSOURCE_INFO {
    DbInfo: PFS_PRIVATE_PAGE_SOURCE,
    EProcess: *mut c_void,
    WsPrivatePages: usize,
    TotalPrivatePages: usize,
    SessionID: c_ulong,
    ImageName: [c_char; 16],
    u: PF_PRIVSOURCE_INFO_u,
    WsTotalPages: usize,
    DeepFreezeTimeMs: c_ulong,
    BitFields: c_ulong,
}}
BITFIELD! {PF_PRIVSOURCE_INFO BitFields: c_ulong [
    ModernApp set_ModernApp[0..1],
    DeepFrozen set_DeepFrozen[1..2],
    Foreground set_Foreground[2..3],
    PerProcessStore set_PerProcessStore[3..4],
    Spare set_Spare[4..32],
]}
pub type PPF_PRIVSOURCE_INFO = *mut PF_PRIVSOURCE_INFO;
pub const PF_PRIVSOURCE_QUERY_REQUEST_VERSION: u32 = 3;
STRUCT! {struct PF_PRIVSOURCE_QUERY_REQUEST {
    Version: c_ulong,
    Flags: c_ulong,
    InfoCount: c_ulong,
    InfoArray: [PF_PRIVSOURCE_INFO; 1],
}}
pub type PPF_PRIVSOURCE_QUERY_REQUEST = *mut PF_PRIVSOURCE_QUERY_REQUEST;
ENUM! {enum PF_PHASED_SCENARIO_TYPE {
    PfScenarioTypeNone = 0,
    PfScenarioTypeStandby = 1,
    PfScenarioTypeHibernate = 2,
    PfScenarioTypeFUS = 3,
    PfScenarioTypeMax = 4,
}}
pub const PF_SCENARIO_PHASE_INFO_VERSION: u32 = 4;
STRUCT! {struct PF_SCENARIO_PHASE_INFO {
    Version: c_ulong,
    ScenType: PF_PHASED_SCENARIO_TYPE,
    PhaseId: c_ulong,
    SequenceNumber: c_ulong,
    Flags: c_ulong,
    FUSUserId: c_ulong,
}}
pub type PPF_SCENARIO_PHASE_INFO = *mut PF_SCENARIO_PHASE_INFO;
STRUCT! {struct PF_MEMORY_LIST_NODE {
    Bitfields: __uint64,
    StandbyLowPageCount: __uint64,
    StandbyMediumPageCount: __uint64,
    StandbyHighPageCount: __uint64,
    FreePageCount: __uint64,
    ModifiedPageCount: __uint64,
}}
BITFIELD! {PF_MEMORY_LIST_NODE Bitfields: __uint64 [
    Node set_Node[0..8],
    Spare set_Spare[8..64],
]}
pub type PPF_MEMORY_LIST_NODE = *mut PF_MEMORY_LIST_NODE;
pub const PF_MEMORY_LIST_INFO_VERSION: u32 = 1;
STRUCT! {struct PF_MEMORY_LIST_INFO {
    Version: c_ulong,
    Size: c_ulong,
    NodeCount: c_ulong,
    Nodes: [PF_MEMORY_LIST_NODE; 1],
}}
pub type PPF_MEMORY_LIST_INFO = *mut PF_MEMORY_LIST_INFO;
STRUCT! {struct PF_PHYSICAL_MEMORY_RANGE {
    BasePfn: usize,
    PageCount: usize,
}}
pub type PPF_PHYSICAL_MEMORY_RANGE = *mut PF_PHYSICAL_MEMORY_RANGE;
pub const PF_PHYSICAL_MEMORY_RANGE_INFO_VERSION: u32 = 1;
STRUCT! {struct PF_PHYSICAL_MEMORY_RANGE_INFO {
    Version: c_ulong,
    RangeCount: c_ulong,
    Ranges: [PF_PHYSICAL_MEMORY_RANGE; 1],
}}
pub type PPF_PHYSICAL_MEMORY_RANGE_INFO = *mut PF_PHYSICAL_MEMORY_RANGE_INFO;
pub const PF_REPURPOSED_BY_PREFETCH_INFO_VERSION: u32 = 1;
STRUCT! {struct PF_REPURPOSED_BY_PREFETCH_INFO {
    Version: c_ulong,
    RepurposedByPrefetch: c_ulong,
}}
pub type PPF_REPURPOSED_BY_PREFETCH_INFO = *mut PF_REPURPOSED_BY_PREFETCH_INFO;
ENUM! {enum SUPERFETCH_INFORMATION_CLASS {
    SuperfetchRetrieveTrace = 1,
    SuperfetchSystemParameters = 2,
    SuperfetchLogEvent = 3,
    SuperfetchGenerateTrace = 4,
    SuperfetchPrefetch = 5,
    SuperfetchPfnQuery = 6,
    SuperfetchPfnSetPriority = 7,
    SuperfetchPrivSourceQuery = 8,
    SuperfetchSequenceNumberQuery = 9,
    SuperfetchScenarioPhase = 10,
    SuperfetchWorkerPriority = 11,
    SuperfetchScenarioQuery = 12,
    SuperfetchScenarioPrefetch = 13,
    SuperfetchRobustnessControl = 14,
    SuperfetchTimeControl = 15,
    SuperfetchMemoryListQuery = 16,
    SuperfetchMemoryRangesQuery = 17,
    SuperfetchTracingControl = 18,
    SuperfetchTrimWhileAgingControl = 19,
    SuperfetchRepurposedByPrefetch = 20,
    SuperfetchInformationMax = 21,
}}
pub const SUPERFETCH_INFORMATION_VERSION: c_ulong = 45;
pub const SUPERFETCH_INFORMATION_MAGIC: c_ulong = 0x6b756843;
STRUCT! {struct SUPERFETCH_INFORMATION {
    Version: c_ulong,
    Magic: c_ulong,
    InfoClass: SUPERFETCH_INFORMATION_CLASS,
    Data: *mut c_void,
    Length: c_ulong,
}}
pub type PSUPERFETCH_INFORMATION = *mut SUPERFETCH_INFORMATION;
