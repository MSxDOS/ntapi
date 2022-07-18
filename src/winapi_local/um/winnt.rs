#[cfg(all(feature = "beta", not(target_arch = "aarch64")))]
use core::arch::asm;
use winapi::shared::basetsd::{DWORD64, SIZE_T, ULONG64};
use winapi::shared::minwindef::DWORD;
use winapi::um::winnt::{HANDLE, PVOID};
#[doc(hidden)]
#[cfg_attr(not(feature = "aggressive-inline"), inline)]
#[cfg_attr(feature = "aggressive-inline", inline(always))]
pub const fn UInt32x32To64(a: u32, b: u32) -> u64 {
    a as u64 * b as u64
}
#[cfg(all(feature = "beta", not(target_arch = "aarch64")))]
IFDEF! {
use crate::ntpebteb::TEB;
#[cfg_attr(not(feature = "aggressive-inline"), inline)]
    #[cfg_attr(feature = "aggressive-inline", inline(always))]
#[cfg(target_pointer_width = "64")]
pub unsafe fn _bittest64(Base: *const i64, Offset: i64) -> u8 {
    let out: u8;
    asm!(
        "bt {1}, {2}",
        "setb {0}",
        out(reg_byte) out,
        in(reg) Base,
        in(reg) Offset,
        options(nostack, pure, readonly),
    );
    out
}
#[cfg_attr(not(feature = "aggressive-inline"), inline)]
    #[cfg_attr(feature = "aggressive-inline", inline(always))]
pub unsafe fn __readfsdword(Offset: DWORD) -> DWORD {
    let out: u32;
    asm!(
        "mov {:e}, fs:[{:e}]",
        lateout(reg) out,
        in(reg) Offset,
        options(nostack, pure, readonly),
    );
    out
}
#[cfg_attr(not(feature = "aggressive-inline"), inline)]
    #[cfg_attr(feature = "aggressive-inline", inline(always))]
#[cfg(target_pointer_width = "64")]
pub unsafe fn __readgsqword(Offset: DWORD) -> DWORD64 {
    let out: u64;
    asm!(
        "mov {}, gs:[{:e}]",
        lateout(reg) out,
        in(reg) Offset,
        options(nostack, pure, readonly),
    );
    out
}
#[cfg_attr(not(feature = "aggressive-inline"), inline)]
    #[cfg_attr(feature = "aggressive-inline", inline(always))] #[allow(unused_unsafe)]
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
ENUM! {enum MEM_EXTENDED_PARAMETER_TYPE {
    MemExtendedParameterInvalidType = 0,
    MemExtendedParameterAddressRequirements = 1,
    MemExtendedParameterNumaNode = 2,
    MemExtendedParameterPartitionHandle = 3,
    MemExtendedParameterMax = 4,
}}
pub type PMEM_EXTENDED_PARAMETER_TYPE = *mut MEM_EXTENDED_PARAMETER_TYPE;
UNION! {union MEM_EXTENDED_PARAMETER_u {
    ULong64: DWORD64,
    Pointer: PVOID,
    Size: SIZE_T,
    Handle: HANDLE,
    ULong: DWORD,
}}
STRUCT! {#[debug] struct MEM_EXTENDED_PARAMETER {
    BitFields: ULONG64,
    u: MEM_EXTENDED_PARAMETER_u,
}}
BITFIELD! {MEM_EXTENDED_PARAMETER BitFields: ULONG64 [
    Type set_Type[0..8],
    Reserved set_Reserved[8..64],
]}
pub type PMEM_EXTENDED_PARAMETER = *mut MEM_EXTENDED_PARAMETER;
