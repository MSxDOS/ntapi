#[cfg(target_arch = "x86")]
use core::hint::spin_loop;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
use core::ptr::addr_of;
use core::{mem::MaybeUninit, ptr::read_volatile};

use windows_sys::{
    core::GUID,
    Win32::{
        Foundation::{HANDLE, LUID, NTSTATUS, UNICODE_STRING},
        Security::{GENERIC_MAPPING, PSECURITY_DESCRIPTOR},
        Storage::FileSystem::{STANDARD_RIGHTS_REQUIRED, SYNCHRONIZE},
        System::{
            Diagnostics::{
                Debug::XSTATE_CONFIGURATION, Etw::PROFILE_SOURCE_INFO,
            },
            Kernel::{EVENT_TYPE, NT_PRODUCT_TYPE, TIMER_TYPE, WNF_STATE_NAME},
            SystemInformation::{FIRMWARE_TYPE, GROUP_AFFINITY},
            SystemServices::ANYSIZE_ARRAY,
            WindowsProgramming::OBJECT_ATTRIBUTES,
        },
    },
};

use crate::{
    ctypes::{
        __int64, __uint64, c_char, c_long, c_uchar, c_ulong, c_ushort, c_void,
        wchar_t,
    },
    ntapi_base::{CLIENT_ID, KPRIORITY, KSYSTEM_TIME, PRTL_ATOM, RTL_ATOM},
    ntioapi::{BUS_DATA_TYPE, FILE_IO_COMPLETION_INFORMATION, INTERFACE_TYPE},
    ntkeapi::{KPROFILE_SOURCE, KTHREAD_STATE, KWAIT_REASON},
    ntldr::RTL_PROCESS_MODULE_INFORMATION_EX,
    ntpebteb::PTEB,
    ntpoapi::COUNTED_REASON_CONTEXT,
    windows_local::{
        shared::ntdef::{LARGE_INTEGER, ULARGE_INTEGER},
        um::winnt::UInt32x32To64,
    },
};

EXTERN! {extern "system" {
    fn NtDelayExecution(
        Alertable: c_uchar,
        DelayInterval: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtQuerySystemEnvironmentValue(
        VariableName: *mut UNICODE_STRING,
        VariableValue: *mut wchar_t,
        ValueLength: c_ushort,
        ReturnLength: *mut c_ushort,
    ) -> NTSTATUS;
    fn NtSetSystemEnvironmentValue(
        VariableName: *mut UNICODE_STRING,
        VariableValue: *mut UNICODE_STRING,
    ) -> NTSTATUS;
    fn NtQuerySystemEnvironmentValueEx(
        VariableName: *mut UNICODE_STRING,
        VendorGuid: *mut GUID,
        Value: *mut c_void,
        ValueLength: *mut c_ulong,
        Attributes: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtSetSystemEnvironmentValueEx(
        VariableName: *mut UNICODE_STRING,
        VendorGuid: *mut GUID,
        Value: *mut c_void,
        ValueLength: c_ulong,
        Attributes: c_ulong,
    ) -> NTSTATUS;
    fn NtEnumerateSystemEnvironmentValuesEx(
        InformationClass: c_ulong,
        Buffer: *mut c_void,
        BufferLength: *mut c_ulong,
    ) -> NTSTATUS;
}}
STRUCT! {struct BOOT_ENTRY {
    Version: c_ulong,
    Length: c_ulong,
    Id: c_ulong,
    Attributes: c_ulong,
    FriendlyNameOffset: c_ulong,
    BootFilePathOffset: c_ulong,
    OsOptionsLength: c_ulong,
    OsOptions: [c_uchar; 1],
}}
pub type PBOOT_ENTRY = *mut BOOT_ENTRY;
STRUCT! {struct BOOT_ENTRY_LIST {
    NextEntryOffset: c_ulong,
    BootEntry: BOOT_ENTRY,
}}
pub type PBOOT_ENTRY_LIST = *mut BOOT_ENTRY_LIST;
STRUCT! {struct BOOT_OPTIONS {
    Version: c_ulong,
    Length: c_ulong,
    Timeout: c_ulong,
    CurrentBootEntryId: c_ulong,
    NextBootEntryId: c_ulong,
    HeadlessRedirection: [wchar_t; 1],
}}
pub type PBOOT_OPTIONS = *mut BOOT_OPTIONS;
STRUCT! {struct FILE_PATH {
    Version: c_ulong,
    Length: c_ulong,
    Type: c_ulong,
    FilePath: [c_uchar; 1],
}}
pub type PFILE_PATH = *mut FILE_PATH;
STRUCT! {struct EFI_DRIVER_ENTRY {
    Version: c_ulong,
    Length: c_ulong,
    Id: c_ulong,
    FriendlyNameOffset: c_ulong,
    DriverFilePathOffset: c_ulong,
}}
pub type PEFI_DRIVER_ENTRY = *mut EFI_DRIVER_ENTRY;
STRUCT! {struct EFI_DRIVER_ENTRY_LIST {
    NextEntryOffset: c_ulong,
    DriverEntry: EFI_DRIVER_ENTRY,
}}
pub type PEFI_DRIVER_ENTRY_LIST = *mut EFI_DRIVER_ENTRY_LIST;
EXTERN! {extern "system" {
    fn NtAddBootEntry(
        BootEntry: PBOOT_ENTRY,
        Id: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtDeleteBootEntry(
        Id: c_ulong,
    ) -> NTSTATUS;
    fn NtModifyBootEntry(
        BootEntry: PBOOT_ENTRY,
    ) -> NTSTATUS;
    fn NtEnumerateBootEntries(
        Buffer: *mut c_void,
        BufferLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtQueryBootEntryOrder(
        Ids: *mut c_ulong,
        Count: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtSetBootEntryOrder(
        Ids: *mut c_ulong,
        Count: c_ulong,
    ) -> NTSTATUS;
    fn NtQueryBootOptions(
        BootOptions: PBOOT_OPTIONS,
        BootOptionsLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtSetBootOptions(
        BootOptions: PBOOT_OPTIONS,
        FieldsToChange: c_ulong,
    ) -> NTSTATUS;
    fn NtTranslateFilePath(
        InputFilePath: PFILE_PATH,
        OutputType: c_ulong,
        OutputFilePath: PFILE_PATH,
        OutputFilePathLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtAddDriverEntry(
        DriverEntry: PEFI_DRIVER_ENTRY,
        Id: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtDeleteDriverEntry(
        Id: c_ulong,
    ) -> NTSTATUS;
    fn NtModifyDriverEntry(
        DriverEntry: PEFI_DRIVER_ENTRY,
    ) -> NTSTATUS;
    fn NtEnumerateDriverEntries(
        Buffer: *mut c_void,
        BufferLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtQueryDriverEntryOrder(
        Ids: *mut c_ulong,
        Count: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtSetDriverEntryOrder(
        Ids: *mut c_ulong,
        Count: c_ulong,
    ) -> NTSTATUS;
}}
ENUM! {enum FILTER_BOOT_OPTION_OPERATION {
    FilterBootOptionOperationOpenSystemStore = 0,
    FilterBootOptionOperationSetElement = 1,
    FilterBootOptionOperationDeleteElement = 2,
    FilterBootOptionOperationMax = 3,
}}
EXTERN! {extern "system" {
    fn NtFilterBootOption(
        FilterOperation: FILTER_BOOT_OPTION_OPERATION,
        ObjectType: c_ulong,
        ElementType: c_ulong,
        Data: *mut c_void,
        DataSize: c_ulong,
    ) -> NTSTATUS;
}}
pub const EVENT_QUERY_STATE: u32 = 0x0001;
ENUM! {enum EVENT_INFORMATION_CLASS {
    EventBasicInformation = 0,
}}
STRUCT! {struct EVENT_BASIC_INFORMATION {
    EventType: EVENT_TYPE,
    EventState: c_long,
}}
pub type PEVENT_BASIC_INFORMATION = *mut EVENT_BASIC_INFORMATION;
EXTERN! {extern "system" {
    fn NtCreateEvent(
        EventHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        EventType: EVENT_TYPE,
        InitialState: c_uchar,
    ) -> NTSTATUS;
    fn NtOpenEvent(
        EventHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn NtSetEvent(
        EventHandle: HANDLE,
        PreviousState: *mut c_long,
    ) -> NTSTATUS;
    fn NtSetEventBoostPriority(
        EventHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtClearEvent(
        EventHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtResetEvent(
        EventHandle: HANDLE,
        PreviousState: *mut c_long,
    ) -> NTSTATUS;
    fn NtPulseEvent(
        EventHandle: HANDLE,
        PreviousState: *mut c_long,
    ) -> NTSTATUS;
    fn NtQueryEvent(
        EventHandle: HANDLE,
        EventInformationClass: EVENT_INFORMATION_CLASS,
        EventInformation: *mut c_void,
        EventInformationLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
}}
pub const EVENT_PAIR_ALL_ACCESS: c_ulong =
    STANDARD_RIGHTS_REQUIRED | SYNCHRONIZE;
EXTERN! {extern "system" {
    fn NtCreateEventPair(
        EventPairHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn NtOpenEventPair(
        EventPairHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn NtSetLowEventPair(
        EventPairHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtSetHighEventPair(
        EventPairHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtWaitLowEventPair(
        EventPairHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtWaitHighEventPair(
        EventPairHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtSetLowWaitHighEventPair(
        EventPairHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtSetHighWaitLowEventPair(
        EventPairHandle: HANDLE,
    ) -> NTSTATUS;
}}
ENUM! {enum MUTANT_INFORMATION_CLASS {
    MutantBasicInformation = 0,
    MutantOwnerInformation = 1,
}}
STRUCT! {struct MUTANT_BASIC_INFORMATION {
    CurrentCount: c_long,
    OwnedByCaller: c_uchar,
    AbandonedState: c_uchar,
}}
pub type PMUTANT_BASIC_INFORMATION = *mut MUTANT_BASIC_INFORMATION;
STRUCT! {struct MUTANT_OWNER_INFORMATION {
    ClientId: CLIENT_ID,
}}
pub type PMUTANT_OWNER_INFORMATION = *mut MUTANT_OWNER_INFORMATION;
EXTERN! {extern "system" {
    fn NtCreateMutant(
        MutantHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        InitialOwner: c_uchar,
    ) -> NTSTATUS;
    fn NtOpenMutant(
        MutantHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn NtReleaseMutant(
        MutantHandle: HANDLE,
        PreviousCount: *mut c_long,
    ) -> NTSTATUS;
    fn NtQueryMutant(
        MutantHandle: HANDLE,
        MutantInformationClass: MUTANT_INFORMATION_CLASS,
        MutantInformation: *mut c_void,
        MutantInformationLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
}}
pub const SEMAPHORE_QUERY_STATE: u32 = 0x0001;
ENUM! {enum SEMAPHORE_INFORMATION_CLASS {
    SemaphoreBasicInformation = 0,
}}
STRUCT! {struct SEMAPHORE_BASIC_INFORMATION {
    CurrentCount: c_long,
    MaximumCount: c_long,
}}
pub type PSEMAPHORE_BASIC_INFORMATION = *mut SEMAPHORE_BASIC_INFORMATION;
EXTERN! {extern "system" {
    fn NtCreateSemaphore(
        SemaphoreHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        InitialCount: c_long,
        MaximumCount: c_long,
    ) -> NTSTATUS;
    fn NtOpenSemaphore(
        SemaphoreHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn NtReleaseSemaphore(
        SemaphoreHandle: HANDLE,
        ReleaseCount: c_long,
        PreviousCount: *mut c_long,
    ) -> NTSTATUS;
    fn NtQuerySemaphore(
        SemaphoreHandle: HANDLE,
        SemaphoreInformationClass: SEMAPHORE_INFORMATION_CLASS,
        SemaphoreInformation: *mut c_void,
        SemaphoreInformationLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
}}
ENUM! {enum TIMER_INFORMATION_CLASS {
    TimerBasicInformation = 0,
}}
STRUCT! {struct TIMER_BASIC_INFORMATION {
    RemainingTime: LARGE_INTEGER,
    TimerState: c_uchar,
}}
pub type PTIMER_BASIC_INFORMATION = *mut TIMER_BASIC_INFORMATION;
FN! {stdcall PTIMER_APC_ROUTINE(
    TimerContext: *mut c_void,
    TimerLowValue: c_ulong,
    TimerHighValue: c_long,
) -> ()}
ENUM! {enum TIMER_SET_INFORMATION_CLASS {
    TimerSetCoalescableTimer = 0,
    MaxTimerInfoClass = 1,
}}
STRUCT! {struct TIMER_SET_COALESCABLE_TIMER_INFO {
    DueTime: LARGE_INTEGER,
    TimerApcRoutine: PTIMER_APC_ROUTINE,
    TimerContext: *mut c_void,
    WakeContext: *mut COUNTED_REASON_CONTEXT,
    Period: c_ulong,
    TolerableDelay: c_ulong,
    PreviousState: *mut c_uchar,
}}
pub type PTIMER_SET_COALESCABLE_TIMER_INFO =
    *mut TIMER_SET_COALESCABLE_TIMER_INFO;
EXTERN! {extern "system" {
    fn NtCreateTimer(
        TimerHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        TimerType: TIMER_TYPE,
    ) -> NTSTATUS;
    fn NtOpenTimer(
        TimerHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn NtSetTimer(
        TimerHandle: HANDLE,
        DueTime: *mut LARGE_INTEGER,
        TimerApcRoutine: PTIMER_APC_ROUTINE,
        TimerContext: *mut c_void,
        ResumeTimer: c_uchar,
        Period: c_long,
        PreviousState: *mut c_uchar,
    ) -> NTSTATUS;
    fn NtSetTimerEx(
        TimerHandle: HANDLE,
        TimerSetInformationClass: TIMER_SET_INFORMATION_CLASS,
        TimerSetInformation: *mut c_void,
        TimerSetInformationLength: c_ulong,
    ) -> NTSTATUS;
    fn NtCancelTimer(
        TimerHandle: HANDLE,
        CurrentState: *mut c_uchar,
    ) -> NTSTATUS;
    fn NtQueryTimer(
        TimerHandle: HANDLE,
        TimerInformationClass: TIMER_INFORMATION_CLASS,
        TimerInformation: *mut c_void,
        TimerInformationLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtCreateIRTimer(
        TimerHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
    ) -> NTSTATUS;
    fn NtSetIRTimer(
        TimerHandle: HANDLE,
        DueTime: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
}}
STRUCT! {struct T2_SET_PARAMETERS {
    Version: c_ulong,
    Reserved: c_ulong,
    NoWakeTolerance: __int64,
}}
pub type PT2_SET_PARAMETERS = *mut T2_SET_PARAMETERS;
pub type PT2_CANCEL_PARAMETERS = *mut c_void;
EXTERN! {extern "system" {
    fn NtCreateTimer2(
        TimerHandle: *mut HANDLE,
        Reserved1: *mut c_void,
        Reserved2: *mut c_void,
        Attributes: c_ulong,
        DesiredAccess: c_ulong,
    ) -> NTSTATUS;
    fn NtSetTimer2(
        TimerHandle: HANDLE,
        DueTime: *mut LARGE_INTEGER,
        Period: *mut LARGE_INTEGER,
        Parameters: PT2_SET_PARAMETERS,
    ) -> NTSTATUS;
    fn NtCancelTimer2(
        TimerHandle: HANDLE,
        Parameters: PT2_CANCEL_PARAMETERS,
    ) -> NTSTATUS;
}}
pub const PROFILE_CONTROL: u32 = 0x0001;
pub const PROFILE_ALL_ACCESS: u32 = STANDARD_RIGHTS_REQUIRED | PROFILE_CONTROL;
EXTERN! {extern "system" {
    fn NtCreateProfile(
        ProfileHandle: *mut HANDLE,
        Process: HANDLE,
        ProfileBase: *mut c_void,
        ProfileSize: usize,
        BucketSize: c_ulong,
        Buffer: *mut c_ulong,
        BufferSize: c_ulong,
        ProfileSource: KPROFILE_SOURCE,
        Affinity: usize,
    ) -> NTSTATUS;
    fn NtCreateProfileEx(
        ProfileHandle: *mut HANDLE,
        Process: HANDLE,
        ProfileBase: *mut c_void,
        ProfileSize: usize,
        BucketSize: c_ulong,
        Buffer: *mut c_ulong,
        BufferSize: c_ulong,
        ProfileSource: KPROFILE_SOURCE,
        GroupCount: c_ushort,
        GroupAffinity: *mut GROUP_AFFINITY,
    ) -> NTSTATUS;
    fn NtStartProfile(
        ProfileHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtStopProfile(
        ProfileHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtQueryIntervalProfile(
        ProfileSource: KPROFILE_SOURCE,
        Interval: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtSetIntervalProfile(
        Interval: c_ulong,
        Source: KPROFILE_SOURCE,
    ) -> NTSTATUS;
}}
pub const KEYEDEVENT_WAIT: c_ulong = 0x0001;
pub const KEYEDEVENT_WAKE: c_ulong = 0x0002;
pub const KEYEDEVENT_ALL_ACCESS: c_ulong =
    STANDARD_RIGHTS_REQUIRED | KEYEDEVENT_WAIT | KEYEDEVENT_WAKE;
EXTERN! {extern "system" {
    fn NtCreateKeyedEvent(
        KeyedEventHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        Flags: c_ulong,
    ) -> NTSTATUS;
    fn NtOpenKeyedEvent(
        KeyedEventHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn NtReleaseKeyedEvent(
        KeyedEventHandle: HANDLE,
        KeyValue: *mut c_void,
        Alertable: c_uchar,
        Timeout: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtWaitForKeyedEvent(
        KeyedEventHandle: HANDLE,
        KeyValue: *mut c_void,
        Alertable: c_uchar,
        Timeout: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtUmsThreadYield(
        SchedulerParam: *mut c_void,
    ) -> NTSTATUS;
}}
ENUM! {enum WNF_STATE_NAME_LIFETIME {
    WnfWellKnownStateName = 0,
    WnfPermanentStateName = 1,
    WnfPersistentStateName = 2,
    WnfTemporaryStateName = 3,
}}
ENUM! {enum WNF_STATE_NAME_INFORMATION {
    WnfInfoStateNameExist = 0,
    WnfInfoSubscribersPresent = 1,
    WnfInfoIsQuiescent = 2,
}}
ENUM! {enum WNF_DATA_SCOPE {
    WnfDataScopeSystem = 0,
    WnfDataScopeSession = 1,
    WnfDataScopeUser = 2,
    WnfDataScopeProcess = 3,
    WnfDataScopeMachine = 4,
}}
STRUCT! {struct WNF_TYPE_ID {
    TypeId: GUID,
}}
pub type PWNF_TYPE_ID = *mut WNF_TYPE_ID;
pub type PCWNF_TYPE_ID = *const WNF_TYPE_ID;
pub type PWNF_CHANGE_STAMP = *mut c_ulong;
pub type WNF_CHANGE_STAMP = c_ulong;
STRUCT! {struct WNF_DELIVERY_DESCRIPTOR {
    SubscriptionId: __uint64,
    StateName: WNF_STATE_NAME,
    ChangeStamp: WNF_CHANGE_STAMP,
    StateDataSize: c_ulong,
    EventMask: c_ulong,
    TypeId: WNF_TYPE_ID,
    StateDataOffset: c_ulong,
}}
pub type PWNF_DELIVERY_DESCRIPTOR = *mut WNF_DELIVERY_DESCRIPTOR;
EXTERN! {extern "system" {
    fn NtCreateWnfStateName(
        StateName: *mut WNF_STATE_NAME,
        NameLifetime: WNF_STATE_NAME_LIFETIME,
        DataScope: WNF_DATA_SCOPE,
        PersistData: c_uchar,
        TypeId: PCWNF_TYPE_ID,
        MaximumStateSize: c_ulong,
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
    ) -> NTSTATUS;
    fn NtDeleteWnfStateName(
        StateName: *const WNF_STATE_NAME,
    ) -> NTSTATUS;
    fn NtUpdateWnfStateData(
        StateName: *const WNF_STATE_NAME,
        Buffer: *const c_void,
        Length: c_ulong,
        TypeId: PCWNF_TYPE_ID,
        ExplicitScope: *const c_void,
        MatchingChangeStamp: WNF_CHANGE_STAMP,
        CheckStamp: c_ulong,
    ) -> NTSTATUS;
    fn NtDeleteWnfStateData(
        StateName: *const WNF_STATE_NAME,
        ExplicitScope: *const c_void,
    ) -> NTSTATUS;
    fn NtQueryWnfStateData(
        StateName: *const WNF_STATE_NAME,
        TypeId: PCWNF_TYPE_ID,
        ExplicitScope: *const c_void,
        ChangeStamp: PWNF_CHANGE_STAMP,
        Buffer: *mut c_void,
        BufferSize: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtQueryWnfStateNameInformation(
        StateName: *const WNF_STATE_NAME,
        NameInfoClass: WNF_STATE_NAME_INFORMATION,
        ExplicitScope: *const c_void,
        InfoBuffer: *mut c_void,
        InfoBufferSize: c_ulong,
    ) -> NTSTATUS;
    fn NtSubscribeWnfStateChange(
        StateName: *const WNF_STATE_NAME,
        ChangeStamp: WNF_CHANGE_STAMP,
        EventMask: c_ulong,
        SubscriptionId: *mut __uint64,
    ) -> NTSTATUS;
    fn NtUnsubscribeWnfStateChange(
        StateName: *const WNF_STATE_NAME,
    ) -> NTSTATUS;
    fn NtGetCompleteWnfStateSubscription(
        OldDescriptorStateName: *mut WNF_STATE_NAME,
        OldSubscriptionId: *mut __uint64,
        OldDescriptorEventMask: c_ulong,
        OldDescriptorStatus: c_ulong,
        NewDeliveryDescriptor: PWNF_DELIVERY_DESCRIPTOR,
        DescriptorSize: c_ulong,
    ) -> NTSTATUS;
    fn NtSetWnfProcessNotificationEvent(
        NotificationEvent: HANDLE,
    ) -> NTSTATUS;
}}
pub const WORKER_FACTORY_RELEASE_WORKER: u32 = 0x0001;
pub const WORKER_FACTORY_WAIT: u32 = 0x0002;
pub const WORKER_FACTORY_SET_INFORMATION: u32 = 0x0004;
pub const WORKER_FACTORY_QUERY_INFORMATION: u32 = 0x0008;
pub const WORKER_FACTORY_READY_WORKER: u32 = 0x0010;
pub const WORKER_FACTORY_SHUTDOWN: u32 = 0x0020;
pub const WORKER_FACTORY_ALL_ACCESS: c_ulong = STANDARD_RIGHTS_REQUIRED
    | WORKER_FACTORY_RELEASE_WORKER
    | WORKER_FACTORY_WAIT
    | WORKER_FACTORY_SET_INFORMATION
    | WORKER_FACTORY_QUERY_INFORMATION
    | WORKER_FACTORY_READY_WORKER
    | WORKER_FACTORY_SHUTDOWN;
ENUM! {enum WORKERFACTORYINFOCLASS {
    WorkerFactoryTimeout = 0,
    WorkerFactoryRetryTimeout = 1,
    WorkerFactoryIdleTimeout = 2,
    WorkerFactoryBindingCount = 3,
    WorkerFactoryThreadMinimum = 4,
    WorkerFactoryThreadMaximum = 5,
    WorkerFactoryPaused = 6,
    WorkerFactoryBasicInformation = 7,
    WorkerFactoryAdjustThreadGoal = 8,
    WorkerFactoryCallbackType = 9,
    WorkerFactoryStackInformation = 10,
    WorkerFactoryThreadBasePriority = 11,
    WorkerFactoryTimeoutWaiters = 12,
    WorkerFactoryFlags = 13,
    WorkerFactoryThreadSoftMaximum = 14,
    MaxWorkerFactoryInfoClass = 15,
}}
pub type PWORKERFACTORYINFOCLASS = *mut WORKERFACTORYINFOCLASS;
STRUCT! {struct WORKER_FACTORY_BASIC_INFORMATION {
    Timeout: LARGE_INTEGER,
    RetryTimeout: LARGE_INTEGER,
    IdleTimeout: LARGE_INTEGER,
    Paused: c_uchar,
    TimerSet: c_uchar,
    QueuedToExWorker: c_uchar,
    MayCreate: c_uchar,
    CreateInProgress: c_uchar,
    InsertedIntoQueue: c_uchar,
    Shutdown: c_uchar,
    BindingCount: c_ulong,
    ThreadMinimum: c_ulong,
    ThreadMaximum: c_ulong,
    PendingWorkerCount: c_ulong,
    WaitingWorkerCount: c_ulong,
    TotalWorkerCount: c_ulong,
    ReleaseCount: c_ulong,
    InfiniteWaitGoal: __int64,
    StartRoutine: *mut c_void,
    StartParameter: *mut c_void,
    ProcessId: HANDLE,
    StackReserve: usize,
    StackCommit: usize,
    LastThreadCreationStatus: NTSTATUS,
}}
pub type PWORKER_FACTORY_BASIC_INFORMATION =
    *mut WORKER_FACTORY_BASIC_INFORMATION;
EXTERN! {extern "system" {
    fn NtCreateWorkerFactory(
        WorkerFactoryHandleReturn: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        CompletionPortHandle: HANDLE,
        WorkerProcessHandle: HANDLE,
        StartRoutine: *mut c_void,
        StartParameter: *mut c_void,
        MaxThreadCount: c_ulong,
        StackReserve: usize,
        StackCommit: usize,
    ) -> NTSTATUS;
    fn NtQueryInformationWorkerFactory(
        WorkerFactoryHandle: HANDLE,
        WorkerFactoryInformationClass: WORKERFACTORYINFOCLASS,
        WorkerFactoryInformation: *mut c_void,
        WorkerFactoryInformationLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtSetInformationWorkerFactory(
        WorkerFactoryHandle: HANDLE,
        WorkerFactoryInformationClass: WORKERFACTORYINFOCLASS,
        WorkerFactoryInformation: *mut c_void,
        WorkerFactoryInformationLength: c_ulong,
    ) -> NTSTATUS;
    fn NtShutdownWorkerFactory(
        WorkerFactoryHandle: HANDLE,
        PendingWorkerCount: *mut c_long,
    ) -> NTSTATUS;
    fn NtReleaseWorkerFactoryWorker(
        WorkerFactoryHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtWorkerFactoryWorkerReady(
        WorkerFactoryHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtWaitForWorkViaWorkerFactory(
        WorkerFactoryHandle: HANDLE,
        MiniPacket: *mut FILE_IO_COMPLETION_INFORMATION,
    ) -> NTSTATUS;
    fn NtQuerySystemTime(
        SystemTime: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtSetSystemTime(
        SystemTime: *mut LARGE_INTEGER,
        PreviousTime: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtQueryTimerResolution(
        MaximumTime: *mut c_ulong,
        MinimumTime: *mut c_ulong,
        CurrentTime: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtSetTimerResolution(
        DesiredTime: c_ulong,
        SetResolution: c_uchar,
        ActualTime: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtQueryPerformanceCounter(
        PerformanceCounter: *mut LARGE_INTEGER,
        PerformanceFrequency: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtAllocateLocallyUniqueId(
        Luid: *mut LUID,
    ) -> NTSTATUS;
    fn NtSetUuidSeed(
        Seed: *mut c_char,
    ) -> NTSTATUS;
    fn NtAllocateUuids(
        Time: *mut ULARGE_INTEGER,
        Range: *mut c_ulong,
        Sequence: *mut c_ulong,
        Seed: *mut c_char,
    ) -> NTSTATUS;
}}
ENUM! {enum SYSTEM_INFORMATION_CLASS {
    SystemBasicInformation = 0,
    SystemProcessorInformation = 1,
    SystemPerformanceInformation = 2,
    SystemTimeOfDayInformation = 3,
    SystemPathInformation = 4,
    SystemProcessInformation = 5,
    SystemCallCountInformation = 6,
    SystemDeviceInformation = 7,
    SystemProcessorPerformanceInformation = 8,
    SystemFlagsInformation = 9,
    SystemCallTimeInformation = 10,
    SystemModuleInformation = 11,
    SystemLocksInformation = 12,
    SystemStackTraceInformation = 13,
    SystemPagedPoolInformation = 14,
    SystemNonPagedPoolInformation = 15,
    SystemHandleInformation = 16,
    SystemObjectInformation = 17,
    SystemPageFileInformation = 18,
    SystemVdmInstemulInformation = 19,
    SystemVdmBopInformation = 20,
    SystemFileCacheInformation = 21,
    SystemPoolTagInformation = 22,
    SystemInterruptInformation = 23,
    SystemDpcBehaviorInformation = 24,
    SystemFullMemoryInformation = 25,
    SystemLoadGdiDriverInformation = 26,
    SystemUnloadGdiDriverInformation = 27,
    SystemTimeAdjustmentInformation = 28,
    SystemSummaryMemoryInformation = 29,
    SystemMirrorMemoryInformation = 30,
    SystemPerformanceTraceInformation = 31,
    SystemObsolete0 = 32,
    SystemExceptionInformation = 33,
    SystemCrashDumpStateInformation = 34,
    SystemKernelDebuggerInformation = 35,
    SystemContextSwitchInformation = 36,
    SystemRegistryQuotaInformation = 37,
    SystemExtendServiceTableInformation = 38,
    SystemPrioritySeperation = 39,
    SystemVerifierAddDriverInformation = 40,
    SystemVerifierRemoveDriverInformation = 41,
    SystemProcessorIdleInformation = 42,
    SystemLegacyDriverInformation = 43,
    SystemCurrentTimeZoneInformation = 44,
    SystemLookasideInformation = 45,
    SystemTimeSlipNotification = 46,
    SystemSessionCreate = 47,
    SystemSessionDetach = 48,
    SystemSessionInformation = 49,
    SystemRangeStartInformation = 50,
    SystemVerifierInformation = 51,
    SystemVerifierThunkExtend = 52,
    SystemSessionProcessInformation = 53,
    SystemLoadGdiDriverInSystemSpace = 54,
    SystemNumaProcessorMap = 55,
    SystemPrefetcherInformation = 56,
    SystemExtendedProcessInformation = 57,
    SystemRecommendedSharedDataAlignment = 58,
    SystemComPlusPackage = 59,
    SystemNumaAvailableMemory = 60,
    SystemProcessorPowerInformation = 61,
    SystemEmulationBasicInformation = 62,
    SystemEmulationProcessorInformation = 63,
    SystemExtendedHandleInformation = 64,
    SystemLostDelayedWriteInformation = 65,
    SystemBigPoolInformation = 66,
    SystemSessionPoolTagInformation = 67,
    SystemSessionMappedViewInformation = 68,
    SystemHotpatchInformation = 69,
    SystemObjectSecurityMode = 70,
    SystemWatchdogTimerHandler = 71,
    SystemWatchdogTimerInformation = 72,
    SystemLogicalProcessorInformation = 73,
    SystemWow64SharedInformationObsolete = 74,
    SystemRegisterFirmwareTableInformationHandler = 75,
    SystemFirmwareTableInformation = 76,
    SystemModuleInformationEx = 77,
    SystemVerifierTriageInformation = 78,
    SystemSuperfetchInformation = 79,
    SystemMemoryListInformation = 80,
    SystemFileCacheInformationEx = 81,
    SystemThreadPriorityClientIdInformation = 82,
    SystemProcessorIdleCycleTimeInformation = 83,
    SystemVerifierCancellationInformation = 84,
    SystemProcessorPowerInformationEx = 85,
    SystemRefTraceInformation = 86,
    SystemSpecialPoolInformation = 87,
    SystemProcessIdInformation = 88,
    SystemErrorPortInformation = 89,
    SystemBootEnvironmentInformation = 90,
    SystemHypervisorInformation = 91,
    SystemVerifierInformationEx = 92,
    SystemTimeZoneInformation = 93,
    SystemImageFileExecutionOptionsInformation = 94,
    SystemCoverageInformation = 95,
    SystemPrefetchPatchInformation = 96,
    SystemVerifierFaultsInformation = 97,
    SystemSystemPartitionInformation = 98,
    SystemSystemDiskInformation = 99,
    SystemProcessorPerformanceDistribution = 100,
    SystemNumaProximityNodeInformation = 101,
    SystemDynamicTimeZoneInformation = 102,
    SystemCodeIntegrityInformation = 103,
    SystemProcessorMicrocodeUpdateInformation = 104,
    SystemProcessorBrandString = 105,
    SystemVirtualAddressInformation = 106,
    SystemLogicalProcessorAndGroupInformation = 107,
    SystemProcessorCycleTimeInformation = 108,
    SystemStoreInformation = 109,
    SystemRegistryAppendString = 110,
    SystemAitSamplingValue = 111,
    SystemVhdBootInformation = 112,
    SystemCpuQuotaInformation = 113,
    SystemNativeBasicInformation = 114,
    SystemSpare1 = 115,
    SystemLowPriorityIoInformation = 116,
    SystemTpmBootEntropyInformation = 117,
    SystemVerifierCountersInformation = 118,
    SystemPagedPoolInformationEx = 119,
    SystemSystemPtesInformationEx = 120,
    SystemNodeDistanceInformation = 121,
    SystemAcpiAuditInformation = 122,
    SystemBasicPerformanceInformation = 123,
    SystemQueryPerformanceCounterInformation = 124,
    SystemSessionBigPoolInformation = 125,
    SystemBootGraphicsInformation = 126,
    SystemScrubPhysicalMemoryInformation = 127,
    SystemBadPageInformation = 128,
    SystemProcessorProfileControlArea = 129,
    SystemCombinePhysicalMemoryInformation = 130,
    SystemEntropyInterruptTimingCallback = 131,
    SystemConsoleInformation = 132,
    SystemPlatformBinaryInformation = 133,
    SystemThrottleNotificationInformation = 134,
    SystemHypervisorProcessorCountInformation = 135,
    SystemDeviceDataInformation = 136,
    SystemDeviceDataEnumerationInformation = 137,
    SystemMemoryTopologyInformation = 138,
    SystemMemoryChannelInformation = 139,
    SystemBootLogoInformation = 140,
    SystemProcessorPerformanceInformationEx = 141,
    SystemSpare0 = 142,
    SystemSecureBootPolicyInformation = 143,
    SystemPageFileInformationEx = 144,
    SystemSecureBootInformation = 145,
    SystemEntropyInterruptTimingRawInformation = 146,
    SystemPortableWorkspaceEfiLauncherInformation = 147,
    SystemFullProcessInformation = 148,
    SystemKernelDebuggerInformationEx = 149,
    SystemBootMetadataInformation = 150,
    SystemSoftRebootInformation = 151,
    SystemElamCertificateInformation = 152,
    SystemOfflineDumpConfigInformation = 153,
    SystemProcessorFeaturesInformation = 154,
    SystemRegistryReconciliationInformation = 155,
    SystemEdidInformation = 156,
    SystemManufacturingInformation = 157,
    SystemEnergyEstimationConfigInformation = 158,
    SystemHypervisorDetailInformation = 159,
    SystemProcessorCycleStatsInformation = 160,
    SystemVmGenerationCountInformation = 161,
    SystemTrustedPlatformModuleInformation = 162,
    SystemKernelDebuggerFlags = 163,
    SystemCodeIntegrityPolicyInformation = 164,
    SystemIsolatedUserModeInformation = 165,
    SystemHardwareSecurityTestInterfaceResultsInformation = 166,
    SystemSingleModuleInformation = 167,
    SystemAllowedCpuSetsInformation = 168,
    SystemVsmProtectionInformation = 169,
    SystemInterruptCpuSetsInformation = 170,
    SystemSecureBootPolicyFullInformation = 171,
    SystemCodeIntegrityPolicyFullInformation = 172,
    SystemAffinitizedInterruptProcessorInformation = 173,
    SystemRootSiloInformation = 174,
    SystemCpuSetInformation = 175,
    SystemCpuSetTagInformation = 176,
    SystemWin32WerStartCallout = 177,
    SystemSecureKernelProfileInformation = 178,
    SystemCodeIntegrityPlatformManifestInformation = 179,
    SystemInterruptSteeringInformation = 180,
    SystemSupportedProcessorArchitectures = 181,
    SystemMemoryUsageInformation = 182,
    SystemCodeIntegrityCertificateInformation = 183,
    SystemPhysicalMemoryInformation = 184,
    SystemControlFlowTransition = 185,
    SystemKernelDebuggingAllowed = 186,
    SystemActivityModerationExeState = 187,
    SystemActivityModerationUserSettings = 188,
    SystemCodeIntegrityPoliciesFullInformation = 189,
    SystemCodeIntegrityUnlockInformation = 190,
    SystemIntegrityQuotaInformation = 191,
    SystemFlushInformation = 192,
    SystemProcessorIdleMaskInformation = 193,
    SystemSecureDumpEncryptionInformation = 194,
    SystemWriteConstraintInformation = 195,
    SystemKernelVaShadowInformation = 196,
    SystemHypervisorSharedPageInformation = 197,
    SystemFirmwareBootPerformanceInformation = 198,
    SystemCodeIntegrityVerificationInformation = 199,
    SystemFirmwarePartitionInformation = 200,
    SystemSpeculationControlInformation = 201,
    SystemDmaGuardPolicyInformation = 202,
    SystemEnclaveLaunchControlInformation = 203,
    SystemWorkloadAllowedCpuSetsInformation = 204,
    SystemCodeIntegrityUnlockModeInformation = 205,
    SystemLeapSecondInformation = 206,
    SystemFlags2Information = 207,
    MaxSystemInfoClass = 208,
}}
STRUCT! {struct SYSTEM_BASIC_INFORMATION {
    Reserved: c_ulong,
    TimerResolution: c_ulong,
    PageSize: c_ulong,
    NumberOfPhysicalPages: c_ulong,
    LowestPhysicalPageNumber: c_ulong,
    HighestPhysicalPageNumber: c_ulong,
    AllocationGranularity: c_ulong,
    MinimumUserModeAddress: usize,
    MaximumUserModeAddress: usize,
    ActiveProcessorsAffinityMask: usize,
    NumberOfProcessors: c_char,
}}
pub type PSYSTEM_BASIC_INFORMATION = *mut SYSTEM_BASIC_INFORMATION;
STRUCT! {struct SYSTEM_PROCESSOR_INFORMATION {
    ProcessorArchitecture: c_ushort,
    ProcessorLevel: c_ushort,
    ProcessorRevision: c_ushort,
    MaximumProcessors: c_ushort,
    ProcessorFeatureBits: c_ulong,
}}
pub type PSYSTEM_PROCESSOR_INFORMATION = *mut SYSTEM_PROCESSOR_INFORMATION;
STRUCT! {struct SYSTEM_PERFORMANCE_INFORMATION {
    IdleProcessTime: LARGE_INTEGER,
    IoReadTransferCount: LARGE_INTEGER,
    IoWriteTransferCount: LARGE_INTEGER,
    IoOtherTransferCount: LARGE_INTEGER,
    IoReadOperationCount: c_ulong,
    IoWriteOperationCount: c_ulong,
    IoOtherOperationCount: c_ulong,
    AvailablePages: c_ulong,
    CommittedPages: c_ulong,
    CommitLimit: c_ulong,
    PeakCommitment: c_ulong,
    PageFaultCount: c_ulong,
    CopyOnWriteCount: c_ulong,
    TransitionCount: c_ulong,
    CacheTransitionCount: c_ulong,
    DemandZeroCount: c_ulong,
    PageReadCount: c_ulong,
    PageReadIoCount: c_ulong,
    CacheReadCount: c_ulong,
    CacheIoCount: c_ulong,
    DirtyPagesWriteCount: c_ulong,
    DirtyWriteIoCount: c_ulong,
    MappedPagesWriteCount: c_ulong,
    MappedWriteIoCount: c_ulong,
    PagedPoolPages: c_ulong,
    NonPagedPoolPages: c_ulong,
    PagedPoolAllocs: c_ulong,
    PagedPoolFrees: c_ulong,
    NonPagedPoolAllocs: c_ulong,
    NonPagedPoolFrees: c_ulong,
    FreeSystemPtes: c_ulong,
    ResidentSystemCodePage: c_ulong,
    TotalSystemDriverPages: c_ulong,
    TotalSystemCodePages: c_ulong,
    NonPagedPoolLookasideHits: c_ulong,
    PagedPoolLookasideHits: c_ulong,
    AvailablePagedPoolPages: c_ulong,
    ResidentSystemCachePage: c_ulong,
    ResidentPagedPoolPage: c_ulong,
    ResidentSystemDriverPage: c_ulong,
    CcFastReadNoWait: c_ulong,
    CcFastReadWait: c_ulong,
    CcFastReadResourceMiss: c_ulong,
    CcFastReadNotPossible: c_ulong,
    CcFastMdlReadNoWait: c_ulong,
    CcFastMdlReadWait: c_ulong,
    CcFastMdlReadResourceMiss: c_ulong,
    CcFastMdlReadNotPossible: c_ulong,
    CcMapDataNoWait: c_ulong,
    CcMapDataWait: c_ulong,
    CcMapDataNoWaitMiss: c_ulong,
    CcMapDataWaitMiss: c_ulong,
    CcPinMappedDataCount: c_ulong,
    CcPinReadNoWait: c_ulong,
    CcPinReadWait: c_ulong,
    CcPinReadNoWaitMiss: c_ulong,
    CcPinReadWaitMiss: c_ulong,
    CcCopyReadNoWait: c_ulong,
    CcCopyReadWait: c_ulong,
    CcCopyReadNoWaitMiss: c_ulong,
    CcCopyReadWaitMiss: c_ulong,
    CcMdlReadNoWait: c_ulong,
    CcMdlReadWait: c_ulong,
    CcMdlReadNoWaitMiss: c_ulong,
    CcMdlReadWaitMiss: c_ulong,
    CcReadAheadIos: c_ulong,
    CcLazyWriteIos: c_ulong,
    CcLazyWritePages: c_ulong,
    CcDataFlushes: c_ulong,
    CcDataPages: c_ulong,
    ContextSwitches: c_ulong,
    FirstLevelTbFills: c_ulong,
    SecondLevelTbFills: c_ulong,
    SystemCalls: c_ulong,
    CcTotalDirtyPages: __uint64,
    CcDirtyPageThreshold: __uint64,
    ResidentAvailablePages: __int64,
    SharedCommittedPages: __uint64,
}}
pub type PSYSTEM_PERFORMANCE_INFORMATION = *mut SYSTEM_PERFORMANCE_INFORMATION;
STRUCT! {struct SYSTEM_TIMEOFDAY_INFORMATION {
    BootTime: LARGE_INTEGER,
    CurrentTime: LARGE_INTEGER,
    TimeZoneBias: LARGE_INTEGER,
    TimeZoneId: c_ulong,
    Reserved: c_ulong,
    BootTimeBias: __uint64,
    SleepTimeBias: __uint64,
}}
pub type PSYSTEM_TIMEOFDAY_INFORMATION = *mut SYSTEM_TIMEOFDAY_INFORMATION;
STRUCT! {struct SYSTEM_THREAD_INFORMATION {
    KernelTime: LARGE_INTEGER,
    UserTime: LARGE_INTEGER,
    CreateTime: LARGE_INTEGER,
    WaitTime: c_ulong,
    StartAddress: *mut c_void,
    ClientId: CLIENT_ID,
    Priority: KPRIORITY,
    BasePriority: c_long,
    ContextSwitches: c_ulong,
    ThreadState: KTHREAD_STATE,
    WaitReason: KWAIT_REASON,
}}
pub type PSYSTEM_THREAD_INFORMATION = *mut SYSTEM_THREAD_INFORMATION;
STRUCT! {struct SYSTEM_EXTENDED_THREAD_INFORMATION {
    ThreadInfo: SYSTEM_THREAD_INFORMATION,
    StackBase: *mut c_void,
    StackLimit: *mut c_void,
    Win32StartAddress: *mut c_void,
    TebBase: PTEB,
    Reserved2: usize,
    Reserved3: usize,
    Reserved4: usize,
}}
pub type PSYSTEM_EXTENDED_THREAD_INFORMATION =
    *mut SYSTEM_EXTENDED_THREAD_INFORMATION;
STRUCT! {struct SYSTEM_PROCESS_INFORMATION {
    NextEntryOffset: c_ulong,
    NumberOfThreads: c_ulong,
    WorkingSetPrivateSize: LARGE_INTEGER,
    HardFaultCount: c_ulong,
    NumberOfThreadsHighWatermark: c_ulong,
    CycleTime: __uint64,
    CreateTime: LARGE_INTEGER,
    UserTime: LARGE_INTEGER,
    KernelTime: LARGE_INTEGER,
    ImageName: UNICODE_STRING,
    BasePriority: KPRIORITY,
    UniqueProcessId: HANDLE,
    InheritedFromUniqueProcessId: HANDLE,
    HandleCount: c_ulong,
    SessionId: c_ulong,
    UniqueProcessKey: usize,
    PeakVirtualSize: usize,
    VirtualSize: usize,
    PageFaultCount: c_ulong,
    PeakWorkingSetSize: usize,
    WorkingSetSize: usize,
    QuotaPeakPagedPoolUsage: usize,
    QuotaPagedPoolUsage: usize,
    QuotaPeakNonPagedPoolUsage: usize,
    QuotaNonPagedPoolUsage: usize,
    PagefileUsage: usize,
    PeakPagefileUsage: usize,
    PrivatePageCount: usize,
    ReadOperationCount: LARGE_INTEGER,
    WriteOperationCount: LARGE_INTEGER,
    OtherOperationCount: LARGE_INTEGER,
    ReadTransferCount: LARGE_INTEGER,
    WriteTransferCount: LARGE_INTEGER,
    OtherTransferCount: LARGE_INTEGER,
    Threads: [SYSTEM_THREAD_INFORMATION; 1],
}}
pub type PSYSTEM_PROCESS_INFORMATION = *mut SYSTEM_PROCESS_INFORMATION;
STRUCT! {struct SYSTEM_CALL_COUNT_INFORMATION {
    Length: c_ulong,
    NumberOfTables: c_ulong,
}}
pub type PSYSTEM_CALL_COUNT_INFORMATION = *mut SYSTEM_CALL_COUNT_INFORMATION;
STRUCT! {struct SYSTEM_DEVICE_INFORMATION {
    NumberOfDisks: c_ulong,
    NumberOfFloppies: c_ulong,
    NumberOfCdRoms: c_ulong,
    NumberOfTapes: c_ulong,
    NumberOfSerialPorts: c_ulong,
    NumberOfParallelPorts: c_ulong,
}}
pub type PSYSTEM_DEVICE_INFORMATION = *mut SYSTEM_DEVICE_INFORMATION;
STRUCT! {struct SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    IdleTime: LARGE_INTEGER,
    KernelTime: LARGE_INTEGER,
    UserTime: LARGE_INTEGER,
    DpcTime: LARGE_INTEGER,
    InterruptTime: LARGE_INTEGER,
    InterruptCount: c_ulong,
}}
pub type PSYSTEM_PROCESSOR_PERFORMANCE_INFORMATION =
    *mut SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION;
STRUCT! {struct SYSTEM_FLAGS_INFORMATION {
    Flags: c_ulong,
}}
pub type PSYSTEM_FLAGS_INFORMATION = *mut SYSTEM_FLAGS_INFORMATION;
STRUCT! {struct SYSTEM_CALL_TIME_INFORMATION {
    Length: c_ulong,
    TotalCalls: c_ulong,
    TimeOfCalls: [LARGE_INTEGER; 1],
}}
pub type PSYSTEM_CALL_TIME_INFORMATION = *mut SYSTEM_CALL_TIME_INFORMATION;
STRUCT! {struct RTL_PROCESS_LOCK_INFORMATION {
    Address: *mut c_void,
    Type: c_ushort,
    CreatorBackTraceIndex: c_ushort,
    OwningThread: HANDLE,
    LockCount: c_long,
    ContentionCount: c_ulong,
    EntryCount: c_ulong,
    RecursionCount: c_long,
    NumberOfWaitingShared: c_ulong,
    NumberOfWaitingExclusive: c_ulong,
}}
pub type PRTL_PROCESS_LOCK_INFORMATION = *mut RTL_PROCESS_LOCK_INFORMATION;
STRUCT! {struct RTL_PROCESS_LOCKS {
    NumberOfLocks: c_ulong,
    Locks: [RTL_PROCESS_LOCK_INFORMATION; 1],
}}
pub type PRTL_PROCESS_LOCKS = *mut RTL_PROCESS_LOCKS;
STRUCT! {struct RTL_PROCESS_BACKTRACE_INFORMATION {
    SymbolicBackTrace: *mut c_char,
    TraceCount: c_ulong,
    Index: c_ushort,
    Depth: c_ushort,
    BackTrace: [*mut c_void; 32],
}}
pub type PRTL_PROCESS_BACKTRACE_INFORMATION =
    *mut RTL_PROCESS_BACKTRACE_INFORMATION;
STRUCT! {struct RTL_PROCESS_BACKTRACES {
    CommittedMemory: c_ulong,
    ReservedMemory: c_ulong,
    NumberOfBackTraceLookups: c_ulong,
    NumberOfBackTraces: c_ulong,
    BackTraces: [RTL_PROCESS_BACKTRACE_INFORMATION; 1],
}}
pub type PRTL_PROCESS_BACKTRACES = *mut RTL_PROCESS_BACKTRACES;
STRUCT! {struct SYSTEM_HANDLE_TABLE_ENTRY_INFO {
    UniqueProcessId: c_ushort,
    CreatorBackTraceIndex: c_ushort,
    ObjectTypeIndex: c_uchar,
    HandleAttributes: c_uchar,
    HandleValue: c_ushort,
    Object: *mut c_void,
    GrantedAccess: c_ulong,
}}
pub type PSYSTEM_HANDLE_TABLE_ENTRY_INFO = *mut SYSTEM_HANDLE_TABLE_ENTRY_INFO;
STRUCT! {struct SYSTEM_HANDLE_INFORMATION {
    NumberOfHandles: c_ulong,
    Handles: [SYSTEM_HANDLE_TABLE_ENTRY_INFO; 1],
}}
pub type PSYSTEM_HANDLE_INFORMATION = *mut SYSTEM_HANDLE_INFORMATION;
STRUCT! {struct SYSTEM_OBJECTTYPE_INFORMATION {
    NextEntryOffset: c_ulong,
    NumberOfObjects: c_ulong,
    NumberOfHandles: c_ulong,
    TypeIndex: c_ulong,
    InvalidAttributes: c_ulong,
    GenericMapping: GENERIC_MAPPING,
    ValidAccessMask: c_ulong,
    PoolType: c_ulong,
    SecurityRequired: c_uchar,
    WaitableObject: c_uchar,
    TypeName: UNICODE_STRING,
}}
pub type PSYSTEM_OBJECTTYPE_INFORMATION = *mut SYSTEM_OBJECTTYPE_INFORMATION;
STRUCT! {struct SYSTEM_OBJECT_INFORMATION {
    NextEntryOffset: c_ulong,
    Object: *mut c_void,
    CreatorUniqueProcess: HANDLE,
    CreatorBackTraceIndex: c_ushort,
    Flags: c_ushort,
    PointerCount: c_long,
    HandleCount: c_long,
    PagedPoolCharge: c_ulong,
    NonPagedPoolCharge: c_ulong,
    ExclusiveProcessId: HANDLE,
    SecurityDescriptor: *mut c_void,
    NameInfo: UNICODE_STRING,
}}
pub type PSYSTEM_OBJECT_INFORMATION = *mut SYSTEM_OBJECT_INFORMATION;
STRUCT! {struct SYSTEM_PAGEFILE_INFORMATION {
    NextEntryOffset: c_ulong,
    TotalSize: c_ulong,
    TotalInUse: c_ulong,
    PeakUsage: c_ulong,
    PageFileName: UNICODE_STRING,
}}
pub type PSYSTEM_PAGEFILE_INFORMATION = *mut SYSTEM_PAGEFILE_INFORMATION;
pub const MM_WORKING_SET_MAX_HARD_ENABLE: c_ulong = 0x1;
pub const MM_WORKING_SET_MAX_HARD_DISABLE: c_ulong = 0x2;
pub const MM_WORKING_SET_MIN_HARD_ENABLE: c_ulong = 0x4;
pub const MM_WORKING_SET_MIN_HARD_DISABLE: c_ulong = 0x8;
STRUCT! {struct SYSTEM_FILECACHE_INFORMATION {
    CurrentSize: usize,
    PeakSize: usize,
    PageFaultCount: c_ulong,
    MinimumWorkingSet: usize,
    MaximumWorkingSet: usize,
    CurrentSizeIncludingTransitionInPages: usize,
    PeakSizeIncludingTransitionInPages: usize,
    TransitionRePurposeCount: c_ulong,
    Flags: c_ulong,
}}
pub type PSYSTEM_FILECACHE_INFORMATION = *mut SYSTEM_FILECACHE_INFORMATION;
STRUCT! {struct SYSTEM_BASIC_WORKING_SET_INFORMATION {
    CurrentSize: usize,
    PeakSize: usize,
    PageFaultCount: c_ulong,
}}
pub type PSYSTEM_BASIC_WORKING_SET_INFORMATION =
    *mut SYSTEM_BASIC_WORKING_SET_INFORMATION;
UNION! {union SYSTEM_POOLTAG_u {
    Tag: [c_uchar; 4],
    TagUlong: c_ulong,
}}
STRUCT! {struct SYSTEM_POOLTAG {
    u: SYSTEM_POOLTAG_u,
    PagedAllocs: c_ulong,
    PagedFrees: c_ulong,
    PagedUsed: usize,
    NonPagedAllocs: c_ulong,
    NonPagedFrees: c_ulong,
    NonPagedUsed: usize,
}}
pub type PSYSTEM_POOLTAG = *mut SYSTEM_POOLTAG;
STRUCT! {struct SYSTEM_POOLTAG_INFORMATION {
    Count: c_ulong,
    TagInfo: [SYSTEM_POOLTAG; 1],
}}
pub type PSYSTEM_POOLTAG_INFORMATION = *mut SYSTEM_POOLTAG_INFORMATION;
STRUCT! {struct SYSTEM_INTERRUPT_INFORMATION {
    ContextSwitches: c_ulong,
    DpcCount: c_ulong,
    DpcRate: c_ulong,
    TimeIncrement: c_ulong,
    DpcBypassCount: c_ulong,
    ApcBypassCount: c_ulong,
}}
pub type PSYSTEM_INTERRUPT_INFORMATION = *mut SYSTEM_INTERRUPT_INFORMATION;
STRUCT! {struct SYSTEM_DPC_BEHAVIOR_INFORMATION {
    Spare: c_ulong,
    DpcQueueDepth: c_ulong,
    MinimumDpcRate: c_ulong,
    AdjustDpcThreshold: c_ulong,
    IdealDpcRate: c_ulong,
}}
pub type PSYSTEM_DPC_BEHAVIOR_INFORMATION =
    *mut SYSTEM_DPC_BEHAVIOR_INFORMATION;
STRUCT! {struct SYSTEM_QUERY_TIME_ADJUST_INFORMATION {
    TimeAdjustment: c_ulong,
    TimeIncrement: c_ulong,
    Enable: c_uchar,
}}
pub type PSYSTEM_QUERY_TIME_ADJUST_INFORMATION =
    *mut SYSTEM_QUERY_TIME_ADJUST_INFORMATION;
STRUCT! {struct SYSTEM_QUERY_TIME_ADJUST_INFORMATION_PRECISE {
    TimeAdjustment: __uint64,
    TimeIncrement: __uint64,
    Enable: c_uchar,
}}
pub type PSYSTEM_QUERY_TIME_ADJUST_INFORMATION_PRECISE =
    *mut SYSTEM_QUERY_TIME_ADJUST_INFORMATION_PRECISE;
STRUCT! {struct SYSTEM_SET_TIME_ADJUST_INFORMATION {
    TimeAdjustment: c_ulong,
    Enable: c_uchar,
}}
pub type PSYSTEM_SET_TIME_ADJUST_INFORMATION =
    *mut SYSTEM_SET_TIME_ADJUST_INFORMATION;
STRUCT! {struct SYSTEM_SET_TIME_ADJUST_INFORMATION_PRECISE {
    TimeAdjustment: __uint64,
    Enable: c_uchar,
}}
pub type PSYSTEM_SET_TIME_ADJUST_INFORMATION_PRECISE =
    *mut SYSTEM_SET_TIME_ADJUST_INFORMATION_PRECISE;
ENUM! {enum EVENT_TRACE_INFORMATION_CLASS {
    EventTraceKernelVersionInformation = 0,
    EventTraceGroupMaskInformation = 1,
    EventTracePerformanceInformation = 2,
    EventTraceTimeProfileInformation = 3,
    EventTraceSessionSecurityInformation = 4,
    EventTraceSpinlockInformation = 5,
    EventTraceStackTracingInformation = 6,
    EventTraceExecutiveResourceInformation = 7,
    EventTraceHeapTracingInformation = 8,
    EventTraceHeapSummaryTracingInformation = 9,
    EventTracePoolTagFilterInformation = 10,
    EventTracePebsTracingInformation = 11,
    EventTraceProfileConfigInformation = 12,
    EventTraceProfileSourceListInformation = 13,
    EventTraceProfileEventListInformation = 14,
    EventTraceProfileCounterListInformation = 15,
    EventTraceStackCachingInformation = 16,
    EventTraceObjectTypeFilterInformation = 17,
    EventTraceSoftRestartInformation = 18,
    EventTraceLastBranchConfigurationInformation = 19,
    EventTraceLastBranchEventListInformation = 20,
    EventTraceProfileSourceAddInformation = 21,
    EventTraceProfileSourceRemoveInformation = 22,
    EventTraceProcessorTraceConfigurationInformation = 23,
    EventTraceProcessorTraceEventListInformation = 24,
    EventTraceCoverageSamplerInformation = 25,
    MaxEventTraceInfoClass = 26,
}}
STRUCT! {struct EVENT_TRACE_VERSION_INFORMATION {
    EventTraceInformationClass: EVENT_TRACE_INFORMATION_CLASS,
    EventTraceKernelVersion: c_ulong,
}}
pub type PEVENT_TRACE_VERSION_INFORMATION =
    *mut EVENT_TRACE_VERSION_INFORMATION;
STRUCT! {struct PERFINFO_GROUPMASK {
    Masks: [c_ulong; 8],
}}
pub type PPERFINFO_GROUPMASK = *mut PERFINFO_GROUPMASK;
STRUCT! {struct EVENT_TRACE_GROUPMASK_INFORMATION {
    EventTraceInformationClass: EVENT_TRACE_INFORMATION_CLASS,
    TraceHandle: HANDLE,
    EventTraceGroupMasks: PERFINFO_GROUPMASK,
}}
pub type PEVENT_TRACE_GROUPMASK_INFORMATION =
    *mut EVENT_TRACE_GROUPMASK_INFORMATION;
STRUCT! {struct EVENT_TRACE_PERFORMANCE_INFORMATION {
    EventTraceInformationClass: EVENT_TRACE_INFORMATION_CLASS,
    LogfileBytesWritten: LARGE_INTEGER,
}}
pub type PEVENT_TRACE_PERFORMANCE_INFORMATION =
    *mut EVENT_TRACE_PERFORMANCE_INFORMATION;
STRUCT! {struct EVENT_TRACE_TIME_PROFILE_INFORMATION {
    EventTraceInformationClass: EVENT_TRACE_INFORMATION_CLASS,
    ProfileInterval: c_ulong,
}}
pub type PEVENT_TRACE_TIME_PROFILE_INFORMATION =
    *mut EVENT_TRACE_TIME_PROFILE_INFORMATION;
STRUCT! {struct EVENT_TRACE_SESSION_SECURITY_INFORMATION {
    EventTraceInformationClass: EVENT_TRACE_INFORMATION_CLASS,
    SecurityInformation: c_ulong,
    TraceHandle: HANDLE,
    SecurityDescriptor: [c_uchar; 1],
}}
pub type PEVENT_TRACE_SESSION_SECURITY_INFORMATION =
    *mut EVENT_TRACE_SESSION_SECURITY_INFORMATION;
STRUCT! {struct EVENT_TRACE_SPINLOCK_INFORMATION {
    EventTraceInformationClass: EVENT_TRACE_INFORMATION_CLASS,
    SpinLockSpinThreshold: c_ulong,
    SpinLockAcquireSampleRate: c_ulong,
    SpinLockContentionSampleRate: c_ulong,
    SpinLockHoldThreshold: c_ulong,
}}
pub type PEVENT_TRACE_SPINLOCK_INFORMATION =
    *mut EVENT_TRACE_SPINLOCK_INFORMATION;
STRUCT! {struct EVENT_TRACE_SYSTEM_EVENT_INFORMATION {
    EventTraceInformationClass: EVENT_TRACE_INFORMATION_CLASS,
    TraceHandle: HANDLE,
    HookId: [c_ulong; 1],
}}
pub type PEVENT_TRACE_SYSTEM_EVENT_INFORMATION =
    *mut EVENT_TRACE_SYSTEM_EVENT_INFORMATION;
STRUCT! {struct EVENT_TRACE_EXECUTIVE_RESOURCE_INFORMATION {
    EventTraceInformationClass: EVENT_TRACE_INFORMATION_CLASS,
    ReleaseSamplingRate: c_ulong,
    ContentionSamplingRate: c_ulong,
    NumberOfExcessiveTimeouts: c_ulong,
}}
pub type PEVENT_TRACE_EXECUTIVE_RESOURCE_INFORMATION =
    *mut EVENT_TRACE_EXECUTIVE_RESOURCE_INFORMATION;
STRUCT! {struct EVENT_TRACE_HEAP_TRACING_INFORMATION {
    EventTraceInformationClass: EVENT_TRACE_INFORMATION_CLASS,
    ProcessId: c_ulong,
}}
pub type PEVENT_TRACE_HEAP_TRACING_INFORMATION =
    *mut EVENT_TRACE_HEAP_TRACING_INFORMATION;
STRUCT! {struct EVENT_TRACE_TAG_FILTER_INFORMATION {
    EventTraceInformationClass: EVENT_TRACE_INFORMATION_CLASS,
    TraceHandle: HANDLE,
    Filter: [c_ulong; 1],
}}
pub type PEVENT_TRACE_TAG_FILTER_INFORMATION =
    *mut EVENT_TRACE_TAG_FILTER_INFORMATION;
STRUCT! {struct EVENT_TRACE_PROFILE_COUNTER_INFORMATION {
    EventTraceInformationClass: EVENT_TRACE_INFORMATION_CLASS,
    TraceHandle: HANDLE,
    ProfileSource: [c_ulong; 1],
}}
pub type PEVENT_TRACE_PROFILE_COUNTER_INFORMATION =
    *mut EVENT_TRACE_PROFILE_COUNTER_INFORMATION;
STRUCT! {struct EVENT_TRACE_PROFILE_LIST_INFORMATION {
    EventTraceInformationClass: EVENT_TRACE_INFORMATION_CLASS,
    Spare: c_ulong,
    Profile: [*mut PROFILE_SOURCE_INFO; 1],
}}
pub type PEVENT_TRACE_PROFILE_LIST_INFORMATION =
    *mut EVENT_TRACE_PROFILE_LIST_INFORMATION;
STRUCT! {struct EVENT_TRACE_STACK_CACHING_INFORMATION {
    EventTraceInformationClass: EVENT_TRACE_INFORMATION_CLASS,
    TraceHandle: HANDLE,
    Enabled: c_uchar,
    Reserved: [c_uchar; 3],
    CacheSize: c_ulong,
    BucketCount: c_ulong,
}}
pub type PEVENT_TRACE_STACK_CACHING_INFORMATION =
    *mut EVENT_TRACE_STACK_CACHING_INFORMATION;
STRUCT! {struct EVENT_TRACE_SOFT_RESTART_INFORMATION {
    EventTraceInformationClass: EVENT_TRACE_INFORMATION_CLASS,
    TraceHandle: HANDLE,
    PersistTraceBuffers: c_uchar,
    FileName: [wchar_t; 1],
}}
pub type PEVENT_TRACE_SOFT_RESTART_INFORMATION =
    *mut EVENT_TRACE_SOFT_RESTART_INFORMATION;
STRUCT! {struct EVENT_TRACE_PROFILE_ADD_INFORMATION {
    EventTraceInformationClass: EVENT_TRACE_INFORMATION_CLASS,
    PerfEvtEventSelect: c_uchar,
    PerfEvtUnitSelect: c_uchar,
    PerfEvtType: c_ulong,
    CpuInfoHierarchy: [c_ulong; 3],
    InitialInterval: c_ulong,
    AllowsHalt: c_uchar,
    Persist: c_uchar,
    ProfileSourceDescription: [wchar_t; 1],
}}
pub type PEVENT_TRACE_PROFILE_ADD_INFORMATION =
    *mut EVENT_TRACE_PROFILE_ADD_INFORMATION;
STRUCT! {struct EVENT_TRACE_PROFILE_REMOVE_INFORMATION {
    EventTraceInformationClass: EVENT_TRACE_INFORMATION_CLASS,
    ProfileSource: KPROFILE_SOURCE,
    CpuInfoHierarchy: [c_ulong; 3],
}}
pub type PEVENT_TRACE_PROFILE_REMOVE_INFORMATION =
    *mut EVENT_TRACE_PROFILE_REMOVE_INFORMATION;
STRUCT! {struct EVENT_TRACE_COVERAGE_SAMPLER_INFORMATION {
    EventTraceInformationClass: EVENT_TRACE_INFORMATION_CLASS,
    CoverageSamplerInformationClass: c_uchar,
    MajorVersion: c_uchar,
    MinorVersion: c_uchar,
    Reserved: c_uchar,
    SamplerHandle: HANDLE,
}}
pub type PEVENT_TRACE_COVERAGE_SAMPLER_INFORMATION =
    *mut EVENT_TRACE_COVERAGE_SAMPLER_INFORMATION;
STRUCT! {struct SYSTEM_EXCEPTION_INFORMATION {
    AlignmentFixupCount: c_ulong,
    ExceptionDispatchCount: c_ulong,
    FloatingEmulationCount: c_ulong,
    ByteWordEmulationCount: c_ulong,
}}
pub type PSYSTEM_EXCEPTION_INFORMATION = *mut SYSTEM_EXCEPTION_INFORMATION;
STRUCT! {struct SYSTEM_KERNEL_DEBUGGER_INFORMATION {
    KernelDebuggerEnabled: c_uchar,
    KernelDebuggerNotPresent: c_uchar,
}}
pub type PSYSTEM_KERNEL_DEBUGGER_INFORMATION =
    *mut SYSTEM_KERNEL_DEBUGGER_INFORMATION;
STRUCT! {struct SYSTEM_CONTEXT_SWITCH_INFORMATION {
    ContextSwitches: c_ulong,
    FindAny: c_ulong,
    FindLast: c_ulong,
    FindIdeal: c_ulong,
    IdleAny: c_ulong,
    IdleCurrent: c_ulong,
    IdleLast: c_ulong,
    IdleIdeal: c_ulong,
    PreemptAny: c_ulong,
    PreemptCurrent: c_ulong,
    PreemptLast: c_ulong,
    SwitchToIdle: c_ulong,
}}
pub type PSYSTEM_CONTEXT_SWITCH_INFORMATION =
    *mut SYSTEM_CONTEXT_SWITCH_INFORMATION;
STRUCT! {struct SYSTEM_REGISTRY_QUOTA_INFORMATION {
    RegistryQuotaAllowed: c_ulong,
    RegistryQuotaUsed: c_ulong,
    PagedPoolSize: usize,
}}
pub type PSYSTEM_REGISTRY_QUOTA_INFORMATION =
    *mut SYSTEM_REGISTRY_QUOTA_INFORMATION;
STRUCT! {struct SYSTEM_PROCESSOR_IDLE_INFORMATION {
    IdleTime: __uint64,
    C1Time: __uint64,
    C2Time: __uint64,
    C3Time: __uint64,
    C1Transitions: c_ulong,
    C2Transitions: c_ulong,
    C3Transitions: c_ulong,
    Padding: c_ulong,
}}
pub type PSYSTEM_PROCESSOR_IDLE_INFORMATION =
    *mut SYSTEM_PROCESSOR_IDLE_INFORMATION;
STRUCT! {struct SYSTEM_LEGACY_DRIVER_INFORMATION {
    VetoType: c_ulong,
    VetoList: UNICODE_STRING,
}}
pub type PSYSTEM_LEGACY_DRIVER_INFORMATION =
    *mut SYSTEM_LEGACY_DRIVER_INFORMATION;
STRUCT! {struct SYSTEM_LOOKASIDE_INFORMATION {
    CurrentDepth: c_ushort,
    MaximumDepth: c_ushort,
    TotalAllocates: c_ulong,
    AllocateMisses: c_ulong,
    TotalFrees: c_ulong,
    FreeMisses: c_ulong,
    Type: c_ulong,
    Tag: c_ulong,
    Size: c_ulong,
}}
pub type PSYSTEM_LOOKASIDE_INFORMATION = *mut SYSTEM_LOOKASIDE_INFORMATION;
STRUCT! {struct SYSTEM_RANGE_START_INFORMATION {
    SystemRangeStart: *mut c_void,
}}
pub type PSYSTEM_RANGE_START_INFORMATION = *mut SYSTEM_RANGE_START_INFORMATION;
STRUCT! {struct SYSTEM_VERIFIER_INFORMATION {
    NextEntryOffset: c_ulong,
    Level: c_ulong,
    DriverName: UNICODE_STRING,
    RaiseIrqls: c_ulong,
    AcquireSpinLocks: c_ulong,
    SynchronizeExecutions: c_ulong,
    AllocationsAttempted: c_ulong,
    AllocationsSucceeded: c_ulong,
    AllocationsSucceededSpecialPool: c_ulong,
    AllocationsWithNoTag: c_ulong,
    TrimRequests: c_ulong,
    Trims: c_ulong,
    AllocationsFailed: c_ulong,
    AllocationsFailedDeliberately: c_ulong,
    Loads: c_ulong,
    Unloads: c_ulong,
    UnTrackedPool: c_ulong,
    CurrentPagedPoolAllocations: c_ulong,
    CurrentNonPagedPoolAllocations: c_ulong,
    PeakPagedPoolAllocations: c_ulong,
    PeakNonPagedPoolAllocations: c_ulong,
    PagedPoolUsageInBytes: usize,
    NonPagedPoolUsageInBytes: usize,
    PeakPagedPoolUsageInBytes: usize,
    PeakNonPagedPoolUsageInBytes: usize,
}}
pub type PSYSTEM_VERIFIER_INFORMATION = *mut SYSTEM_VERIFIER_INFORMATION;
STRUCT! {struct SYSTEM_SESSION_PROCESS_INFORMATION {
    SessionId: c_ulong,
    SizeOfBuf: c_ulong,
    Buffer: *mut c_void,
}}
pub type PSYSTEM_SESSION_PROCESS_INFORMATION =
    *mut SYSTEM_SESSION_PROCESS_INFORMATION;
STRUCT! {struct SYSTEM_PROCESSOR_POWER_INFORMATION {
    CurrentFrequency: c_uchar,
    ThermalLimitFrequency: c_uchar,
    ConstantThrottleFrequency: c_uchar,
    DegradedThrottleFrequency: c_uchar,
    LastBusyFrequency: c_uchar,
    LastC3Frequency: c_uchar,
    LastAdjustedBusyFrequency: c_uchar,
    ProcessorMinThrottle: c_uchar,
    ProcessorMaxThrottle: c_uchar,
    NumberOfFrequencies: c_ulong,
    PromotionCount: c_ulong,
    DemotionCount: c_ulong,
    ErrorCount: c_ulong,
    RetryCount: c_ulong,
    CurrentFrequencyTime: __uint64,
    CurrentProcessorTime: __uint64,
    CurrentProcessorIdleTime: __uint64,
    LastProcessorTime: __uint64,
    LastProcessorIdleTime: __uint64,
    Energy: __uint64,
}}
pub type PSYSTEM_PROCESSOR_POWER_INFORMATION =
    *mut SYSTEM_PROCESSOR_POWER_INFORMATION;
STRUCT! {struct SYSTEM_HANDLE_TABLE_ENTRY_INFO_EX {
    Object: *mut c_void,
    UniqueProcessId: usize,
    HandleValue: usize,
    GrantedAccess: c_ulong,
    CreatorBackTraceIndex: c_ushort,
    ObjectTypeIndex: c_ushort,
    HandleAttributes: c_ulong,
    Reserved: c_ulong,
}}
pub type PSYSTEM_HANDLE_TABLE_ENTRY_INFO_EX =
    *mut SYSTEM_HANDLE_TABLE_ENTRY_INFO_EX;
STRUCT! {struct SYSTEM_HANDLE_INFORMATION_EX {
    NumberOfHandles: usize,
    Reserved: usize,
    Handles: [SYSTEM_HANDLE_TABLE_ENTRY_INFO_EX; 1],
}}
pub type PSYSTEM_HANDLE_INFORMATION_EX = *mut SYSTEM_HANDLE_INFORMATION_EX;
UNION! {union SYSTEM_BIGPOOL_ENTRY_u1 {
    VirtualAddress: *mut c_void,
    Bitfields: usize,
}}
UNION! {union SYSTEM_BIGPOOL_ENTRY_u2 {
    Tag: [c_uchar; 4],
    TagUlong: c_ulong,
}}
BITFIELD! {unsafe SYSTEM_BIGPOOL_ENTRY_u1 Bitfields: usize [
    NonPaged set_NonPaged[0..1],
]}
STRUCT! {struct SYSTEM_BIGPOOL_ENTRY {
    u1: SYSTEM_BIGPOOL_ENTRY_u1,
    SizeInBytes: usize,
    u2: SYSTEM_BIGPOOL_ENTRY_u2,
}}
pub type PSYSTEM_BIGPOOL_ENTRY = *mut SYSTEM_BIGPOOL_ENTRY;
STRUCT! {struct SYSTEM_BIGPOOL_INFORMATION {
    Count: c_ulong,
    AllocatedInfo: [SYSTEM_BIGPOOL_ENTRY; 1],
}}
pub type PSYSTEM_BIGPOOL_INFORMATION = *mut SYSTEM_BIGPOOL_INFORMATION;
UNION! {union SYSTEM_POOL_ENTRY_u {
    Tag: [c_uchar; 4],
    TagUlong: c_ulong,
    ProcessChargedQuota: *mut c_void,
}}
STRUCT! {struct SYSTEM_POOL_ENTRY {
    Allocated: c_uchar,
    Spare0: c_uchar,
    AllocatorBackTraceIndex: c_ushort,
    Size: c_ulong,
    u: SYSTEM_POOL_ENTRY_u,
}}
pub type PSYSTEM_POOL_ENTRY = *mut SYSTEM_POOL_ENTRY;
STRUCT! {struct SYSTEM_POOL_INFORMATION {
    TotalSize: usize,
    FirstEntry: *mut c_void,
    EntryOverhead: c_ushort,
    PoolTagPresent: c_uchar,
    Spare0: c_uchar,
    NumberOfEntries: c_ulong,
    Entries: [SYSTEM_POOL_ENTRY; 1],
}}
pub type PSYSTEM_POOL_INFORMATION = *mut SYSTEM_POOL_INFORMATION;
STRUCT! {struct SYSTEM_SESSION_POOLTAG_INFORMATION {
    NextEntryOffset: usize,
    SessionId: c_ulong,
    Count: c_ulong,
    TagInfo: [SYSTEM_POOLTAG; 1],
}}
pub type PSYSTEM_SESSION_POOLTAG_INFORMATION =
    *mut SYSTEM_SESSION_POOLTAG_INFORMATION;
STRUCT! {struct SYSTEM_SESSION_MAPPED_VIEW_INFORMATION {
    NextEntryOffset: usize,
    SessionId: c_ulong,
    ViewFailures: c_ulong,
    NumberOfBytesAvailable: usize,
    NumberOfBytesAvailableContiguous: usize,
}}
pub type PSYSTEM_SESSION_MAPPED_VIEW_INFORMATION =
    *mut SYSTEM_SESSION_MAPPED_VIEW_INFORMATION;
ENUM! {enum SYSTEM_FIRMWARE_TABLE_ACTION {
    SystemFirmwareTableEnumerate = 0,
    SystemFirmwareTableGet = 1,
    SystemFirmwareTableMax = 2,
}}
STRUCT! {struct SYSTEM_FIRMWARE_TABLE_INFORMATION {
    ProviderSignature: c_ulong,
    Action: SYSTEM_FIRMWARE_TABLE_ACTION,
    TableID: c_ulong,
    TableBufferLength: c_ulong,
    TableBuffer: [c_uchar; 1],
}}
pub type PSYSTEM_FIRMWARE_TABLE_INFORMATION =
    *mut SYSTEM_FIRMWARE_TABLE_INFORMATION;
STRUCT! {struct SYSTEM_MEMORY_LIST_INFORMATION {
    ZeroPageCount: usize,
    FreePageCount: usize,
    ModifiedPageCount: usize,
    ModifiedNoWritePageCount: usize,
    BadPageCount: usize,
    PageCountByPriority: [usize; 8],
    RepurposedPagesByPriority: [usize; 8],
    ModifiedPageCountPageFile: usize,
}}
pub type PSYSTEM_MEMORY_LIST_INFORMATION = *mut SYSTEM_MEMORY_LIST_INFORMATION;
ENUM! {enum SYSTEM_MEMORY_LIST_COMMAND {
    MemoryCaptureAccessedBits = 0,
    MemoryCaptureAndResetAccessedBits = 1,
    MemoryEmptyWorkingSets = 2,
    MemoryFlushModifiedList = 3,
    MemoryPurgeStandbyList = 4,
    MemoryPurgeLowPriorityStandbyList = 5,
    MemoryCommandMax = 6,
}}
STRUCT! {struct SYSTEM_THREAD_CID_PRIORITY_INFORMATION {
    ClientId: CLIENT_ID,
    Priority: KPRIORITY,
}}
pub type PSYSTEM_THREAD_CID_PRIORITY_INFORMATION =
    *mut SYSTEM_THREAD_CID_PRIORITY_INFORMATION;
STRUCT! {struct SYSTEM_PROCESSOR_IDLE_CYCLE_TIME_INFORMATION {
    CycleTime: __uint64,
}}
pub type PSYSTEM_PROCESSOR_IDLE_CYCLE_TIME_INFORMATION =
    *mut SYSTEM_PROCESSOR_IDLE_CYCLE_TIME_INFORMATION;
STRUCT! {struct SYSTEM_REF_TRACE_INFORMATION {
    TraceEnable: c_uchar,
    TracePermanent: c_uchar,
    TraceProcessName: UNICODE_STRING,
    TracePoolTags: UNICODE_STRING,
}}
pub type PSYSTEM_REF_TRACE_INFORMATION = *mut SYSTEM_REF_TRACE_INFORMATION;
STRUCT! {struct SYSTEM_PROCESS_ID_INFORMATION {
    ProcessId: HANDLE,
    ImageName: UNICODE_STRING,
}}
pub type PSYSTEM_PROCESS_ID_INFORMATION = *mut SYSTEM_PROCESS_ID_INFORMATION;
STRUCT! {struct SYSTEM_BOOT_ENVIRONMENT_INFORMATION {
    BootIdentifier: GUID,
    FirmwareType: FIRMWARE_TYPE,
    BootFlags: __uint64,
}}
BITFIELD! {SYSTEM_BOOT_ENVIRONMENT_INFORMATION BootFlags: __uint64 [
    DbgMenuOsSelection set_DbgMenuOsSelection[0..1],
    DbgHiberBoot set_DbgHiberBoot[1..2],
    DbgSoftBoot set_DbgSoftBoot[2..3],
    DbgMeasuredLaunch set_DbgMeasuredLaunch[3..4],
]}
pub type PSYSTEM_BOOT_ENVIRONMENT_INFORMATION =
    *mut SYSTEM_BOOT_ENVIRONMENT_INFORMATION;
STRUCT! {struct SYSTEM_IMAGE_FILE_EXECUTION_OPTIONS_INFORMATION {
    FlagsToEnable: c_ulong,
    FlagsToDisable: c_ulong,
}}
pub type PSYSTEM_IMAGE_FILE_EXECUTION_OPTIONS_INFORMATION =
    *mut SYSTEM_IMAGE_FILE_EXECUTION_OPTIONS_INFORMATION;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
STRUCT! {struct SYSTEM_VERIFIER_INFORMATION_EX {
    VerifyMode: c_ulong,
    OptionChanges: c_ulong,
    PreviousBucketName: UNICODE_STRING,
    IrpCancelTimeoutMsec: c_ulong,
    VerifierExtensionEnabled: c_ulong,
    Reserved: [c_ulong; 1],
}}
#[cfg(target_arch = "x86")]
STRUCT! {struct SYSTEM_VERIFIER_INFORMATION_EX {
    VerifyMode: c_ulong,
    OptionChanges: c_ulong,
    PreviousBucketName: UNICODE_STRING,
    IrpCancelTimeoutMsec: c_ulong,
    VerifierExtensionEnabled: c_ulong,
    Reserved: [c_ulong; 3],
}}
pub type PSYSTEM_VERIFIER_INFORMATION_EX = *mut SYSTEM_VERIFIER_INFORMATION_EX;
STRUCT! {struct SYSTEM_SYSTEM_PARTITION_INFORMATION {
    SystemPartition: UNICODE_STRING,
}}
pub type PSYSTEM_SYSTEM_PARTITION_INFORMATION =
    *mut SYSTEM_SYSTEM_PARTITION_INFORMATION;
STRUCT! {struct SYSTEM_SYSTEM_DISK_INFORMATION {
    SystemDisk: UNICODE_STRING,
}}
pub type PSYSTEM_SYSTEM_DISK_INFORMATION = *mut SYSTEM_SYSTEM_DISK_INFORMATION;
STRUCT! {struct SYSTEM_PROCESSOR_PERFORMANCE_HITCOUNT {
    Hits: __uint64,
    PercentFrequency: c_uchar,
}}
pub type PSYSTEM_PROCESSOR_PERFORMANCE_HITCOUNT =
    *mut SYSTEM_PROCESSOR_PERFORMANCE_HITCOUNT;
STRUCT! {struct SYSTEM_PROCESSOR_PERFORMANCE_HITCOUNT_WIN8 {
    Hits: c_ulong,
    PercentFrequency: c_uchar,
}}
pub type PSYSTEM_PROCESSOR_PERFORMANCE_HITCOUNT_WIN8 =
    *mut SYSTEM_PROCESSOR_PERFORMANCE_HITCOUNT_WIN8;
STRUCT! {struct SYSTEM_PROCESSOR_PERFORMANCE_STATE_DISTRIBUTION {
    ProcessorNumber: c_ulong,
    StateCount: c_ulong,
    States: [SYSTEM_PROCESSOR_PERFORMANCE_HITCOUNT; 1],
}}
pub type PSYSTEM_PROCESSOR_PERFORMANCE_STATE_DISTRIBUTION =
    *mut SYSTEM_PROCESSOR_PERFORMANCE_STATE_DISTRIBUTION;
STRUCT! {struct SYSTEM_PROCESSOR_PERFORMANCE_DISTRIBUTION {
    ProcessorCount: c_ulong,
    Offsets: [c_ulong; 1],
}}
pub type PSYSTEM_PROCESSOR_PERFORMANCE_DISTRIBUTION =
    *mut SYSTEM_PROCESSOR_PERFORMANCE_DISTRIBUTION;
STRUCT! {struct SYSTEM_CODEINTEGRITY_INFORMATION {
    Length: c_ulong,
    CodeIntegrityOptions: c_ulong,
}}
pub type PSYSTEM_CODEINTEGRITY_INFORMATION =
    *mut SYSTEM_CODEINTEGRITY_INFORMATION;
ENUM! {enum SYSTEM_VA_TYPE {
    SystemVaTypeAll = 0,
    SystemVaTypeNonPagedPool = 1,
    SystemVaTypePagedPool = 2,
    SystemVaTypeSystemCache = 3,
    SystemVaTypeSystemPtes = 4,
    SystemVaTypeSessionSpace = 5,
    SystemVaTypeMax = 6,
}}
pub type PSYSTEM_VA_TYPE = *mut SYSTEM_VA_TYPE;
STRUCT! {struct SYSTEM_VA_LIST_INFORMATION {
    VirtualSize: usize,
    VirtualPeak: usize,
    VirtualLimit: usize,
    AllocationFailures: usize,
}}
pub type PSYSTEM_VA_LIST_INFORMATION = *mut SYSTEM_VA_LIST_INFORMATION;
STRUCT! {struct SYSTEM_REGISTRY_APPEND_STRING_PARAMETERS {
    KeyHandle: HANDLE,
    ValueNamePointer: *mut UNICODE_STRING,
    RequiredLengthPointer: *mut c_ulong,
    Buffer: *mut c_uchar,
    BufferLength: c_ulong,
    Type: c_ulong,
    AppendBuffer: *mut c_uchar,
    AppendBufferLength: c_ulong,
    CreateIfDoesntExist: c_uchar,
    TruncateExistingValue: c_uchar,
}}
pub type PSYSTEM_REGISTRY_APPEND_STRING_PARAMETERS =
    *mut SYSTEM_REGISTRY_APPEND_STRING_PARAMETERS;
STRUCT! {struct SYSTEM_VHD_BOOT_INFORMATION {
    OsDiskIsVhd: c_uchar,
    OsVhdFilePathOffset: c_ulong,
    OsVhdParentVolume: [wchar_t; ANYSIZE_ARRAY as usize],
}}
pub type PSYSTEM_VHD_BOOT_INFORMATION = *mut SYSTEM_VHD_BOOT_INFORMATION;
STRUCT! {struct SYSTEM_LOW_PRIORITY_IO_INFORMATION {
    LowPriReadOperations: c_ulong,
    LowPriWriteOperations: c_ulong,
    KernelBumpedToNormalOperations: c_ulong,
    LowPriPagingReadOperations: c_ulong,
    KernelPagingReadsBumpedToNormal: c_ulong,
    LowPriPagingWriteOperations: c_ulong,
    KernelPagingWritesBumpedToNormal: c_ulong,
    BoostedIrpCount: c_ulong,
    BoostedPagingIrpCount: c_ulong,
    BlanketBoostCount: c_ulong,
}}
pub type PSYSTEM_LOW_PRIORITY_IO_INFORMATION =
    *mut SYSTEM_LOW_PRIORITY_IO_INFORMATION;
ENUM! {enum TPM_BOOT_ENTROPY_RESULT_CODE {
    TpmBootEntropyStructureUninitialized = 0,
    TpmBootEntropyDisabledByPolicy = 1,
    TpmBootEntropyNoTpmFound = 2,
    TpmBootEntropyTpmError = 3,
    TpmBootEntropySuccess = 4,
}}
STRUCT! {struct TPM_BOOT_ENTROPY_NT_RESULT {
    Policy: __uint64,
    ResultCode: TPM_BOOT_ENTROPY_RESULT_CODE,
    ResultStatus: NTSTATUS,
    Time: __uint64,
    EntropyLength: c_ulong,
    EntropyData: [c_uchar; 40],
}}
pub type PTPM_BOOT_ENTROPY_NT_RESULT = *mut TPM_BOOT_ENTROPY_NT_RESULT;
STRUCT! {struct SYSTEM_VERIFIER_COUNTERS_INFORMATION {
    Legacy: SYSTEM_VERIFIER_INFORMATION,
    RaiseIrqls: c_ulong,
    AcquireSpinLocks: c_ulong,
    SynchronizeExecutions: c_ulong,
    AllocationsWithNoTag: c_ulong,
    AllocationsFailed: c_ulong,
    AllocationsFailedDeliberately: c_ulong,
    LockedBytes: usize,
    PeakLockedBytes: usize,
    MappedLockedBytes: usize,
    PeakMappedLockedBytes: usize,
    MappedIoSpaceBytes: usize,
    PeakMappedIoSpaceBytes: usize,
    PagesForMdlBytes: usize,
    PeakPagesForMdlBytes: usize,
    ContiguousMemoryBytes: usize,
    PeakContiguousMemoryBytes: usize,
    ExecutePoolTypes: c_ulong,
    ExecutePageProtections: c_ulong,
    ExecutePageMappings: c_ulong,
    ExecuteWriteSections: c_ulong,
    SectionAlignmentFailures: c_ulong,
    UnsupportedRelocs: c_ulong,
    IATInExecutableSection: c_ulong,
}}
pub type PSYSTEM_VERIFIER_COUNTERS_INFORMATION =
    *mut SYSTEM_VERIFIER_COUNTERS_INFORMATION;
STRUCT! {struct SYSTEM_ACPI_AUDIT_INFORMATION {
    RsdpCount: c_ulong,
    Bitfields: c_ulong,
}}
BITFIELD! {SYSTEM_ACPI_AUDIT_INFORMATION Bitfields: c_ulong [
    SameRsdt set_SameRsdt[0..1],
    SlicPresent set_SlicPresent[1..2],
    SlicDifferent set_SlicDifferent[2..3],
]}
pub type PSYSTEM_ACPI_AUDIT_INFORMATION = *mut SYSTEM_ACPI_AUDIT_INFORMATION;
STRUCT! {struct SYSTEM_BASIC_PERFORMANCE_INFORMATION {
    AvailablePages: usize,
    CommittedPages: usize,
    CommitLimit: usize,
    PeakCommitment: usize,
}}
pub type PSYSTEM_BASIC_PERFORMANCE_INFORMATION =
    *mut SYSTEM_BASIC_PERFORMANCE_INFORMATION;
STRUCT! {struct QUERY_PERFORMANCE_COUNTER_FLAGS {
    ul: c_ulong,
}}
BITFIELD! {QUERY_PERFORMANCE_COUNTER_FLAGS ul: c_ulong [
    KernelTransition set_KernelTransition[0..1],
    Reserved set_Reserved[1..32],
]}
STRUCT! {struct SYSTEM_QUERY_PERFORMANCE_COUNTER_INFORMATION {
    Version: c_ulong,
    Flags: QUERY_PERFORMANCE_COUNTER_FLAGS,
    ValidFlags: QUERY_PERFORMANCE_COUNTER_FLAGS,
}}
pub type PSYSTEM_QUERY_PERFORMANCE_COUNTER_INFORMATION =
    *mut SYSTEM_QUERY_PERFORMANCE_COUNTER_INFORMATION;
ENUM! {enum SYSTEM_PIXEL_FORMAT {
    SystemPixelFormatUnknown = 0,
    SystemPixelFormatR8G8B8 = 1,
    SystemPixelFormatR8G8B8X8 = 2,
    SystemPixelFormatB8G8R8 = 3,
    SystemPixelFormatB8G8R8X8 = 4,
}}
STRUCT! {struct SYSTEM_BOOT_GRAPHICS_INFORMATION {
    FrameBuffer: LARGE_INTEGER,
    Width: c_ulong,
    Height: c_ulong,
    PixelStride: c_ulong,
    Flags: c_ulong,
    Format: SYSTEM_PIXEL_FORMAT,
    DisplayRotation: c_ulong,
}}
pub type PSYSTEM_BOOT_GRAPHICS_INFORMATION =
    *mut SYSTEM_BOOT_GRAPHICS_INFORMATION;
STRUCT! {struct MEMORY_SCRUB_INFORMATION {
    Handle: HANDLE,
    PagesScrubbed: c_ulong,
}}
pub type PMEMORY_SCRUB_INFORMATION = *mut MEMORY_SCRUB_INFORMATION;
STRUCT! {struct PEBS_DS_SAVE_AREA {
    BtsBufferBase: __uint64,
    BtsIndex: __uint64,
    BtsAbsoluteMaximum: __uint64,
    BtsInterruptThreshold: __uint64,
    PebsBufferBase: __uint64,
    PebsIndex: __uint64,
    PebsAbsoluteMaximum: __uint64,
    PebsInterruptThreshold: __uint64,
    PebsCounterReset0: __uint64,
    PebsCounterReset1: __uint64,
    PebsCounterReset2: __uint64,
    PebsCounterReset3: __uint64,
}}
pub type PPEBS_DS_SAVE_AREA = *mut PEBS_DS_SAVE_AREA;
STRUCT! {struct PROCESSOR_PROFILE_CONTROL_AREA {
    PebsDsSaveArea: PEBS_DS_SAVE_AREA,
}}
pub type PPROCESSOR_PROFILE_CONTROL_AREA = *mut PROCESSOR_PROFILE_CONTROL_AREA;
STRUCT! {struct SYSTEM_PROCESSOR_PROFILE_CONTROL_AREA {
    ProcessorProfileControlArea: PROCESSOR_PROFILE_CONTROL_AREA,
    Allocate: c_uchar,
}}
pub type PSYSTEM_PROCESSOR_PROFILE_CONTROL_AREA =
    *mut SYSTEM_PROCESSOR_PROFILE_CONTROL_AREA;
STRUCT! {struct MEMORY_COMBINE_INFORMATION {
    Handle: HANDLE,
    PagesCombined: usize,
}}
pub type PMEMORY_COMBINE_INFORMATION = *mut MEMORY_COMBINE_INFORMATION;
pub const MEMORY_COMBINE_FLAGS_COMMON_PAGES_ONLY: c_ulong = 0x4;
STRUCT! {struct MEMORY_COMBINE_INFORMATION_EX {
    Handle: HANDLE,
    PagesCombined: usize,
    Flags: c_ulong,
}}
pub type PMEMORY_COMBINE_INFORMATION_EX = *mut MEMORY_COMBINE_INFORMATION_EX;
STRUCT! {struct MEMORY_COMBINE_INFORMATION_EX2 {
    Handle: HANDLE,
    PagesCombined: usize,
    Flags: c_ulong,
    ProcessHandle: HANDLE,
}}
pub type PMEMORY_COMBINE_INFORMATION_EX2 = *mut MEMORY_COMBINE_INFORMATION_EX2;
STRUCT! {struct SYSTEM_CONSOLE_INFORMATION {
    Bitfields: c_ulong,
}}
BITFIELD! {SYSTEM_CONSOLE_INFORMATION Bitfields: c_ulong [
    DriverLoaded set_DriverLoaded[0..1],
    Spare set_Spare[1..32],
]}
pub type PSYSTEM_CONSOLE_INFORMATION = *mut SYSTEM_CONSOLE_INFORMATION;
STRUCT! {struct SYSTEM_PLATFORM_BINARY_INFORMATION {
    PhysicalAddress: __uint64,
    HandoffBuffer: *mut c_void,
    CommandLineBuffer: *mut c_void,
    HandoffBufferSize: c_ulong,
    CommandLineBufferSize: c_ulong,
}}
pub type PSYSTEM_PLATFORM_BINARY_INFORMATION =
    *mut SYSTEM_PLATFORM_BINARY_INFORMATION;
STRUCT! {struct SYSTEM_HYPERVISOR_PROCESSOR_COUNT_INFORMATION {
    NumberOfLogicalProcessors: c_ulong,
    NumberOfCores: c_ulong,
}}
pub type PSYSTEM_HYPERVISOR_PROCESSOR_COUNT_INFORMATION =
    *mut SYSTEM_HYPERVISOR_PROCESSOR_COUNT_INFORMATION;
STRUCT! {struct SYSTEM_DEVICE_DATA_INFORMATION {
    DeviceId: UNICODE_STRING,
    DataName: UNICODE_STRING,
    DataType: c_ulong,
    DataBufferLength: c_ulong,
    DataBuffer: *mut c_void,
}}
pub type PSYSTEM_DEVICE_DATA_INFORMATION = *mut SYSTEM_DEVICE_DATA_INFORMATION;
STRUCT! {struct PHYSICAL_CHANNEL_RUN {
    NodeNumber: c_ulong,
    ChannelNumber: c_ulong,
    BasePage: __uint64,
    PageCount: __uint64,
    Flags: c_ulong,
}}
pub type PPHYSICAL_CHANNEL_RUN = *mut PHYSICAL_CHANNEL_RUN;
STRUCT! {struct SYSTEM_MEMORY_TOPOLOGY_INFORMATION {
    NumberOfRuns: __uint64,
    NumberOfNodes: c_ulong,
    NumberOfChannels: c_ulong,
    Run: [PHYSICAL_CHANNEL_RUN; 1],
}}
pub type PSYSTEM_MEMORY_TOPOLOGY_INFORMATION =
    *mut SYSTEM_MEMORY_TOPOLOGY_INFORMATION;
STRUCT! {struct SYSTEM_MEMORY_CHANNEL_INFORMATION {
    ChannelNumber: c_ulong,
    ChannelHeatIndex: c_ulong,
    TotalPageCount: __uint64,
    ZeroPageCount: __uint64,
    FreePageCount: __uint64,
    StandbyPageCount: __uint64,
}}
pub type PSYSTEM_MEMORY_CHANNEL_INFORMATION =
    *mut SYSTEM_MEMORY_CHANNEL_INFORMATION;
STRUCT! {struct SYSTEM_BOOT_LOGO_INFORMATION {
    Flags: c_ulong,
    BitmapOffset: c_ulong,
}}
pub type PSYSTEM_BOOT_LOGO_INFORMATION = *mut SYSTEM_BOOT_LOGO_INFORMATION;
STRUCT! {struct SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION_EX {
    IdleTime: LARGE_INTEGER,
    KernelTime: LARGE_INTEGER,
    UserTime: LARGE_INTEGER,
    DpcTime: LARGE_INTEGER,
    InterruptTime: LARGE_INTEGER,
    InterruptCount: c_ulong,
    Spare0: c_ulong,
    AvailableTime: LARGE_INTEGER,
    Spare1: LARGE_INTEGER,
    Spare2: LARGE_INTEGER,
}}
pub type PSYSTEM_PROCESSOR_PERFORMANCE_INFORMATION_EX =
    *mut SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION_EX;
STRUCT! {struct SYSTEM_SECUREBOOT_POLICY_INFORMATION {
    PolicyPublisher: GUID,
    PolicyVersion: c_ulong,
    PolicyOptions: c_ulong,
}}
pub type PSYSTEM_SECUREBOOT_POLICY_INFORMATION =
    *mut SYSTEM_SECUREBOOT_POLICY_INFORMATION;
STRUCT! {struct SYSTEM_PAGEFILE_INFORMATION_EX {
    Info: SYSTEM_PAGEFILE_INFORMATION,
    MinimumSize: c_ulong,
    MaximumSize: c_ulong,
}}
pub type PSYSTEM_PAGEFILE_INFORMATION_EX = *mut SYSTEM_PAGEFILE_INFORMATION_EX;
STRUCT! {struct SYSTEM_SECUREBOOT_INFORMATION {
    SecureBootEnabled: c_uchar,
    SecureBootCapable: c_uchar,
}}
pub type PSYSTEM_SECUREBOOT_INFORMATION = *mut SYSTEM_SECUREBOOT_INFORMATION;
STRUCT! {struct PROCESS_DISK_COUNTERS {
    BytesRead: __uint64,
    BytesWritten: __uint64,
    ReadOperationCount: __uint64,
    WriteOperationCount: __uint64,
    FlushOperationCount: __uint64,
}}
pub type PPROCESS_DISK_COUNTERS = *mut PROCESS_DISK_COUNTERS;
UNION! {union ENERGY_STATE_DURATION_u {
    Value: __uint64,
    LastChangeTime: c_ulong,
}}
UNION! {union ENERGY_STATE_DURATION {
    u: ENERGY_STATE_DURATION_u,
    BitFields: c_ulong,
}}
pub type PENERGY_STATE_DURATION = *mut ENERGY_STATE_DURATION;
BITFIELD! {unsafe ENERGY_STATE_DURATION BitFields: c_ulong [
    Duration set_Duration[0..31],
    IsInState set_IsInState[31..32],
]}
STRUCT! {struct PROCESS_ENERGY_VALUES {
    Cycles: [[__uint64; 4]; 2],
    DiskEnergy: __uint64,
    NetworkTailEnergy: __uint64,
    MBBTailEnergy: __uint64,
    NetworkTxRxBytes: __uint64,
    MBBTxRxBytes: __uint64,
    ForegroundDuration: ENERGY_STATE_DURATION,
    DesktopVisibleDuration: ENERGY_STATE_DURATION,
    PSMForegroundDuration: ENERGY_STATE_DURATION,
    CompositionRendered: c_ulong,
    CompositionDirtyGenerated: c_ulong,
    CompositionDirtyPropagated: c_ulong,
    Reserved1: c_ulong,
    AttributedCycles: [[__uint64; 2]; 4],
    WorkOnBehalfCycles: [[__uint64; 2]; 4],
}}
pub type PPROCESS_ENERGY_VALUES = *mut PROCESS_ENERGY_VALUES;
STRUCT! {struct TIMELINE_BITMAP {
    Value: __uint64,
    EndTime: c_ulong,
    Bitmap: c_ulong,
}}
pub type PTIMELINE_BITMAP = *mut TIMELINE_BITMAP;
STRUCT! {struct PROCESS_ENERGY_VALUES_EXTENSION_Timelines {
    CpuTimeline: TIMELINE_BITMAP,
    DiskTimeline: TIMELINE_BITMAP,
    NetworkTimeline: TIMELINE_BITMAP,
    MBBTimeline: TIMELINE_BITMAP,
    ForegroundTimeline: TIMELINE_BITMAP,
    DesktopVisibleTimeline: TIMELINE_BITMAP,
    CompositionRenderedTimeline: TIMELINE_BITMAP,
    CompositionDirtyGeneratedTimeline: TIMELINE_BITMAP,
    CompositionDirtyPropagatedTimeline: TIMELINE_BITMAP,
    InputTimeline: TIMELINE_BITMAP,
    AudioInTimeline: TIMELINE_BITMAP,
    AudioOutTimeline: TIMELINE_BITMAP,
    DisplayRequiredTimeline: TIMELINE_BITMAP,
    KeyboardInputTimeline: TIMELINE_BITMAP,
}}
STRUCT! {struct PROCESS_ENERGY_VALUES_EXTENSION_Durations {
    InputDuration: ENERGY_STATE_DURATION,
    AudioInDuration: ENERGY_STATE_DURATION,
    AudioOutDuration: ENERGY_STATE_DURATION,
    DisplayRequiredDuration: ENERGY_STATE_DURATION,
    PSMBackgroundDuration: ENERGY_STATE_DURATION,
}}
STRUCT! {struct PROCESS_ENERGY_VALUES_EXTENSION {
    Timelines: PROCESS_ENERGY_VALUES_EXTENSION_Timelines,
    Durations: PROCESS_ENERGY_VALUES_EXTENSION_Durations,
    KeyboardInput: c_ulong,
    MouseInput: c_ulong,
}}
pub type PPROCESS_ENERGY_VALUES_EXTENSION =
    *mut PROCESS_ENERGY_VALUES_EXTENSION;
STRUCT! {struct PROCESS_EXTENDED_ENERGY_VALUES {
    Base: PROCESS_ENERGY_VALUES,
    Extension: PROCESS_ENERGY_VALUES_EXTENSION,
}}
pub type PPROCESS_EXTENDED_ENERGY_VALUES = *mut PROCESS_EXTENDED_ENERGY_VALUES;
ENUM! {enum SYSTEM_PROCESS_CLASSIFICATION {
    SystemProcessClassificationNormal = 0,
    SystemProcessClassificationSystem = 1,
    SystemProcessClassificationSecureSystem = 2,
    SystemProcessClassificationMemCompression = 3,
    SystemProcessClassificationRegistry = 4,
    SystemProcessClassificationMaximum = 5,
}}
STRUCT! {struct SYSTEM_PROCESS_INFORMATION_EXTENSION {
    DiskCounters: PROCESS_DISK_COUNTERS,
    ContextSwitches: __uint64,
    Flags: c_ulong,
    UserSidOffset: c_ulong,
    PackageFullNameOffset: c_ulong,
    EnergyValues: PROCESS_ENERGY_VALUES,
    AppIdOffset: c_ulong,
    SharedCommitCharge: usize,
    JobObjectId: c_ulong,
    SpareUlong: c_ulong,
    ProcessSequenceNumber: __uint64,
}}
BITFIELD! {SYSTEM_PROCESS_INFORMATION_EXTENSION Flags: c_ulong [
    HasStrongId set_HasStrongId[0..1],
    Classification set_Classification[1..5],
    BackgroundActivityModerated set_BackgroundActivityModerated[5..6],
    Spare set_Spare[6..32],
]}
pub type PSYSTEM_PROCESS_INFORMATION_EXTENSION =
    *mut SYSTEM_PROCESS_INFORMATION_EXTENSION;
STRUCT! {struct SYSTEM_PORTABLE_WORKSPACE_EFI_LAUNCHER_INFORMATION {
    EfiLauncherEnabled: c_uchar,
}}
pub type PSYSTEM_PORTABLE_WORKSPACE_EFI_LAUNCHER_INFORMATION =
    *mut SYSTEM_PORTABLE_WORKSPACE_EFI_LAUNCHER_INFORMATION;
STRUCT! {struct SYSTEM_KERNEL_DEBUGGER_INFORMATION_EX {
    DebuggerAllowed: c_uchar,
    DebuggerEnabled: c_uchar,
    DebuggerPresent: c_uchar,
}}
pub type PSYSTEM_KERNEL_DEBUGGER_INFORMATION_EX =
    *mut SYSTEM_KERNEL_DEBUGGER_INFORMATION_EX;
STRUCT! {struct SYSTEM_ELAM_CERTIFICATE_INFORMATION {
    ElamDriverFile: HANDLE,
}}
pub type PSYSTEM_ELAM_CERTIFICATE_INFORMATION =
    *mut SYSTEM_ELAM_CERTIFICATE_INFORMATION;
STRUCT! {struct SYSTEM_PROCESSOR_FEATURES_INFORMATION {
    ProcessorFeatureBits: __uint64,
    Reserved: [__uint64; 3],
}}
pub type PSYSTEM_PROCESSOR_FEATURES_INFORMATION =
    *mut SYSTEM_PROCESSOR_FEATURES_INFORMATION;
STRUCT! {struct SYSTEM_MANUFACTURING_INFORMATION {
    Options: c_ulong,
    ProfileName: UNICODE_STRING,
}}
pub type PSYSTEM_MANUFACTURING_INFORMATION =
    *mut SYSTEM_MANUFACTURING_INFORMATION;
STRUCT! {struct SYSTEM_ENERGY_ESTIMATION_CONFIG_INFORMATION {
    Enabled: c_uchar,
}}
pub type PSYSTEM_ENERGY_ESTIMATION_CONFIG_INFORMATION =
    *mut SYSTEM_ENERGY_ESTIMATION_CONFIG_INFORMATION;
STRUCT! {struct HV_DETAILS {
    Data: [c_ulong; 4],
}}
pub type PHV_DETAILS = *mut HV_DETAILS;
STRUCT! {struct SYSTEM_HYPERVISOR_DETAIL_INFORMATION {
    HvVendorAndMaxFunction: HV_DETAILS,
    HypervisorInterface: HV_DETAILS,
    HypervisorVersion: HV_DETAILS,
    HvFeatures: HV_DETAILS,
    HwFeatures: HV_DETAILS,
    EnlightenmentInfo: HV_DETAILS,
    ImplementationLimits: HV_DETAILS,
}}
pub type PSYSTEM_HYPERVISOR_DETAIL_INFORMATION =
    *mut SYSTEM_HYPERVISOR_DETAIL_INFORMATION;
STRUCT! {struct SYSTEM_PROCESSOR_CYCLE_STATS_INFORMATION {
    Cycles: [[__uint64; 4]; 2],
}}
pub type PSYSTEM_PROCESSOR_CYCLE_STATS_INFORMATION =
    *mut SYSTEM_PROCESSOR_CYCLE_STATS_INFORMATION;
STRUCT! {struct SYSTEM_TPM_INFORMATION {
    Flags: c_ulong,
}}
pub type PSYSTEM_TPM_INFORMATION = *mut SYSTEM_TPM_INFORMATION;
STRUCT! {struct SYSTEM_VSM_PROTECTION_INFORMATION {
    DmaProtectionsAvailable: c_uchar,
    DmaProtectionsInUse: c_uchar,
    HardwareMbecAvailable: c_uchar,
}}
pub type PSYSTEM_VSM_PROTECTION_INFORMATION =
    *mut SYSTEM_VSM_PROTECTION_INFORMATION;
STRUCT! {struct SYSTEM_CODEINTEGRITYPOLICY_INFORMATION {
    Options: c_ulong,
    HVCIOptions: c_ulong,
    Version: __uint64,
    PolicyGuid: GUID,
}}
pub type PSYSTEM_CODEINTEGRITYPOLICY_INFORMATION =
    *mut SYSTEM_CODEINTEGRITYPOLICY_INFORMATION;
STRUCT! {struct SYSTEM_ISOLATED_USER_MODE_INFORMATION {
    Bitfields1: c_uchar,
    Bitfields2: c_uchar,
    Spare0: [c_uchar; 6],
    Spare1: __uint64,
}}
BITFIELD! {SYSTEM_ISOLATED_USER_MODE_INFORMATION Bitfields1: c_uchar [
    SecureKernelRunning set_SecureKernelRunning[0..1],
    HvciEnabled set_HvciEnabled[1..2],
    HvciStrictMode set_HvciStrictMode[2..3],
    DebugEnabled set_DebugEnabled[3..4],
    FirmwarePageProtection set_FirmwarePageProtection[4..5],
    EncryptionKeyAvailable set_EncryptionKeyAvailable[5..6],
    SpareFlags set_SpareFlags[6..7],
    TrustletRunning set_TrustletRunning[7..8],
]}
BITFIELD! {SYSTEM_ISOLATED_USER_MODE_INFORMATION Bitfields2: c_uchar [
    SpareFlags2 set_SpareFlags2[0..1],
]}
pub type PSYSTEM_ISOLATED_USER_MODE_INFORMATION =
    *mut SYSTEM_ISOLATED_USER_MODE_INFORMATION;
STRUCT! {struct SYSTEM_SINGLE_MODULE_INFORMATION {
    TargetModuleAddress: *mut c_void,
    ExInfo: RTL_PROCESS_MODULE_INFORMATION_EX,
}}
pub type PSYSTEM_SINGLE_MODULE_INFORMATION =
    *mut SYSTEM_SINGLE_MODULE_INFORMATION;
STRUCT! {struct SYSTEM_INTERRUPT_CPU_SET_INFORMATION {
    Gsiv: c_ulong,
    Group: c_ushort,
    CpuSets: __uint64,
}}
pub type PSYSTEM_INTERRUPT_CPU_SET_INFORMATION =
    *mut SYSTEM_INTERRUPT_CPU_SET_INFORMATION;
STRUCT! {struct SYSTEM_SECUREBOOT_POLICY_FULL_INFORMATION {
    PolicyInformation: SYSTEM_SECUREBOOT_POLICY_INFORMATION,
    PolicySize: c_ulong,
    Policy: [c_uchar; 1],
}}
pub type PSYSTEM_SECUREBOOT_POLICY_FULL_INFORMATION =
    *mut SYSTEM_SECUREBOOT_POLICY_FULL_INFORMATION;
STRUCT! {struct SYSTEM_ROOT_SILO_INFORMATION {
    NumberOfSilos: c_ulong,
    SiloIdList: [c_ulong; 1],
}}
pub type PSYSTEM_ROOT_SILO_INFORMATION = *mut SYSTEM_ROOT_SILO_INFORMATION;
STRUCT! {struct SYSTEM_CPU_SET_TAG_INFORMATION {
    Tag: __uint64,
    CpuSets: [__uint64; 1],
}}
pub type PSYSTEM_CPU_SET_TAG_INFORMATION = *mut SYSTEM_CPU_SET_TAG_INFORMATION;
STRUCT! {struct SYSTEM_SECURE_KERNEL_HYPERGUARD_PROFILE_INFORMATION {
    ExtentCount: c_ulong,
    ValidStructureSize: c_ulong,
    NextExtentIndex: c_ulong,
    ExtentRestart: c_ulong,
    CycleCount: c_ulong,
    TimeoutCount: c_ulong,
    CycleTime: __uint64,
    CycleTimeMax: __uint64,
    ExtentTime: __uint64,
    ExtentTimeIndex: c_ulong,
    ExtentTimeMaxIndex: c_ulong,
    ExtentTimeMax: __uint64,
    HyperFlushTimeMax: __uint64,
    TranslateVaTimeMax: __uint64,
    DebugExemptionCount: __uint64,
    TbHitCount: __uint64,
    TbMissCount: __uint64,
    VinaPendingYield: __uint64,
    HashCycles: __uint64,
    HistogramOffset: c_ulong,
    HistogramBuckets: c_ulong,
    HistogramShift: c_ulong,
    Reserved1: c_ulong,
    PageNotPresentCount: __uint64,
}}
pub type PSYSTEM_SECURE_KERNEL_HYPERGUARD_PROFILE_INFORMATION =
    *mut SYSTEM_SECURE_KERNEL_HYPERGUARD_PROFILE_INFORMATION;
STRUCT! {struct SYSTEM_SECUREBOOT_PLATFORM_MANIFEST_INFORMATION {
    PlatformManifestSize: c_ulong,
    PlatformManifest: [c_uchar; 1],
}}
pub type PSYSTEM_SECUREBOOT_PLATFORM_MANIFEST_INFORMATION =
    *mut SYSTEM_SECUREBOOT_PLATFORM_MANIFEST_INFORMATION;
STRUCT! {struct SYSTEM_MEMORY_USAGE_INFORMATION {
    TotalPhysicalBytes: __uint64,
    AvailableBytes: __uint64,
    ResidentAvailableBytes: __int64,
    CommittedBytes: __uint64,
    SharedCommittedBytes: __uint64,
    CommitLimitBytes: __uint64,
    PeakCommitmentBytes: __uint64,
}}
pub type PSYSTEM_MEMORY_USAGE_INFORMATION =
    *mut SYSTEM_MEMORY_USAGE_INFORMATION;
STRUCT! {struct SYSTEM_CODEINTEGRITY_CERTIFICATE_INFORMATION {
    ImageFile: HANDLE,
    Type: c_ulong,
}}
pub type PSYSTEM_CODEINTEGRITY_CERTIFICATE_INFORMATION =
    *mut SYSTEM_CODEINTEGRITY_CERTIFICATE_INFORMATION;
STRUCT! {struct SYSTEM_PHYSICAL_MEMORY_INFORMATION {
    TotalPhysicalBytes: __uint64,
    LowestPhysicalAddress: __uint64,
    HighestPhysicalAddress: __uint64,
}}
pub type PSYSTEM_PHYSICAL_MEMORY_INFORMATION =
    *mut SYSTEM_PHYSICAL_MEMORY_INFORMATION;
ENUM! {enum SYSTEM_ACTIVITY_MODERATION_STATE {
    SystemActivityModerationStateSystemManaged = 0,
    SystemActivityModerationStateUserManagedAllowThrottling = 1,
    SystemActivityModerationStateUserManagedDisableThrottling = 2,
    MaxSystemActivityModerationState = 3,
}}
ENUM! {enum SYSTEM_ACTIVITY_MODERATION_APP_TYPE {
    SystemActivityModerationAppTypeClassic = 0,
    SystemActivityModerationAppTypePackaged = 1,
    MaxSystemActivityModerationAppType = 2,
}}
STRUCT! {struct SYSTEM_ACTIVITY_MODERATION_INFO {
    Identifier: UNICODE_STRING,
    ModerationState: SYSTEM_ACTIVITY_MODERATION_STATE,
    AppType: SYSTEM_ACTIVITY_MODERATION_APP_TYPE,
}}
pub type PSYSTEM_ACTIVITY_MODERATION_INFO =
    *mut SYSTEM_ACTIVITY_MODERATION_INFO;
STRUCT! {struct SYSTEM_ACTIVITY_MODERATION_USER_SETTINGS {
    UserKeyHandle: HANDLE,
}}
pub type PSYSTEM_ACTIVITY_MODERATION_USER_SETTINGS =
    *mut SYSTEM_ACTIVITY_MODERATION_USER_SETTINGS;
STRUCT! {struct SYSTEM_CODEINTEGRITY_UNLOCK_INFORMATION {
    Flags: c_ulong,
    UnlockId: [c_uchar; 32],
}}
BITFIELD! {SYSTEM_CODEINTEGRITY_UNLOCK_INFORMATION Flags: c_ulong [
    Locked set_Locked[0..1],
    Unlockable set_Unlockable[1..2],
    UnlockApplied set_UnlockApplied[2..3],
    UnlockIdValid set_UnlockIdValid[3..4],
    Reserved set_Reserved[4..32],
]}
pub type PSYSTEM_CODEINTEGRITY_UNLOCK_INFORMATION =
    *mut SYSTEM_CODEINTEGRITY_UNLOCK_INFORMATION;
STRUCT! {struct SYSTEM_FLUSH_INFORMATION {
    SupportedFlushMethods: c_ulong,
    ProcessorCacheFlushSize: c_ulong,
    SystemFlushCapabilities: __uint64,
    Reserved: [__uint64; 2],
}}
pub type PSYSTEM_FLUSH_INFORMATION = *mut SYSTEM_FLUSH_INFORMATION;
STRUCT! {struct SYSTEM_WRITE_CONSTRAINT_INFORMATION {
    WriteConstraintPolicy: c_ulong,
    Reserved: c_ulong,
}}
pub type PSYSTEM_WRITE_CONSTRAINT_INFORMATION =
    *mut SYSTEM_WRITE_CONSTRAINT_INFORMATION;
STRUCT! {struct SYSTEM_KERNEL_VA_SHADOW_INFORMATION {
    Flags: c_ulong,
}}
BITFIELD! {SYSTEM_KERNEL_VA_SHADOW_INFORMATION Flags: c_ulong [
    KvaShadowEnabled set_KvaShadowEnabled[0..1],
    KvaShadowUserGlobal set_KvaShadowUserGlobal[1..2],
    KvaShadowPcid set_KvaShadowPcid[2..3],
    KvaShadowInvpcid set_KvaShadowInvpcid[3..4],
    KvaShadowRequired set_KvaShadowRequired[4..5],
    KvaShadowRequiredAvailable set_KvaShadowRequiredAvailable[5..6],
    InvalidPteBit set_InvalidPteBit[6..12],
    L1DataCacheFlushSupported set_L1DataCacheFlushSupported[12..13],
    L1TerminalFaultMitigationPresent set_L1TerminalFaultMitigationPresent[13..14],
    Reserved set_Reserved[14..32],
]}
pub type PSYSTEM_KERNEL_VA_SHADOW_INFORMATION =
    *mut SYSTEM_KERNEL_VA_SHADOW_INFORMATION;
STRUCT! {struct SYSTEM_CODEINTEGRITYVERIFICATION_INFORMATION {
    FileHandle: HANDLE,
    ImageSize: c_ulong,
    Image: *mut c_void,
}}
pub type PSYSTEM_CODEINTEGRITYVERIFICATION_INFORMATION =
    *mut SYSTEM_CODEINTEGRITYVERIFICATION_INFORMATION;
STRUCT! {struct SYSTEM_HYPERVISOR_SHARED_PAGE_INFORMATION {
    HypervisorSharedUserVa: *mut c_void,
}}
pub type PSYSTEM_HYPERVISOR_SHARED_PAGE_INFORMATION =
    *mut SYSTEM_HYPERVISOR_SHARED_PAGE_INFORMATION;
STRUCT! {struct SYSTEM_SPECULATION_CONTROL_INFORMATION {
    Flags: c_ulong,
}}
BITFIELD! {SYSTEM_SPECULATION_CONTROL_INFORMATION Flags: c_ulong [
    BpbEnabled set_BpbEnabled[0..1],
    BpbDisabledSystemPolicy set_BpbDisabledSystemPolicy[1..2],
    BpbDisabledNoHardwareSupport set_BpbDisabledNoHardwareSupport[2..3],
    SpecCtrlEnumerated set_SpecCtrlEnumerated[3..4],
    SpecCmdEnumerated set_SpecCmdEnumerated[4..5],
    IbrsPresent set_IbrsPresent[5..6],
    StibpPresent set_StibpPresent[6..7],
    SmepPresent set_SmepPresent[7..8],
    SpeculativeStoreBypassDisableAvailable set_SpeculativeStoreBypassDisableAvailable[8..9],
    SpeculativeStoreBypassDisableSupported set_SpeculativeStoreBypassDisableSupported[9..10],
    SpeculativeStoreBypassDisabledSystemWide set_SpeculativeStoreBypassDisabledSystemWide[10..11],
    SpeculativeStoreBypassDisabledKernel set_SpeculativeStoreBypassDisabledKernel[11..12],
    SpeculativeStoreBypassDisableRequired set_SpeculativeStoreBypassDisableRequired[12..13],
    BpbDisabledKernelToUser set_BpbDisabledKernelToUser[13..14],
    SpecCtrlRetpolineEnabled set_SpecCtrlRetpolineEnabled[14..15],
    SpecCtrlImportOptimizationEnabled set_SpecCtrlImportOptimizationEnabled[15..16],
    Reserved set_Reserved[16..32],
]}
pub type PSYSTEM_SPECULATION_CONTROL_INFORMATION =
    *mut SYSTEM_SPECULATION_CONTROL_INFORMATION;
STRUCT! {struct SYSTEM_DMA_GUARD_POLICY_INFORMATION {
    DmaGuardPolicyEnabled: c_uchar,
}}
pub type PSYSTEM_DMA_GUARD_POLICY_INFORMATION =
    *mut SYSTEM_DMA_GUARD_POLICY_INFORMATION;
STRUCT! {struct SYSTEM_ENCLAVE_LAUNCH_CONTROL_INFORMATION {
    EnclaveLaunchSigner: [c_uchar; 32],
}}
pub type PSYSTEM_ENCLAVE_LAUNCH_CONTROL_INFORMATION =
    *mut SYSTEM_ENCLAVE_LAUNCH_CONTROL_INFORMATION;
STRUCT! {struct SYSTEM_WORKLOAD_ALLOWED_CPU_SET_INFORMATION {
    WorkloadClass: __uint64,
    CpuSets: [__uint64; 1],
}}
pub type PSYSTEM_WORKLOAD_ALLOWED_CPU_SET_INFORMATION =
    *mut SYSTEM_WORKLOAD_ALLOWED_CPU_SET_INFORMATION;
EXTERN! {extern "system" {
    fn NtQuerySystemInformation(
        SystemInformationClass: SYSTEM_INFORMATION_CLASS,
        SystemInformation: *mut c_void,
        SystemInformationLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtQuerySystemInformationEx(
        SystemInformationClass: SYSTEM_INFORMATION_CLASS,
        InputBuffer: *mut c_void,
        InputBufferLength: c_ulong,
        SystemInformation: *mut c_void,
        SystemInformationLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtSetSystemInformation(
        SystemInformationClass: SYSTEM_INFORMATION_CLASS,
        SystemInformation: *mut c_void,
        SystemInformationLength: c_ulong,
    ) -> NTSTATUS;
}}
ENUM! {enum SYSDBG_COMMAND {
    SysDbgQueryModuleInformation = 0,
    SysDbgQueryTraceInformation = 1,
    SysDbgSetTracepoint = 2,
    SysDbgSetSpecialCall = 3,
    SysDbgClearSpecialCalls = 4,
    SysDbgQuerySpecialCalls = 5,
    SysDbgBreakPoint = 6,
    SysDbgQueryVersion = 7,
    SysDbgReadVirtual = 8,
    SysDbgWriteVirtual = 9,
    SysDbgReadPhysical = 10,
    SysDbgWritePhysical = 11,
    SysDbgReadControlSpace = 12,
    SysDbgWriteControlSpace = 13,
    SysDbgReadIoSpace = 14,
    SysDbgWriteIoSpace = 15,
    SysDbgReadMsr = 16,
    SysDbgWriteMsr = 17,
    SysDbgReadBusData = 18,
    SysDbgWriteBusData = 19,
    SysDbgCheckLowMemory = 20,
    SysDbgEnableKernelDebugger = 21,
    SysDbgDisableKernelDebugger = 22,
    SysDbgGetAutoKdEnable = 23,
    SysDbgSetAutoKdEnable = 24,
    SysDbgGetPrintBufferSize = 25,
    SysDbgSetPrintBufferSize = 26,
    SysDbgGetKdUmExceptionEnable = 27,
    SysDbgSetKdUmExceptionEnable = 28,
    SysDbgGetTriageDump = 29,
    SysDbgGetKdBlockEnable = 30,
    SysDbgSetKdBlockEnable = 31,
    SysDbgRegisterForUmBreakInfo = 32,
    SysDbgGetUmBreakPid = 33,
    SysDbgClearUmBreakPid = 34,
    SysDbgGetUmAttachPid = 35,
    SysDbgClearUmAttachPid = 36,
    SysDbgGetLiveKernelDump = 37,
}}
pub type PSYSDBG_COMMAND = *mut SYSDBG_COMMAND;
STRUCT! {struct SYSDBG_VIRTUAL {
    Address: *mut c_void,
    Buffer: *mut c_void,
    Request: c_ulong,
}}
pub type PSYSDBG_VIRTUAL = *mut SYSDBG_VIRTUAL;
STRUCT! {struct SYSDBG_PHYSICAL {
    Address: LARGE_INTEGER,
    Buffer: *mut c_void,
    Request: c_ulong,
}}
pub type PSYSDBG_PHYSICAL = *mut SYSDBG_PHYSICAL;
STRUCT! {struct SYSDBG_CONTROL_SPACE {
    Address: __uint64,
    Buffer: *mut c_void,
    Request: c_ulong,
    Processor: c_ulong,
}}
pub type PSYSDBG_CONTROL_SPACE = *mut SYSDBG_CONTROL_SPACE;
STRUCT! {struct SYSDBG_IO_SPACE {
    Address: __uint64,
    Buffer: *mut c_void,
    Request: c_ulong,
    InterfaceType: INTERFACE_TYPE,
    BusNumber: c_ulong,
    AddressSpace: c_ulong,
}}
pub type PSYSDBG_IO_SPACE = *mut SYSDBG_IO_SPACE;
STRUCT! {struct SYSDBG_MSR {
    Msr: c_ulong,
    Data: __uint64,
}}
pub type PSYSDBG_MSR = *mut SYSDBG_MSR;
STRUCT! {struct SYSDBG_BUS_DATA {
    Address: c_ulong,
    Buffer: *mut c_void,
    Request: c_ulong,
    BusDataType: BUS_DATA_TYPE,
    BusNumber: c_ulong,
    SlotNumber: c_ulong,
}}
pub type PSYSDBG_BUS_DATA = *mut SYSDBG_BUS_DATA;
STRUCT! {struct SYSDBG_TRIAGE_DUMP {
    Flags: c_ulong,
    BugCheckCode: c_ulong,
    BugCheckParam1: usize,
    BugCheckParam2: usize,
    BugCheckParam3: usize,
    BugCheckParam4: usize,
    ProcessHandles: c_ulong,
    ThreadHandles: c_ulong,
    Handles: *mut HANDLE,
}}
pub type PSYSDBG_TRIAGE_DUMP = *mut SYSDBG_TRIAGE_DUMP;
STRUCT! {struct SYSDBG_LIVEDUMP_CONTROL_FLAGS {
    AsUlong: c_ulong,
}}
BITFIELD! {SYSDBG_LIVEDUMP_CONTROL_FLAGS AsUlong: c_ulong [
    UseDumpStorageStack set_UseDumpStorageStack[0..1],
    CompressMemoryPagesData set_CompressMemoryPagesData[1..2],
    IncludeUserSpaceMemoryPages set_IncludeUserSpaceMemoryPages[2..3],
    AbortIfMemoryPressure set_AbortIfMemoryPressure[3..4],
    Reserved set_Reserved[4..32],
]}
pub type PSYSDBG_LIVEDUMP_CONTROL_FLAGS = *mut SYSDBG_LIVEDUMP_CONTROL_FLAGS;
STRUCT! {struct SYSDBG_LIVEDUMP_CONTROL_ADDPAGES {
    AsUlong: c_ulong,
}}
BITFIELD! {SYSDBG_LIVEDUMP_CONTROL_ADDPAGES AsUlong: c_ulong [
    HypervisorPages set_HypervisorPages[0..1],
    Reserved set_Reserved[1..32],
]}
pub type PSYSDBG_LIVEDUMP_CONTROL_ADDPAGES =
    *mut SYSDBG_LIVEDUMP_CONTROL_ADDPAGES;
pub const SYSDBG_LIVEDUMP_CONTROL_VERSION: c_ulong = 1;
STRUCT! {struct SYSDBG_LIVEDUMP_CONTROL {
    Version: c_ulong,
    BugCheckCode: c_ulong,
    BugCheckParam1: usize,
    BugCheckParam2: usize,
    BugCheckParam3: usize,
    BugCheckParam4: usize,
    DumpFileHandle: HANDLE,
    CancelEventHandle: HANDLE,
    Flags: SYSDBG_LIVEDUMP_CONTROL_FLAGS,
    AddPagesControl: SYSDBG_LIVEDUMP_CONTROL_ADDPAGES,
}}
pub type PSYSDBG_LIVEDUMP_CONTROL = *mut SYSDBG_LIVEDUMP_CONTROL;
EXTERN! {extern "system" {
    fn NtSystemDebugControl(
        Command: SYSDBG_COMMAND,
        InputBuffer: *mut c_void,
        InputBufferLength: c_ulong,
        OutputBuffer: *mut c_void,
        OutputBufferLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
}}
ENUM! {enum HARDERROR_RESPONSE_OPTION {
    OptionAbortRetryIgnore = 0,
    OptionOk = 1,
    OptionOkCancel = 2,
    OptionRetryCancel = 3,
    OptionYesNo = 4,
    OptionYesNoCancel = 5,
    OptionShutdownSystem = 6,
    OptionOkNoWait = 7,
    OptionCancelTryContinue = 8,
}}
ENUM! {enum HARDERROR_RESPONSE {
    ResponseReturnToCaller = 0,
    ResponseNotHandled = 1,
    ResponseAbort = 2,
    ResponseCancel = 3,
    ResponseIgnore = 4,
    ResponseNo = 5,
    ResponseOk = 6,
    ResponseRetry = 7,
    ResponseYes = 8,
    ResponseTryAgain = 9,
    ResponseContinue = 10,
}}
pub const HARDERROR_OVERRIDE_ERRORMODE: c_ulong = 0x10000000;
EXTERN! {extern "system" {
    fn NtRaiseHardError(
        ErrorStatus: NTSTATUS,
        NumberOfParameters: c_ulong,
        UnicodeStringParameterMask: c_ulong,
        Parameters: *mut usize,
        ValidResponseOptions: c_ulong,
        Response: *mut c_ulong,
    ) -> NTSTATUS;
}}
ENUM! {enum ALTERNATIVE_ARCHITECTURE_TYPE {
    StandardDesign = 0,
    NEC98x86 = 1,
    EndAlternatives = 2,
}}
pub const PROCESSOR_FEATURE_MAX: usize = 64;
pub const MAX_WOW64_SHARED_ENTRIES: u32 = 16;
pub const NX_SUPPORT_POLICY_ALWAYSOFF: u32 = 0;
pub const NX_SUPPORT_POLICY_ALWAYSON: u32 = 1;
pub const NX_SUPPORT_POLICY_OPTIN: u32 = 2;
pub const NX_SUPPORT_POLICY_OPTOUT: u32 = 3;
UNION! {union KUSER_SHARED_DATA_u {
    TickCount: KSYSTEM_TIME,
    TickCountQuad: __uint64,
    ReservedTickCountOverlay: [c_ulong; 3],
}}
STRUCT! {#[repr(packed(4))] struct KUSER_SHARED_DATA {
    TickCountLowDeprecated: c_ulong,
    TickCountMultiplier: c_ulong,
    InterruptTime: KSYSTEM_TIME,
    SystemTime: KSYSTEM_TIME,
    TimeZoneBias: KSYSTEM_TIME,
    ImageNumberLow: c_ushort,
    ImageNumberHigh: c_ushort,
    NtSystemRoot: [wchar_t; 260],
    MaxStackTraceDepth: c_ulong,
    CryptoExponent: c_ulong,
    TimeZoneId: c_ulong,
    LargePageMinimum: c_ulong,
    AitSamplingValue: c_ulong,
    AppCompatFlag: c_ulong,
    RNGSeedVersion: __uint64,
    GlobalValidationRunlevel: c_ulong,
    TimeZoneBiasStamp: c_long,
    NtBuildNumber: c_ulong,
    NtProductType: NT_PRODUCT_TYPE,
    ProductTypeIsValid: c_uchar,
    Reserved0: [c_uchar; 1],
    NativeProcessorArchitecture: c_ushort,
    NtMajorVersion: c_ulong,
    NtMinorVersion: c_ulong,
    ProcessorFeatures: [c_uchar; PROCESSOR_FEATURE_MAX],
    Reserved1: c_ulong,
    Reserved3: c_ulong,
    TimeSlip: c_ulong,
    AlternativeArchitecture: ALTERNATIVE_ARCHITECTURE_TYPE,
    BootId: c_ulong,
    SystemExpirationDate: LARGE_INTEGER,
    SuiteMask: c_ulong,
    KdDebuggerEnabled: c_uchar,
    MitigationPolicies: c_uchar,
    Reserved6: [c_uchar; 2],
    ActiveConsoleId: c_ulong,
    DismountCount: c_ulong,
    ComPlusPackage: c_ulong,
    LastSystemRITEventTickCount: c_ulong,
    NumberOfPhysicalPages: c_ulong,
    SafeBootMode: c_uchar,
    VirtualizationFlags: c_uchar,
    Reserved12: [c_uchar; 2],
    SharedDataFlags: c_ulong,
    DataFlagsPad: [c_ulong; 1],
    TestRetInstruction: __uint64,
    QpcFrequency: __int64,
    SystemCall: c_ulong,
    SystemCallPad0: c_ulong,
    SystemCallPad: [__uint64; 2],
    u: KUSER_SHARED_DATA_u,
    //TickCountPad: [c_ulong; 1],
    Cookie: c_ulong,
    CookiePad: [c_ulong; 1],
    ConsoleSessionForegroundProcessId: __int64,
    TimeUpdateLock: __uint64,
    BaselineSystemTimeQpc: __uint64,
    BaselineInterruptTimeQpc: __uint64,
    QpcSystemTimeIncrement: __uint64,
    QpcInterruptTimeIncrement: __uint64,
    QpcSystemTimeIncrementShift: c_uchar,
    QpcInterruptTimeIncrementShift: c_uchar,
    UnparkedProcessorCount: c_ushort,
    EnclaveFeatureMask: [c_ulong; 4],
    TelemetryCoverageRound: c_ulong,
    UserModeGlobalLogger: [c_ushort; 16],
    ImageFileExecutionOptions: c_ulong,
    LangGenerationCount: c_ulong,
    Reserved4: __uint64,
    InterruptTimeBias: __uint64,
    QpcBias: __uint64,
    ActiveProcessorCount: c_ulong,
    ActiveGroupCount: c_uchar,
    Reserved9: c_uchar,
    QpcData: c_uchar,
    TimeZoneBiasEffectiveStart: LARGE_INTEGER,
    TimeZoneBiasEffectiveEnd: LARGE_INTEGER,
    XState: XSTATE_CONFIGURATION,
}}
BITFIELD! {KUSER_SHARED_DATA MitigationPolicies: c_uchar [
    NXSupportPolicy set_NXSupportPolicy[0..2],
    SEHValidationPolicy set_SEHValidationPolicy[2..4],
    CurDirDevicesSkippedForDlls set_CurDirDevicesSkippedForDlls[4..6],
    Reserved set_Reserved[6..8],
]}
BITFIELD! {KUSER_SHARED_DATA SharedDataFlags: c_ulong [
    DbgErrorPortPresent set_DbgErrorPortPresent[0..1],
    DbgElevationEnabled set_DbgElevationEnabled[1..2],
    DbgVirtEnabled set_DbgVirtEnabled[2..3],
    DbgInstallerDetectEnabled set_DbgInstallerDetectEnabled[3..4],
    DbgLkgEnabled set_DbgLkgEnabled[4..5],
    DbgDynProcessorEnabled set_DbgDynProcessorEnabled[5..6],
    DbgConsoleBrokerEnabled set_DbgConsoleBrokerEnabled[6..7],
    DbgSecureBootEnabled set_DbgSecureBootEnabled[7..8],
    DbgMultiSessionSku set_DbgMultiSessionSku[8..9],
    DbgMultiUsersInSessionSku set_DbgMultiUsersInSessionSku[9..10],
    DbgStateSeparationEnabled set_DbgStateSeparationEnabled[10..11],
    SpareBits set_SpareBits[11..32],
]}
BITFIELD! {KUSER_SHARED_DATA QpcData: c_uchar [
    QpcBypassEnabled set_QpcBypassEnabled[0..1],
    QpcShift set_QpcShift[1..2],
]}
pub type PKUSER_SHARED_DATA = *mut KUSER_SHARED_DATA;
pub const USER_SHARED_DATA: *const KUSER_SHARED_DATA = 0x7ffe0000 as *const _;
#[inline]
pub unsafe fn NtGetTickCount64() -> __uint64 {
    let mut tick_count: ULARGE_INTEGER = MaybeUninit::zeroed().assume_init();
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
    {
        *tick_count.QuadPart_mut() =
            read_volatile(addr_of!((*USER_SHARED_DATA).u.TickCountQuad));
    }
    #[cfg(target_arch = "x86")]
    {
        loop {
            tick_count.s_mut().HighPart =
                read_volatile(&(*USER_SHARED_DATA).u.TickCount.High1Time)
                    as u32;
            tick_count.s_mut().LowPart =
                read_volatile(&(*USER_SHARED_DATA).u.TickCount.LowPart);
            if tick_count.s().HighPart
                == read_volatile(&(*USER_SHARED_DATA).u.TickCount.High2Time)
                    as u32
            {
                break;
            }
            spin_loop();
        }
    }
    (UInt32x32To64(
        tick_count.s().LowPart,
        (*USER_SHARED_DATA).TickCountMultiplier,
    ) >> 24)
        + (UInt32x32To64(
            tick_count.s().HighPart,
            (*USER_SHARED_DATA).TickCountMultiplier,
        ) << 8)
}
#[inline]
pub unsafe fn NtGetTickCount() -> c_ulong {
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
    {
        ((read_volatile(addr_of!((*USER_SHARED_DATA).u.TickCountQuad))
            * (*USER_SHARED_DATA).TickCountMultiplier as u64)
            >> 24) as u32
    }
    #[cfg(target_arch = "x86")]
    {
        let mut tick_count: ULARGE_INTEGER =
            MaybeUninit::zeroed().assume_init();
        loop {
            tick_count.s_mut().HighPart =
                read_volatile(&(*USER_SHARED_DATA).u.TickCount.High1Time)
                    as u32;
            tick_count.s_mut().LowPart =
                read_volatile(&(*USER_SHARED_DATA).u.TickCount.LowPart);
            if tick_count.s().HighPart
                == read_volatile(&(*USER_SHARED_DATA).u.TickCount.High2Time)
                    as u32
            {
                break;
            }
            spin_loop();
        }
        ((UInt32x32To64(
            tick_count.s().LowPart,
            (*USER_SHARED_DATA).TickCountMultiplier,
        ) >> 24)
            + UInt32x32To64(
                (tick_count.s().HighPart as u32) << 8,
                (*USER_SHARED_DATA).TickCountMultiplier,
            )) as u32
    }
}
EXTERN! {extern "system" {
    fn NtQueryDefaultLocale(
        UserProfile: c_uchar,
        DefaultLocaleId: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtSetDefaultLocale(
        UserProfile: c_uchar,
        DefaultLocaleId: c_ulong,
    ) -> NTSTATUS;
    fn NtQueryInstallUILanguage(
        InstallUILanguageId: *mut c_ushort,
    ) -> NTSTATUS;
    fn NtFlushInstallUILanguage(
        InstallUILanguage: c_ushort,
        SetComittedFlag: c_ulong,
    ) -> NTSTATUS;
    fn NtQueryDefaultUILanguage(
        DefaultUILanguageId: *mut c_ushort,
    ) -> NTSTATUS;
    fn NtSetDefaultUILanguage(
        DefaultUILanguageId: c_ushort,
    ) -> NTSTATUS;
    fn NtIsUILanguageComitted() -> NTSTATUS;
    fn NtInitializeNlsFiles(
        BaseAddress: *mut *mut c_void,
        DefaultLocaleId: *mut c_ulong,
        DefaultCasingTableSize: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtGetNlsSectionPtr(
        SectionType: c_ulong,
        SectionData: c_ulong,
        ContextData: *mut c_void,
        SectionPointer: *mut *mut c_void,
        SectionSize: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtMapCMFModule(
        What: c_ulong,
        Index: c_ulong,
        CacheIndexOut: *mut c_ulong,
        CacheFlagsOut: *mut c_ulong,
        ViewSizeOut: *mut c_ulong,
        BaseAddress: *mut *mut c_void,
    ) -> NTSTATUS;
    fn NtGetMUIRegistryInfo(
        Flags: c_ulong,
        DataSize: *mut c_ulong,
        Data: *mut c_void,
    ) -> NTSTATUS;
    fn NtAddAtom(
        AtomName: *mut wchar_t,
        Length: c_ulong,
        Atom: PRTL_ATOM,
    ) -> NTSTATUS;
}}
pub const ATOM_FLAG_GLOBAL: c_ulong = 0x2;
EXTERN! {extern "system" {
    fn NtAddAtomEx(
        AtomName: *mut wchar_t,
        Length: c_ulong,
        Atom: PRTL_ATOM,
        Flags: c_ulong,
    ) -> NTSTATUS;
    fn NtFindAtom(
        AtomName: *mut wchar_t,
        Length: c_ulong,
        Atom: PRTL_ATOM,
    ) -> NTSTATUS;
    fn NtDeleteAtom(
        Atom: RTL_ATOM,
    ) -> NTSTATUS;
}}
ENUM! {enum ATOM_INFORMATION_CLASS {
    AtomBasicInformation = 0,
    AtomTableInformation = 1,
}}
STRUCT! {struct ATOM_BASIC_INFORMATION {
    UsageCount: c_ushort,
    Flags: c_ushort,
    NameLength: c_ushort,
    Name: [wchar_t; 1],
}}
pub type PATOM_BASIC_INFORMATION = *mut ATOM_BASIC_INFORMATION;
STRUCT! {struct ATOM_TABLE_INFORMATION {
    NumberOfAtoms: c_ulong,
    Atoms: [RTL_ATOM; 1],
}}
pub type PATOM_TABLE_INFORMATION = *mut ATOM_TABLE_INFORMATION;
EXTERN! {extern "system" {
    fn NtQueryInformationAtom(
        Atom: RTL_ATOM,
        AtomInformationClass: ATOM_INFORMATION_CLASS,
        AtomInformation: *mut c_void,
        AtomInformationLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
}}
pub const FLG_STOP_ON_EXCEPTION: u32 = 0x00000001;
pub const FLG_SHOW_LDR_SNAPS: u32 = 0x00000002;
pub const FLG_DEBUG_INITIAL_COMMAND: u32 = 0x00000004;
pub const FLG_STOP_ON_HUNG_GUI: u32 = 0x00000008;
pub const FLG_HEAP_ENABLE_TAIL_CHECK: u32 = 0x00000010;
pub const FLG_HEAP_ENABLE_FREE_CHECK: u32 = 0x00000020;
pub const FLG_HEAP_VALIDATE_PARAMETERS: u32 = 0x00000040;
pub const FLG_HEAP_VALIDATE_ALL: u32 = 0x00000080;
pub const FLG_APPLICATION_VERIFIER: u32 = 0x00000100;
pub const FLG_POOL_ENABLE_TAGGING: u32 = 0x00000400;
pub const FLG_HEAP_ENABLE_TAGGING: u32 = 0x00000800;
pub const FLG_USER_STACK_TRACE_DB: u32 = 0x00001000;
pub const FLG_KERNEL_STACK_TRACE_DB: u32 = 0x00002000;
pub const FLG_MAINTAIN_OBJECT_TYPELIST: u32 = 0x00004000;
pub const FLG_HEAP_ENABLE_TAG_BY_DLL: u32 = 0x00008000;
pub const FLG_DISABLE_STACK_EXTENSION: u32 = 0x00010000;
pub const FLG_ENABLE_CSRDEBUG: u32 = 0x00020000;
pub const FLG_ENABLE_KDEBUG_SYMBOL_LOAD: u32 = 0x00040000;
pub const FLG_DISABLE_PAGE_KERNEL_STACKS: u32 = 0x00080000;
pub const FLG_ENABLE_SYSTEM_CRIT_BREAKS: u32 = 0x00100000;
pub const FLG_HEAP_DISABLE_COALESCING: u32 = 0x00200000;
pub const FLG_ENABLE_CLOSE_EXCEPTIONS: u32 = 0x00400000;
pub const FLG_ENABLE_EXCEPTION_LOGGING: u32 = 0x00800000;
pub const FLG_ENABLE_HANDLE_TYPE_TAGGING: u32 = 0x01000000;
pub const FLG_HEAP_PAGE_ALLOCS: u32 = 0x02000000;
pub const FLG_DEBUG_INITIAL_COMMAND_EX: u32 = 0x04000000;
pub const FLG_DISABLE_DBGPRINT: u32 = 0x08000000;
pub const FLG_CRITSEC_EVENT_CREATION: u32 = 0x10000000;
pub const FLG_LDR_TOP_DOWN: u32 = 0x20000000;
pub const FLG_ENABLE_HANDLE_EXCEPTIONS: u32 = 0x40000000;
pub const FLG_DISABLE_PROTDLLS: u32 = 0x80000000;
pub const FLG_VALID_BITS: u32 = 0xfffffdff;
pub const FLG_USERMODE_VALID_BITS: u32 = FLG_STOP_ON_EXCEPTION
    | FLG_SHOW_LDR_SNAPS
    | FLG_HEAP_ENABLE_TAIL_CHECK
    | FLG_HEAP_ENABLE_FREE_CHECK
    | FLG_HEAP_VALIDATE_PARAMETERS
    | FLG_HEAP_VALIDATE_ALL
    | FLG_APPLICATION_VERIFIER
    | FLG_HEAP_ENABLE_TAGGING
    | FLG_USER_STACK_TRACE_DB
    | FLG_HEAP_ENABLE_TAG_BY_DLL
    | FLG_DISABLE_STACK_EXTENSION
    | FLG_ENABLE_SYSTEM_CRIT_BREAKS
    | FLG_HEAP_DISABLE_COALESCING
    | FLG_DISABLE_PROTDLLS
    | FLG_HEAP_PAGE_ALLOCS
    | FLG_CRITSEC_EVENT_CREATION
    | FLG_LDR_TOP_DOWN;
pub const FLG_BOOTONLY_VALID_BITS: u32 = FLG_KERNEL_STACK_TRACE_DB
    | FLG_MAINTAIN_OBJECT_TYPELIST
    | FLG_ENABLE_CSRDEBUG
    | FLG_DEBUG_INITIAL_COMMAND
    | FLG_DEBUG_INITIAL_COMMAND_EX
    | FLG_DISABLE_PAGE_KERNEL_STACKS;
pub const FLG_KERNELMODE_VALID_BITS: u32 = FLG_STOP_ON_EXCEPTION
    | FLG_SHOW_LDR_SNAPS
    | FLG_STOP_ON_HUNG_GUI
    | FLG_POOL_ENABLE_TAGGING
    | FLG_ENABLE_KDEBUG_SYMBOL_LOAD
    | FLG_ENABLE_CLOSE_EXCEPTIONS
    | FLG_ENABLE_EXCEPTION_LOGGING
    | FLG_ENABLE_HANDLE_TYPE_TAGGING
    | FLG_DISABLE_DBGPRINT
    | FLG_ENABLE_HANDLE_EXCEPTIONS;
EXTERN! {extern "system" {
    fn NtQueryLicenseValue(
        ValueName: *mut UNICODE_STRING,
        Type: *mut c_ulong,
        Data: *mut c_void,
        DataSize: c_ulong,
        ResultDataSize: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtSetDefaultHardErrorPort(
        DefaultHardErrorPort: HANDLE,
    ) -> NTSTATUS;
}}
ENUM! {enum SHUTDOWN_ACTION {
    ShutdownNoReboot = 0,
    ShutdownReboot = 1,
    ShutdownPowerOff = 2,
}}
EXTERN! {extern "system" {
    fn NtShutdownSystem(
        Action: SHUTDOWN_ACTION,
    ) -> NTSTATUS;
    fn NtDisplayString(
        String: *mut UNICODE_STRING,
    ) -> NTSTATUS;
    fn NtDrawText(
        Text: *mut UNICODE_STRING,
    ) -> NTSTATUS;
}}
