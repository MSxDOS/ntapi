use windows_sys::Win32::{
    Foundation::{HANDLE, NTSTATUS, UNICODE_STRING},
    System::WindowsProgramming::OBJECT_ATTRIBUTES,
};

use crate::{
    ctypes::{c_uchar, c_ulong, c_ushort, c_void, wchar_t},
    ntioapi::{PIO_APC_ROUTINE, PIO_STATUS_BLOCK},
    windows_local::shared::ntdef::LARGE_INTEGER,
};

pub const REG_INIT_BOOT_SM: c_ushort = 0x0000;
pub const REG_INIT_BOOT_SETUP: c_ushort = 0x0001;
pub const REG_INIT_BOOT_ACCEPTED_BASE: c_ushort = 0x0002;
pub const REG_INIT_BOOT_ACCEPTED_MAX: c_ushort = REG_INIT_BOOT_ACCEPTED_BASE;
pub const REG_MAX_KEY_VALUE_NAME_LENGTH: u32 = 32767;
pub const REG_MAX_KEY_NAME_LENGTH: u32 = 512;
ENUM! {enum KEY_INFORMATION_CLASS {
    KeyBasicInformation = 0,
    KeyNodeInformation = 1,
    KeyFullInformation = 2,
    KeyNameInformation = 3,
    KeyCachedInformation = 4,
    KeyFlagsInformation = 5,
    KeyVirtualizationInformation = 6,
    KeyHandleTagsInformation = 7,
    KeyTrustInformation = 8,
    KeyLayerInformation = 9,
    MaxKeyInfoClass = 10,
}}
STRUCT! {struct KEY_BASIC_INFORMATION {
    LastWriteTime: LARGE_INTEGER,
    TitleIndex: c_ulong,
    NameLength: c_ulong,
    Name: [wchar_t; 1],
}}
pub type PKEY_BASIC_INFORMATION = *mut KEY_BASIC_INFORMATION;
STRUCT! {struct KEY_NODE_INFORMATION {
    LastWriteTime: LARGE_INTEGER,
    TitleIndex: c_ulong,
    ClassOffset: c_ulong,
    ClassLength: c_ulong,
    NameLength: c_ulong,
    Name: [wchar_t; 1],
}}
pub type PKEY_NODE_INFORMATION = *mut KEY_NODE_INFORMATION;
STRUCT! {struct KEY_FULL_INFORMATION {
    LastWriteTime: LARGE_INTEGER,
    TitleIndex: c_ulong,
    ClassOffset: c_ulong,
    ClassLength: c_ulong,
    SubKeys: c_ulong,
    MaxNameLen: c_ulong,
    MaxClassLen: c_ulong,
    Values: c_ulong,
    MaxValueNameLen: c_ulong,
    MaxValueDataLen: c_ulong,
    Class: [wchar_t; 1],
}}
pub type PKEY_FULL_INFORMATION = *mut KEY_FULL_INFORMATION;
STRUCT! {struct KEY_NAME_INFORMATION {
    NameLength: c_ulong,
    Name: [wchar_t; 1],
}}
pub type PKEY_NAME_INFORMATION = *mut KEY_NAME_INFORMATION;
STRUCT! {struct KEY_CACHED_INFORMATION {
    LastWriteTime: LARGE_INTEGER,
    TitleIndex: c_ulong,
    SubKeys: c_ulong,
    MaxNameLen: c_ulong,
    Values: c_ulong,
    MaxValueNameLen: c_ulong,
    MaxValueDataLen: c_ulong,
    NameLength: c_ulong,
    Name: [wchar_t; 1],
}}
pub type PKEY_CACHED_INFORMATION = *mut KEY_CACHED_INFORMATION;
STRUCT! {struct KEY_FLAGS_INFORMATION {
    UserFlags: c_ulong,
}}
pub type PKEY_FLAGS_INFORMATION = *mut KEY_FLAGS_INFORMATION;
STRUCT! {struct KEY_VIRTUALIZATION_INFORMATION {
    Bitfields: c_ulong,
}}
BITFIELD! {KEY_VIRTUALIZATION_INFORMATION Bitfields: c_ulong [
    VirtualizationCandidate set_VirtualizationCandidate[0..1],
    VirtualizationEnabled set_VirtualizationEnabled[1..2],
    VirtualTarget set_VirtualTarget[2..3],
    VirtualStore set_VirtualStore[3..4],
    VirtualSource set_VirtualSource[4..5],
    Reserved set_Reserved[5..32],
]}
pub type PKEY_VIRTUALIZATION_INFORMATION = *mut KEY_VIRTUALIZATION_INFORMATION;
STRUCT! {struct KEY_TRUST_INFORMATION {
    Bitfields: c_ulong,
}}
BITFIELD! {KEY_TRUST_INFORMATION Bitfields: c_ulong [
    TrustedKey set_TrustedKey[0..1],
    Reserved set_Reserved[1..32],
]}
pub type PKEY_TRUST_INFORMATION = *mut KEY_TRUST_INFORMATION;
STRUCT! {struct KEY_LAYER_INFORMATION {
    IsTombstone: c_ulong,
    IsSupersedeLocal: c_ulong,
    IsSupersedeTree: c_ulong,
    ClassIsInherited: c_ulong,
    Reserved: c_ulong,
}}
pub type PKEY_LAYER_INFORMATION = *mut KEY_LAYER_INFORMATION;
ENUM! {enum KEY_SET_INFORMATION_CLASS {
    KeyWriteTimeInformation = 0,
    KeyWow64FlagsInformation = 1,
    KeyControlFlagsInformation = 2,
    KeySetVirtualizationInformation = 3,
    KeySetDebugInformation = 4,
    KeySetHandleTagsInformation = 5,
    KeySetLayerInformation = 6,
    MaxKeySetInfoClass = 7,
}}
STRUCT! {struct KEY_WRITE_TIME_INFORMATION {
    LastWriteTime: LARGE_INTEGER,
}}
pub type PKEY_WRITE_TIME_INFORMATION = *mut KEY_WRITE_TIME_INFORMATION;
STRUCT! {struct KEY_WOW64_FLAGS_INFORMATION {
    UserFlags: c_ulong,
}}
pub type PKEY_WOW64_FLAGS_INFORMATION = *mut KEY_WOW64_FLAGS_INFORMATION;
STRUCT! {struct KEY_HANDLE_TAGS_INFORMATION {
    HandleTags: c_ulong,
}}
pub type PKEY_HANDLE_TAGS_INFORMATION = *mut KEY_HANDLE_TAGS_INFORMATION;
STRUCT! {struct KEY_SET_LAYER_INFORMATION {
    Bitfields: c_ulong,
}}
BITFIELD! {KEY_SET_LAYER_INFORMATION Bitfields: c_ulong [
    IsTombstone set_IsTombstone[0..1],
    IsSupersedeLocal set_IsSupersedeLocal[1..2],
    IsSupersedeTree set_IsSupersedeTree[2..3],
    ClassIsInherited set_ClassIsInherited[3..4],
    Reserved set_Reserved[4..32],
]}
pub type PKEY_SET_LAYER_INFORMATION = *mut KEY_SET_LAYER_INFORMATION;
STRUCT! {struct KEY_CONTROL_FLAGS_INFORMATION {
    ControlFlags: c_ulong,
}}
pub type PKEY_CONTROL_FLAGS_INFORMATION = *mut KEY_CONTROL_FLAGS_INFORMATION;
STRUCT! {struct KEY_SET_VIRTUALIZATION_INFORMATION {
    HandleTags: c_ulong,
}}
BITFIELD! {KEY_SET_VIRTUALIZATION_INFORMATION HandleTags: c_ulong [
    VirtualTarget set_VirtualTarget[0..1],
    VirtualStore set_VirtualStore[1..2],
    VirtualSource set_VirtualSource[2..3],
    Reserved set_Reserved[3..32],
]}
pub type PKEY_SET_VIRTUALIZATION_INFORMATION =
    *mut KEY_SET_VIRTUALIZATION_INFORMATION;
ENUM! {enum KEY_VALUE_INFORMATION_CLASS {
    KeyValueBasicInformation = 0,
    KeyValueFullInformation = 1,
    KeyValuePartialInformation = 2,
    KeyValueFullInformationAlign64 = 3,
    KeyValuePartialInformationAlign64 = 4,
    KeyValueLayerInformation = 5,
    MaxKeyValueInfoClass = 6,
}}
STRUCT! {struct KEY_VALUE_BASIC_INFORMATION {
    TitleIndex: c_ulong,
    Type: c_ulong,
    NameLength: c_ulong,
    Name: [wchar_t; 1],
}}
pub type PKEY_VALUE_BASIC_INFORMATION = *mut KEY_VALUE_BASIC_INFORMATION;
STRUCT! {struct KEY_VALUE_FULL_INFORMATION {
    TitleIndex: c_ulong,
    Type: c_ulong,
    DataOffset: c_ulong,
    DataLength: c_ulong,
    NameLength: c_ulong,
    Name: [wchar_t; 1],
}}
pub type PKEY_VALUE_FULL_INFORMATION = *mut KEY_VALUE_FULL_INFORMATION;
STRUCT! {struct KEY_VALUE_PARTIAL_INFORMATION {
    TitleIndex: c_ulong,
    Type: c_ulong,
    DataLength: c_ulong,
    Data: [c_uchar; 1],
}}
pub type PKEY_VALUE_PARTIAL_INFORMATION = *mut KEY_VALUE_PARTIAL_INFORMATION;
STRUCT! {struct KEY_VALUE_PARTIAL_INFORMATION_ALIGN64 {
    Type: c_ulong,
    DataLength: c_ulong,
    Data: [c_uchar; 1],
}}
pub type PKEY_VALUE_PARTIAL_INFORMATION_ALIGN64 =
    *mut KEY_VALUE_PARTIAL_INFORMATION_ALIGN64;
STRUCT! {struct KEY_VALUE_LAYER_INFORMATION {
    IsTombstone: c_ulong,
    Reserved: c_ulong,
}}
pub type PKEY_VALUE_LAYER_INFORMATION = *mut KEY_VALUE_LAYER_INFORMATION;
STRUCT! {struct KEY_VALUE_ENTRY {
    ValueName: *mut UNICODE_STRING,
    DataLength: c_ulong,
    DataOffset: c_ulong,
    Type: c_ulong,
}}
pub type PKEY_VALUE_ENTRY = *mut KEY_VALUE_ENTRY;
ENUM! {enum REG_ACTION {
    KeyAdded = 0,
    KeyRemoved = 1,
    KeyModified = 2,
}}
STRUCT! {struct REG_NOTIFY_INFORMATION {
    NextEntryOffset: c_ulong,
    Action: REG_ACTION,
    KeyLength: c_ulong,
    Key: [wchar_t; 1],
}}
pub type PREG_NOTIFY_INFORMATION = *mut REG_NOTIFY_INFORMATION;
STRUCT! {struct KEY_PID_ARRAY {
    PID: HANDLE,
    KeyName: UNICODE_STRING,
}}
pub type PKEY_PID_ARRAY = *mut KEY_PID_ARRAY;
STRUCT! {struct KEY_OPEN_SUBKEYS_INFORMATION {
    Count: c_ulong,
    KeyArray: [KEY_PID_ARRAY; 1],
}}
pub type PKEY_OPEN_SUBKEYS_INFORMATION = *mut KEY_OPEN_SUBKEYS_INFORMATION;
EXTERN! {extern "system" {
    fn NtCreateKey(
        KeyHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        TitleIndex: c_ulong,
        Class: *mut UNICODE_STRING,
        CreateOptions: c_ulong,
        Disposition: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtCreateKeyTransacted(
        KeyHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        TitleIndex: c_ulong,
        Class: *mut UNICODE_STRING,
        CreateOptions: c_ulong,
        TransactionHandle: HANDLE,
        Disposition: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtOpenKey(
        KeyHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn NtOpenKeyTransacted(
        KeyHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        TransactionHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtOpenKeyEx(
        KeyHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        OpenOptions: c_ulong,
    ) -> NTSTATUS;
    fn NtOpenKeyTransactedEx(
        KeyHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        OpenOptions: c_ulong,
        TransactionHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtDeleteKey(
        KeyHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtRenameKey(
        KeyHandle: HANDLE,
        NewName: *mut UNICODE_STRING,
    ) -> NTSTATUS;
    fn NtDeleteValueKey(
        KeyHandle: HANDLE,
        ValueName: *mut UNICODE_STRING,
    ) -> NTSTATUS;
    fn NtQueryKey(
        KeyHandle: HANDLE,
        KeyInformationClass: KEY_INFORMATION_CLASS,
        KeyInformation: *mut c_void,
        Length: c_ulong,
        ResultLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtSetInformationKey(
        KeyHandle: HANDLE,
        KeySetInformationClass: KEY_SET_INFORMATION_CLASS,
        KeySetInformation: *mut c_void,
        KeySetInformationLength: c_ulong,
    ) -> NTSTATUS;
    fn NtQueryValueKey(
        KeyHandle: HANDLE,
        ValueName: *mut UNICODE_STRING,
        KeyValueInformationClass: KEY_VALUE_INFORMATION_CLASS,
        KeyValueInformation: *mut c_void,
        Length: c_ulong,
        ResultLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtSetValueKey(
        KeyHandle: HANDLE,
        ValueName: *mut UNICODE_STRING,
        TitleIndex: c_ulong,
        Type: c_ulong,
        Data: *mut c_void,
        DataSize: c_ulong,
    ) -> NTSTATUS;
    fn NtQueryMultipleValueKey(
        KeyHandle: HANDLE,
        ValueEntries: PKEY_VALUE_ENTRY,
        EntryCount: c_ulong,
        ValueBuffer: *mut c_void,
        BufferLength: *mut c_ulong,
        RequiredBufferLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtEnumerateKey(
        KeyHandle: HANDLE,
        Index: c_ulong,
        KeyInformationClass: KEY_INFORMATION_CLASS,
        KeyInformation: *mut c_void,
        Length: c_ulong,
        ResultLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtEnumerateValueKey(
        KeyHandle: HANDLE,
        Index: c_ulong,
        KeyValueInformationClass: KEY_VALUE_INFORMATION_CLASS,
        KeyValueInformation: *mut c_void,
        Length: c_ulong,
        ResultLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtFlushKey(
        KeyHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtCompactKeys(
        Count: c_ulong,
        KeyArray: *mut HANDLE,
    ) -> NTSTATUS;
    fn NtCompressKey(
        Key: HANDLE,
    ) -> NTSTATUS;
    fn NtLoadKey(
        TargetKey: *mut OBJECT_ATTRIBUTES,
        SourceFile: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn NtLoadKey2(
        TargetKey: *mut OBJECT_ATTRIBUTES,
        SourceFile: *mut OBJECT_ATTRIBUTES,
        Flags: c_ulong,
    ) -> NTSTATUS;
    fn NtLoadKeyEx(
        TargetKey: *mut OBJECT_ATTRIBUTES,
        SourceFile: *mut OBJECT_ATTRIBUTES,
        Flags: c_ulong,
        TrustClassKey: HANDLE,
        Event: HANDLE,
        DesiredAccess: c_ulong,
        RootHandle: *mut HANDLE,
        IoStatus: PIO_STATUS_BLOCK,
    ) -> NTSTATUS;
    fn NtReplaceKey(
        NewFile: *mut OBJECT_ATTRIBUTES,
        TargetHandle: HANDLE,
        OldFile: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn NtSaveKey(
        KeyHandle: HANDLE,
        FileHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtSaveKeyEx(
        KeyHandle: HANDLE,
        FileHandle: HANDLE,
        Format: c_ulong,
    ) -> NTSTATUS;
    fn NtSaveMergedKeys(
        HighPrecedenceKeyHandle: HANDLE,
        LowPrecedenceKeyHandle: HANDLE,
        FileHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtRestoreKey(
        KeyHandle: HANDLE,
        FileHandle: HANDLE,
        Flags: c_ulong,
    ) -> NTSTATUS;
    fn NtUnloadKey(
        TargetKey: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}}
pub const REG_FORCE_UNLOAD: c_ulong = 1;
pub const REG_UNLOAD_LEGAL_FLAGS: c_ulong = REG_FORCE_UNLOAD;
EXTERN! {extern "system" {
    fn NtUnloadKey2(
        TargetKey: *mut OBJECT_ATTRIBUTES,
        Flags: c_ulong,
    ) -> NTSTATUS;
    fn NtUnloadKeyEx(
        TargetKey: *mut OBJECT_ATTRIBUTES,
        Event: HANDLE,
    ) -> NTSTATUS;
    fn NtNotifyChangeKey(
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
    fn NtNotifyChangeMultipleKeys(
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
    fn NtQueryOpenSubKeys(
        TargetKey: *mut OBJECT_ATTRIBUTES,
        HandleCount: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtQueryOpenSubKeysEx(
        TargetKey: *mut OBJECT_ATTRIBUTES,
        BufferLength: c_ulong,
        Buffer: *mut c_void,
        RequiredSize: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtInitializeRegistry(
        BootCondition: c_ushort,
    ) -> NTSTATUS;
    fn NtLockRegistryKey(
        KeyHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtLockProductActivationKeys(
        pPrivateVer: *mut c_ulong,
        pSafeMode: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtFreezeRegistry(
        TimeOutInSeconds: c_ulong,
    ) -> NTSTATUS;
    fn NtThawRegistry() -> NTSTATUS;
}}
