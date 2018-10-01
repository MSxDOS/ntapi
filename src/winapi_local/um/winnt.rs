// Copyright 2018 ntapi Developers
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
#[inline]
pub fn UInt32x32To64(a: u32, b: u32) -> u64 {
    a as u64 * b as u64
}
#[cfg(feature = "nightly")]
IFDEF!{
use ntpebteb::TEB;
use winapi::shared::basetsd::DWORD64;
use winapi::shared::minwindef::DWORD;
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
