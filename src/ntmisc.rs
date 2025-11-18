use windows_sys::Win32::{
    Foundation::{HANDLE, NTSTATUS},
    Storage::FileSystem::STANDARD_RIGHTS_ALL,
};

use crate::ctypes::{c_ulong, c_void};

pub const FLT_PORT_CONNECT: u32 = 0x0001;
pub const FLT_PORT_ALL_ACCESS: u32 = FLT_PORT_CONNECT | STANDARD_RIGHTS_ALL;
ENUM! {enum VDMSERVICECLASS {
    VdmStartExecution = 0,
    VdmQueueInterrupt = 1,
    VdmDelayInterrupt = 2,
    VdmInitialize = 3,
    VdmFeatures = 4,
    VdmSetInt21Handler = 5,
    VdmQueryDir = 6,
    VdmPrinterDirectIoOpen = 7,
    VdmPrinterDirectIoClose = 8,
    VdmPrinterInitialize = 9,
    VdmSetLdtEntries = 10,
    VdmSetProcessLdtInfo = 11,
    VdmAdlibEmulation = 12,
    VdmPMCliControl = 13,
    VdmQueryVdmProcess = 14,
}}
pub type PVDMSERVICECLASS = *mut VDMSERVICECLASS;
EXTERN! {extern "system" {
    fn NtVdmControl(
        Service: VDMSERVICECLASS,
        ServiceData: *mut c_void,
    ) -> NTSTATUS;
    fn NtTraceEvent(
        TraceHandle: HANDLE,
        Flags: c_ulong,
        FieldSize: c_ulong,
        Fields: *mut c_void,
    ) -> NTSTATUS;
    fn NtTraceControl(
        FunctionCode: c_ulong,
        InBuffer: *mut c_void,
        InBufferLen: c_ulong,
        OutBuffer: *mut c_void,
        OutBufferLen: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
}}
