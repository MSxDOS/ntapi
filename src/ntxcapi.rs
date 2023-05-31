use windows_sys::Win32::{
    Foundation::NTSTATUS,
    System::Diagnostics::Debug::{CONTEXT, EXCEPTION_RECORD},
};

use crate::ctypes::{c_char, c_uchar, c_ulong, c_void};

EXTERN! {extern "system" {
    fn RtlDispatchException(
        ExceptionRecord: *mut EXCEPTION_RECORD,
        ContextRecord: *mut CONTEXT,
    ) -> c_uchar;
    fn RtlRaiseStatus(
        Status: NTSTATUS,
    );
    fn RtlRaiseException(
        ExceptionRecord: *mut EXCEPTION_RECORD,
    );
    fn NtContinue(
        ContextRecord: *mut CONTEXT,
        TestAlert: c_uchar,
    ) -> NTSTATUS;
    fn NtRaiseException(
        ExceptionRecord: *mut EXCEPTION_RECORD,
        ContextRecord: *mut CONTEXT,
        FirstChance: c_uchar,
    ) -> NTSTATUS;
    fn RtlAssert(
        VoidFailedAssertion: *mut c_void,
        VoidFileName: *mut c_void,
        LineNumber: c_ulong,
        MutableMessage: *mut c_char,
    );
}}
