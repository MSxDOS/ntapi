use crate::ctypes::{c_uchar, c_ushort, c_void};

pub const MAXIMUM_LEADBYTES: usize = 12;
STRUCT! {struct CPTABLEINFO {
    CodePage: c_ushort,
    MaximumCharacterSize: c_ushort,
    DefaultChar: c_ushort,
    UniDefaultChar: c_ushort,
    TransDefaultChar: c_ushort,
    TransUniDefaultChar: c_ushort,
    DBCSCodePage: c_ushort,
    LeadByte: [c_uchar; MAXIMUM_LEADBYTES],
    MultiByteTable: *mut c_ushort,
    WideCharTable: *mut c_void,
    DBCSRanges: *mut c_ushort,
    DBCSOffsets: *mut c_ushort,
}}
pub type PCPTABLEINFO = *mut CPTABLEINFO;
STRUCT! {struct NLSTABLEINFO {
    OemTableInfo: CPTABLEINFO,
    AnsiTableInfo: CPTABLEINFO,
    UpperCaseTable: *mut c_ushort,
    LowerCaseTable: *mut c_ushort,
}}
pub type PNLSTABLEINFO = *mut NLSTABLEINFO;
EXTERN! {extern "C" {
    static mut NlsAnsiCodePage: c_ushort;
    static mut NlsMbCodePageTag: c_uchar;
    static mut NlsMbOemCodePageTag: c_uchar;
}}
