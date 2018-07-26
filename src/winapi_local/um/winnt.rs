// Copyright 2018 ntapi Developers
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
use winapi::shared::minwindef::DWORD;
#[inline]
pub fn UInt32x32To64(a: u32, b: u32) -> u64 {
    a as u64 * b as u64
}
#[cfg(feature = "nightly")]
IFDEF!{
use ntpebteb::TEB;
use winapi::shared::basetsd::DWORD64;
#[inline]
pub unsafe fn _bittest64(Base: *const i64, Offset: i64) -> u8 {
    let out: u8;
    asm!("bt $1, $2; setb $0"
    : "=r"(out)
    :  "*m"(Base), "r"(Offset)
    : "cc"
    : "intel"
    );
    out
}
#[inline]
pub unsafe fn __readfsdword(Offset: DWORD) -> DWORD {
    let out: u32;
    asm!("mov $0, fs:[$1]"
    : "=r"(out)
    : "ri"(Offset)
    :
    : "intel"
    );
    out
}
#[inline]
pub unsafe fn __readgsqword(Offset: DWORD) -> DWORD64 {
    let out: u64;
    asm!("mov $0, gs:[$1]"
    : "=r"(out)
    : "ri"(Offset)
    :
    : "intel"
    );
    out
}
#[inline] #[allow(unused_unsafe)]
pub unsafe fn NtCurrentTeb() -> *mut TEB {
    use winapi::um::winnt::NT_TIB;
    let teb_offset = FIELD_OFFSET!(NT_TIB, _Self) as u32;
    #[cfg(target_arch = "x86_64")] {
        __readgsqword(teb_offset) as *mut TEB
    }
    #[cfg(target_arch = "x86")] {
        __readfsdword(teb_offset) as *mut TEB
    }
}
}
STRUCT!{struct PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY {
    Flags: DWORD,
}}
pub type PPPROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY =
    *mut PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY;
BITFIELD!{PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY Flags: DWORD [
    FilterId set_FilterId[0..4],
    ReservedFlags set_ReservedFlags[4..32],
]}
STRUCT!{struct PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY {
    Flags: DWORD,
}}
pub type PPROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY =
    *mut PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY;
BITFIELD!{PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY Flags: DWORD [
    EnableExportAddressFilter set_EnableExportAddressFilter[0..1],
    AuditExportAddressFilter set_AuditExportAddressFilter[1..2],
    EnableExportAddressFilterPlus set_EnableExportAddressFilterPlus[2..3],
    AuditExportAddressFilterPlus set_AuditExportAddressFilterPlus[3..4],
    EnableImportAddressFilter set_EnableImportAddressFilter[4..5],
    AuditImportAddressFilter set_AuditImportAddressFilter[5..6],
    EnableRopStackPivot set_EnableRopStackPivot[6..7],
    AuditRopStackPivot set_AuditRopStackPivot[7..8],
    EnableRopCallerCheck set_EnableRopCallerCheck[8..9],
    AuditRopCallerCheck set_AuditRopCallerCheck[9..10],
    EnableRopSimExec set_EnableRopSimExec[10..11],
    AuditRopSimExec set_AuditRopSimExec[11..12],
    ReservedFlags set_ReservedFlags[12..32],
]}
STRUCT!{struct PROCESS_MITIGATION_CHILD_PROCESS_POLICY {
    Flags: DWORD,
}}
pub type PPROCESS_MITIGATION_CHILD_PROCESS_POLICY = *mut PROCESS_MITIGATION_CHILD_PROCESS_POLICY;
BITFIELD!{PROCESS_MITIGATION_CHILD_PROCESS_POLICY Flags: DWORD [
    NoChildProcessCreation set_NoChildProcessCreation[0..1],
    AuditNoChildProcessCreation set_AuditNoChildProcessCreation[1..2],
    AllowSecureProcessCreation set_AllowSecureProcessCreation[2..3],
    ReservedFlags set_ReservedFlags[3..32],
]}
