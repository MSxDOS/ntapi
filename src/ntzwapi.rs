use windows_sys::{
    core::GUID,
    Win32::{
        Foundation::{HANDLE, LUID, NTSTATUS, PSID, UNICODE_STRING},
        Security::{
            AUDIT_EVENT_TYPE, GENERIC_MAPPING, OBJECT_TYPE_LIST, PRIVILEGE_SET,
            PSECURITY_DESCRIPTOR, SECURITY_QUALITY_OF_SERVICE,
            SID_AND_ATTRIBUTES, TOKEN_DEFAULT_DACL, TOKEN_GROUPS,
            TOKEN_INFORMATION_CLASS, TOKEN_MANDATORY_POLICY, TOKEN_OWNER,
            TOKEN_PRIMARY_GROUP, TOKEN_PRIVILEGES, TOKEN_SOURCE, TOKEN_TYPE,
            TOKEN_USER,
        },
        Storage::FileSystem::{FILE_SEGMENT_ELEMENT, TRANSACTION_NOTIFICATION},
        System::{
            Diagnostics::Debug::{CONTEXT, EXCEPTION_RECORD},
            JobObjects::{JOBOBJECTINFOCLASS, JOB_SET_ARRAY},
            Kernel::{EVENT_TYPE, TIMER_TYPE, WAIT_TYPE, WNF_STATE_NAME},
            Power::{
                DEVICE_POWER_STATE, EXECUTION_STATE, LATENCY_TIME,
                POWER_ACTION, POWER_INFORMATION_LEVEL, SYSTEM_POWER_STATE,
            },
            SystemInformation::GROUP_AFFINITY,
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
    ctypes::{
        __uint64, c_char, c_long, c_uchar, c_ulong, c_ushort, c_void, wchar_t,
    },
    ntapi_base::{PCLIENT_ID, PRTL_ATOM, RTL_ATOM},
    ntdbg::DEBUGOBJECTINFOCLASS,
    ntexapi::{
        ATOM_INFORMATION_CLASS, EVENT_INFORMATION_CLASS,
        MUTANT_INFORMATION_CLASS, PBOOT_ENTRY, PBOOT_OPTIONS, PCWNF_TYPE_ID,
        PEFI_DRIVER_ENTRY, PFILE_PATH, PT2_CANCEL_PARAMETERS,
        PT2_SET_PARAMETERS, PTIMER_APC_ROUTINE, PWNF_CHANGE_STAMP,
        PWNF_DELIVERY_DESCRIPTOR, SEMAPHORE_INFORMATION_CLASS, SHUTDOWN_ACTION,
        SYSDBG_COMMAND, SYSTEM_INFORMATION_CLASS, TIMER_INFORMATION_CLASS,
        TIMER_SET_INFORMATION_CLASS, WNF_CHANGE_STAMP, WNF_DATA_SCOPE,
        WNF_STATE_NAME_INFORMATION, WNF_STATE_NAME_LIFETIME,
        WORKERFACTORYINFOCLASS,
    },
    ntioapi::{
        FILE_INFORMATION_CLASS, FILE_IO_COMPLETION_INFORMATION,
        FS_INFORMATION_CLASS, IO_COMPLETION_INFORMATION_CLASS,
        IO_SESSION_EVENT, IO_SESSION_STATE, PFILE_BASIC_INFORMATION,
        PFILE_IO_COMPLETION_INFORMATION, PFILE_NETWORK_OPEN_INFORMATION,
        PIO_APC_ROUTINE, PIO_STATUS_BLOCK,
    },
    ntkeapi::KPROFILE_SOURCE,
    ntlpcapi::{
        ALPC_HANDLE, ALPC_MESSAGE_INFORMATION_CLASS,
        ALPC_PORT_INFORMATION_CLASS, PALPC_CONTEXT_ATTR, PALPC_DATA_VIEW_ATTR,
        PALPC_HANDLE, PALPC_MESSAGE_ATTRIBUTES, PALPC_PORT_ATTRIBUTES,
        PALPC_SECURITY_ATTR, PORT_INFORMATION_CLASS, PPORT_MESSAGE, PPORT_VIEW,
        PREMOTE_PORT_VIEW,
    },
    ntmisc::VDMSERVICECLASS,
    ntmmapi::{
        MEMORY_INFORMATION_CLASS, MEMORY_PARTITION_INFORMATION_CLASS,
        PMEMORY_RANGE_ENTRY, SECTION_INFORMATION_CLASS, SECTION_INHERIT,
        VIRTUAL_MEMORY_INFORMATION_CLASS,
    },
    ntobapi::OBJECT_INFORMATION_CLASS,
    ntpnpapi::{PLUGPLAY_CONTROL_CLASS, PPLUGPLAY_EVENT_BLOCK},
    ntpsapi::{
        MEMORY_RESERVE_TYPE, PINITIAL_TEB, PPS_APC_ROUTINE, PPS_ATTRIBUTE_LIST,
        PPS_CREATE_INFO, PROCESSINFOCLASS, THREADINFOCLASS,
    },
    ntregapi::{
        KEY_INFORMATION_CLASS, KEY_SET_INFORMATION_CLASS,
        KEY_VALUE_INFORMATION_CLASS, PKEY_VALUE_ENTRY,
    },
    ntseapi::PTOKEN_SECURITY_ATTRIBUTES_INFORMATION,
    windows_local::shared::ntdef::{LARGE_INTEGER, ULARGE_INTEGER},
};

EXTERN! {extern "system" {
    fn ZwAcceptConnectPort(
        PortHandle: *mut HANDLE,
        PortContext: *mut c_void,
        ConnectionRequest: PPORT_MESSAGE,
        AcceptConnection: c_uchar,
        ServerView: PPORT_VIEW,
        ClientView: PREMOTE_PORT_VIEW,
    ) -> NTSTATUS;
    fn ZwAccessCheck(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        ClientToken: HANDLE,
        DesiredAccess: c_ulong,
        GenericMapping: *mut GENERIC_MAPPING,
        PrivilegeSet: *mut PRIVILEGE_SET,
        PrivilegeSetLength: *mut c_ulong,
        GrantedAccess: *mut c_ulong,
        AccessStatus: *mut NTSTATUS,
    ) -> NTSTATUS;
    fn ZwAccessCheckAndAuditAlarm(
        SubsystemName: *mut UNICODE_STRING,
        HandleId: *mut c_void,
        ObjectTypeName: *mut UNICODE_STRING,
        ObjectName: *mut UNICODE_STRING,
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        DesiredAccess: c_ulong,
        GenericMapping: *mut GENERIC_MAPPING,
        ObjectCreation: c_uchar,
        GrantedAccess: *mut c_ulong,
        AccessStatus: *mut NTSTATUS,
        GenerateOnClose: *mut c_uchar,
    ) -> NTSTATUS;
    fn ZwAccessCheckByType(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        PrincipalSelfSid: PSID,
        ClientToken: HANDLE,
        DesiredAccess: c_ulong,
        ObjectTypeList: *mut OBJECT_TYPE_LIST,
        ObjectTypeListLength: c_ulong,
        GenericMapping: *mut GENERIC_MAPPING,
        PrivilegeSet: *mut PRIVILEGE_SET,
        PrivilegeSetLength: *mut c_ulong,
        GrantedAccess: *mut c_ulong,
        AccessStatus: *mut NTSTATUS,
    ) -> NTSTATUS;
    fn ZwAccessCheckByTypeAndAuditAlarm(
        SubsystemName: *mut UNICODE_STRING,
        HandleId: *mut c_void,
        ObjectTypeName: *mut UNICODE_STRING,
        ObjectName: *mut UNICODE_STRING,
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        PrincipalSelfSid: PSID,
        DesiredAccess: c_ulong,
        AuditType: AUDIT_EVENT_TYPE,
        Flags: c_ulong,
        ObjectTypeList: *mut OBJECT_TYPE_LIST,
        ObjectTypeListLength: c_ulong,
        GenericMapping: *mut GENERIC_MAPPING,
        ObjectCreation: c_uchar,
        GrantedAccess: *mut c_ulong,
        AccessStatus: *mut NTSTATUS,
        GenerateOnClose: *mut c_uchar,
    ) -> NTSTATUS;
    fn ZwAccessCheckByTypeResultList(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        PrincipalSelfSid: PSID,
        ClientToken: HANDLE,
        DesiredAccess: c_ulong,
        ObjectTypeList: *mut OBJECT_TYPE_LIST,
        ObjectTypeListLength: c_ulong,
        GenericMapping: *mut GENERIC_MAPPING,
        PrivilegeSet: *mut PRIVILEGE_SET,
        PrivilegeSetLength: *mut c_ulong,
        GrantedAccess: *mut c_ulong,
        AccessStatus: *mut NTSTATUS,
    ) -> NTSTATUS;
    fn ZwAccessCheckByTypeResultListAndAuditAlarm(
        SubsystemName: *mut UNICODE_STRING,
        HandleId: *mut c_void,
        ObjectTypeName: *mut UNICODE_STRING,
        ObjectName: *mut UNICODE_STRING,
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        PrincipalSelfSid: PSID,
        DesiredAccess: c_ulong,
        AuditType: AUDIT_EVENT_TYPE,
        Flags: c_ulong,
        ObjectTypeList: *mut OBJECT_TYPE_LIST,
        ObjectTypeListLength: c_ulong,
        GenericMapping: *mut GENERIC_MAPPING,
        ObjectCreation: c_uchar,
        GrantedAccess: *mut c_ulong,
        AccessStatus: *mut NTSTATUS,
        GenerateOnClose: *mut c_uchar,
    ) -> NTSTATUS;
    fn ZwAccessCheckByTypeResultListAndAuditAlarmByHandle(
        SubsystemName: *mut UNICODE_STRING,
        HandleId: *mut c_void,
        ClientToken: HANDLE,
        ObjectTypeName: *mut UNICODE_STRING,
        ObjectName: *mut UNICODE_STRING,
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        PrincipalSelfSid: PSID,
        DesiredAccess: c_ulong,
        AuditType: AUDIT_EVENT_TYPE,
        Flags: c_ulong,
        ObjectTypeList: *mut OBJECT_TYPE_LIST,
        ObjectTypeListLength: c_ulong,
        GenericMapping: *mut GENERIC_MAPPING,
        ObjectCreation: c_uchar,
        GrantedAccess: *mut c_ulong,
        AccessStatus: *mut NTSTATUS,
        GenerateOnClose: *mut c_uchar,
    ) -> NTSTATUS;
    fn ZwAcquireCMFViewOwnership(
        TimeStamp: *mut __uint64,
        tokenTaken: *mut c_uchar,
        replaceExisting: c_uchar,
    ) -> NTSTATUS;
    fn ZwAddAtom(
        AtomName: *mut wchar_t,
        Length: c_ulong,
        Atom: PRTL_ATOM,
    ) -> NTSTATUS;
    fn ZwAddAtomEx(
        AtomName: *mut wchar_t,
        Length: c_ulong,
        Atom: PRTL_ATOM,
        Flags: c_ulong,
    ) -> NTSTATUS;
    fn ZwAddBootEntry(
        BootEntry: PBOOT_ENTRY,
        Id: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwAddDriverEntry(
        DriverEntry: PEFI_DRIVER_ENTRY,
        Id: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwAdjustGroupsToken(
        TokenHandle: HANDLE,
        ResetToDefault: c_uchar,
        NewState: *mut TOKEN_GROUPS,
        BufferLength: c_ulong,
        PreviousState: *mut TOKEN_GROUPS,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwAdjustPrivilegesToken(
        TokenHandle: HANDLE,
        DisableAllPrivileges: c_uchar,
        NewState: *mut TOKEN_PRIVILEGES,
        BufferLength: c_ulong,
        PreviousState: *mut TOKEN_PRIVILEGES,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwAdjustTokenClaimsAndDeviceGroups(
        TokenHandle: HANDLE,
        UserResetToDefault: c_uchar,
        DeviceResetToDefault: c_uchar,
        DeviceGroupsResetToDefault: c_uchar,
        NewUserState: PTOKEN_SECURITY_ATTRIBUTES_INFORMATION,
        NewDeviceState: PTOKEN_SECURITY_ATTRIBUTES_INFORMATION,
        NewDeviceGroupsState: *mut TOKEN_GROUPS,
        UserBufferLength: c_ulong,
        PreviousUserState: PTOKEN_SECURITY_ATTRIBUTES_INFORMATION,
        DeviceBufferLength: c_ulong,
        PreviousDeviceState: PTOKEN_SECURITY_ATTRIBUTES_INFORMATION,
        DeviceGroupsBufferLength: c_ulong,
        PreviousDeviceGroups: *mut TOKEN_GROUPS,
        UserReturnLength: *mut c_ulong,
        DeviceReturnLength: *mut c_ulong,
        DeviceGroupsReturnBufferLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwAlertResumeThread(
        ThreadHandle: HANDLE,
        PreviousSuspendCount: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwAlertThread(
        ThreadHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwAlertThreadByThreadId(
        ThreadId: HANDLE,
    ) -> NTSTATUS;
    fn ZwAllocateLocallyUniqueId(
        Luid: *mut LUID,
    ) -> NTSTATUS;
    fn ZwAllocateReserveObject(
        MemoryReserveHandle: *mut HANDLE,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        Type: MEMORY_RESERVE_TYPE,
    ) -> NTSTATUS;
    fn ZwAllocateUserPhysicalPages(
        ProcessHandle: HANDLE,
        NumberOfPages: *mut usize,
        UserPfnArray: *mut usize,
    ) -> NTSTATUS;
    fn ZwAllocateUuids(
        Time: *mut ULARGE_INTEGER,
        Range: *mut c_ulong,
        Sequence: *mut c_ulong,
        Seed: *mut c_char,
    ) -> NTSTATUS;
    fn ZwAllocateVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut *mut c_void,
        ZeroBits: usize,
        RegionSize: *mut usize,
        AllocationType: c_ulong,
        Protect: c_ulong,
    ) -> NTSTATUS;
    fn ZwAlpcAcceptConnectPort(
        PortHandle: *mut HANDLE,
        ConnectionPortHandle: HANDLE,
        Flags: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        PortAttributes: PALPC_PORT_ATTRIBUTES,
        PortContext: *mut c_void,
        ConnectionRequest: PPORT_MESSAGE,
        ConnectionMessageAttributes: PALPC_MESSAGE_ATTRIBUTES,
        AcceptConnection: c_uchar,
    ) -> NTSTATUS;
    fn ZwAlpcCancelMessage(
        PortHandle: HANDLE,
        Flags: c_ulong,
        MessageContext: PALPC_CONTEXT_ATTR,
    ) -> NTSTATUS;
    fn ZwAlpcConnectPort(
        PortHandle: *mut HANDLE,
        PortName: *mut UNICODE_STRING,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        PortAttributes: PALPC_PORT_ATTRIBUTES,
        Flags: c_ulong,
        RequiredServerSid: PSID,
        ConnectionMessage: PPORT_MESSAGE,
        BufferLength: *mut c_ulong,
        OutMessageAttributes: PALPC_MESSAGE_ATTRIBUTES,
        InMessageAttributes: PALPC_MESSAGE_ATTRIBUTES,
        Timeout: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwAlpcConnectPortEx(
        PortHandle: *mut HANDLE,
        ConnectionPortObjectAttributes: *mut OBJECT_ATTRIBUTES,
        ClientPortObjectAttributes: *mut OBJECT_ATTRIBUTES,
        PortAttributes: PALPC_PORT_ATTRIBUTES,
        Flags: c_ulong,
        ServerSecurityRequirements: PSECURITY_DESCRIPTOR,
        ConnectionMessage: PPORT_MESSAGE,
        BufferLength: *mut usize,
        OutMessageAttributes: PALPC_MESSAGE_ATTRIBUTES,
        InMessageAttributes: PALPC_MESSAGE_ATTRIBUTES,
        Timeout: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwAlpcCreatePort(
        PortHandle: *mut HANDLE,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        PortAttributes: PALPC_PORT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwAlpcCreatePortSection(
        PortHandle: HANDLE,
        Flags: c_ulong,
        SectionHandle: HANDLE,
        SectionSize: usize,
        AlpcSectionHandle: PALPC_HANDLE,
        ActualSectionSize: *mut usize,
    ) -> NTSTATUS;
    fn ZwAlpcCreateResourceReserve(
        PortHandle: HANDLE,
        Flags: c_ulong,
        MessageSize: usize,
        ResourceId: PALPC_HANDLE,
    ) -> NTSTATUS;
    fn ZwAlpcCreateSectionView(
        PortHandle: HANDLE,
        Flags: c_ulong,
        ViewAttributes: PALPC_DATA_VIEW_ATTR,
    ) -> NTSTATUS;
    fn ZwAlpcCreateSecurityContext(
        PortHandle: HANDLE,
        Flags: c_ulong,
        SecurityAttribute: PALPC_SECURITY_ATTR,
    ) -> NTSTATUS;
    fn ZwAlpcDeletePortSection(
        PortHandle: HANDLE,
        Flags: c_ulong,
        SectionHandle: ALPC_HANDLE,
    ) -> NTSTATUS;
    fn ZwAlpcDeleteResourceReserve(
        PortHandle: HANDLE,
        Flags: c_ulong,
        ResourceId: ALPC_HANDLE,
    ) -> NTSTATUS;
    fn ZwAlpcDeleteSectionView(
        PortHandle: HANDLE,
        Flags: c_ulong,
        ViewBase: *mut c_void,
    ) -> NTSTATUS;
    fn ZwAlpcDeleteSecurityContext(
        PortHandle: HANDLE,
        Flags: c_ulong,
        ContextHandle: ALPC_HANDLE,
    ) -> NTSTATUS;
    fn ZwAlpcDisconnectPort(
        PortHandle: HANDLE,
        Flags: c_ulong,
    ) -> NTSTATUS;
    fn ZwAlpcImpersonateClientContainerOfPort(
        PortHandle: HANDLE,
        Message: PPORT_MESSAGE,
        Flags: c_ulong,
    ) -> NTSTATUS;
    fn ZwAlpcImpersonateClientOfPort(
        PortHandle: HANDLE,
        Message: PPORT_MESSAGE,
        Flags: *mut c_void,
    ) -> NTSTATUS;
    fn ZwAlpcOpenSenderProcess(
        ProcessHandle: *mut HANDLE,
        PortHandle: HANDLE,
        PortMessage: PPORT_MESSAGE,
        Flags: c_ulong,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwAlpcOpenSenderThread(
        ThreadHandle: *mut HANDLE,
        PortHandle: HANDLE,
        PortMessage: PPORT_MESSAGE,
        Flags: c_ulong,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwAlpcQueryInformation(
        PortHandle: HANDLE,
        PortInformationClass: ALPC_PORT_INFORMATION_CLASS,
        PortInformation: *mut c_void,
        Length: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwAlpcQueryInformationMessage(
        PortHandle: HANDLE,
        PortMessage: PPORT_MESSAGE,
        MessageInformationClass: ALPC_MESSAGE_INFORMATION_CLASS,
        MessageInformation: *mut c_void,
        Length: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwAlpcRevokeSecurityContext(
        PortHandle: HANDLE,
        Flags: c_ulong,
        ContextHandle: ALPC_HANDLE,
    ) -> NTSTATUS;
    fn ZwAlpcSendWaitReceivePort(
        PortHandle: HANDLE,
        Flags: c_ulong,
        SendMessageA: PPORT_MESSAGE,
        SendMessageAttributes: PALPC_MESSAGE_ATTRIBUTES,
        ReceiveMessage: PPORT_MESSAGE,
        BufferLength: *mut usize,
        ReceiveMessageAttributes: PALPC_MESSAGE_ATTRIBUTES,
        Timeout: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwAlpcSetInformation(
        PortHandle: HANDLE,
        PortInformationClass: ALPC_PORT_INFORMATION_CLASS,
        PortInformation: *mut c_void,
        Length: c_ulong,
    ) -> NTSTATUS;
    fn ZwAreMappedFilesTheSame(
        File1MappedAsAnImage: *mut c_void,
        File2MappedAsFile: *mut c_void,
    ) -> NTSTATUS;
    fn ZwAssignProcessToJobObject(
        JobHandle: HANDLE,
        ProcessHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwAssociateWaitCompletionPacket(
        WaitCompletionPacketHandle: HANDLE,
        IoCompletionHandle: HANDLE,
        TargetObjectHandle: HANDLE,
        KeyContext: *mut c_void,
        ApcContext: *mut c_void,
        IoStatus: NTSTATUS,
        IoStatusInformation: usize,
        AlreadySignaled: *mut c_uchar,
    ) -> NTSTATUS;
    fn ZwCallbackReturn(
        OutputBuffer: *mut c_void,
        OutputLength: c_ulong,
        Status: NTSTATUS,
    ) -> NTSTATUS;
    fn ZwCancelIoFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
    ) -> NTSTATUS;
    fn ZwCancelIoFileEx(
        FileHandle: HANDLE,
        IoRequestToCancel: PIO_STATUS_BLOCK,
        IoStatusBlock: PIO_STATUS_BLOCK,
    ) -> NTSTATUS;
    fn ZwCancelSynchronousIoFile(
        ThreadHandle: HANDLE,
        IoRequestToCancel: PIO_STATUS_BLOCK,
        IoStatusBlock: PIO_STATUS_BLOCK,
    ) -> NTSTATUS;
    fn ZwCancelTimer(
        TimerHandle: HANDLE,
        CurrentState: *mut c_uchar,
    ) -> NTSTATUS;
    fn ZwCancelTimer2(
        TimerHandle: HANDLE,
        Parameters: PT2_CANCEL_PARAMETERS,
    ) -> NTSTATUS;
    fn ZwCancelWaitCompletionPacket(
        WaitCompletionPacketHandle: HANDLE,
        RemoveSignaledPacket: c_uchar,
    ) -> NTSTATUS;
    fn ZwClearEvent(
        EventHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwClose(
        Handle: HANDLE,
    ) -> NTSTATUS;
    fn ZwCloseObjectAuditAlarm(
        SubsystemName: *mut UNICODE_STRING,
        HandleId: *mut c_void,
        GenerateOnClose: c_uchar,
    ) -> NTSTATUS;
    fn ZwCommitComplete(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwCommitEnlistment(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwCommitTransaction(
        TransactionHandle: HANDLE,
        Wait: c_uchar,
    ) -> NTSTATUS;
    fn ZwCompactKeys(
        Count: c_ulong,
        KeyArray: *mut HANDLE,
    ) -> NTSTATUS;
    fn ZwCompareObjects(
        FirstObjectHandle: HANDLE,
        SecondObjectHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwCompareTokens(
        FirstTokenHandle: HANDLE,
        SecondTokenHandle: HANDLE,
        Equal: *mut c_uchar,
    ) -> NTSTATUS;
    fn ZwCompleteConnectPort(
        PortHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwCompressKey(
        Key: HANDLE,
    ) -> NTSTATUS;
    fn ZwConnectPort(
        PortHandle: *mut HANDLE,
        PortName: *mut UNICODE_STRING,
        SecurityQos: *mut SECURITY_QUALITY_OF_SERVICE,
        ClientView: PPORT_VIEW,
        ServerView: PREMOTE_PORT_VIEW,
        MaxMessageLength: *mut c_ulong,
        ConnectionInformation: *mut c_void,
        ConnectionInformationLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwContinue(
        ContextRecord: *mut CONTEXT,
        TestAlert: c_uchar,
    ) -> NTSTATUS;
    fn ZwCreateDebugObject(
        DebugObjectHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        Flags: c_ulong,
    ) -> NTSTATUS;
    fn ZwCreateDirectoryObject(
        DirectoryHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwCreateDirectoryObjectEx(
        DirectoryHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        ShadowDirectoryHandle: HANDLE,
        Flags: c_ulong,
    ) -> NTSTATUS;
    fn ZwCreateEnlistment(
        EnlistmentHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ResourceManagerHandle: HANDLE,
        TransactionHandle: HANDLE,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        CreateOptions: c_ulong,
        NotificationMask: c_ulong,
        EnlistmentKey: *mut c_void,
    ) -> NTSTATUS;
    fn ZwCreateEvent(
        EventHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        EventType: EVENT_TYPE,
        InitialState: c_uchar,
    ) -> NTSTATUS;
    fn ZwCreateEventPair(
        EventPairHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwCreateFile(
        FileHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        IoStatusBlock: PIO_STATUS_BLOCK,
        AllocationSize: *mut LARGE_INTEGER,
        FileAttributes: c_ulong,
        ShareAccess: c_ulong,
        CreateDisposition: c_ulong,
        CreateOptions: c_ulong,
        EaBuffer: *mut c_void,
        EaLength: c_ulong,
    ) -> NTSTATUS;
    fn ZwCreateIRTimer(
        TimerHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
    ) -> NTSTATUS;
    fn ZwCreateIoCompletion(
        IoCompletionHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        Count: c_ulong,
    ) -> NTSTATUS;
    fn ZwCreateJobObject(
        JobHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwCreateJobSet(
        NumJob: c_ulong,
        UserJobSet: *mut JOB_SET_ARRAY,
        Flags: c_ulong,
    ) -> NTSTATUS;
    fn ZwCreateKey(
        KeyHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        TitleIndex: c_ulong,
        Class: *mut UNICODE_STRING,
        CreateOptions: c_ulong,
        Disposition: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwCreateKeyTransacted(
        KeyHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        TitleIndex: c_ulong,
        Class: *mut UNICODE_STRING,
        CreateOptions: c_ulong,
        TransactionHandle: HANDLE,
        Disposition: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwCreateKeyedEvent(
        KeyedEventHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        Flags: c_ulong,
    ) -> NTSTATUS;
    fn ZwCreateLowBoxToken(
        TokenHandle: *mut HANDLE,
        ExistingTokenHandle: HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        PackageSid: PSID,
        CapabilityCount: c_ulong,
        Capabilities: *mut SID_AND_ATTRIBUTES,
        HandleCount: c_ulong,
        Handles: *mut HANDLE,
    ) -> NTSTATUS;
    fn ZwCreateMailslotFile(
        FileHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        IoStatusBlock: PIO_STATUS_BLOCK,
        CreateOptions: c_ulong,
        MailslotQuota: c_ulong,
        MaximumMessageSize: c_ulong,
        ReadTimeout: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwCreateMutant(
        MutantHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        InitialOwner: c_uchar,
    ) -> NTSTATUS;
    fn ZwCreateNamedPipeFile(
        FileHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        IoStatusBlock: PIO_STATUS_BLOCK,
        ShareAccess: c_ulong,
        CreateDisposition: c_ulong,
        CreateOptions: c_ulong,
        NamedPipeType: c_ulong,
        ReadMode: c_ulong,
        CompletionMode: c_ulong,
        MaximumInstances: c_ulong,
        InboundQuota: c_ulong,
        OutboundQuota: c_ulong,
        DefaultTimeout: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwCreatePagingFile(
        PageFileName: *mut UNICODE_STRING,
        MinimumSize: *mut LARGE_INTEGER,
        MaximumSize: *mut LARGE_INTEGER,
        Priority: c_ulong,
    ) -> NTSTATUS;
    fn ZwCreatePartition(
        PartitionHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        PreferredNode: c_ulong,
    ) -> NTSTATUS;
    fn ZwCreatePort(
        PortHandle: *mut HANDLE,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        MaxConnectionInfoLength: c_ulong,
        MaxMessageLength: c_ulong,
        MaxPoolUsage: c_ulong,
    ) -> NTSTATUS;
    fn ZwCreatePrivateNamespace(
        NamespaceHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        BoundaryDescriptor: *mut c_void,
    ) -> NTSTATUS;
    fn ZwCreateProcess(
        ProcessHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        ParentProcess: HANDLE,
        InheritObjectTable: c_uchar,
        SectionHandle: HANDLE,
        DebugPort: HANDLE,
        ExceptionPort: HANDLE,
    ) -> NTSTATUS;
    fn ZwCreateProcessEx(
        ProcessHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        ParentProcess: HANDLE,
        Flags: c_ulong,
        SectionHandle: HANDLE,
        DebugPort: HANDLE,
        ExceptionPort: HANDLE,
        JobMemberLevel: c_ulong,
    ) -> NTSTATUS;
    fn ZwCreateProfile(
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
    fn ZwCreateProfileEx(
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
    fn ZwCreateResourceManager(
        ResourceManagerHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        TmHandle: HANDLE,
        ResourceManagerGuid: *mut GUID,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        CreateOptions: c_ulong,
        Description: *mut UNICODE_STRING,
    ) -> NTSTATUS;
    fn ZwCreateSection(
        SectionHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        MaximumSize: *mut LARGE_INTEGER,
        SectionPageProtection: c_ulong,
        AllocationAttributes: c_ulong,
        FileHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwCreateSemaphore(
        SemaphoreHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        InitialCount: c_long,
        MaximumCount: c_long,
    ) -> NTSTATUS;
    fn ZwCreateSymbolicLinkObject(
        LinkHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        LinkTarget: *mut UNICODE_STRING,
    ) -> NTSTATUS;
    fn ZwCreateThread(
        ThreadHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        ProcessHandle: HANDLE,
        ClientId: PCLIENT_ID,
        ThreadContext: *mut CONTEXT,
        InitialTeb: PINITIAL_TEB,
        CreateSuspended: c_uchar,
    ) -> NTSTATUS;
    fn ZwCreateThreadEx(
        ThreadHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        ProcessHandle: HANDLE,
        StartRoutine: *mut c_void,
        Argument: *mut c_void,
        CreateFlags: c_ulong,
        ZeroBits: usize,
        StackSize: usize,
        MaximumStackSize: usize,
        AttributeList: PPS_ATTRIBUTE_LIST,
    ) -> NTSTATUS;
    fn ZwCreateTimer(
        TimerHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        TimerType: TIMER_TYPE,
    ) -> NTSTATUS;
    fn ZwCreateTimer2(
        TimerHandle: *mut HANDLE,
        Reserved1: *mut c_void,
        Reserved2: *mut c_void,
        Attributes: c_ulong,
        DesiredAccess: c_ulong,
    ) -> NTSTATUS;
    fn ZwCreateToken(
        TokenHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        TokenType: TOKEN_TYPE,
        AuthenticationId: *mut LUID,
        ExpirationTime: *mut LARGE_INTEGER,
        User: *mut TOKEN_USER,
        Groups: *mut TOKEN_GROUPS,
        Privileges: *mut TOKEN_PRIVILEGES,
        Owner: *mut TOKEN_OWNER,
        PrimaryGroup: *mut TOKEN_PRIMARY_GROUP,
        DefaultDacl: *mut TOKEN_DEFAULT_DACL,
        TokenSource: *mut TOKEN_SOURCE,
    ) -> NTSTATUS;
    fn ZwCreateTokenEx(
        TokenHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        TokenType: TOKEN_TYPE,
        AuthenticationId: *mut LUID,
        ExpirationTime: *mut LARGE_INTEGER,
        User: *mut TOKEN_USER,
        Groups: *mut TOKEN_GROUPS,
        Privileges: *mut TOKEN_PRIVILEGES,
        UserAttributes: PTOKEN_SECURITY_ATTRIBUTES_INFORMATION,
        DeviceAttributes: PTOKEN_SECURITY_ATTRIBUTES_INFORMATION,
        DeviceGroups: *mut TOKEN_GROUPS,
        TokenMandatoryPolicy: *mut TOKEN_MANDATORY_POLICY,
        Owner: *mut TOKEN_OWNER,
        PrimaryGroup: *mut TOKEN_PRIMARY_GROUP,
        DefaultDacl: *mut TOKEN_DEFAULT_DACL,
        TokenSource: *mut TOKEN_SOURCE,
    ) -> NTSTATUS;
    fn ZwCreateTransaction(
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
    fn ZwCreateTransactionManager(
        TmHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        LogFileName: *mut UNICODE_STRING,
        CreateOptions: c_ulong,
        CommitStrength: c_ulong,
    ) -> NTSTATUS;
    fn ZwCreateUserProcess(
        ProcessHandle: *mut HANDLE,
        ThreadHandle: *mut HANDLE,
        ProcessDesiredAccess: c_ulong,
        ThreadDesiredAccess: c_ulong,
        ProcessObjectAttributes: *mut OBJECT_ATTRIBUTES,
        ThreadObjectAttributes: *mut OBJECT_ATTRIBUTES,
        ProcessFlags: c_ulong,
        ThreadFlags: c_ulong,
        ProcessParameters: *mut c_void,
        CreateInfo: PPS_CREATE_INFO,
        AttributeList: PPS_ATTRIBUTE_LIST,
    ) -> NTSTATUS;
    fn ZwCreateWaitCompletionPacket(
        WaitCompletionPacketHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwCreateWaitablePort(
        PortHandle: *mut HANDLE,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        MaxConnectionInfoLength: c_ulong,
        MaxMessageLength: c_ulong,
        MaxPoolUsage: c_ulong,
    ) -> NTSTATUS;
    fn ZwCreateWnfStateName(
        StateName: *mut WNF_STATE_NAME,
        NameLifetime: WNF_STATE_NAME_LIFETIME,
        DataScope: WNF_DATA_SCOPE,
        PersistData: c_uchar,
        TypeId: PCWNF_TYPE_ID,
        MaximumStateSize: c_ulong,
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
    ) -> NTSTATUS;
    fn ZwCreateWorkerFactory(
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
    fn ZwDebugActiveProcess(
        ProcessHandle: HANDLE,
        DebugObjectHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwDebugContinue(
        DebugObjectHandle: HANDLE,
        ClientId: PCLIENT_ID,
        ContinueStatus: NTSTATUS,
    ) -> NTSTATUS;
    fn ZwDelayExecution(
        Alertable: c_uchar,
        DelayInterval: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwDeleteAtom(
        Atom: RTL_ATOM,
    ) -> NTSTATUS;
    fn ZwDeleteBootEntry(
        Id: c_ulong,
    ) -> NTSTATUS;
    fn ZwDeleteDriverEntry(
        Id: c_ulong,
    ) -> NTSTATUS;
    fn ZwDeleteFile(
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwDeleteKey(
        KeyHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwDeleteObjectAuditAlarm(
        SubsystemName: *mut UNICODE_STRING,
        HandleId: *mut c_void,
        GenerateOnClose: c_uchar,
    ) -> NTSTATUS;
    fn ZwDeletePrivateNamespace(
        NamespaceHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwDeleteValueKey(
        KeyHandle: HANDLE,
        ValueName: *mut UNICODE_STRING,
    ) -> NTSTATUS;
    fn ZwDeleteWnfStateData(
        StateName: *const WNF_STATE_NAME,
        ExplicitScope: *const c_void,
    ) -> NTSTATUS;
    fn ZwDeleteWnfStateName(
        StateName: *const WNF_STATE_NAME,
    ) -> NTSTATUS;
    fn ZwDeviceIoControlFile(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: *mut c_void,
        IoStatusBlock: PIO_STATUS_BLOCK,
        IoControlCode: c_ulong,
        InputBuffer: *mut c_void,
        InputBufferLength: c_ulong,
        OutputBuffer: *mut c_void,
        OutputBufferLength: c_ulong,
    ) -> NTSTATUS;
    fn ZwDisableLastKnownGood() -> NTSTATUS;
    fn ZwDisplayString(
        String: *mut UNICODE_STRING,
    ) -> NTSTATUS;
    fn ZwDrawText(
        String: *mut UNICODE_STRING,
    ) -> NTSTATUS;
    fn ZwDuplicateObject(
        SourceProcessHandle: HANDLE,
        SourceHandle: HANDLE,
        TargetProcessHandle: HANDLE,
        TargetHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        HandleAttributes: c_ulong,
        Options: c_ulong,
    ) -> NTSTATUS;
    fn ZwDuplicateToken(
        ExistingTokenHandle: HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        EffectiveOnly: c_uchar,
        TokenType: TOKEN_TYPE,
        NewTokenHandle: *mut HANDLE,
    ) -> NTSTATUS;
    fn ZwEnableLastKnownGood() -> NTSTATUS;
    fn ZwEnumerateBootEntries(
        Buffer: *mut c_void,
        BufferLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwEnumerateDriverEntries(
        Buffer: *mut c_void,
        BufferLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwEnumerateKey(
        KeyHandle: HANDLE,
        Index: c_ulong,
        KeyInformationClass: KEY_INFORMATION_CLASS,
        KeyInformation: *mut c_void,
        Length: c_ulong,
        ResultLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwEnumerateSystemEnvironmentValuesEx(
        InformationClass: c_ulong,
        Buffer: *mut c_void,
        BufferLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwEnumerateTransactionObject(
        RootObjectHandle: HANDLE,
        QueryType: KTMOBJECT_TYPE,
        ObjectCursor: *mut KTMOBJECT_CURSOR,
        ObjectCursorLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwEnumerateValueKey(
        KeyHandle: HANDLE,
        Index: c_ulong,
        KeyValueInformationClass: KEY_VALUE_INFORMATION_CLASS,
        KeyValueInformation: *mut c_void,
        Length: c_ulong,
        ResultLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwExtendSection(
        SectionHandle: HANDLE,
        NewSectionSize: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwFilterToken(
        ExistingTokenHandle: HANDLE,
        Flags: c_ulong,
        SidsToDisable: *mut TOKEN_GROUPS,
        PrivilegesToDelete: *mut TOKEN_PRIVILEGES,
        RestrictedSids: *mut TOKEN_GROUPS,
        NewTokenHandle: *mut HANDLE,
    ) -> NTSTATUS;
    fn ZwFilterTokenEx(
        ExistingTokenHandle: HANDLE,
        Flags: c_ulong,
        SidsToDisable: *mut TOKEN_GROUPS,
        PrivilegesToDelete: *mut TOKEN_PRIVILEGES,
        RestrictedSids: *mut TOKEN_GROUPS,
        DisableUserClaimsCount: c_ulong,
        UserClaimsToDisable: *mut UNICODE_STRING,
        DisableDeviceClaimsCount: c_ulong,
        DeviceClaimsToDisable: *mut UNICODE_STRING,
        DeviceGroupsToDisable: *mut TOKEN_GROUPS,
        RestrictedUserAttributes: PTOKEN_SECURITY_ATTRIBUTES_INFORMATION,
        RestrictedDeviceAttributes: PTOKEN_SECURITY_ATTRIBUTES_INFORMATION,
        RestrictedDeviceGroups: *mut TOKEN_GROUPS,
        NewTokenHandle: *mut HANDLE,
    ) -> NTSTATUS;
    fn ZwFindAtom(
        AtomName: *mut wchar_t,
        Length: c_ulong,
        Atom: PRTL_ATOM,
    ) -> NTSTATUS;
    fn ZwFlushBuffersFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
    ) -> NTSTATUS;
    fn ZwFlushBuffersFileEx(
        FileHandle: HANDLE,
        Flags: c_ulong,
        Parameters: *mut c_void,
        ParametersSize: c_ulong,
        IoStatusBlock: PIO_STATUS_BLOCK,
    ) -> NTSTATUS;
    fn ZwFlushInstallUILanguage(
        InstallUILanguage: c_ushort,
        SetComittedFlag: c_ulong,
    ) -> NTSTATUS;
    fn ZwFlushInstructionCache(
        ProcessHandle: HANDLE,
        BaseAddress: *mut c_void,
        Length: usize,
    ) -> NTSTATUS;
    fn ZwFlushKey(
        KeyHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwFlushProcessWriteBuffers();
    fn ZwFlushWriteBuffer() -> NTSTATUS;
    fn ZwFreeUserPhysicalPages(
        ProcessHandle: HANDLE,
        NumberOfPages: *mut usize,
        UserPfnArray: *mut usize,
    ) -> NTSTATUS;
    fn ZwFreeVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut *mut c_void,
        RegionSize: *mut usize,
        FreeType: c_ulong,
    ) -> NTSTATUS;
    fn ZwFreezeRegistry(
        TimeOutInSeconds: c_ulong,
    ) -> NTSTATUS;
    fn ZwFreezeTransactions(
        FreezeTimeout: *mut LARGE_INTEGER,
        ThawTimeout: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwFsControlFile(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: *mut c_void,
        IoStatusBlock: PIO_STATUS_BLOCK,
        FsControlCode: c_ulong,
        InputBuffer: *mut c_void,
        InputBufferLength: c_ulong,
        OutputBuffer: *mut c_void,
        OutputBufferLength: c_ulong,
    ) -> NTSTATUS;
    fn ZwGetCachedSigningLevel(
        File: HANDLE,
        Flags: *mut c_ulong,
        SigningLevel: *mut c_uchar,
        Thumbprint: *mut c_uchar,
        ThumbprintSize: *mut c_ulong,
        ThumbprintAlgorithm: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwGetCompleteWnfStateSubscription(
        OldDescriptorStateName: *mut WNF_STATE_NAME,
        OldSubscriptionId: *mut __uint64,
        OldDescriptorEventMask: c_ulong,
        OldDescriptorStatus: c_ulong,
        NewDeliveryDescriptor: PWNF_DELIVERY_DESCRIPTOR,
        DescriptorSize: c_ulong,
    ) -> NTSTATUS;
    fn ZwGetContextThread(
        ThreadHandle: HANDLE,
        ThreadContext: *mut CONTEXT,
    ) -> NTSTATUS;
    fn ZwGetCurrentProcessorNumber() -> c_ulong;
    fn ZwGetDevicePowerState(
        Device: HANDLE,
        State: *mut DEVICE_POWER_STATE,
    ) -> NTSTATUS;
    fn ZwGetMUIRegistryInfo(
        Flags: c_ulong,
        DataSize: *mut c_ulong,
        Data: *mut c_void,
    ) -> NTSTATUS;
    fn ZwGetNextProcess(
        ProcessHandle: HANDLE,
        DesiredAccess: c_ulong,
        HandleAttributes: c_ulong,
        Flags: c_ulong,
        NewProcessHandle: *mut HANDLE,
    ) -> NTSTATUS;
    fn ZwGetNextThread(
        ProcessHandle: HANDLE,
        ThreadHandle: HANDLE,
        DesiredAccess: c_ulong,
        HandleAttributes: c_ulong,
        Flags: c_ulong,
        NewThreadHandle: *mut HANDLE,
    ) -> NTSTATUS;
    fn ZwGetNlsSectionPtr(
        SectionType: c_ulong,
        SectionData: c_ulong,
        ContextData: *mut c_void,
        SectionPointer: *mut *mut c_void,
        SectionSize: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwGetNotificationResourceManager(
        ResourceManagerHandle: HANDLE,
        TransactionNotification: *mut TRANSACTION_NOTIFICATION,
        NotificationLength: c_ulong,
        Timeout: *mut LARGE_INTEGER,
        ReturnLength: *mut c_ulong,
        Asynchronous: c_ulong,
        AsynchronousContext: usize,
    ) -> NTSTATUS;
    fn ZwGetPlugPlayEvent(
        EventHandle: HANDLE,
        Context: *mut c_void,
        EventBlock: PPLUGPLAY_EVENT_BLOCK,
        EventBufferSize: c_ulong,
    ) -> NTSTATUS;
    fn ZwGetWriteWatch(
        ProcessHandle: HANDLE,
        Flags: c_ulong,
        BaseAddress: *mut c_void,
        RegionSize: usize,
        UserAddressArray: *mut *mut c_void,
        EntriesInUserAddressArray: *mut usize,
        Granularity: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwImpersonateAnonymousToken(
        ThreadHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwImpersonateClientOfPort(
        PortHandle: HANDLE,
        Message: PPORT_MESSAGE,
    ) -> NTSTATUS;
    fn ZwImpersonateThread(
        ServerThreadHandle: HANDLE,
        ClientThreadHandle: HANDLE,
        SecurityQos: *mut SECURITY_QUALITY_OF_SERVICE,
    ) -> NTSTATUS;
    fn ZwInitializeNlsFiles(
        BaseAddress: *mut *mut c_void,
        DefaultLocaleId: *mut c_ulong,
        DefaultCasingTableSize: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwInitializeRegistry(
        BootCondition: c_ushort,
    ) -> NTSTATUS;
    fn ZwInitiatePowerAction(
        SystemAction: POWER_ACTION,
        LightestSystemState: SYSTEM_POWER_STATE,
        Flags: c_ulong,
        Asynchronous: c_uchar,
    ) -> NTSTATUS;
    fn ZwIsProcessInJob(
        ProcessHandle: HANDLE,
        JobHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwIsSystemResumeAutomatic() -> c_uchar;
    fn ZwIsUILanguageComitted() -> NTSTATUS;
    fn ZwListenPort(
        PortHandle: HANDLE,
        ConnectionRequest: PPORT_MESSAGE,
    ) -> NTSTATUS;
    fn ZwLoadDriver(
        DriverServiceName: *mut UNICODE_STRING,
    ) -> NTSTATUS;
    fn ZwLoadKey(
        TargetKey: *mut OBJECT_ATTRIBUTES,
        SourceFile: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwLoadKey2(
        TargetKey: *mut OBJECT_ATTRIBUTES,
        SourceFile: *mut OBJECT_ATTRIBUTES,
        Flags: c_ulong,
    ) -> NTSTATUS;
    fn ZwLoadKeyEx(
        TargetKey: *mut OBJECT_ATTRIBUTES,
        SourceFile: *mut OBJECT_ATTRIBUTES,
        Flags: c_ulong,
        TrustClassKey: HANDLE,
        Event: HANDLE,
        DesiredAccess: c_ulong,
        RootHandle: *mut HANDLE,
        IoStatus: PIO_STATUS_BLOCK,
    ) -> NTSTATUS;
    fn ZwLockFile(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: *mut c_void,
        IoStatusBlock: PIO_STATUS_BLOCK,
        ByteOffset: *mut LARGE_INTEGER,
        Length: *mut LARGE_INTEGER,
        Key: c_ulong,
        FailImmediately: c_uchar,
        ExclusiveLock: c_uchar,
    ) -> NTSTATUS;
    fn ZwLockProductActivationKeys(
        pPrivateVer: *mut c_ulong,
        pSafeMode: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwLockRegistryKey(
        KeyHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwLockVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut *mut c_void,
        RegionSize: *mut usize,
        MapType: c_ulong,
    ) -> NTSTATUS;
    fn ZwMakePermanentObject(
        Handle: HANDLE,
    ) -> NTSTATUS;
    fn ZwMakeTemporaryObject(
        Handle: HANDLE,
    ) -> NTSTATUS;
    fn ZwManagePartition(
        PartitionInformationClass: MEMORY_PARTITION_INFORMATION_CLASS,
        PartitionInformation: *mut c_void,
        PartitionInformationLength: c_ulong,
    ) -> NTSTATUS;
    fn ZwMapCMFModule(
        What: c_ulong,
        Index: c_ulong,
        CacheIndexOut: *mut c_ulong,
        CacheFlagsOut: *mut c_ulong,
        ViewSizeOut: *mut c_ulong,
        BaseAddress: *mut *mut c_void,
    ) -> NTSTATUS;
    fn ZwMapUserPhysicalPages(
        VirtualAddress: *mut c_void,
        NumberOfPages: usize,
        UserPfnArray: *mut usize,
    ) -> NTSTATUS;
    fn ZwMapUserPhysicalPagesScatter(
        VirtualAddresses: *mut *mut c_void,
        NumberOfPages: usize,
        UserPfnArray: *mut usize,
    ) -> NTSTATUS;
    fn ZwMapViewOfSection(
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
    fn ZwModifyBootEntry(
        BootEntry: PBOOT_ENTRY,
    ) -> NTSTATUS;
    fn ZwModifyDriverEntry(
        DriverEntry: PEFI_DRIVER_ENTRY,
    ) -> NTSTATUS;
    fn ZwNotifyChangeDirectoryFile(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: *mut c_void,
        IoStatusBlock: PIO_STATUS_BLOCK,
        Buffer: *mut c_void,
        Length: c_ulong,
        CompletionFilter: c_ulong,
        WatchTree: c_uchar,
    ) -> NTSTATUS;
    fn ZwNotifyChangeKey(
        KeyHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: *mut c_void,
        IoStatusBlock: PIO_STATUS_BLOCK,
        CompletionFilter: c_ulong,
        WatchTree: c_uchar,
        Buffer: *mut c_void,
        BufferSize: c_ulong,
        Asynchronous: c_uchar,
    ) -> NTSTATUS;
    fn ZwNotifyChangeMultipleKeys(
        MasterKeyHandle: HANDLE,
        Count: c_ulong,
        SubordinateObjects: *mut OBJECT_ATTRIBUTES,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: *mut c_void,
        IoStatusBlock: PIO_STATUS_BLOCK,
        CompletionFilter: c_ulong,
        WatchTree: c_uchar,
        Buffer: *mut c_void,
        BufferSize: c_ulong,
        Asynchronous: c_uchar,
    ) -> NTSTATUS;
    fn ZwNotifyChangeSession(
        SessionHandle: HANDLE,
        ChangeSequenceNumber: c_ulong,
        ChangeTimeStamp: *mut LARGE_INTEGER,
        Event: IO_SESSION_EVENT,
        NewState: IO_SESSION_STATE,
        PreviousState: IO_SESSION_STATE,
        Payload: *mut c_void,
        PayloadSize: c_ulong,
    ) -> NTSTATUS;
    fn ZwOpenDirectoryObject(
        DirectoryHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwOpenEnlistment(
        EnlistmentHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        RmHandle: HANDLE,
        EnlistmentGuid: *mut GUID,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwOpenEvent(
        EventHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwOpenEventPair(
        EventPairHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwOpenFile(
        FileHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        IoStatusBlock: PIO_STATUS_BLOCK,
        ShareAccess: c_ulong,
        OpenOptions: c_ulong,
    ) -> NTSTATUS;
    fn ZwOpenIoCompletion(
        IoCompletionHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwOpenJobObject(
        JobHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwOpenKey(
        KeyHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwOpenKeyEx(
        KeyHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        OpenOptions: c_ulong,
    ) -> NTSTATUS;
    fn ZwOpenKeyTransacted(
        KeyHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        TransactionHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwOpenKeyTransactedEx(
        KeyHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        OpenOptions: c_ulong,
        TransactionHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwOpenKeyedEvent(
        KeyedEventHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwOpenMutant(
        MutantHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwOpenObjectAuditAlarm(
        SubsystemName: *mut UNICODE_STRING,
        HandleId: *mut c_void,
        ObjectTypeName: *mut UNICODE_STRING,
        ObjectName: *mut UNICODE_STRING,
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        ClientToken: HANDLE,
        DesiredAccess: c_ulong,
        GrantedAccess: c_ulong,
        Privileges: *mut PRIVILEGE_SET,
        ObjectCreation: c_uchar,
        AccessGranted: c_uchar,
        GenerateOnClose: *mut c_uchar,
    ) -> NTSTATUS;
    fn ZwOpenPartition(
        PartitionHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwOpenPrivateNamespace(
        NamespaceHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        BoundaryDescriptor: *mut c_void,
    ) -> NTSTATUS;
    fn ZwOpenProcess(
        ProcessHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        ClientId: PCLIENT_ID,
    ) -> NTSTATUS;
    fn ZwOpenProcessToken(
        ProcessHandle: HANDLE,
        DesiredAccess: c_ulong,
        TokenHandle: *mut HANDLE,
    ) -> NTSTATUS;
    fn ZwOpenProcessTokenEx(
        ProcessHandle: HANDLE,
        DesiredAccess: c_ulong,
        HandleAttributes: c_ulong,
        TokenHandle: *mut HANDLE,
    ) -> NTSTATUS;
    fn ZwOpenResourceManager(
        ResourceManagerHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        TmHandle: HANDLE,
        ResourceManagerGuid: *mut GUID,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwOpenSection(
        SectionHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwOpenSemaphore(
        SemaphoreHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwOpenSession(
        SessionHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwOpenSymbolicLinkObject(
        LinkHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwOpenThread(
        ThreadHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        ClientId: PCLIENT_ID,
    ) -> NTSTATUS;
    fn ZwOpenThreadToken(
        ThreadHandle: HANDLE,
        DesiredAccess: c_ulong,
        OpenAsSelf: c_uchar,
        TokenHandle: *mut HANDLE,
    ) -> NTSTATUS;
    fn ZwOpenThreadTokenEx(
        ThreadHandle: HANDLE,
        DesiredAccess: c_ulong,
        OpenAsSelf: c_uchar,
        HandleAttributes: c_ulong,
        TokenHandle: *mut HANDLE,
    ) -> NTSTATUS;
    fn ZwOpenTimer(
        TimerHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwOpenTransaction(
        TransactionHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        Uow: *mut GUID,
        TmHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwOpenTransactionManager(
        TmHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        LogFileName: *mut UNICODE_STRING,
        TmIdentity: *mut GUID,
        OpenOptions: c_ulong,
    ) -> NTSTATUS;
    fn ZwPlugPlayControl(
        PnPControlClass: PLUGPLAY_CONTROL_CLASS,
        PnPControlData: *mut c_void,
        PnPControlDataLength: c_ulong,
    ) -> NTSTATUS;
    fn ZwPowerInformation(
        InformationLevel: POWER_INFORMATION_LEVEL,
        InputBuffer: *mut c_void,
        InputBufferLength: c_ulong,
        OutputBuffer: *mut c_void,
        OutputBufferLength: c_ulong,
    ) -> NTSTATUS;
    fn ZwPrePrepareComplete(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwPrePrepareEnlistment(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwPrepareComplete(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwPrepareEnlistment(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwPrivilegeCheck(
        ClientToken: HANDLE,
        RequiredPrivileges: *mut PRIVILEGE_SET,
        Result: *mut c_uchar,
    ) -> NTSTATUS;
    fn ZwPrivilegeObjectAuditAlarm(
        SubsystemName: *mut UNICODE_STRING,
        HandleId: *mut c_void,
        ClientToken: HANDLE,
        DesiredAccess: c_ulong,
        Privileges: *mut PRIVILEGE_SET,
        AccessGranted: c_uchar,
    ) -> NTSTATUS;
    fn ZwPrivilegedServiceAuditAlarm(
        SubsystemName: *mut UNICODE_STRING,
        ServiceName: *mut UNICODE_STRING,
        ClientToken: HANDLE,
        Privileges: *mut PRIVILEGE_SET,
        AccessGranted: c_uchar,
    ) -> NTSTATUS;
    fn ZwPropagationComplete(
        ResourceManagerHandle: HANDLE,
        RequestCookie: c_ulong,
        BufferLength: c_ulong,
        Buffer: *mut c_void,
    ) -> NTSTATUS;
    fn ZwPropagationFailed(
        ResourceManagerHandle: HANDLE,
        RequestCookie: c_ulong,
        PropStatus: NTSTATUS,
    ) -> NTSTATUS;
    fn ZwProtectVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut *mut c_void,
        RegionSize: *mut usize,
        NewProtect: c_ulong,
        OldProtect: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwPulseEvent(
        EventHandle: HANDLE,
        PreviousState: *mut c_long,
    ) -> NTSTATUS;
    fn ZwQueryAttributesFile(
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        FileInformation: PFILE_BASIC_INFORMATION,
    ) -> NTSTATUS;
    fn ZwQueryBootEntryOrder(
        Ids: *mut c_ulong,
        Count: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwQueryBootOptions(
        BootOptions: PBOOT_OPTIONS,
        BootOptionsLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwQueryDebugFilterState(
        ComponentId: c_ulong,
        Level: c_ulong,
    ) -> NTSTATUS;
    fn ZwQueryDefaultLocale(
        UserProfile: c_uchar,
        DefaultLocaleId: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwQueryDefaultUILanguage(
        DefaultUILanguageId: *mut c_ushort,
    ) -> NTSTATUS;
    fn ZwQueryDirectoryFile(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: *mut c_void,
        IoStatusBlock: PIO_STATUS_BLOCK,
        FileInformation: *mut c_void,
        Length: c_ulong,
        FileInformationClass: FILE_INFORMATION_CLASS,
        ReturnSingleEntry: c_uchar,
        FileName: *mut UNICODE_STRING,
        RestartScan: c_uchar,
    ) -> NTSTATUS;
    fn ZwQueryDirectoryObject(
        DirectoryHandle: HANDLE,
        Buffer: *mut c_void,
        Length: c_ulong,
        ReturnSingleEntry: c_uchar,
        RestartScan: c_uchar,
        Context: *mut c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwQueryDriverEntryOrder(
        Ids: *mut c_ulong,
        Count: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwQueryEaFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
        Buffer: *mut c_void,
        Length: c_ulong,
        ReturnSingleEntry: c_uchar,
        EaList: *mut c_void,
        EaListLength: c_ulong,
        EaIndex: *mut c_ulong,
        RestartScan: c_uchar,
    ) -> NTSTATUS;
    fn ZwQueryEvent(
        EventHandle: HANDLE,
        EventInformationClass: EVENT_INFORMATION_CLASS,
        EventInformation: *mut c_void,
        EventInformationLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwQueryFullAttributesFile(
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        FileInformation: PFILE_NETWORK_OPEN_INFORMATION,
    ) -> NTSTATUS;
    fn ZwQueryInformationAtom(
        Atom: RTL_ATOM,
        AtomInformationClass: ATOM_INFORMATION_CLASS,
        AtomInformation: *mut c_void,
        AtomInformationLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwQueryInformationEnlistment(
        EnlistmentHandle: HANDLE,
        EnlistmentInformationClass: ENLISTMENT_INFORMATION_CLASS,
        EnlistmentInformation: *mut c_void,
        EnlistmentInformationLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwQueryInformationFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
        FileInformation: *mut c_void,
        Length: c_ulong,
        FileInformationClass: FILE_INFORMATION_CLASS,
    ) -> NTSTATUS;
    fn ZwQueryInformationJobObject(
        JobHandle: HANDLE,
        JobObjectInformationClass: JOBOBJECTINFOCLASS,
        JobObjectInformation: *mut c_void,
        JobObjectInformationLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwQueryInformationPort(
        PortHandle: HANDLE,
        PortInformationClass: PORT_INFORMATION_CLASS,
        PortInformation: *mut c_void,
        Length: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwQueryInformationProcess(
        ProcessHandle: HANDLE,
        ProcessInformationClass: PROCESSINFOCLASS,
        ProcessInformation: *mut c_void,
        ProcessInformationLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwQueryInformationResourceManager(
        ResourceManagerHandle: HANDLE,
        ResourceManagerInformationClass: RESOURCEMANAGER_INFORMATION_CLASS,
        ResourceManagerInformation: *mut c_void,
        ResourceManagerInformationLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwQueryInformationThread(
        ThreadHandle: HANDLE,
        ThreadInformationClass: THREADINFOCLASS,
        ThreadInformation: *mut c_void,
        ThreadInformationLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwQueryInformationToken(
        TokenHandle: HANDLE,
        TokenInformationClass: TOKEN_INFORMATION_CLASS,
        TokenInformation: *mut c_void,
        TokenInformationLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwQueryInformationTransaction(
        TransactionHandle: HANDLE,
        TransactionInformationClass: TRANSACTION_INFORMATION_CLASS,
        TransactionInformation: *mut c_void,
        TransactionInformationLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwQueryInformationTransactionManager(
        TransactionManagerHandle: HANDLE,
        TransactionManagerInformationClass: TRANSACTIONMANAGER_INFORMATION_CLASS,
        TransactionManagerInformation: *mut c_void,
        TransactionManagerInformationLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwQueryInformationWorkerFactory(
        WorkerFactoryHandle: HANDLE,
        WorkerFactoryInformationClass: WORKERFACTORYINFOCLASS,
        WorkerFactoryInformation: *mut c_void,
        WorkerFactoryInformationLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwQueryInstallUILanguage(
        InstallUILanguageId: *mut c_ushort,
    ) -> NTSTATUS;
    fn ZwQueryIntervalProfile(
        ProfileSource: KPROFILE_SOURCE,
        Interval: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwQueryIoCompletion(
        IoCompletionHandle: HANDLE,
        IoCompletionInformationClass: IO_COMPLETION_INFORMATION_CLASS,
        IoCompletionInformation: *mut c_void,
        IoCompletionInformationLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwQueryKey(
        KeyHandle: HANDLE,
        KeyInformationClass: KEY_INFORMATION_CLASS,
        KeyInformation: *mut c_void,
        Length: c_ulong,
        ResultLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwQueryLicenseValue(
        ValueName: *mut UNICODE_STRING,
        Type: *mut c_ulong,
        Data: *mut c_void,
        DataSize: c_ulong,
        ResultDataSize: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwQueryMultipleValueKey(
        KeyHandle: HANDLE,
        ValueEntries: PKEY_VALUE_ENTRY,
        EntryCount: c_ulong,
        ValueBuffer: *mut c_void,
        BufferLength: *mut c_ulong,
        RequiredBufferLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwQueryMutant(
        MutantHandle: HANDLE,
        MutantInformationClass: MUTANT_INFORMATION_CLASS,
        MutantInformation: *mut c_void,
        MutantInformationLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwQueryObject(
        Handle: HANDLE,
        ObjectInformationClass: OBJECT_INFORMATION_CLASS,
        ObjectInformation: *mut c_void,
        ObjectInformationLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwQueryOpenSubKeys(
        TargetKey: *mut OBJECT_ATTRIBUTES,
        HandleCount: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwQueryOpenSubKeysEx(
        TargetKey: *mut OBJECT_ATTRIBUTES,
        BufferLength: c_ulong,
        Buffer: *mut c_void,
        RequiredSize: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwQueryPerformanceCounter(
        PerformanceCounter: *mut LARGE_INTEGER,
        PerformanceFrequency: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwQueryPortInformationProcess() -> NTSTATUS;
    fn ZwQueryQuotaInformationFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
        Buffer: *mut c_void,
        Length: c_ulong,
        ReturnSingleEntry: c_uchar,
        SidList: *mut c_void,
        SidListLength: c_ulong,
        StartSid: PSID,
        RestartScan: c_uchar,
    ) -> NTSTATUS;
    fn ZwQuerySection(
        SectionHandle: HANDLE,
        SectionInformationClass: SECTION_INFORMATION_CLASS,
        SectionInformation: *mut c_void,
        SectionInformationLength: usize,
        ReturnLength: *mut usize,
    ) -> NTSTATUS;
    fn ZwQuerySecurityAttributesToken(
        TokenHandle: HANDLE,
        Attributes: *mut UNICODE_STRING,
        NumberOfAttributes: c_ulong,
        Buffer: *mut c_void,
        Length: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwQuerySecurityObject(
        Handle: HANDLE,
        SecurityInformation: c_ulong,
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        Length: c_ulong,
        LengthNeeded: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwQuerySemaphore(
        SemaphoreHandle: HANDLE,
        SemaphoreInformationClass: SEMAPHORE_INFORMATION_CLASS,
        SemaphoreInformation: *mut c_void,
        SemaphoreInformationLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwQuerySymbolicLinkObject(
        LinkHandle: HANDLE,
        LinkTarget: *mut UNICODE_STRING,
        ReturnedLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwQuerySystemEnvironmentValue(
        VariableName: *mut UNICODE_STRING,
        VariableValue: *mut wchar_t,
        ValueLength: c_ushort,
        ReturnLength: *mut c_ushort,
    ) -> NTSTATUS;
    fn ZwQuerySystemEnvironmentValueEx(
        VariableName: *mut UNICODE_STRING,
        VendorGuid: *mut GUID,
        Value: *mut c_void,
        ValueLength: *mut c_ulong,
        Attributes: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwQuerySystemInformation(
        SystemInformationClass: SYSTEM_INFORMATION_CLASS,
        SystemInformation: *mut c_void,
        SystemInformationLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwQuerySystemInformationEx(
        SystemInformationClass: SYSTEM_INFORMATION_CLASS,
        InputBuffer: *mut c_void,
        InputBufferLength: c_ulong,
        SystemInformation: *mut c_void,
        SystemInformationLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwQuerySystemTime(
        SystemTime: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwQueryTimer(
        TimerHandle: HANDLE,
        TimerInformationClass: TIMER_INFORMATION_CLASS,
        TimerInformation: *mut c_void,
        TimerInformationLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwQueryTimerResolution(
        MaximumTime: *mut c_ulong,
        MinimumTime: *mut c_ulong,
        CurrentTime: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwQueryValueKey(
        KeyHandle: HANDLE,
        ValueName: *mut UNICODE_STRING,
        KeyValueInformationClass: KEY_VALUE_INFORMATION_CLASS,
        KeyValueInformation: *mut c_void,
        Length: c_ulong,
        ResultLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwQueryVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut c_void,
        MemoryInformationClass: MEMORY_INFORMATION_CLASS,
        MemoryInformation: *mut c_void,
        MemoryInformationLength: usize,
        ReturnLength: *mut usize,
    ) -> NTSTATUS;
    fn ZwQueryVolumeInformationFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
        FsInformation: *mut c_void,
        Length: c_ulong,
        FsInformationClass: FS_INFORMATION_CLASS,
    ) -> NTSTATUS;
    fn ZwQueryWnfStateData(
        StateName: *const WNF_STATE_NAME,
        TypeId: PCWNF_TYPE_ID,
        ExplicitScope: *const c_void,
        ChangeStamp: PWNF_CHANGE_STAMP,
        Buffer: *mut c_void,
        BufferSize: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwQueryWnfStateNameInformation(
        StateName: *const WNF_STATE_NAME,
        NameInfoClass: WNF_STATE_NAME_INFORMATION,
        ExplicitScope: *const c_void,
        InfoBuffer: *mut c_void,
        InfoBufferSize: c_ulong,
    ) -> NTSTATUS;
    fn ZwQueueApcThread(
        ThreadHandle: HANDLE,
        ApcRoutine: PPS_APC_ROUTINE,
        ApcArgument1: *mut c_void,
        ApcArgument2: *mut c_void,
        ApcArgument3: *mut c_void,
    ) -> NTSTATUS;
    fn ZwQueueApcThreadEx(
        ThreadHandle: HANDLE,
        UserApcReserveHandle: HANDLE,
        ApcRoutine: PPS_APC_ROUTINE,
        ApcArgument1: *mut c_void,
        ApcArgument2: *mut c_void,
        ApcArgument3: *mut c_void,
    ) -> NTSTATUS;
    fn ZwRaiseException(
        ExceptionRecord: *mut EXCEPTION_RECORD,
        ContextRecord: *mut CONTEXT,
        FirstChance: c_uchar,
    ) -> NTSTATUS;
    fn ZwRaiseHardError(
        ErrorStatus: NTSTATUS,
        NumberOfParameters: c_ulong,
        UnicodeStringParameterMask: c_ulong,
        Parameters: *mut usize,
        ValidResponseOptions: c_ulong,
        Response: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwReadFile(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: *mut c_void,
        IoStatusBlock: PIO_STATUS_BLOCK,
        Buffer: *mut c_void,
        Length: c_ulong,
        ByteOffset: *mut LARGE_INTEGER,
        Key: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwReadFileScatter(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: *mut c_void,
        IoStatusBlock: PIO_STATUS_BLOCK,
        SegmentArray: *mut FILE_SEGMENT_ELEMENT,
        Length: c_ulong,
        ByteOffset: *mut LARGE_INTEGER,
        Key: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwReadOnlyEnlistment(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwReadRequestData(
        PortHandle: HANDLE,
        Message: PPORT_MESSAGE,
        DataEntryIndex: c_ulong,
        Buffer: *mut c_void,
        BufferSize: usize,
        NumberOfBytesRead: *mut usize,
    ) -> NTSTATUS;
    fn ZwReadVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut c_void,
        Buffer: *mut c_void,
        BufferSize: usize,
        NumberOfBytesRead: *mut usize,
    ) -> NTSTATUS;
    fn ZwRecoverEnlistment(
        EnlistmentHandle: HANDLE,
        EnlistmentKey: *mut c_void,
    ) -> NTSTATUS;
    fn ZwRecoverResourceManager(
        ResourceManagerHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwRecoverTransactionManager(
        TransactionManagerHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwRegisterProtocolAddressInformation(
        ResourceManager: HANDLE,
        ProtocolId: *mut GUID,
        ProtocolInformationSize: c_ulong,
        ProtocolInformation: *mut c_void,
        CreateOptions: c_ulong,
    ) -> NTSTATUS;
    fn ZwRegisterThreadTerminatePort(
        PortHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwReleaseCMFViewOwnership() -> NTSTATUS;
    fn ZwReleaseKeyedEvent(
        KeyedEventHandle: HANDLE,
        KeyValue: *mut c_void,
        Alertable: c_uchar,
        Timeout: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwReleaseMutant(
        MutantHandle: HANDLE,
        PreviousCount: *mut c_long,
    ) -> NTSTATUS;
    fn ZwReleaseSemaphore(
        SemaphoreHandle: HANDLE,
        ReleaseCount: c_long,
        PreviousCount: *mut c_long,
    ) -> NTSTATUS;
    fn ZwReleaseWorkerFactoryWorker(
        WorkerFactoryHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwRemoveIoCompletion(
        IoCompletionHandle: HANDLE,
        KeyContext: *mut *mut c_void,
        ApcContext: *mut *mut c_void,
        IoStatusBlock: PIO_STATUS_BLOCK,
        Timeout: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwRemoveIoCompletionEx(
        IoCompletionHandle: HANDLE,
        IoCompletionInformation: PFILE_IO_COMPLETION_INFORMATION,
        Count: c_ulong,
        NumEntriesRemoved: *mut c_ulong,
        Timeout: *mut LARGE_INTEGER,
        Alertable: c_uchar,
    ) -> NTSTATUS;
    fn ZwRemoveProcessDebug(
        ProcessHandle: HANDLE,
        DebugObjectHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwRenameKey(
        KeyHandle: HANDLE,
        NewName: *mut UNICODE_STRING,
    ) -> NTSTATUS;
    fn ZwRenameTransactionManager(
        LogFileName: *mut UNICODE_STRING,
        ExistingTransactionManagerGuid: *mut GUID,
    ) -> NTSTATUS;
    fn ZwReplaceKey(
        NewFile: *mut OBJECT_ATTRIBUTES,
        TargetHandle: HANDLE,
        OldFile: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwReplacePartitionUnit(
        TargetInstancePath: *mut UNICODE_STRING,
        SpareInstancePath: *mut UNICODE_STRING,
        Flags: c_ulong,
    ) -> NTSTATUS;
    fn ZwReplyPort(
        PortHandle: HANDLE,
        ReplyMessage: PPORT_MESSAGE,
    ) -> NTSTATUS;
    fn ZwReplyWaitReceivePort(
        PortHandle: HANDLE,
        PortContext: *mut *mut c_void,
        ReplyMessage: PPORT_MESSAGE,
        ReceiveMessage: PPORT_MESSAGE,
    ) -> NTSTATUS;
    fn ZwReplyWaitReceivePortEx(
        PortHandle: HANDLE,
        PortContext: *mut *mut c_void,
        ReplyMessage: PPORT_MESSAGE,
        ReceiveMessage: PPORT_MESSAGE,
        Timeout: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwReplyWaitReplyPort(
        PortHandle: HANDLE,
        ReplyMessage: PPORT_MESSAGE,
    ) -> NTSTATUS;
    fn ZwRequestPort(
        PortHandle: HANDLE,
        RequestMessage: PPORT_MESSAGE,
    ) -> NTSTATUS;
    fn ZwRequestWaitReplyPort(
        PortHandle: HANDLE,
        RequestMessage: PPORT_MESSAGE,
        ReplyMessage: PPORT_MESSAGE,
    ) -> NTSTATUS;
    fn ZwRequestWakeupLatency(
        latency: LATENCY_TIME,
    ) -> NTSTATUS;
    fn ZwResetEvent(
        EventHandle: HANDLE,
        PreviousState: *mut c_long,
    ) -> NTSTATUS;
    fn ZwResetWriteWatch(
        ProcessHandle: HANDLE,
        BaseAddress: *mut c_void,
        RegionSize: usize,
    ) -> NTSTATUS;
    fn ZwRestoreKey(
        KeyHandle: HANDLE,
        FileHandle: HANDLE,
        Flags: c_ulong,
    ) -> NTSTATUS;
    fn ZwResumeProcess(
        ProcessHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwResumeThread(
        ThreadHandle: HANDLE,
        PreviousSuspendCount: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwRevertContainerImpersonation() -> NTSTATUS;
    fn ZwRollbackComplete(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwRollbackEnlistment(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwRollbackTransaction(
        TransactionHandle: HANDLE,
        Wait: c_uchar,
    ) -> NTSTATUS;
    fn ZwRollforwardTransactionManager(
        TransactionManagerHandle: HANDLE,
        TmVirtualClock: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwSaveKey(
        KeyHandle: HANDLE,
        FileHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwSaveKeyEx(
        KeyHandle: HANDLE,
        FileHandle: HANDLE,
        Format: c_ulong,
    ) -> NTSTATUS;
    fn ZwSaveMergedKeys(
        HighPrecedenceKeyHandle: HANDLE,
        LowPrecedenceKeyHandle: HANDLE,
        FileHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwSecureConnectPort(
        PortHandle: *mut HANDLE,
        PortName: *mut UNICODE_STRING,
        SecurityQos: *mut SECURITY_QUALITY_OF_SERVICE,
        ClientView: PPORT_VIEW,
        RequiredServerSid: PSID,
        ServerView: PREMOTE_PORT_VIEW,
        MaxMessageLength: *mut c_ulong,
        ConnectionInformation: *mut c_void,
        ConnectionInformationLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwSerializeBoot() -> NTSTATUS;
    fn ZwSetBootEntryOrder(
        Ids: *mut c_ulong,
        Count: c_ulong,
    ) -> NTSTATUS;
    fn ZwSetBootOptions(
        BootOptions: PBOOT_OPTIONS,
        FieldsToChange: c_ulong,
    ) -> NTSTATUS;
    fn ZwSetCachedSigningLevel(
        Flags: c_ulong,
        InputSigningLevel: c_uchar,
        SourceFiles: *mut HANDLE,
        SourceFileCount: c_ulong,
        TargetFile: HANDLE,
    ) -> NTSTATUS;
    fn ZwSetContextThread(
        ThreadHandle: HANDLE,
        ThreadContext: *mut CONTEXT,
    ) -> NTSTATUS;
    fn ZwSetDebugFilterState(
        ComponentId: c_ulong,
        Level: c_ulong,
        State: c_uchar,
    ) -> NTSTATUS;
    fn ZwSetDefaultHardErrorPort(
        DefaultHardErrorPort: HANDLE,
    ) -> NTSTATUS;
    fn ZwSetDefaultLocale(
        UserProfile: c_uchar,
        DefaultLocaleId: c_ulong,
    ) -> NTSTATUS;
    fn ZwSetDefaultUILanguage(
        DefaultUILanguageId: c_ushort,
    ) -> NTSTATUS;
    fn ZwSetDriverEntryOrder(
        Ids: *mut c_ulong,
        Count: c_ulong,
    ) -> NTSTATUS;
    fn ZwSetEaFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
        Buffer: *mut c_void,
        Length: c_ulong,
    ) -> NTSTATUS;
    fn ZwSetEvent(
        EventHandle: HANDLE,
        PreviousState: *mut c_long,
    ) -> NTSTATUS;
    fn ZwSetEventBoostPriority(
        EventHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwSetHighEventPair(
        EventPairHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwSetHighWaitLowEventPair(
        EventPairHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwSetIRTimer(
        TimerHandle: HANDLE,
        DueTime: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwSetInformationDebugObject(
        DebugObjectHandle: HANDLE,
        DebugObjectInformationClass: DEBUGOBJECTINFOCLASS,
        DebugInformation: *mut c_void,
        DebugInformationLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwSetInformationEnlistment(
        EnlistmentHandle: HANDLE,
        EnlistmentInformationClass: ENLISTMENT_INFORMATION_CLASS,
        EnlistmentInformation: *mut c_void,
        EnlistmentInformationLength: c_ulong,
    ) -> NTSTATUS;
    fn ZwSetInformationFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
        FileInformation: *mut c_void,
        Length: c_ulong,
        FileInformationClass: FILE_INFORMATION_CLASS,
    ) -> NTSTATUS;
    fn ZwSetInformationJobObject(
        JobHandle: HANDLE,
        JobObjectInformationClass: JOBOBJECTINFOCLASS,
        JobObjectInformation: *mut c_void,
        JobObjectInformationLength: c_ulong,
    ) -> NTSTATUS;
    fn ZwSetInformationKey(
        KeyHandle: HANDLE,
        KeySetInformationClass: KEY_SET_INFORMATION_CLASS,
        KeySetInformation: *mut c_void,
        KeySetInformationLength: c_ulong,
    ) -> NTSTATUS;
    fn ZwSetInformationObject(
        Handle: HANDLE,
        ObjectInformationClass: OBJECT_INFORMATION_CLASS,
        ObjectInformation: *mut c_void,
        ObjectInformationLength: c_ulong,
    ) -> NTSTATUS;
    fn ZwSetInformationProcess(
        ProcessHandle: HANDLE,
        ProcessInformationClass: PROCESSINFOCLASS,
        ProcessInformation: *mut c_void,
        ProcessInformationLength: c_ulong,
    ) -> NTSTATUS;
    fn ZwSetInformationResourceManager(
        ResourceManagerHandle: HANDLE,
        ResourceManagerInformationClass: RESOURCEMANAGER_INFORMATION_CLASS,
        ResourceManagerInformation: *mut c_void,
        ResourceManagerInformationLength: c_ulong,
    ) -> NTSTATUS;
    fn ZwSetInformationThread(
        ThreadHandle: HANDLE,
        ThreadInformationClass: THREADINFOCLASS,
        ThreadInformation: *mut c_void,
        ThreadInformationLength: c_ulong,
    ) -> NTSTATUS;
    fn ZwSetInformationToken(
        TokenHandle: HANDLE,
        TokenInformationClass: TOKEN_INFORMATION_CLASS,
        TokenInformation: *mut c_void,
        TokenInformationLength: c_ulong,
    ) -> NTSTATUS;
    fn ZwSetInformationTransaction(
        TransactionHandle: HANDLE,
        TransactionInformationClass: TRANSACTION_INFORMATION_CLASS,
        TransactionInformation: *mut c_void,
        TransactionInformationLength: c_ulong,
    ) -> NTSTATUS;
    fn ZwSetInformationTransactionManager(
        TmHandle: HANDLE,
        TransactionManagerInformationClass: TRANSACTIONMANAGER_INFORMATION_CLASS,
        TransactionManagerInformation: *mut c_void,
        TransactionManagerInformationLength: c_ulong,
    ) -> NTSTATUS;
    fn ZwSetInformationVirtualMemory(
        ProcessHandle: HANDLE,
        VmInformationClass: VIRTUAL_MEMORY_INFORMATION_CLASS,
        NumberOfEntries: usize,
        VirtualAddresses: PMEMORY_RANGE_ENTRY,
        VmInformation: *mut c_void,
        VmInformationLength: c_ulong,
    ) -> NTSTATUS;
    fn ZwSetInformationWorkerFactory(
        WorkerFactoryHandle: HANDLE,
        WorkerFactoryInformationClass: WORKERFACTORYINFOCLASS,
        WorkerFactoryInformation: *mut c_void,
        WorkerFactoryInformationLength: c_ulong,
    ) -> NTSTATUS;
    fn ZwSetIntervalProfile(
        Interval: c_ulong,
        Source: KPROFILE_SOURCE,
    ) -> NTSTATUS;
    fn ZwSetIoCompletion(
        IoCompletionHandle: HANDLE,
        KeyContext: *mut c_void,
        ApcContext: *mut c_void,
        IoStatus: NTSTATUS,
        IoStatusInformation: usize,
    ) -> NTSTATUS;
    fn ZwSetIoCompletionEx(
        IoCompletionHandle: HANDLE,
        IoCompletionPacketHandle: HANDLE,
        KeyContext: *mut c_void,
        ApcContext: *mut c_void,
        IoStatus: NTSTATUS,
        IoStatusInformation: usize,
    ) -> NTSTATUS;
    fn ZwSetLdtEntries(
        Selector0: c_ulong,
        Entry0Low: c_ulong,
        Entry0Hi: c_ulong,
        Selector1: c_ulong,
        Entry1Low: c_ulong,
        Entry1Hi: c_ulong,
    ) -> NTSTATUS;
    fn ZwSetLowEventPair(
        EventPairHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwSetLowWaitHighEventPair(
        EventPairHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwSetQuotaInformationFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
        Buffer: *mut c_void,
        Length: c_ulong,
    ) -> NTSTATUS;
    fn ZwSetSecurityObject(
        Handle: HANDLE,
        SecurityInformation: c_ulong,
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
    ) -> NTSTATUS;
    fn ZwSetSystemEnvironmentValue(
        VariableName: *mut UNICODE_STRING,
        VariableValue: *mut UNICODE_STRING,
    ) -> NTSTATUS;
    fn ZwSetSystemEnvironmentValueEx(
        VariableName: *mut UNICODE_STRING,
        VendorGuid: *mut GUID,
        Value: *mut c_void,
        ValueLength: c_ulong,
        Attributes: c_ulong,
    ) -> NTSTATUS;
    fn ZwSetSystemInformation(
        SystemInformationClass: SYSTEM_INFORMATION_CLASS,
        SystemInformation: *mut c_void,
        SystemInformationLength: c_ulong,
    ) -> NTSTATUS;
    fn ZwSetSystemPowerState(
        SystemAction: POWER_ACTION,
        LightestSystemState: SYSTEM_POWER_STATE,
        Flags: c_ulong,
    ) -> NTSTATUS;
    fn ZwSetSystemTime(
        SystemTime: *mut LARGE_INTEGER,
        PreviousTime: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwSetThreadExecutionState(
        NewFlags: EXECUTION_STATE,
        PreviousFlags: *mut EXECUTION_STATE,
    ) -> NTSTATUS;
    fn ZwSetTimer(
        TimerHandle: HANDLE,
        DueTime: *mut LARGE_INTEGER,
        TimerApcRoutine: PTIMER_APC_ROUTINE,
        TimerContext: *mut c_void,
        ResumeTimer: c_uchar,
        Period: c_long,
        PreviousState: *mut c_uchar,
    ) -> NTSTATUS;
    fn ZwSetTimer2(
        TimerHandle: HANDLE,
        DueTime: *mut LARGE_INTEGER,
        Period: *mut LARGE_INTEGER,
        Parameters: PT2_SET_PARAMETERS,
    ) -> NTSTATUS;
    fn ZwSetTimerEx(
        TimerHandle: HANDLE,
        TimerSetInformationClass: TIMER_SET_INFORMATION_CLASS,
        TimerSetInformation: *mut c_void,
        TimerSetInformationLength: c_ulong,
    ) -> NTSTATUS;
    fn ZwSetTimerResolution(
        DesiredTime: c_ulong,
        SetResolution: c_uchar,
        ActualTime: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwSetUuidSeed(
        Seed: *mut c_char,
    ) -> NTSTATUS;
    fn ZwSetValueKey(
        KeyHandle: HANDLE,
        ValueName: *mut UNICODE_STRING,
        TitleIndex: c_ulong,
        Type: c_ulong,
        Data: *mut c_void,
        DataSize: c_ulong,
    ) -> NTSTATUS;
    fn ZwSetVolumeInformationFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
        FsInformation: *mut c_void,
        Length: c_ulong,
        FsInformationClass: FS_INFORMATION_CLASS,
    ) -> NTSTATUS;
    fn ZwSetWnfProcessNotificationEvent(
        NotificationEvent: HANDLE,
    ) -> NTSTATUS;
    fn ZwShutdownSystem(
        Action: SHUTDOWN_ACTION,
    ) -> NTSTATUS;
    fn ZwShutdownWorkerFactory(
        WorkerFactoryHandle: HANDLE,
        PendingWorkerCount: *mut c_long,
    ) -> NTSTATUS;
    fn ZwSignalAndWaitForSingleObject(
        SignalHandle: HANDLE,
        WaitHandle: HANDLE,
        Alertable: c_uchar,
        Timeout: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwSinglePhaseReject(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwStartProfile(
        ProfileHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwStopProfile(
        ProfileHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwSubscribeWnfStateChange(
        StateName: *const WNF_STATE_NAME,
        ChangeStamp: WNF_CHANGE_STAMP,
        EventMask: c_ulong,
        SubscriptionId: *mut __uint64,
    ) -> NTSTATUS;
    fn ZwSuspendProcess(
        ProcessHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwSuspendThread(
        ThreadHandle: HANDLE,
        PreviousSuspendCount: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwSystemDebugControl(
        Command: SYSDBG_COMMAND,
        InputBuffer: *mut c_void,
        InputBufferLength: c_ulong,
        OutputBuffer: *mut c_void,
        OutputBufferLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwTerminateJobObject(
        JobHandle: HANDLE,
        ExitStatus: NTSTATUS,
    ) -> NTSTATUS;
    fn ZwTerminateProcess(
        ProcessHandle: HANDLE,
        ExitStatus: NTSTATUS,
    ) -> NTSTATUS;
    fn ZwTerminateThread(
        ThreadHandle: HANDLE,
        ExitStatus: NTSTATUS,
    ) -> NTSTATUS;
    fn ZwTestAlert() -> NTSTATUS;
    fn ZwThawRegistry() -> NTSTATUS;
    fn ZwThawTransactions() -> NTSTATUS;
    fn ZwTraceControl(
        FunctionCode: c_ulong,
        InBuffer: *mut c_void,
        InBufferLen: c_ulong,
        OutBuffer: *mut c_void,
        OutBufferLen: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwTraceEvent(
        TraceHandle: HANDLE,
        Flags: c_ulong,
        FieldSize: c_ulong,
        Fields: *mut c_void,
    ) -> NTSTATUS;
    fn ZwTranslateFilePath(
        InputFilePath: PFILE_PATH,
        OutputType: c_ulong,
        OutputFilePath: PFILE_PATH,
        OutputFilePathLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwUmsThreadYield(
        SchedulerParam: *mut c_void,
    ) -> NTSTATUS;
    fn ZwUnloadDriver(
        DriverServiceName: *mut UNICODE_STRING,
    ) -> NTSTATUS;
    fn ZwUnloadKey(
        TargetKey: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwUnloadKey2(
        TargetKey: *mut OBJECT_ATTRIBUTES,
        Flags: c_ulong,
    ) -> NTSTATUS;
    fn ZwUnloadKeyEx(
        TargetKey: *mut OBJECT_ATTRIBUTES,
        Event: HANDLE,
    ) -> NTSTATUS;
    fn ZwUnlockFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
        ByteOffset: *mut LARGE_INTEGER,
        Length: *mut LARGE_INTEGER,
        Key: c_ulong,
    ) -> NTSTATUS;
    fn ZwUnlockVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut *mut c_void,
        RegionSize: *mut usize,
        MapType: c_ulong,
    ) -> NTSTATUS;
    fn ZwUnmapViewOfSection(
        ProcessHandle: HANDLE,
        BaseAddress: *mut c_void,
    ) -> NTSTATUS;
    fn ZwUnmapViewOfSectionEx(
        ProcessHandle: HANDLE,
        BaseAddress: *mut c_void,
        Flags: c_ulong,
    ) -> NTSTATUS;
    fn ZwUnsubscribeWnfStateChange(
        StateName: *const WNF_STATE_NAME,
    ) -> NTSTATUS;
    fn ZwUpdateWnfStateData(
        StateName: *const WNF_STATE_NAME,
        Buffer: *const c_void,
        Length: c_ulong,
        TypeId: PCWNF_TYPE_ID,
        ExplicitScope: *const c_void,
        MatchingChangeStamp: WNF_CHANGE_STAMP,
        CheckStamp: c_ulong,
    ) -> NTSTATUS;
    fn ZwVdmControl(
        Service: VDMSERVICECLASS,
        ServiceData: *mut c_void,
    ) -> NTSTATUS;
    fn ZwWaitForAlertByThreadId(
        Address: *mut c_void,
        Timeout: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwWaitForDebugEvent(
        DebugObjectHandle: HANDLE,
        Alertable: c_uchar,
        Timeout: *mut LARGE_INTEGER,
        WaitStateChange: *mut c_void,
    ) -> NTSTATUS;
    fn ZwWaitForKeyedEvent(
        KeyedEventHandle: HANDLE,
        KeyValue: *mut c_void,
        Alertable: c_uchar,
        Timeout: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwWaitForMultipleObjects(
        Count: c_ulong,
        Handles: *mut HANDLE,
        WaitType: WAIT_TYPE,
        Alertable: c_uchar,
        Timeout: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwWaitForMultipleObjects32(
        Count: c_ulong,
        Handles: *mut c_long,
        WaitType: WAIT_TYPE,
        Alertable: c_uchar,
        Timeout: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwWaitForSingleObject(
        Handle: HANDLE,
        Alertable: c_uchar,
        Timeout: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwWaitForWorkViaWorkerFactory(
        WorkerFactoryHandle: HANDLE,
        MiniPacket: *mut FILE_IO_COMPLETION_INFORMATION,
    ) -> NTSTATUS;
    fn ZwWaitHighEventPair(
        EventPairHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwWaitLowEventPair(
        EventPairHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwWorkerFactoryWorkerReady(
        WorkerFactoryHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwWriteFile(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: *mut c_void,
        IoStatusBlock: PIO_STATUS_BLOCK,
        Buffer: *mut c_void,
        Length: c_ulong,
        ByteOffset: *mut LARGE_INTEGER,
        Key: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwWriteFileGather(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: *mut c_void,
        IoStatusBlock: PIO_STATUS_BLOCK,
        SegmentArray: *mut FILE_SEGMENT_ELEMENT,
        Length: c_ulong,
        ByteOffset: *mut LARGE_INTEGER,
        Key: *mut c_ulong,
    ) -> NTSTATUS;
    fn ZwWriteRequestData(
        PortHandle: HANDLE,
        Message: PPORT_MESSAGE,
        DataEntryIndex: c_ulong,
        Buffer: *mut c_void,
        BufferSize: usize,
        NumberOfBytesWritten: *mut usize,
    ) -> NTSTATUS;
    fn ZwWriteVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut c_void,
        Buffer: *mut c_void,
        BufferSize: usize,
        NumberOfBytesWritten: *mut usize,
    ) -> NTSTATUS;
    fn ZwYieldExecution() -> NTSTATUS;
}}
