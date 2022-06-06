#[macro_export]
macro_rules! EXTERN {
    (extern $c:tt {$(
        fn $n:ident ($( $p:tt $(: $t:ty)?),* $(,)?) $(-> $r:ty)?;
    )+}) => {
        #[cfg_attr(all(target_env = "msvc", feature = "user"), link(name = "ntdll"))]
        #[cfg_attr(all(target_env = "msvc", feature = "kernel"), link(name = "ntoskrnl"))]
        extern $c {$(
            pub fn $n(
                $($p $(: $t)?),*
            ) $(-> $r)?;
        )+}
        $(
            #[cfg(feature = "func-types")]
            pub type $n = unsafe extern $c fn($($p $(: $t)?),*) $(-> $r)?;
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
#[doc(hidden)]
macro_rules! FIELD_OFFSET {
    ($_type:ty, $field:ident$(.$cfields:ident)*) => {
        unsafe {
            union Transmuter<T: 'static> {
                p: *const T,
                r: &'static T,
                i: usize,
            }
            #[allow(unaligned_references)]
            Transmuter {
                r: &(&Transmuter {
                    p: $crate::_core::ptr::null::<$_type>()
                }.r).$field$(.$cfields)*
            }.i
        }
    };
}
macro_rules! BITFIELD {
    ($base:ident $field:ident: $fieldtype:ty [
        $($thing:ident $set_thing:ident[$r:expr],)+
    ]) => {
        impl $base {$(
            #[cfg_attr(not(feature = "aggressive-inline"), inline)]
    #[cfg_attr(feature = "aggressive-inline", inline(always))]
            pub const fn $thing(&self) -> $fieldtype {
                const SIZE: usize = $crate::_core::mem::size_of::<$fieldtype>() * 8;
                self.$field << (SIZE - $r.end) >> (SIZE - $r.end + $r.start)
            }
            #[cfg_attr(not(feature = "aggressive-inline"), inline)]
    #[cfg_attr(feature = "aggressive-inline", inline(always))]
            pub fn $set_thing(&mut self, val: $fieldtype) {
                const MASK: $fieldtype = ((1 << ($r.end - $r.start)) - 1) << $r.start;
                self.$field &= !MASK;
                self.$field |= (val << $r.start) & MASK;
            }
        )+}
    };
    (unsafe $base:ident $field:ident: $fieldtype:ty [
        $($thing:ident $set_thing:ident[$r:expr],)+
    ]) => {
        impl $base {$(
            #[cfg_attr(not(feature = "aggressive-inline"), inline)]
    #[cfg_attr(feature = "aggressive-inline", inline(always))]
            pub unsafe fn $thing(&self) -> $fieldtype {
                const SIZE: usize = $crate::_core::mem::size_of::<$fieldtype>() * 8;
                self.$field << (SIZE - $r.end) >> (SIZE - $r.end + $r.start)
            }
            #[cfg_attr(not(feature = "aggressive-inline"), inline)]
    #[cfg_attr(feature = "aggressive-inline", inline(always))]
            pub unsafe fn $set_thing(&mut self, val: $fieldtype) {
                const MASK: $fieldtype = ((1 << ($r.end - $r.start)) - 1) << $r.start;
                self.$field &= !MASK;
                self.$field |= (val << $r.start) & MASK;
            }
        )+}
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
            #[cfg_attr(not(feature = "aggressive-inline"), inline)]
    #[cfg_attr(feature = "aggressive-inline", inline(always))]
            fn clone(&self) -> $name { *self }
        }
        #[cfg(feature = "impl-default")]
        impl Default for $name {
            #[cfg_attr(not(feature = "aggressive-inline"), inline)]
    #[cfg_attr(feature = "aggressive-inline", inline(always))]
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
