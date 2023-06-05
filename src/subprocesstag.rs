use crate::ctypes::{c_ulong, c_void, wchar_t};

ENUM! {enum TAG_INFO_LEVEL {
    eTagInfoLevelNameFromTag = 1,
    eTagInfoLevelNamesReferencingModule = 2,
    eTagInfoLevelNameTagMapping = 3,
    eTagInfoLevelMax = 4,
}}
ENUM! {enum TAG_TYPE {
    eTagTypeService = 1,
    eTagTypeMax = 2,
}}
STRUCT! {struct TAG_INFO_NAME_FROM_TAG_IN_PARAMS {
    dwPid: c_ulong,
    dwTag: c_ulong,
}}
pub type PTAG_INFO_NAME_FROM_TAG_IN_PARAMS =
    *mut TAG_INFO_NAME_FROM_TAG_IN_PARAMS;
STRUCT! {struct TAG_INFO_NAME_FROM_TAG_OUT_PARAMS {
    eTagType: c_ulong,
    pszName: *mut wchar_t,
}}
pub type PTAG_INFO_NAME_FROM_TAG_OUT_PARAMS =
    *mut TAG_INFO_NAME_FROM_TAG_OUT_PARAMS;
STRUCT! {struct TAG_INFO_NAME_FROM_TAG {
    InParams: TAG_INFO_NAME_FROM_TAG_IN_PARAMS,
    OutParams: TAG_INFO_NAME_FROM_TAG_OUT_PARAMS,
}}
pub type PTAG_INFO_NAME_FROM_TAG = *mut TAG_INFO_NAME_FROM_TAG;
STRUCT! {struct TAG_INFO_NAMES_REFERENCING_MODULE_IN_PARAMS {
    dwPid: c_ulong,
    pszModule: *mut wchar_t,
}}
pub type PTAG_INFO_NAMES_REFERENCING_MODULE_IN_PARAMS =
    *mut TAG_INFO_NAMES_REFERENCING_MODULE_IN_PARAMS;
STRUCT! {struct TAG_INFO_NAMES_REFERENCING_MODULE_OUT_PARAMS {
    eTagType: c_ulong,
    pmszNames: *mut wchar_t,
}}
pub type PTAG_INFO_NAMES_REFERENCING_MODULE_OUT_PARAMS =
    *mut TAG_INFO_NAMES_REFERENCING_MODULE_OUT_PARAMS;
STRUCT! {struct TAG_INFO_NAMES_REFERENCING_MODULE {
    InParams: TAG_INFO_NAMES_REFERENCING_MODULE_IN_PARAMS,
    OutParams: TAG_INFO_NAMES_REFERENCING_MODULE_OUT_PARAMS,
}}
pub type PTAG_INFO_NAMES_REFERENCING_MODULE =
    *mut TAG_INFO_NAMES_REFERENCING_MODULE;
STRUCT! {struct TAG_INFO_NAME_TAG_MAPPING_IN_PARAMS {
    dwPid: c_ulong,
}}
pub type PTAG_INFO_NAME_TAG_MAPPING_IN_PARAMS =
    *mut TAG_INFO_NAME_TAG_MAPPING_IN_PARAMS;
STRUCT! {struct TAG_INFO_NAME_TAG_MAPPING_ELEMENT {
    eTagType: c_ulong,
    dwTag: c_ulong,
    pszName: *mut wchar_t,
    pszGroupName: *mut wchar_t,
}}
pub type PTAG_INFO_NAME_TAG_MAPPING_ELEMENT =
    *mut TAG_INFO_NAME_TAG_MAPPING_ELEMENT;
STRUCT! {struct TAG_INFO_NAME_TAG_MAPPING_OUT_PARAMS {
    cElements: c_ulong,
    pNameTagMappingElements: PTAG_INFO_NAME_TAG_MAPPING_ELEMENT,
}}
pub type PTAG_INFO_NAME_TAG_MAPPING_OUT_PARAMS =
    *mut TAG_INFO_NAME_TAG_MAPPING_OUT_PARAMS;
STRUCT! {struct TAG_INFO_NAME_TAG_MAPPING {
    InParams: TAG_INFO_NAME_TAG_MAPPING_IN_PARAMS,
    pOutParams: PTAG_INFO_NAME_TAG_MAPPING_OUT_PARAMS,
}}
pub type PTAG_INFO_NAME_TAG_MAPPING = *mut TAG_INFO_NAME_TAG_MAPPING;
EXTERN! {extern "system" {
    fn I_QueryTagInformation(
        pszMachineName: *const wchar_t,
        eInfoLevel: TAG_INFO_LEVEL,
        pTagInfo: *mut c_void,
    ) -> c_ulong;
}}
FN! {stdcall PQUERY_TAG_INFORMATION(
    pszMachineName: *const wchar_t,
    eInfoLevel: TAG_INFO_LEVEL,
    pTagInfo: *mut c_void,
) -> c_ulong}
