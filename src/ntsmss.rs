use windows_sys::Win32::Foundation::{HANDLE, NTSTATUS, UNICODE_STRING};

use crate::{ctypes::c_ulong, ntlpcapi::PPORT_MESSAGE};

EXTERN! {extern "system" {
    fn RtlConnectToSm(
        ApiPortName: *mut UNICODE_STRING,
        ApiPortHandle: HANDLE,
        ProcessImageType: c_ulong,
        SmssConnection: *mut HANDLE,
    ) -> NTSTATUS;
    fn RtlSendMsgToSm(
        ApiPortHandle: HANDLE,
        MessageData: PPORT_MESSAGE,
    ) -> NTSTATUS;
}}
