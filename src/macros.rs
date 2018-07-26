// Copyright 2018 ntapi Developers
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
#[macro_export]
macro_rules! EXTERN {
    (extern $c:tt {$(
        fn $n:ident ($( $p:tt $(: $t:ty)*),* $(,)*) $(-> $r:ty)*;
    )+}) => {
        #[cfg_attr(all(target_env = "msvc", feature = "user"), link(name = "ntdll"))]
        #[cfg_attr(all(target_env = "msvc", feature = "kernel"), link(name = "ntoskrnl"))]
        extern $c {$(
            pub fn $n(
                $($p $(: $t)*),*
            ) $(-> $r)*;
        )+}
        $(
            #[cfg(feature = "func-types")]
            pub type $n = unsafe extern $c fn($($p $(: $t)*),*) $(-> $r)*;
        )+
    };
    (extern $c:tt {$(
        static mut $n:ident : $t:ty;
    )+}) => {
        #[cfg_attr(all(target_env = "msvc", feature = "user"), link(name = "ntdll"))]
        extern $c {$(
            pub static mut $n: $t;
        )+}
    };
}
#[macro_export]
macro_rules! FIELD_OFFSET {
    ($_type:ident, $field:ident$(.$cfields:ident)*) => {
        unsafe {
            &(*$crate::_core::ptr::null::<$_type>()).$field$(.$cfields)* as *const _ as usize
        }
    };
}
macro_rules! BITFIELD {
    ($base:ident $field:ident: $fieldtype:ty [
        $($thing:ident $set_thing:ident[$r:expr],)+
    ]) => {
        impl $base {$(
            #[inline]
            pub fn $thing(&self) -> $fieldtype {
                let size = $crate::_core::mem::size_of::<$fieldtype>() * 8;
                self.$field << (size - $r.end) >> (size - $r.end + $r.start)
            }
            #[inline]
            pub fn $set_thing(&mut self, val: $fieldtype) {
                let mask = ((1 << ($r.end - $r.start)) - 1) << $r.start;
                self.$field &= !mask;
                self.$field |= (val << $r.start) & mask;
            }
        )+}
    };
    (unsafe $base:ident $field:ident: $fieldtype:ty [
        $($thing:ident $set_thing:ident[$r:expr],)+
    ]) => {
        impl $base {$(
            #[inline]
            pub unsafe fn $thing(&self) -> $fieldtype {
                let size = $crate::_core::mem::size_of::<$fieldtype>() * 8;
                self.$field << (size - $r.end) >> (size - $r.end + $r.start)
            }
            #[inline]
            pub unsafe fn $set_thing(&mut self, val: $fieldtype) {
                let mask = ((1 << ($r.end - $r.start)) - 1) << $r.start;
                self.$field &= !mask;
                self.$field |= (val << $r.start) & mask;
            }
        )+}
    };
}
macro_rules! CTL_CODE {
    ($DeviceType:expr, $Function:expr, $Method:expr, $Access:expr) => {
        ($DeviceType << 16) | ($Access << 14) | ($Function << 2) | $Method
    };
}
macro_rules! UNION {
    ($(#[$attrs:meta])* union $name:ident {
        $($variant:ident: $ftype:ty,)+
    }) => (
        #[repr(C)] $(#[$attrs])*
        pub union $name {
            $(pub $variant: $ftype,)+
        }
        impl Copy for $name {}
        impl Clone for $name {
            #[inline]
            fn clone(&self) -> $name { *self }
        }
        #[cfg(feature = "impl-default")]
        impl Default for $name {
            #[inline]
            fn default() -> $name { unsafe { $crate::_core::mem::zeroed() } }
        }
    );
}
macro_rules! FN {
    (stdcall $func:ident($($p:ident: $t:ty,)*) -> $ret:ty) => (
        pub type $func = Option<unsafe extern "system" fn($($p: $t,)*) -> $ret>;
    );
    (cdecl $func:ident($($p:ident: $t:ty,)*) -> $ret:ty) => (
        pub type $func = Option<unsafe extern "C" fn($($p: $t,)*) -> $ret>;
    );
}
macro_rules! IFDEF {
    ($($thing:item)*) => ($($thing)*)
}
