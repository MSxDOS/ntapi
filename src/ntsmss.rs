// Copyright 2018 ntapi Developers
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
use ntlpcapi::PPORT_MESSAGE;
use winapi::shared::minwindef::DWORD;
use winapi::shared::ntdef::{HANDLE, NTSTATUS, PHANDLE, PUNICODE_STRING};
EXTERN!{extern "system" {
    fn RtlConnectToSm(
        ApiPortName: PUNICODE_STRING,
        ApiPortHandle: HANDLE,
        ProcessImageType: DWORD,
        SmssConnection: PHANDLE,
    ) -> NTSTATUS;
    fn RtlSendMsgToSm(
        ApiPortHandle: HANDLE,
        MessageData: PPORT_MESSAGE,
    ) -> NTSTATUS;
}}
