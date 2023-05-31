use windows_sys::Win32::{
    Foundation::{HANDLE, NTSTATUS, UNICODE_STRING},
    Storage::FileSystem::{STANDARD_RIGHTS_REQUIRED, SYNCHRONIZE},
    System::{
        Memory::CFG_CALL_TARGET_INFO, WindowsProgramming::OBJECT_ATTRIBUTES,
    },
};

use crate::{
    ctypes::{__uint64, c_uchar, c_ulong, c_ushort, c_void},
    windows_local::{
        shared::ntdef::LARGE_INTEGER, um::winnt::PMEM_EXTENDED_PARAMETER,
    },
};

ENUM! {enum MEMORY_INFORMATION_CLASS {
    MemoryBasicInformation = 0,
    MemoryWorkingSetInformation = 1,
    MemoryMappedFilenameInformation = 2,
    MemoryRegionInformation = 3,
    MemoryWorkingSetExInformation = 4,
    MemorySharedCommitInformation = 5,
    MemoryImageInformation = 6,
    MemoryRegionInformationEx = 7,
    MemoryPrivilegedBasicInformation = 8,
    MemoryEnclaveImageInformation = 9,
    MemoryBasicInformationCapped = 10,
}}
STRUCT! {struct MEMORY_WORKING_SET_BLOCK {
    Bitfields: usize,
}}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
BITFIELD! {MEMORY_WORKING_SET_BLOCK Bitfields: usize [
    Protection set_Protection[0..5],
    ShareCount set_ShareCount[5..8],
    Shared set_Shared[8..9],
    Node set_Node[9..12],
    VirtualPage set_VirtualPage[12..64],
]}
#[cfg(target_arch = "x86")]
BITFIELD! {MEMORY_WORKING_SET_BLOCK Bitfields: usize [
    Protection set_Protection[0..5],
    ShareCount set_ShareCount[5..8],
    Shared set_Shared[8..9],
    Node set_Node[9..12],
    VirtualPage set_VirtualPage[12..32],
]}
pub type PMEMORY_WORKING_SET_BLOCK = *mut MEMORY_WORKING_SET_BLOCK;
STRUCT! {struct MEMORY_WORKING_SET_INFORMATION {
    NumberOfEntries: usize,
    WorkingSetInfo: [MEMORY_WORKING_SET_BLOCK; 1],
}}
pub type PMEMORY_WORKING_SET_INFORMATION = *mut MEMORY_WORKING_SET_INFORMATION;
STRUCT! {struct MEMORY_REGION_INFORMATION {
    AllocationBase: *mut c_void,
    AllocationProtect: c_ulong,
    RegionType: c_ulong,
    RegionSize: usize,
    CommitSize: usize,
}}
BITFIELD! {MEMORY_REGION_INFORMATION RegionType: c_ulong [
    Private set_Private[0..1],
    MappedDataFile set_MappedDataFile[1..2],
    MappedImage set_MappedImage[2..3],
    MappedPageFile set_MappedPageFile[3..4],
    MappedPhysical set_MappedPhysical[4..5],
    DirectMapped set_DirectMapped[5..6],
    SoftwareEnclave set_SoftwareEnclave[6..7],
    PageSize64K set_PageSize64K[7..8],
    PlaceholderReservation set_PlaceholderReservation[8..9],
    Reserved set_Reserved[9..32],
]}
pub type PMEMORY_REGION_INFORMATION = *mut MEMORY_REGION_INFORMATION;
ENUM! {enum MEMORY_WORKING_SET_EX_LOCATION {
    MemoryLocationInvalid = 0,
    MemoryLocationResident = 1,
    MemoryLocationPagefile = 2,
    MemoryLocationReserved = 3,
}}
UNION! {union MEMORY_WORKING_SET_EX_BLOCK_u {
    Bitfields: usize,
    Invalid: usize,
}}
STRUCT! {struct MEMORY_WORKING_SET_EX_BLOCK {
    u: MEMORY_WORKING_SET_EX_BLOCK_u,
}}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
BITFIELD! {unsafe MEMORY_WORKING_SET_EX_BLOCK_u Bitfields: usize [
    Valid set_Valid[0..1],
    ShareCount set_ShareCount[1..4],
    Win32Protection set_Win32Protection[4..15],
    Shared set_Shared[15..16],
    Node set_Node[16..22],
    Locked set_Locked[22..23],
    LargePage set_LargePage[23..24],
    Priority set_Priority[24..27],
    Reserved set_Reserved[27..30],
    SharedOriginal set_SharedOriginal[30..31],
    Bad set_Bad[31..32],
    ReservedUlong set_ReservedUlong[32..64],
]}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
BITFIELD! {unsafe MEMORY_WORKING_SET_EX_BLOCK_u Invalid: usize [
    Invalid_Valid set_Invalid_Valid[0..1],
    Invalid_Reserved0 set_Invalid_Reserved0[1..15],
    Invalid_Shared set_Invalid_Shared[15..16],
    Invalid_Reserved1 set_Invalid_Reserved1[16..21],
    Invalid_PageTable set_Invalid_PageTable[21..22],
    Invalid_Location set_Invalid_Location[22..24],
    Invalid_Priority set_Invalid_Priority[24..27],
    Invalid_ModifiedList set_Invalid_ModifiedList[27..28],
    Invalid_Reserved2 set_Invalid_Reserved2[28..30],
    Invalid_SharedOriginal set_Invalid_SharedOriginal[30..31],
    Invalid_Bad set_Invalid_Bad[31..32],
    Invalid_ReservedUlong set_Invalid_ReservedUlong[32..64],
]}
#[cfg(target_arch = "x86")]
BITFIELD! {unsafe MEMORY_WORKING_SET_EX_BLOCK_u Bitfields: usize [
    Valid set_Valid[0..1],
    ShareCount set_ShareCount[1..4],
    Win32Protection set_Win32Protection[4..15],
    Shared set_Shared[15..16],
    Node set_Node[16..22],
    Locked set_Locked[22..23],
    LargePage set_LargePage[23..24],
    Priority set_Priority[24..27],
    Reserved set_Reserved[27..30],
    SharedOriginal set_SharedOriginal[30..31],
    Bad set_Bad[31..32],
]}
#[cfg(target_arch = "x86")]
BITFIELD! {unsafe MEMORY_WORKING_SET_EX_BLOCK_u Invalid: usize [
    Invalid_Valid set_Invalid_Valid[0..1],
    Invalid_Reserved0 set_Invalid_Reserved0[1..15],
    Invalid_Shared set_Invalid_Shared[15..16],
    Invalid_Reserved1 set_Invalid_Reserved1[16..21],
    Invalid_PageTable set_Invalid_PageTable[21..22],
    Invalid_Location set_Invalid_Location[22..24],
    Invalid_Priority set_Invalid_Priority[24..27],
    Invalid_ModifiedList set_Invalid_ModifiedList[27..28],
    Invalid_Reserved2 set_Invalid_Reserved2[28..30],
    Invalid_SharedOriginal set_Invalid_SharedOriginal[30..31],
    Invalid_Bad set_Invalid_Bad[31..32],
]}
pub type PMEMORY_WORKING_SET_EX_BLOCK = *mut MEMORY_WORKING_SET_EX_BLOCK;
STRUCT! {struct MEMORY_WORKING_SET_EX_INFORMATION {
    VirtualAddress: *mut c_void,
    VirtualAttributes: MEMORY_WORKING_SET_EX_BLOCK,
}}
pub type PMEMORY_WORKING_SET_EX_INFORMATION =
    *mut MEMORY_WORKING_SET_EX_INFORMATION;
STRUCT! {struct MEMORY_SHARED_COMMIT_INFORMATION {
    CommitSize: usize,
}}
pub type PMEMORY_SHARED_COMMIT_INFORMATION =
    *mut MEMORY_SHARED_COMMIT_INFORMATION;
STRUCT! {struct MEMORY_IMAGE_INFORMATION {
    ImageBase: *mut c_void,
    SizeOfImage: usize,
    ImageFlags: c_ulong,
}}
BITFIELD! {MEMORY_IMAGE_INFORMATION ImageFlags: c_ulong [
    ImagePartialMap set_ImagePartialMap[0..1],
    ImageNotExecutable set_ImageNotExecutable[1..2],
    ImageSigningLevel set_ImageSigningLevel[2..6],
    Reserved set_Reserved[6..32],
]}
pub type PMEMORY_IMAGE_INFORMATION = *mut MEMORY_IMAGE_INFORMATION;
STRUCT! {struct MEMORY_ENCLAVE_IMAGE_INFORMATION {
    ImageInfo: MEMORY_IMAGE_INFORMATION,
    UniqueID: [c_uchar; 32],
    AuthorID: [c_uchar; 32],
}}
pub type PMEMORY_ENCLAVE_IMAGE_INFORMATION =
    *mut MEMORY_ENCLAVE_IMAGE_INFORMATION;
pub const MMPFNLIST_ZERO: u32 = 0;
pub const MMPFNLIST_FREE: u32 = 1;
pub const MMPFNLIST_STANDBY: u32 = 2;
pub const MMPFNLIST_MODIFIED: u32 = 3;
pub const MMPFNLIST_MODIFIEDNOWRITE: u32 = 4;
pub const MMPFNLIST_BAD: u32 = 5;
pub const MMPFNLIST_ACTIVE: u32 = 6;
pub const MMPFNLIST_TRANSITION: u32 = 7;
pub const MMPFNUSE_PROCESSPRIVATE: u32 = 0;
pub const MMPFNUSE_FILE: u32 = 1;
pub const MMPFNUSE_PAGEFILEMAPPED: u32 = 2;
pub const MMPFNUSE_PAGETABLE: u32 = 3;
pub const MMPFNUSE_PAGEDPOOL: u32 = 4;
pub const MMPFNUSE_NONPAGEDPOOL: u32 = 5;
pub const MMPFNUSE_SYSTEMPTE: u32 = 6;
pub const MMPFNUSE_SESSIONPRIVATE: u32 = 7;
pub const MMPFNUSE_METAFILE: u32 = 8;
pub const MMPFNUSE_AWEPAGE: u32 = 9;
pub const MMPFNUSE_DRIVERLOCKPAGE: u32 = 10;
pub const MMPFNUSE_KERNELSTACK: u32 = 11;
STRUCT! {struct MEMORY_FRAME_INFORMATION {
    Bitfields: __uint64,
}}
BITFIELD! {MEMORY_FRAME_INFORMATION Bitfields: __uint64 [
    UseDescription set_UseDescription[0..4],
    ListDescription set_ListDescription[4..7],
    Reserved0 set_Reserved0[7..8],
    Pinned set_Pinned[8..9],
    DontUse set_DontUse[9..57],
    Priority set_Priority[57..60],
    Reserved set_Reserved[60..64],
]}
STRUCT! {struct FILEOFFSET_INFORMATION {
    Bitfields: __uint64,
}}
BITFIELD! {FILEOFFSET_INFORMATION Bitfields: __uint64 [
    DontUse set_DontUse[0..9],
    Offset set_Offset[9..57],
    Reserved set_Reserved[57..64],
]}
STRUCT! {struct PAGEDIR_INFORMATION {
    Bitfields: __uint64,
}}
BITFIELD! {PAGEDIR_INFORMATION Bitfields: __uint64 [
    DontUse set_DontUse[0..9],
    PageDirectoryBase set_PageDirectoryBase[9..57],
    Reserved set_Reserved[57..64],
]}
STRUCT! {struct UNIQUE_PROCESS_INFORMATION {
    Bitfields: __uint64,
}}
BITFIELD! {UNIQUE_PROCESS_INFORMATION Bitfields: __uint64 [
    DontUse set_DontUse[0..9],
    UniqueProcessKey set_UniqueProcessKey[9..57],
    Reserved set_Reserved[57..64],
]}
pub type PUNIQUE_PROCESS_INFORMATION = *mut UNIQUE_PROCESS_INFORMATION;
UNION! {union MMPFN_IDENTITY_u1 {
    e1: MEMORY_FRAME_INFORMATION,
    e2: FILEOFFSET_INFORMATION,
    e3: PAGEDIR_INFORMATION,
    e4: UNIQUE_PROCESS_INFORMATION,
}}
UNION! {union MMPFN_IDENTITY_u2 {
    e1: usize,
    e2_CombinedPage: usize,
    FileObject: usize,
    UniqueFileObjectKey: usize,
    ProtoPteAddress: usize,
    VirtualAddress: usize,
}}
STRUCT! {struct MMPFN_IDENTITY {
    u1: MMPFN_IDENTITY_u1,
    PageFrameIndex: usize,
    u2: MMPFN_IDENTITY_u2,
}}
BITFIELD! {unsafe MMPFN_IDENTITY_u2 e1: usize [
    Image set_Image[0..1],
    Mismatch set_Mismatch[1..2],
]}
pub type PMMPFN_IDENTITY = *mut MMPFN_IDENTITY;
STRUCT! {struct MMPFN_MEMSNAP_INFORMATION {
    InitialPageFrameIndex: usize,
    Count: usize,
}}
pub type PMMPFN_MEMSNAP_INFORMATION = *mut MMPFN_MEMSNAP_INFORMATION;
ENUM! {enum SECTION_INFORMATION_CLASS {
    SectionBasicInformation = 0,
    SectionImageInformation = 1,
    SectionRelocationInformation = 2,
    SectionOriginalBaseInformation = 3,
    SectionInternalImageInformation = 4,
    MaxSectionInfoClass = 5,
}}
STRUCT! {struct SECTION_BASIC_INFORMATION {
    BaseAddress: *mut c_void,
    AllocationAttributes: c_ulong,
    MaximumSize: LARGE_INTEGER,
}}
pub type PSECTION_BASIC_INFORMATION = *mut SECTION_BASIC_INFORMATION;
STRUCT! {struct SECTION_IMAGE_INFORMATION_u1_s {
    SubSystemMinorVersion: c_ushort,
    SubSystemMajorVersion: c_ushort,
}}
UNION! {union SECTION_IMAGE_INFORMATION_u1 {
    s: SECTION_IMAGE_INFORMATION_u1_s,
    SubSystemVersion: c_ulong,
}}
STRUCT! {struct SECTION_IMAGE_INFORMATION_u2_s {
    MajorOperatingSystemVersion: c_ushort,
    MinorOperatingSystemVersion: c_ushort,
}}
UNION! {union SECTION_IMAGE_INFORMATION_u2 {
    s: SECTION_IMAGE_INFORMATION_u2_s,
    OperatingSystemVersion: c_ulong,
}}
STRUCT! {struct SECTION_IMAGE_INFORMATION {
    TransferAddress: *mut c_void,
    ZeroBits: c_ulong,
    MaximumStackSize: usize,
    CommittedStackSize: usize,
    SubSystemType: c_ulong,
    u1: SECTION_IMAGE_INFORMATION_u1,
    u2: SECTION_IMAGE_INFORMATION_u2,
    ImageCharacteristics: c_ushort,
    DllCharacteristics: c_ushort,
    Machine: c_ushort,
    ImageContainsCode: c_uchar,
    ImageFlags: c_uchar,
    LoaderFlags: c_ulong,
    ImageFileSize: c_ulong,
    CheckSum: c_ulong,
}}
BITFIELD! {SECTION_IMAGE_INFORMATION ImageFlags: c_uchar [
    ComPlusNativeReady set_ComPlusNativeReady[0..1],
    ComPlusILOnly set_ComPlusILOnly[1..2],
    ImageDynamicallyRelocated set_ImageDynamicallyRelocated[2..3],
    ImageMappedFlat set_ImageMappedFlat[3..4],
    BaseBelow4gb set_BaseBelow4gb[4..5],
    ComPlusPrefer32bit set_ComPlusPrefer32bit[5..6],
    Reserved set_Reserved[6..8],
]}
pub type PSECTION_IMAGE_INFORMATION = *mut SECTION_IMAGE_INFORMATION;
STRUCT! {struct SECTION_INTERNAL_IMAGE_INFORMATION {
    SectionInformation: SECTION_IMAGE_INFORMATION,
    ExtendedFlags: c_ulong,
}}
BITFIELD! {SECTION_INTERNAL_IMAGE_INFORMATION ExtendedFlags: c_ulong [
    ImageExportSuppressionEnabled set_ImageExportSuppressionEnabled[0..1],
    Reserved set_Reserved[1..32],
]}
pub type PSECTION_INTERNAL_IMAGE_INFORMATION =
    *mut SECTION_INTERNAL_IMAGE_INFORMATION;
ENUM! {enum SECTION_INHERIT {
    ViewShare = 1,
    ViewUnmap = 2,
}}
pub const SEC_BASED: u32 = 0x200000;
pub const SEC_NO_CHANGE: u32 = 0x400000;
pub const SEC_GLOBAL: u32 = 0x20000000;
pub const MEM_EXECUTE_OPTION_DISABLE: u32 = 0x1;
pub const MEM_EXECUTE_OPTION_ENABLE: u32 = 0x2;
pub const MEM_EXECUTE_OPTION_DISABLE_THUNK_EMULATION: u32 = 0x4;
pub const MEM_EXECUTE_OPTION_PERMANENT: u32 = 0x8;
pub const MEM_EXECUTE_OPTION_EXECUTE_DISPATCH_ENABLE: u32 = 0x10;
pub const MEM_EXECUTE_OPTION_IMAGE_DISPATCH_ENABLE: u32 = 0x20;
pub const MEM_EXECUTE_OPTION_VALID_FLAGS: u32 = 0x3f;
EXTERN! {extern "system" {
    fn NtAllocateVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut *mut c_void,
        ZeroBits: usize,
        RegionSize: *mut usize,
        AllocationType: c_ulong,
        Protect: c_ulong,
    ) -> NTSTATUS;
    fn NtFreeVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut *mut c_void,
        RegionSize: *mut usize,
        FreeType: c_ulong,
    ) -> NTSTATUS;
    fn NtReadVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut c_void,
        Buffer: *mut c_void,
        BufferSize: usize,
        NumberOfBytesRead: *mut usize,
    ) -> NTSTATUS;
    fn NtWriteVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut c_void,
        Buffer: *mut c_void,
        BufferSize: usize,
        NumberOfBytesWritten: *mut usize,
    ) -> NTSTATUS;
    fn NtProtectVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut *mut c_void,
        RegionSize: *mut usize,
        NewProtect: c_ulong,
        OldProtect: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtQueryVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut c_void,
        MemoryInformationClass: MEMORY_INFORMATION_CLASS,
        MemoryInformation: *mut c_void,
        MemoryInformationLength: usize,
        ReturnLength: *mut usize,
    ) -> NTSTATUS;
}}
ENUM! {enum VIRTUAL_MEMORY_INFORMATION_CLASS {
    VmPrefetchInformation = 0,
    VmPagePriorityInformation = 1,
    VmCfgCallTargetInformation = 2,
    VmPageDirtyStateInformation = 3,
}}
STRUCT! {struct MEMORY_RANGE_ENTRY {
    VirtualAddress: *mut c_void,
    NumberOfBytes: usize,
}}
pub type PMEMORY_RANGE_ENTRY = *mut MEMORY_RANGE_ENTRY;
STRUCT! {struct CFG_CALL_TARGET_LIST_INFORMATION {
    NumberOfEntries: c_ulong,
    Reserved: c_ulong,
    NumberOfEntriesProcessed: *mut c_ulong,
    CallTargetInfo: *mut CFG_CALL_TARGET_INFO,
    Section: *mut c_void,
    FileOffset: __uint64,
}}
pub type PCFG_CALL_TARGET_LIST_INFORMATION =
    *mut CFG_CALL_TARGET_LIST_INFORMATION;
EXTERN! {extern "system" {
    fn NtSetInformationVirtualMemory(
        ProcessHandle: HANDLE,
        VmInformationClass: VIRTUAL_MEMORY_INFORMATION_CLASS,
        NumberOfEntries: usize,
        VirtualAddresses: PMEMORY_RANGE_ENTRY,
        VmInformation: *mut c_void,
        VmInformationLength: c_ulong,
    ) -> NTSTATUS;
    fn NtLockVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut *mut c_void,
        RegionSize: *mut usize,
        MapType: c_ulong,
    ) -> NTSTATUS;
    fn NtUnlockVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut *mut c_void,
        RegionSize: *mut usize,
        MapType: c_ulong,
    ) -> NTSTATUS;
    fn NtCreateSection(
        SectionHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        MaximumSize: *mut LARGE_INTEGER,
        SectionPageProtection: c_ulong,
        AllocationAttributes: c_ulong,
        FileHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtCreateSectionEx(
        SectionHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        MaximumSize: *mut LARGE_INTEGER,
        SectionPageProtection: c_ulong,
        AllocationAttributes: c_ulong,
        FileHandle: HANDLE,
        ExtendedParameters: PMEM_EXTENDED_PARAMETER,
        ExtendedParameterCount: c_ulong,
    ) -> NTSTATUS;
    fn NtOpenSection(
        SectionHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn NtMapViewOfSection(
        SectionHandle: HANDLE,
        ProcessHandle: HANDLE,
        BaseAddress: *mut *mut c_void,
        ZeroBits: usize,
        CommitSize: usize,
        SectionOffset: *mut LARGE_INTEGER,
        ViewSize: *mut usize,
        InheritDisposition: SECTION_INHERIT,
        AllocationType: c_ulong,
        Win32Protect: c_ulong,
    ) -> NTSTATUS;
    fn NtUnmapViewOfSection(
        ProcessHandle: HANDLE,
        BaseAddress: *mut c_void,
    ) -> NTSTATUS;
    fn NtUnmapViewOfSectionEx(
        ProcessHandle: HANDLE,
        BaseAddress: *mut c_void,
        Flags: c_ulong,
    ) -> NTSTATUS;
    fn NtExtendSection(
        SectionHandle: HANDLE,
        NewSectionSize: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtQuerySection(
        SectionHandle: HANDLE,
        SectionInformationClass: SECTION_INFORMATION_CLASS,
        SectionInformation: *mut c_void,
        SectionInformationLength: usize,
        ReturnLength: *mut usize,
    ) -> NTSTATUS;
    fn NtAreMappedFilesTheSame(
        File1MappedAsAnImage: *mut c_void,
        File2MappedAsFile: *mut c_void,
    ) -> NTSTATUS;
}}
pub const MEMORY_PARTITION_QUERY_ACCESS: u32 = 0x0001;
pub const MEMORY_PARTITION_MODIFY_ACCESS: u32 = 0x0002;
pub const MEMORY_PARTITION_ALL_ACCESS: u32 = STANDARD_RIGHTS_REQUIRED
    | SYNCHRONIZE
    | MEMORY_PARTITION_QUERY_ACCESS
    | MEMORY_PARTITION_MODIFY_ACCESS;
ENUM! {enum MEMORY_PARTITION_INFORMATION_CLASS {
    SystemMemoryPartitionInformation = 0,
    SystemMemoryPartitionMoveMemory = 1,
    SystemMemoryPartitionAddPagefile = 2,
    SystemMemoryPartitionCombineMemory = 3,
    SystemMemoryPartitionInitialAddMemory = 4,
    SystemMemoryPartitionGetMemoryEvents = 5,
    SystemMemoryPartitionMax = 6,
}}
STRUCT! {struct MEMORY_PARTITION_CONFIGURATION_INFORMATION {
    Flags: c_ulong,
    NumaNode: c_ulong,
    Channel: c_ulong,
    NumberOfNumaNodes: c_ulong,
    ResidentAvailablePages: usize,
    CommittedPages: usize,
    CommitLimit: usize,
    PeakCommitment: usize,
    TotalNumberOfPages: usize,
    AvailablePages: usize,
    ZeroPages: usize,
    FreePages: usize,
    StandbyPages: usize,
    StandbyPageCountByPriority: [usize; 8],
    RepurposedPagesByPriority: [usize; 8],
    MaximumCommitLimit: usize,
    DonatedPagesToPartitions: usize,
    PartitionId: c_ulong,
}}
pub type PMEMORY_PARTITION_CONFIGURATION_INFORMATION =
    *mut MEMORY_PARTITION_CONFIGURATION_INFORMATION;
STRUCT! {struct MEMORY_PARTITION_TRANSFER_INFORMATION {
    NumberOfPages: usize,
    NumaNode: c_ulong,
    Flags: c_ulong,
}}
pub type PMEMORY_PARTITION_TRANSFER_INFORMATION =
    *mut MEMORY_PARTITION_TRANSFER_INFORMATION;
STRUCT! {struct MEMORY_PARTITION_PAGEFILE_INFORMATION {
    PageFileName: UNICODE_STRING,
    MinimumSize: LARGE_INTEGER,
    MaximumSize: LARGE_INTEGER,
    Flags: c_ulong,
}}
pub type PMEMORY_PARTITION_PAGEFILE_INFORMATION =
    *mut MEMORY_PARTITION_PAGEFILE_INFORMATION;
STRUCT! {struct MEMORY_PARTITION_PAGE_COMBINE_INFORMATION {
    StopHandle: HANDLE,
    Flags: c_ulong,
    TotalNumberOfPages: usize,
}}
pub type PMEMORY_PARTITION_PAGE_COMBINE_INFORMATION =
    *mut MEMORY_PARTITION_PAGE_COMBINE_INFORMATION;
STRUCT! {struct MEMORY_PARTITION_PAGE_RANGE {
    StartPage: usize,
    NumberOfPages: usize,
}}
pub type PMEMORY_PARTITION_PAGE_RANGE = *mut MEMORY_PARTITION_PAGE_RANGE;
STRUCT! {struct MEMORY_PARTITION_INITIAL_ADD_INFORMATION {
    Flags: c_ulong,
    NumberOfRanges: c_ulong,
    NumberOfPagesAdded: usize,
    PartitionRanges: [MEMORY_PARTITION_PAGE_RANGE; 1],
}}
pub type PMEMORY_PARTITION_INITIAL_ADD_INFORMATION =
    *mut MEMORY_PARTITION_INITIAL_ADD_INFORMATION;
STRUCT! {struct MEMORY_PARTITION_MEMORY_EVENTS_INFORMATION {
    Flags: c_ulong,
    HandleAttributes: c_ulong,
    DesiredAccess: c_ulong,
    LowCommitCondition: HANDLE,
    HighCommitCondition: HANDLE,
    MaximumCommitCondition: HANDLE,
}}
BITFIELD! {MEMORY_PARTITION_MEMORY_EVENTS_INFORMATION Flags: c_ulong [
    CommitEvents set_CommitEvents[0..1],
    Spare set_Spare[1..32],
]}
pub type PMEMORY_PARTITION_MEMORY_EVENTS_INFORMATION =
    *mut MEMORY_PARTITION_MEMORY_EVENTS_INFORMATION;
EXTERN! {extern "system" {
    fn NtCreatePartition(
        PartitionHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        PreferredNode: c_ulong,
    ) -> NTSTATUS;
    fn NtOpenPartition(
        PartitionHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn NtManagePartition(
        PartitionInformationClass: MEMORY_PARTITION_INFORMATION_CLASS,
        PartitionInformation: *mut c_void,
        PartitionInformationLength: c_ulong,
    ) -> NTSTATUS;
    fn NtMapUserPhysicalPages(
        VirtualAddress: *mut c_void,
        NumberOfPages: usize,
        UserPfnArray: *mut usize,
    ) -> NTSTATUS;
    fn NtMapUserPhysicalPagesScatter(
        VirtualAddresses: *mut *mut c_void,
        NumberOfPages: usize,
        UserPfnArray: *mut usize,
    ) -> NTSTATUS;
    fn NtAllocateUserPhysicalPages(
        ProcessHandle: HANDLE,
        NumberOfPages: *mut usize,
        UserPfnArray: *mut usize,
    ) -> NTSTATUS;
    fn NtFreeUserPhysicalPages(
        ProcessHandle: HANDLE,
        NumberOfPages: *mut usize,
        UserPfnArray: *mut usize,
    ) -> NTSTATUS;
    fn NtOpenSession(
        SessionHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn NtGetWriteWatch(
        ProcessHandle: HANDLE,
        Flags: c_ulong,
        BaseAddress: *mut c_void,
        RegionSize: usize,
        UserAddressArray: *mut *mut c_void,
        EntriesInUserAddressArray: *mut usize,
        Granularity: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtResetWriteWatch(
        ProcessHandle: HANDLE,
        BaseAddress: *mut c_void,
        RegionSize: usize,
    ) -> NTSTATUS;
    fn NtCreatePagingFile(
        PageFileName: *mut UNICODE_STRING,
        MinimumSize: *mut LARGE_INTEGER,
        MaximumSize: *mut LARGE_INTEGER,
        Priority: c_ulong,
    ) -> NTSTATUS;
    fn NtFlushInstructionCache(
        ProcessHandle: HANDLE,
        BaseAddress: *mut c_void,
        Length: usize,
    ) -> NTSTATUS;
    fn NtFlushWriteBuffer() -> NTSTATUS;
}}
