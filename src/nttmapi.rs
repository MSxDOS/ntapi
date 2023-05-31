use windows_sys::{
    core::GUID,
    Win32::{
        Foundation::{HANDLE, NTSTATUS, UNICODE_STRING},
        Storage::FileSystem::TRANSACTION_NOTIFICATION,
        System::{
            SystemServices::{
                ENLISTMENT_INFORMATION_CLASS, KTMOBJECT_CURSOR, KTMOBJECT_TYPE,
                RESOURCEMANAGER_INFORMATION_CLASS,
                TRANSACTIONMANAGER_INFORMATION_CLASS,
                TRANSACTION_INFORMATION_CLASS,
            },
            WindowsProgramming::OBJECT_ATTRIBUTES,
        },
    },
};

use crate::{
    ctypes::{c_uchar, c_ulong, c_void},
    windows_local::shared::ntdef::LARGE_INTEGER,
};

EXTERN! {extern "system" {
    fn NtCreateTransactionManager(
        TmHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        LogFileName: *mut UNICODE_STRING,
        CreateOptions: c_ulong,
        CommitStrength: c_ulong,
    ) -> NTSTATUS;
    fn NtOpenTransactionManager(
        TmHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        LogFileName: *mut UNICODE_STRING,
        TmIdentity: *mut GUID,
        OpenOptions: c_ulong,
    ) -> NTSTATUS;
    fn NtRenameTransactionManager(
        LogFileName: *mut UNICODE_STRING,
        ExistingTransactionManagerGuid: *mut GUID,
    ) -> NTSTATUS;
    fn NtRollforwardTransactionManager(
        TransactionManagerHandle: HANDLE,
        TmVirtualClock: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtRecoverTransactionManager(
        TransactionManagerHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtQueryInformationTransactionManager(
        TransactionManagerHandle: HANDLE,
        TransactionManagerInformationClass: TRANSACTIONMANAGER_INFORMATION_CLASS,
        TransactionManagerInformation: *mut c_void,
        TransactionManagerInformationLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtSetInformationTransactionManager(
        TmHandle: HANDLE,
        TransactionManagerInformationClass: TRANSACTIONMANAGER_INFORMATION_CLASS,
        TransactionManagerInformation: *mut c_void,
        TransactionManagerInformationLength: c_ulong,
    ) -> NTSTATUS;
    fn NtEnumerateTransactionObject(
        RootObjectHandle: HANDLE,
        QueryType: KTMOBJECT_TYPE,
        ObjectCursor: *mut KTMOBJECT_CURSOR,
        ObjectCursorLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtCreateTransaction(
        TransactionHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        Uow: *mut GUID,
        TmHandle: HANDLE,
        CreateOptions: c_ulong,
        IsolationLevel: c_ulong,
        IsolationFlags: c_ulong,
        Timeout: *mut LARGE_INTEGER,
        Description: *mut UNICODE_STRING,
    ) -> NTSTATUS;
    fn NtOpenTransaction(
        TransactionHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        Uow: *mut GUID,
        TmHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtQueryInformationTransaction(
        TransactionHandle: HANDLE,
        TransactionInformationClass: TRANSACTION_INFORMATION_CLASS,
        TransactionInformation: *mut c_void,
        TransactionInformationLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtSetInformationTransaction(
        TransactionHandle: HANDLE,
        TransactionInformationClass: TRANSACTION_INFORMATION_CLASS,
        TransactionInformation: *mut c_void,
        TransactionInformationLength: c_ulong,
    ) -> NTSTATUS;
    fn NtCommitTransaction(
        TransactionHandle: HANDLE,
        Wait: c_uchar,
    ) -> NTSTATUS;
    fn NtRollbackTransaction(
        TransactionHandle: HANDLE,
        Wait: c_uchar,
    ) -> NTSTATUS;
    fn NtCreateEnlistment(
        EnlistmentHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ResourceManagerHandle: HANDLE,
        TransactionHandle: HANDLE,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        CreateOptions: c_ulong,
        NotificationMask: c_ulong,
        EnlistmentKey: *mut c_void,
    ) -> NTSTATUS;
    fn NtOpenEnlistment(
        EnlistmentHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ResourceManagerHandle: HANDLE,
        EnlistmentGuid: *mut GUID,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn NtQueryInformationEnlistment(
        EnlistmentHandle: HANDLE,
        EnlistmentInformationClass: ENLISTMENT_INFORMATION_CLASS,
        EnlistmentInformation: *mut c_void,
        EnlistmentInformationLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtSetInformationEnlistment(
        EnlistmentHandle: HANDLE,
        EnlistmentInformationClass: ENLISTMENT_INFORMATION_CLASS,
        EnlistmentInformation: *mut c_void,
        EnlistmentInformationLength: c_ulong,
    ) -> NTSTATUS;
    fn NtRecoverEnlistment(
        EnlistmentHandle: HANDLE,
        EnlistmentKey: *mut c_void,
    ) -> NTSTATUS;
    fn NtPrePrepareEnlistment(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtPrepareEnlistment(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtCommitEnlistment(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtRollbackEnlistment(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtPrePrepareComplete(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtPrepareComplete(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtCommitComplete(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtReadOnlyEnlistment(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtRollbackComplete(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtSinglePhaseReject(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtCreateResourceManager(
        ResourceManagerHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        TmHandle: HANDLE,
        RmGuid: *mut GUID,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        CreateOptions: c_ulong,
        Description: *mut UNICODE_STRING,
    ) -> NTSTATUS;
    fn NtOpenResourceManager(
        ResourceManagerHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        TmHandle: HANDLE,
        ResourceManagerGuid: *mut GUID,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn NtRecoverResourceManager(
        ResourceManagerHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtGetNotificationResourceManager(
        ResourceManagerHandle: HANDLE,
        TransactionNotification: *mut TRANSACTION_NOTIFICATION,
        NotificationLength: c_ulong,
        Timeout: *mut LARGE_INTEGER,
        ReturnLength: *mut c_ulong,
        Asynchronous: c_ulong,
        AsynchronousContext: usize,
    ) -> NTSTATUS;
    fn NtQueryInformationResourceManager(
        ResourceManagerHandle: HANDLE,
        ResourceManagerInformationClass: RESOURCEMANAGER_INFORMATION_CLASS,
        ResourceManagerInformation: *mut c_void,
        ResourceManagerInformationLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtSetInformationResourceManager(
        ResourceManagerHandle: HANDLE,
        ResourceManagerInformationClass: RESOURCEMANAGER_INFORMATION_CLASS,
        ResourceManagerInformation: *mut c_void,
        ResourceManagerInformationLength: c_ulong,
    ) -> NTSTATUS;
    fn NtRegisterProtocolAddressInformation(
        ResourceManager: HANDLE,
        ProtocolId: *mut GUID,
        ProtocolInformationSize: c_ulong,
        ProtocolInformation: *mut c_void,
        CreateOptions: c_ulong,
    ) -> NTSTATUS;
    fn NtPropagationComplete(
        ResourceManagerHandle: HANDLE,
        RequestCookie: c_ulong,
        BufferLength: c_ulong,
        Buffer: *mut c_void,
    ) -> NTSTATUS;
    fn NtPropagationFailed(
        ResourceManagerHandle: HANDLE,
        RequestCookie: c_ulong,
        PropStatus: NTSTATUS,
    ) -> NTSTATUS;
    fn NtFreezeTransactions(
        FreezeTimeout: *mut LARGE_INTEGER,
        ThawTimeout: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtThawTransactions() -> NTSTATUS;
}}
