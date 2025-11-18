use core::ptr::null_mut;

use windows_sys::Win32::{
    Foundation::{FILETIME, HANDLE, HWND, PSID, UNICODE_STRING},
    Storage::FileSystem::STANDARD_RIGHTS_REQUIRED,
};

use crate::{
    ctypes::{c_char, c_long, c_uchar, c_ulong, c_ushort, c_void, wchar_t},
    ntrtl::RTL_TIME_ZONE_INFORMATION,
    windows_local::shared::ntdef::LARGE_INTEGER,
};

pub const WINSTATION_QUERY: u32 = 0x00000001;
pub const WINSTATION_SET: u32 = 0x00000002;
pub const WINSTATION_RESET: u32 = 0x00000004;
pub const WINSTATION_VIRTUAL: u32 = 0x00000008;
pub const WINSTATION_SHADOW: u32 = 0x00000010;
pub const WINSTATION_LOGON: u32 = 0x00000020;
pub const WINSTATION_LOGOFF: u32 = 0x00000040;
pub const WINSTATION_MSG: u32 = 0x00000080;
pub const WINSTATION_CONNECT: u32 = 0x00000100;
pub const WINSTATION_DISCONNECT: u32 = 0x00000200;
pub const WINSTATION_GUEST_ACCESS: u32 = WINSTATION_LOGON;
pub const WINSTATION_CURRENT_GUEST_ACCESS: u32 =
    WINSTATION_VIRTUAL | WINSTATION_LOGOFF;
pub const WINSTATION_USER_ACCESS: u32 =
    WINSTATION_GUEST_ACCESS | WINSTATION_QUERY | WINSTATION_CONNECT;
pub const WINSTATION_CURRENT_USER_ACCESS: u32 = WINSTATION_SET
    | WINSTATION_RESET
    | WINSTATION_VIRTUAL
    | WINSTATION_LOGOFF
    | WINSTATION_DISCONNECT;
pub const WINSTATION_ALL_ACCESS: u32 = STANDARD_RIGHTS_REQUIRED
    | WINSTATION_QUERY
    | WINSTATION_SET
    | WINSTATION_RESET
    | WINSTATION_VIRTUAL
    | WINSTATION_SHADOW
    | WINSTATION_LOGON
    | WINSTATION_MSG
    | WINSTATION_CONNECT
    | WINSTATION_DISCONNECT;
pub const WDPREFIX_LENGTH: usize = 12;
pub const CALLBACK_LENGTH: usize = 50;
pub const DLLNAME_LENGTH: usize = 32;
pub const CDNAME_LENGTH: usize = 32;
pub const WDNAME_LENGTH: usize = 32;
pub const PDNAME_LENGTH: usize = 32;
pub const DEVICENAME_LENGTH: usize = 128;
pub const MODEMNAME_LENGTH: usize = DEVICENAME_LENGTH;
pub const STACK_ADDRESS_LENGTH: usize = 128;
pub const MAX_BR_NAME: usize = 65;
pub const DIRECTORY_LENGTH: usize = 256;
pub const INITIALPROGRAM_LENGTH: usize = 256;
pub const USERNAME_LENGTH: usize = 20;
pub const DOMAIN_LENGTH: usize = 17;
pub const PASSWORD_LENGTH: usize = 14;
pub const NASISPECIFICNAME_LENGTH: usize = 14;
pub const NASIUSERNAME_LENGTH: usize = 47;
pub const NASIPASSWORD_LENGTH: usize = 24;
pub const NASISESSIONNAME_LENGTH: usize = 16;
pub const NASIFILESERVER_LENGTH: usize = 47;
pub const CLIENTDATANAME_LENGTH: usize = 7;
pub const CLIENTNAME_LENGTH: usize = 20;
pub const CLIENTADDRESS_LENGTH: usize = 30;
pub const IMEFILENAME_LENGTH: usize = 32;
pub const CLIENTLICENSE_LENGTH: usize = 32;
pub const CLIENTMODEM_LENGTH: usize = 40;
pub const CLIENT_PRODUCT_ID_LENGTH: usize = 32;
pub const MAX_COUNTER_EXTENSIONS: u32 = 2;
pub const WINSTATIONNAME_LENGTH: usize = 32;
pub const TERMSRV_TOTAL_SESSIONS: u32 = 1;
pub const TERMSRV_DISC_SESSIONS: u32 = 2;
pub const TERMSRV_RECON_SESSIONS: u32 = 3;
pub const TERMSRV_CURRENT_ACTIVE_SESSIONS: u32 = 4;
pub const TERMSRV_CURRENT_DISC_SESSIONS: u32 = 5;
pub const TERMSRV_PENDING_SESSIONS: u32 = 6;
pub const TERMSRV_SUCC_TOTAL_LOGONS: u32 = 7;
pub const TERMSRV_SUCC_LOCAL_LOGONS: u32 = 8;
pub const TERMSRV_SUCC_REMOTE_LOGONS: u32 = 9;
pub const TERMSRV_SUCC_SESSION0_LOGONS: u32 = 10;
pub const TERMSRV_CURRENT_TERMINATING_SESSIONS: u32 = 11;
pub const TERMSRV_CURRENT_LOGGEDON_SESSIONS: u32 = 12;
pub type PTS_TIME_ZONE_INFORMATION = *mut RTL_TIME_ZONE_INFORMATION;
pub type TS_TIME_ZONE_INFORMATION = RTL_TIME_ZONE_INFORMATION;
pub type WINSTATIONNAME = [wchar_t; WINSTATIONNAME_LENGTH + 1];
STRUCT! {struct VARDATA_WIRE {
    Size: c_ushort,
    Offset: c_ushort,
}}
pub type PVARDATA_WIRE = *mut VARDATA_WIRE;
ENUM! {enum WINSTATIONSTATECLASS {
    State_Active = 0,
    State_Connected = 1,
    State_ConnectQuery = 2,
    State_Shadow = 3,
    State_Disconnected = 4,
    State_Idle = 5,
    State_Listen = 6,
    State_Reset = 7,
    State_Down = 8,
    State_Init = 9,
}}
UNION! {union SESSIONIDW_u {
    SessionId: c_ulong,
    LogonId: c_ulong,
}}
STRUCT! {struct SESSIONIDW {
    u: SESSIONIDW_u,
    WinStationName: WINSTATIONNAME,
    State: WINSTATIONSTATECLASS,
}}
pub type PSESSIONIDW = *mut SESSIONIDW;
ENUM! {enum WINSTATIONINFOCLASS {
    WinStationCreateData = 0,
    WinStationConfiguration = 1,
    WinStationPdParams = 2,
    WinStationWd = 3,
    WinStationPd = 4,
    WinStationPrinter = 5,
    WinStationClient = 6,
    WinStationModules = 7,
    WinStationInformation = 8,
    WinStationTrace = 9,
    WinStationBeep = 10,
    WinStationEncryptionOff = 11,
    WinStationEncryptionPerm = 12,
    WinStationNtSecurity = 13,
    WinStationUserToken = 14,
    WinStationUnused1 = 15,
    WinStationVideoData = 16,
    WinStationInitialProgram = 17,
    WinStationCd = 18,
    WinStationSystemTrace = 19,
    WinStationVirtualData = 20,
    WinStationClientData = 21,
    WinStationSecureDesktopEnter = 22,
    WinStationSecureDesktopExit = 23,
    WinStationLoadBalanceSessionTarget = 24,
    WinStationLoadIndicator = 25,
    WinStationShadowInfo = 26,
    WinStationDigProductId = 27,
    WinStationLockedState = 28,
    WinStationRemoteAddress = 29,
    WinStationIdleTime = 30,
    WinStationLastReconnectType = 31,
    WinStationDisallowAutoReconnect = 32,
    WinStationMprNotifyInfo = 33,
    WinStationExecSrvSystemPipe = 34,
    WinStationSmartCardAutoLogon = 35,
    WinStationIsAdminLoggedOn = 36,
    WinStationReconnectedFromId = 37,
    WinStationEffectsPolicy = 38,
    WinStationType = 39,
    WinStationInformationEx = 40,
    WinStationValidationInfo = 41,
}}
STRUCT! {struct WINSTATIONCREATE {
    Bitfields: c_ulong,
    MaxInstanceCount: c_ulong,
}}
BITFIELD! {WINSTATIONCREATE Bitfields: c_ulong [
    fEnableWinStation set_fEnableWinStation[0..1],
]}
pub type PWINSTATIONCREATE = *mut WINSTATIONCREATE;
STRUCT! {struct WINSTACONFIGWIRE {
    Comment: [wchar_t; 61],
    OEMId: [c_char; 4],
    UserConfig: VARDATA_WIRE,
    NewFields: VARDATA_WIRE,
}}
pub type PWINSTACONFIGWIRE = *mut WINSTACONFIGWIRE;
ENUM! {enum CALLBACKCLASS {
    Callback_Disable = 0,
    Callback_Roving = 1,
    Callback_Fixed = 2,
}}
ENUM! {enum SHADOWCLASS {
    Shadow_Disable = 0,
    Shadow_EnableInputNotify = 1,
    Shadow_EnableInputNoNotify = 2,
    Shadow_EnableNoInputNotify = 3,
    Shadow_EnableNoInputNoNotify = 4,
}}
STRUCT! {struct USERCONFIG {
    Bitfields: c_ulong,
    Bitfields2: c_ulong,
    UserName: [wchar_t; USERNAME_LENGTH + 1],
    Domain: [wchar_t; DOMAIN_LENGTH + 1],
    Password: [wchar_t; PASSWORD_LENGTH + 1],
    WorkDirectory: [wchar_t; DIRECTORY_LENGTH + 1],
    InitialProgram: [wchar_t; INITIALPROGRAM_LENGTH + 1],
    CallbackNumber: [wchar_t; CALLBACK_LENGTH + 1],
    Callback: CALLBACKCLASS,
    Shadow: SHADOWCLASS,
    MaxConnectionTime: c_ulong,
    MaxDisconnectionTime: c_ulong,
    MaxIdleTime: c_ulong,
    KeyboardLayout: c_ulong,
    MinEncryptionLevel: c_uchar,
    NWLogonServer: [wchar_t; NASIFILESERVER_LENGTH + 1],
    PublishedName: [wchar_t; MAX_BR_NAME],
    WFProfilePath: [wchar_t; DIRECTORY_LENGTH + 1],
    WFHomeDir: [wchar_t; DIRECTORY_LENGTH + 1],
    WFHomeDirDrive: [wchar_t; 4],
}}
BITFIELD! {USERCONFIG Bitfields: c_ulong [
    fInheritAutoLogon set_fInheritAutoLogon[0..1],
    fInheritResetBroken set_fInheritResetBroken[1..2],
    fInheritReconnectSame set_fInheritReconnectSame[2..3],
    fInheritInitialProgram set_fInheritInitialProgram[3..4],
    fInheritCallback set_fInheritCallback[4..5],
    fInheritCallbackNumber set_fInheritCallbackNumber[5..6],
    fInheritShadow set_fInheritShadow[6..7],
    fInheritMaxSessionTime set_fInheritMaxSessionTime[7..8],
    fInheritMaxDisconnectionTime set_fInheritMaxDisconnectionTime[8..9],
    fInheritMaxIdleTime set_fInheritMaxIdleTime[9..10],
    fInheritAutoClient set_fInheritAutoClient[10..11],
    fInheritSecurity set_fInheritSecurity[11..12],
    fPromptForPassword set_fPromptForPassword[12..13],
    fResetBroken set_fResetBroken[13..14],
    fReconnectSame set_fReconnectSame[14..15],
    fLogonDisabled set_fLogonDisabled[15..16],
    fWallPaperDisabled set_fWallPaperDisabled[16..17],
    fAutoClientDrives set_fAutoClientDrives[17..18],
    fAutoClientLpts set_fAutoClientLpts[18..19],
    fForceClientLptDef set_fForceClientLptDef[19..20],
    fRequireEncryption set_fRequireEncryption[20..21],
    fDisableEncryption set_fDisableEncryption[21..22],
    fUnused1 set_fUnused1[22..23],
    fHomeDirectoryMapRoot set_fHomeDirectoryMapRoot[23..24],
    fUseDefaultGina set_fUseDefaultGina[24..25],
    fCursorBlinkDisabled set_fCursorBlinkDisabled[25..26],
    fPublishedApp set_fPublishedApp[26..27],
    fHideTitleBar set_fHideTitleBar[27..28],
    fMaximize set_fMaximize[28..29],
    fDisableCpm set_fDisableCpm[29..30],
    fDisableCdm set_fDisableCdm[30..31],
    fDisableCcm set_fDisableCcm[31..32],
]}
BITFIELD! {USERCONFIG Bitfields2: c_ulong [
    fDisableLPT set_fDisableLPT[0..1],
    fDisableClip set_fDisableClip[1..2],
    fDisableExe set_fDisableExe[2..3],
    fDisableCam set_fDisableCam[3..4],
    fDisableAutoReconnect set_fDisableAutoReconnect[4..5],
    ColorDepth set_ColorDepth[5..6],
    fInheritColorDepth set_fInheritColorDepth[6..7],
    fErrorInvalidProfile set_fErrorInvalidProfile[7..8],
    fPasswordIsScPin set_fPasswordIsScPin[8..9],
    fDisablePNPRedir set_fDisablePNPRedir[9..10],
]}
pub type PUSERCONFIG = *mut USERCONFIG;
ENUM! {enum SDCLASS {
    SdNone = 0,
    SdConsole = 1,
    SdNetwork = 2,
    SdAsync = 3,
    SdOemTransport = 4,
}}
pub type DEVICENAME = [wchar_t; DEVICENAME_LENGTH + 1];
pub type MODEMNAME = [wchar_t; MODEMNAME_LENGTH + 1];
pub type NASISPECIFICNAME = [wchar_t; NASISPECIFICNAME_LENGTH + 1];
pub type NASIUSERNAME = [wchar_t; NASIUSERNAME_LENGTH + 1];
pub type NASIPASSWORD = [wchar_t; NASIPASSWORD_LENGTH + 1];
pub type NASISESIONNAME = [wchar_t; NASISESSIONNAME_LENGTH + 1];
pub type NASIFILESERVER = [wchar_t; NASIFILESERVER_LENGTH + 1];
pub type WDNAME = [wchar_t; WDNAME_LENGTH + 1];
pub type WDPREFIX = [wchar_t; WDPREFIX_LENGTH + 1];
pub type CDNAME = [wchar_t; CDNAME_LENGTH + 1];
pub type DLLNAME = [wchar_t; DLLNAME_LENGTH + 1];
pub type PDNAME = [wchar_t; PDNAME_LENGTH + 1];
STRUCT! {struct NETWORKCONFIG {
    LanAdapter: c_long,
    NetworkName: DEVICENAME,
    Flags: c_ulong,
}}
pub type PNETWORKCONFIG = *mut NETWORKCONFIG;
ENUM! {enum FLOWCONTROLCLASS {
    FlowControl_None = 0,
    FlowControl_Hardware = 1,
    FlowControl_Software = 2,
}}
ENUM! {enum RECEIVEFLOWCONTROLCLASS {
    ReceiveFlowControl_None = 0,
    ReceiveFlowControl_RTS = 1,
    ReceiveFlowControl_DTR = 2,
}}
ENUM! {enum TRANSMITFLOWCONTROLCLASS {
    TransmitFlowControl_None = 0,
    TransmitFlowControl_CTS = 1,
    TransmitFlowControl_DSR = 2,
}}
ENUM! {enum ASYNCCONNECTCLASS {
    Connect_CTS = 0,
    Connect_DSR = 1,
    Connect_RI = 2,
    Connect_DCD = 3,
    Connect_FirstChar = 4,
    Connect_Perm = 5,
}}
STRUCT! {struct FLOWCONTROLCONFIG {
    Bitfields: c_ulong,
    XonChar: c_char,
    XoffChar: c_char,
    Type: FLOWCONTROLCLASS,
    HardwareReceive: RECEIVEFLOWCONTROLCLASS,
    HardwareTransmit: TRANSMITFLOWCONTROLCLASS,
}}
BITFIELD! {FLOWCONTROLCONFIG Bitfields: c_ulong [
    fEnableSoftwareTx set_fEnableSoftwareTx[0..1],
    fEnableSoftwareRx set_fEnableSoftwareRx[1..2],
    fEnableDTR set_fEnableDTR[2..3],
    fEnableRTS set_fEnableRTS[3..4],
]}
pub type PFLOWCONTROLCONFIG = *mut FLOWCONTROLCONFIG;
STRUCT! {struct CONNECTCONFIG {
    Type: ASYNCCONNECTCLASS,
    Bitfields: c_ulong,
}}
BITFIELD! {CONNECTCONFIG Bitfields: c_ulong [
    fEnableBreakDisconnect set_fEnableBreakDisconnect[0..1],
]}
pub type PCONNECTCONFIG = *mut CONNECTCONFIG;
STRUCT! {struct ASYNCCONFIG {
    DeviceName: DEVICENAME,
    ModemName: MODEMNAME,
    BaudRate: c_ulong,
    Parity: c_ulong,
    StopBits: c_ulong,
    ByteSize: c_ulong,
    Bitfields: c_ulong,
    FlowControl: FLOWCONTROLCONFIG,
    Connect: CONNECTCONFIG,
}}
BITFIELD! {ASYNCCONFIG Bitfields: c_ulong [
    fEnableDsrSensitivity set_fEnableDsrSensitivity[0..1],
    fConnectionDriver set_fConnectionDriver[1..2],
]}
pub type PASYNCCONFIG = *mut ASYNCCONFIG;
STRUCT! {struct NASICONFIG {
    SpecificName: NASISPECIFICNAME,
    UserName: NASIUSERNAME,
    PassWord: NASIPASSWORD,
    SessionName: NASISESIONNAME,
    FileServer: NASIFILESERVER,
    GlobalSession: c_uchar,
}}
pub type PNASICONFIG = *mut NASICONFIG;
STRUCT! {struct OEMTDCONFIG {
    Adapter: c_long,
    DeviceName: DEVICENAME,
    Flags: c_ulong,
}}
pub type POEMTDCONFIG = *mut OEMTDCONFIG;
UNION! {union PDPARAMS_u {
    Network: NETWORKCONFIG,
    Async: ASYNCCONFIG,
    Nasi: NASICONFIG,
    OemTd: OEMTDCONFIG,
}}
STRUCT! {struct PDPARAMS {
    SdClass: SDCLASS,
    u: PDPARAMS_u,
}}
pub type PPDPARAMS = *mut PDPARAMS;
STRUCT! {struct WDCONFIG {
    WdName: WDNAME,
    WdDLL: DLLNAME,
    WsxDLL: DLLNAME,
    WdFlag: c_ulong,
    WdInputBufferLength: c_ulong,
    CfgDLL: DLLNAME,
    WdPrefix: WDPREFIX,
}}
pub type PWDCONFIG = *mut WDCONFIG;
STRUCT! {struct PDCONFIG2 {
    PdName: PDNAME,
    SdClass: SDCLASS,
    PdDLL: DLLNAME,
    PdFlag: c_ulong,
    OutBufLength: c_ulong,
    OutBufCount: c_ulong,
    OutBufDelay: c_ulong,
    InteractiveDelay: c_ulong,
    PortNumber: c_ulong,
    KeepAliveTimeout: c_ulong,
}}
pub type PPDCONFIG2 = *mut PDCONFIG2;
STRUCT! {struct WINSTATIONCLIENT {
    Bitfields: c_ulong,
    ClientName: [wchar_t; CLIENTNAME_LENGTH + 1],
    Domain: [wchar_t; DOMAIN_LENGTH + 1],
    UserName: [wchar_t; USERNAME_LENGTH + 1],
    Password: [wchar_t; PASSWORD_LENGTH + 1],
    WorkDirectory: [wchar_t; DIRECTORY_LENGTH + 1],
    InitialProgram: [wchar_t; INITIALPROGRAM_LENGTH + 1],
    SerialNumber: c_ulong,
    EncryptionLevel: c_uchar,
    ClientAddressFamily: c_ulong,
    ClientAddress: [wchar_t; CLIENTADDRESS_LENGTH + 1],
    HRes: c_ushort,
    VRes: c_ushort,
    ColorDepth: c_ushort,
    ProtocolType: c_ushort,
    KeyboardLayout: c_ulong,
    KeyboardType: c_ulong,
    KeyboardSubType: c_ulong,
    KeyboardFunctionKey: c_ulong,
    ImeFileName: [wchar_t; IMEFILENAME_LENGTH + 1],
    ClientDirectory: [wchar_t; DIRECTORY_LENGTH + 1],
    ClientLicense: [wchar_t; CLIENTLICENSE_LENGTH + 1],
    ClientModem: [wchar_t; CLIENTMODEM_LENGTH + 1],
    ClientBuildNumber: c_ulong,
    ClientHardwareId: c_ulong,
    ClientProductId: c_ushort,
    OutBufCountHost: c_ushort,
    OutBufCountClient: c_ushort,
    OutBufLength: c_ushort,
    AudioDriverName: [wchar_t; 9],
    ClientTimeZone: TS_TIME_ZONE_INFORMATION,
    ClientSessionId: c_ulong,
    ClientDigProductId: [wchar_t; CLIENT_PRODUCT_ID_LENGTH],
    PerformanceFlags: c_ulong,
    ActiveInputLocale: c_ulong,
}}
BITFIELD! {WINSTATIONCLIENT Bitfields: c_ulong [
    fTextOnly set_fTextOnly[0..1],
    fDisableCtrlAltDel set_fDisableCtrlAltDel[1..2],
    fMouse set_fMouse[2..3],
    fDoubleClickDetect set_fDoubleClickDetect[3..4],
    fINetClient set_fINetClient[4..5],
    fPromptForPassword set_fPromptForPassword[5..6],
    fMaximizeShell set_fMaximizeShell[6..7],
    fEnableWindowsKey set_fEnableWindowsKey[7..8],
    fRemoteConsoleAudio set_fRemoteConsoleAudio[8..9],
    fPasswordIsScPin set_fPasswordIsScPin[9..10],
    fNoAudioPlayback set_fNoAudioPlayback[10..11],
    fUsingSavedCreds set_fUsingSavedCreds[11..12],
]}
pub type PWINSTATIONCLIENT = *mut WINSTATIONCLIENT;
STRUCT! {struct TSHARE_COUNTERS {
    Reserved: c_ulong,
}}
pub type PTSHARE_COUNTERS = *mut TSHARE_COUNTERS;
UNION! {union PROTOCOLCOUNTERS_Specific {
    TShareCounters: TSHARE_COUNTERS,
    Reserved: [c_ulong; 100],
}}
STRUCT! {struct PROTOCOLCOUNTERS {
    WdBytes: c_ulong,
    WdFrames: c_ulong,
    WaitForOutBuf: c_ulong,
    Frames: c_ulong,
    Bytes: c_ulong,
    CompressedBytes: c_ulong,
    CompressFlushes: c_ulong,
    Errors: c_ulong,
    Timeouts: c_ulong,
    AsyncFramingError: c_ulong,
    AsyncOverrunError: c_ulong,
    AsyncOverflowError: c_ulong,
    AsyncParityError: c_ulong,
    TdErrors: c_ulong,
    ProtocolType: c_ushort,
    Length: c_ushort,
    Specific: PROTOCOLCOUNTERS_Specific,
}}
pub type PPROTOCOLCOUNTERS = *mut PROTOCOLCOUNTERS;
STRUCT! {struct THINWIRECACHE {
    CacheReads: c_ulong,
    CacheHits: c_ulong,
}}
pub type PTHINWIRECACHE = *mut THINWIRECACHE;
pub const MAX_THINWIRECACHE: usize = 4;
STRUCT! {struct RESERVED_CACHE {
    ThinWireCache: [THINWIRECACHE; MAX_THINWIRECACHE],
}}
pub type PRESERVED_CACHE = *mut RESERVED_CACHE;
STRUCT! {struct TSHARE_CACHE {
    Reserved: c_ulong,
}}
pub type PTSHARE_CACHE = *mut TSHARE_CACHE;
UNION! {union CACHE_STATISTICS_Specific {
    ReservedCacheStats: RESERVED_CACHE,
    TShareCacheStats: TSHARE_CACHE,
    Reserved: [c_ulong; 20],
}}
STRUCT! {struct CACHE_STATISTICS {
    ProtocolType: c_ushort,
    Length: c_ushort,
    Specific: CACHE_STATISTICS_Specific,
}}
pub type PCACHE_STATISTICS = *mut CACHE_STATISTICS;
STRUCT! {struct PROTOCOLSTATUS {
    Output: PROTOCOLCOUNTERS,
    Input: PROTOCOLCOUNTERS,
    Cache: CACHE_STATISTICS,
    AsyncSignal: c_ulong,
    AsyncSignalMask: c_ulong,
}}
pub type PPROTOCOLSTATUS = *mut PROTOCOLSTATUS;
STRUCT! {struct WINSTATIONINFORMATION {
    ConnectState: WINSTATIONSTATECLASS,
    WinStationName: WINSTATIONNAME,
    LogonId: c_ulong,
    ConnectTime: LARGE_INTEGER,
    DisconnectTime: LARGE_INTEGER,
    LastInputTime: LARGE_INTEGER,
    LogonTime: LARGE_INTEGER,
    Status: PROTOCOLSTATUS,
    Domain: [wchar_t; DOMAIN_LENGTH + 1],
    UserName: [wchar_t; USERNAME_LENGTH + 1],
    CurrentTime: LARGE_INTEGER,
}}
pub type PWINSTATIONINFORMATION = *mut WINSTATIONINFORMATION;
STRUCT! {struct WINSTATIONUSERTOKEN {
    ProcessId: HANDLE,
    ThreadId: HANDLE,
    UserToken: HANDLE,
}}
pub type PWINSTATIONUSERTOKEN = *mut WINSTATIONUSERTOKEN;
STRUCT! {struct WINSTATIONVIDEODATA {
    HResolution: c_ushort,
    VResolution: c_ushort,
    fColorDepth: c_ushort,
}}
pub type PWINSTATIONVIDEODATA = *mut WINSTATIONVIDEODATA;
ENUM! {enum CDCLASS {
    CdNone = 0,
    CdModem = 1,
    CdClass_Maximum = 2,
}}
STRUCT! {struct CDCONFIG {
    CdClass: CDCLASS,
    CdName: CDNAME,
    CdDLL: DLLNAME,
    CdFlag: c_ulong,
}}
pub type PCDCONFIG = *mut CDCONFIG;
pub type CLIENTDATANAME = [c_char; CLIENTDATANAME_LENGTH + 1];
pub type PCLIENTDATANAME = *mut c_char;
STRUCT! {struct WINSTATIONCLIENTDATA {
    DataName: CLIENTDATANAME,
    fUnicodeData: c_uchar,
}}
pub type PWINSTATIONCLIENTDATA = *mut WINSTATIONCLIENTDATA;
ENUM! {enum LOADFACTORTYPE {
    ErrorConstraint = 0,
    PagedPoolConstraint = 1,
    NonPagedPoolConstraint = 2,
    AvailablePagesConstraint = 3,
    SystemPtesConstraint = 4,
    CPUConstraint = 5,
}}
STRUCT! {struct WINSTATIONLOADINDICATORDATA {
    RemainingSessionCapacity: c_ulong,
    LoadFactor: LOADFACTORTYPE,
    TotalSessions: c_ulong,
    DisconnectedSessions: c_ulong,
    IdleCPU: LARGE_INTEGER,
    TotalCPU: LARGE_INTEGER,
    RawSessionCapacity: c_ulong,
    reserved: [c_ulong; 9],
}}
pub type PWINSTATIONLOADINDICATORDATA = *mut WINSTATIONLOADINDICATORDATA;
ENUM! {enum SHADOWSTATECLASS {
    State_NoShadow = 0,
    State_Shadowing = 1,
    State_Shadowed = 2,
}}
STRUCT! {struct WINSTATIONSHADOW {
    ShadowState: SHADOWSTATECLASS,
    ShadowClass: SHADOWCLASS,
    SessionId: c_ulong,
    ProtocolType: c_ulong,
}}
pub type PWINSTATIONSHADOW = *mut WINSTATIONSHADOW;
STRUCT! {struct WINSTATIONPRODID {
    DigProductId: [wchar_t; CLIENT_PRODUCT_ID_LENGTH],
    ClientDigProductId: [wchar_t; CLIENT_PRODUCT_ID_LENGTH],
    OuterMostDigProductId: [wchar_t; CLIENT_PRODUCT_ID_LENGTH],
    CurrentSessionId: c_ulong,
    ClientSessionId: c_ulong,
    OuterMostSessionId: c_ulong,
}}
pub type PWINSTATIONPRODID = *mut WINSTATIONPRODID;
STRUCT! {struct WINSTATIONREMOTEADDRESS_u_ipv4 {
    sin_port: c_ushort,
    sin_addr: c_ulong,
    sin_zero: [c_uchar; 8],
}}
STRUCT! {struct WINSTATIONREMOTEADDRESS_u_ipv6 {
    sin6_port: c_ushort,
    sin6_flowinfo: c_ulong,
    sin6_addr: [c_ushort; 8],
    sin6_scope_id: c_ulong,
}}
UNION! {union WINSTATIONREMOTEADDRESS_u {
    ipv4: WINSTATIONREMOTEADDRESS_u_ipv4,
    ipv6: WINSTATIONREMOTEADDRESS_u_ipv6,
}}
STRUCT! {struct WINSTATIONREMOTEADDRESS {
    sin_family: c_ushort,
    u: WINSTATIONREMOTEADDRESS_u,
}}
pub type PWINSTATIONREMOTEADDRESS = *mut WINSTATIONREMOTEADDRESS;
STRUCT! {struct WINSTATIONINFORMATIONEX_LEVEL1 {
    SessionId: c_ulong,
    SessionState: WINSTATIONSTATECLASS,
    SessionFlags: c_long,
    WinStationName: WINSTATIONNAME,
    UserName: [wchar_t; USERNAME_LENGTH + 1],
    DomainName: [wchar_t; DOMAIN_LENGTH + 1],
    LogonTime: LARGE_INTEGER,
    ConnectTime: LARGE_INTEGER,
    DisconnectTime: LARGE_INTEGER,
    LastInputTime: LARGE_INTEGER,
    CurrentTime: LARGE_INTEGER,
    ProtocolStatus: PROTOCOLSTATUS,
}}
pub type PWINSTATIONINFORMATIONEX_LEVEL1 = *mut WINSTATIONINFORMATIONEX_LEVEL1;
STRUCT! {struct WINSTATIONINFORMATIONEX_LEVEL2 {
    SessionId: c_ulong,
    SessionState: WINSTATIONSTATECLASS,
    SessionFlags: c_long,
    WinStationName: WINSTATIONNAME,
    SamCompatibleUserName: [wchar_t; USERNAME_LENGTH + 1],
    SamCompatibleDomainName: [wchar_t; DOMAIN_LENGTH + 1],
    LogonTime: LARGE_INTEGER,
    ConnectTime: LARGE_INTEGER,
    DisconnectTime: LARGE_INTEGER,
    LastInputTime: LARGE_INTEGER,
    CurrentTime: LARGE_INTEGER,
    ProtocolStatus: PROTOCOLSTATUS,
    UserName: [wchar_t; 257],
    DomainName: [wchar_t; 256],
}}
pub type PWINSTATIONINFORMATIONEX_LEVEL2 = *mut WINSTATIONINFORMATIONEX_LEVEL2;
UNION! {union WINSTATIONINFORMATIONEX_LEVEL {
    WinStationInfoExLevel1: WINSTATIONINFORMATIONEX_LEVEL1,
    WinStationInfoExLevel2: WINSTATIONINFORMATIONEX_LEVEL2,
}}
pub type PWINSTATIONINFORMATIONEX_LEVEL = *mut WINSTATIONINFORMATIONEX_LEVEL;
STRUCT! {struct WINSTATIONINFORMATIONEX {
    Level: c_ulong,
    Data: WINSTATIONINFORMATIONEX_LEVEL,
}}
pub type PWINSTATIONINFORMATIONEX = *mut WINSTATIONINFORMATIONEX;
pub const TS_PROCESS_INFO_MAGIC_NT4: u32 = 0x23495452;
STRUCT! {struct TS_PROCESS_INFORMATION_NT4 {
    MagicNumber: c_ulong,
    LogonId: c_ulong,
    ProcessSid: *mut c_void,
    Pad: c_ulong,
}}
pub type PTS_PROCESS_INFORMATION_NT4 = *mut TS_PROCESS_INFORMATION_NT4;
pub const SIZEOF_TS4_SYSTEM_THREAD_INFORMATION: u32 = 64;
pub const SIZEOF_TS4_SYSTEM_PROCESS_INFORMATION: u32 = 136;
STRUCT! {struct TS_SYS_PROCESS_INFORMATION {
    NextEntryOffset: c_ulong,
    NumberOfThreads: c_ulong,
    SpareLi1: LARGE_INTEGER,
    SpareLi2: LARGE_INTEGER,
    SpareLi3: LARGE_INTEGER,
    CreateTime: LARGE_INTEGER,
    UserTime: LARGE_INTEGER,
    KernelTime: LARGE_INTEGER,
    ImageName: UNICODE_STRING,
    BasePriority: c_long,
    UniqueProcessId: c_ulong,
    InheritedFromUniqueProcessId: c_ulong,
    HandleCount: c_ulong,
    SessionId: c_ulong,
    SpareUl3: c_ulong,
    PeakVirtualSize: usize,
    VirtualSize: usize,
    PageFaultCount: c_ulong,
    PeakWorkingSetSize: c_ulong,
    WorkingSetSize: c_ulong,
    QuotaPeakPagedPoolUsage: usize,
    QuotaPagedPoolUsage: usize,
    QuotaPeakNonPagedPoolUsage: usize,
    QuotaNonPagedPoolUsage: usize,
    PagefileUsage: usize,
    PeakPagefileUsage: usize,
    PrivatePageCount: usize,
}}
pub type PTS_SYS_PROCESS_INFORMATION = *mut TS_SYS_PROCESS_INFORMATION;
STRUCT! {struct TS_ALL_PROCESSES_INFO {
    pTsProcessInfo: PTS_SYS_PROCESS_INFORMATION,
    SizeOfSid: c_ulong,
    pSid: PSID,
}}
pub type PTS_ALL_PROCESSES_INFO = *mut TS_ALL_PROCESSES_INFO;
STRUCT! {struct TS_COUNTER_HEADER {
    dwCounterID: c_ulong,
    bResult: c_uchar,
}}
pub type PTS_COUNTER_HEADER = *mut TS_COUNTER_HEADER;
STRUCT! {struct TS_COUNTER {
    CounterHead: TS_COUNTER_HEADER,
    dwValue: c_ulong,
    StartTime: LARGE_INTEGER,
}}
pub type PTS_COUNTER = *mut TS_COUNTER;
pub const WSD_LOGOFF: c_ulong = 0x1;
pub const WSD_SHUTDOWN: c_ulong = 0x2;
pub const WSD_REBOOT: c_ulong = 0x4;
pub const WSD_POWEROFF: c_ulong = 0x8;
pub const WEVENT_NONE: c_ulong = 0x0;
pub const WEVENT_CREATE: c_ulong = 0x1;
pub const WEVENT_DELETE: c_ulong = 0x2;
pub const WEVENT_RENAME: c_ulong = 0x4;
pub const WEVENT_CONNECT: c_ulong = 0x8;
pub const WEVENT_DISCONNECT: c_ulong = 0x10;
pub const WEVENT_LOGON: c_ulong = 0x20;
pub const WEVENT_LOGOFF: c_ulong = 0x40;
pub const WEVENT_STATECHANGE: c_ulong = 0x80;
pub const WEVENT_LICENSE: c_ulong = 0x100;
pub const WEVENT_ALL: c_ulong = 0x7fffffff;
pub const WEVENT_FLUSH: c_ulong = 0x80000000;
pub const KBDSHIFT: c_ushort = 0x1;
pub const KBDCTRL: c_ushort = 0x2;
pub const KBDALT: c_ushort = 0x4;
pub const WNOTIFY_ALL_SESSIONS: c_ulong = 0x1;
pub const LOGONID_CURRENT: i32 = -1;
pub const SERVERNAME_CURRENT: *mut wchar_t = null_mut();
EXTERN! {extern "system" {
    fn WinStationFreeMemory(
        Buffer: *mut c_void,
    ) -> c_uchar;
    fn WinStationOpenServerW(
        ServerName: *mut wchar_t,
    ) -> HANDLE;
    fn WinStationCloseServer(
        ServerHandle: HANDLE,
    ) -> c_uchar;
    fn WinStationServerPing(
        ServerHandle: HANDLE,
    ) -> c_uchar;
    fn WinStationGetTermSrvCountersValue(
        ServerHandle: HANDLE,
        Count: c_ulong,
        Counters: PTS_COUNTER,
    ) -> c_uchar;
    fn WinStationShutdownSystem(
        ServerHandle: HANDLE,
        ShutdownFlags: c_ulong,
    ) -> c_uchar;
    fn WinStationWaitSystemEvent(
        ServerHandle: HANDLE,
        EventMask: c_ulong,
        EventFlags: *mut c_ulong,
    ) -> c_uchar;
    fn WinStationRegisterConsoleNotification(
        ServerHandle: HANDLE,
        WindowHandle: HWND,
        Flags: c_ulong,
    ) -> c_uchar;
    fn WinStationUnRegisterConsoleNotification(
        ServerHandle: HANDLE,
        WindowHandle: HWND,
    ) -> c_uchar;
    fn WinStationEnumerateW(
        ServerHandle: HANDLE,
        SessionIds: *mut PSESSIONIDW,
        Count: *mut c_ulong,
    ) -> c_uchar;
    fn WinStationQueryInformationW(
        ServerHandle: HANDLE,
        SessionId: c_ulong,
        WinStationInformationClass: WINSTATIONINFOCLASS,
        pWinStationInformation: *mut c_void,
        WinStationInformationLength: c_ulong,
        pReturnLength: *mut c_ulong,
    ) -> c_uchar;
    fn WinStationSetInformationW(
        ServerHandle: HANDLE,
        SessionId: c_ulong,
        WinStationInformationClass: WINSTATIONINFOCLASS,
        pWinStationInformation: *mut c_void,
        WinStationInformationLength: c_ulong,
    ) -> c_uchar;
    fn WinStationNameFromLogonIdW(
        ServerHandle: HANDLE,
        SessionId: c_ulong,
        pWinStationName: *mut wchar_t,
    ) -> c_uchar;
    fn WinStationSendMessageW(
        ServerHandle: HANDLE,
        SessionId: c_ulong,
        Title: *mut wchar_t,
        TitleLength: c_ulong,
        Message: *mut wchar_t,
        MessageLength: c_ulong,
        Style: c_ulong,
        Timeout: c_ulong,
        Response: *mut c_ulong,
        DoNotWait: c_uchar,
    ) -> c_uchar;
    fn WinStationConnectW(
        ServerHandle: HANDLE,
        SessionId: c_ulong,
        TargetSessionId: c_ulong,
        pPassword: *mut wchar_t,
        bWait: c_uchar,
    ) -> c_uchar;
    fn WinStationDisconnect(
        ServerHandle: HANDLE,
        SessionId: c_ulong,
        bWait: c_uchar,
    ) -> c_uchar;
    fn WinStationReset(
        ServerHandle: HANDLE,
        SessionId: c_ulong,
        bWait: c_uchar,
    ) -> c_uchar;
    fn WinStationShadow(
        ServerHandle: HANDLE,
        TargetServerName: *mut wchar_t,
        TargetSessionId: c_ulong,
        HotKeyVk: c_uchar,
        HotkeyModifiers: c_ushort,
    ) -> c_uchar;
    fn WinStationShadowStop(
        ServerHandle: HANDLE,
        SessionId: c_ulong,
        bWait: c_uchar,
    ) -> c_uchar;
    fn WinStationEnumerateProcesses(
        ServerHandle: HANDLE,
        Processes: *mut *mut c_void,
    ) -> c_uchar;
    fn WinStationGetAllProcesses(
        ServerHandle: HANDLE,
        Level: c_ulong,
        NumberOfProcesses: *mut c_ulong,
        Processes: *mut PTS_ALL_PROCESSES_INFO,
    ) -> c_uchar;
    fn WinStationFreeGAPMemory(
        Level: c_ulong,
        Processes: PTS_ALL_PROCESSES_INFO,
        NumberOfProcesses: c_ulong,
    ) -> c_uchar;
    fn WinStationTerminateProcess(
        ServerHandle: HANDLE,
        ProcessId: c_ulong,
        ExitCode: c_ulong,
    ) -> c_uchar;
    fn WinStationGetProcessSid(
        ServerHandle: HANDLE,
        ProcessId: c_ulong,
        ProcessStartTime: FILETIME,
        pProcessUserSid: *mut c_void,
        dwSidSize: *mut c_ulong,
    ) -> c_uchar;
    fn WinStationSwitchToServicesSession() -> c_uchar;
    fn WinStationRevertFromServicesSession() -> c_uchar;
    fn _WinStationWaitForConnect() -> c_uchar;
}}
