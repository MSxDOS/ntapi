// Copyright 2018 ntapi Developers
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
use winapi::shared::ntdef::{BOOLEAN, NTSTATUS, PSTR, PVOID, ULONG};
use winapi::um::winnt::{PCONTEXT, PEXCEPTION_RECORD};
EXTERN!{extern "system" {
    fn RtlDispatchException(
        ExceptionRecord: PEXCEPTION_RECORD,
        ContextRecord: PCONTEXT,
    ) -> BOOLEAN;
    fn RtlRaiseStatus(Status: NTSTATUS);
    fn RtlRaiseException(ExceptionRecord: PEXCEPTION_RECORD);
    fn NtContinue(
        ContextRecord: PCONTEXT,
        TestAlert: BOOLEAN,
    ) -> NTSTATUS;
    fn NtRaiseException(
        ExceptionRecord: PEXCEPTION_RECORD,
        ContextRecord: PCONTEXT,
        FirstChance: BOOLEAN,
    ) -> NTSTATUS;
    fn RtlAssert(
        VoidFailedAssertion: PVOID,
        VoidFileName: PVOID,
        LineNumber: ULONG,
        MutableMessage: PSTR,
    );
}}
