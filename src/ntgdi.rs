use crate::ctypes::{c_uchar, c_ulong, c_ushort, c_void};

pub const GDI_MAX_HANDLE_COUNT: usize = 0x4000;
pub const GDI_HANDLE_INDEX_SHIFT: c_ulong = 0;
pub const GDI_HANDLE_INDEX_BITS: c_ulong = 16;
pub const GDI_HANDLE_INDEX_MASK: c_ulong = 0xffff;
pub const GDI_HANDLE_TYPE_SHIFT: c_ulong = 16;
pub const GDI_HANDLE_TYPE_BITS: c_ulong = 5;
pub const GDI_HANDLE_TYPE_MASK: c_ulong = 0x1f;
pub const GDI_HANDLE_ALTTYPE_SHIFT: c_ulong = 21;
pub const GDI_HANDLE_ALTTYPE_BITS: c_ulong = 2;
pub const GDI_HANDLE_ALTTYPE_MASK: c_ulong = 0x3;
pub const GDI_HANDLE_STOCK_SHIFT: c_ulong = 23;
pub const GDI_HANDLE_STOCK_BITS: c_ulong = 1;
pub const GDI_HANDLE_STOCK_MASK: c_ulong = 0x1;
pub const GDI_HANDLE_UNIQUE_SHIFT: c_ulong = 24;
pub const GDI_HANDLE_UNIQUE_BITS: c_ulong = 8;
pub const GDI_HANDLE_UNIQUE_MASK: c_ulong = 0xff;
#[inline]
pub const fn GDI_HANDLE_INDEX(Handle: c_ulong) -> c_ulong {
    Handle & GDI_HANDLE_INDEX_MASK
}
#[inline]
pub const fn GDI_HANDLE_TYPE(Handle: c_ulong) -> c_ulong {
    Handle >> GDI_HANDLE_TYPE_SHIFT & GDI_HANDLE_TYPE_MASK
}
#[inline]
pub const fn GDI_HANDLE_ALTTYPE(Handle: c_ulong) -> c_ulong {
    Handle >> GDI_HANDLE_ALTTYPE_SHIFT & GDI_HANDLE_ALTTYPE_MASK
}
#[inline]
pub const fn GDI_HANDLE_STOCK(Handle: c_ulong) -> c_ulong {
    Handle >> GDI_HANDLE_STOCK_SHIFT & GDI_HANDLE_STOCK_MASK
}
#[inline]
pub const fn GDI_MAKE_HANDLE(Index: c_ulong, Unique: c_ulong) -> c_ulong {
    Unique << GDI_HANDLE_INDEX_BITS | Index
}
pub const GDI_DEF_TYPE: c_ulong = 0;
pub const GDI_DC_TYPE: c_ulong = 1;
pub const GDI_DD_DIRECTDRAW_TYPE: c_ulong = 2;
pub const GDI_DD_SURFACE_TYPE: c_ulong = 3;
pub const GDI_RGN_TYPE: c_ulong = 4;
pub const GDI_SURF_TYPE: c_ulong = 5;
pub const GDI_CLIENTOBJ_TYPE: c_ulong = 6;
pub const GDI_PATH_TYPE: c_ulong = 7;
pub const GDI_PAL_TYPE: c_ulong = 8;
pub const GDI_ICMLCS_TYPE: c_ulong = 9;
pub const GDI_LFONT_TYPE: c_ulong = 10;
pub const GDI_RFONT_TYPE: c_ulong = 11;
pub const GDI_PFE_TYPE: c_ulong = 12;
pub const GDI_PFT_TYPE: c_ulong = 13;
pub const GDI_ICMCXF_TYPE: c_ulong = 14;
pub const GDI_ICMDLL_TYPE: c_ulong = 15;
pub const GDI_BRUSH_TYPE: c_ulong = 16;
pub const GDI_PFF_TYPE: c_ulong = 17;
pub const GDI_CACHE_TYPE: c_ulong = 18;
pub const GDI_SPACE_TYPE: c_ulong = 19;
pub const GDI_DBRUSH_TYPE: c_ulong = 20;
pub const GDI_META_TYPE: c_ulong = 21;
pub const GDI_EFSTATE_TYPE: c_ulong = 22;
pub const GDI_BMFD_TYPE: c_ulong = 23;
pub const GDI_VTFD_TYPE: c_ulong = 24;
pub const GDI_TTFD_TYPE: c_ulong = 25;
pub const GDI_RC_TYPE: c_ulong = 26;
pub const GDI_TEMP_TYPE: c_ulong = 27;
pub const GDI_DRVOBJ_TYPE: c_ulong = 28;
pub const GDI_DCIOBJ_TYPE: c_ulong = 29;
pub const GDI_SPOOL_TYPE: c_ulong = 30;
#[inline]
pub const fn GDI_CLIENT_TYPE_FROM_HANDLE(Handle: c_ulong) -> c_ulong {
    Handle
        & (GDI_HANDLE_ALTTYPE_MASK << GDI_HANDLE_ALTTYPE_SHIFT
            | GDI_HANDLE_TYPE_MASK << GDI_HANDLE_TYPE_SHIFT)
}
#[inline]
pub const fn GDI_CLIENT_TYPE_FROM_UNIQUE(Unique: c_ulong) -> c_ulong {
    GDI_CLIENT_TYPE_FROM_HANDLE(Unique << 16)
}
pub const GDI_ALTTYPE_1: c_ulong = 1 << GDI_HANDLE_ALTTYPE_SHIFT;
pub const GDI_ALTTYPE_2: c_ulong = 2 << GDI_HANDLE_ALTTYPE_SHIFT;
pub const GDI_ALTTYPE_3: c_ulong = 3 << GDI_HANDLE_ALTTYPE_SHIFT;
pub const GDI_CLIENT_BITMAP_TYPE: c_ulong =
    GDI_SURF_TYPE << GDI_HANDLE_TYPE_SHIFT;
pub const GDI_CLIENT_BRUSH_TYPE: c_ulong =
    GDI_BRUSH_TYPE << GDI_HANDLE_TYPE_SHIFT;
pub const GDI_CLIENT_CLIENTOBJ_TYPE: c_ulong =
    GDI_CLIENTOBJ_TYPE << GDI_HANDLE_TYPE_SHIFT;
pub const GDI_CLIENT_DC_TYPE: c_ulong = GDI_DC_TYPE << GDI_HANDLE_TYPE_SHIFT;
pub const GDI_CLIENT_FONT_TYPE: c_ulong =
    GDI_LFONT_TYPE << GDI_HANDLE_TYPE_SHIFT;
pub const GDI_CLIENT_PALETTE_TYPE: c_ulong =
    GDI_PAL_TYPE << GDI_HANDLE_TYPE_SHIFT;
pub const GDI_CLIENT_REGION_TYPE: c_ulong =
    GDI_RGN_TYPE << GDI_HANDLE_TYPE_SHIFT;
pub const GDI_CLIENT_ALTDC_TYPE: c_ulong = GDI_CLIENT_DC_TYPE | GDI_ALTTYPE_1;
pub const GDI_CLIENT_DIBSECTION_TYPE: c_ulong =
    GDI_CLIENT_BITMAP_TYPE | GDI_ALTTYPE_1;
pub const GDI_CLIENT_EXTPEN_TYPE: c_ulong =
    GDI_CLIENT_BRUSH_TYPE | GDI_ALTTYPE_2;
pub const GDI_CLIENT_METADC16_TYPE: c_ulong =
    GDI_CLIENT_CLIENTOBJ_TYPE | GDI_ALTTYPE_3;
pub const GDI_CLIENT_METAFILE_TYPE: c_ulong =
    GDI_CLIENT_CLIENTOBJ_TYPE | GDI_ALTTYPE_2;
pub const GDI_CLIENT_METAFILE16_TYPE: c_ulong =
    GDI_CLIENT_CLIENTOBJ_TYPE | GDI_ALTTYPE_1;
pub const GDI_CLIENT_PEN_TYPE: c_ulong = GDI_CLIENT_BRUSH_TYPE | GDI_ALTTYPE_1;
UNION! {union GDI_HANDLE_ENTRY_u {
    Object: *mut c_void,
    NextFree: *mut c_void,
}}
STRUCT! {struct GDI_HANDLE_ENTRY_Owner_s {
    ProcessId: c_ushort,
    Bitfields: c_ushort,
}}
BITFIELD! {GDI_HANDLE_ENTRY_Owner_s Bitfields: c_ushort [
    Lock set_Lock[0..1],
    Count set_Count[1..16],
]}
UNION! {union GDI_HANDLE_ENTRY_Owner {
    s: GDI_HANDLE_ENTRY_Owner_s,
    Value: c_ulong,
}}
STRUCT! {struct GDI_HANDLE_ENTRY {
    u: GDI_HANDLE_ENTRY_u,
    Owner: GDI_HANDLE_ENTRY_Owner,
    Unique: c_ushort,
    Type: c_uchar,
    Flags: c_uchar,
    UserPointer: *mut c_void,
}}
pub type PGDI_HANDLE_ENTRY = *mut GDI_HANDLE_ENTRY;
STRUCT! {struct GDI_SHARED_MEMORY {
    Handles: [GDI_HANDLE_ENTRY; GDI_MAX_HANDLE_COUNT],
}}
pub type PGDI_SHARED_MEMORY = *mut GDI_SHARED_MEMORY;
