use windows_sys::Win32::Foundation::{FACILITY_NTWIN32, HANDLE, NTSTATUS};

use crate::ctypes::{__uint64, c_long, c_ulong, c_ushort};

pub type KPRIORITY = c_long;
pub type RTL_ATOM = c_ushort;
pub type PRTL_ATOM = *mut RTL_ATOM;
pub const NT_FACILITY_MASK: c_ulong = 0xfff;
pub const NT_FACILITY_SHIFT: c_ulong = 16;
#[inline]
pub const fn NT_FACILITY(Status: NTSTATUS) -> c_ulong {
    (Status as u32) >> NT_FACILITY_SHIFT & NT_FACILITY_MASK
}
#[inline]
pub const fn NT_NTWIN32(Status: NTSTATUS) -> bool {
    NT_FACILITY(Status) == FACILITY_NTWIN32
}
#[inline]
pub const fn WIN32_FROM_NTSTATUS(Status: NTSTATUS) -> c_ulong {
    (Status as u32) & 0xffff
}
STRUCT! {struct CLIENT_ID {
    UniqueProcess: HANDLE,
    UniqueThread: HANDLE,
}}
pub type PCLIENT_ID = *mut CLIENT_ID;
STRUCT! {struct CLIENT_ID32 {
    UniqueProcess: c_ulong,
    UniqueThread: c_ulong,
}}
pub type PCLIENT_ID32 = *mut CLIENT_ID32;
STRUCT! {struct CLIENT_ID64 {
    UniqueProcess: __uint64,
    UniqueThread: __uint64,
}}
pub type PCLIENT_ID64 = *mut CLIENT_ID64;
STRUCT! {struct KSYSTEM_TIME {
    LowPart: c_ulong,
    High1Time: c_long,
    High2Time: c_long,
}}
pub type PKSYSTEM_TIME = *mut KSYSTEM_TIME;
