use windows_sys::Win32::{
    Foundation::{HANDLE, NTSTATUS},
    System::{
        SystemServices::TP_CLEANUP_GROUP,
        Threading::{
            PTP_CALLBACK_INSTANCE, PTP_IO, PTP_POOL, PTP_SIMPLE_CALLBACK,
            PTP_TIMER, PTP_TIMER_CALLBACK, PTP_WAIT, PTP_WAIT_CALLBACK,
            PTP_WORK, PTP_WORK_CALLBACK, RTL_CRITICAL_SECTION,
            TP_CALLBACK_ENVIRON_V3, TP_POOL_STACK_INFORMATION,
        },
    },
};

use crate::{
    ctypes::{c_long, c_ulong, c_void},
    ntioapi::PIO_STATUS_BLOCK,
    windows_local::shared::ntdef::LARGE_INTEGER,
};

#[repr(C)]
pub struct TP_ALPC([u8; 0]);
pub type PTP_ALPC = *mut TP_ALPC;
FN! {stdcall PTP_ALPC_CALLBACK(
    Instance: PTP_CALLBACK_INSTANCE,
    Context: *mut c_void,
    Alpc: PTP_ALPC,
) -> ()}
FN! {stdcall PTP_ALPC_CALLBACK_EX(
    Instanc: PTP_CALLBACK_INSTANCE,
    Contex: *mut c_void,
    Alp: PTP_ALPC,
    ApcContext: *mut c_void,
) -> ()}
EXTERN! {extern "system" {
    fn TpAllocPool(
        PoolReturn: *mut PTP_POOL,
        Reserved: *mut c_void,
    ) -> NTSTATUS;
    fn TpReleasePool(
        Pool: PTP_POOL,
    );
    fn TpSetPoolMaxThreads(
        Pool: PTP_POOL,
        MaxThreads: c_long,
    );
    fn TpSetPoolMinThreads(
        Pool: PTP_POOL,
        MinThreads: c_long,
    ) -> NTSTATUS;
    fn TpQueryPoolStackInformation(
        Pool: PTP_POOL,
        PoolStackInformation: *mut TP_POOL_STACK_INFORMATION,
    ) -> NTSTATUS;
    fn TpSetPoolStackInformation(
        Pool: PTP_POOL,
        PoolStackInformation: *mut TP_POOL_STACK_INFORMATION,
    ) -> NTSTATUS;
    fn TpAllocCleanupGroup(
        CleanupGroupReturn: *mut *mut TP_CLEANUP_GROUP,
    ) -> NTSTATUS;
    fn TpReleaseCleanupGroup(
        CleanupGroup: *mut TP_CLEANUP_GROUP,
    );
    fn TpReleaseCleanupGroupMembers(
        CleanupGroup: *mut TP_CLEANUP_GROUP,
        CancelPendingCallbacks: c_ulong,
        CleanupParameter: *mut c_void,
    );
    fn TpCallbackSetEventOnCompletion(
        Instance: PTP_CALLBACK_INSTANCE,
        Event: HANDLE,
    );
    fn TpCallbackReleaseSemaphoreOnCompletion(
        Instance: PTP_CALLBACK_INSTANCE,
        Semaphore: HANDLE,
        ReleaseCount: c_long,
    );
    fn TpCallbackReleaseMutexOnCompletion(
        Instance: PTP_CALLBACK_INSTANCE,
        Mutex: HANDLE,
    );
    fn TpCallbackLeaveCriticalSectionOnCompletion(
        Instance: PTP_CALLBACK_INSTANCE,
        CriticalSection: *mut RTL_CRITICAL_SECTION,
    );
    fn TpCallbackUnloadDllOnCompletion(
        Instance: PTP_CALLBACK_INSTANCE,
        DllHandle: *mut c_void,
    );
    fn TpCallbackMayRunLong(
        Instance: PTP_CALLBACK_INSTANCE,
    ) -> NTSTATUS;
    fn TpDisassociateCallback(
        Instance: PTP_CALLBACK_INSTANCE,
    );
    fn TpSimpleTryPost(
        Callback: PTP_SIMPLE_CALLBACK,
        Context: *mut c_void,
        CallbackEnviron: *mut TP_CALLBACK_ENVIRON_V3,
    ) -> NTSTATUS;
    fn TpAllocWork(
        WorkReturn: *mut PTP_WORK,
        Callback: PTP_WORK_CALLBACK,
        Context: *mut c_void,
        CallbackEnviron: *mut TP_CALLBACK_ENVIRON_V3,
    ) -> NTSTATUS;
    fn TpReleaseWork(
        Work: PTP_WORK,
    );
    fn TpPostWork(
        Work: PTP_WORK,
    );
    fn TpWaitForWork(
        Work: PTP_WORK,
        CancelPendingCallbacks: c_ulong,
    );
    fn TpAllocTimer(
        Timer: *mut PTP_TIMER,
        Callback: PTP_TIMER_CALLBACK,
        Context: *mut c_void,
        CallbackEnviron: *mut TP_CALLBACK_ENVIRON_V3,
    ) -> NTSTATUS;
    fn TpReleaseTimer(
        Timer: PTP_TIMER,
    );
    fn TpSetTimer(
        Timer: PTP_TIMER,
        DueTime: *mut LARGE_INTEGER,
        Period: c_long,
        WindowLength: c_long,
    );
    fn TpIsTimerSet(
        Timer: PTP_TIMER,
    ) -> c_ulong;
    fn TpWaitForTimer(
        Timer: PTP_TIMER,
        CancelPendingCallbacks: c_ulong,
    );
    fn TpAllocWait(
        WaitReturn: *mut PTP_WAIT,
        Callback: PTP_WAIT_CALLBACK,
        Context: *mut c_void,
        CallbackEnviron: *mut TP_CALLBACK_ENVIRON_V3,
    ) -> NTSTATUS;
    fn TpReleaseWait(
        Wait: PTP_WAIT,
    );
    fn TpSetWait(
        Wait: PTP_WAIT,
        Handle: HANDLE,
        Timeout: *mut LARGE_INTEGER,
    );
    fn TpWaitForWait(
        Wait: PTP_WAIT,
        CancelPendingCallbacks: c_ulong,
    );
}}
FN! {stdcall PTP_IO_CALLBACK(
    Instance: PTP_CALLBACK_INSTANCE,
    Context: *mut c_void,
    ApcContext: *mut c_void,
    IoSB: PIO_STATUS_BLOCK,
    Io: PTP_IO,
) -> ()}
EXTERN! {extern "system" {
    fn TpAllocIoCompletion(
        IoReturn: *mut PTP_IO,
        File: HANDLE,
        Callback: PTP_IO_CALLBACK,
        Context: *mut c_void,
        CallbackEnviron: *mut TP_CALLBACK_ENVIRON_V3,
    ) -> NTSTATUS;
    fn TpReleaseIoCompletion(
        Io: PTP_IO,
    );
    fn TpStartAsyncIoOperation(
        Io: PTP_IO,
    );
    fn TpCancelAsyncIoOperation(
        Io: PTP_IO,
    );
    fn TpWaitForIoCompletion(
        Io: PTP_IO,
        CancelPendingCallbacks: c_ulong,
    );
    fn TpAllocAlpcCompletion(
        AlpcReturn: *mut PTP_ALPC,
        AlpcPort: HANDLE,
        Callback: PTP_ALPC_CALLBACK,
        Context: *mut c_void,
        CallbackEnviron: *mut TP_CALLBACK_ENVIRON_V3,
    ) -> NTSTATUS;
    fn TpAllocAlpcCompletionEx(
        AlpcReturn: *mut PTP_ALPC,
        AlpcPort: HANDLE,
        Callback: PTP_ALPC_CALLBACK_EX,
        Context: *mut c_void,
        CallbackEnviron: *mut TP_CALLBACK_ENVIRON_V3,
    ) -> NTSTATUS;
    fn TpReleaseAlpcCompletion(
        Alpc: PTP_ALPC,
    );
    fn TpWaitForAlpcCompletion(
        Alpc: PTP_ALPC,
    );
}}
ENUM! {enum TP_TRACE_TYPE {
    TpTraceThreadPriority = 1,
    TpTraceThreadAffinity = 2,
    MaxTpTraceType = 3,
}}
EXTERN! {extern "system" {
    fn TpCaptureCaller(
        Type: TP_TRACE_TYPE,
    );
    fn TpCheckTerminateWorker(
        Thread: HANDLE,
    );
}}
