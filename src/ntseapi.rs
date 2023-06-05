use windows_sys::Win32::{
    Foundation::{HANDLE, LUID, NTSTATUS, PSID, UNICODE_STRING},
    Security::{
        AUDIT_EVENT_TYPE, GENERIC_MAPPING, OBJECT_TYPE_LIST, PRIVILEGE_SET,
        PSECURITY_DESCRIPTOR, SID_AND_ATTRIBUTES, TOKEN_DEFAULT_DACL,
        TOKEN_GROUPS, TOKEN_INFORMATION_CLASS, TOKEN_MANDATORY_POLICY,
        TOKEN_OWNER, TOKEN_PRIMARY_GROUP, TOKEN_PRIVILEGES, TOKEN_SOURCE,
        TOKEN_TYPE, TOKEN_USER,
    },
    System::WindowsProgramming::OBJECT_ATTRIBUTES,
};

use crate::{
    ctypes::{__int64, __uint64, c_long, c_uchar, c_ulong, c_ushort, c_void},
    windows_local::shared::ntdef::LARGE_INTEGER,
};

pub const SE_MIN_WELL_KNOWN_PRIVILEGE: c_long = 2;
pub const SE_CREATE_TOKEN_PRIVILEGE: c_long = 2;
pub const SE_ASSIGNPRIMARYTOKEN_PRIVILEGE: c_long = 3;
pub const SE_LOCK_MEMORY_PRIVILEGE: c_long = 4;
pub const SE_INCREASE_QUOTA_PRIVILEGE: c_long = 5;
pub const SE_MACHINE_ACCOUNT_PRIVILEGE: c_long = 6;
pub const SE_TCB_PRIVILEGE: c_long = 7;
pub const SE_SECURITY_PRIVILEGE: c_long = 8;
pub const SE_TAKE_OWNERSHIP_PRIVILEGE: c_long = 9;
pub const SE_LOAD_DRIVER_PRIVILEGE: c_long = 10;
pub const SE_SYSTEM_PROFILE_PRIVILEGE: c_long = 11;
pub const SE_SYSTEMTIME_PRIVILEGE: c_long = 12;
pub const SE_PROF_SINGLE_PROCESS_PRIVILEGE: c_long = 13;
pub const SE_INC_BASE_PRIORITY_PRIVILEGE: c_long = 14;
pub const SE_CREATE_PAGEFILE_PRIVILEGE: c_long = 15;
pub const SE_CREATE_PERMANENT_PRIVILEGE: c_long = 16;
pub const SE_BACKUP_PRIVILEGE: c_long = 17;
pub const SE_RESTORE_PRIVILEGE: c_long = 18;
pub const SE_SHUTDOWN_PRIVILEGE: c_long = 19;
pub const SE_DEBUG_PRIVILEGE: c_long = 20;
pub const SE_AUDIT_PRIVILEGE: c_long = 21;
pub const SE_SYSTEM_ENVIRONMENT_PRIVILEGE: c_long = 22;
pub const SE_CHANGE_NOTIFY_PRIVILEGE: c_long = 23;
pub const SE_REMOTE_SHUTDOWN_PRIVILEGE: c_long = 24;
pub const SE_UNDOCK_PRIVILEGE: c_long = 25;
pub const SE_SYNC_AGENT_PRIVILEGE: c_long = 26;
pub const SE_ENABLE_DELEGATION_PRIVILEGE: c_long = 27;
pub const SE_MANAGE_VOLUME_PRIVILEGE: c_long = 28;
pub const SE_IMPERSONATE_PRIVILEGE: c_long = 29;
pub const SE_CREATE_GLOBAL_PRIVILEGE: c_long = 30;
pub const SE_TRUSTED_CREDMAN_ACCESS_PRIVILEGE: c_long = 31;
pub const SE_RELABEL_PRIVILEGE: c_long = 32;
pub const SE_INC_WORKING_SET_PRIVILEGE: c_long = 33;
pub const SE_TIME_ZONE_PRIVILEGE: c_long = 34;
pub const SE_CREATE_SYMBOLIC_LINK_PRIVILEGE: c_long = 35;
pub const SE_DELEGATE_SESSION_USER_IMPERSONATE_PRIVILEGE: c_long = 36;
pub const SE_MAX_WELL_KNOWN_PRIVILEGE: c_long =
    SE_DELEGATE_SESSION_USER_IMPERSONATE_PRIVILEGE;
pub const TOKEN_SECURITY_ATTRIBUTE_TYPE_INVALID: c_ushort = 0x00;
pub const TOKEN_SECURITY_ATTRIBUTE_TYPE_INT64: c_ushort = 0x01;
pub const TOKEN_SECURITY_ATTRIBUTE_TYPE_UINT64: c_ushort = 0x02;
pub const TOKEN_SECURITY_ATTRIBUTE_TYPE_STRING: c_ushort = 0x03;
pub const TOKEN_SECURITY_ATTRIBUTE_TYPE_FQBN: c_ushort = 0x04;
pub const TOKEN_SECURITY_ATTRIBUTE_TYPE_SID: c_ushort = 0x05;
pub const TOKEN_SECURITY_ATTRIBUTE_TYPE_BOOLEAN: c_ushort = 0x06;
pub const TOKEN_SECURITY_ATTRIBUTE_TYPE_OCTET_STRING: c_ushort = 0x10;
pub const TOKEN_SECURITY_ATTRIBUTE_NON_INHERITABLE: c_ushort = 0x0001;
pub const TOKEN_SECURITY_ATTRIBUTE_VALUE_CASE_SENSITIVE: c_ushort = 0x0002;
pub const TOKEN_SECURITY_ATTRIBUTE_USE_FOR_DENY_ONLY: c_ushort = 0x0004;
pub const TOKEN_SECURITY_ATTRIBUTE_DISABLED_BY_DEFAULT: c_ushort = 0x0008;
pub const TOKEN_SECURITY_ATTRIBUTE_DISABLED: c_ushort = 0x0010;
pub const TOKEN_SECURITY_ATTRIBUTE_MANDATORY: c_ushort = 0x0020;
pub const TOKEN_SECURITY_ATTRIBUTE_COMPARE_IGNORE: c_ushort = 0x0040;
pub const TOKEN_SECURITY_ATTRIBUTE_VALID_FLAGS: c_ushort =
    TOKEN_SECURITY_ATTRIBUTE_NON_INHERITABLE
        | TOKEN_SECURITY_ATTRIBUTE_VALUE_CASE_SENSITIVE
        | TOKEN_SECURITY_ATTRIBUTE_USE_FOR_DENY_ONLY
        | TOKEN_SECURITY_ATTRIBUTE_DISABLED_BY_DEFAULT
        | TOKEN_SECURITY_ATTRIBUTE_DISABLED
        | TOKEN_SECURITY_ATTRIBUTE_MANDATORY;
pub const TOKEN_SECURITY_ATTRIBUTE_CUSTOM_FLAGS: u32 = 0xffff0000;
STRUCT! {struct TOKEN_SECURITY_ATTRIBUTE_FQBN_VALUE {
    Version: __uint64,
    Name: UNICODE_STRING,
}}
pub type PTOKEN_SECURITY_ATTRIBUTE_FQBN_VALUE =
    *mut TOKEN_SECURITY_ATTRIBUTE_FQBN_VALUE;
STRUCT! {struct TOKEN_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    pValue: *mut c_void,
    ValueLength: c_ulong,
}}
pub type PTOKEN_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE =
    *mut TOKEN_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE;
UNION! {union TOKEN_SECURITY_ATTRIBUTE_V1_Values {
    pInt64: *mut __int64,
    pUint64: *mut __uint64,
    pString: *mut UNICODE_STRING,
    pFqbn: PTOKEN_SECURITY_ATTRIBUTE_FQBN_VALUE,
    pOctetString: PTOKEN_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE,
}}
STRUCT! {struct TOKEN_SECURITY_ATTRIBUTE_V1 {
    Name: UNICODE_STRING,
    ValueType: c_ushort,
    Reserved: c_ushort,
    Flags: c_ulong,
    ValueCount: c_ulong,
    Values: TOKEN_SECURITY_ATTRIBUTE_V1_Values,
}}
pub type PTOKEN_SECURITY_ATTRIBUTE_V1 = *mut TOKEN_SECURITY_ATTRIBUTE_V1;
pub const TOKEN_SECURITY_ATTRIBUTES_INFORMATION_VERSION_V1: c_ushort = 1;
pub const TOKEN_SECURITY_ATTRIBUTES_INFORMATION_VERSION: c_ushort =
    TOKEN_SECURITY_ATTRIBUTES_INFORMATION_VERSION_V1;
STRUCT! {struct TOKEN_SECURITY_ATTRIBUTES_INFORMATION {
    Version: c_ushort,
    Reserved: c_ushort,
    AttributeCount: c_ulong,
    pAttributeV1: PTOKEN_SECURITY_ATTRIBUTE_V1,
}}
pub type PTOKEN_SECURITY_ATTRIBUTES_INFORMATION =
    *mut TOKEN_SECURITY_ATTRIBUTES_INFORMATION;
STRUCT! {struct TOKEN_PROCESS_TRUST_LEVEL {
    TrustLevelSid: PSID,
}}
pub type PTOKEN_PROCESS_TRUST_LEVEL = *mut TOKEN_PROCESS_TRUST_LEVEL;
EXTERN! {extern "system" {
    fn NtCreateToken(
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
    fn NtCreateLowBoxToken(
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
    fn NtCreateTokenEx(
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
    fn NtOpenProcessToken(
        ProcessHandle: HANDLE,
        DesiredAccess: c_ulong,
        TokenHandle: *mut HANDLE,
    ) -> NTSTATUS;
    fn NtOpenProcessTokenEx(
        ProcessHandle: HANDLE,
        DesiredAccess: c_ulong,
        HandleAttributes: c_ulong,
        TokenHandle: *mut HANDLE,
    ) -> NTSTATUS;
    fn NtOpenThreadToken(
        ThreadHandle: HANDLE,
        DesiredAccess: c_ulong,
        OpenAsSelf: c_uchar,
        TokenHandle: *mut HANDLE,
    ) -> NTSTATUS;
    fn NtOpenThreadTokenEx(
        ThreadHandle: HANDLE,
        DesiredAccess: c_ulong,
        OpenAsSelf: c_uchar,
        HandleAttributes: c_ulong,
        TokenHandle: *mut HANDLE,
    ) -> NTSTATUS;
    fn NtDuplicateToken(
        ExistingTokenHandle: HANDLE,
        DesiredAccess: c_ulong,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        EffectiveOnly: c_uchar,
        TokenType: TOKEN_TYPE,
        NewTokenHandle: *mut HANDLE,
    ) -> NTSTATUS;
    fn NtQueryInformationToken(
        TokenHandle: HANDLE,
        TokenInformationClass: TOKEN_INFORMATION_CLASS,
        TokenInformation: *mut c_void,
        TokenInformationLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtSetInformationToken(
        TokenHandle: HANDLE,
        TokenInformationClass: TOKEN_INFORMATION_CLASS,
        TokenInformation: *mut c_void,
        TokenInformationLength: c_ulong,
    ) -> NTSTATUS;
    fn NtAdjustPrivilegesToken(
        TokenHandle: HANDLE,
        DisableAllPrivileges: c_uchar,
        NewState: *mut TOKEN_PRIVILEGES,
        BufferLength: c_ulong,
        PreviousState: *mut TOKEN_PRIVILEGES,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtAdjustGroupsToken(
        TokenHandle: HANDLE,
        ResetToDefault: c_uchar,
        NewState: *mut TOKEN_GROUPS,
        BufferLength: c_ulong,
        PreviousState: *mut TOKEN_GROUPS,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtAdjustTokenClaimsAndDeviceGroups(
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
    fn NtFilterToken(
        ExistingTokenHandle: HANDLE,
        Flags: c_ulong,
        SidsToDisable: *mut TOKEN_GROUPS,
        PrivilegesToDelete: *mut TOKEN_PRIVILEGES,
        RestrictedSids: *mut TOKEN_GROUPS,
        NewTokenHandle: *mut HANDLE,
    ) -> NTSTATUS;
    fn NtFilterTokenEx(
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
    fn NtCompareTokens(
        FirstTokenHandle: HANDLE,
        SecondTokenHandle: HANDLE,
        Equal: *mut c_uchar,
    ) -> NTSTATUS;
    fn NtPrivilegeCheck(
        ClientToken: HANDLE,
        RequiredPrivileges: *mut PRIVILEGE_SET,
        Result: *mut c_uchar,
    ) -> NTSTATUS;
    fn NtImpersonateAnonymousToken(
        ThreadHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtQuerySecurityAttributesToken(
        TokenHandle: HANDLE,
        Attributes: *mut UNICODE_STRING,
        NumberOfAttributes: c_ulong,
        Buffer: *mut c_void,
        Length: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtAccessCheck(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        ClientToken: HANDLE,
        DesiredAccess: c_ulong,
        GenericMapping: *mut GENERIC_MAPPING,
        PrivilegeSet: *mut PRIVILEGE_SET,
        PrivilegeSetLength: *mut c_ulong,
        GrantedAccess: *mut c_ulong,
        AccessStatus: *mut NTSTATUS,
    ) -> NTSTATUS;
    fn NtAccessCheckByType(
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
    fn NtAccessCheckByTypeResultList(
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
    fn NtSetCachedSigningLevel(
        Flags: c_ulong,
        InputSigningLevel: c_uchar,
        SourceFiles: *mut HANDLE,
        SourceFileCount: c_ulong,
        TargetFile: HANDLE,
    ) -> NTSTATUS;
    fn NtGetCachedSigningLevel(
        File: HANDLE,
        Flags: *mut c_ulong,
        SigningLevel: *mut c_uchar,
        Thumbprint: *mut c_uchar,
        ThumbprintSize: *mut c_ulong,
        ThumbprintAlgorithm: *mut c_ulong,
    ) -> NTSTATUS;
    fn NtAccessCheckAndAuditAlarm(
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
    fn NtAccessCheckByTypeAndAuditAlarm(
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
    fn NtAccessCheckByTypeResultListAndAuditAlarm(
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
    fn NtAccessCheckByTypeResultListAndAuditAlarmByHandle(
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
    fn NtOpenObjectAuditAlarm(
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
    fn NtPrivilegeObjectAuditAlarm(
        SubsystemName: *mut UNICODE_STRING,
        HandleId: *mut c_void,
        ClientToken: HANDLE,
        DesiredAccess: c_ulong,
        Privileges: *mut PRIVILEGE_SET,
        AccessGranted: c_uchar,
    ) -> NTSTATUS;
    fn NtCloseObjectAuditAlarm(
        SubsystemName: *mut UNICODE_STRING,
        HandleId: *mut c_void,
        GenerateOnClose: c_uchar,
    ) -> NTSTATUS;
    fn NtDeleteObjectAuditAlarm(
        SubsystemName: *mut UNICODE_STRING,
        HandleId: *mut c_void,
        GenerateOnClose: c_uchar,
    ) -> NTSTATUS;
    fn NtPrivilegedServiceAuditAlarm(
        SubsystemName: *mut UNICODE_STRING,
        ServiceName: *mut UNICODE_STRING,
        ClientToken: HANDLE,
        Privileges: *mut PRIVILEGE_SET,
        AccessGranted: c_uchar,
    ) -> NTSTATUS;
}}
