// Copyright 2018 ntapi Developers
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
use winapi::shared::ntdef::{BOOLEAN, PUSHORT, PVOID, UCHAR, USHORT};
pub const MAXIMUM_LEADBYTES: usize = 12;
STRUCT!{struct CPTABLEINFO {
    CodePage: USHORT,
    MaximumCharacterSize: USHORT,
    DefaultChar: USHORT,
    UniDefaultChar: USHORT,
    TransDefaultChar: USHORT,
    TransUniDefaultChar: USHORT,
    DBCSCodePage: USHORT,
    LeadByte: [UCHAR; MAXIMUM_LEADBYTES],
    MultiByteTable: PUSHORT,
    WideCharTable: PVOID,
    DBCSRanges: PUSHORT,
    DBCSOffsets: PUSHORT,
}}
pub type PCPTABLEINFO = *mut CPTABLEINFO;
STRUCT!{struct NLSTABLEINFO {
    OemTableInfo: CPTABLEINFO,
    AnsiTableInfo: CPTABLEINFO,
    UpperCaseTable: PUSHORT,
    LowerCaseTable: PUSHORT,
}}
pub type PNLSTABLEINFO = *mut NLSTABLEINFO;
EXTERN!{extern "C" {
    static mut NlsAnsiCodePage: USHORT;
    static mut NlsMbCodePageTag: BOOLEAN;
    static mut NlsMbOemCodePageTag: BOOLEAN;
}}
