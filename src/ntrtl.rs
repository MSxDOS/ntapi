use core::ptr::{eq, null_mut};

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
use windows_sys::Win32::System::Diagnostics::Debug::{
    IMAGE_RUNTIME_FUNCTION_ENTRY, PGET_RUNTIME_FUNCTION_CALLBACK, WOW64_CONTEXT,
};
use windows_sys::{
    core::GUID,
    Win32::{
        Foundation::{HANDLE, LUID, NTSTATUS, PSID, UNICODE_STRING},
        Networking::WinSock::{IN6_ADDR, IN_ADDR},
        Security::{
            ACL, ACL_INFORMATION_CLASS, GENERIC_MAPPING, LUID_AND_ATTRIBUTES,
            PSECURITY_DESCRIPTOR, SECURITY_DESCRIPTOR_CONTROL,
            SECURITY_IMPERSONATION_LEVEL, SID_AND_ATTRIBUTES,
            SID_AND_ATTRIBUTES_HASH, SID_IDENTIFIER_AUTHORITY,
        },
        System::{
            Diagnostics::Debug::{
                CONTEXT, EXCEPTION_POINTERS, EXCEPTION_RECORD,
                IMAGE_NT_HEADERS64, IMAGE_SECTION_HEADER,
                PVECTORED_EXCEPTION_HANDLER, XSAVE_AREA_HEADER,
            },
            Kernel::{
                LIST_ENTRY, NT_PRODUCT_TYPE, PROCESSOR_NUMBER,
                RTL_BALANCED_NODE, SINGLE_LIST_ENTRY, STRING,
            },
            Memory::{
                HEAP_INFORMATION_CLASS, HEAP_REALLOC_IN_PLACE_ONLY,
                HEAP_ZERO_MEMORY,
            },
            Performance::HardwareCounterProfiling::PERFORMANCE_DATA,
            SystemInformation::{
                OSVERSIONINFOEXW, OSVERSIONINFOW, OS_DEPLOYEMENT_STATE_VALUES,
            },
            Threading::{
                APC_CALLBACK_FUNCTION, LPTHREAD_START_ROUTINE,
                PFLS_CALLBACK_FUNCTION, RTL_BARRIER, RTL_CONDITION_VARIABLE,
                RTL_CRITICAL_SECTION, RTL_CRITICAL_SECTION_DEBUG, RTL_SRWLOCK,
                WAITORTIMERCALLBACK, WORKERCALLBACKFUNC,
            },
        },
        UI::WindowsAndMessaging::MESSAGE_RESOURCE_ENTRY,
    },
};

use crate::{
    ctypes::{
        __uint64, c_char, c_int, c_long, c_short, c_uchar, c_uint, c_ulong,
        c_ushort, c_void, wchar_t,
    },
    ntapi_base::{CLIENT_ID, PCLIENT_ID, PRTL_ATOM, RTL_ATOM},
    ntexapi::{RTL_PROCESS_BACKTRACES, RTL_PROCESS_LOCKS},
    ntioapi::FILE_INFORMATION_CLASS,
    ntldr::{RTL_PROCESS_MODULES, RTL_PROCESS_MODULE_INFORMATION_EX},
    ntmmapi::SECTION_IMAGE_INFORMATION,
    ntnls::{PCPTABLEINFO, PNLSTABLEINFO},
    ntpebteb::{PPEB, PTEB_ACTIVE_FRAME},
    ntpsapi::{PINITIAL_TEB, PPS_APC_ROUTINE, PS_PROTECTION},
    string::UTF16Const,
    windows_local::shared::ntdef::LARGE_INTEGER,
};

#[inline]
pub fn InitializeListHead(ListHead: &mut LIST_ENTRY) {
    ListHead.Flink = ListHead;
    ListHead.Blink = ListHead;
}
#[inline]
pub fn IsListEmpty(ListHead: &LIST_ENTRY) -> bool {
    eq(ListHead.Flink, ListHead)
}
#[inline]
pub unsafe fn RemoveEntryList(Entry: &mut LIST_ENTRY) -> bool {
    let (Blink, Flink) = (Entry.Blink, Entry.Flink);
    (*Blink).Flink = Flink;
    (*Flink).Blink = Blink;
    Flink == Blink
}
#[inline]
pub unsafe fn RemoveHeadList(ListHead: &mut LIST_ENTRY) -> *mut LIST_ENTRY {
    let Entry = ListHead.Flink;
    let Flink = (*Entry).Flink;
    ListHead.Flink = Flink;
    (*Flink).Blink = ListHead;
    Entry
}
#[inline]
pub unsafe fn RemoveTailList(ListHead: &mut LIST_ENTRY) -> *mut LIST_ENTRY {
    let Entry = ListHead.Blink;
    let Blink = (*Entry).Blink;
    ListHead.Blink = Blink;
    (*Blink).Flink = ListHead;
    Entry
}
#[inline]
pub unsafe fn InsertTailList(
    ListHead: &mut LIST_ENTRY,
    Entry: &mut LIST_ENTRY,
) {
    let Blink = ListHead.Blink;
    Entry.Flink = ListHead;
    Entry.Blink = Blink;
    (*Blink).Flink = Entry;
    ListHead.Blink = Entry;
}
#[inline]
pub unsafe fn InsertHeadList(
    ListHead: &mut LIST_ENTRY,
    Entry: &mut LIST_ENTRY,
) {
    let Flink = ListHead.Flink;
    Entry.Flink = Flink;
    Entry.Blink = ListHead;
    (*Flink).Blink = Entry;
    ListHead.Flink = Entry;
}
#[inline]
pub unsafe fn AppendTailList(
    ListHead: &mut LIST_ENTRY,
    ListToAppend: &mut LIST_ENTRY,
) {
    let ListEnd = ListHead.Blink;
    (*ListHead.Blink).Flink = ListToAppend;
    ListHead.Blink = ListToAppend.Blink;
    (*ListToAppend.Blink).Flink = ListHead;
    ListToAppend.Blink = ListEnd;
}
#[inline]
pub unsafe fn PopEntryList(
    ListHead: &mut SINGLE_LIST_ENTRY,
) -> *mut SINGLE_LIST_ENTRY {
    let FirstEntry = ListHead.Next;
    if !FirstEntry.is_null() {
        ListHead.Next = (*FirstEntry).Next;
    }
    FirstEntry
}
#[inline]
pub fn PushEntryList(
    ListHead: &mut SINGLE_LIST_ENTRY,
    Entry: &mut SINGLE_LIST_ENTRY,
) {
    Entry.Next = ListHead.Next;
    ListHead.Next = Entry;
}
ENUM! {enum TABLE_SEARCH_RESULT {
    TableEmptyTree = 0,
    TableFoundNode = 1,
    TableInsertAsLeft = 2,
    TableInsertAsRight = 3,
}}
ENUM! {enum RTL_GENERIC_COMPARE_RESULTS {
    GenericLessThan = 0,
    GenericGreaterThan = 1,
    GenericEqual = 2,
}}
FN! {stdcall PRTL_AVL_COMPARE_ROUTINE(
    Table: *mut RTL_AVL_TABLE,
    FirstStruct: *mut c_void,
    SecondStruct: *mut c_void,
) -> RTL_GENERIC_COMPARE_RESULTS}
FN! {stdcall PRTL_AVL_ALLOCATE_ROUTINE(
    Table: *mut RTL_AVL_TABLE,
    ByteSize: c_ulong,
) -> *mut c_void}
FN! {stdcall PRTL_AVL_FREE_ROUTINE(
    Table: *mut RTL_AVL_TABLE,
    Buffer: *mut c_void,
) -> ()}
FN! {stdcall PRTL_AVL_MATCH_FUNCTION(
    Table: *mut RTL_AVL_TABLE,
    UserData: *mut c_void,
    MatchData: *mut c_void,
) -> NTSTATUS}
STRUCT! {struct RTL_BALANCED_LINKS {
    Parent: *mut RTL_BALANCED_LINKS,
    LeftChild: *mut RTL_BALANCED_LINKS,
    RightChild: *mut RTL_BALANCED_LINKS,
    Balance: c_char,
    Reserved: [c_uchar; 3],
}}
pub type PRTL_BALANCED_LINKS = *mut RTL_BALANCED_LINKS;
STRUCT! {struct RTL_AVL_TABLE {
    BalancedRoot: RTL_BALANCED_LINKS,
    OrderedPointer: *mut c_void,
    WhichOrderedElement: c_ulong,
    NumberGenericTableElements: c_ulong,
    DepthOfTree: c_ulong,
    RestartKey: PRTL_BALANCED_LINKS,
    DeleteCount: c_ulong,
    CompareRoutine: PRTL_AVL_COMPARE_ROUTINE,
    AllocateRoutine: PRTL_AVL_ALLOCATE_ROUTINE,
    FreeRoutine: PRTL_AVL_FREE_ROUTINE,
    TableContext: *mut c_void,
}}
pub type PRTL_AVL_TABLE = *mut RTL_AVL_TABLE;
EXTERN! {extern "system" {
    fn RtlInitializeGenericTableAvl(
        Table: PRTL_AVL_TABLE,
        CompareRoutine: PRTL_AVL_COMPARE_ROUTINE,
        AllocateRoutine: PRTL_AVL_ALLOCATE_ROUTINE,
        FreeRoutine: PRTL_AVL_FREE_ROUTINE,
        TableContext: *mut c_void,
    );
    fn RtlInsertElementGenericTableAvl(
        Table: PRTL_AVL_TABLE,
        Buffer: *mut c_void,
        BufferSize: c_ulong,
        NewElement: *mut c_uchar,
    ) -> *mut c_void;
    fn RtlInsertElementGenericTableFullAvl(
        Table: PRTL_AVL_TABLE,
        Buffer: *mut c_void,
        BufferSize: c_ulong,
        NewElement: *mut c_uchar,
        NodeOrParent: *mut c_void,
        SearchResult: TABLE_SEARCH_RESULT,
    ) -> *mut c_void;
    fn RtlDeleteElementGenericTableAvl(
        Table: PRTL_AVL_TABLE,
        Buffer: *mut c_void,
    ) -> c_uchar;
    fn RtlLookupElementGenericTableAvl(
        Table: PRTL_AVL_TABLE,
        Buffer: *mut c_void,
    ) -> *mut c_void;
    fn RtlLookupElementGenericTableFullAvl(
        Table: PRTL_AVL_TABLE,
        Buffer: *mut c_void,
        NodeOrParent: *mut *mut c_void,
        SearchResult: *mut TABLE_SEARCH_RESULT,
    ) -> *mut c_void;
    fn RtlEnumerateGenericTableAvl(
        Table: PRTL_AVL_TABLE,
        Restart: c_uchar,
    ) -> *mut c_void;
    fn RtlEnumerateGenericTableWithoutSplayingAvl(
        Table: PRTL_AVL_TABLE,
        RestartKey: *mut *mut c_void,
    ) -> *mut c_void;
    fn RtlLookupFirstMatchingElementGenericTableAvl(
        Table: PRTL_AVL_TABLE,
        Buffer: *mut c_void,
        RestartKey: *mut *mut c_void,
    ) -> *mut c_void;
    fn RtlEnumerateGenericTableLikeADirectory(
        Table: PRTL_AVL_TABLE,
        MatchFunction: PRTL_AVL_MATCH_FUNCTION,
        MatchData: *mut c_void,
        NextFlag: c_ulong,
        RestartKey: *mut *mut c_void,
        DeleteCount: *mut c_ulong,
        Buffer: *mut c_void,
    ) -> *mut c_void;
    fn RtlGetElementGenericTableAvl(
        Table: PRTL_AVL_TABLE,
        I: c_ulong,
    ) -> *mut c_void;
    fn RtlNumberGenericTableElementsAvl(
        Table: PRTL_AVL_TABLE,
    ) -> c_ulong;
    fn RtlIsGenericTableEmptyAvl(
        Table: PRTL_AVL_TABLE,
    ) -> c_uchar;
}}
STRUCT! {struct RTL_SPLAY_LINKS {
    Parent: *mut RTL_SPLAY_LINKS,
    LeftChild: *mut RTL_SPLAY_LINKS,
    RightChild: *mut RTL_SPLAY_LINKS,
}}
pub type PRTL_SPLAY_LINKS = *mut RTL_SPLAY_LINKS;
#[inline]
pub fn RtlInitializeSplayLinks(Links: &mut RTL_SPLAY_LINKS) {
    Links.Parent = Links;
    Links.LeftChild = null_mut();
    Links.RightChild = null_mut();
}
#[inline]
pub const fn RtlParent(Links: &RTL_SPLAY_LINKS) -> PRTL_SPLAY_LINKS {
    Links.Parent
}
#[inline]
pub const fn RtlLeftChild(Links: &RTL_SPLAY_LINKS) -> PRTL_SPLAY_LINKS {
    Links.LeftChild
}
#[inline]
pub const fn RtlRightChild(Links: &RTL_SPLAY_LINKS) -> PRTL_SPLAY_LINKS {
    Links.RightChild
}
#[inline]
pub unsafe fn RtlIsRoot(Links: *const RTL_SPLAY_LINKS) -> bool {
    (*Links).Parent as *const _ == Links
}
#[inline]
pub unsafe fn RtlIsLeftChild(Links: *const RTL_SPLAY_LINKS) -> bool {
    RtlLeftChild(&*RtlParent(&*Links)) as *const _ == Links
}
#[inline]
pub unsafe fn RtlIsRightChild(Links: *const RTL_SPLAY_LINKS) -> bool {
    RtlRightChild(&*RtlParent(&*Links)) as *const _ == Links
}
#[inline]
pub fn RtlInsertAsLeftChild(
    ParentLinks: &mut RTL_SPLAY_LINKS,
    ChildLinks: &mut RTL_SPLAY_LINKS,
) {
    ParentLinks.LeftChild = ChildLinks;
    ChildLinks.Parent = ParentLinks;
}
#[inline]
pub fn RtlInsertAsRightChild(
    ParentLinks: &mut RTL_SPLAY_LINKS,
    ChildLinks: &mut RTL_SPLAY_LINKS,
) {
    ParentLinks.RightChild = ChildLinks;
    ChildLinks.Parent = ParentLinks;
}
EXTERN! {extern "system" {
    fn RtlSplay(
        Links: PRTL_SPLAY_LINKS,
    ) -> PRTL_SPLAY_LINKS;
    fn RtlDelete(
        Links: PRTL_SPLAY_LINKS,
    ) -> PRTL_SPLAY_LINKS;
    fn RtlDeleteNoSplay(
        Links: PRTL_SPLAY_LINKS,
        Root: *mut PRTL_SPLAY_LINKS,
    );
    fn RtlSubtreeSuccessor(
        Links: PRTL_SPLAY_LINKS,
    ) -> PRTL_SPLAY_LINKS;
    fn RtlSubtreePredecessor(
        Links: PRTL_SPLAY_LINKS,
    ) -> PRTL_SPLAY_LINKS;
    fn RtlRealSuccessor(
        Links: PRTL_SPLAY_LINKS,
    ) -> PRTL_SPLAY_LINKS;
    fn RtlRealPredecessor(
        Links: PRTL_SPLAY_LINKS,
    ) -> PRTL_SPLAY_LINKS;
}}
FN! {stdcall PRTL_GENERIC_COMPARE_ROUTINE(
    Table: *mut RTL_GENERIC_TABLE,
    FirstStruct: *mut c_void,
    SecondStruct: *mut c_void,
) -> RTL_GENERIC_COMPARE_RESULTS}
FN! {stdcall PRTL_GENERIC_ALLOCATE_ROUTINE(
    Table: *mut RTL_GENERIC_TABLE,
    ByteSize: c_ulong,
) -> *mut c_void}
FN! {stdcall PRTL_GENERIC_FREE_ROUTINE(
    Table: *mut RTL_GENERIC_TABLE,
    Buffer: *mut c_void,
) -> ()}
STRUCT! {struct RTL_GENERIC_TABLE {
    TableRoot: PRTL_SPLAY_LINKS,
    InsertOrderList: LIST_ENTRY,
    OrderedPointer: *mut LIST_ENTRY,
    WhichOrderedElement: c_ulong,
    NumberGenericTableElements: c_ulong,
    CompareRoutine: PRTL_GENERIC_COMPARE_ROUTINE,
    AllocateRoutine: PRTL_GENERIC_ALLOCATE_ROUTINE,
    FreeRoutine: PRTL_GENERIC_FREE_ROUTINE,
    TableContext: *mut c_void,
}}
pub type PRTL_GENERIC_TABLE = *mut RTL_GENERIC_TABLE;
EXTERN! {extern "system" {
    fn RtlInitializeGenericTable(
        Table: PRTL_GENERIC_TABLE,
        CompareRoutine: PRTL_GENERIC_COMPARE_ROUTINE,
        AllocateRoutine: PRTL_GENERIC_ALLOCATE_ROUTINE,
        FreeRoutine: PRTL_GENERIC_FREE_ROUTINE,
        TableContext: *mut c_void,
    );
    fn RtlInsertElementGenericTable(
        Table: PRTL_GENERIC_TABLE,
        Buffer: *mut c_void,
        BufferSize: c_ulong,
        NewElement: *mut c_uchar,
    ) -> *mut c_void;
    fn RtlInsertElementGenericTableFull(
        Table: PRTL_GENERIC_TABLE,
        Buffer: *mut c_void,
        BufferSize: c_ulong,
        NewElement: *mut c_uchar,
        NodeOrParent: *mut c_void,
        SearchResult: TABLE_SEARCH_RESULT,
    ) -> *mut c_void;
    fn RtlDeleteElementGenericTable(
        Table: PRTL_GENERIC_TABLE,
        Buffer: *mut c_void,
    ) -> c_uchar;
    fn RtlLookupElementGenericTable(
        Table: PRTL_GENERIC_TABLE,
        Buffer: *mut c_void,
    ) -> *mut c_void;
    fn RtlLookupElementGenericTableFull(
        Table: PRTL_GENERIC_TABLE,
        Buffer: *mut c_void,
        NodeOrParent: *mut *mut c_void,
        SearchResult: *mut TABLE_SEARCH_RESULT,
    ) -> *mut c_void;
    fn RtlEnumerateGenericTable(
        Table: PRTL_GENERIC_TABLE,
        Restart: c_uchar,
    ) -> *mut c_void;
    fn RtlEnumerateGenericTableWithoutSplaying(
        Table: PRTL_GENERIC_TABLE,
        RestartKey: *mut *mut c_void,
    ) -> *mut c_void;
    fn RtlGetElementGenericTable(
        Table: PRTL_GENERIC_TABLE,
        I: c_ulong,
    ) -> *mut c_void;
    fn RtlNumberGenericTableElements(
        Table: PRTL_GENERIC_TABLE,
    ) -> c_ulong;
    fn RtlIsGenericTableEmpty(
        Table: PRTL_GENERIC_TABLE,
    ) -> c_uchar;
}}
STRUCT! {struct RTL_RB_TREE {
    Root: *mut RTL_BALANCED_NODE,
    Min: *mut RTL_BALANCED_NODE,
}}
pub type PRTL_RB_TREE = *mut RTL_RB_TREE;
EXTERN! {extern "system" {
    fn RtlRbInsertNodeEx(
        Tree: PRTL_RB_TREE,
        Parent: *mut RTL_BALANCED_NODE,
        Right: c_uchar,
        Node: *mut RTL_BALANCED_NODE,
    );
    fn RtlRbRemoveNode(
        Tree: PRTL_RB_TREE,
        Node: *mut RTL_BALANCED_NODE,
    );
}}
pub const RTL_HASH_ALLOCATED_HEADER: u32 = 0x00000001;
pub const RTL_HASH_RESERVED_SIGNATURE: u32 = 0;
STRUCT! {struct RTL_DYNAMIC_HASH_TABLE_ENTRY {
    Linkage: LIST_ENTRY,
    Signature: usize,
}}
pub type PRTL_DYNAMIC_HASH_TABLE_ENTRY = *mut RTL_DYNAMIC_HASH_TABLE_ENTRY;
#[inline]
pub const fn HASH_ENTRY_KEY(x: &RTL_DYNAMIC_HASH_TABLE_ENTRY) -> usize {
    x.Signature
}
STRUCT! {struct RTL_DYNAMIC_HASH_TABLE_CONTEXT {
    ChainHead: *mut LIST_ENTRY,
    PrevLinkage: *mut LIST_ENTRY,
    Signature: usize,
}}
pub type PRTL_DYNAMIC_HASH_TABLE_CONTEXT = *mut RTL_DYNAMIC_HASH_TABLE_CONTEXT;
STRUCT! {struct RTL_DYNAMIC_HASH_TABLE_ENUMERATOR {
    HashEntry: RTL_DYNAMIC_HASH_TABLE_ENTRY,
    ChainHead: *mut LIST_ENTRY,
    BucketIndex: c_ulong,
}}
pub type PRTL_DYNAMIC_HASH_TABLE_ENUMERATOR =
    *mut RTL_DYNAMIC_HASH_TABLE_ENUMERATOR;
STRUCT! {struct RTL_DYNAMIC_HASH_TABLE {
    Flags: c_ulong,
    Shift: c_ulong,
    TableSize: c_ulong,
    Pivot: c_ulong,
    DivisorMask: c_ulong,
    NumEntries: c_ulong,
    NonEmptyBuckets: c_ulong,
    NumEnumerators: c_ulong,
    Directory: *mut c_void,
}}
pub type PRTL_DYNAMIC_HASH_TABLE = *mut RTL_DYNAMIC_HASH_TABLE;
#[inline]
pub fn RtlInitHashTableContext(Context: &mut RTL_DYNAMIC_HASH_TABLE_CONTEXT) {
    Context.ChainHead = null_mut();
    Context.PrevLinkage = null_mut();
}
#[inline]
pub fn RtlInitHashTableContextFromEnumerator(
    Context: &mut RTL_DYNAMIC_HASH_TABLE_CONTEXT,
    Enumerator: &RTL_DYNAMIC_HASH_TABLE_ENUMERATOR,
) {
    Context.ChainHead = Enumerator.ChainHead;
    Context.PrevLinkage = Enumerator.HashEntry.Linkage.Blink;
}
// RtlReleaseHashTableContext
#[inline]
pub const fn RtlTotalBucketsHashTable(
    HashTable: &RTL_DYNAMIC_HASH_TABLE,
) -> c_ulong {
    HashTable.TableSize
}
#[inline]
pub const fn RtlNonEmptyBucketsHashTable(
    HashTable: &RTL_DYNAMIC_HASH_TABLE,
) -> c_ulong {
    HashTable.NonEmptyBuckets
}
#[inline]
pub const fn RtlEmptyBucketsHashTable(
    HashTable: &RTL_DYNAMIC_HASH_TABLE,
) -> c_ulong {
    HashTable.TableSize - HashTable.NonEmptyBuckets
}
#[inline]
pub const fn RtlTotalEntriesHashTable(
    HashTable: &RTL_DYNAMIC_HASH_TABLE,
) -> c_ulong {
    HashTable.NumEntries
}
#[inline]
pub const fn RtlActiveEnumeratorsHashTable(
    HashTable: &RTL_DYNAMIC_HASH_TABLE,
) -> c_ulong {
    HashTable.NumEnumerators
}
EXTERN! {extern "system" {
    fn RtlCreateHashTable(
        HashTable: *mut PRTL_DYNAMIC_HASH_TABLE,
        Shift: c_ulong,
        Flags: c_ulong,
    ) -> c_uchar;
    fn RtlDeleteHashTable(
        HashTable: PRTL_DYNAMIC_HASH_TABLE,
    );
    fn RtlInsertEntryHashTable(
        HashTable: PRTL_DYNAMIC_HASH_TABLE,
        Entry: PRTL_DYNAMIC_HASH_TABLE_ENTRY,
        Signature: usize,
        Context: PRTL_DYNAMIC_HASH_TABLE_CONTEXT,
    ) -> c_uchar;
    fn RtlRemoveEntryHashTable(
        HashTable: PRTL_DYNAMIC_HASH_TABLE,
        Entry: PRTL_DYNAMIC_HASH_TABLE_ENTRY,
        Context: PRTL_DYNAMIC_HASH_TABLE_CONTEXT,
    ) -> c_uchar;
    fn RtlLookupEntryHashTable(
        HashTable: PRTL_DYNAMIC_HASH_TABLE,
        Signature: usize,
        Context: PRTL_DYNAMIC_HASH_TABLE_CONTEXT,
    ) -> PRTL_DYNAMIC_HASH_TABLE_ENTRY;
    fn RtlGetNextEntryHashTable(
        HashTable: PRTL_DYNAMIC_HASH_TABLE,
        Context: PRTL_DYNAMIC_HASH_TABLE_CONTEXT,
    ) -> PRTL_DYNAMIC_HASH_TABLE_ENTRY;
    fn RtlInitEnumerationHashTable(
        HashTable: PRTL_DYNAMIC_HASH_TABLE,
        Enumerator: PRTL_DYNAMIC_HASH_TABLE_ENUMERATOR,
    ) -> c_uchar;
    fn RtlEnumerateEntryHashTable(
        HashTable: PRTL_DYNAMIC_HASH_TABLE,
        Enumerator: PRTL_DYNAMIC_HASH_TABLE_ENUMERATOR,
    ) -> PRTL_DYNAMIC_HASH_TABLE_ENTRY;
    fn RtlEndEnumerationHashTable(
        HashTable: PRTL_DYNAMIC_HASH_TABLE,
        Enumerator: PRTL_DYNAMIC_HASH_TABLE_ENUMERATOR,
    );
    fn RtlInitWeakEnumerationHashTable(
        HashTable: PRTL_DYNAMIC_HASH_TABLE,
        Enumerator: PRTL_DYNAMIC_HASH_TABLE_ENUMERATOR,
    ) -> c_uchar;
    fn RtlWeaklyEnumerateEntryHashTable(
        HashTable: PRTL_DYNAMIC_HASH_TABLE,
        Enumerator: PRTL_DYNAMIC_HASH_TABLE_ENUMERATOR,
    ) -> PRTL_DYNAMIC_HASH_TABLE_ENTRY;
    fn RtlEndWeakEnumerationHashTable(
        HashTable: PRTL_DYNAMIC_HASH_TABLE,
        Enumerator: PRTL_DYNAMIC_HASH_TABLE_ENUMERATOR,
    );
    fn RtlExpandHashTable(
        HashTable: PRTL_DYNAMIC_HASH_TABLE,
    ) -> c_uchar;
    fn RtlContractHashTable(
        HashTable: PRTL_DYNAMIC_HASH_TABLE,
    ) -> c_uchar;
    fn RtlInitStrongEnumerationHashTable(
        HashTable: PRTL_DYNAMIC_HASH_TABLE,
        Enumerator: PRTL_DYNAMIC_HASH_TABLE_ENUMERATOR,
    ) -> c_uchar;
    fn RtlStronglyEnumerateEntryHashTable(
        HashTable: PRTL_DYNAMIC_HASH_TABLE,
        Enumerator: PRTL_DYNAMIC_HASH_TABLE_ENUMERATOR,
    ) -> PRTL_DYNAMIC_HASH_TABLE_ENTRY;
    fn RtlEndStrongEnumerationHashTable(
        HashTable: PRTL_DYNAMIC_HASH_TABLE,
        Enumerator: PRTL_DYNAMIC_HASH_TABLE_ENUMERATOR,
    );
    fn RtlInitializeCriticalSection(
        CriticalSection: *mut RTL_CRITICAL_SECTION,
    ) -> NTSTATUS;
    fn RtlInitializeCriticalSectionAndSpinCount(
        CriticalSection: *mut RTL_CRITICAL_SECTION,
        SpinCount: c_ulong,
    ) -> NTSTATUS;
    fn RtlDeleteCriticalSection(
        CriticalSection: *mut RTL_CRITICAL_SECTION,
    ) -> NTSTATUS;
    fn RtlEnterCriticalSection(
        CriticalSection: *mut RTL_CRITICAL_SECTION,
    ) -> NTSTATUS;
    fn RtlLeaveCriticalSection(
        CriticalSection: *mut RTL_CRITICAL_SECTION,
    ) -> NTSTATUS;
    fn RtlTryEnterCriticalSection(
        CriticalSection: *mut RTL_CRITICAL_SECTION,
    ) -> c_ulong;
    fn RtlIsCriticalSectionLocked(
        CriticalSection: *mut RTL_CRITICAL_SECTION,
    ) -> c_ulong;
    fn RtlIsCriticalSectionLockedByThread(
        CriticalSection: *mut RTL_CRITICAL_SECTION,
    ) -> c_ulong;
    fn RtlGetCriticalSectionRecursionCount(
        CriticalSection: *mut RTL_CRITICAL_SECTION,
    ) -> c_ulong;
    fn RtlSetCriticalSectionSpinCount(
        CriticalSection: *mut RTL_CRITICAL_SECTION,
        SpinCount: c_ulong,
    ) -> c_ulong;
    fn RtlQueryCriticalSectionOwner(
        EventHandle: HANDLE,
    ) -> HANDLE;
    fn RtlCheckForOrphanedCriticalSections(
        ThreadHandle: HANDLE,
    );
}}
STRUCT! {struct RTL_RESOURCE {
    CriticalSection: RTL_CRITICAL_SECTION,
    SharedSemaphore: HANDLE,
    NumberOfWaitingShared: c_ulong,
    ExclusiveSemaphore: HANDLE,
    NumberOfWaitingExclusive: c_ulong,
    NumberOfActive: c_long,
    ExclusiveOwnerThread: HANDLE,
    Flags: c_ulong,
    DebugInfo: *mut RTL_CRITICAL_SECTION_DEBUG,
}}
pub type PRTL_RESOURCE = *mut RTL_RESOURCE;
pub const RTL_RESOURCE_FLAG_LONG_TERM: c_ulong = 0x00000001;
EXTERN! {extern "system" {
    fn RtlInitializeResource(
        Resource: PRTL_RESOURCE,
    );
    fn RtlDeleteResource(
        Resource: PRTL_RESOURCE,
    );
    fn RtlAcquireResourceShared(
        Resource: PRTL_RESOURCE,
        Wait: c_uchar,
    ) -> c_uchar;
    fn RtlAcquireResourceExclusive(
        Resource: PRTL_RESOURCE,
        Wait: c_uchar,
    ) -> c_uchar;
    fn RtlReleaseResource(
        Resource: PRTL_RESOURCE,
    );
    fn RtlConvertSharedToExclusive(
        Resource: PRTL_RESOURCE,
    );
    fn RtlConvertExclusiveToShared(
        Resource: PRTL_RESOURCE,
    );
    fn RtlInitializeSRWLock(
        SRWLock: *mut RTL_SRWLOCK,
    );
    fn RtlAcquireSRWLockExclusive(
        SRWLock: *mut RTL_SRWLOCK,
    );
    fn RtlAcquireSRWLockShared(
        SRWLock: *mut RTL_SRWLOCK,
    );
    fn RtlReleaseSRWLockExclusive(
        SRWLock: *mut RTL_SRWLOCK,
    );
    fn RtlReleaseSRWLockShared(
        SRWLock: *mut RTL_SRWLOCK,
    );
    fn RtlTryAcquireSRWLockExclusive(
        SRWLock: *mut RTL_SRWLOCK,
    ) -> c_uchar;
    fn RtlTryAcquireSRWLockShared(
        SRWLock: *mut RTL_SRWLOCK,
    ) -> c_uchar;
    fn RtlAcquireReleaseSRWLockExclusive(
        SRWLock: *mut RTL_SRWLOCK,
    );
    fn RtlInitializeConditionVariable(
        ConditionVariable: *mut RTL_CONDITION_VARIABLE,
    );
    fn RtlSleepConditionVariableCS(
        ConditionVariable: *mut RTL_CONDITION_VARIABLE,
        CriticalSection: *mut RTL_CRITICAL_SECTION,
        Timeout: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn RtlSleepConditionVariableSRW(
        ConditionVariable: *mut RTL_CONDITION_VARIABLE,
        SRWLock: *mut RTL_SRWLOCK,
        Timeout: *mut LARGE_INTEGER,
        Flags: c_ulong,
    ) -> NTSTATUS;
    fn RtlWakeConditionVariable(
        ConditionVariable: *mut RTL_CONDITION_VARIABLE,
    );
    fn RtlWakeAllConditionVariable(
        ConditionVariable: *mut RTL_CONDITION_VARIABLE,
    );
}}
pub const RTL_BARRIER_FLAGS_SPIN_ONLY: c_ulong = 0x00000001;
pub const RTL_BARRIER_FLAGS_BLOCK_ONLY: c_ulong = 0x00000002;
pub const RTL_BARRIER_FLAGS_NO_DELETE: c_ulong = 0x00000004;
EXTERN! {extern "system" {
    fn RtlInitBarrier(
        Barrier: *mut RTL_BARRIER,
        TotalThreads: c_ulong,
        SpinCount: c_ulong,
    ) -> NTSTATUS;
    fn RtlDeleteBarrier(
        Barrier: *mut RTL_BARRIER,
    ) -> NTSTATUS;
    fn RtlBarrier(
        Barrier: *mut RTL_BARRIER,
        Flags: c_ulong,
    ) -> c_uchar;
    fn RtlBarrierForDelete(
        Barrier: *mut RTL_BARRIER,
        Flags: c_ulong,
    ) -> c_uchar;
    fn RtlWaitOnAddress(
        Address: *mut c_void,
        CompareAddress: *mut c_void,
        AddressSize: usize,
        Timeout: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn RtlWakeAddressAll(
        Address: *mut c_void,
    );
    fn RtlWakeAddressSingle(
        Address: *mut c_void,
    );
    fn RtlInitString(
        DestinationString: *mut STRING,
        SourceString: *const c_char,
    );
    fn RtlInitStringEx(
        DestinationString: *mut STRING,
        SourceString: *const c_char,
    ) -> NTSTATUS;
    fn RtlInitAnsiString(
        DestinationString: *mut STRING,
        SourceString: *const c_char,
    );
    fn RtlInitAnsiStringEx(
        DestinationString: *mut STRING,
        SourceString: *const c_char,
    ) -> NTSTATUS;
    fn RtlFreeAnsiString(
        AnsiString: *mut STRING,
    );
    fn RtlFreeOemString(
        OemString: *mut STRING,
    );
    fn RtlCopyString(
        DestinationString: *mut STRING,
        SourceString: *const STRING,
    );
    fn RtlUpperChar(
        Character: c_char,
    ) -> c_char;
    fn RtlCompareString(
        String1: *const STRING,
        String2: *const STRING,
        CaseInSensitive: c_uchar,
    ) -> c_long;
    fn RtlEqualString(
        String1: *const STRING,
        String2: *const STRING,
        CaseInSensitive: c_uchar,
    ) -> c_uchar;
    fn RtlPrefixString(
        String1: *const STRING,
        String2: *const STRING,
        CaseInSensitive: c_uchar,
    ) -> c_uchar;
    fn RtlAppendStringToString(
        Destination: *mut STRING,
        Source: *const STRING,
    ) -> NTSTATUS;
    fn RtlAppendAsciizToString(
        Destination: *mut STRING,
        Source: *mut c_char,
    ) -> NTSTATUS;
    fn RtlUpperString(
        DestinationString: *mut STRING,
        SourceString: *const STRING,
    );
}}
#[inline]
pub unsafe fn RtlIsNullOrEmptyUnicodeString(
    String: *mut UNICODE_STRING,
) -> bool {
    String.is_null() || (*String).Length == 0
}
#[inline]
pub fn RtlInitEmptyUnicodeString(
    UnicodeString: &mut UNICODE_STRING,
    Buffer: *mut wchar_t,
    MaximumLength: c_ushort,
) {
    UnicodeString.Buffer = Buffer;
    UnicodeString.MaximumLength = MaximumLength;
    UnicodeString.Length = 0;
}
EXTERN! {extern "system" {
    fn RtlInitUnicodeString(
        DestinationString: *mut UNICODE_STRING,
        SourceString: *const wchar_t,
    );
    fn RtlInitUnicodeStringEx(
        DestinationString: *mut UNICODE_STRING,
        SourceString: *const wchar_t,
    ) -> NTSTATUS;
    fn RtlCreateUnicodeString(
        DestinationString: *mut UNICODE_STRING,
        SourceString: *const wchar_t,
    ) -> c_uchar;
    fn RtlCreateUnicodeStringFromAsciiz(
        DestinationString: *mut UNICODE_STRING,
        SourceString: *mut c_char,
    ) -> c_uchar;
    fn RtlFreeUnicodeString(
        UnicodeString: *mut UNICODE_STRING,
    );
}}
pub const RTL_DUPLICATE_UNICODE_STRING_NULL_TERMINATE: c_ulong = 0x00000001;
pub const RTL_DUPLICATE_UNICODE_STRING_ALLOCATE_NULL_STRING: c_ulong =
    0x00000002;
EXTERN! {extern "system" {
    fn RtlDuplicateUnicodeString(
        Flags: c_ulong,
        StringIn: *const UNICODE_STRING,
        StringOut: *mut UNICODE_STRING,
    ) -> NTSTATUS;
    fn RtlCopyUnicodeString(
        DestinationString: *mut UNICODE_STRING,
        SourceString: *const UNICODE_STRING,
    );
    fn RtlUpcaseUnicodeChar(
        SourceCharacter: wchar_t,
    ) -> wchar_t;
    fn RtlDowncaseUnicodeChar(
        SourceCharacter: wchar_t,
    ) -> wchar_t;
    fn RtlCompareUnicodeString(
        String1: *const UNICODE_STRING,
        String2: *const UNICODE_STRING,
        CaseInSensitive: c_uchar,
    ) -> c_long;
    fn RtlCompareUnicodeStrings(
        String1: *const wchar_t,
        String1Length: usize,
        String2: *const wchar_t,
        String2Length: usize,
        CaseInSensitive: c_uchar,
    ) -> c_long;
    fn RtlEqualUnicodeString(
        String1: *const UNICODE_STRING,
        String2: *const UNICODE_STRING,
        CaseInSensitive: c_uchar,
    ) -> c_uchar;
}}
pub const HASH_STRING_ALGORITHM_DEFAULT: c_ulong = 0;
pub const HASH_STRING_ALGORITHM_X65599: c_ulong = 1;
pub const HASH_STRING_ALGORITHM_INVALID: c_ulong = 0xffffffff;
EXTERN! {extern "system" {
    fn RtlHashUnicodeString(
        String: *const UNICODE_STRING,
        CaseInSensitive: c_uchar,
        HashAlgorithm: c_ulong,
        HashValue: *mut c_ulong,
    ) -> NTSTATUS;
    fn RtlValidateUnicodeString(
        Flags: c_ulong,
        String: *const UNICODE_STRING,
    ) -> NTSTATUS;
    fn RtlPrefixUnicodeString(
        String1: *const UNICODE_STRING,
        String2: *const UNICODE_STRING,
        CaseInSensitive: c_uchar,
    ) -> c_uchar;
    fn RtlSuffixUnicodeString(
        String1: *mut UNICODE_STRING,
        String2: *mut UNICODE_STRING,
        CaseInSensitive: c_uchar,
    ) -> c_uchar;
    fn RtlFindUnicodeSubstring(
        FullString: *mut UNICODE_STRING,
        SearchString: *mut UNICODE_STRING,
        CaseInSensitive: c_uchar,
    ) -> *mut wchar_t;
}}
pub const RTL_FIND_CHAR_IN_UNICODE_STRING_START_AT_END: c_ulong = 0x00000001;
pub const RTL_FIND_CHAR_IN_UNICODE_STRING_COMPLEMENT_CHAR_SET: c_ulong =
    0x00000002;
pub const RTL_FIND_CHAR_IN_UNICODE_STRING_CASE_INSENSITIVE: c_ulong =
    0x00000004;
EXTERN! {extern "system" {
    fn RtlFindCharInUnicodeString(
        Flags: c_ulong,
        StringToSearch: *mut UNICODE_STRING,
        CharSet: *mut UNICODE_STRING,
        NonInclusivePrefixLength: *mut c_ushort,
    ) -> NTSTATUS;
    fn RtlAppendUnicodeStringToString(
        Destination: *mut UNICODE_STRING,
        Source: *const UNICODE_STRING,
    ) -> NTSTATUS;
    fn RtlAppendUnicodeToString(
        Destination: *mut UNICODE_STRING,
        Source: *const wchar_t,
    ) -> NTSTATUS;
    fn RtlUpcaseUnicodeString(
        DestinationString: *mut UNICODE_STRING,
        SourceString: *const UNICODE_STRING,
        AllocateDestinationString: c_uchar,
    ) -> NTSTATUS;
    fn RtlDowncaseUnicodeString(
        DestinationString: *mut UNICODE_STRING,
        SourceString: *const UNICODE_STRING,
        AllocateDestinationString: c_uchar,
    ) -> NTSTATUS;
    fn RtlEraseUnicodeString(
        String: *mut UNICODE_STRING,
    );
    fn RtlAnsiStringToUnicodeString(
        DestinationString: *mut UNICODE_STRING,
        SourceString: *mut STRING,
        AllocateDestinationString: c_uchar,
    ) -> NTSTATUS;
    fn RtlUnicodeStringToAnsiString(
        DestinationString: *mut STRING,
        SourceString: *const UNICODE_STRING,
        AllocateDestinationString: c_uchar,
    ) -> NTSTATUS;
    fn RtlAnsiCharToUnicodeChar(
        SourceCharacter: *mut *mut c_uchar,
    ) -> wchar_t;
    fn RtlUpcaseUnicodeStringToAnsiString(
        DestinationString: *mut STRING,
        SourceString: *mut UNICODE_STRING,
        AllocateDestinationString: c_uchar,
    ) -> NTSTATUS;
    fn RtlOemStringToUnicodeString(
        DestinationString: *mut UNICODE_STRING,
        SourceString: *const STRING,
        AllocateDestinationString: c_uchar,
    ) -> NTSTATUS;
    fn RtlUnicodeStringToOemString(
        DestinationString: *mut STRING,
        SourceString: *const UNICODE_STRING,
        AllocateDestinationString: c_uchar,
    ) -> NTSTATUS;
    fn RtlUpcaseUnicodeStringToOemString(
        DestinationString: *mut STRING,
        SourceString: *mut UNICODE_STRING,
        AllocateDestinationString: c_uchar,
    ) -> NTSTATUS;
    fn RtlUnicodeStringToCountedOemString(
        DestinationString: *mut STRING,
        SourceString: *const UNICODE_STRING,
        AllocateDestinationString: c_uchar,
    ) -> NTSTATUS;
    fn RtlUpcaseUnicodeStringToCountedOemString(
        DestinationString: *mut STRING,
        SourceString: *const UNICODE_STRING,
        AllocateDestinationString: c_uchar,
    ) -> NTSTATUS;
    fn RtlMultiByteToUnicodeN(
        UnicodeString: *mut wchar_t,
        MaxBytesInUnicodeString: c_ulong,
        BytesInUnicodeString: *mut c_ulong,
        MultiByteString: *const c_char,
        BytesInMultiByteString: c_ulong,
    ) -> NTSTATUS;
    fn RtlMultiByteToUnicodeSize(
        BytesInUnicodeString: *mut c_ulong,
        MultiByteString: *const c_char,
        BytesInMultiByteString: c_ulong,
    ) -> NTSTATUS;
    fn RtlUnicodeToMultiByteN(
        MultiByteString: *mut c_char,
        MaxBytesInMultiByteString: c_ulong,
        BytesInMultiByteString: *mut c_ulong,
        UnicodeString: *const wchar_t,
        BytesInUnicodeString: c_ulong,
    ) -> NTSTATUS;
    fn RtlUnicodeToMultiByteSize(
        BytesInMultiByteString: *mut c_ulong,
        UnicodeString: *const wchar_t,
        BytesInUnicodeString: c_ulong,
    ) -> NTSTATUS;
    fn RtlUpcaseUnicodeToMultiByteN(
        MultiByteString: *mut c_char,
        MaxBytesInMultiByteString: c_ulong,
        BytesInMultiByteString: *mut c_ulong,
        UnicodeString: *const wchar_t,
        BytesInUnicodeString: c_ulong,
    ) -> NTSTATUS;
    fn RtlOemToUnicodeN(
        UnicodeString: *mut wchar_t,
        MaxBytesInUnicodeString: c_ulong,
        BytesInUnicodeString: *mut c_ulong,
        OemString: *const c_char,
        BytesInOemString: c_ulong,
    ) -> NTSTATUS;
    fn RtlUnicodeToOemN(
        OemString: *mut c_char,
        MaxBytesInOemString: c_ulong,
        BytesInOemString: *mut c_ulong,
        UnicodeString: *const wchar_t,
        BytesInUnicodeString: c_ulong,
    ) -> NTSTATUS;
    fn RtlUpcaseUnicodeToOemN(
        OemString: *mut c_char,
        MaxBytesInOemString: c_ulong,
        BytesInOemString: *mut c_ulong,
        UnicodeString: *const wchar_t,
        BytesInUnicodeString: c_ulong,
    ) -> NTSTATUS;
    fn RtlConsoleMultiByteToUnicodeN(
        UnicodeString: *mut wchar_t,
        MaxBytesInUnicodeString: c_ulong,
        BytesInUnicodeString: *mut c_ulong,
        MultiByteString: *mut c_char,
        BytesInMultiByteString: c_ulong,
        pdwSpecialChar: *mut c_ulong,
    ) -> NTSTATUS;
    fn RtlUTF8ToUnicodeN(
        UnicodeStringDestination: *mut wchar_t,
        UnicodeStringMaxByteCount: c_ulong,
        UnicodeStringActualByteCount: *mut c_ulong,
        UTF8StringSource: *const c_char,
        UTF8StringByteCount: c_ulong,
    ) -> NTSTATUS;
    fn RtlUnicodeToUTF8N(
        UTF8StringDestination: *mut c_char,
        UTF8StringMaxByteCount: c_ulong,
        UTF8StringActualByteCount: *mut c_ulong,
        UnicodeStringSource: *const wchar_t,
        UnicodeStringByteCount: c_ulong,
    ) -> NTSTATUS;
    fn RtlCustomCPToUnicodeN(
        CustomCP: PCPTABLEINFO,
        UnicodeString: *mut wchar_t,
        MaxBytesInUnicodeString: c_ulong,
        BytesInUnicodeString: *mut c_ulong,
        CustomCPString: *mut c_char,
        BytesInCustomCPString: c_ulong,
    ) -> NTSTATUS;
    fn RtlUnicodeToCustomCPN(
        CustomCP: PCPTABLEINFO,
        CustomCPString: *mut c_char,
        MaxBytesInCustomCPString: c_ulong,
        BytesInCustomCPString: *mut c_ulong,
        UnicodeString: *mut wchar_t,
        BytesInUnicodeString: c_ulong,
    ) -> NTSTATUS;
    fn RtlUpcaseUnicodeToCustomCPN(
        CustomCP: PCPTABLEINFO,
        CustomCPString: *mut c_char,
        MaxBytesInCustomCPString: c_ulong,
        BytesInCustomCPString: *mut c_ulong,
        UnicodeString: *mut wchar_t,
        BytesInUnicodeString: c_ulong,
    ) -> NTSTATUS;
    fn RtlInitCodePageTable(
        TableBase: *mut c_ushort,
        CodePageTable: PCPTABLEINFO,
    );
    fn RtlInitNlsTables(
        AnsiNlsBase: *mut c_ushort,
        OemNlsBase: *mut c_ushort,
        LanguageNlsBase: *mut c_ushort,
        TableInfo: PNLSTABLEINFO,
    );
    fn RtlResetRtlTranslations(
        TableInfo: PNLSTABLEINFO,
    );
    fn RtlIsTextUnicode(
        Buffer: *mut c_void,
        Size: c_ulong,
        Result: *mut c_ulong,
    ) -> c_uchar;
}}
ENUM! {enum RTL_NORM_FORM {
    NormOther = 0x0,
    NormC = 0x1,
    NormD = 0x2,
    NormKC = 0x5,
    NormKD = 0x6,
    NormIdna = 0xd,
    DisallowUnassigned = 0x100,
    NormCDisallowUnassigned = 0x101,
    NormDDisallowUnassigned = 0x102,
    NormKCDisallowUnassigned = 0x105,
    NormKDDisallowUnassigned = 0x106,
    NormIdnaDisallowUnassigned = 0x10d,
}}
EXTERN! {extern "system" {
    fn RtlNormalizeString(
        NormForm: c_ulong,
        SourceString: *const wchar_t,
        SourceStringLength: c_long,
        DestinationString: *mut wchar_t,
        DestinationStringLength: *mut c_long,
    ) -> NTSTATUS;
    fn RtlIsNormalizedString(
        NormForm: c_ulong,
        SourceString: *const wchar_t,
        SourceStringLength: c_long,
        Normalized: *mut c_uchar,
    ) -> NTSTATUS;
    fn RtlIsNameInExpression(
        Expression: *mut UNICODE_STRING,
        Name: *mut UNICODE_STRING,
        IgnoreCase: c_uchar,
        UpcaseTable: *mut wchar_t,
    ) -> c_uchar;
    fn RtlIsNameInUnUpcasedExpression(
        Expression: *mut UNICODE_STRING,
        Name: *mut UNICODE_STRING,
        IgnoreCase: c_uchar,
        UpcaseTable: *mut wchar_t,
    ) -> c_uchar;
    fn RtlEqualDomainName(
        String1: *mut UNICODE_STRING,
        String2: *mut UNICODE_STRING,
    ) -> c_uchar;
    fn RtlEqualComputerName(
        String1: *mut UNICODE_STRING,
        String2: *mut UNICODE_STRING,
    ) -> c_uchar;
    fn RtlDnsHostNameToComputerName(
        ComputerNameString: *mut UNICODE_STRING,
        DnsHostNameString: *mut UNICODE_STRING,
        AllocateComputerNameString: c_uchar,
    ) -> NTSTATUS;
    fn RtlStringFromGUID(
        Guid: *const GUID,
        GuidString: *mut UNICODE_STRING,
    ) -> NTSTATUS;
    fn RtlStringFromGUIDEx(
        Guid: *mut GUID,
        GuidString: *mut UNICODE_STRING,
        AllocateGuidString: c_uchar,
    ) -> NTSTATUS;
    fn RtlGUIDFromString(
        GuidString: *const UNICODE_STRING,
        Guid: *mut GUID,
    ) -> NTSTATUS;
    fn RtlCompareAltitudes(
        Altitude1: *const UNICODE_STRING,
        Altitude2: *const UNICODE_STRING,
    ) -> c_long;
    fn RtlIdnToAscii(
        Flags: c_ulong,
        SourceString: *const wchar_t,
        SourceStringLength: c_long,
        DestinationString: *mut wchar_t,
        DestinationStringLength: *mut c_long,
    ) -> NTSTATUS;
    fn RtlIdnToUnicode(
        Flags: c_ulong,
        SourceString: *const wchar_t,
        SourceStringLength: c_long,
        DestinationString: *mut wchar_t,
        DestinationStringLength: *mut c_long,
    ) -> NTSTATUS;
    fn RtlIdnToNameprepUnicode(
        Flags: c_ulong,
        SourceString: *const wchar_t,
        SourceStringLength: c_long,
        DestinationString: *mut wchar_t,
        DestinationStringLength: *mut c_long,
    ) -> NTSTATUS;
}}
STRUCT! {struct PREFIX_TABLE_ENTRY {
    NodeTypeCode: c_short,
    NameLength: c_short,
    NextPrefixTree: *mut PREFIX_TABLE_ENTRY,
    Links: RTL_SPLAY_LINKS,
    Prefix: *mut STRING,
}}
pub type PPREFIX_TABLE_ENTRY = *mut PREFIX_TABLE_ENTRY;
STRUCT! {struct PREFIX_TABLE {
    NodeTypeCode: c_short,
    NameLength: c_short,
    NextPrefixTree: PPREFIX_TABLE_ENTRY,
}}
pub type PPREFIX_TABLE = *mut PREFIX_TABLE;
EXTERN! {extern "system" {
    fn PfxInitialize(
        PrefixTable: PPREFIX_TABLE,
    );
    fn PfxInsertPrefix(
        PrefixTable: PPREFIX_TABLE,
        Prefix: *mut STRING,
        PrefixTableEntry: PPREFIX_TABLE_ENTRY,
    ) -> c_uchar;
    fn PfxRemovePrefix(
        PrefixTable: PPREFIX_TABLE,
        PrefixTableEntry: PPREFIX_TABLE_ENTRY,
    );
    fn PfxFindPrefix(
        PrefixTable: PPREFIX_TABLE,
        FullName: *mut STRING,
    ) -> PPREFIX_TABLE_ENTRY;
}}
STRUCT! {struct UNICODE_PREFIX_TABLE_ENTRY {
    NodeTypeCode: c_short,
    NameLength: c_short,
    NextPrefixTree: *mut UNICODE_PREFIX_TABLE_ENTRY,
    CaseMatch: *mut UNICODE_PREFIX_TABLE_ENTRY,
    Links: RTL_SPLAY_LINKS,
    Prefix: *mut UNICODE_STRING,
}}
pub type PUNICODE_PREFIX_TABLE_ENTRY = *mut UNICODE_PREFIX_TABLE_ENTRY;
STRUCT! {struct UNICODE_PREFIX_TABLE {
    NodeTypeCode: c_short,
    NameLength: c_short,
    NextPrefixTree: PUNICODE_PREFIX_TABLE_ENTRY,
    LastNextEntry: PUNICODE_PREFIX_TABLE_ENTRY,
}}
pub type PUNICODE_PREFIX_TABLE = *mut UNICODE_PREFIX_TABLE;
EXTERN! {extern "system" {
    fn RtlInitializeUnicodePrefix(
        PrefixTable: PUNICODE_PREFIX_TABLE,
    );
    fn RtlInsertUnicodePrefix(
        PrefixTable: PUNICODE_PREFIX_TABLE,
        Prefix: *mut UNICODE_STRING,
        PrefixTableEntry: PUNICODE_PREFIX_TABLE_ENTRY,
    ) -> c_uchar;
    fn RtlRemoveUnicodePrefix(
        PrefixTable: PUNICODE_PREFIX_TABLE,
        PrefixTableEntry: PUNICODE_PREFIX_TABLE_ENTRY,
    );
    fn RtlFindUnicodePrefix(
        PrefixTable: PUNICODE_PREFIX_TABLE,
        FullName: *const UNICODE_STRING,
        CaseInsensitiveIndex: c_ulong,
    ) -> PUNICODE_PREFIX_TABLE_ENTRY;
    fn RtlNextUnicodePrefix(
        PrefixTable: PUNICODE_PREFIX_TABLE,
        Restart: c_uchar,
    ) -> PUNICODE_PREFIX_TABLE_ENTRY;
}}
STRUCT! {struct COMPRESSED_DATA_INFO {
    CompressionFormatAndEngine: c_ushort,
    CompressionUnitShift: c_uchar,
    ChunkShift: c_uchar,
    ClusterShift: c_uchar,
    Reserved: c_uchar,
    NumberOfChunks: c_ushort,
    CompressedChunkSizes: [c_ulong; 1],
}}
pub type PCOMPRESSED_DATA_INFO = *mut COMPRESSED_DATA_INFO;
EXTERN! {extern "system" {
    fn RtlGetCompressionWorkSpaceSize(
        CompressionFormatAndEngine: c_ushort,
        CompressBufferWorkSpaceSize: *mut c_ulong,
        CompressFragmentWorkSpaceSize: *mut c_ulong,
    ) -> NTSTATUS;
    fn RtlCompressBuffer(
        CompressionFormatAndEngine: c_ushort,
        UncompressedBuffer: *mut c_uchar,
        UncompressedBufferSize: c_ulong,
        CompressedBuffer: *mut c_uchar,
        CompressedBufferSize: c_ulong,
        UncompressedChunkSize: c_ulong,
        FinalCompressedSize: *mut c_ulong,
        WorkSpace: *mut c_void,
    ) -> NTSTATUS;
    fn RtlDecompressBuffer(
        CompressionFormat: c_ushort,
        UncompressedBuffer: *mut c_uchar,
        UncompressedBufferSize: c_ulong,
        CompressedBuffer: *mut c_uchar,
        CompressedBufferSize: c_ulong,
        FinalUncompressedSize: *mut c_ulong,
    ) -> NTSTATUS;
    fn RtlDecompressBufferEx(
        CompressionFormat: c_ushort,
        UncompressedBuffer: *mut c_uchar,
        UncompressedBufferSize: c_ulong,
        CompressedBuffer: *mut c_uchar,
        CompressedBufferSize: c_ulong,
        FinalUncompressedSize: *mut c_ulong,
        WorkSpace: *mut c_void,
    ) -> NTSTATUS;
    fn RtlDecompressFragment(
        CompressionFormat: c_ushort,
        UncompressedFragment: *mut c_uchar,
        UncompressedFragmentSize: c_ulong,
        CompressedBuffer: *mut c_uchar,
        CompressedBufferSize: c_ulong,
        FragmentOffset: c_ulong,
        FinalUncompressedSize: *mut c_ulong,
        WorkSpace: *mut c_void,
    ) -> NTSTATUS;
    fn RtlDescribeChunk(
        CompressionFormat: c_ushort,
        CompressedBuffer: *mut *mut c_uchar,
        EndOfCompressedBufferPlus1: *mut c_uchar,
        ChunkBuffer: *mut *mut c_uchar,
        ChunkSize: *mut c_ulong,
    ) -> NTSTATUS;
    fn RtlReserveChunk(
        CompressionFormat: c_ushort,
        CompressedBuffer: *mut *mut c_uchar,
        EndOfCompressedBufferPlus1: *mut c_uchar,
        ChunkBuffer: *mut *mut c_uchar,
        ChunkSize: c_ulong,
    ) -> NTSTATUS;
    fn RtlDecompressChunks(
        UncompressedBuffer: *mut c_uchar,
        UncompressedBufferSize: c_ulong,
        CompressedBuffer: *mut c_uchar,
        CompressedBufferSize: c_ulong,
        CompressedTail: *mut c_uchar,
        CompressedTailSize: c_ulong,
        CompressedDataInfo: PCOMPRESSED_DATA_INFO,
    ) -> NTSTATUS;
    fn RtlCompressChunks(
        UncompressedBuffer: *mut c_uchar,
        UncompressedBufferSize: c_ulong,
        CompressedBuffer: *mut c_uchar,
        CompressedBufferSize: c_ulong,
        CompressedDataInfo: PCOMPRESSED_DATA_INFO,
        CompressedDataInfoLength: c_ulong,
        WorkSpace: *mut c_void,
    ) -> NTSTATUS;
    fn RtlConvertLCIDToString(
        LcidValue: c_ulong,
        Base: c_ulong,
        Padding: c_ulong,
        pResultBuf: *mut wchar_t,
        Size: c_ulong,
    ) -> NTSTATUS;
    fn RtlIsValidLocaleName(
        LocaleName: *mut wchar_t,
        Flags: c_ulong,
    ) -> c_uchar;
    fn RtlGetParentLocaleName(
        LocaleName: *mut wchar_t,
        ParentLocaleName: *mut UNICODE_STRING,
        Flags: c_ulong,
        AllocateDestinationString: c_uchar,
    ) -> NTSTATUS;
    fn RtlLcidToLocaleName(
        lcid: c_ulong,
        LocaleName: *mut UNICODE_STRING,
        Flags: c_ulong,
        AllocateDestinationString: c_uchar,
    ) -> NTSTATUS;
    fn RtlLocaleNameToLcid(
        LocaleName: *mut wchar_t,
        lcid: *mut c_ulong,
        Flags: c_ulong,
    ) -> NTSTATUS;
    fn RtlLCIDToCultureName(
        Lcid: c_ulong,
        String: *mut UNICODE_STRING,
    ) -> c_uchar;
    fn RtlCultureNameToLCID(
        String: *mut UNICODE_STRING,
        Lcid: *mut c_ulong,
    ) -> c_uchar;
    fn RtlCleanUpTEBLangLists();
    fn RtlGetLocaleFileMappingAddress(
        BaseAddress: *mut *mut c_void,
        DefaultLocaleId: *mut c_ulong,
        DefaultCasingTableSize: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn RtlGetCurrentPeb() -> PPEB;
    fn RtlAcquirePebLock();
    fn RtlReleasePebLock();
    fn RtlTryAcquirePebLock() -> c_ulong;
    fn RtlAllocateFromPeb(
        Size: c_ulong,
        Block: *mut *mut c_void,
    ) -> NTSTATUS;
    fn RtlFreeToPeb(
        Block: *mut c_void,
        Size: c_ulong,
    ) -> NTSTATUS;
}}
pub const DOS_MAX_COMPONENT_LENGTH: u32 = 255;
pub const DOS_MAX_PATH_LENGTH: u32 = DOS_MAX_COMPONENT_LENGTH + 5;
STRUCT! {struct CURDIR {
    DosPath: UNICODE_STRING,
    Handle: HANDLE,
}}
pub type PCURDIR = *mut CURDIR;
pub const RTL_USER_PROC_CURDIR_CLOSE: u32 = 0x00000002;
pub const RTL_USER_PROC_CURDIR_INHERIT: u32 = 0x00000003;
STRUCT! {struct RTL_DRIVE_LETTER_CURDIR {
    Flags: c_ushort,
    Length: c_ushort,
    TimeStamp: c_ulong,
    DosPath: STRING,
}}
pub type PRTL_DRIVE_LETTER_CURDIR = *mut RTL_DRIVE_LETTER_CURDIR;
pub const RTL_MAX_DRIVE_LETTERS: usize = 32;
pub const RTL_DRIVE_LETTER_VALID: c_ushort = 0x0001;
STRUCT! {struct RTL_USER_PROCESS_PARAMETERS {
    MaximumLength: c_ulong,
    Length: c_ulong,
    Flags: c_ulong,
    DebugFlags: c_ulong,
    ConsoleHandle: HANDLE,
    ConsoleFlags: c_ulong,
    StandardInput: HANDLE,
    StandardOutput: HANDLE,
    StandardError: HANDLE,
    CurrentDirectory: CURDIR,
    DllPath: UNICODE_STRING,
    ImagePathName: UNICODE_STRING,
    CommandLine: UNICODE_STRING,
    Environment: *mut c_void,
    StartingX: c_ulong,
    StartingY: c_ulong,
    CountX: c_ulong,
    CountY: c_ulong,
    CountCharsX: c_ulong,
    CountCharsY: c_ulong,
    FillAttribute: c_ulong,
    WindowFlags: c_ulong,
    ShowWindowFlags: c_ulong,
    WindowTitle: UNICODE_STRING,
    DesktopInfo: UNICODE_STRING,
    ShellInfo: UNICODE_STRING,
    RuntimeData: UNICODE_STRING,
    CurrentDirectories: [RTL_DRIVE_LETTER_CURDIR; RTL_MAX_DRIVE_LETTERS],
    EnvironmentSize: usize,
    EnvironmentVersion: usize,
    PackageDependencyData: *mut c_void,
    ProcessGroupId: c_ulong,
    LoaderThreads: c_ulong,
}}
pub type PRTL_USER_PROCESS_PARAMETERS = *mut RTL_USER_PROCESS_PARAMETERS;
pub const RTL_USER_PROC_PARAMS_NORMALIZED: c_ulong = 0x00000001;
pub const RTL_USER_PROC_PROFILE_USER: c_ulong = 0x00000002;
pub const RTL_USER_PROC_PROFILE_KERNEL: c_ulong = 0x00000004;
pub const RTL_USER_PROC_PROFILE_SERVER: c_ulong = 0x00000008;
pub const RTL_USER_PROC_RESERVE_1MB: c_ulong = 0x00000020;
pub const RTL_USER_PROC_RESERVE_16MB: c_ulong = 0x00000040;
pub const RTL_USER_PROC_CASE_SENSITIVE: c_ulong = 0x00000080;
pub const RTL_USER_PROC_DISABLE_HEAP_DECOMMIT: c_ulong = 0x00000100;
pub const RTL_USER_PROC_DLL_REDIRECTION_LOCAL: c_ulong = 0x00001000;
pub const RTL_USER_PROC_APP_MANIFEST_PRESENT: c_ulong = 0x00002000;
pub const RTL_USER_PROC_IMAGE_KEY_MISSING: c_ulong = 0x00004000;
pub const RTL_USER_PROC_OPTIN_PROCESS: c_ulong = 0x00020000;
EXTERN! {extern "system" {
    fn RtlCreateProcessParameters(
        pProcessParameters: *mut PRTL_USER_PROCESS_PARAMETERS,
        ImagePathName: *mut UNICODE_STRING,
        DllPath: *mut UNICODE_STRING,
        CurrentDirectory: *mut UNICODE_STRING,
        CommandLine: *mut UNICODE_STRING,
        Environment: *mut c_void,
        WindowTitle: *mut UNICODE_STRING,
        DesktopInfo: *mut UNICODE_STRING,
        ShellInfo: *mut UNICODE_STRING,
        RuntimeData: *mut UNICODE_STRING,
    ) -> NTSTATUS;
    fn RtlCreateProcessParametersEx(
        pProcessParameters: *mut PRTL_USER_PROCESS_PARAMETERS,
        ImagePathName: *mut UNICODE_STRING,
        DllPath: *mut UNICODE_STRING,
        CurrentDirectory: *mut UNICODE_STRING,
        CommandLine: *mut UNICODE_STRING,
        Environment: *mut c_void,
        WindowTitle: *mut UNICODE_STRING,
        DesktopInfo: *mut UNICODE_STRING,
        ShellInfo: *mut UNICODE_STRING,
        RuntimeData: *mut UNICODE_STRING,
        Flags: c_ulong,
    ) -> NTSTATUS;
    fn RtlDestroyProcessParameters(
        ProcessParameters: PRTL_USER_PROCESS_PARAMETERS,
    ) -> NTSTATUS;
    fn RtlNormalizeProcessParams(
        ProcessParameters: PRTL_USER_PROCESS_PARAMETERS,
    ) -> PRTL_USER_PROCESS_PARAMETERS;
    fn RtlDeNormalizeProcessParams(
        ProcessParameters: PRTL_USER_PROCESS_PARAMETERS,
    ) -> PRTL_USER_PROCESS_PARAMETERS;
}}
STRUCT! {struct RTL_USER_PROCESS_INFORMATION {
    Length: c_ulong,
    Process: HANDLE,
    Thread: HANDLE,
    ClientId: CLIENT_ID,
    ImageInformation: SECTION_IMAGE_INFORMATION,
}}
pub type PRTL_USER_PROCESS_INFORMATION = *mut RTL_USER_PROCESS_INFORMATION;
EXTERN! {extern "system" {
    fn RtlCreateUserProcess(
        NtImagePathName: *mut UNICODE_STRING,
        AttributesDeprecated: c_ulong,
        ProcessParameters: PRTL_USER_PROCESS_PARAMETERS,
        ProcessSecurityDescriptor: PSECURITY_DESCRIPTOR,
        ThreadSecurityDescriptor: PSECURITY_DESCRIPTOR,
        ParentProcess: HANDLE,
        InheritHandles: c_uchar,
        DebugPort: HANDLE,
        TokenHandle: HANDLE,
        ProcessInformation: PRTL_USER_PROCESS_INFORMATION,
    ) -> NTSTATUS;
    fn RtlCreateUserProcessEx(
        NtImagePathName: *mut UNICODE_STRING,
        ProcessParameters: PRTL_USER_PROCESS_PARAMETERS,
        InheritHandles: c_uchar,
        Flags: c_ulong,
        ProcessInformation: PRTL_USER_PROCESS_INFORMATION,
    ) -> NTSTATUS;
    fn RtlExitUserProcess(
        ExitStatus: NTSTATUS,
    );
}}
pub const RTL_CLONE_PROCESS_FLAGS_CREATE_SUSPENDED: c_ulong = 0x00000001;
pub const RTL_CLONE_PROCESS_FLAGS_INHERIT_HANDLES: c_ulong = 0x00000002;
pub const RTL_CLONE_PROCESS_FLAGS_NO_SYNCHRONIZE: c_ulong = 0x00000004;
EXTERN! {extern "system" {
    fn RtlCloneUserProcess(
        ProcessFlags: c_ulong,
        ProcessSecurityDescriptor: PSECURITY_DESCRIPTOR,
        ThreadSecurityDescriptor: PSECURITY_DESCRIPTOR,
        DebugPort: HANDLE,
        ProcessInformation: PRTL_USER_PROCESS_INFORMATION,
    ) -> NTSTATUS;
    fn RtlUpdateClonedCriticalSection(
        CriticalSection: *mut RTL_CRITICAL_SECTION,
    );
    fn RtlUpdateClonedSRWLock(
        SRWLock: *mut RTL_SRWLOCK,
        Shared: c_ulong,
    );
}}
STRUCT! {struct RTLP_PROCESS_REFLECTION_REFLECTION_INFORMATION {
    ReflectionProcessHandle: HANDLE,
    ReflectionThreadHandle: HANDLE,
    ReflectionClientId: CLIENT_ID,
}}
pub type PRTLP_PROCESS_REFLECTION_REFLECTION_INFORMATION =
    *mut RTLP_PROCESS_REFLECTION_REFLECTION_INFORMATION;
EXTERN! {extern "system" {
    fn RtlCreateProcessReflection(
        ProcessHandle: HANDLE,
        Flags: c_ulong,
        StartRoutine: *mut c_void,
        StartContext: *mut c_void,
        EventHandle: HANDLE,
        ReflectionInformation: PRTLP_PROCESS_REFLECTION_REFLECTION_INFORMATION,
    ) -> NTSTATUS;
}}
EXTERN! {extern "C" {
    fn RtlSetProcessIsCritical(
        NewValue: c_uchar,
        OldValue: *mut c_uchar,
        CheckFlag: c_uchar,
    ) -> NTSTATUS;
    fn RtlSetThreadIsCritical(
        NewValue: c_uchar,
        OldValue: *mut c_uchar,
        CheckFlag: c_uchar,
    ) -> NTSTATUS;
}}
EXTERN! {extern "system" {
    fn RtlValidProcessProtection(
        ProcessProtection: PS_PROTECTION,
    ) -> c_uchar;
    fn RtlTestProtectedAccess(
        Source: PS_PROTECTION,
        Target: PS_PROTECTION,
    ) -> c_uchar;
    fn RtlIsCurrentProcess(
        ProcessHandle: HANDLE,
    ) -> c_uchar;
    fn RtlIsCurrentThread(
        ThreadHandle: HANDLE,
    ) -> c_uchar;
}}
FN! {stdcall PUSER_THREAD_START_ROUTINE(
    ThreadParameter: *mut c_void,
) -> NTSTATUS}
EXTERN! {extern "system" {
    fn RtlCreateUserThread(
        Process: HANDLE,
        ThreadSecurityDescriptor: PSECURITY_DESCRIPTOR,
        CreateSuspended: c_uchar,
        ZeroBits: c_ulong,
        MaximumStackSize: usize,
        CommittedStackSize: usize,
        StartAddress: PUSER_THREAD_START_ROUTINE,
        Parameter: *mut c_void,
        Thread: *mut HANDLE,
        ClientId: PCLIENT_ID,
    ) -> NTSTATUS;
    fn RtlExitUserThread(
        ExitStatus: NTSTATUS,
    );
    fn RtlIsCurrentThreadAttachExempt() -> c_uchar;
    fn RtlCreateUserStack(
        CommittedStackSize: usize,
        MaximumStackSize: usize,
        ZeroBits: usize,
        PageSize: usize,
        ReserveAlignment: usize,
        InitialTeb: PINITIAL_TEB,
    ) -> NTSTATUS;
    fn RtlFreeUserStack(
        AllocationBase: *mut c_void,
    ) -> NTSTATUS;
}}
STRUCT! {struct CONTEXT_CHUNK {
    Offset: c_long,
    Length: c_ulong,
}}
pub type PCONTEXT_CHUNK = *mut CONTEXT_CHUNK;
STRUCT! {struct CONTEXT_EX {
    All: CONTEXT_CHUNK,
    Legacy: CONTEXT_CHUNK,
    XState: CONTEXT_CHUNK,
}}
pub type PCONTEXT_EX = *mut CONTEXT_EX;
pub const CONTEXT_EX_LENGTH: usize = 4096;
#[macro_export]
macro_rules! RTL_CONTEXT_EX_OFFSET {
    ($ContextEx:expr, $Chunk:ident) => {
        (*$ContextEx).$Chunk.Offset
    };
}
#[macro_export]
macro_rules! RTL_CONTEXT_EX_LENGTH {
    ($ContextEx:expr, $Chunk:ident) => {
        (*$ContextEx).$Chunk.Length
    };
}
#[macro_export]
macro_rules! RTL_CONTEXT_EX_CHUNK {
    ($Base:expr, $Layout:expr, $Chunk:ident) => {
        ($Base as usize + RTL_CONTEXT_EX_OFFSET!($Layout, $Chunk) as usize)
            as *mut c_void
    };
}
#[macro_export]
macro_rules! RTL_CONTEXT_OFFSET {
    ($Context:expr, $Chunk:ident) => {
        RTL_CONTEXT_EX_OFFSET!(
            ($Context as *const $crate::winapi::um::winnt::CONTEXT).offset(1)
                as *const $crate::ntrtl::CONTEXT_EX,
            $Chunk
        )
    };
}
#[macro_export]
macro_rules! RTL_CONTEXT_LENGTH {
    ($Context:expr, $Chunk:ident) => {
        RTL_CONTEXT_EX_LENGTH!(
            ($Context as *const $crate::winapi::um::winnt::CONTEXT).offset(1)
                as *const $crate::ntrtl::CONTEXT_EX,
            $Chunk
        )
    };
}
#[macro_export]
macro_rules! RTL_CONTEXT_CHUNK {
    ($Context:expr, $Chunk:ident) => {
        RTL_CONTEXT_EX_CHUNK!(
            ($Context as *const $crate::winapi::um::winnt::CONTEXT).offset(1)
                as *const $crate::ntrtl::CONTEXT_EX,
            ($Context as *const $crate::winapi::um::winnt::CONTEXT).offset(1)
                as *const $crate::ntrtl::CONTEXT_EX,
            $Chunk
        )
    };
}
EXTERN! {extern "system" {
    fn RtlInitializeContext(
        Process: HANDLE,
        Context: *mut CONTEXT,
        Parameter: *mut c_void,
        InitialPc: *mut c_void,
        InitialSp: *mut c_void,
    );
    fn RtlInitializeExtendedContext(
        Context: *mut CONTEXT,
        ContextFlags: c_ulong,
        ContextEx: *mut PCONTEXT_EX,
    ) -> c_ulong;
    fn RtlCopyExtendedContext(
        Destination: PCONTEXT_EX,
        ContextFlags: c_ulong,
        Source: PCONTEXT_EX,
    ) -> c_ulong;
    fn RtlGetExtendedContextLength(
        ContextFlags: c_ulong,
        ContextLength: *mut c_ulong,
    ) -> c_ulong;
    fn RtlGetExtendedFeaturesMask(
        ContextEx: PCONTEXT_EX,
    ) -> __uint64;
    fn RtlLocateExtendedFeature(
        ContextEx: PCONTEXT_EX,
        FeatureId: c_ulong,
        Length: *mut c_ulong,
    ) -> *mut c_void;
    fn RtlLocateLegacyContext(
        ContextEx: PCONTEXT_EX,
        Length: *mut c_ulong,
    ) -> *mut CONTEXT;
    fn RtlSetExtendedFeaturesMask(
        ContextEx: PCONTEXT_EX,
        FeatureMask: __uint64,
    );
}}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
EXTERN! {extern "system" {
    fn RtlWow64GetThreadContext(
        ThreadHandle: HANDLE,
        ThreadContext: *mut WOW64_CONTEXT,
    ) -> NTSTATUS;
    fn RtlWow64SetThreadContext(
        ThreadHandle: HANDLE,
        ThreadContext: *mut WOW64_CONTEXT,
    ) -> NTSTATUS;
}}
EXTERN! {extern "system" {
    fn RtlRemoteCall(
        Process: HANDLE,
        Thread: HANDLE,
        CallSite: *mut c_void,
        ArgumentCount: c_ulong,
        Arguments: *mut usize,
        PassContext: c_uchar,
        AlreadySuspended: c_uchar,
    ) -> NTSTATUS;
    fn RtlAddVectoredExceptionHandler(
        First: c_ulong,
        Handler: PVECTORED_EXCEPTION_HANDLER,
    ) -> *mut c_void;
    fn RtlRemoveVectoredExceptionHandler(
        Handle: *mut c_void,
    ) -> c_ulong;
    fn RtlAddVectoredContinueHandler(
        First: c_ulong,
        Handler: PVECTORED_EXCEPTION_HANDLER,
    ) -> *mut c_void;
    fn RtlRemoveVectoredContinueHandler(
        Handle: *mut c_void,
    ) -> c_ulong;
}}
FN! {stdcall PRTLP_UNHANDLED_EXCEPTION_FILTER(
    ExceptionInfo: *mut EXCEPTION_POINTERS,
) -> c_ulong}
EXTERN! {extern "system" {
    fn RtlSetUnhandledExceptionFilter(
        UnhandledExceptionFilter: PRTLP_UNHANDLED_EXCEPTION_FILTER,
    );
    fn RtlUnhandledExceptionFilter(
        ExceptionPointers: *mut EXCEPTION_POINTERS,
    ) -> c_long;
    fn RtlUnhandledExceptionFilter2(
        ExceptionPointers: *mut EXCEPTION_POINTERS,
        Flags: c_ulong,
    ) -> c_long;
    fn RtlKnownExceptionFilter(
        ExceptionPointers: *mut EXCEPTION_POINTERS,
    ) -> c_long;
}}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
IFDEF! {
ENUM!{enum FUNCTION_TABLE_TYPE {
    RF_SORTED = 0,
    RF_UNSORTED = 1,
    RF_CALLBACK = 2,
    RF_KERNEL_DYNAMIC = 3,
}}
STRUCT!{struct DYNAMIC_FUNCTION_TABLE {
    ListEntry: LIST_ENTRY,
    FunctionTable: *mut IMAGE_RUNTIME_FUNCTION_ENTRY,
    TimeStamp: LARGE_INTEGER,
    MinimumAddress: __uint64,
    MaximumAddress: __uint64,
    BaseAddress: __uint64,
    Callback: PGET_RUNTIME_FUNCTION_CALLBACK,
    Context: *mut c_void,
    OutOfProcessCallbackDll: *mut wchar_t,
    Type: FUNCTION_TABLE_TYPE,
    EntryCount: c_ulong,
    TreeNode: RTL_BALANCED_NODE,
}}
pub type PDYNAMIC_FUNCTION_TABLE = *mut DYNAMIC_FUNCTION_TABLE;
EXTERN!{extern "system" {
    fn RtlGetFunctionTableListHead() -> *mut LIST_ENTRY;
}}
}
EXTERN! {extern "system" {
    fn RtlImageNtHeader(
        BaseOfImage: *mut c_void,
    ) -> *mut IMAGE_NT_HEADERS64;
}}
pub const RTL_IMAGE_NT_HEADER_EX_FLAG_NO_RANGE_CHECK: c_ulong = 0x00000001;
EXTERN! {extern "system" {
    fn RtlImageNtHeaderEx(
        Flags: c_ulong,
        BaseOfImage: *mut c_void,
        Size: __uint64,
        OutHeaders: *mut *mut IMAGE_NT_HEADERS64,
    ) -> NTSTATUS;
    fn RtlAddressInSectionTable(
        NtHeaders: *mut IMAGE_NT_HEADERS64,
        BaseOfImage: *mut c_void,
        VirtualAddress: c_ulong,
    ) -> *mut c_void;
    fn RtlSectionTableFromVirtualAddress(
        NtHeaders: *mut IMAGE_NT_HEADERS64,
        BaseOfImage: *mut c_void,
        VirtualAddress: c_ulong,
    ) -> *mut IMAGE_SECTION_HEADER;
    fn RtlImageDirectoryEntryToData(
        BaseOfImage: *mut c_void,
        MappedAsImage: c_uchar,
        DirectoryEntry: c_ushort,
        Size: *mut c_ulong,
    ) -> *mut c_void;
    fn RtlImageRvaToSection(
        NtHeaders: *mut IMAGE_NT_HEADERS64,
        BaseOfImage: *mut c_void,
        Rva: c_ulong,
    ) -> *mut IMAGE_SECTION_HEADER;
    fn RtlImageRvaToVa(
        NtHeaders: *mut IMAGE_NT_HEADERS64,
        BaseOfImage: *mut c_void,
        Rva: c_ulong,
        LastRvaSection: *mut *mut IMAGE_SECTION_HEADER,
    ) -> *mut c_void;
    fn RtlFindExportedRoutineByName(
        BaseOfImage: *mut c_void,
        RoutineName: *mut c_char,
    ) -> *mut c_void;
    fn RtlGuardCheckLongJumpTarget(
        PcValue: *mut c_void,
        IsFastFail: c_int,
        IsLongJumpTarget: *mut c_int,
    ) -> NTSTATUS;
    fn RtlCompareMemoryUlong(
        Source: *mut c_void,
        Length: usize,
        Pattern: c_ulong,
    ) -> usize;
    fn RtlFillMemoryUlong(
        Destination: *mut c_void,
        Length: usize,
        Pattern: c_ulong,
    );
    fn RtlFillMemoryUlonglong(
        Destination: *mut c_void,
        Length: usize,
        Pattern: __uint64,
    );
    fn RtlCreateEnvironment(
        CloneCurrentEnvironment: c_uchar,
        Environment: *mut *mut c_void,
    ) -> NTSTATUS;
}}
pub const RTL_CREATE_ENVIRONMENT_TRANSLATE: c_ulong = 0x1;
pub const RTL_CREATE_ENVIRONMENT_TRANSLATE_FROM_OEM: c_ulong = 0x2;
pub const RTL_CREATE_ENVIRONMENT_EMPTY: c_ulong = 0x4;
EXTERN! {extern "system" {
    fn RtlCreateEnvironmentEx(
        SourceEnv: *mut c_void,
        Environment: *mut *mut c_void,
        Flags: c_ulong,
    ) -> NTSTATUS;
    fn RtlDestroyEnvironment(
        Environment: *mut c_void,
    ) -> NTSTATUS;
    fn RtlSetCurrentEnvironment(
        Environment: *mut c_void,
        PreviousEnvironment: *mut *mut c_void,
    ) -> NTSTATUS;
    fn RtlSetEnvironmentVar(
        Environment: *mut *mut wchar_t,
        Name: *mut wchar_t,
        NameLength: usize,
        Value: *mut wchar_t,
        ValueLength: usize,
    ) -> NTSTATUS;
    fn RtlSetEnvironmentVariable(
        Environment: *mut *mut c_void,
        Name: *mut UNICODE_STRING,
        Value: *mut UNICODE_STRING,
    ) -> NTSTATUS;
    fn RtlQueryEnvironmentVariable(
        Environment: *mut c_void,
        Name: *mut wchar_t,
        NameLength: usize,
        Value: *mut wchar_t,
        ValueLength: usize,
        ReturnLength: *mut usize,
    ) -> NTSTATUS;
    fn RtlQueryEnvironmentVariable_U(
        Environment: *mut c_void,
        Name: *mut UNICODE_STRING,
        Value: *mut UNICODE_STRING,
    ) -> NTSTATUS;
    fn RtlExpandEnvironmentStrings(
        Environment: *mut c_void,
        Src: *mut wchar_t,
        SrcLength: usize,
        Dst: *mut wchar_t,
        DstLength: usize,
        ReturnLength: *mut usize,
    ) -> NTSTATUS;
    fn RtlExpandEnvironmentStrings_U(
        Environment: *mut c_void,
        Source: *mut UNICODE_STRING,
        Destination: *mut UNICODE_STRING,
        ReturnedLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn RtlSetEnvironmentStrings(
        NewEnvironment: *mut wchar_t,
        NewEnvironmentSize: usize,
    ) -> NTSTATUS;
}}
STRUCT! {struct RTLP_CURDIR_REF {
    ReferenceCount: c_long,
    DirectoryHandle: HANDLE,
}}
pub type PRTLP_CURDIR_REF = *mut RTLP_CURDIR_REF;
STRUCT! {struct RTL_RELATIVE_NAME_U {
    RelativeName: UNICODE_STRING,
    ContainingDirectory: HANDLE,
    CurDirRef: PRTLP_CURDIR_REF,
}}
pub type PRTL_RELATIVE_NAME_U = *mut RTL_RELATIVE_NAME_U;
ENUM! {enum RTL_PATH_TYPE {
    RtlPathTypeUnknown = 0,
    RtlPathTypeUncAbsolute = 1,
    RtlPathTypeDriveAbsolute = 2,
    RtlPathTypeDriveRelative = 3,
    RtlPathTypeRooted = 4,
    RtlPathTypeRelative = 5,
    RtlPathTypeLocalDevice = 6,
    RtlPathTypeRootLocalDevice = 7,
}}
EXTERN! {extern "C" {
    static mut RtlDosPathSeperatorsString: UNICODE_STRING;
    static mut RtlAlternateDosPathSeperatorString: UNICODE_STRING;
    static mut RtlNtPathSeperatorString: UNICODE_STRING;
}}
/// "ntdll.dll"
pub const RtlNtdllName: UTF16Const = UTF16Const(&[
    0x006E, 0x0074, 0x0064, 0x006C, 0x006C, 0x002E, 0x0064, 0x006C, 0x006C,
    0u16,
]);
EXTERN! {extern "system" {
    fn RtlDetermineDosPathNameType_U(
        DosFileName: *mut wchar_t,
    ) -> RTL_PATH_TYPE;
    fn RtlDetermineDosPathNameType_Ustr(
        DosFileName: *const UNICODE_STRING,
    ) -> RTL_PATH_TYPE;
    fn RtlIsDosDeviceName_U(
        DosFileName: *mut wchar_t,
    ) -> c_ulong;
    fn RtlIsDosDeviceName_Ustr(
        DosFileName: *mut UNICODE_STRING,
    ) -> c_ulong;
    fn RtlGetFullPathName_U(
        FileName: *mut wchar_t,
        BufferLength: c_ulong,
        Buffer: *mut wchar_t,
        FilePart: *mut *mut wchar_t,
    ) -> c_ulong;
    fn RtlGetFullPathName_UEx(
        FileName: *mut wchar_t,
        BufferLength: c_ulong,
        Buffer: *mut wchar_t,
        FilePart: *mut *mut wchar_t,
        BytesRequired: *mut c_ulong,
    ) -> NTSTATUS;
    fn RtlGetFullPathName_UstrEx(
        FileName: *mut UNICODE_STRING,
        StaticString: *mut UNICODE_STRING,
        DynamicString: *mut UNICODE_STRING,
        StringUsed: *mut *mut UNICODE_STRING,
        FilePartPrefixCch: *mut usize,
        NameInvalid: *mut c_uchar,
        InputPathType: *mut RTL_PATH_TYPE,
        BytesRequired: *mut usize,
    ) -> NTSTATUS;
    fn RtlGetCurrentDirectory_U(
        BufferLength: c_ulong,
        Buffer: *mut wchar_t,
    ) -> c_ulong;
    fn RtlSetCurrentDirectory_U(
        PathName: *mut UNICODE_STRING,
    ) -> NTSTATUS;
    fn RtlGetLongestNtPathLength() -> c_ulong;
    fn RtlDosPathNameToNtPathName_U(
        DosFileName: *mut wchar_t,
        NtFileName: *mut UNICODE_STRING,
        FilePart: *mut *mut wchar_t,
        RelativeName: PRTL_RELATIVE_NAME_U,
    ) -> c_uchar;
    fn RtlDosPathNameToNtPathName_U_WithStatus(
        DosFileName: *mut wchar_t,
        NtFileName: *mut UNICODE_STRING,
        FilePart: *mut *mut wchar_t,
        RelativeName: PRTL_RELATIVE_NAME_U,
    ) -> NTSTATUS;
    fn RtlDosLongPathNameToNtPathName_U_WithStatus(
        DosFileName: *mut wchar_t,
        NtFileName: *mut UNICODE_STRING,
        FilePart: *mut *mut wchar_t,
        RelativeName: PRTL_RELATIVE_NAME_U,
    ) -> NTSTATUS;
    fn RtlDosPathNameToRelativeNtPathName_U(
        DosFileName: *mut wchar_t,
        NtFileName: *mut UNICODE_STRING,
        FilePart: *mut *mut wchar_t,
        RelativeName: PRTL_RELATIVE_NAME_U,
    ) -> c_uchar;
    fn RtlDosPathNameToRelativeNtPathName_U_WithStatus(
        DosFileName: *mut wchar_t,
        NtFileName: *mut UNICODE_STRING,
        FilePart: *mut *mut wchar_t,
        RelativeName: PRTL_RELATIVE_NAME_U,
    ) -> NTSTATUS;
    fn RtlDosLongPathNameToRelativeNtPathName_U_WithStatus(
        DosFileName: *mut wchar_t,
        NtFileName: *mut UNICODE_STRING,
        FilePart: *mut *mut wchar_t,
        RelativeName: PRTL_RELATIVE_NAME_U,
    ) -> NTSTATUS;
    fn RtlReleaseRelativeName(
        RelativeName: PRTL_RELATIVE_NAME_U,
    );
    fn RtlDosSearchPath_U(
        Path: *mut wchar_t,
        FileName: *mut wchar_t,
        Extension: *mut wchar_t,
        BufferLength: c_ulong,
        Buffer: *mut wchar_t,
        FilePart: *mut *mut wchar_t,
    ) -> c_ulong;
}}
pub const RTL_DOS_SEARCH_PATH_FLAG_APPLY_ISOLATION_REDIRECTION: c_ulong =
    0x00000001;
pub const RTL_DOS_SEARCH_PATH_FLAG_DISALLOW_DOT_RELATIVE_PATH_SEARCH: c_ulong =
    0x00000002;
pub const RTL_DOS_SEARCH_PATH_FLAG_APPLY_DEFAULT_EXTENSION_WHEN_NOT_RELATIVE_PATH_EVEN_IF_FILE_HAS_EXTENSION: c_ulong =
    0x00000004;
EXTERN! {extern "system" {
    fn RtlDosSearchPath_Ustr(
        Flags: c_ulong,
        Path: *mut UNICODE_STRING,
        FileName: *mut UNICODE_STRING,
        DefaultExtension: *mut UNICODE_STRING,
        StaticString: *mut UNICODE_STRING,
        DynamicString: *mut UNICODE_STRING,
        FullFileNameOut: *mut *const UNICODE_STRING,
        FilePartPrefixCch: *mut usize,
        BytesRequired: *mut usize,
    ) -> NTSTATUS;
    fn RtlDoesFileExists_U(
        FileName: *mut wchar_t,
    ) -> c_uchar;
    fn RtlGetLengthWithoutLastFullDosOrNtPathElement(
        Flags: c_ulong,
        PathString: *mut UNICODE_STRING,
        Length: *mut c_ulong,
    ) -> NTSTATUS;
    fn RtlGetLengthWithoutTrailingPathSeperators(
        Flags: c_ulong,
        PathString: *mut UNICODE_STRING,
        Length: *mut c_ulong,
    ) -> NTSTATUS;
}}
STRUCT! {struct GENERATE_NAME_CONTEXT {
    Checksum: c_ushort,
    CheckSumInserted: c_uchar,
    NameLength: c_uchar,
    NameBuffer: [wchar_t; 8],
    ExtensionLength: c_ulong,
    ExtensionBuffer: [wchar_t; 4],
    LastIndexValue: c_ulong,
}}
pub type PGENERATE_NAME_CONTEXT = *mut GENERATE_NAME_CONTEXT;
EXTERN! {extern "system" {
    fn RtlGenerate8dot3Name(
        Name: *const UNICODE_STRING,
        AllowExtendedCharacters: c_uchar,
        Context: PGENERATE_NAME_CONTEXT,
        Name8dot3: *mut UNICODE_STRING,
    ) -> NTSTATUS;
    fn RtlComputePrivatizedDllName_U(
        DllName: *mut UNICODE_STRING,
        RealName: *mut UNICODE_STRING,
        LocalName: *mut UNICODE_STRING,
    ) -> NTSTATUS;
    fn RtlGetSearchPath(
        SearchPathA: *mut *mut wchar_t,
    ) -> c_uchar;
    fn RtlSetSearchPathMode(
        Flags: c_ulong,
    ) -> NTSTATUS;
    fn RtlGetExePath() -> *mut wchar_t;
    fn RtlGetNtSystemRoot() -> *mut wchar_t;
    fn RtlAreLongPathsEnabled() -> c_uchar;
    fn RtlIsThreadWithinLoaderCallout() -> c_uchar;
    fn RtlDllShutdownInProgress() -> c_uchar;
}}
STRUCT! {struct RTL_HEAP_ENTRY_u_s1 {
    Settable: usize,
    Tag: c_ulong,
}}
STRUCT! {struct RTL_HEAP_ENTRY_u_s2 {
    CommittedSize: usize,
    FirstBlock: *mut c_void,
}}
UNION! {union RTL_HEAP_ENTRY_u {
    s1: RTL_HEAP_ENTRY_u_s1,
    s2: RTL_HEAP_ENTRY_u_s2,
}}
STRUCT! {struct RTL_HEAP_ENTRY {
    Size: usize,
    Flags: c_ushort,
    AllocatorBackTraceIndex: c_ushort,
    u: RTL_HEAP_ENTRY_u,
}}
pub type PRTL_HEAP_ENTRY = *mut RTL_HEAP_ENTRY;
pub const RTL_HEAP_BUSY: c_ushort = 0x0001;
pub const RTL_HEAP_SEGMENT: c_ushort = 0x0002;
pub const RTL_HEAP_SETTABLE_VALUE: c_ushort = 0x0010;
pub const RTL_HEAP_SETTABLE_FLAG1: c_ushort = 0x0020;
pub const RTL_HEAP_SETTABLE_FLAG2: c_ushort = 0x0040;
pub const RTL_HEAP_SETTABLE_FLAG3: c_ushort = 0x0080;
pub const RTL_HEAP_SETTABLE_FLAGS: c_ushort = 0x00e0;
pub const RTL_HEAP_UNCOMMITTED_RANGE: c_ushort = 0x0100;
pub const RTL_HEAP_PROTECTED_ENTRY: c_ushort = 0x0200;
STRUCT! {struct RTL_HEAP_TAG {
    NumberOfAllocations: c_ulong,
    NumberOfFrees: c_ulong,
    BytesAllocated: usize,
    TagIndex: c_ushort,
    CreatorBackTraceIndex: c_ushort,
    TagName: [wchar_t; 24],
}}
pub type PRTL_HEAP_TAG = *mut RTL_HEAP_TAG;
STRUCT! {struct RTL_HEAP_INFORMATION {
    BaseAddress: *mut c_void,
    Flags: c_ulong,
    EntryOverhead: c_ushort,
    CreatorBackTraceIndex: c_ushort,
    BytesAllocated: usize,
    BytesCommitted: usize,
    NumberOfTags: c_ulong,
    NumberOfEntries: c_ulong,
    NumberOfPseudoTags: c_ulong,
    PseudoTagGranularity: c_ulong,
    Reserved: [c_ulong; 5],
    Tags: PRTL_HEAP_TAG,
    Entries: PRTL_HEAP_ENTRY,
}}
pub type PRTL_HEAP_INFORMATION = *mut RTL_HEAP_INFORMATION;
STRUCT! {struct RTL_PROCESS_HEAPS {
    NumberOfHeaps: c_ulong,
    Heaps: [RTL_HEAP_INFORMATION; 1],
}}
pub type PRTL_PROCESS_HEAPS = *mut RTL_PROCESS_HEAPS;
FN! {stdcall PRTL_HEAP_COMMIT_ROUTINE(
    Base: *mut c_void,
    CommitAddress: *mut *mut c_void,
    CommitSize: *mut usize,
) -> NTSTATUS}
STRUCT! {struct RTL_HEAP_PARAMETERS {
    Length: c_ulong,
    SegmentReserve: usize,
    SegmentCommit: usize,
    DeCommitFreeBlockThreshold: usize,
    DeCommitTotalFreeThreshold: usize,
    MaximumAllocationSize: usize,
    VirtualMemoryThreshold: usize,
    InitialCommit: usize,
    InitialReserve: usize,
    CommitRoutine: PRTL_HEAP_COMMIT_ROUTINE,
    Reserved: [usize; 2],
}}
pub type PRTL_HEAP_PARAMETERS = *mut RTL_HEAP_PARAMETERS;
pub const HEAP_SETTABLE_USER_VALUE: c_ulong = 0x00000100;
pub const HEAP_SETTABLE_USER_FLAG1: c_ulong = 0x00000200;
pub const HEAP_SETTABLE_USER_FLAG2: c_ulong = 0x00000400;
pub const HEAP_SETTABLE_USER_FLAG3: c_ulong = 0x00000800;
pub const HEAP_SETTABLE_USER_FLAGS: c_ulong = 0x00000e00;
pub const HEAP_CLASS_0: c_ulong = 0x00000000;
pub const HEAP_CLASS_1: c_ulong = 0x00001000;
pub const HEAP_CLASS_2: c_ulong = 0x00002000;
pub const HEAP_CLASS_3: c_ulong = 0x00003000;
pub const HEAP_CLASS_4: c_ulong = 0x00004000;
pub const HEAP_CLASS_5: c_ulong = 0x00005000;
pub const HEAP_CLASS_6: c_ulong = 0x00006000;
pub const HEAP_CLASS_7: c_ulong = 0x00007000;
pub const HEAP_CLASS_8: c_ulong = 0x00008000;
pub const HEAP_CLASS_MASK: c_ulong = 0x0000f000;
EXTERN! {extern "system" {
    fn RtlCreateHeap(
        Flags: c_ulong,
        HeapBase: *mut c_void,
        ReserveSize: usize,
        CommitSize: usize,
        Lock: *mut c_void,
        Parameters: PRTL_HEAP_PARAMETERS,
    ) -> *mut c_void;
    fn RtlDestroyHeap(
        HeapHandle: *mut c_void,
    ) -> *mut c_void;
    fn RtlAllocateHeap(
        HeapHandle: *mut c_void,
        Flags: c_ulong,
        Size: usize,
    ) -> *mut c_void;
    fn RtlFreeHeap(
        HeapHandle: *mut c_void,
        Flags: c_ulong,
        BaseAddress: *mut c_void,
    ) -> c_uchar;
    fn RtlSizeHeap(
        HeapHandle: *mut c_void,
        Flags: c_ulong,
        BaseAddress: *mut c_void,
    ) -> usize;
    fn RtlZeroHeap(
        HeapHandle: *mut c_void,
        Flags: c_ulong,
    ) -> NTSTATUS;
    fn RtlProtectHeap(
        HeapHandle: *mut c_void,
        MakeReadOnly: c_uchar,
    );
}}
#[inline]
#[cfg(not(target_arch = "aarch64"))]
pub unsafe fn RtlProcessHeap() -> *mut c_void {
    use crate::ntpsapi::NtCurrentPeb;
    (*NtCurrentPeb()).ProcessHeap
}
EXTERN! {extern "system" {
    fn RtlLockHeap(
        HeapHandle: *mut c_void,
    ) -> c_uchar;
    fn RtlUnlockHeap(
        HeapHandle: *mut c_void,
    ) -> c_uchar;
    fn RtlReAllocateHeap(
        HeapHandle: *mut c_void,
        Flags: c_ulong,
        BaseAddress: *mut c_void,
        Size: usize,
    ) -> *mut c_void;
    fn RtlGetUserInfoHeap(
        HeapHandle: *mut c_void,
        Flags: c_ulong,
        BaseAddress: *mut c_void,
        UserValue: *mut *mut c_void,
        UserFlags: *mut c_ulong,
    ) -> c_uchar;
    fn RtlSetUserValueHeap(
        HeapHandle: *mut c_void,
        Flags: c_ulong,
        BaseAddress: *mut c_void,
        UserValue: *mut c_void,
    ) -> c_uchar;
    fn RtlSetUserFlagsHeap(
        HeapHandle: *mut c_void,
        Flags: c_ulong,
        BaseAddress: *mut c_void,
        UserFlagsReset: c_ulong,
        UserFlagsSet: c_ulong,
    ) -> c_uchar;
}}
STRUCT! {struct RTL_HEAP_TAG_INFO {
    NumberOfAllocations: c_ulong,
    NumberOfFrees: c_ulong,
    BytesAllocated: usize,
}}
pub type PRTL_HEAP_TAG_INFO = *mut RTL_HEAP_TAG_INFO;
EXTERN! {extern "system" {
    fn RtlCreateTagHeap(
        HeapHandle: *mut c_void,
        Flags: c_ulong,
        TagPrefix: *mut wchar_t,
        TagNames: *mut wchar_t,
    ) -> c_ulong;
    fn RtlQueryTagHeap(
        HeapHandle: *mut c_void,
        Flags: c_ulong,
        TagIndex: c_ushort,
        ResetCounters: c_uchar,
        TagInfo: PRTL_HEAP_TAG_INFO,
    ) -> *mut wchar_t;
    fn RtlExtendHeap(
        HeapHandle: *mut c_void,
        Flags: c_ulong,
        Base: *mut c_void,
        Size: usize,
    ) -> NTSTATUS;
    fn RtlCompactHeap(
        HeapHandle: *mut c_void,
        Flags: c_ulong,
    ) -> usize;
    fn RtlValidateHeap(
        HeapHandle: *mut c_void,
        Flags: c_ulong,
        BaseAddress: *mut c_void,
    ) -> c_uchar;
    fn RtlValidateProcessHeaps() -> c_uchar;
    fn RtlGetProcessHeaps(
        NumberOfHeaps: c_ulong,
        ProcessHeaps: *mut *mut c_void,
    ) -> c_ulong;
}}
FN! {stdcall PRTL_ENUM_HEAPS_ROUTINE(
    HeapHandle: *mut c_void,
    Parameter: *mut c_void,
) -> NTSTATUS}
EXTERN! {extern "system" {
    fn RtlEnumProcessHeaps(
        EnumRoutine: PRTL_ENUM_HEAPS_ROUTINE,
        Parameter: *mut c_void,
    ) -> NTSTATUS;
}}
STRUCT! {struct RTL_HEAP_USAGE_ENTRY {
    Next: *mut RTL_HEAP_USAGE_ENTRY,
    Address: *mut c_void,
    Size: usize,
    AllocatorBackTraceIndex: c_ushort,
    TagIndex: c_ushort,
}}
pub type PRTL_HEAP_USAGE_ENTRY = *mut RTL_HEAP_USAGE_ENTRY;
STRUCT! {struct RTL_HEAP_USAGE {
    Length: c_ulong,
    BytesAllocated: usize,
    BytesCommitted: usize,
    BytesReserved: usize,
    BytesReservedMaximum: usize,
    Entries: PRTL_HEAP_USAGE_ENTRY,
    AddedEntries: PRTL_HEAP_USAGE_ENTRY,
    RemovedEntries: PRTL_HEAP_USAGE_ENTRY,
    Reserved: [usize; 8],
}}
pub type PRTL_HEAP_USAGE = *mut RTL_HEAP_USAGE;
pub const HEAP_USAGE_ALLOCATED_BLOCKS: c_ulong = HEAP_REALLOC_IN_PLACE_ONLY;
pub const HEAP_USAGE_FREE_BUFFER: c_ulong = HEAP_ZERO_MEMORY;
EXTERN! {extern "system" {
    fn RtlUsageHeap(
        HeapHandle: *mut c_void,
        Flags: c_ulong,
        Usage: PRTL_HEAP_USAGE,
    ) -> NTSTATUS;
}}
STRUCT! {struct RTL_HEAP_WALK_ENTRY_u_Block {
    Settable: usize,
    TagIndex: c_ushort,
    AllocatorBackTraceIndex: c_ushort,
    Reserved: [c_ulong; 2],
}}
STRUCT! {struct RTL_HEAP_WALK_ENTRY_u_Segment {
    CommittedSize: c_ulong,
    UnCommittedSize: c_ulong,
    FirstEntry: *mut c_void,
    LastEntry: *mut c_void,
}}
UNION! {union RTL_HEAP_WALK_ENTRY_u {
    Block: RTL_HEAP_WALK_ENTRY_u_Block,
    Segment: RTL_HEAP_WALK_ENTRY_u_Segment,
}}
STRUCT! {struct RTL_HEAP_WALK_ENTRY {
    DataAddress: *mut c_void,
    DataSize: usize,
    OverheadBytes: c_uchar,
    SegmentIndex: c_uchar,
    Flags: c_ushort,
    u: RTL_HEAP_WALK_ENTRY_u,
}}
pub type PRTL_HEAP_WALK_ENTRY = *mut RTL_HEAP_WALK_ENTRY;
EXTERN! {extern "system" {
    fn RtlWalkHeap(
        HeapHandle: *mut c_void,
        Entry: PRTL_HEAP_WALK_ENTRY,
    ) -> NTSTATUS;
}}
pub const HeapDetailedFailureInformation: u32 = 0x80000001;
pub const HeapSetDebuggingInformation: u32 = 0x80000002;
ENUM! {enum HEAP_COMPATIBILITY_MODE {
    HEAP_COMPATIBILITY_STANDARD = 0,
    HEAP_COMPATIBILITY_LAL = 1,
    HEAP_COMPATIBILITY_LFH = 2,
}}
STRUCT! {struct PROCESS_HEAP_INFORMATION {
    ReserveSize: usize,
    CommitSize: usize,
    NumberOfHeaps: c_ulong,
    FirstHeapInformationOffset: usize,
}}
pub type PPROCESS_HEAP_INFORMATION = *mut PROCESS_HEAP_INFORMATION;
STRUCT! {struct HEAP_INFORMATION {
    Address: usize,
    Mode: c_ulong,
    ReserveSize: usize,
    CommitSize: usize,
    FirstRegionInformationOffset: usize,
    NextHeapInformationOffset: usize,
}}
pub type PHEAP_INFORMATION = *mut HEAP_INFORMATION;
UNION! {union HEAP_EXTENDED_INFORMATION_u {
    ProcessHeapInformation: PROCESS_HEAP_INFORMATION,
    HeapInformation: HEAP_INFORMATION,
}}
STRUCT! {struct HEAP_EXTENDED_INFORMATION {
    Process: HANDLE,
    Heap: usize,
    Level: c_ulong,
    CallbackRoutine: *mut c_void,
    CallbackContext: *mut c_void,
    u: HEAP_EXTENDED_INFORMATION_u,
}}
pub type PHEAP_EXTENDED_INFORMATION = *mut HEAP_EXTENDED_INFORMATION;
FN! {stdcall PRTL_HEAP_LEAK_ENUMERATION_ROUTINE(
    Reserved: c_long,
    HeapHandle: *mut c_void,
    BaseAddress: *mut c_void,
    BlockSize: usize,
    StackTraceDepth: c_ulong,
    StackTrace: *mut *mut c_void,
) -> NTSTATUS}
STRUCT! {struct HEAP_DEBUGGING_INFORMATION {
    InterceptorFunction: *mut c_void,
    InterceptorValue: c_ushort,
    ExtendedOptions: c_ulong,
    StackTraceDepth: c_ulong,
    MinTotalBlockSize: usize,
    MaxTotalBlockSize: usize,
    HeapLeakEnumerationRoutine: PRTL_HEAP_LEAK_ENUMERATION_ROUTINE,
}}
pub type PHEAP_DEBUGGING_INFORMATION = *mut HEAP_DEBUGGING_INFORMATION;
EXTERN! {extern "system" {
    fn RtlQueryHeapInformation(
        HeapHandle: *mut c_void,
        HeapInformationClass: HEAP_INFORMATION_CLASS,
        HeapInformation: *mut c_void,
        HeapInformationLength: usize,
        ReturnLength: *mut usize,
    ) -> NTSTATUS;
    fn RtlSetHeapInformation(
        HeapHandle: *mut c_void,
        HeapInformationClass: HEAP_INFORMATION_CLASS,
        HeapInformation: *mut c_void,
        HeapInformationLength: usize,
    ) -> NTSTATUS;
    fn RtlMultipleAllocateHeap(
        HeapHandle: *mut c_void,
        Flags: c_ulong,
        Size: usize,
        Count: c_ulong,
        Array: *mut *mut c_void,
    ) -> c_ulong;
    fn RtlMultipleFreeHeap(
        HeapHandle: *mut c_void,
        Flags: c_ulong,
        Count: c_ulong,
        Array: *mut *mut c_void,
    ) -> c_ulong;
    fn RtlDetectHeapLeaks();
    fn RtlFlushHeaps();
}}
STRUCT! {struct RTL_MEMORY_ZONE_SEGMENT {
    NextSegment: *mut RTL_MEMORY_ZONE_SEGMENT,
    Size: usize,
    Next: *mut c_void,
    Limit: *mut c_void,
}}
pub type PRTL_MEMORY_ZONE_SEGMENT = *mut RTL_MEMORY_ZONE_SEGMENT;
STRUCT! {struct RTL_MEMORY_ZONE {
    Segment: RTL_MEMORY_ZONE_SEGMENT,
    Lock: RTL_SRWLOCK,
    LockCount: c_ulong,
    FirstSegment: PRTL_MEMORY_ZONE_SEGMENT,
}}
pub type PRTL_MEMORY_ZONE = *mut RTL_MEMORY_ZONE;
EXTERN! {extern "system" {
    fn RtlCreateMemoryZone(
        MemoryZone: *mut *mut c_void,
        InitialSize: usize,
        Flags: c_ulong,
    ) -> NTSTATUS;
    fn RtlDestroyMemoryZone(
        MemoryZone: *mut c_void,
    ) -> NTSTATUS;
    fn RtlAllocateMemoryZone(
        MemoryZone: *mut c_void,
        BlockSize: usize,
        Block: *mut *mut c_void,
    ) -> NTSTATUS;
    fn RtlResetMemoryZone(
        MemoryZone: *mut c_void,
    ) -> NTSTATUS;
    fn RtlLockMemoryZone(
        MemoryZone: *mut c_void,
    ) -> NTSTATUS;
    fn RtlUnlockMemoryZone(
        MemoryZone: *mut c_void,
    ) -> NTSTATUS;
    fn RtlCreateMemoryBlockLookaside(
        MemoryBlockLookaside: *mut *mut c_void,
        Flags: c_ulong,
        InitialSize: c_ulong,
        MinimumBlockSize: c_ulong,
        MaximumBlockSize: c_ulong,
    ) -> NTSTATUS;
    fn RtlDestroyMemoryBlockLookaside(
        MemoryBlockLookaside: *mut c_void,
    ) -> NTSTATUS;
    fn RtlAllocateMemoryBlockLookaside(
        MemoryBlockLookaside: *mut c_void,
        BlockSize: c_ulong,
        Block: *mut *mut c_void,
    ) -> NTSTATUS;
    fn RtlFreeMemoryBlockLookaside(
        MemoryBlockLookaside: *mut c_void,
        Block: *mut c_void,
    ) -> NTSTATUS;
    fn RtlExtendMemoryBlockLookaside(
        MemoryBlockLookaside: *mut c_void,
        Increment: c_ulong,
    ) -> NTSTATUS;
    fn RtlResetMemoryBlockLookaside(
        MemoryBlockLookaside: *mut c_void,
    ) -> NTSTATUS;
    fn RtlLockMemoryBlockLookaside(
        MemoryBlockLookaside: *mut c_void,
    ) -> NTSTATUS;
    fn RtlUnlockMemoryBlockLookaside(
        MemoryBlockLookaside: *mut c_void,
    ) -> NTSTATUS;
    fn RtlGetCurrentTransaction() -> HANDLE;
    fn RtlSetCurrentTransaction(
        TransactionHandle: HANDLE,
    ) -> c_ulong;
}}
#[inline]
pub const fn RtlIsEqualLuid(L1: &LUID, L2: &LUID) -> bool {
    (L1.LowPart == L2.LowPart) && (L1.HighPart == L2.HighPart)
}
#[inline]
pub const fn RtlIsZeroLuid(L1: &LUID) -> bool {
    (L1.LowPart | L1.HighPart as u32) == 0
}
#[inline]
pub const fn RtlConvertLongToLuid(Long: c_long) -> LUID {
    LUID {
        LowPart: Long as u32,
        HighPart: ((Long as i64) >> 32) as i32,
    }
}
#[inline]
pub const fn RtlConvertUlongToLuid(Ulong: c_ulong) -> LUID {
    LUID {
        LowPart: Ulong,
        HighPart: 0,
    }
}
EXTERN! {extern "system" {
    fn RtlCopyLuid(
        DestinationLuid: *mut LUID,
        SourceLuid: *mut LUID,
    );
    fn RtlCopyLuidAndAttributesArray(
        Count: c_ulong,
        Src: *mut LUID_AND_ATTRIBUTES,
        Dest: *mut LUID_AND_ATTRIBUTES,
    );
}}
STRUCT! {struct RTL_PROCESS_VERIFIER_OPTIONS {
    SizeStruct: c_ulong,
    Option: c_ulong,
    OptionData: [c_uchar; 1],
}}
pub type PRTL_PROCESS_VERIFIER_OPTIONS = *mut RTL_PROCESS_VERIFIER_OPTIONS;
UNION! {union RTL_DEBUG_INFORMATION_u {
    Modules: *mut RTL_PROCESS_MODULES,
    ModulesEx: *mut RTL_PROCESS_MODULE_INFORMATION_EX,
}}
STRUCT! {struct RTL_DEBUG_INFORMATION {
    SectionHandleClient: HANDLE,
    ViewBaseClient: *mut c_void,
    ViewBaseTarget: *mut c_void,
    ViewBaseDelta: usize,
    EventPairClient: HANDLE,
    EventPairTarget: HANDLE,
    TargetProcessId: HANDLE,
    TargetThreadHandle: HANDLE,
    Flags: c_ulong,
    OffsetFree: usize,
    CommitSize: usize,
    ViewSize: usize,
    u: RTL_DEBUG_INFORMATION_u,
    BackTraces: *mut RTL_PROCESS_BACKTRACES,
    Heaps: *mut RTL_PROCESS_HEAPS,
    Locks: *mut RTL_PROCESS_LOCKS,
    SpecificHeap: *mut c_void,
    TargetProcessHandle: HANDLE,
    VerifierOptions: PRTL_PROCESS_VERIFIER_OPTIONS,
    ProcessHeap: *mut c_void,
    CriticalSectionHandle: HANDLE,
    CriticalSectionOwnerThread: HANDLE,
    Reserved: [*mut c_void; 4],
}}
pub type PRTL_DEBUG_INFORMATION = *mut RTL_DEBUG_INFORMATION;
EXTERN! {extern "system" {
    fn RtlCreateQueryDebugBuffer(
        MaximumCommit: c_ulong,
        UseEventPair: c_uchar,
    ) -> PRTL_DEBUG_INFORMATION;
    fn RtlDestroyQueryDebugBuffer(
        Buffer: PRTL_DEBUG_INFORMATION,
    ) -> NTSTATUS;
    fn RtlCommitDebugInfo(
        Buffer: PRTL_DEBUG_INFORMATION,
        Size: usize,
    ) -> *mut c_void;
    fn RtlDeCommitDebugInfo(
        Buffer: PRTL_DEBUG_INFORMATION,
        p: *mut c_void,
        Size: usize,
    );
}}
pub const RTL_QUERY_PROCESS_MODULES: c_ulong = 0x00000001;
pub const RTL_QUERY_PROCESS_BACKTRACES: c_ulong = 0x00000002;
pub const RTL_QUERY_PROCESS_HEAP_SUMMARY: c_ulong = 0x00000004;
pub const RTL_QUERY_PROCESS_HEAP_TAGS: c_ulong = 0x00000008;
pub const RTL_QUERY_PROCESS_HEAP_ENTRIES: c_ulong = 0x00000010;
pub const RTL_QUERY_PROCESS_LOCKS: c_ulong = 0x00000020;
pub const RTL_QUERY_PROCESS_MODULES32: c_ulong = 0x00000040;
pub const RTL_QUERY_PROCESS_VERIFIER_OPTIONS: c_ulong = 0x00000080;
pub const RTL_QUERY_PROCESS_MODULESEX: c_ulong = 0x00000100;
pub const RTL_QUERY_PROCESS_HEAP_ENTRIES_EX: c_ulong = 0x00000200;
pub const RTL_QUERY_PROCESS_CS_OWNER: c_ulong = 0x00000400;
pub const RTL_QUERY_PROCESS_NONINVASIVE: c_ulong = 0x80000000;
EXTERN! {extern "system" {
    fn RtlQueryProcessDebugInformation(
        UniqueProcessId: HANDLE,
        Flags: c_ulong,
        Buffer: PRTL_DEBUG_INFORMATION,
    ) -> NTSTATUS;
    fn RtlFindMessage(
        DllHandle: *mut c_void,
        MessageTableId: c_ulong,
        MessageLanguageId: c_ulong,
        MessageId: c_ulong,
        MessageEntry: *mut *mut MESSAGE_RESOURCE_ENTRY,
    ) -> NTSTATUS;
    fn RtlFormatMessage(
        MessageFormat: *mut wchar_t,
        MaximumWidth: c_ulong,
        IgnoreInserts: c_uchar,
        ArgumentsAreAnsi: c_uchar,
        ArgumentsAreAnArray: c_uchar,
        Arguments: *mut *mut c_char,
        Buffer: *mut wchar_t,
        Length: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
}}
STRUCT! {struct PARSE_MESSAGE_CONTEXT {
    fFlags: c_ulong,
    cwSavColumn: c_ulong,
    iwSrc: usize,
    iwDst: usize,
    iwDstSpace: usize,
    lpvArgStart: *mut c_char,
}}
pub type PPARSE_MESSAGE_CONTEXT = *mut PARSE_MESSAGE_CONTEXT;
#[inline]
pub fn INIT_PARSE_MESSAGE_CONTEXT(ctx: &mut PARSE_MESSAGE_CONTEXT) {
    ctx.fFlags = 0;
}
#[inline]
pub fn TEST_PARSE_MESSAGE_CONTEXT_FLAG(
    ctx: &mut PARSE_MESSAGE_CONTEXT,
    flag: c_ulong,
) -> c_ulong {
    ctx.fFlags & flag
}
#[inline]
pub fn SET_PARSE_MESSAGE_CONTEXT_FLAG(
    ctx: &mut PARSE_MESSAGE_CONTEXT,
    flag: c_ulong,
) -> c_ulong {
    ctx.fFlags |= flag;
    ctx.fFlags
}
#[inline]
pub fn CLEAR_PARSE_MESSAGE_CONTEXT_FLAG(
    ctx: &mut PARSE_MESSAGE_CONTEXT,
    flag: c_ulong,
) -> c_ulong {
    ctx.fFlags &= !flag;
    ctx.fFlags
}
EXTERN! {extern "system" {
    fn RtlFormatMessageEx(
        MessageFormat: *mut wchar_t,
        MaximumWidth: c_ulong,
        IgnoreInserts: c_uchar,
        ArgumentsAreAnsi: c_uchar,
        ArgumentsAreAnArray: c_uchar,
        Arguments: *mut *mut c_char,
        Buffer: *mut wchar_t,
        Length: c_ulong,
        ReturnLength: *mut c_ulong,
        ParseContext: PPARSE_MESSAGE_CONTEXT,
    ) -> NTSTATUS;
    fn RtlNtStatusToDosError(
        Status: NTSTATUS,
    ) -> c_ulong;
    fn RtlNtStatusToDosErrorNoTeb(
        Status: NTSTATUS,
    ) -> c_ulong;
    fn RtlGetLastNtStatus() -> NTSTATUS;
    fn RtlGetLastWin32Error() -> c_long;
    fn RtlSetLastWin32ErrorAndNtStatusFromNtStatus(
        Status: NTSTATUS,
    );
    fn RtlSetLastWin32Error(
        Win32Error: c_long,
    );
    fn RtlRestoreLastWin32Error(
        Win32Error: c_long,
    );
}}
pub const RTL_ERRORMODE_FAILCRITICALERRORS: c_ulong = 0x0010;
pub const RTL_ERRORMODE_NOGPFAULTERRORBOX: c_ulong = 0x0020;
pub const RTL_ERRORMODE_NOOPENFILEERRORBOX: c_ulong = 0x0040;
EXTERN! {extern "system" {
    fn RtlGetThreadErrorMode() -> c_ulong;
    fn RtlSetThreadErrorMode(
        NewMode: c_ulong,
        OldMode: *mut c_ulong,
    ) -> NTSTATUS;
    fn RtlReportException(
        ExceptionRecord: *mut EXCEPTION_RECORD,
        ContextRecord: *mut CONTEXT,
        Flags: c_ulong,
    ) -> NTSTATUS;
    fn RtlReportExceptionEx(
        ExceptionRecord: *mut EXCEPTION_RECORD,
        ContextRecord: *mut CONTEXT,
        Flags: c_ulong,
        Timeout: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn RtlWerpReportException(
        ProcessId: c_ulong,
        CrashReportSharedMem: HANDLE,
        Flags: c_ulong,
        CrashVerticalProcessHandle: *mut HANDLE,
    ) -> NTSTATUS;
    fn RtlReportSilentProcessExit(
        ProcessHandle: HANDLE,
        ExitStatus: NTSTATUS,
    ) -> NTSTATUS;
    fn RtlUniform(
        Seed: *mut c_ulong,
    ) -> c_ulong;
    fn RtlRandom(
        Seed: *mut c_ulong,
    ) -> c_ulong;
    fn RtlRandomEx(
        Seed: *mut c_ulong,
    ) -> c_ulong;
    fn RtlComputeImportTableHash(
        FileHandle: HANDLE,
        Hash: *mut c_char,
        ImportTableHashRevision: c_ulong,
    ) -> NTSTATUS;
    fn RtlIntegerToChar(
        Value: c_ulong,
        Base: c_ulong,
        OutputLength: c_long,
        String: *mut c_char,
    ) -> NTSTATUS;
    fn RtlCharToInteger(
        String: *const c_char,
        Base: c_ulong,
        Value: *mut c_ulong,
    ) -> NTSTATUS;
    fn RtlLargeIntegerToChar(
        Value: *mut LARGE_INTEGER,
        Base: c_ulong,
        OutputLength: c_long,
        String: *mut c_char,
    ) -> NTSTATUS;
    fn RtlIntegerToUnicodeString(
        Value: c_ulong,
        Base: c_ulong,
        String: *mut UNICODE_STRING,
    ) -> NTSTATUS;
    fn RtlInt64ToUnicodeString(
        Value: __uint64,
        Base: c_ulong,
        String: *mut UNICODE_STRING,
    ) -> NTSTATUS;
    fn RtlUnicodeStringToInteger(
        String: *const UNICODE_STRING,
        Base: c_ulong,
        Value: *mut c_ulong,
    ) -> NTSTATUS;
    fn RtlIpv4AddressToStringExW(
        Address: *const IN_ADDR,
        Port: c_ushort,
        AddressString: *mut wchar_t,
        AddressStringLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn RtlIpv6AddressToStringExW(
        Address: *const IN6_ADDR,
        ScopeId: c_ulong,
        Port: c_ushort,
        AddressString: *mut wchar_t,
        AddressStringLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn RtlIpv4StringToAddressExW(
        AddressString: *const wchar_t,
        Strict: c_uchar,
        Address: *mut IN_ADDR,
        Port: *mut c_ushort,
    ) -> NTSTATUS;
    fn RtlIpv6StringToAddressExW(
        AddressString: *const wchar_t,
        Address: *mut IN6_ADDR,
        ScopeId: *mut c_ulong,
        Port: *mut c_ushort,
    ) -> NTSTATUS;
}}
STRUCT! {struct TIME_FIELDS {
    Year: c_short,
    Month: c_short,
    Day: c_short,
    Hour: c_short,
    Minute: c_short,
    Second: c_short,
    Milliseconds: c_short,
    Weekday: c_short,
}}
pub type PTIME_FIELDS = *mut TIME_FIELDS;
EXTERN! {extern "system" {
    fn RtlCutoverTimeToSystemTime(
        CutoverTime: PTIME_FIELDS,
        SystemTime: *mut LARGE_INTEGER,
        CurrentSystemTime: *mut LARGE_INTEGER,
        ThisYear: c_uchar,
    ) -> c_uchar;
    fn RtlSystemTimeToLocalTime(
        SystemTime: *mut LARGE_INTEGER,
        LocalTime: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn RtlLocalTimeToSystemTime(
        LocalTime: *mut LARGE_INTEGER,
        SystemTime: *mut LARGE_INTEGER,
    ) -> NTSTATUS;
    fn RtlTimeToElapsedTimeFields(
        Time: *mut LARGE_INTEGER,
        TimeFields: PTIME_FIELDS,
    );
    fn RtlTimeToTimeFields(
        Time: *mut LARGE_INTEGER,
        TimeFields: PTIME_FIELDS,
    );
    fn RtlTimeFieldsToTime(
        TimeFields: PTIME_FIELDS,
        Time: *mut LARGE_INTEGER,
    ) -> c_uchar;
    fn RtlTimeToSecondsSince1980(
        Time: *mut LARGE_INTEGER,
        ElapsedSeconds: *mut c_ulong,
    ) -> c_uchar;
    fn RtlSecondsSince1980ToTime(
        ElapsedSeconds: c_ulong,
        Time: *mut LARGE_INTEGER,
    );
    fn RtlTimeToSecondsSince1970(
        Time: *mut LARGE_INTEGER,
        ElapsedSeconds: *mut c_ulong,
    ) -> c_uchar;
    fn RtlSecondsSince1970ToTime(
        ElapsedSeconds: c_ulong,
        Time: *mut LARGE_INTEGER,
    );
}}
STRUCT! {struct RTL_TIME_ZONE_INFORMATION {
    Bias: c_long,
    StandardName: [wchar_t; 32],
    StandardStart: TIME_FIELDS,
    StandardBias: c_long,
    DaylightName: [wchar_t; 32],
    DaylightStart: TIME_FIELDS,
    DaylightBias: c_long,
}}
pub type PRTL_TIME_ZONE_INFORMATION = *mut RTL_TIME_ZONE_INFORMATION;
EXTERN! {extern "system" {
    fn RtlQueryTimeZoneInformation(
        TimeZoneInformation: PRTL_TIME_ZONE_INFORMATION,
    ) -> NTSTATUS;
    fn RtlSetTimeZoneInformation(
        TimeZoneInformation: PRTL_TIME_ZONE_INFORMATION,
    ) -> NTSTATUS;
}}
STRUCT! {struct RTL_BITMAP {
    SizeOfBitMap: c_ulong,
    Buffer: *mut c_ulong,
}}
pub type PRTL_BITMAP = *mut RTL_BITMAP;
EXTERN! {extern "system" {
    fn RtlInitializeBitMap(
        BitMapHeader: PRTL_BITMAP,
        BitMapBuffer: *mut c_ulong,
        SizeOfBitMap: c_ulong,
    );
    fn RtlClearBit(
        BitMapHeader: PRTL_BITMAP,
        BitNumber: c_ulong,
    );
    fn RtlSetBit(
        BitMapHeader: PRTL_BITMAP,
        BitNumber: c_ulong,
    );
    fn RtlTestBit(
        BitMapHeader: PRTL_BITMAP,
        BitNumber: c_ulong,
    ) -> c_uchar;
    fn RtlClearAllBits(
        BitMapHeader: PRTL_BITMAP,
    );
    fn RtlSetAllBits(
        BitMapHeader: PRTL_BITMAP,
    );
    fn RtlFindClearBits(
        BitMapHeader: PRTL_BITMAP,
        NumberToFind: c_ulong,
        HintIndex: c_ulong,
    ) -> c_ulong;
    fn RtlFindSetBits(
        BitMapHeader: PRTL_BITMAP,
        NumberToFind: c_ulong,
        HintIndex: c_ulong,
    ) -> c_ulong;
    fn RtlFindClearBitsAndSet(
        BitMapHeader: PRTL_BITMAP,
        NumberToFind: c_ulong,
        HintIndex: c_ulong,
    ) -> c_ulong;
    fn RtlFindSetBitsAndClear(
        BitMapHeader: PRTL_BITMAP,
        NumberToFind: c_ulong,
        HintIndex: c_ulong,
    ) -> c_ulong;
    fn RtlClearBits(
        BitMapHeader: PRTL_BITMAP,
        StartingIndex: c_ulong,
        NumberToClear: c_ulong,
    );
    fn RtlSetBits(
        BitMapHeader: PRTL_BITMAP,
        StartingIndex: c_ulong,
        NumberToSet: c_ulong,
    );
    fn RtlFindMostSignificantBit(
        Set: __uint64,
    ) -> c_char;
    fn RtlFindLeastSignificantBit(
        Set: __uint64,
    ) -> c_char;
}}
STRUCT! {struct RTL_BITMAP_RUN {
    StartingIndex: c_ulong,
    NumberOfBits: c_ulong,
}}
pub type PRTL_BITMAP_RUN = *mut RTL_BITMAP_RUN;
EXTERN! {extern "system" {
    fn RtlFindClearRuns(
        BitMapHeader: PRTL_BITMAP,
        RunArray: PRTL_BITMAP_RUN,
        SizeOfRunArray: c_ulong,
        LocateLongestRuns: c_uchar,
    ) -> c_ulong;
    fn RtlFindLongestRunClear(
        BitMapHeader: PRTL_BITMAP,
        StartingIndex: *mut c_ulong,
    ) -> c_ulong;
    fn RtlFindFirstRunClear(
        BitMapHeader: PRTL_BITMAP,
        StartingIndex: *mut c_ulong,
    ) -> c_ulong;
}}
#[inline]
pub unsafe fn RtlCheckBit(
    BitMapHeader: &RTL_BITMAP,
    BitPosition: c_ulong,
) -> u8 {
    #[cfg(target_arch = "x86_64")]
    {
        core::arch::x86_64::_bittest64(
            BitMapHeader.Buffer as *const i64,
            BitPosition as i64,
        )
    }
    #[cfg(any(target_arch = "x86", target_arch = "aarch64"))]
    {
        (*BitMapHeader.Buffer.offset(BitPosition as isize / 32)
            >> (BitPosition % 32)
            & 1) as u8
    }
}
EXTERN! {extern "system" {
    fn RtlNumberOfClearBits(
        BitMapHeader: PRTL_BITMAP,
    ) -> c_ulong;
    fn RtlNumberOfSetBits(
        BitMapHeader: PRTL_BITMAP,
    ) -> c_ulong;
    fn RtlAreBitsClear(
        BitMapHeader: PRTL_BITMAP,
        StartingIndex: c_ulong,
        Length: c_ulong,
    ) -> c_uchar;
    fn RtlAreBitsSet(
        BitMapHeader: PRTL_BITMAP,
        StartingIndex: c_ulong,
        Length: c_ulong,
    ) -> c_uchar;
    fn RtlFindNextForwardRunClear(
        BitMapHeader: PRTL_BITMAP,
        FromIndex: c_ulong,
        StartingRunIndex: *mut c_ulong,
    ) -> c_ulong;
    fn RtlFindLastBackwardRunClear(
        BitMapHeader: PRTL_BITMAP,
        FromIndex: c_ulong,
        StartingRunIndex: *mut c_ulong,
    ) -> c_ulong;
    fn RtlNumberOfSetBitsUlongPtr(
        Target: usize,
    ) -> c_ulong;
    fn RtlInterlockedClearBitRun(
        BitMapHeader: PRTL_BITMAP,
        StartingIndex: c_ulong,
        NumberToClear: c_ulong,
    );
    fn RtlInterlockedSetBitRun(
        BitMapHeader: PRTL_BITMAP,
        StartingIndex: c_ulong,
        NumberToSet: c_ulong,
    );
    fn RtlCopyBitMap(
        Source: PRTL_BITMAP,
        Destination: PRTL_BITMAP,
        TargetBit: c_ulong,
    );
    fn RtlExtractBitMap(
        Source: PRTL_BITMAP,
        Destination: PRTL_BITMAP,
        TargetBit: c_ulong,
        NumberOfBits: c_ulong,
    );
    fn RtlNumberOfClearBitsInRange(
        BitMapHeader: PRTL_BITMAP,
        StartingIndex: c_ulong,
        Length: c_ulong,
    ) -> c_ulong;
    fn RtlNumberOfSetBitsInRange(
        BitMapHeader: PRTL_BITMAP,
        StartingIndex: c_ulong,
        Length: c_ulong,
    ) -> c_ulong;
}}
STRUCT! {struct RTL_BITMAP_EX {
    SizeOfBitMap: __uint64,
    Buffer: *mut __uint64,
}}
pub type PRTL_BITMAP_EX = *mut RTL_BITMAP_EX;
EXTERN! {extern "system" {
    fn RtlInitializeBitMapEx(
        BitMapHeader: PRTL_BITMAP_EX,
        BitMapBuffer: *mut __uint64,
        SizeOfBitMap: __uint64,
    );
    fn RtlTestBitEx(
        BitMapHeader: PRTL_BITMAP_EX,
        BitNumber: __uint64,
    ) -> c_uchar;
    fn RtlClearAllBitsEx(
        BitMapHeader: PRTL_BITMAP_EX,
    );
    fn RtlClearBitEx(
        BitMapHeader: PRTL_BITMAP_EX,
        BitNumber: __uint64,
    );
    fn RtlSetBitEx(
        BitMapHeader: PRTL_BITMAP_EX,
        BitNumber: __uint64,
    );
    fn RtlFindSetBitsEx(
        BitMapHeader: PRTL_BITMAP_EX,
        NumberToFind: __uint64,
        HintIndex: __uint64,
    ) -> __uint64;
    fn RtlFindSetBitsAndClearEx(
        BitMapHeader: PRTL_BITMAP_EX,
        NumberToFind: __uint64,
        HintIndex: __uint64,
    ) -> __uint64;
}}
UNION! {union RTL_HANDLE_TABLE_ENTRY {
    Flags: c_ulong,
    NextFree: *mut RTL_HANDLE_TABLE_ENTRY,
}}
pub type PRTL_HANDLE_TABLE_ENTRY = *mut RTL_HANDLE_TABLE_ENTRY;
pub const RTL_HANDLE_ALLOCATED: c_ushort = 0x0001;
STRUCT! {struct RTL_HANDLE_TABLE {
    MaximumNumberOfHandles: c_ulong,
    SizeOfHandleTableEntry: c_ulong,
    Reserved: [c_ulong; 2],
    FreeHandles: PRTL_HANDLE_TABLE_ENTRY,
    CommittedHandles: PRTL_HANDLE_TABLE_ENTRY,
    UnCommittedHandles: PRTL_HANDLE_TABLE_ENTRY,
    MaxReservedHandles: PRTL_HANDLE_TABLE_ENTRY,
}}
pub type PRTL_HANDLE_TABLE = *mut RTL_HANDLE_TABLE;
EXTERN! {extern "system" {
    fn RtlInitializeHandleTable(
        MaximumNumberOfHandles: c_ulong,
        SizeOfHandleTableEntry: c_ulong,
        HandleTable: PRTL_HANDLE_TABLE,
    );
    fn RtlDestroyHandleTable(
        HandleTable: PRTL_HANDLE_TABLE,
    ) -> NTSTATUS;
    fn RtlAllocateHandle(
        HandleTable: PRTL_HANDLE_TABLE,
        HandleIndex: *mut c_ulong,
    ) -> PRTL_HANDLE_TABLE_ENTRY;
    fn RtlFreeHandle(
        HandleTable: PRTL_HANDLE_TABLE,
        Handle: PRTL_HANDLE_TABLE_ENTRY,
    ) -> c_uchar;
    fn RtlIsValidHandle(
        HandleTable: PRTL_HANDLE_TABLE,
        Handle: PRTL_HANDLE_TABLE_ENTRY,
    ) -> c_uchar;
    fn RtlIsValidIndexHandle(
        HandleTable: PRTL_HANDLE_TABLE,
        HandleIndex: c_ulong,
        Handle: *mut PRTL_HANDLE_TABLE_ENTRY,
    ) -> c_uchar;
}}
pub const RTL_ATOM_MAXIMUM_INTEGER_ATOM: RTL_ATOM = 0xc000;
pub const RTL_ATOM_INVALID_ATOM: RTL_ATOM = 0x0000;
pub const RTL_ATOM_TABLE_DEFAULT_NUMBER_OF_BUCKETS: u32 = 37;
pub const RTL_ATOM_MAXIMUM_NAME_LENGTH: u32 = 255;
pub const RTL_ATOM_PINNED: u32 = 0x01;
EXTERN! {extern "system" {
    fn RtlCreateAtomTable(
        NumberOfBuckets: c_ulong,
        AtomTableHandle: *mut *mut c_void,
    ) -> NTSTATUS;
    fn RtlDestroyAtomTable(
        AtomTableHandle: *mut c_void,
    ) -> NTSTATUS;
    fn RtlEmptyAtomTable(
        AtomTableHandle: *mut c_void,
        IncludePinnedAtoms: c_uchar,
    ) -> NTSTATUS;
    fn RtlAddAtomToAtomTable(
        AtomTableHandle: *mut c_void,
        AtomName: *mut wchar_t,
        Atom: PRTL_ATOM,
    ) -> NTSTATUS;
    fn RtlLookupAtomInAtomTable(
        AtomTableHandle: *mut c_void,
        AtomName: *mut wchar_t,
        Atom: PRTL_ATOM,
    ) -> NTSTATUS;
    fn RtlDeleteAtomFromAtomTable(
        AtomTableHandle: *mut c_void,
        Atom: RTL_ATOM,
    ) -> NTSTATUS;
    fn RtlPinAtomInAtomTable(
        AtomTableHandle: *mut c_void,
        Atom: RTL_ATOM,
    ) -> NTSTATUS;
    fn RtlQueryAtomInAtomTable(
        AtomTableHandle: *mut c_void,
        Atom: RTL_ATOM,
        AtomUsage: *mut c_ulong,
        AtomFlags: *mut c_ulong,
        AtomName: *mut wchar_t,
        AtomNameLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn RtlGetIntegerAtom(
        AtomName: *mut wchar_t,
        IntegerAtom: *mut c_ushort,
    ) -> c_uchar;
    fn RtlValidSid(
        Sid: PSID,
    ) -> c_uchar;
    fn RtlEqualSid(
        Sid1: PSID,
        Sid2: PSID,
    ) -> c_uchar;
    fn RtlEqualPrefixSid(
        Sid1: PSID,
        Sid2: PSID,
    ) -> c_uchar;
    fn RtlLengthRequiredSid(
        SubAuthorityCount: c_ulong,
    ) -> c_ulong;
    fn RtlFreeSid(
        Sid: PSID,
    ) -> *mut c_void;
    fn RtlAllocateAndInitializeSid(
        IdentifierAuthority: *mut SID_IDENTIFIER_AUTHORITY,
        SubAuthorityCount: c_uchar,
        SubAuthority0: c_ulong,
        SubAuthority1: c_ulong,
        SubAuthority2: c_ulong,
        SubAuthority3: c_ulong,
        SubAuthority4: c_ulong,
        SubAuthority5: c_ulong,
        SubAuthority6: c_ulong,
        SubAuthority7: c_ulong,
        Sid: *mut PSID,
    ) -> NTSTATUS;
    fn RtlInitializeSid(
        Sid: PSID,
        IdentifierAuthority: *mut SID_IDENTIFIER_AUTHORITY,
        SubAuthorityCount: c_uchar,
    ) -> NTSTATUS;
}}
EXTERN! {extern "C" {
    fn RtlInitializeSidEx(
        Sid: PSID,
        IdentifierAuthority: *mut SID_IDENTIFIER_AUTHORITY,
        SubAuthorityCount: c_uchar,
        ...
    ) -> NTSTATUS;
}}
EXTERN! {extern "system" {
    fn RtlIdentifierAuthoritySid(
        Sid: PSID,
    ) -> *mut SID_IDENTIFIER_AUTHORITY;
    fn RtlSubAuthoritySid(
        Sid: PSID,
        SubAuthority: c_ulong,
    ) -> *mut c_ulong;
    fn RtlSubAuthorityCountSid(
        Sid: PSID,
    ) -> *mut c_uchar;
    fn RtlLengthSid(
        Sid: PSID,
    ) -> c_ulong;
    fn RtlCopySid(
        DestinationSidLength: c_ulong,
        DestinationSid: PSID,
        SourceSid: PSID,
    ) -> NTSTATUS;
    fn RtlCopySidAndAttributesArray(
        Count: c_ulong,
        Src: *mut SID_AND_ATTRIBUTES,
        SidAreaSize: c_ulong,
        Dest: *mut SID_AND_ATTRIBUTES,
        SidArea: PSID,
        RemainingSidArea: *mut PSID,
        RemainingSidAreaSize: *mut c_ulong,
    ) -> NTSTATUS;
    fn RtlCreateServiceSid(
        ServiceName: *mut UNICODE_STRING,
        ServiceSid: PSID,
        ServiceSidLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn RtlSidDominates(
        Sid1: PSID,
        Sid2: PSID,
        Dominates: *mut c_uchar,
    ) -> NTSTATUS;
    fn RtlSidDominatesForTrust(
        Sid1: PSID,
        Sid2: PSID,
        DominatesTrust: *mut c_uchar,
    ) -> NTSTATUS;
    fn RtlSidEqualLevel(
        Sid1: PSID,
        Sid2: PSID,
        EqualLevel: *mut c_uchar,
    ) -> NTSTATUS;
    fn RtlSidIsHigherLevel(
        Sid1: PSID,
        Sid2: PSID,
        HigherLevel: *mut c_uchar,
    ) -> NTSTATUS;
    fn RtlCreateVirtualAccountSid(
        Name: *const UNICODE_STRING,
        BaseSubAuthority: c_ulong,
        Sid: PSID,
        SidLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn RtlReplaceSidInSd(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        OldSid: PSID,
        NewSid: PSID,
        NumChanges: *mut c_ulong,
    ) -> NTSTATUS;
}}
pub const MAX_UNICODE_STACK_BUFFER_LENGTH: usize = 256;
EXTERN! {extern "system" {
    fn RtlConvertSidToUnicodeString(
        UnicodeString: *mut UNICODE_STRING,
        Sid: PSID,
        AllocateDestinationString: c_uchar,
    ) -> NTSTATUS;
    fn RtlSidHashInitialize(
        SidAttr: *mut SID_AND_ATTRIBUTES,
        SidCount: c_ulong,
        SidAttrHash: *mut SID_AND_ATTRIBUTES_HASH,
    ) -> NTSTATUS;
    fn RtlSidHashLookup(
        SidAttrHash: *mut SID_AND_ATTRIBUTES_HASH,
        Sid: PSID,
    ) -> *mut SID_AND_ATTRIBUTES;
    fn RtlIsElevatedRid(
        SidAttr: *mut SID_AND_ATTRIBUTES,
    ) -> c_uchar;
    fn RtlDeriveCapabilitySidsFromName(
        UnicodeString: *mut UNICODE_STRING,
        CapabilityGroupSid: PSID,
        CapabilitySid: PSID,
    ) -> NTSTATUS;
    fn RtlCreateSecurityDescriptor(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        Revision: c_ulong,
    ) -> NTSTATUS;
    fn RtlValidSecurityDescriptor(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
    ) -> c_uchar;
    fn RtlLengthSecurityDescriptor(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
    ) -> c_ulong;
    fn RtlValidRelativeSecurityDescriptor(
        SecurityDescriptorInput: PSECURITY_DESCRIPTOR,
        SecurityDescriptorLength: c_ulong,
        RequiredInformation: c_ulong,
    ) -> c_uchar;
    fn RtlGetControlSecurityDescriptor(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        Control: *mut c_ushort,
        Revision: *mut c_ulong,
    ) -> NTSTATUS;
    fn RtlSetControlSecurityDescriptor(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        ControlBitsOfInterest: SECURITY_DESCRIPTOR_CONTROL,
        ControlBitsToSet: SECURITY_DESCRIPTOR_CONTROL,
    ) -> NTSTATUS;
    fn RtlSetAttributesSecurityDescriptor(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        Control: SECURITY_DESCRIPTOR_CONTROL,
        Revision: *mut c_ulong,
    ) -> NTSTATUS;
    fn RtlGetSecurityDescriptorRMControl(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        RMControl: *mut c_uchar,
    ) -> c_uchar;
    fn RtlSetSecurityDescriptorRMControl(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        RMControl: *mut c_uchar,
    );
    fn RtlSetDaclSecurityDescriptor(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        DaclPresent: c_uchar,
        Dacl: *mut ACL,
        DaclDefaulted: c_uchar,
    ) -> NTSTATUS;
    fn RtlGetDaclSecurityDescriptor(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        DaclPresent: *mut c_uchar,
        Dacl: *mut *mut ACL,
        DaclDefaulted: *mut c_uchar,
    ) -> NTSTATUS;
    fn RtlSetSaclSecurityDescriptor(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        SaclPresent: c_uchar,
        Sacl: *mut ACL,
        SaclDefaulted: c_uchar,
    ) -> NTSTATUS;
    fn RtlGetSaclSecurityDescriptor(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        SaclPresent: *mut c_uchar,
        Sacl: *mut *mut ACL,
        SaclDefaulted: *mut c_uchar,
    ) -> NTSTATUS;
    fn RtlSetOwnerSecurityDescriptor(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        Owner: PSID,
        OwnerDefaulted: c_uchar,
    ) -> NTSTATUS;
    fn RtlGetOwnerSecurityDescriptor(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        Owner: *mut PSID,
        OwnerDefaulted: *mut c_uchar,
    ) -> NTSTATUS;
    fn RtlSetGroupSecurityDescriptor(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        Group: PSID,
        GroupDefaulted: c_uchar,
    ) -> NTSTATUS;
    fn RtlGetGroupSecurityDescriptor(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        Group: *mut PSID,
        GroupDefaulted: *mut c_uchar,
    ) -> NTSTATUS;
    fn RtlMakeSelfRelativeSD(
        AbsoluteSecurityDescriptor: PSECURITY_DESCRIPTOR,
        SelfRelativeSecurityDescriptor: PSECURITY_DESCRIPTOR,
        BufferLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn RtlAbsoluteToSelfRelativeSD(
        AbsoluteSecurityDescriptor: PSECURITY_DESCRIPTOR,
        SelfRelativeSecurityDescriptor: PSECURITY_DESCRIPTOR,
        BufferLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn RtlSelfRelativeToAbsoluteSD(
        SelfRelativeSecurityDescriptor: PSECURITY_DESCRIPTOR,
        AbsoluteSecurityDescriptor: PSECURITY_DESCRIPTOR,
        AbsoluteSecurityDescriptorSize: *mut c_ulong,
        Dacl: *mut ACL,
        DaclSize: *mut c_ulong,
        Sacl: *mut ACL,
        SaclSize: *mut c_ulong,
        Owner: PSID,
        OwnerSize: *mut c_ulong,
        PrimaryGroup: PSID,
        PrimaryGroupSize: *mut c_ulong,
    ) -> NTSTATUS;
    fn RtlSelfRelativeToAbsoluteSD2(
        pSelfRelativeSecurityDescriptor: PSECURITY_DESCRIPTOR,
        pBufferSize: *mut c_ulong,
    ) -> NTSTATUS;
    fn RtlAreAllAccessesGranted(
        GrantedAccess: c_ulong,
        DesiredAccess: c_ulong,
    ) -> c_uchar;
    fn RtlAreAnyAccessesGranted(
        GrantedAccess: c_ulong,
        DesiredAccess: c_ulong,
    ) -> c_uchar;
    fn RtlMapGenericMask(
        AccessMask: *mut c_ulong,
        GenericMapping: *mut GENERIC_MAPPING,
    );
    fn RtlCreateAcl(
        Acl: *mut ACL,
        AclLength: c_ulong,
        AclRevision: c_ulong,
    ) -> NTSTATUS;
    fn RtlValidAcl(
        Acl: *mut ACL,
    ) -> c_uchar;
    fn RtlQueryInformationAcl(
        Acl: *mut ACL,
        AclInformation: *mut c_void,
        AclInformationLength: c_ulong,
        AclInformationClass: ACL_INFORMATION_CLASS,
    ) -> NTSTATUS;
    fn RtlSetInformationAcl(
        Acl: *mut ACL,
        AclInformation: *mut c_void,
        AclInformationLength: c_ulong,
        AclInformationClass: ACL_INFORMATION_CLASS,
    ) -> NTSTATUS;
    fn RtlAddAce(
        Acl: *mut ACL,
        AceRevision: c_ulong,
        StartingAceIndex: c_ulong,
        AceList: *mut c_void,
        AceListLength: c_ulong,
    ) -> NTSTATUS;
    fn RtlDeleteAce(
        Acl: *mut ACL,
        AceIndex: c_ulong,
    ) -> NTSTATUS;
    fn RtlGetAce(
        Acl: *mut ACL,
        AceIndex: c_ulong,
        Ace: *mut *mut c_void,
    ) -> NTSTATUS;
    fn RtlFirstFreeAce(
        Acl: *mut ACL,
        FirstFree: *mut *mut c_void,
    ) -> c_uchar;
    fn RtlFindAceByType(
        pAcl: *mut ACL,
        AceType: c_uchar,
        pIndex: *mut c_ulong,
    ) -> *mut c_void;
    fn RtlOwnerAcesPresent(
        pAcl: *mut ACL,
    ) -> c_uchar;
    fn RtlAddAccessAllowedAce(
        Acl: *mut ACL,
        AceRevision: c_ulong,
        AccessMask: c_ulong,
        Sid: PSID,
    ) -> NTSTATUS;
    fn RtlAddAccessAllowedAceEx(
        Acl: *mut ACL,
        AceRevision: c_ulong,
        AceFlags: c_ulong,
        AccessMask: c_ulong,
        Sid: PSID,
    ) -> NTSTATUS;
    fn RtlAddAccessDeniedAce(
        Acl: *mut ACL,
        AceRevision: c_ulong,
        AccessMask: c_ulong,
        Sid: PSID,
    ) -> NTSTATUS;
    fn RtlAddAccessDeniedAceEx(
        Acl: *mut ACL,
        AceRevision: c_ulong,
        AceFlags: c_ulong,
        AccessMask: c_ulong,
        Sid: PSID,
    ) -> NTSTATUS;
    fn RtlAddAuditAccessAce(
        Acl: *mut ACL,
        AceRevision: c_ulong,
        AccessMask: c_ulong,
        Sid: PSID,
        AuditSuccess: c_uchar,
        AuditFailure: c_uchar,
    ) -> NTSTATUS;
    fn RtlAddAuditAccessAceEx(
        Acl: *mut ACL,
        AceRevision: c_ulong,
        AceFlags: c_ulong,
        AccessMask: c_ulong,
        Sid: PSID,
        AuditSuccess: c_uchar,
        AuditFailure: c_uchar,
    ) -> NTSTATUS;
    fn RtlAddAccessAllowedObjectAce(
        Acl: *mut ACL,
        AceRevision: c_ulong,
        AceFlags: c_ulong,
        AccessMask: c_ulong,
        ObjectTypeGuid: *mut GUID,
        InheritedObjectTypeGuid: *mut GUID,
        Sid: PSID,
    ) -> NTSTATUS;
    fn RtlAddAccessDeniedObjectAce(
        Acl: *mut ACL,
        AceRevision: c_ulong,
        AceFlags: c_ulong,
        AccessMask: c_ulong,
        ObjectTypeGuid: *mut GUID,
        InheritedObjectTypeGuid: *mut GUID,
        Sid: PSID,
    ) -> NTSTATUS;
    fn RtlAddAuditAccessObjectAce(
        Acl: *mut ACL,
        AceRevision: c_ulong,
        AceFlags: c_ulong,
        AccessMask: c_ulong,
        ObjectTypeGuid: *mut GUID,
        InheritedObjectTypeGuid: *mut GUID,
        Sid: PSID,
        AuditSuccess: c_uchar,
        AuditFailure: c_uchar,
    ) -> NTSTATUS;
    fn RtlAddCompoundAce(
        Acl: *mut ACL,
        AceRevision: c_ulong,
        AceType: c_uchar,
        AccessMask: c_ulong,
        ServerSid: PSID,
        ClientSid: PSID,
    ) -> NTSTATUS;
    fn RtlAddMandatoryAce(
        Acl: *mut ACL,
        AceRevision: c_ulong,
        AceFlags: c_ulong,
        Sid: PSID,
        AceType: c_uchar,
        AccessMask: c_ulong,
    ) -> NTSTATUS;
    fn RtlDefaultNpAcl(
        Acl: *mut *mut ACL,
    ) -> NTSTATUS;
    fn RtlNewSecurityObject(
        ParentDescriptor: PSECURITY_DESCRIPTOR,
        CreatorDescriptor: PSECURITY_DESCRIPTOR,
        NewDescriptor: *mut PSECURITY_DESCRIPTOR,
        IsDirectoryObject: c_uchar,
        Token: HANDLE,
        GenericMapping: *mut GENERIC_MAPPING,
    ) -> NTSTATUS;
    fn RtlNewSecurityObjectEx(
        ParentDescriptor: PSECURITY_DESCRIPTOR,
        CreatorDescriptor: PSECURITY_DESCRIPTOR,
        NewDescriptor: *mut PSECURITY_DESCRIPTOR,
        ObjectType: *mut GUID,
        IsDirectoryObject: c_uchar,
        AutoInheritFlags: c_ulong,
        Token: HANDLE,
        GenericMapping: *mut GENERIC_MAPPING,
    ) -> NTSTATUS;
    fn RtlNewSecurityObjectWithMultipleInheritance(
        ParentDescriptor: PSECURITY_DESCRIPTOR,
        CreatorDescriptor: PSECURITY_DESCRIPTOR,
        NewDescriptor: *mut PSECURITY_DESCRIPTOR,
        ObjectType: *mut *mut GUID,
        GuidCount: c_ulong,
        IsDirectoryObject: c_uchar,
        AutoInheritFlags: c_ulong,
        Token: HANDLE,
        GenericMapping: *mut GENERIC_MAPPING,
    ) -> NTSTATUS;
    fn RtlDeleteSecurityObject(
        ObjectDescriptor: *mut PSECURITY_DESCRIPTOR,
    ) -> NTSTATUS;
    fn RtlQuerySecurityObject(
        ObjectDescriptor: PSECURITY_DESCRIPTOR,
        SecurityInformation: c_ulong,
        ResultantDescriptor: PSECURITY_DESCRIPTOR,
        DescriptorLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn RtlSetSecurityObject(
        SecurityInformation: c_ulong,
        ModificationDescriptor: PSECURITY_DESCRIPTOR,
        ObjectsSecurityDescriptor: *mut PSECURITY_DESCRIPTOR,
        GenericMapping: *mut GENERIC_MAPPING,
        Token: HANDLE,
    ) -> NTSTATUS;
    fn RtlSetSecurityObjectEx(
        SecurityInformation: c_ulong,
        ModificationDescriptor: PSECURITY_DESCRIPTOR,
        ObjectsSecurityDescriptor: *mut PSECURITY_DESCRIPTOR,
        AutoInheritFlags: c_ulong,
        GenericMapping: *mut GENERIC_MAPPING,
        Token: HANDLE,
    ) -> NTSTATUS;
    fn RtlConvertToAutoInheritSecurityObject(
        ParentDescriptor: PSECURITY_DESCRIPTOR,
        CurrentSecurityDescriptor: PSECURITY_DESCRIPTOR,
        NewSecurityDescriptor: *mut PSECURITY_DESCRIPTOR,
        ObjectType: *mut GUID,
        IsDirectoryObject: c_uchar,
        GenericMapping: *mut GENERIC_MAPPING,
    ) -> NTSTATUS;
    fn RtlNewInstanceSecurityObject(
        ParentDescriptorChanged: c_uchar,
        CreatorDescriptorChanged: c_uchar,
        OldClientTokenModifiedId: *mut LUID,
        NewClientTokenModifiedId: *mut LUID,
        ParentDescriptor: PSECURITY_DESCRIPTOR,
        CreatorDescriptor: PSECURITY_DESCRIPTOR,
        NewDescriptor: *mut PSECURITY_DESCRIPTOR,
        IsDirectoryObject: c_uchar,
        Token: HANDLE,
        GenericMapping: *mut GENERIC_MAPPING,
    ) -> NTSTATUS;
    fn RtlCopySecurityDescriptor(
        InputSecurityDescriptor: PSECURITY_DESCRIPTOR,
        OutputSecurityDescriptor: *mut PSECURITY_DESCRIPTOR,
    ) -> NTSTATUS;
    fn RtlRunEncodeUnicodeString(
        Seed: *mut c_uchar,
        String: *mut UNICODE_STRING,
    );
    fn RtlRunDecodeUnicodeString(
        Seed: c_uchar,
        String: *mut UNICODE_STRING,
    );
    fn RtlImpersonateSelf(
        ImpersonationLevel: SECURITY_IMPERSONATION_LEVEL,
    ) -> NTSTATUS;
    fn RtlImpersonateSelfEx(
        ImpersonationLevel: SECURITY_IMPERSONATION_LEVEL,
        AdditionalAccess: c_ulong,
        ThreadToken: *mut HANDLE,
    ) -> NTSTATUS;
    fn RtlAdjustPrivilege(
        Privilege: c_ulong,
        Enable: c_uchar,
        Client: c_uchar,
        WasEnabled: *mut c_uchar,
    ) -> NTSTATUS;
}}
pub const RTL_ACQUIRE_PRIVILEGE_REVERT: c_ulong = 0x00000001;
pub const RTL_ACQUIRE_PRIVILEGE_PROCESS: c_ulong = 0x00000002;
EXTERN! {extern "system" {
    fn RtlAcquirePrivilege(
        Privilege: *mut c_ulong,
        NumPriv: c_ulong,
        Flags: c_ulong,
        ReturnedState: *mut *mut c_void,
    ) -> NTSTATUS;
    fn RtlReleasePrivilege(
        StatePointer: *mut c_void,
    );
    fn RtlRemovePrivileges(
        TokenHandle: HANDLE,
        PrivilegesToKeep: *mut c_ulong,
        PrivilegeCount: c_ulong,
    ) -> NTSTATUS;
    fn RtlIsUntrustedObject(
        Handle: HANDLE,
        Object: *mut c_void,
        IsUntrustedObject: *mut c_uchar,
    ) -> NTSTATUS;
    fn RtlQueryValidationRunlevel(
        ComponentName: *mut UNICODE_STRING,
    ) -> c_ulong;
    fn RtlCreateBoundaryDescriptor(
        Name: *mut UNICODE_STRING,
        Flags: c_ulong,
    ) -> *mut c_void;
    fn RtlDeleteBoundaryDescriptor(
        BoundaryDescriptor: *mut c_void,
    );
    fn RtlAddSIDToBoundaryDescriptor(
        BoundaryDescriptor: *mut *mut c_void,
        RequiredSid: PSID,
    ) -> NTSTATUS;
    fn RtlAddIntegrityLabelToBoundaryDescriptor(
        BoundaryDescriptor: *mut *mut c_void,
        IntegrityLabel: PSID,
    ) -> NTSTATUS;
    fn RtlGetVersion(
        lpVersionInformation: *mut OSVERSIONINFOW,
    ) -> NTSTATUS;
    fn RtlVerifyVersionInfo(
        VersionInfo: *mut OSVERSIONINFOEXW,
        TypeMask: c_ulong,
        ConditionMask: __uint64,
    ) -> NTSTATUS;
    fn RtlGetNtVersionNumbers(
        NtMajorVersion: *mut c_ulong,
        NtMinorVersion: *mut c_ulong,
        NtBuildNumber: *mut c_ulong,
    );
    fn RtlGetNtGlobalFlags() -> c_ulong;
    fn RtlGetNtProductType(
        NtProductType: *mut NT_PRODUCT_TYPE,
    ) -> c_uchar;
    fn RtlGetSuiteMask() -> c_ulong;
    fn RtlRegisterWait(
        WaitHandle: *mut HANDLE,
        Handle: HANDLE,
        Function: WAITORTIMERCALLBACK,
        Context: *mut c_void,
        Milliseconds: c_ulong,
        Flags: c_ulong,
    ) -> NTSTATUS;
    fn RtlDeregisterWait(
        WaitHandle: HANDLE,
    ) -> NTSTATUS;
    fn RtlDeregisterWaitEx(
        WaitHandle: HANDLE,
        Event: HANDLE,
    ) -> NTSTATUS;
    fn RtlQueueWorkItem(
        Function: WORKERCALLBACKFUNC,
        Context: *mut c_void,
        Flags: c_ulong,
    ) -> NTSTATUS;
    fn RtlSetIoCompletionCallback(
        FileHandle: HANDLE,
        CompletionProc: APC_CALLBACK_FUNCTION,
        Flags: c_ulong,
    ) -> NTSTATUS;
}}
FN! {stdcall PRTL_START_POOL_THREAD(
    Function: LPTHREAD_START_ROUTINE,
    Parameter: *mut c_void,
    ThreadHandle: *mut HANDLE,
) -> NTSTATUS}
FN! {stdcall PRTL_EXIT_POOL_THREAD(
    ExitStatus: NTSTATUS,
) -> NTSTATUS}
EXTERN! {extern "system" {
    fn RtlSetThreadPoolStartFunc(
        StartPoolThread: PRTL_START_POOL_THREAD,
        ExitPoolThread: PRTL_EXIT_POOL_THREAD,
    ) -> NTSTATUS;
    fn RtlUserThreadStart(
        Function: LPTHREAD_START_ROUTINE,
        Parameter: *mut c_void,
    );
    fn LdrInitializeThunk(
        ContextRecord: *mut CONTEXT,
        Parameter: *mut c_void,
    );
    fn RtlCreateTimerQueue(
        TimerQueueHandle: *mut HANDLE,
    ) -> NTSTATUS;
    fn RtlCreateTimer(
        TimerQueueHandle: HANDLE,
        Handle: *mut HANDLE,
        Function: WAITORTIMERCALLBACK,
        Context: *mut c_void,
        DueTime: c_ulong,
        Period: c_ulong,
        Flags: c_ulong,
    ) -> NTSTATUS;
    fn RtlUpdateTimer(
        TimerQueueHandle: HANDLE,
        TimerHandle: HANDLE,
        DueTime: c_ulong,
        Period: c_ulong,
    ) -> NTSTATUS;
    fn RtlDeleteTimer(
        TimerQueueHandle: HANDLE,
        TimerToCancel: HANDLE,
        Event: HANDLE,
    ) -> NTSTATUS;
    fn RtlDeleteTimerQueue(
        TimerQueueHandle: HANDLE,
    ) -> NTSTATUS;
    fn RtlDeleteTimerQueueEx(
        TimerQueueHandle: HANDLE,
        Event: HANDLE,
    ) -> NTSTATUS;
    fn RtlFormatCurrentUserKeyPath(
        CurrentUserKeyPath: *mut UNICODE_STRING,
    ) -> NTSTATUS;
    fn RtlOpenCurrentUser(
        DesiredAccess: c_ulong,
        CurrentUserKey: *mut HANDLE,
    ) -> NTSTATUS;
}}
pub const RTL_REGISTRY_ABSOLUTE: c_ulong = 0;
pub const RTL_REGISTRY_SERVICES: c_ulong = 1;
pub const RTL_REGISTRY_CONTROL: c_ulong = 2;
pub const RTL_REGISTRY_WINDOWS_NT: c_ulong = 3;
pub const RTL_REGISTRY_DEVICEMAP: c_ulong = 4;
pub const RTL_REGISTRY_USER: c_ulong = 5;
pub const RTL_REGISTRY_MAXIMUM: c_ulong = 6;
pub const RTL_REGISTRY_HANDLE: c_ulong = 0x40000000;
pub const RTL_REGISTRY_OPTIONAL: c_ulong = 0x80000000;
EXTERN! {extern "system" {
    fn RtlCreateRegistryKey(
        RelativeTo: c_ulong,
        Path: *mut wchar_t,
    ) -> NTSTATUS;
    fn RtlCheckRegistryKey(
        RelativeTo: c_ulong,
        Path: *mut wchar_t,
    ) -> NTSTATUS;
}}
FN! {stdcall PRTL_QUERY_REGISTRY_ROUTINE(
    ValueName: *mut wchar_t,
    ValueType: c_ulong,
    ValueData: *mut c_void,
    ValueLength: c_ulong,
    Context: *mut c_void,
    EntryContext: *mut c_void,
) -> NTSTATUS}
STRUCT! {struct RTL_QUERY_REGISTRY_TABLE {
    QueryRoutine: PRTL_QUERY_REGISTRY_ROUTINE,
    Flags: c_ulong,
    Name: *mut wchar_t,
    EntryContext: *mut c_void,
    DefaultType: c_ulong,
    DefaultData: *mut c_void,
    DefaultLength: c_ulong,
}}
pub type PRTL_QUERY_REGISTRY_TABLE = *mut RTL_QUERY_REGISTRY_TABLE;
pub const RTL_QUERY_REGISTRY_SUBKEY: c_ulong = 0x00000001;
pub const RTL_QUERY_REGISTRY_TOPKEY: c_ulong = 0x00000002;
pub const RTL_QUERY_REGISTRY_REQUIRED: c_ulong = 0x00000004;
pub const RTL_QUERY_REGISTRY_NOVALUE: c_ulong = 0x00000008;
pub const RTL_QUERY_REGISTRY_NOEXPAND: c_ulong = 0x00000010;
pub const RTL_QUERY_REGISTRY_DIRECT: c_ulong = 0x00000020;
pub const RTL_QUERY_REGISTRY_DELETE: c_ulong = 0x00000040;
EXTERN! {extern "system" {
    fn RtlQueryRegistryValues(
        RelativeTo: c_ulong,
        Path: *const wchar_t,
        QueryTable: PRTL_QUERY_REGISTRY_TABLE,
        Context: *mut c_void,
        Environment: *mut c_void,
    ) -> NTSTATUS;
    fn RtlQueryRegistryValuesEx(
        RelativeTo: c_ulong,
        Path: *mut wchar_t,
        QueryTable: PRTL_QUERY_REGISTRY_TABLE,
        Context: *mut c_void,
        Environment: *mut c_void,
    ) -> NTSTATUS;
    fn RtlWriteRegistryValue(
        RelativeTo: c_ulong,
        Path: *const wchar_t,
        ValueName: *const wchar_t,
        ValueType: c_ulong,
        ValueData: *mut c_void,
        ValueLength: c_ulong,
    ) -> NTSTATUS;
    fn RtlDeleteRegistryValue(
        RelativeTo: c_ulong,
        Path: *const wchar_t,
        ValueName: *const wchar_t,
    ) -> NTSTATUS;
    fn RtlEnableThreadProfiling(
        ThreadHandle: HANDLE,
        Flags: c_ulong,
        HardwareCounters: __uint64,
        PerformanceDataHandle: *mut *mut c_void,
    ) -> NTSTATUS;
    fn RtlDisableThreadProfiling(
        PerformanceDataHandle: *mut c_void,
    ) -> NTSTATUS;
    fn RtlQueryThreadProfiling(
        ThreadHandle: HANDLE,
        Enabled: *mut c_uchar,
    ) -> NTSTATUS;
    fn RtlReadThreadProfilingData(
        PerformanceDataHandle: HANDLE,
        Flags: c_ulong,
        PerformanceData: *mut PERFORMANCE_DATA,
    ) -> NTSTATUS;
    fn RtlGetNativeSystemInformation(
        SystemInformationClass: c_ulong,
        NativeSystemInformation: *mut c_void,
        InformationLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn RtlQueueApcWow64Thread(
        ThreadHandle: HANDLE,
        ApcRoutine: PPS_APC_ROUTINE,
        ApcArgument1: *mut c_void,
        ApcArgument2: *mut c_void,
        ApcArgument3: *mut c_void,
    ) -> NTSTATUS;
    fn RtlWow64EnableFsRedirection(
        Wow64FsEnableRedirection: c_uchar,
    ) -> NTSTATUS;
    fn RtlWow64EnableFsRedirectionEx(
        Wow64FsEnableRedirection: *mut c_void,
        OldFsRedirectionLevel: *mut *mut c_void,
    ) -> NTSTATUS;
    fn RtlComputeCrc32(
        PartialCrc: c_uint,
        Buffer: *mut c_void,
        Length: c_ulong,
    ) -> c_uint;
    fn RtlEncodePointer(
        Ptr: *mut c_void,
    ) -> *mut c_void;
    fn RtlDecodePointer(
        Ptr: *mut c_void,
    ) -> *mut c_void;
    fn RtlEncodeSystemPointer(
        Ptr: *mut c_void,
    ) -> *mut c_void;
    fn RtlDecodeSystemPointer(
        Ptr: *mut c_void,
    ) -> *mut c_void;
    fn RtlEncodeRemotePointer(
        ProcessHandle: HANDLE,
        Pointer: *mut c_void,
        EncodedPointer: *mut *mut c_void,
    ) -> NTSTATUS;
    fn RtlDecodeRemotePointer(
        ProcessHandle: HANDLE,
        Pointer: *mut c_void,
        DecodedPointer: *mut *mut c_void,
    ) -> NTSTATUS;
    fn RtlIsProcessorFeaturePresent(
        ProcessorFeature: c_ulong,
    ) -> c_uchar;
    fn RtlGetCurrentProcessorNumber() -> c_ulong;
    fn RtlGetCurrentProcessorNumberEx(
        ProcessorNumber: *mut PROCESSOR_NUMBER,
    );
    fn RtlPushFrame(
        Frame: PTEB_ACTIVE_FRAME,
    );
    fn RtlPopFrame(
        Frame: PTEB_ACTIVE_FRAME,
    );
    fn RtlGetFrame() -> PTEB_ACTIVE_FRAME;
}}
pub const RTL_WALK_USER_MODE_STACK: c_ulong = 0x00000001;
pub const RTL_WALK_VALID_FLAGS: c_ulong = 0x00000001;
pub const RTL_STACK_WALKING_MODE_FRAMES_TO_SKIP_SHIFT: c_ulong = 0x00000008;
EXTERN! {extern "system" {
    fn RtlWalkFrameChain(
        Callers: *mut *mut c_void,
        Count: c_ulong,
        Flags: c_ulong,
    ) -> c_ulong;
    fn RtlGetCallersAddress(
        CallersAddress: *mut *mut c_void,
        CallersCaller: *mut *mut c_void,
    );
    fn RtlGetEnabledExtendedFeatures(
        FeatureMask: __uint64,
    ) -> __uint64;
    fn RtlGetEnabledExtendedAndSupervisorFeatures(
        FeatureMask: __uint64,
    ) -> __uint64;
    fn RtlLocateSupervisorFeature(
        XStateHeader: *mut XSAVE_AREA_HEADER,
        FeatureId: c_ulong,
        Length: *mut c_ulong,
    ) -> *mut c_void;
}}
STRUCT! {struct RTL_ELEVATION_FLAGS {
    Flags: c_ulong,
}}
BITFIELD! {RTL_ELEVATION_FLAGS Flags: c_ulong [
    ElevationEnabled set_ElevationEnabled[0..1],
    VirtualizationEnabled set_VirtualizationEnabled[1..2],
    InstallerDetectEnabled set_InstallerDetectEnabled[2..3],
    ReservedBits set_ReservedBits[3..32],
]}
pub type PRTL_ELEVATION_FLAGS = *mut RTL_ELEVATION_FLAGS;
EXTERN! {extern "system" {
    fn RtlQueryElevationFlags(
        Flags: PRTL_ELEVATION_FLAGS,
    ) -> NTSTATUS;
    fn RtlRegisterThreadWithCsrss() -> NTSTATUS;
    fn RtlLockCurrentThread() -> NTSTATUS;
    fn RtlUnlockCurrentThread() -> NTSTATUS;
    fn RtlLockModuleSection(
        Address: *mut c_void,
    ) -> NTSTATUS;
    fn RtlUnlockModuleSection(
        Address: *mut c_void,
    ) -> NTSTATUS;
}}
pub const RTL_UNLOAD_EVENT_TRACE_NUMBER: u32 = 64;
STRUCT! {struct RTL_UNLOAD_EVENT_TRACE {
    BaseAddress: *mut c_void,
    SizeOfImage: usize,
    Sequence: c_ulong,
    TimeDateStamp: c_ulong,
    CheckSum: c_ulong,
    ImageName: [wchar_t; 32],
    Version: [c_ulong; 2],
}}
pub type PRTL_UNLOAD_EVENT_TRACE = *mut RTL_UNLOAD_EVENT_TRACE;
STRUCT! {struct RTL_UNLOAD_EVENT_TRACE32 {
    BaseAddress: c_ulong,
    SizeOfImage: c_ulong,
    Sequence: c_ulong,
    TimeDateStamp: c_ulong,
    CheckSum: c_ulong,
    ImageName: [wchar_t; 32],
    Version: [c_ulong; 2],
}}
pub type PRTL_UNLOAD_EVENT_TRACE32 = *mut RTL_UNLOAD_EVENT_TRACE32;
EXTERN! {extern "system" {
    fn RtlGetUnloadEventTrace() -> PRTL_UNLOAD_EVENT_TRACE;
    fn RtlGetUnloadEventTraceEx(
        ElementSize: *mut *mut c_ulong,
        ElementCount: *mut *mut c_ulong,
        EventTrace: *mut *mut c_void,
    );
    fn RtlQueryPerformanceCounter(
        PerformanceCounter: *mut LARGE_INTEGER,
    ) -> c_ulong;
    fn RtlQueryPerformanceFrequency(
        PerformanceFrequency: *mut LARGE_INTEGER,
    ) -> c_ulong;
}}
ENUM! {enum IMAGE_MITIGATION_POLICY {
    ImageDepPolicy = 0,
    ImageAslrPolicy = 1,
    ImageDynamicCodePolicy = 2,
    ImageStrictHandleCheckPolicy = 3,
    ImageSystemCallDisablePolicy = 4,
    ImageMitigationOptionsMask = 5,
    ImageExtensionPointDisablePolicy = 6,
    ImageControlFlowGuardPolicy = 7,
    ImageSignaturePolicy = 8,
    ImageFontDisablePolicy = 9,
    ImageImageLoadPolicy = 10,
    ImagePayloadRestrictionPolicy = 11,
    ImageChildProcessPolicy = 12,
    ImageSehopPolicy = 13,
    ImageHeapPolicy = 14,
    MaxImageMitigationPolicy = 15,
}}
UNION! {union RTL_IMAGE_MITIGATION_POLICY {
    Bitfields1: __uint64,
    Bitfields2: __uint64,
}}
BITFIELD! {unsafe RTL_IMAGE_MITIGATION_POLICY Bitfields1: __uint64 [
    AuditState set_AuditState[0..2],
    AuditFlag set_AuditFlag[2..3],
    EnableAdditionalAuditingOption set_EnableAdditionalAuditingOption[3..4],
    Reserved set_Reserved[4..64],
]}
BITFIELD! {unsafe RTL_IMAGE_MITIGATION_POLICY Bitfields2: __uint64 [
    PolicyState set_PolicyState[0..2],
    AlwaysInherit set_AlwaysInherit[2..3],
    EnableAdditionalPolicyOption set_EnableAdditionalPolicyOption[3..4],
    AuditReserved set_AuditReserved[4..64],
]}
pub type PRTL_IMAGE_MITIGATION_POLICY = *mut RTL_IMAGE_MITIGATION_POLICY;
STRUCT! {struct RTL_IMAGE_MITIGATION_DEP_POLICY {
    Dep: RTL_IMAGE_MITIGATION_POLICY,
}}
pub type PRTL_IMAGE_MITIGATION_DEP_POLICY =
    *mut RTL_IMAGE_MITIGATION_DEP_POLICY;
STRUCT! {struct RTL_IMAGE_MITIGATION_ASLR_POLICY {
    ForceRelocateImages: RTL_IMAGE_MITIGATION_POLICY,
    BottomUpRandomization: RTL_IMAGE_MITIGATION_POLICY,
    HighEntropyRandomization: RTL_IMAGE_MITIGATION_POLICY,
}}
pub type PRTL_IMAGE_MITIGATION_ASLR_POLICY =
    *mut RTL_IMAGE_MITIGATION_ASLR_POLICY;
STRUCT! {struct RTL_IMAGE_MITIGATION_DYNAMIC_CODE_POLICY {
    BlockDynamicCode: RTL_IMAGE_MITIGATION_POLICY,
}}
pub type PRTL_IMAGE_MITIGATION_DYNAMIC_CODE_POLICY =
    *mut RTL_IMAGE_MITIGATION_DYNAMIC_CODE_POLICY;
STRUCT! {struct RTL_IMAGE_MITIGATION_STRICT_HANDLE_CHECK_POLICY {
    StrictHandleChecks: RTL_IMAGE_MITIGATION_POLICY,
}}
pub type PRTL_IMAGE_MITIGATION_STRICT_HANDLE_CHECK_POLICY =
    *mut RTL_IMAGE_MITIGATION_STRICT_HANDLE_CHECK_POLICY;
STRUCT! {struct RTL_IMAGE_MITIGATION_SYSTEM_CALL_DISABLE_POLICY {
    BlockWin32kSystemCalls: RTL_IMAGE_MITIGATION_POLICY,
}}
pub type PRTL_IMAGE_MITIGATION_SYSTEM_CALL_DISABLE_POLICY =
    *mut RTL_IMAGE_MITIGATION_SYSTEM_CALL_DISABLE_POLICY;
STRUCT! {struct RTL_IMAGE_MITIGATION_EXTENSION_POINT_DISABLE_POLICY {
    DisableExtensionPoints: RTL_IMAGE_MITIGATION_POLICY,
}}
pub type PRTL_IMAGE_MITIGATION_EXTENSION_POINT_DISABLE_POLICY =
    *mut RTL_IMAGE_MITIGATION_EXTENSION_POINT_DISABLE_POLICY;
STRUCT! {struct RTL_IMAGE_MITIGATION_CONTROL_FLOW_GUARD_POLICY {
    ControlFlowGuard: RTL_IMAGE_MITIGATION_POLICY,
    StrictControlFlowGuard: RTL_IMAGE_MITIGATION_POLICY,
}}
pub type PRTL_IMAGE_MITIGATION_CONTROL_FLOW_GUARD_POLICY =
    *mut RTL_IMAGE_MITIGATION_CONTROL_FLOW_GUARD_POLICY;
STRUCT! {struct RTL_IMAGE_MITIGATION_BINARY_SIGNATURE_POLICY {
    BlockNonMicrosoftSignedBinaries: RTL_IMAGE_MITIGATION_POLICY,
    EnforceSigningOnModuleDependencies: RTL_IMAGE_MITIGATION_POLICY,
}}
pub type PRTL_IMAGE_MITIGATION_BINARY_SIGNATURE_POLICY =
    *mut RTL_IMAGE_MITIGATION_BINARY_SIGNATURE_POLICY;
STRUCT! {struct RTL_IMAGE_MITIGATION_FONT_DISABLE_POLICY {
    DisableNonSystemFonts: RTL_IMAGE_MITIGATION_POLICY,
}}
pub type PRTL_IMAGE_MITIGATION_FONT_DISABLE_POLICY =
    *mut RTL_IMAGE_MITIGATION_FONT_DISABLE_POLICY;
STRUCT! {struct RTL_IMAGE_MITIGATION_IMAGE_LOAD_POLICY {
    BlockRemoteImageLoads: RTL_IMAGE_MITIGATION_POLICY,
    BlockLowLabelImageLoads: RTL_IMAGE_MITIGATION_POLICY,
    PreferSystem32: RTL_IMAGE_MITIGATION_POLICY,
}}
pub type PRTL_IMAGE_MITIGATION_IMAGE_LOAD_POLICY =
    *mut RTL_IMAGE_MITIGATION_IMAGE_LOAD_POLICY;
STRUCT! {struct RTL_IMAGE_MITIGATION_PAYLOAD_RESTRICTION_POLICY {
    EnableExportAddressFilter: RTL_IMAGE_MITIGATION_POLICY,
    EnableExportAddressFilterPlus: RTL_IMAGE_MITIGATION_POLICY,
    EnableImportAddressFilter: RTL_IMAGE_MITIGATION_POLICY,
    EnableRopStackPivot: RTL_IMAGE_MITIGATION_POLICY,
    EnableRopCallerCheck: RTL_IMAGE_MITIGATION_POLICY,
    EnableRopSimExec: RTL_IMAGE_MITIGATION_POLICY,
}}
pub type PRTL_IMAGE_MITIGATION_PAYLOAD_RESTRICTION_POLICY =
    *mut RTL_IMAGE_MITIGATION_PAYLOAD_RESTRICTION_POLICY;
STRUCT! {struct RTL_IMAGE_MITIGATION_CHILD_PROCESS_POLICY {
    DisallowChildProcessCreation: RTL_IMAGE_MITIGATION_POLICY,
}}
pub type PRTL_IMAGE_MITIGATION_CHILD_PROCESS_POLICY =
    *mut RTL_IMAGE_MITIGATION_CHILD_PROCESS_POLICY;
STRUCT! {struct RTL_IMAGE_MITIGATION_SEHOP_POLICY {
    Sehop: RTL_IMAGE_MITIGATION_POLICY,
}}
pub type PRTL_IMAGE_MITIGATION_SEHOP_POLICY =
    *mut RTL_IMAGE_MITIGATION_SEHOP_POLICY;
STRUCT! {struct RTL_IMAGE_MITIGATION_HEAP_POLICY {
    TerminateOnHeapErrors: RTL_IMAGE_MITIGATION_POLICY,
}}
pub type PRTL_IMAGE_MITIGATION_HEAP_POLICY =
    *mut RTL_IMAGE_MITIGATION_HEAP_POLICY;
ENUM! {enum RTL_IMAGE_MITIGATION_OPTION_STATE {
    RtlMitigationOptionStateNotConfigured = 0,
    RtlMitigationOptionStateOn = 1,
    RtlMitigationOptionStateOff = 2,
}}
pub const RTL_IMAGE_MITIGATION_FLAG_RESET: c_ulong = 0x1;
pub const RTL_IMAGE_MITIGATION_FLAG_REMOVE: c_ulong = 0x2;
pub const RTL_IMAGE_MITIGATION_FLAG_OSDEFAULT: c_ulong = 0x4;
pub const RTL_IMAGE_MITIGATION_FLAG_AUDIT: c_ulong = 0x8;
EXTERN! {extern "system" {
    fn RtlQueryImageMitigationPolicy(
        ImagePath: *mut wchar_t,
        Policy: IMAGE_MITIGATION_POLICY,
        Flags: c_ulong,
        Buffer: *mut c_void,
        BufferSize: c_ulong,
    ) -> NTSTATUS;
    fn RtlSetImageMitigationPolicy(
        ImagePath: *mut wchar_t,
        Policy: IMAGE_MITIGATION_POLICY,
        Flags: c_ulong,
        Buffer: *mut c_void,
        BufferSize: c_ulong,
    ) -> NTSTATUS;
    fn RtlGetCurrentServiceSessionId() -> c_ulong;
    fn RtlGetActiveConsoleId() -> c_ulong;
    fn RtlGetConsoleSessionForegroundProcessId() -> __uint64;
    fn RtlGetTokenNamedObjectPath(
        Token: HANDLE,
        Sid: PSID,
        ObjectPath: *mut UNICODE_STRING,
    ) -> NTSTATUS;
    fn RtlGetAppContainerNamedObjectPath(
        Token: HANDLE,
        AppContainerSid: PSID,
        RelativePath: c_uchar,
        ObjectPath: *mut UNICODE_STRING,
    ) -> NTSTATUS;
    fn RtlGetAppContainerParent(
        AppContainerSid: PSID,
        AppContainerSidParent: *mut PSID,
    ) -> NTSTATUS;
    fn RtlCheckSandboxedToken(
        TokenHandle: HANDLE,
        IsSandboxed: *mut c_uchar,
    ) -> NTSTATUS;
    fn RtlCheckTokenCapability(
        TokenHandle: HANDLE,
        CapabilitySidToCheck: PSID,
        HasCapability: *mut c_uchar,
    ) -> NTSTATUS;
    fn RtlCapabilityCheck(
        TokenHandle: HANDLE,
        CapabilityName: *mut UNICODE_STRING,
        HasCapability: *mut c_uchar,
    ) -> NTSTATUS;
    fn RtlCheckTokenMembership(
        TokenHandle: HANDLE,
        SidToCheck: PSID,
        IsMember: *mut c_uchar,
    ) -> NTSTATUS;
    fn RtlCheckTokenMembershipEx(
        TokenHandle: HANDLE,
        SidToCheck: PSID,
        Flags: c_ulong,
        IsMember: *mut c_uchar,
    ) -> NTSTATUS;
    fn RtlIsParentOfChildAppContainer(
        ParentAppContainerSid: PSID,
        ChildAppContainerSid: PSID,
    ) -> NTSTATUS;
    fn RtlIsCapabilitySid(
        Sid: PSID,
    ) -> c_uchar;
    fn RtlIsPackageSid(
        Sid: PSID,
    ) -> c_uchar;
    fn RtlIsValidProcessTrustLabelSid(
        Sid: PSID,
    ) -> c_uchar;
    fn RtlIsStateSeparationEnabled() -> c_uchar;
}}
ENUM! {enum APPCONTAINER_SID_TYPE {
    NotAppContainerSidType = 0,
    ChildAppContainerSidType = 1,
    ParentAppContainerSidType = 2,
    InvalidAppContainerSidType = 3,
    MaxAppContainerSidType = 4,
}}
pub type PAPPCONTAINER_SID_TYPE = *mut APPCONTAINER_SID_TYPE;
EXTERN! {extern "system" {
    fn RtlGetAppContainerSidType(
        AppContainerSid: PSID,
        AppContainerSidType: PAPPCONTAINER_SID_TYPE,
    ) -> NTSTATUS;
    fn RtlFlsAlloc(
        Callback: PFLS_CALLBACK_FUNCTION,
        FlsIndex: *mut c_ulong,
    ) -> NTSTATUS;
    fn RtlFlsFree(
        FlsIndex: c_ulong,
    ) -> NTSTATUS;
}}
ENUM! {enum STATE_LOCATION_TYPE {
    LocationTypeRegistry = 0,
    LocationTypeFileSystem = 1,
    LocationTypeMaximum = 2,
}}
EXTERN! {extern "system" {
    fn RtlGetPersistedStateLocation(
        SourceID: *const wchar_t,
        CustomValue: *const wchar_t,
        DefaultPath: *const wchar_t,
        StateLocationType: STATE_LOCATION_TYPE,
        TargetPath: *mut wchar_t,
        BufferLengthIn: c_ulong,
        BufferLengthOut: *mut c_ulong,
    ) -> NTSTATUS;
    fn RtlIsCloudFilesPlaceholder(
        FileAttributes: c_ulong,
        ReparseTag: c_ulong,
    ) -> c_uchar;
    fn RtlIsPartialPlaceholder(
        FileAttributes: c_ulong,
        ReparseTag: c_ulong,
    ) -> c_uchar;
    fn RtlIsPartialPlaceholderFileHandle(
        FileHandle: HANDLE,
        IsPartialPlaceholder: *mut c_uchar,
    ) -> NTSTATUS;
    fn RtlIsPartialPlaceholderFileInfo(
        InfoBuffer: *const c_void,
        InfoClass: FILE_INFORMATION_CLASS,
        IsPartialPlaceholder: *mut c_uchar,
    ) -> NTSTATUS;
    fn RtlIsNonEmptyDirectoryReparsePointAllowed(
        ReparseTag: c_ulong,
    ) -> c_uchar;
    fn RtlAppxIsFileOwnedByTrustedInstaller(
        FileHandle: HANDLE,
        IsFileOwnedByTrustedInstaller: *mut c_uchar,
    ) -> NTSTATUS;
}}
STRUCT! {struct PS_PKG_CLAIM {
    Flags: __uint64,
    Origin: __uint64,
}}
pub type PPS_PKG_CLAIM = *mut PS_PKG_CLAIM;
EXTERN! {extern "system" {
    fn RtlQueryPackageClaims(
        TokenHandle: HANDLE,
        PackageFullName: *mut wchar_t,
        PackageSize: *mut usize,
        AppId: *mut wchar_t,
        AppIdSize: *mut usize,
        DynamicId: *mut GUID,
        PkgClaim: PPS_PKG_CLAIM,
        AttributesPresent: *mut __uint64,
    ) -> NTSTATUS;
    fn RtlQueryProtectedPolicy(
        PolicyGuid: *mut GUID,
        PolicyValue: *mut usize,
    ) -> NTSTATUS;
    fn RtlSetProtectedPolicy(
        PolicyGuid: *mut GUID,
        PolicyValue: usize,
        OldPolicyValue: *mut usize,
    ) -> NTSTATUS;
    fn RtlIsMultiSessionSku() -> c_uchar;
    fn RtlIsMultiUsersInSessionSku() -> c_uchar;
}}
ENUM! {enum RTL_BSD_ITEM_TYPE {
    RtlBsdItemVersionNumber = 0,
    RtlBsdItemProductType = 1,
    RtlBsdItemAabEnabled = 2,
    RtlBsdItemAabTimeout = 3,
    RtlBsdItemBootGood = 4,
    RtlBsdItemBootShutdown = 5,
    RtlBsdSleepInProgress = 6,
    RtlBsdPowerTransition = 7,
    RtlBsdItemBootAttemptCount = 8,
    RtlBsdItemBootCheckpoint = 9,
    RtlBsdItemBootId = 10,
    RtlBsdItemShutdownBootId = 11,
    RtlBsdItemReportedAbnormalShutdownBootId = 12,
    RtlBsdItemErrorInfo = 13,
    RtlBsdItemPowerButtonPressInfo = 14,
    RtlBsdItemChecksum = 15,
    RtlBsdItemMax = 16,
}}
STRUCT! {struct RTL_BSD_ITEM {
    Type: RTL_BSD_ITEM_TYPE,
    DataBuffer: *mut c_void,
    DataLength: c_ulong,
}}
pub type PRTL_BSD_ITEM = *mut RTL_BSD_ITEM;
EXTERN! {extern "system" {
    fn RtlCreateBootStatusDataFile() -> NTSTATUS;
    fn RtlLockBootStatusData(
        FileHandle: *mut HANDLE,
    ) -> NTSTATUS;
    fn RtlUnlockBootStatusData(
        FileHandle: HANDLE,
    ) -> NTSTATUS;
    fn RtlGetSetBootStatusData(
        FileHandle: HANDLE,
        Read: c_uchar,
        DataClass: RTL_BSD_ITEM_TYPE,
        Buffer: *mut c_void,
        BufferSize: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> NTSTATUS;
    fn RtlCheckBootStatusIntegrity(
        FileHandle: HANDLE,
        Verified: *mut c_uchar,
    ) -> NTSTATUS;
    fn RtlCheckPortableOperatingSystem(
        IsPortable: *mut c_uchar,
    ) -> NTSTATUS;
    fn RtlSetPortableOperatingSystem(
        IsPortable: c_uchar,
    ) -> NTSTATUS;
}}
EXTERN! {extern "system" {
    fn RtlOsDeploymentState(
        Flags: c_ulong,
    ) -> OS_DEPLOYEMENT_STATE_VALUES;
    fn RtlFindClosestEncodableLength(
        SourceLength: __uint64,
        TargetLength: *mut __uint64,
    ) -> NTSTATUS;
}}
FN! {stdcall PRTL_SECURE_MEMORY_CACHE_CALLBACK(
    Address: *mut c_void,
    Length: usize,
) -> NTSTATUS}
EXTERN! {extern "system" {
    fn RtlRegisterSecureMemoryCacheCallback(
        Callback: PRTL_SECURE_MEMORY_CACHE_CALLBACK,
    ) -> NTSTATUS;
    fn RtlDeregisterSecureMemoryCacheCallback(
        Callback: PRTL_SECURE_MEMORY_CACHE_CALLBACK,
    ) -> NTSTATUS;
    fn RtlFlushSecureMemoryCache(
        MemoryCache: *mut c_void,
        MemoryLength: usize,
    ) -> c_uchar;
}}
