use core::mem::size_of;

use windows_sys::Win32::{
    Foundation::{HANDLE, NTSTATUS, PSID, UNICODE_STRING},
    Security::{PSECURITY_DESCRIPTOR, SECURITY_QUALITY_OF_SERVICE},
    Storage::FileSystem::{STANDARD_RIGHTS_REQUIRED, SYNCHRONIZE},
    System::{
        Kernel::OBJ_CASE_INSENSITIVE, Threading::RTL_SRWLOCK,
        WindowsProgramming::OBJECT_ATTRIBUTES,
    },
};

use crate::{
    ctypes::{__uint64, c_double, c_short, c_uchar, c_ulong, c_void},
    ntapi_base::{CLIENT_ID, CLIENT_ID64},
    windows_local::shared::ntdef::LARGE_INTEGER,
};

pub const PORT_CONNECT: u32 = 0x0001;
pub const PORT_ALL_ACCESS: u32 = STANDARD_RIGHTS_REQUIRED | SYNCHRONIZE | 0x1;
STRUCT! {struct PORT_MESSAGE_u1_s {
    DataLength: c_short,
    TotalLength: c_short,
}}
STRUCT! {struct PORT_MESSAGE_u2_s {
    Type: c_short,
    DataInfoOffset: c_short,
}}
UNION! {union PORT_MESSAGE_u1 {
    s: PORT_MESSAGE_u1_s,
    Length: c_ulong,
}}
UNION! {union PORT_MESSAGE_u2 {
    s: PORT_MESSAGE_u2_s,
    ZeroInit: c_ulong,
}}
UNION! {union PORT_MESSAGE_u3 {
    ClientId: CLIENT_ID,
    DoNotUseThisField: c_double,
}}
UNION! {union PORT_MESSAGE_u4 {
    ClientViewSize: usize,
    CallbackId: c_ulong,
}}
STRUCT! {struct PORT_MESSAGE {
    u1: PORT_MESSAGE_u1,
    u2: PORT_MESSAGE_u2,
    u3: PORT_MESSAGE_u3,
    MessageId: c_ulong,
    u4: PORT_MESSAGE_u4,
}}
pub type PPORT_MESSAGE = *mut PORT_MESSAGE;
STRUCT! {struct PORT_DATA_ENTRY {
    Base: *mut c_void,
    Size: c_ulong,
}}
pub type PPORT_DATA_ENTRY = *mut PORT_DATA_ENTRY;
STRUCT! {struct PORT_DATA_INFORMATION {
    CountDataEntries: c_ulong,
    DataEntries: [PORT_DATA_ENTRY; 1],
}}
pub type PPORT_DATA_INFORMATION = *mut PORT_DATA_INFORMATION;
pub const LPC_REQUEST: c_ulong = 1;
pub const LPC_REPLY: c_ulong = 2;
pub const LPC_DATAGRAM: c_ulong = 3;
pub const LPC_LOST_REPLY: c_ulong = 4;
pub const LPC_PORT_CLOSED: c_ulong = 5;
pub const LPC_CLIENT_DIED: c_ulong = 6;
pub const LPC_EXCEPTION: c_ulong = 7;
pub const LPC_DEBUG_EVENT: c_ulong = 8;
pub const LPC_ERROR_EVENT: c_ulong = 9;
pub const LPC_CONNECTION_REQUEST: c_ulong = 10;
pub const LPC_KERNELMODE_MESSAGE: c_short = 0x8000;
pub const LPC_NO_IMPERSONATE: c_short = 0x4000;
pub const PORT_VALID_OBJECT_ATTRIBUTES: u32 = OBJ_CASE_INSENSITIVE as u32;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
pub const PORT_MAXIMUM_MESSAGE_LENGTH: u32 = 512;
#[cfg(target_arch = "x86")]
pub const PORT_MAXIMUM_MESSAGE_LENGTH: u32 = 256;
pub const LPC_MAX_CONNECTION_INFO_SIZE: u32 = 16 * size_of::<usize>() as u32;
pub const PORT_TOTAL_MAXIMUM_MESSAGE_LENGTH: u32 = (PORT_MAXIMUM_MESSAGE_LENGTH
    + size_of::<PORT_MESSAGE>() as u32
    + LPC_MAX_CONNECTION_INFO_SIZE
    + 0xf)
    & !0xf;
STRUCT! {struct LPC_CLIENT_DIED_MSG {
    PortMsg: PORT_MESSAGE,
    CreateTime: LARGE_INTEGER,
}}
pub type PLPC_CLIENT_DIED_MSG = *mut LPC_CLIENT_DIED_MSG;
STRUCT! {struct PORT_VIEW {
    Length: c_ulong,
    SectionHandle: HANDLE,
    SectionOffset: c_ulong,
    ViewSize: usize,
    ViewBase: *mut c_void,
    ViewRemoteBase: *mut c_void,
}}
pub type PPORT_VIEW = *mut PORT_VIEW;
STRUCT! {struct REMOTE_PORT_VIEW {
    Length: c_ulong,
    ViewSize: usize,
    ViewBase: *mut c_void,
}}
pub type PREMOTE_PORT_VIEW = *mut REMOTE_PORT_VIEW;
STRUCT! {struct PORT_MESSAGE64_u1_s {
    DataLength: c_short,
    TotalLength: c_short,
}}
STRUCT! {struct PORT_MESSAGE64_u2_s {
    Type: c_short,
    DataInfoOffset: c_short,
}}
UNION! {union PORT_MESSAGE64_u1 {
    s: PORT_MESSAGE64_u1_s,
    Length: c_ulong,
}}
UNION! {union PORT_MESSAGE64_u2 {
    s: PORT_MESSAGE64_u2_s,
    ZeroInit: c_ulong,
}}
UNION! {union PORT_MESSAGE64_u3 {
    ClientId: CLIENT_ID64,
    DoNotUseThisField: c_double,
}}
UNION! {union PORT_MESSAGE64_u4 {
    ClientViewSize: __uint64,
    CallbackId: c_ulong,
}}
STRUCT! {struct PORT_MESSAGE64 {
    u1: PORT_MESSAGE64_u1,
    u2: PORT_MESSAGE64_u2,
    u3: PORT_MESSAGE64_u3,
    MessageId: c_ulong,
    u4: PORT_MESSAGE64_u4,
}}
pub type PPORT_MESSAGE64 = *mut PORT_MESSAGE64;
STRUCT! {struct LPC_CLIENT_DIED_MSG64 {
    PortMsg: PORT_MESSAGE64,
    CreateTime: LARGE_INTEGER,
}}
pub type PLPC_CLIENT_DIED_MSG64 = *mut LPC_CLIENT_DIED_MSG64;
STRUCT! {struct PORT_VIEW64 {
    Length: c_ulong,
    SectionHandle: __uint64,
    SectionOffset: c_ulong,
    ViewSize: __uint64,
    ViewBase: __uint64,
    ViewRemoteBase: __uint64,
}}
pub type PPORT_VIEW64 = *mut PORT_VIEW64;
STRUCT! {struct REMOTE_PORT_VIEW64 {
    Length: c_ulong,
    ViewSize: __uint64,
    ViewBase: __uint64,
}}
pub type PREMOTE_PORT_VIEW64 = *mut REMOTE_PORT_VIEW64;
EXTERN! {extern "system" {
    fn NtCreatePort(
        PortHandle: *mut HANDLE,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        MaxConnectionInfoLength: c_ulong,
        MaxMessageLength: c_ulong,
        MaxPoolUsage: c_ulong,
    ) -> NTSTATUS;
    fn NtCreateWaitablePort(
        PortHandle: *mut HANDLE,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        MaxConnectionInfoLength: c_ulong,
        MaxMessageLength: c_ulong,
        MaxPoolUsage: c_ulong,
    ) -> NTSTATUS;
    fn NtConnectPort(
        PortHandle: *mut HANDLE,
        PortName: *mut UNICODE_STRING,
        SecurityQos: *mut SECURITY_QUALITY_OF_SERVICE,
        ClientView: PPORT_VIEW,
        ServerView: PREMOTE_PORT_VIEW,
        MaxMessageLength: *mut c_ulong,
        ConnectionInformation: *mut c_void,
        ConnectionInformationLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtSecureConnectPort(
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
    fn NtListenPort(
        PortHandle: HANDLE,
        ConnectionRequest: PPORT_MESSAGE,
    ) -> NTSTATUS;
    fn NtAcceptConnectPort(
        PortHandle: *mut HANDLE,
        PortContext: *mut c_void,
        ConnectionRequest: PPORT_MESSAGE,
        AcceptConnection: c_uchar,
        ServerView: PPORT_VIEW,
        ClientView: PREMOTE_PORT_VIEW,
    ) -> NTSTATUS;
    fn NtCompleteConnectPort(
        PortHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtRequestPort(
        PortHandle: HANDLE,
        RequestMessage: PPORT_MESSAGE,
    ) -> NTSTATUS;
    fn NtRequestWaitReplyPort(
        PortHandle: HANDLE,
        RequestMessage: PPORT_MESSAGE,
        ReplyMessage: PPORT_MESSAGE,
    ) -> NTSTATUS;
    fn NtReplyPort(
        PortHandle: HANDLE,
        ReplyMessage: PPORT_MESSAGE,
    ) -> NTSTATUS;
    fn NtReplyWaitReplyPort(
        PortHandle: HANDLE,
        ReplyMessage: PPORT_MESSAGE,
    ) -> NTSTATUS;
    fn NtReplyWaitReceivePort(
        PortHandle: HANDLE,
        PortContext: *mut *mut c_void,
        ReplyMessage: PPORT_MESSAGE,
        ReceiveMessage: PPORT_MESSAGE,
    ) -> NTSTATUS;
    fn NtReplyWaitReceivePortEx(
        PortHandle: HANDLE,
        PortContext: *mut *mut c_void,
        ReplyMessage: PPORT_MESSAGE,
        ReceiveMessage: PPORT_MESSAGE,
        Timeout: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtImpersonateClientOfPort(
        PortHandle: HANDLE,
        Message: PPORT_MESSAGE,
    ) -> NTSTATUS;
    fn NtReadRequestData(
        PortHandle: HANDLE,
        Message: PPORT_MESSAGE,
        DataEntryIndex: c_ulong,
        Buffer: *mut c_void,
        BufferSize: usize,
        NumberOfBytesRead: *mut usize,
    ) -> NTSTATUS;
    fn NtWriteRequestData(
        PortHandle: HANDLE,
        Message: PPORT_MESSAGE,
        DataEntryIndex: c_ulong,
        Buffer: *mut c_void,
        BufferSize: usize,
        NumberOfBytesWritten: *mut usize,
    ) -> NTSTATUS;
}}
ENUM! {enum PORT_INFORMATION_CLASS {
    PortBasicInformation = 0,
    PortDumpInformation = 1,
}}
EXTERN! {extern "system" {
    fn NtQueryInformationPort(
        PortHandle: HANDLE,
        PortInformationClass: PORT_INFORMATION_CLASS,
        PortInformation: *mut c_void,
        Length: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
}}
pub type PALPC_HANDLE = *mut HANDLE;
pub type ALPC_HANDLE = HANDLE;
pub const ALPC_PORFLG_ALLOW_LPC_REQUESTS: c_ulong = 0x20000;
pub const ALPC_PORFLG_WAITABLE_PORT: c_ulong = 0x40000;
pub const ALPC_PORFLG_SYSTEM_PROCESS: c_ulong = 0x100000;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
STRUCT! {struct ALPC_PORT_ATTRIBUTES {
    Flags: c_ulong,
    SecurityQos: SECURITY_QUALITY_OF_SERVICE,
    MaxMessageLength: usize,
    MemoryBandwidth: usize,
    MaxPoolUsage: usize,
    MaxSectionSize: usize,
    MaxViewSize: usize,
    MaxTotalSectionSize: usize,
    DupObjectTypes: c_ulong,
    Reserved: c_ulong,
}}
#[cfg(target_arch = "x86")]
STRUCT! {struct ALPC_PORT_ATTRIBUTES {
    Flags: c_ulong,
    SecurityQos: SECURITY_QUALITY_OF_SERVICE,
    MaxMessageLength: usize,
    MemoryBandwidth: usize,
    MaxPoolUsage: usize,
    MaxSectionSize: usize,
    MaxViewSize: usize,
    MaxTotalSectionSize: usize,
    DupObjectTypes: c_ulong,
}}
pub type PALPC_PORT_ATTRIBUTES = *mut ALPC_PORT_ATTRIBUTES;
pub const ALPC_MESSAGE_SECURITY_ATTRIBUTE: c_ulong = 0x80000000;
pub const ALPC_MESSAGE_VIEW_ATTRIBUTE: c_ulong = 0x40000000;
pub const ALPC_MESSAGE_CONTEXT_ATTRIBUTE: c_ulong = 0x20000000;
pub const ALPC_MESSAGE_HANDLE_ATTRIBUTE: c_ulong = 0x10000000;
STRUCT! {struct ALPC_MESSAGE_ATTRIBUTES {
    AllocatedAttributes: c_ulong,
    ValidAttributes: c_ulong,
}}
pub type PALPC_MESSAGE_ATTRIBUTES = *mut ALPC_MESSAGE_ATTRIBUTES;
STRUCT! {struct ALPC_COMPLETION_LIST_STATE {
    Value: __uint64,
}}
BITFIELD! {ALPC_COMPLETION_LIST_STATE Value: __uint64 [
    Head set_Head[0..24],
    Tail set_Tail[24..48],
    ActiveThreadCount set_ActiveThreadCount[48..64],
]}
pub type PALPC_COMPLETION_LIST_STATE = *mut ALPC_COMPLETION_LIST_STATE;
pub const ALPC_COMPLETION_LIST_BUFFER_GRANULARITY_MASK: c_ulong = 0x3f;
STRUCT! {#[repr(align(128))] struct ALPC_COMPLETION_LIST_HEADER {
    StartMagic: __uint64,
    TotalSize: c_ulong,
    ListOffset: c_ulong,
    ListSize: c_ulong,
    BitmapOffset: c_ulong,
    BitmapSize: c_ulong,
    DataOffset: c_ulong,
    DataSize: c_ulong,
    AttributeFlags: c_ulong,
    AttributeSize: c_ulong,
    __padding0: [u64; 10],
    State: ALPC_COMPLETION_LIST_STATE,
    LastMessageId: c_ulong,
    LastCallbackId: c_ulong,
    __padding1: [u32; 28],
    PostCount: c_ulong,
    __padding2: [u32; 31],
    ReturnCount: c_ulong,
    __padding3: [u32; 31],
    LogSequenceNumber: c_ulong,
    __padding4: [u64; 15],
    UserLock: RTL_SRWLOCK,
    EndMagic: __uint64,
    __padding5: [u64; 14],
}}
pub type PALPC_COMPLETION_LIST_HEADER = *mut ALPC_COMPLETION_LIST_HEADER;
STRUCT! {struct ALPC_CONTEXT_ATTR {
    PortContext: *mut c_void,
    MessageContext: *mut c_void,
    Sequence: c_ulong,
    MessageId: c_ulong,
    CallbackId: c_ulong,
}}
pub type PALPC_CONTEXT_ATTR = *mut ALPC_CONTEXT_ATTR;
pub const ALPC_HANDLEFLG_DUPLICATE_SAME_ACCESS: c_ulong = 0x10000;
pub const ALPC_HANDLEFLG_DUPLICATE_SAME_ATTRIBUTES: c_ulong = 0x20000;
pub const ALPC_HANDLEFLG_DUPLICATE_INHERIT: c_ulong = 0x80000;
STRUCT! {struct ALPC_HANDLE_ATTR32 {
    Flags: c_ulong,
    Reserved0: c_ulong,
    SameAccess: c_ulong,
    SameAttributes: c_ulong,
    Indirect: c_ulong,
    Inherit: c_ulong,
    Reserved1: c_ulong,
    Handle: c_ulong,
    ObjectType: c_ulong,
    DesiredAccess: c_ulong,
    GrantedAccess: c_ulong,
}}
pub type PALPC_HANDLE_ATTR32 = *mut ALPC_HANDLE_ATTR32;
STRUCT! {struct ALPC_HANDLE_ATTR {
    Flags: c_ulong,
    Reserved0: c_ulong,
    SameAccess: c_ulong,
    SameAttributes: c_ulong,
    Indirect: c_ulong,
    Inherit: c_ulong,
    Reserved1: c_ulong,
    Handle: HANDLE,
    HandleAttrArray: PALPC_HANDLE_ATTR32,
    ObjectType: c_ulong,
    HandleCount: c_ulong,
    DesiredAccess: c_ulong,
    GrantedAccess: c_ulong,
}}
pub type PALPC_HANDLE_ATTR = *mut ALPC_HANDLE_ATTR;
pub const ALPC_SECFLG_CREATE_HANDLE: c_ulong = 0x20000;
STRUCT! {struct ALPC_SECURITY_ATTR {
    Flags: c_ulong,
    QoS: *mut SECURITY_QUALITY_OF_SERVICE,
    ContextHandle: ALPC_HANDLE,
}}
pub type PALPC_SECURITY_ATTR = *mut ALPC_SECURITY_ATTR;
pub const ALPC_VIEWFLG_NOT_SECURE: c_ulong = 0x40000;
STRUCT! {struct ALPC_DATA_VIEW_ATTR {
    Flags: c_ulong,
    SectionHandle: ALPC_HANDLE,
    ViewBase: *mut c_void,
    ViewSize: usize,
}}
pub type PALPC_DATA_VIEW_ATTR = *mut ALPC_DATA_VIEW_ATTR;
ENUM! {enum ALPC_PORT_INFORMATION_CLASS {
    AlpcBasicInformation = 0,
    AlpcPortInformation = 1,
    AlpcAssociateCompletionPortInformation = 2,
    AlpcConnectedSIDInformation = 3,
    AlpcServerInformation = 4,
    AlpcMessageZoneInformation = 5,
    AlpcRegisterCompletionListInformation = 6,
    AlpcUnregisterCompletionListInformation = 7,
    AlpcAdjustCompletionListConcurrencyCountInformation = 8,
    AlpcRegisterCallbackInformation = 9,
    AlpcCompletionListRundownInformation = 10,
    AlpcWaitForPortReferences = 11,
}}
STRUCT! {struct ALPC_BASIC_INFORMATION {
    Flags: c_ulong,
    SequenceNo: c_ulong,
    PortContext: *mut c_void,
}}
pub type PALPC_BASIC_INFORMATION = *mut ALPC_BASIC_INFORMATION;
STRUCT! {struct ALPC_PORT_ASSOCIATE_COMPLETION_PORT {
    CompletionKey: *mut c_void,
    CompletionPort: HANDLE,
}}
pub type PALPC_PORT_ASSOCIATE_COMPLETION_PORT =
    *mut ALPC_PORT_ASSOCIATE_COMPLETION_PORT;
STRUCT! {struct ALPC_SERVER_INFORMATION_Out {
    ThreadBlocked: c_uchar,
    ConnectedProcessId: HANDLE,
    ConnectionPortName: UNICODE_STRING,
}}
UNION! {union ALPC_SERVER_INFORMATION {
    ThreadHandle: HANDLE,
    Out: ALPC_SERVER_INFORMATION_Out,
}}
pub type PALPC_SERVER_INFORMATION = *mut ALPC_SERVER_INFORMATION;
STRUCT! {struct ALPC_PORT_MESSAGE_ZONE_INFORMATION {
    Buffer: *mut c_void,
    Size: c_ulong,
}}
pub type PALPC_PORT_MESSAGE_ZONE_INFORMATION =
    *mut ALPC_PORT_MESSAGE_ZONE_INFORMATION;
STRUCT! {struct ALPC_PORT_COMPLETION_LIST_INFORMATION {
    Buffer: *mut c_void,
    Size: c_ulong,
    ConcurrencyCount: c_ulong,
    AttributeFlags: c_ulong,
}}
pub type PALPC_PORT_COMPLETION_LIST_INFORMATION =
    *mut ALPC_PORT_COMPLETION_LIST_INFORMATION;
ENUM! {enum ALPC_MESSAGE_INFORMATION_CLASS {
    AlpcMessageSidInformation = 0,
    AlpcMessageTokenModifiedIdInformation = 1,
    AlpcMessageDirectStatusInformation = 2,
    AlpcMessageHandleInformation = 3,
    MaxAlpcMessageInfoClass = 4,
}}
pub type PALPC_MESSAGE_INFORMATION_CLASS = *mut ALPC_MESSAGE_INFORMATION_CLASS;
STRUCT! {struct ALPC_MESSAGE_HANDLE_INFORMATION {
    Index: c_ulong,
    Flags: c_ulong,
    Handle: c_ulong,
    ObjectType: c_ulong,
    GrantedAccess: c_ulong,
}}
pub type PALPC_MESSAGE_HANDLE_INFORMATION =
    *mut ALPC_MESSAGE_HANDLE_INFORMATION;
EXTERN! {extern "system" {
    fn NtAlpcCreatePort(
        PortHandle: *mut HANDLE,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        PortAttributes: PALPC_PORT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn NtAlpcDisconnectPort(
        PortHandle: HANDLE,
        Flags: c_ulong,
    ) -> NTSTATUS;
    fn NtAlpcQueryInformation(
        PortHandle: HANDLE,
        PortInformationClass: ALPC_PORT_INFORMATION_CLASS,
        PortInformation: *mut c_void,
        Length: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtAlpcSetInformation(
        PortHandle: HANDLE,
        PortInformationClass: ALPC_PORT_INFORMATION_CLASS,
        PortInformation: *mut c_void,
        Length: c_ulong,
    ) -> NTSTATUS;
    fn NtAlpcCreatePortSection(
        PortHandle: HANDLE,
        Flags: c_ulong,
        SectionHandle: HANDLE,
        SectionSize: usize,
        AlpcSectionHandle: PALPC_HANDLE,
        ActualSectionSize: *mut usize,
    ) -> NTSTATUS;
    fn NtAlpcDeletePortSection(
        PortHandle: HANDLE,
        Flags: c_ulong,
        SectionHandle: ALPC_HANDLE,
    ) -> NTSTATUS;
    fn NtAlpcCreateResourceReserve(
        PortHandle: HANDLE,
        Flags: c_ulong,
        MessageSize: usize,
        ResourceId: PALPC_HANDLE,
    ) -> NTSTATUS;
    fn NtAlpcDeleteResourceReserve(
        PortHandle: HANDLE,
        Flags: c_ulong,
        ResourceId: ALPC_HANDLE,
    ) -> NTSTATUS;
    fn NtAlpcCreateSectionView(
        PortHandle: HANDLE,
        Flags: c_ulong,
        ViewAttributes: PALPC_DATA_VIEW_ATTR,
    ) -> NTSTATUS;
    fn NtAlpcDeleteSectionView(
        PortHandle: HANDLE,
        Flags: c_ulong,
        ViewBase: *mut c_void,
    ) -> NTSTATUS;
    fn NtAlpcCreateSecurityContext(
        PortHandle: HANDLE,
        Flags: c_ulong,
        SecurityAttribute: PALPC_SECURITY_ATTR,
    ) -> NTSTATUS;
    fn NtAlpcDeleteSecurityContext(
        PortHandle: HANDLE,
        Flags: c_ulong,
        ContextHandle: ALPC_HANDLE,
    ) -> NTSTATUS;
    fn NtAlpcRevokeSecurityContext(
        PortHandle: HANDLE,
        Flags: c_ulong,
        ContextHandle: ALPC_HANDLE,
    ) -> NTSTATUS;
    fn NtAlpcQueryInformationMessage(
        PortHandle: HANDLE,
        PortMessage: PPORT_MESSAGE,
        MessageInformationClass: ALPC_MESSAGE_INFORMATION_CLASS,
        MessageInformation: *mut c_void,
        Length: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
}}
pub const ALPC_MSGFLG_REPLY_MESSAGE: c_ulong = 0x1;
pub const ALPC_MSGFLG_LPC_MODE: c_ulong = 0x2;
pub const ALPC_MSGFLG_RELEASE_MESSAGE: c_ulong = 0x10000;
pub const ALPC_MSGFLG_SYNC_REQUEST: c_ulong = 0x20000;
pub const ALPC_MSGFLG_WAIT_USER_MODE: c_ulong = 0x100000;
pub const ALPC_MSGFLG_WAIT_ALERTABLE: c_ulong = 0x200000;
pub const ALPC_MSGFLG_WOW64_CALL: c_ulong = 0x80000000;
EXTERN! {extern "system" {
    fn NtAlpcConnectPort(
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
    fn NtAlpcConnectPortEx(
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
    fn NtAlpcAcceptConnectPort(
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
    fn NtAlpcSendWaitReceivePort(
        PortHandle: HANDLE,
        Flags: c_ulong,
        SendMessageA: PPORT_MESSAGE,
        SendMessageAttributes: PALPC_MESSAGE_ATTRIBUTES,
        ReceiveMessage: PPORT_MESSAGE,
        BufferLength: *mut usize,
        ReceiveMessageAttributes: PALPC_MESSAGE_ATTRIBUTES,
        Timeout: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
}}
pub const ALPC_CANCELFLG_TRY_CANCEL: c_ulong = 0x1;
pub const ALPC_CANCELFLG_NO_CONTEXT_CHECK: c_ulong = 0x8;
pub const ALPC_CANCELFLGP_FLUSH: c_ulong = 0x10000;
EXTERN! {extern "system" {
    fn NtAlpcCancelMessage(
        PortHandle: HANDLE,
        Flags: c_ulong,
        MessageContext: PALPC_CONTEXT_ATTR,
    ) -> NTSTATUS;
    fn NtAlpcImpersonateClientOfPort(
        PortHandle: HANDLE,
        Message: PPORT_MESSAGE,
        Flags: *mut c_void,
    ) -> NTSTATUS;
    fn NtAlpcImpersonateClientContainerOfPort(
        PortHandle: HANDLE,
        Message: PPORT_MESSAGE,
        Flags: c_ulong,
    ) -> NTSTATUS;
    fn NtAlpcOpenSenderProcess(
        ProcessHandle: *mut HANDLE,
        PortHandle: HANDLE,
        PortMessage: PPORT_MESSAGE,
        Flags: c_ulong,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn NtAlpcOpenSenderThread(
        ThreadHandle: *mut HANDLE,
        PortHandle: HANDLE,
        PortMessage: PPORT_MESSAGE,
        Flags: c_ulong,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn AlpcMaxAllowedMessageLength() -> c_ulong;
    fn AlpcGetHeaderSize(
        Flags: c_ulong,
    ) -> c_ulong;
    fn AlpcInitializeMessageAttribute(
        AttributeFlags: c_ulong,
        Buffer: PALPC_MESSAGE_ATTRIBUTES,
        BufferSize: c_ulong,
        RequiredBufferSize: *mut c_ulong,
    ) -> NTSTATUS;
    fn AlpcGetMessageAttribute(
        Buffer: PALPC_MESSAGE_ATTRIBUTES,
        AttributeFlag: c_ulong,
    ) -> *mut c_void;
    fn AlpcRegisterCompletionList(
        PortHandle: HANDLE,
        Buffer: PALPC_COMPLETION_LIST_HEADER,
        Size: c_ulong,
        ConcurrencyCount: c_ulong,
        AttributeFlags: c_ulong,
    ) -> NTSTATUS;
    fn AlpcUnregisterCompletionList(
        PortHandle: HANDLE,
    ) -> NTSTATUS;
    fn AlpcRundownCompletionList(
        PortHandle: HANDLE,
    ) -> NTSTATUS;
    fn AlpcAdjustCompletionListConcurrencyCount(
        PortHandle: HANDLE,
        ConcurrencyCount: c_ulong,
    ) -> NTSTATUS;
    fn AlpcRegisterCompletionListWorkerThread(
        CompletionList: *mut c_void,
    ) -> c_uchar;
    fn AlpcUnregisterCompletionListWorkerThread(
        CompletionList: *mut c_void,
    ) -> c_uchar;
    fn AlpcGetCompletionListLastMessageInformation(
        CompletionList: *mut c_void,
        LastMessageId: *mut c_ulong,
        LastCallbackId: *mut c_ulong,
    );
    fn AlpcGetOutstandingCompletionListMessageCount(
        CompletionList: *mut c_void,
    ) -> c_ulong;
    fn AlpcGetMessageFromCompletionList(
        CompletionList: *mut c_void,
        MessageAttributes: *mut PALPC_MESSAGE_ATTRIBUTES,
    ) -> PPORT_MESSAGE;
    fn AlpcFreeCompletionListMessage(
        CompletionList: *mut c_void,
        Message: PPORT_MESSAGE,
    );
    fn AlpcGetCompletionListMessageAttributes(
        CompletionList: *mut c_void,
        Message: PPORT_MESSAGE,
    ) -> PALPC_MESSAGE_ATTRIBUTES;
}}
