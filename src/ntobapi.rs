use windows_sys::Win32::{
    Foundation::{HANDLE, NTSTATUS, UNICODE_STRING},
    Security::{GENERIC_MAPPING, PSECURITY_DESCRIPTOR},
    Storage::FileSystem::STANDARD_RIGHTS_REQUIRED,
    System::{Kernel::WAIT_TYPE, WindowsProgramming::OBJECT_ATTRIBUTES},
};

use crate::{
    ctypes::{c_char, c_long, c_uchar, c_ulong, c_void},
    windows_local::shared::ntdef::LARGE_INTEGER,
};

pub const OBJECT_TYPE_CREATE: u32 = 0x0001;
pub const OBJECT_TYPE_ALL_ACCESS: c_ulong = STANDARD_RIGHTS_REQUIRED | 0x1;
pub const DIRECTORY_QUERY: u32 = 0x0001;
pub const DIRECTORY_TRAVERSE: u32 = 0x0002;
pub const DIRECTORY_CREATE_OBJECT: u32 = 0x0004;
pub const DIRECTORY_CREATE_SUBDIRECTORY: u32 = 0x0008;
pub const DIRECTORY_ALL_ACCESS: c_ulong = STANDARD_RIGHTS_REQUIRED | 0xf;
pub const SYMBOLIC_LINK_QUERY: u32 = 0x0001;
pub const SYMBOLIC_LINK_ALL_ACCESS: c_ulong = STANDARD_RIGHTS_REQUIRED | 0x1;
pub const OBJ_PROTECT_CLOSE: u32 = 0x00000001;
pub const OBJ_INHERIT: u32 = 0x00000002;
pub const OBJ_AUDIT_OBJECT_CLOSE: u32 = 0x00000004;
ENUM! {enum OBJECT_INFORMATION_CLASS {
    ObjectBasicInformation = 0,
    ObjectNameInformation = 1,
    ObjectTypeInformation = 2,
    ObjectTypesInformation = 3,
    ObjectHandleFlagInformation = 4,
    ObjectSessionInformation = 5,
    ObjectSessionObjectInformation = 6,
    MaxObjectInfoClass = 7,
}}
STRUCT! {struct OBJECT_BASIC_INFORMATION {
    Attributes: c_ulong,
    GrantedAccess: c_ulong,
    HandleCount: c_ulong,
    PointerCount: c_ulong,
    PagedPoolCharge: c_ulong,
    NonPagedPoolCharge: c_ulong,
    Reserved: [c_ulong; 3],
    NameInfoSize: c_ulong,
    TypeInfoSize: c_ulong,
    SecurityDescriptorSize: c_ulong,
    CreationTime: LARGE_INTEGER,
}}
pub type POBJECT_BASIC_INFORMATION = *mut OBJECT_BASIC_INFORMATION;
STRUCT! {struct OBJECT_NAME_INFORMATION {
    Name: UNICODE_STRING,
}}
pub type POBJECT_NAME_INFORMATION = *mut OBJECT_NAME_INFORMATION;
STRUCT! {struct OBJECT_TYPE_INFORMATION {
    TypeName: UNICODE_STRING,
    TotalNumberOfObjects: c_ulong,
    TotalNumberOfHandles: c_ulong,
    TotalPagedPoolUsage: c_ulong,
    TotalNonPagedPoolUsage: c_ulong,
    TotalNamePoolUsage: c_ulong,
    TotalHandleTableUsage: c_ulong,
    HighWaterNumberOfObjects: c_ulong,
    HighWaterNumberOfHandles: c_ulong,
    HighWaterPagedPoolUsage: c_ulong,
    HighWaterNonPagedPoolUsage: c_ulong,
    HighWaterNamePoolUsage: c_ulong,
    HighWaterHandleTableUsage: c_ulong,
    InvalidAttributes: c_ulong,
    GenericMapping: GENERIC_MAPPING,
    ValidAccessMask: c_ulong,
    SecurityRequired: c_uchar,
    MaintainHandleCount: c_uchar,
    TypeIndex: c_uchar,
    ReservedByte: c_char,
    PoolType: c_ulong,
    DefaultPagedPoolCharge: c_ulong,
    DefaultNonPagedPoolCharge: c_ulong,
}}
pub type POBJECT_TYPE_INFORMATION = *mut OBJECT_TYPE_INFORMATION;
STRUCT! {struct OBJECT_TYPES_INFORMATION {
    NumberOfTypes: c_ulong,
}}
pub type POBJECT_TYPES_INFORMATION = *mut OBJECT_TYPES_INFORMATION;
STRUCT! {struct OBJECT_HANDLE_FLAG_INFORMATION {
    Inherit: c_uchar,
    ProtectFromClose: c_uchar,
}}
pub type POBJECT_HANDLE_FLAG_INFORMATION = *mut OBJECT_HANDLE_FLAG_INFORMATION;
EXTERN! {extern "system" {
    fn NtQueryObject(
        Handle: HANDLE,
        ObjectInformationClass: OBJECT_INFORMATION_CLASS,
        ObjectInformation: *mut c_void,
        ObjectInformationLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtSetInformationObject(
        Handle: HANDLE,
        ObjectInformationClass: OBJECT_INFORMATION_CLASS,
        ObjectInformation: *mut c_void,
        ObjectInformationLength: c_ulong,
    ) -> NTSTATUS;
}}
pub const DUPLICATE_CLOSE_SOURCE: u32 = 0x00000001;
pub const DUPLICATE_SAME_ACCESS: u32 = 0x00000002;
pub const DUPLICATE_SAME_ATTRIBUTES: u32 = 0x00000004;
EXTERN! {extern "system" {
    fn NtDuplicateObject(
        SourceProcessHandle: HANDLE,
        SourceHandle: HANDLE,
        TargetProcessHandle: HANDLE,
        TargetHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        HandleAttributes: c_ulong,
        Options: c_ulong,
    ) -> NTSTATUS;
    fn NtMakeTemporaryObject(
        Handle: HANDLE,
    ) -> NTSTATUS;
    fn NtMakePermanentObject(
        Handle: HANDLE,
    ) -> NTSTATUS;
    fn NtSignalAndWaitForSingleObject(
        SignalHandle: HANDLE,
        WaitHandle: HANDLE,
        Alertable: c_uchar,
        Timeout: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtWaitForSingleObject(
        Handle: HANDLE,
        Alertable: c_uchar,
        Timeout: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtWaitForMultipleObjects(
        Count: c_ulong,
        Handles: *mut HANDLE,
        WaitType: WAIT_TYPE,
        Alertable: c_uchar,
        Timeout: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtWaitForMultipleObjects32(
        Count: c_ulong,
        Handles: *mut c_long,
        WaitType: WAIT_TYPE,
        Alertable: c_uchar,
        Timeout: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtSetSecurityObject(
        Handle: HANDLE,
        SecurityInformation: c_ulong,
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
    ) -> NTSTATUS;
    fn NtQuerySecurityObject(
        Handle: HANDLE,
        SecurityInformation: c_ulong,
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        Length: c_ulong,
        LengthNeeded: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtClose(
        Handle: HANDLE,
    ) -> NTSTATUS;
    fn NtCompareObjects(
        FirstObjectHandle: HANDLE,
        SecondObjectHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtCreateDirectoryObject(
        DirectoryHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn NtCreateDirectoryObjectEx(
        DirectoryHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        ShadowDirectoryHandle: HANDLE,
        Flags: c_ulong,
    ) -> NTSTATUS;
    fn NtOpenDirectoryObject(
        DirectoryHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}}
STRUCT! {struct OBJECT_DIRECTORY_INFORMATION {
    Name: UNICODE_STRING,
    TypeName: UNICODE_STRING,
}}
pub type POBJECT_DIRECTORY_INFORMATION = *mut OBJECT_DIRECTORY_INFORMATION;
EXTERN! {extern "system" {
    fn NtQueryDirectoryObject(
        DirectoryHandle: HANDLE,
        Buffer: *mut c_void,
        Length: c_ulong,
        ReturnSingleEntry: c_uchar,
        RestartScan: c_uchar,
        Context: *mut c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtCreatePrivateNamespace(
        NamespaceHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        BoundaryDescriptor: *mut c_void,
    ) -> NTSTATUS;
    fn NtOpenPrivateNamespace(
        NamespaceHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        BoundaryDescriptor: *mut c_void,
    ) -> NTSTATUS;
    fn NtDeletePrivateNamespace(
        NamespaceHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtCreateSymbolicLinkObject(
        LinkHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        LinkTarget: *mut UNICODE_STRING,
    ) -> NTSTATUS;
    fn NtOpenSymbolicLinkObject(
        LinkHandle: *mut HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn NtQuerySymbolicLinkObject(
        LinkHandle: HANDLE,
        LinkTarget: *mut UNICODE_STRING,
        ReturnedLength: *mut c_ulong,
    ) -> NTSTATUS;
}}
