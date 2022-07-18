use winapi::shared::ntdef::{PVOID, UCHAR, ULONG, USHORT};
pub const GDI_MAX_HANDLE_COUNT: usize = 0x4000;
pub const GDI_HANDLE_INDEX_SHIFT: ULONG = 0;
pub const GDI_HANDLE_INDEX_BITS: ULONG = 16;
pub const GDI_HANDLE_INDEX_MASK: ULONG = 0xffff;
pub const GDI_HANDLE_TYPE_SHIFT: ULONG = 16;
pub const GDI_HANDLE_TYPE_BITS: ULONG = 5;
pub const GDI_HANDLE_TYPE_MASK: ULONG = 0x1f;
pub const GDI_HANDLE_ALTTYPE_SHIFT: ULONG = 21;
pub const GDI_HANDLE_ALTTYPE_BITS: ULONG = 2;
pub const GDI_HANDLE_ALTTYPE_MASK: ULONG = 0x3;
pub const GDI_HANDLE_STOCK_SHIFT: ULONG = 23;
pub const GDI_HANDLE_STOCK_BITS: ULONG = 1;
pub const GDI_HANDLE_STOCK_MASK: ULONG = 0x1;
pub const GDI_HANDLE_UNIQUE_SHIFT: ULONG = 24;
pub const GDI_HANDLE_UNIQUE_BITS: ULONG = 8;
pub const GDI_HANDLE_UNIQUE_MASK: ULONG = 0xff;
#[cfg_attr(not(feature = "aggressive-inline"), inline)]
#[cfg_attr(feature = "aggressive-inline", inline(always))]
pub const fn GDI_HANDLE_INDEX(Handle: ULONG) -> ULONG {
    Handle & GDI_HANDLE_INDEX_MASK
}
#[cfg_attr(not(feature = "aggressive-inline"), inline)]
#[cfg_attr(feature = "aggressive-inline", inline(always))]
pub const fn GDI_HANDLE_TYPE(Handle: ULONG) -> ULONG {
    Handle >> GDI_HANDLE_TYPE_SHIFT & GDI_HANDLE_TYPE_MASK
}
#[cfg_attr(not(feature = "aggressive-inline"), inline)]
#[cfg_attr(feature = "aggressive-inline", inline(always))]
pub const fn GDI_HANDLE_ALTTYPE(Handle: ULONG) -> ULONG {
    Handle >> GDI_HANDLE_ALTTYPE_SHIFT & GDI_HANDLE_ALTTYPE_MASK
}
#[cfg_attr(not(feature = "aggressive-inline"), inline)]
#[cfg_attr(feature = "aggressive-inline", inline(always))]
pub const fn GDI_HANDLE_STOCK(Handle: ULONG) -> ULONG {
    Handle >> GDI_HANDLE_STOCK_SHIFT & GDI_HANDLE_STOCK_MASK
}
#[cfg_attr(not(feature = "aggressive-inline"), inline)]
#[cfg_attr(feature = "aggressive-inline", inline(always))]
pub const fn GDI_MAKE_HANDLE(Index: ULONG, Unique: ULONG) -> ULONG {
    Unique << GDI_HANDLE_INDEX_BITS | Index
}
pub const GDI_DEF_TYPE: ULONG = 0;
pub const GDI_DC_TYPE: ULONG = 1;
pub const GDI_DD_DIRECTDRAW_TYPE: ULONG = 2;
pub const GDI_DD_SURFACE_TYPE: ULONG = 3;
pub const GDI_RGN_TYPE: ULONG = 4;
pub const GDI_SURF_TYPE: ULONG = 5;
pub const GDI_CLIENTOBJ_TYPE: ULONG = 6;
pub const GDI_PATH_TYPE: ULONG = 7;
pub const GDI_PAL_TYPE: ULONG = 8;
pub const GDI_ICMLCS_TYPE: ULONG = 9;
pub const GDI_LFONT_TYPE: ULONG = 10;
pub const GDI_RFONT_TYPE: ULONG = 11;
pub const GDI_PFE_TYPE: ULONG = 12;
pub const GDI_PFT_TYPE: ULONG = 13;
pub const GDI_ICMCXF_TYPE: ULONG = 14;
pub const GDI_ICMDLL_TYPE: ULONG = 15;
pub const GDI_BRUSH_TYPE: ULONG = 16;
pub const GDI_PFF_TYPE: ULONG = 17;
pub const GDI_CACHE_TYPE: ULONG = 18;
pub const GDI_SPACE_TYPE: ULONG = 19;
pub const GDI_DBRUSH_TYPE: ULONG = 20;
pub const GDI_META_TYPE: ULONG = 21;
pub const GDI_EFSTATE_TYPE: ULONG = 22;
pub const GDI_BMFD_TYPE: ULONG = 23;
pub const GDI_VTFD_TYPE: ULONG = 24;
pub const GDI_TTFD_TYPE: ULONG = 25;
pub const GDI_RC_TYPE: ULONG = 26;
pub const GDI_TEMP_TYPE: ULONG = 27;
pub const GDI_DRVOBJ_TYPE: ULONG = 28;
pub const GDI_DCIOBJ_TYPE: ULONG = 29;
pub const GDI_SPOOL_TYPE: ULONG = 30;
#[cfg_attr(not(feature = "aggressive-inline"), inline)]
#[cfg_attr(feature = "aggressive-inline", inline(always))]
pub const fn GDI_CLIENT_TYPE_FROM_HANDLE(Handle: ULONG) -> ULONG {
    Handle
        & (GDI_HANDLE_ALTTYPE_MASK << GDI_HANDLE_ALTTYPE_SHIFT
            | GDI_HANDLE_TYPE_MASK << GDI_HANDLE_TYPE_SHIFT)
}
#[cfg_attr(not(feature = "aggressive-inline"), inline)]
#[cfg_attr(feature = "aggressive-inline", inline(always))]
pub const fn GDI_CLIENT_TYPE_FROM_UNIQUE(Unique: ULONG) -> ULONG {
    GDI_CLIENT_TYPE_FROM_HANDLE(Unique << 16)
}
pub const GDI_ALTTYPE_1: ULONG = 1 << GDI_HANDLE_ALTTYPE_SHIFT;
pub const GDI_ALTTYPE_2: ULONG = 2 << GDI_HANDLE_ALTTYPE_SHIFT;
pub const GDI_ALTTYPE_3: ULONG = 3 << GDI_HANDLE_ALTTYPE_SHIFT;
pub const GDI_CLIENT_BITMAP_TYPE: ULONG = GDI_SURF_TYPE << GDI_HANDLE_TYPE_SHIFT;
pub const GDI_CLIENT_BRUSH_TYPE: ULONG = GDI_BRUSH_TYPE << GDI_HANDLE_TYPE_SHIFT;
pub const GDI_CLIENT_CLIENTOBJ_TYPE: ULONG = GDI_CLIENTOBJ_TYPE << GDI_HANDLE_TYPE_SHIFT;
pub const GDI_CLIENT_DC_TYPE: ULONG = GDI_DC_TYPE << GDI_HANDLE_TYPE_SHIFT;
pub const GDI_CLIENT_FONT_TYPE: ULONG = GDI_LFONT_TYPE << GDI_HANDLE_TYPE_SHIFT;
pub const GDI_CLIENT_PALETTE_TYPE: ULONG = GDI_PAL_TYPE << GDI_HANDLE_TYPE_SHIFT;
pub const GDI_CLIENT_REGION_TYPE: ULONG = GDI_RGN_TYPE << GDI_HANDLE_TYPE_SHIFT;
pub const GDI_CLIENT_ALTDC_TYPE: ULONG = GDI_CLIENT_DC_TYPE | GDI_ALTTYPE_1;
pub const GDI_CLIENT_DIBSECTION_TYPE: ULONG = GDI_CLIENT_BITMAP_TYPE | GDI_ALTTYPE_1;
pub const GDI_CLIENT_EXTPEN_TYPE: ULONG = GDI_CLIENT_BRUSH_TYPE | GDI_ALTTYPE_2;
pub const GDI_CLIENT_METADC16_TYPE: ULONG = GDI_CLIENT_CLIENTOBJ_TYPE | GDI_ALTTYPE_3;
pub const GDI_CLIENT_METAFILE_TYPE: ULONG = GDI_CLIENT_CLIENTOBJ_TYPE | GDI_ALTTYPE_2;
pub const GDI_CLIENT_METAFILE16_TYPE: ULONG = GDI_CLIENT_CLIENTOBJ_TYPE | GDI_ALTTYPE_1;
pub const GDI_CLIENT_PEN_TYPE: ULONG = GDI_CLIENT_BRUSH_TYPE | GDI_ALTTYPE_1;
UNION! {union GDI_HANDLE_ENTRY_u {
    Object: PVOID,
    NextFree: PVOID,
}}
STRUCT! {struct GDI_HANDLE_ENTRY_Owner_s {
    ProcessId: USHORT,
    Bitfields: USHORT,
}}
BITFIELD! {GDI_HANDLE_ENTRY_Owner_s Bitfields: USHORT [
    Lock set_Lock[0..1],
    Count set_Count[1..16],
]}
UNION! {union GDI_HANDLE_ENTRY_Owner {
    s: GDI_HANDLE_ENTRY_Owner_s,
    Value: ULONG,
}}
STRUCT! {struct GDI_HANDLE_ENTRY {
    u: GDI_HANDLE_ENTRY_u,
    Owner: GDI_HANDLE_ENTRY_Owner,
    Unique: USHORT,
    Type: UCHAR,
    Flags: UCHAR,
    UserPointer: PVOID,
}}
pub type PGDI_HANDLE_ENTRY = *mut GDI_HANDLE_ENTRY;
STRUCT! {struct GDI_SHARED_MEMORY {
    Handles: [GDI_HANDLE_ENTRY; GDI_MAX_HANDLE_COUNT],
}}
pub type PGDI_SHARED_MEMORY = *mut GDI_SHARED_MEMORY;
