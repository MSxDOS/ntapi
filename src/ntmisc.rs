// Copyright 2018 ntapi Developers
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
use winapi::shared::ntdef::{HANDLE, NTSTATUS, PULONG, PVOID, ULONG};
use winapi::um::winnt::STANDARD_RIGHTS_ALL;
pub const FLT_PORT_CONNECT: u32 = 0x0001;
pub const FLT_PORT_ALL_ACCESS: u32 = FLT_PORT_CONNECT | STANDARD_RIGHTS_ALL;
ENUM!{enum VDMSERVICECLASS {
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
EXTERN!{extern "system" {
    fn NtVdmControl(
        Service: VDMSERVICECLASS,
        ServiceData: PVOID,
    ) -> NTSTATUS;
    fn NtTraceEvent(
        TraceHandle: HANDLE,
        Flags: ULONG,
        FieldSize: ULONG,
        Fields: PVOID,
    ) -> NTSTATUS;
    fn NtTraceControl(
        FunctionCode: ULONG,
        InBuffer: PVOID,
        InBufferLen: ULONG,
        OutBuffer: PVOID,
        OutBufferLen: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
}}
