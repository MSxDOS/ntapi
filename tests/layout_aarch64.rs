#![cfg(all(windows, target_arch = "aarch64"))]
#[macro_use]
extern crate ntapi;
use std::mem::{align_of, size_of};
#[test]
fn ntapi_base() {
    use ntapi::ntapi_base::*;
    assert_eq!(size_of::<CLIENT_ID>(), 16);
    assert_eq!(align_of::<CLIENT_ID>(), 8);
    assert_eq!(size_of::<CLIENT_ID32>(), 8);
    assert_eq!(align_of::<CLIENT_ID32>(), 4);
    assert_eq!(size_of::<CLIENT_ID64>(), 16);
    assert_eq!(align_of::<CLIENT_ID64>(), 8);
    assert_eq!(size_of::<KSYSTEM_TIME>(), 12);
    assert_eq!(align_of::<KSYSTEM_TIME>(), 4);
}
#[test]
fn ntdbg() {
    use ntapi::ntdbg::*;
    assert_eq!(size_of::<DBGKM_EXCEPTION>(), 160);
    assert_eq!(align_of::<DBGKM_EXCEPTION>(), 8);
    assert_eq!(size_of::<DBGKM_CREATE_THREAD>(), 16);
    assert_eq!(align_of::<DBGKM_CREATE_THREAD>(), 8);
    assert_eq!(size_of::<DBGKM_CREATE_PROCESS>(), 48);
    assert_eq!(align_of::<DBGKM_CREATE_PROCESS>(), 8);
    assert_eq!(size_of::<DBGKM_EXIT_THREAD>(), 4);
    assert_eq!(align_of::<DBGKM_EXIT_THREAD>(), 4);
    assert_eq!(size_of::<DBGKM_EXIT_PROCESS>(), 4);
    assert_eq!(align_of::<DBGKM_EXIT_PROCESS>(), 4);
    assert_eq!(size_of::<DBGKM_LOAD_DLL>(), 32);
    assert_eq!(align_of::<DBGKM_LOAD_DLL>(), 8);
    assert_eq!(size_of::<DBGKM_UNLOAD_DLL>(), 8);
    assert_eq!(align_of::<DBGKM_UNLOAD_DLL>(), 8);
    assert_eq!(size_of::<DBGUI_CREATE_THREAD>(), 24);
    assert_eq!(align_of::<DBGUI_CREATE_THREAD>(), 8);
    assert_eq!(size_of::<DBGUI_CREATE_PROCESS>(), 64);
    assert_eq!(align_of::<DBGUI_CREATE_PROCESS>(), 8);
    assert_eq!(size_of::<DBGUI_WAIT_STATE_CHANGE>(), 184);
    assert_eq!(align_of::<DBGUI_WAIT_STATE_CHANGE>(), 8);
}
#[test]
fn ntexapi() {
    use ntapi::ntexapi::*;
    assert_eq!(size_of::<BOOT_ENTRY>(), 32);
    assert_eq!(align_of::<BOOT_ENTRY>(), 4);
    assert_eq!(size_of::<BOOT_ENTRY_LIST>(), 36);
    assert_eq!(align_of::<BOOT_ENTRY_LIST>(), 4);
    assert_eq!(size_of::<BOOT_OPTIONS>(), 24);
    assert_eq!(align_of::<BOOT_OPTIONS>(), 4);
    assert_eq!(size_of::<FILE_PATH>(), 16);
    assert_eq!(align_of::<FILE_PATH>(), 4);
    assert_eq!(size_of::<EFI_DRIVER_ENTRY>(), 20);
    assert_eq!(align_of::<EFI_DRIVER_ENTRY>(), 4);
    assert_eq!(size_of::<EFI_DRIVER_ENTRY_LIST>(), 24);
    assert_eq!(align_of::<EFI_DRIVER_ENTRY_LIST>(), 4);
    assert_eq!(size_of::<EVENT_BASIC_INFORMATION>(), 8);
    assert_eq!(align_of::<EVENT_BASIC_INFORMATION>(), 4);
    assert_eq!(size_of::<MUTANT_BASIC_INFORMATION>(), 8);
    assert_eq!(align_of::<MUTANT_BASIC_INFORMATION>(), 4);
    assert_eq!(size_of::<MUTANT_OWNER_INFORMATION>(), 16);
    assert_eq!(align_of::<MUTANT_OWNER_INFORMATION>(), 8);
    assert_eq!(size_of::<SEMAPHORE_BASIC_INFORMATION>(), 8);
    assert_eq!(align_of::<SEMAPHORE_BASIC_INFORMATION>(), 4);
    assert_eq!(size_of::<TIMER_BASIC_INFORMATION>(), 16);
    assert_eq!(align_of::<TIMER_BASIC_INFORMATION>(), 8);
    assert_eq!(size_of::<TIMER_SET_COALESCABLE_TIMER_INFO>(), 48);
    assert_eq!(align_of::<TIMER_SET_COALESCABLE_TIMER_INFO>(), 8);
    assert_eq!(size_of::<T2_SET_PARAMETERS>(), 16);
    assert_eq!(align_of::<T2_SET_PARAMETERS>(), 8);
    assert_eq!(size_of::<WNF_TYPE_ID>(), 16);
    assert_eq!(align_of::<WNF_TYPE_ID>(), 4);
    assert_eq!(size_of::<WNF_DELIVERY_DESCRIPTOR>(), 48);
    assert_eq!(align_of::<WNF_DELIVERY_DESCRIPTOR>(), 8);
    assert_eq!(size_of::<WORKER_FACTORY_BASIC_INFORMATION>(), 120);
    assert_eq!(align_of::<WORKER_FACTORY_BASIC_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_BASIC_INFORMATION>(), 64);
    assert_eq!(align_of::<SYSTEM_BASIC_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_PROCESSOR_INFORMATION>(), 12);
    assert_eq!(align_of::<SYSTEM_PROCESSOR_INFORMATION>(), 4);
    assert_eq!(size_of::<SYSTEM_PERFORMANCE_INFORMATION>(), 344);
    assert_eq!(align_of::<SYSTEM_PERFORMANCE_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_TIMEOFDAY_INFORMATION>(), 48);
    assert_eq!(align_of::<SYSTEM_TIMEOFDAY_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_THREAD_INFORMATION>(), 80);
    assert_eq!(align_of::<SYSTEM_THREAD_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_EXTENDED_THREAD_INFORMATION>(), 136);
    assert_eq!(align_of::<SYSTEM_EXTENDED_THREAD_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_PROCESS_INFORMATION>(), 336);
    assert_eq!(align_of::<SYSTEM_PROCESS_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_CALL_COUNT_INFORMATION>(), 8);
    assert_eq!(align_of::<SYSTEM_CALL_COUNT_INFORMATION>(), 4);
    assert_eq!(size_of::<SYSTEM_DEVICE_INFORMATION>(), 24);
    assert_eq!(align_of::<SYSTEM_DEVICE_INFORMATION>(), 4);
    assert_eq!(size_of::<SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION>(), 48);
    assert_eq!(align_of::<SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_FLAGS_INFORMATION>(), 4);
    assert_eq!(align_of::<SYSTEM_FLAGS_INFORMATION>(), 4);
    assert_eq!(size_of::<SYSTEM_CALL_TIME_INFORMATION>(), 16);
    assert_eq!(align_of::<SYSTEM_CALL_TIME_INFORMATION>(), 8);
    assert_eq!(size_of::<RTL_PROCESS_LOCK_INFORMATION>(), 48);
    assert_eq!(align_of::<RTL_PROCESS_LOCK_INFORMATION>(), 8);
    assert_eq!(size_of::<RTL_PROCESS_LOCKS>(), 56);
    assert_eq!(align_of::<RTL_PROCESS_LOCKS>(), 8);
    assert_eq!(size_of::<RTL_PROCESS_BACKTRACE_INFORMATION>(), 272);
    assert_eq!(align_of::<RTL_PROCESS_BACKTRACE_INFORMATION>(), 8);
    assert_eq!(size_of::<RTL_PROCESS_BACKTRACES>(), 288);
    assert_eq!(align_of::<RTL_PROCESS_BACKTRACES>(), 8);
    assert_eq!(size_of::<SYSTEM_HANDLE_TABLE_ENTRY_INFO>(), 24);
    assert_eq!(align_of::<SYSTEM_HANDLE_TABLE_ENTRY_INFO>(), 8);
    assert_eq!(size_of::<SYSTEM_HANDLE_INFORMATION>(), 32);
    assert_eq!(align_of::<SYSTEM_HANDLE_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_OBJECTTYPE_INFORMATION>(), 64);
    assert_eq!(align_of::<SYSTEM_OBJECTTYPE_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_OBJECT_INFORMATION>(), 80);
    assert_eq!(align_of::<SYSTEM_OBJECT_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_PAGEFILE_INFORMATION>(), 32);
    assert_eq!(align_of::<SYSTEM_PAGEFILE_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_FILECACHE_INFORMATION>(), 64);
    assert_eq!(align_of::<SYSTEM_FILECACHE_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_BASIC_WORKING_SET_INFORMATION>(), 24);
    assert_eq!(align_of::<SYSTEM_BASIC_WORKING_SET_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_POOLTAG>(), 40);
    assert_eq!(align_of::<SYSTEM_POOLTAG>(), 8);
    assert_eq!(size_of::<SYSTEM_POOLTAG_INFORMATION>(), 48);
    assert_eq!(align_of::<SYSTEM_POOLTAG_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_INTERRUPT_INFORMATION>(), 24);
    assert_eq!(align_of::<SYSTEM_INTERRUPT_INFORMATION>(), 4);
    assert_eq!(size_of::<SYSTEM_DPC_BEHAVIOR_INFORMATION>(), 20);
    assert_eq!(align_of::<SYSTEM_DPC_BEHAVIOR_INFORMATION>(), 4);
    assert_eq!(size_of::<SYSTEM_QUERY_TIME_ADJUST_INFORMATION>(), 12);
    assert_eq!(align_of::<SYSTEM_QUERY_TIME_ADJUST_INFORMATION>(), 4);
    assert_eq!(
        size_of::<SYSTEM_QUERY_TIME_ADJUST_INFORMATION_PRECISE>(),
        24
    );
    assert_eq!(
        align_of::<SYSTEM_QUERY_TIME_ADJUST_INFORMATION_PRECISE>(),
        8
    );
    assert_eq!(size_of::<SYSTEM_SET_TIME_ADJUST_INFORMATION>(), 8);
    assert_eq!(align_of::<SYSTEM_SET_TIME_ADJUST_INFORMATION>(), 4);
    assert_eq!(size_of::<SYSTEM_SET_TIME_ADJUST_INFORMATION_PRECISE>(), 16);
    assert_eq!(align_of::<SYSTEM_SET_TIME_ADJUST_INFORMATION_PRECISE>(), 8);
    assert_eq!(size_of::<EVENT_TRACE_VERSION_INFORMATION>(), 8);
    assert_eq!(align_of::<EVENT_TRACE_VERSION_INFORMATION>(), 4);
    assert_eq!(size_of::<PERFINFO_GROUPMASK>(), 32);
    assert_eq!(align_of::<PERFINFO_GROUPMASK>(), 4);
    assert_eq!(size_of::<EVENT_TRACE_GROUPMASK_INFORMATION>(), 48);
    assert_eq!(align_of::<EVENT_TRACE_GROUPMASK_INFORMATION>(), 8);
    assert_eq!(size_of::<EVENT_TRACE_PERFORMANCE_INFORMATION>(), 16);
    assert_eq!(align_of::<EVENT_TRACE_PERFORMANCE_INFORMATION>(), 8);
    assert_eq!(size_of::<EVENT_TRACE_TIME_PROFILE_INFORMATION>(), 8);
    assert_eq!(align_of::<EVENT_TRACE_TIME_PROFILE_INFORMATION>(), 4);
    assert_eq!(size_of::<EVENT_TRACE_SESSION_SECURITY_INFORMATION>(), 24);
    assert_eq!(align_of::<EVENT_TRACE_SESSION_SECURITY_INFORMATION>(), 8);
    assert_eq!(size_of::<EVENT_TRACE_SPINLOCK_INFORMATION>(), 20);
    assert_eq!(align_of::<EVENT_TRACE_SPINLOCK_INFORMATION>(), 4);
    assert_eq!(size_of::<EVENT_TRACE_SYSTEM_EVENT_INFORMATION>(), 24);
    assert_eq!(align_of::<EVENT_TRACE_SYSTEM_EVENT_INFORMATION>(), 8);
    assert_eq!(size_of::<EVENT_TRACE_EXECUTIVE_RESOURCE_INFORMATION>(), 16);
    assert_eq!(align_of::<EVENT_TRACE_EXECUTIVE_RESOURCE_INFORMATION>(), 4);
    assert_eq!(size_of::<EVENT_TRACE_HEAP_TRACING_INFORMATION>(), 8);
    assert_eq!(align_of::<EVENT_TRACE_HEAP_TRACING_INFORMATION>(), 4);
    assert_eq!(size_of::<EVENT_TRACE_TAG_FILTER_INFORMATION>(), 24);
    assert_eq!(align_of::<EVENT_TRACE_TAG_FILTER_INFORMATION>(), 8);
    assert_eq!(size_of::<EVENT_TRACE_PROFILE_COUNTER_INFORMATION>(), 24);
    assert_eq!(align_of::<EVENT_TRACE_PROFILE_COUNTER_INFORMATION>(), 8);
    assert_eq!(size_of::<EVENT_TRACE_PROFILE_LIST_INFORMATION>(), 16);
    assert_eq!(align_of::<EVENT_TRACE_PROFILE_LIST_INFORMATION>(), 8);
    assert_eq!(size_of::<EVENT_TRACE_STACK_CACHING_INFORMATION>(), 32);
    assert_eq!(align_of::<EVENT_TRACE_STACK_CACHING_INFORMATION>(), 8);
    assert_eq!(size_of::<EVENT_TRACE_SOFT_RESTART_INFORMATION>(), 24);
    assert_eq!(align_of::<EVENT_TRACE_SOFT_RESTART_INFORMATION>(), 8);
    assert_eq!(size_of::<EVENT_TRACE_PROFILE_ADD_INFORMATION>(), 32);
    assert_eq!(align_of::<EVENT_TRACE_PROFILE_ADD_INFORMATION>(), 4);
    assert_eq!(size_of::<EVENT_TRACE_PROFILE_REMOVE_INFORMATION>(), 20);
    assert_eq!(align_of::<EVENT_TRACE_PROFILE_REMOVE_INFORMATION>(), 4);
    assert_eq!(size_of::<EVENT_TRACE_COVERAGE_SAMPLER_INFORMATION>(), 16);
    assert_eq!(align_of::<EVENT_TRACE_COVERAGE_SAMPLER_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_EXCEPTION_INFORMATION>(), 16);
    assert_eq!(align_of::<SYSTEM_EXCEPTION_INFORMATION>(), 4);
    assert_eq!(size_of::<SYSTEM_KERNEL_DEBUGGER_INFORMATION>(), 2);
    assert_eq!(align_of::<SYSTEM_KERNEL_DEBUGGER_INFORMATION>(), 1);
    assert_eq!(size_of::<SYSTEM_CONTEXT_SWITCH_INFORMATION>(), 48);
    assert_eq!(align_of::<SYSTEM_CONTEXT_SWITCH_INFORMATION>(), 4);
    assert_eq!(size_of::<SYSTEM_REGISTRY_QUOTA_INFORMATION>(), 16);
    assert_eq!(align_of::<SYSTEM_REGISTRY_QUOTA_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_PROCESSOR_IDLE_INFORMATION>(), 48);
    assert_eq!(align_of::<SYSTEM_PROCESSOR_IDLE_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_LEGACY_DRIVER_INFORMATION>(), 24);
    assert_eq!(align_of::<SYSTEM_LEGACY_DRIVER_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_LOOKASIDE_INFORMATION>(), 32);
    assert_eq!(align_of::<SYSTEM_LOOKASIDE_INFORMATION>(), 4);
    assert_eq!(size_of::<SYSTEM_RANGE_START_INFORMATION>(), 8);
    assert_eq!(align_of::<SYSTEM_RANGE_START_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_VERIFIER_INFORMATION>(), 128);
    assert_eq!(align_of::<SYSTEM_VERIFIER_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_SESSION_PROCESS_INFORMATION>(), 16);
    assert_eq!(align_of::<SYSTEM_SESSION_PROCESS_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_PROCESSOR_POWER_INFORMATION>(), 80);
    assert_eq!(align_of::<SYSTEM_PROCESSOR_POWER_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_HANDLE_TABLE_ENTRY_INFO_EX>(), 40);
    assert_eq!(align_of::<SYSTEM_HANDLE_TABLE_ENTRY_INFO_EX>(), 8);
    assert_eq!(size_of::<SYSTEM_HANDLE_INFORMATION_EX>(), 56);
    assert_eq!(align_of::<SYSTEM_HANDLE_INFORMATION_EX>(), 8);
    assert_eq!(size_of::<SYSTEM_BIGPOOL_ENTRY>(), 24);
    assert_eq!(align_of::<SYSTEM_BIGPOOL_ENTRY>(), 8);
    assert_eq!(size_of::<SYSTEM_BIGPOOL_INFORMATION>(), 32);
    assert_eq!(align_of::<SYSTEM_BIGPOOL_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_POOL_ENTRY>(), 16);
    assert_eq!(align_of::<SYSTEM_POOL_ENTRY>(), 8);
    assert_eq!(size_of::<SYSTEM_POOL_INFORMATION>(), 40);
    assert_eq!(align_of::<SYSTEM_POOL_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_SESSION_POOLTAG_INFORMATION>(), 56);
    assert_eq!(align_of::<SYSTEM_SESSION_POOLTAG_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_SESSION_MAPPED_VIEW_INFORMATION>(), 32);
    assert_eq!(align_of::<SYSTEM_SESSION_MAPPED_VIEW_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_FIRMWARE_TABLE_INFORMATION>(), 20);
    assert_eq!(align_of::<SYSTEM_FIRMWARE_TABLE_INFORMATION>(), 4);
    assert_eq!(size_of::<SYSTEM_MEMORY_LIST_INFORMATION>(), 176);
    assert_eq!(align_of::<SYSTEM_MEMORY_LIST_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_THREAD_CID_PRIORITY_INFORMATION>(), 24);
    assert_eq!(align_of::<SYSTEM_THREAD_CID_PRIORITY_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_PROCESSOR_IDLE_CYCLE_TIME_INFORMATION>(), 8);
    assert_eq!(
        align_of::<SYSTEM_PROCESSOR_IDLE_CYCLE_TIME_INFORMATION>(),
        8
    );
    assert_eq!(size_of::<SYSTEM_REF_TRACE_INFORMATION>(), 40);
    assert_eq!(align_of::<SYSTEM_REF_TRACE_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_PROCESS_ID_INFORMATION>(), 24);
    assert_eq!(align_of::<SYSTEM_PROCESS_ID_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_BOOT_ENVIRONMENT_INFORMATION>(), 32);
    assert_eq!(align_of::<SYSTEM_BOOT_ENVIRONMENT_INFORMATION>(), 8);
    assert_eq!(
        size_of::<SYSTEM_IMAGE_FILE_EXECUTION_OPTIONS_INFORMATION>(),
        8
    );
    assert_eq!(
        align_of::<SYSTEM_IMAGE_FILE_EXECUTION_OPTIONS_INFORMATION>(),
        4
    );
    assert_eq!(size_of::<SYSTEM_VERIFIER_INFORMATION_EX>(), 40);
    assert_eq!(align_of::<SYSTEM_VERIFIER_INFORMATION_EX>(), 8);
    assert_eq!(size_of::<SYSTEM_SYSTEM_PARTITION_INFORMATION>(), 16);
    assert_eq!(align_of::<SYSTEM_SYSTEM_PARTITION_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_SYSTEM_DISK_INFORMATION>(), 16);
    assert_eq!(align_of::<SYSTEM_SYSTEM_DISK_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_PROCESSOR_PERFORMANCE_HITCOUNT>(), 16);
    assert_eq!(align_of::<SYSTEM_PROCESSOR_PERFORMANCE_HITCOUNT>(), 8);
    assert_eq!(size_of::<SYSTEM_PROCESSOR_PERFORMANCE_HITCOUNT_WIN8>(), 8);
    assert_eq!(align_of::<SYSTEM_PROCESSOR_PERFORMANCE_HITCOUNT_WIN8>(), 4);
    assert_eq!(
        size_of::<SYSTEM_PROCESSOR_PERFORMANCE_STATE_DISTRIBUTION>(),
        24
    );
    assert_eq!(
        align_of::<SYSTEM_PROCESSOR_PERFORMANCE_STATE_DISTRIBUTION>(),
        8
    );
    assert_eq!(size_of::<SYSTEM_PROCESSOR_PERFORMANCE_DISTRIBUTION>(), 8);
    assert_eq!(align_of::<SYSTEM_PROCESSOR_PERFORMANCE_DISTRIBUTION>(), 4);
    assert_eq!(size_of::<SYSTEM_CODEINTEGRITY_INFORMATION>(), 8);
    assert_eq!(align_of::<SYSTEM_CODEINTEGRITY_INFORMATION>(), 4);
    assert_eq!(size_of::<SYSTEM_VA_LIST_INFORMATION>(), 32);
    assert_eq!(align_of::<SYSTEM_VA_LIST_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_REGISTRY_APPEND_STRING_PARAMETERS>(), 56);
    assert_eq!(align_of::<SYSTEM_REGISTRY_APPEND_STRING_PARAMETERS>(), 8);
    assert_eq!(size_of::<SYSTEM_VHD_BOOT_INFORMATION>(), 12);
    assert_eq!(align_of::<SYSTEM_VHD_BOOT_INFORMATION>(), 4);
    assert_eq!(size_of::<SYSTEM_LOW_PRIORITY_IO_INFORMATION>(), 40);
    assert_eq!(align_of::<SYSTEM_LOW_PRIORITY_IO_INFORMATION>(), 4);
    assert_eq!(size_of::<TPM_BOOT_ENTROPY_NT_RESULT>(), 72);
    assert_eq!(align_of::<TPM_BOOT_ENTROPY_NT_RESULT>(), 8);
    assert_eq!(size_of::<SYSTEM_VERIFIER_COUNTERS_INFORMATION>(), 264);
    assert_eq!(align_of::<SYSTEM_VERIFIER_COUNTERS_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_ACPI_AUDIT_INFORMATION>(), 8);
    assert_eq!(align_of::<SYSTEM_ACPI_AUDIT_INFORMATION>(), 4);
    assert_eq!(size_of::<SYSTEM_BASIC_PERFORMANCE_INFORMATION>(), 32);
    assert_eq!(align_of::<SYSTEM_BASIC_PERFORMANCE_INFORMATION>(), 8);
    assert_eq!(size_of::<QUERY_PERFORMANCE_COUNTER_FLAGS>(), 4);
    assert_eq!(align_of::<QUERY_PERFORMANCE_COUNTER_FLAGS>(), 4);
    assert_eq!(
        size_of::<SYSTEM_QUERY_PERFORMANCE_COUNTER_INFORMATION>(),
        12
    );
    assert_eq!(
        align_of::<SYSTEM_QUERY_PERFORMANCE_COUNTER_INFORMATION>(),
        4
    );
    assert_eq!(size_of::<SYSTEM_BOOT_GRAPHICS_INFORMATION>(), 32);
    assert_eq!(align_of::<SYSTEM_BOOT_GRAPHICS_INFORMATION>(), 8);
    assert_eq!(size_of::<MEMORY_SCRUB_INFORMATION>(), 16);
    assert_eq!(align_of::<MEMORY_SCRUB_INFORMATION>(), 8);
    assert_eq!(size_of::<PEBS_DS_SAVE_AREA>(), 96);
    assert_eq!(align_of::<PEBS_DS_SAVE_AREA>(), 8);
    assert_eq!(size_of::<PROCESSOR_PROFILE_CONTROL_AREA>(), 96);
    assert_eq!(align_of::<PROCESSOR_PROFILE_CONTROL_AREA>(), 8);
    assert_eq!(size_of::<SYSTEM_PROCESSOR_PROFILE_CONTROL_AREA>(), 104);
    assert_eq!(align_of::<SYSTEM_PROCESSOR_PROFILE_CONTROL_AREA>(), 8);
    assert_eq!(size_of::<MEMORY_COMBINE_INFORMATION>(), 16);
    assert_eq!(align_of::<MEMORY_COMBINE_INFORMATION>(), 8);
    assert_eq!(size_of::<MEMORY_COMBINE_INFORMATION_EX>(), 24);
    assert_eq!(align_of::<MEMORY_COMBINE_INFORMATION_EX>(), 8);
    assert_eq!(size_of::<MEMORY_COMBINE_INFORMATION_EX2>(), 32);
    assert_eq!(align_of::<MEMORY_COMBINE_INFORMATION_EX2>(), 8);
    assert_eq!(size_of::<SYSTEM_CONSOLE_INFORMATION>(), 4);
    assert_eq!(align_of::<SYSTEM_CONSOLE_INFORMATION>(), 4);
    assert_eq!(size_of::<SYSTEM_PLATFORM_BINARY_INFORMATION>(), 32);
    assert_eq!(align_of::<SYSTEM_PLATFORM_BINARY_INFORMATION>(), 8);
    assert_eq!(
        size_of::<SYSTEM_HYPERVISOR_PROCESSOR_COUNT_INFORMATION>(),
        8
    );
    assert_eq!(
        align_of::<SYSTEM_HYPERVISOR_PROCESSOR_COUNT_INFORMATION>(),
        4
    );
    assert_eq!(size_of::<SYSTEM_DEVICE_DATA_INFORMATION>(), 48);
    assert_eq!(align_of::<SYSTEM_DEVICE_DATA_INFORMATION>(), 8);
    assert_eq!(size_of::<PHYSICAL_CHANNEL_RUN>(), 32);
    assert_eq!(align_of::<PHYSICAL_CHANNEL_RUN>(), 8);
    assert_eq!(size_of::<SYSTEM_MEMORY_TOPOLOGY_INFORMATION>(), 48);
    assert_eq!(align_of::<SYSTEM_MEMORY_TOPOLOGY_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_MEMORY_CHANNEL_INFORMATION>(), 40);
    assert_eq!(align_of::<SYSTEM_MEMORY_CHANNEL_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_BOOT_LOGO_INFORMATION>(), 8);
    assert_eq!(align_of::<SYSTEM_BOOT_LOGO_INFORMATION>(), 4);
    assert_eq!(size_of::<SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION_EX>(), 72);
    assert_eq!(align_of::<SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION_EX>(), 8);
    assert_eq!(size_of::<SYSTEM_SECUREBOOT_POLICY_INFORMATION>(), 24);
    assert_eq!(align_of::<SYSTEM_SECUREBOOT_POLICY_INFORMATION>(), 4);
    assert_eq!(size_of::<SYSTEM_PAGEFILE_INFORMATION_EX>(), 40);
    assert_eq!(align_of::<SYSTEM_PAGEFILE_INFORMATION_EX>(), 8);
    assert_eq!(size_of::<SYSTEM_SECUREBOOT_INFORMATION>(), 2);
    assert_eq!(align_of::<SYSTEM_SECUREBOOT_INFORMATION>(), 1);
    assert_eq!(size_of::<PROCESS_DISK_COUNTERS>(), 40);
    assert_eq!(align_of::<PROCESS_DISK_COUNTERS>(), 8);
    assert_eq!(size_of::<ENERGY_STATE_DURATION>(), 8);
    assert_eq!(align_of::<ENERGY_STATE_DURATION>(), 8);
    assert_eq!(size_of::<PROCESS_ENERGY_VALUES>(), 272);
    assert_eq!(align_of::<PROCESS_ENERGY_VALUES>(), 8);
    assert_eq!(size_of::<TIMELINE_BITMAP>(), 16);
    assert_eq!(align_of::<TIMELINE_BITMAP>(), 8);
    assert_eq!(size_of::<PROCESS_ENERGY_VALUES_EXTENSION>(), 272);
    assert_eq!(align_of::<PROCESS_ENERGY_VALUES_EXTENSION>(), 8);
    assert_eq!(size_of::<PROCESS_EXTENDED_ENERGY_VALUES>(), 544);
    assert_eq!(align_of::<PROCESS_EXTENDED_ENERGY_VALUES>(), 8);
    assert_eq!(size_of::<SYSTEM_PROCESS_INFORMATION_EXTENSION>(), 368);
    assert_eq!(align_of::<SYSTEM_PROCESS_INFORMATION_EXTENSION>(), 8);
    assert_eq!(
        size_of::<SYSTEM_PORTABLE_WORKSPACE_EFI_LAUNCHER_INFORMATION>(),
        1
    );
    assert_eq!(
        align_of::<SYSTEM_PORTABLE_WORKSPACE_EFI_LAUNCHER_INFORMATION>(),
        1
    );
    assert_eq!(size_of::<SYSTEM_KERNEL_DEBUGGER_INFORMATION_EX>(), 3);
    assert_eq!(align_of::<SYSTEM_KERNEL_DEBUGGER_INFORMATION_EX>(), 1);
    assert_eq!(size_of::<SYSTEM_ELAM_CERTIFICATE_INFORMATION>(), 8);
    assert_eq!(align_of::<SYSTEM_ELAM_CERTIFICATE_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_PROCESSOR_FEATURES_INFORMATION>(), 32);
    assert_eq!(align_of::<SYSTEM_PROCESSOR_FEATURES_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_MANUFACTURING_INFORMATION>(), 24);
    assert_eq!(align_of::<SYSTEM_MANUFACTURING_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_ENERGY_ESTIMATION_CONFIG_INFORMATION>(), 1);
    assert_eq!(align_of::<SYSTEM_ENERGY_ESTIMATION_CONFIG_INFORMATION>(), 1);
    assert_eq!(size_of::<HV_DETAILS>(), 16);
    assert_eq!(align_of::<HV_DETAILS>(), 4);
    assert_eq!(size_of::<SYSTEM_HYPERVISOR_DETAIL_INFORMATION>(), 112);
    assert_eq!(align_of::<SYSTEM_HYPERVISOR_DETAIL_INFORMATION>(), 4);
    assert_eq!(size_of::<SYSTEM_PROCESSOR_CYCLE_STATS_INFORMATION>(), 64);
    assert_eq!(align_of::<SYSTEM_PROCESSOR_CYCLE_STATS_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_TPM_INFORMATION>(), 4);
    assert_eq!(align_of::<SYSTEM_TPM_INFORMATION>(), 4);
    assert_eq!(size_of::<SYSTEM_VSM_PROTECTION_INFORMATION>(), 3);
    assert_eq!(align_of::<SYSTEM_VSM_PROTECTION_INFORMATION>(), 1);
    assert_eq!(size_of::<SYSTEM_CODEINTEGRITYPOLICY_INFORMATION>(), 32);
    assert_eq!(align_of::<SYSTEM_CODEINTEGRITYPOLICY_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_ISOLATED_USER_MODE_INFORMATION>(), 16);
    assert_eq!(align_of::<SYSTEM_ISOLATED_USER_MODE_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_SINGLE_MODULE_INFORMATION>(), 328);
    assert_eq!(align_of::<SYSTEM_SINGLE_MODULE_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_INTERRUPT_CPU_SET_INFORMATION>(), 16);
    assert_eq!(align_of::<SYSTEM_INTERRUPT_CPU_SET_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_SECUREBOOT_POLICY_FULL_INFORMATION>(), 32);
    assert_eq!(align_of::<SYSTEM_SECUREBOOT_POLICY_FULL_INFORMATION>(), 4);
    assert_eq!(size_of::<SYSTEM_ROOT_SILO_INFORMATION>(), 8);
    assert_eq!(align_of::<SYSTEM_ROOT_SILO_INFORMATION>(), 4);
    assert_eq!(size_of::<SYSTEM_CPU_SET_TAG_INFORMATION>(), 16);
    assert_eq!(align_of::<SYSTEM_CPU_SET_TAG_INFORMATION>(), 8);
    assert_eq!(
        size_of::<SYSTEM_SECURE_KERNEL_HYPERGUARD_PROFILE_INFORMATION>(),
        144
    );
    assert_eq!(
        align_of::<SYSTEM_SECURE_KERNEL_HYPERGUARD_PROFILE_INFORMATION>(),
        8
    );
    assert_eq!(
        size_of::<SYSTEM_SECUREBOOT_PLATFORM_MANIFEST_INFORMATION>(),
        8
    );
    assert_eq!(
        align_of::<SYSTEM_SECUREBOOT_PLATFORM_MANIFEST_INFORMATION>(),
        4
    );
    assert_eq!(size_of::<SYSTEM_MEMORY_USAGE_INFORMATION>(), 56);
    assert_eq!(align_of::<SYSTEM_MEMORY_USAGE_INFORMATION>(), 8);
    assert_eq!(
        size_of::<SYSTEM_CODEINTEGRITY_CERTIFICATE_INFORMATION>(),
        16
    );
    assert_eq!(
        align_of::<SYSTEM_CODEINTEGRITY_CERTIFICATE_INFORMATION>(),
        8
    );
    assert_eq!(size_of::<SYSTEM_PHYSICAL_MEMORY_INFORMATION>(), 24);
    assert_eq!(align_of::<SYSTEM_PHYSICAL_MEMORY_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_ACTIVITY_MODERATION_INFO>(), 24);
    assert_eq!(align_of::<SYSTEM_ACTIVITY_MODERATION_INFO>(), 8);
    assert_eq!(size_of::<SYSTEM_ACTIVITY_MODERATION_USER_SETTINGS>(), 8);
    assert_eq!(align_of::<SYSTEM_ACTIVITY_MODERATION_USER_SETTINGS>(), 8);
    assert_eq!(size_of::<SYSTEM_CODEINTEGRITY_UNLOCK_INFORMATION>(), 36);
    assert_eq!(align_of::<SYSTEM_CODEINTEGRITY_UNLOCK_INFORMATION>(), 4);
    assert_eq!(size_of::<SYSTEM_FLUSH_INFORMATION>(), 32);
    assert_eq!(align_of::<SYSTEM_FLUSH_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_WRITE_CONSTRAINT_INFORMATION>(), 8);
    assert_eq!(align_of::<SYSTEM_WRITE_CONSTRAINT_INFORMATION>(), 4);
    assert_eq!(size_of::<SYSTEM_KERNEL_VA_SHADOW_INFORMATION>(), 4);
    assert_eq!(align_of::<SYSTEM_KERNEL_VA_SHADOW_INFORMATION>(), 4);
    assert_eq!(
        size_of::<SYSTEM_CODEINTEGRITYVERIFICATION_INFORMATION>(),
        24
    );
    assert_eq!(
        align_of::<SYSTEM_CODEINTEGRITYVERIFICATION_INFORMATION>(),
        8
    );
    assert_eq!(size_of::<SYSTEM_HYPERVISOR_SHARED_PAGE_INFORMATION>(), 8);
    assert_eq!(align_of::<SYSTEM_HYPERVISOR_SHARED_PAGE_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_SPECULATION_CONTROL_INFORMATION>(), 4);
    assert_eq!(align_of::<SYSTEM_SPECULATION_CONTROL_INFORMATION>(), 4);
    assert_eq!(size_of::<SYSTEM_DMA_GUARD_POLICY_INFORMATION>(), 1);
    assert_eq!(align_of::<SYSTEM_DMA_GUARD_POLICY_INFORMATION>(), 1);
    assert_eq!(size_of::<SYSTEM_ENCLAVE_LAUNCH_CONTROL_INFORMATION>(), 32);
    assert_eq!(align_of::<SYSTEM_ENCLAVE_LAUNCH_CONTROL_INFORMATION>(), 1);
    assert_eq!(size_of::<SYSTEM_WORKLOAD_ALLOWED_CPU_SET_INFORMATION>(), 16);
    assert_eq!(align_of::<SYSTEM_WORKLOAD_ALLOWED_CPU_SET_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSDBG_VIRTUAL>(), 24);
    assert_eq!(align_of::<SYSDBG_VIRTUAL>(), 8);
    assert_eq!(size_of::<SYSDBG_PHYSICAL>(), 24);
    assert_eq!(align_of::<SYSDBG_PHYSICAL>(), 8);
    assert_eq!(size_of::<SYSDBG_CONTROL_SPACE>(), 24);
    assert_eq!(align_of::<SYSDBG_CONTROL_SPACE>(), 8);
    assert_eq!(size_of::<SYSDBG_IO_SPACE>(), 32);
    assert_eq!(align_of::<SYSDBG_IO_SPACE>(), 8);
    assert_eq!(size_of::<SYSDBG_MSR>(), 16);
    assert_eq!(align_of::<SYSDBG_MSR>(), 8);
    assert_eq!(size_of::<SYSDBG_BUS_DATA>(), 32);
    assert_eq!(align_of::<SYSDBG_BUS_DATA>(), 8);
    assert_eq!(size_of::<SYSDBG_TRIAGE_DUMP>(), 56);
    assert_eq!(align_of::<SYSDBG_TRIAGE_DUMP>(), 8);
    assert_eq!(size_of::<SYSDBG_LIVEDUMP_CONTROL_FLAGS>(), 4);
    assert_eq!(align_of::<SYSDBG_LIVEDUMP_CONTROL_FLAGS>(), 4);
    assert_eq!(size_of::<SYSDBG_LIVEDUMP_CONTROL_ADDPAGES>(), 4);
    assert_eq!(align_of::<SYSDBG_LIVEDUMP_CONTROL_ADDPAGES>(), 4);
    assert_eq!(size_of::<SYSDBG_LIVEDUMP_CONTROL>(), 64);
    assert_eq!(align_of::<SYSDBG_LIVEDUMP_CONTROL>(), 8);
    assert_eq!(size_of::<KUSER_SHARED_DATA>(), 1800);
    assert_eq!(align_of::<KUSER_SHARED_DATA>(), 4);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, TickCountLowDeprecated), 0);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, TickCountMultiplier), 4);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, InterruptTime), 8);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, SystemTime), 20);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, TimeZoneBias), 32);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, ImageNumberLow), 44);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, ImageNumberHigh), 46);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, NtSystemRoot), 48);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, MaxStackTraceDepth), 568);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, CryptoExponent), 572);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, TimeZoneId), 576);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, LargePageMinimum), 580);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, AitSamplingValue), 584);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, AppCompatFlag), 588);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, RNGSeedVersion), 592);
    assert_eq!(
        FIELD_OFFSET!(KUSER_SHARED_DATA, GlobalValidationRunlevel),
        600
    );
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, TimeZoneBiasStamp), 604);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, NtBuildNumber), 608);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, NtProductType), 612);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, ProductTypeIsValid), 616);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, Reserved0), 617);
    assert_eq!(
        FIELD_OFFSET!(KUSER_SHARED_DATA, NativeProcessorArchitecture),
        618
    );
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, NtMajorVersion), 620);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, NtMinorVersion), 624);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, ProcessorFeatures), 628);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, Reserved1), 692);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, Reserved3), 696);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, TimeSlip), 700);
    assert_eq!(
        FIELD_OFFSET!(KUSER_SHARED_DATA, AlternativeArchitecture),
        704
    );
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, BootId), 708);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, SystemExpirationDate), 712);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, SuiteMask), 720);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, KdDebuggerEnabled), 724);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, MitigationPolicies), 725);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, Reserved6), 726);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, ActiveConsoleId), 728);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, DismountCount), 732);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, ComPlusPackage), 736);
    assert_eq!(
        FIELD_OFFSET!(KUSER_SHARED_DATA, LastSystemRITEventTickCount),
        740
    );
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, NumberOfPhysicalPages), 744);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, SafeBootMode), 748);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, VirtualizationFlags), 749);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, Reserved12), 750);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, SharedDataFlags), 752);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, DataFlagsPad), 756);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, TestRetInstruction), 760);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, QpcFrequency), 768);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, SystemCall), 776);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, SystemCallPad0), 780);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, SystemCallPad), 784);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, u), 800);
    // assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, TickCountPad), 812);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, Cookie), 816);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, CookiePad), 820);
    assert_eq!(
        FIELD_OFFSET!(KUSER_SHARED_DATA, ConsoleSessionForegroundProcessId),
        824
    );
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, TimeUpdateLock), 832);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, BaselineSystemTimeQpc), 840);
    assert_eq!(
        FIELD_OFFSET!(KUSER_SHARED_DATA, BaselineInterruptTimeQpc),
        848
    );
    assert_eq!(
        FIELD_OFFSET!(KUSER_SHARED_DATA, QpcSystemTimeIncrement),
        856
    );
    assert_eq!(
        FIELD_OFFSET!(KUSER_SHARED_DATA, QpcInterruptTimeIncrement),
        864
    );
    assert_eq!(
        FIELD_OFFSET!(KUSER_SHARED_DATA, QpcSystemTimeIncrementShift),
        872
    );
    assert_eq!(
        FIELD_OFFSET!(KUSER_SHARED_DATA, QpcInterruptTimeIncrementShift),
        873
    );
    assert_eq!(
        FIELD_OFFSET!(KUSER_SHARED_DATA, UnparkedProcessorCount),
        874
    );
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, EnclaveFeatureMask), 876);
    assert_eq!(
        FIELD_OFFSET!(KUSER_SHARED_DATA, TelemetryCoverageRound),
        892
    );
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, UserModeGlobalLogger), 896);
    assert_eq!(
        FIELD_OFFSET!(KUSER_SHARED_DATA, ImageFileExecutionOptions),
        928
    );
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, LangGenerationCount), 932);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, Reserved4), 936);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, InterruptTimeBias), 944);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, QpcBias), 952);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, ActiveProcessorCount), 960);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, ActiveGroupCount), 964);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, Reserved9), 965);
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, QpcData), 966);
    assert_eq!(
        FIELD_OFFSET!(KUSER_SHARED_DATA, TimeZoneBiasEffectiveStart),
        968
    );
    assert_eq!(
        FIELD_OFFSET!(KUSER_SHARED_DATA, TimeZoneBiasEffectiveEnd),
        976
    );
    assert_eq!(FIELD_OFFSET!(KUSER_SHARED_DATA, XState), 984);
    assert_eq!(size_of::<ATOM_BASIC_INFORMATION>(), 8);
    assert_eq!(align_of::<ATOM_BASIC_INFORMATION>(), 2);
    assert_eq!(size_of::<ATOM_TABLE_INFORMATION>(), 8);
    assert_eq!(align_of::<ATOM_TABLE_INFORMATION>(), 4);
}
#[test]
fn ntgdi() {
    use ntapi::ntgdi::*;
    assert_eq!(size_of::<GDI_HANDLE_ENTRY>(), 24);
    assert_eq!(align_of::<GDI_HANDLE_ENTRY>(), 8);
    assert_eq!(size_of::<GDI_SHARED_MEMORY>(), 393216);
    assert_eq!(align_of::<GDI_SHARED_MEMORY>(), 8);
}
#[test]
fn ntioapi() {
    use ntapi::ntioapi::*;
    assert_eq!(size_of::<IO_STATUS_BLOCK>(), 16);
    assert_eq!(align_of::<IO_STATUS_BLOCK>(), 8);
    assert_eq!(size_of::<FILE_IO_COMPLETION_INFORMATION>(), 32);
    assert_eq!(align_of::<FILE_IO_COMPLETION_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_BASIC_INFORMATION>(), 40);
    assert_eq!(align_of::<FILE_BASIC_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_STANDARD_INFORMATION>(), 24);
    assert_eq!(align_of::<FILE_STANDARD_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_STANDARD_INFORMATION_EX>(), 24);
    assert_eq!(align_of::<FILE_STANDARD_INFORMATION_EX>(), 8);
    assert_eq!(size_of::<FILE_INTERNAL_INFORMATION>(), 8);
    assert_eq!(align_of::<FILE_INTERNAL_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_EA_INFORMATION>(), 4);
    assert_eq!(align_of::<FILE_EA_INFORMATION>(), 4);
    assert_eq!(size_of::<FILE_ACCESS_INFORMATION>(), 4);
    assert_eq!(align_of::<FILE_ACCESS_INFORMATION>(), 4);
    assert_eq!(size_of::<FILE_POSITION_INFORMATION>(), 8);
    assert_eq!(align_of::<FILE_POSITION_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_MODE_INFORMATION>(), 4);
    assert_eq!(align_of::<FILE_MODE_INFORMATION>(), 4);
    assert_eq!(size_of::<FILE_ALIGNMENT_INFORMATION>(), 4);
    assert_eq!(align_of::<FILE_ALIGNMENT_INFORMATION>(), 4);
    assert_eq!(size_of::<FILE_NAME_INFORMATION>(), 8);
    assert_eq!(align_of::<FILE_NAME_INFORMATION>(), 4);
    assert_eq!(size_of::<FILE_ALL_INFORMATION>(), 104);
    assert_eq!(align_of::<FILE_ALL_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_NETWORK_OPEN_INFORMATION>(), 56);
    assert_eq!(align_of::<FILE_NETWORK_OPEN_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_ATTRIBUTE_TAG_INFORMATION>(), 8);
    assert_eq!(align_of::<FILE_ATTRIBUTE_TAG_INFORMATION>(), 4);
    assert_eq!(size_of::<FILE_ALLOCATION_INFORMATION>(), 8);
    assert_eq!(align_of::<FILE_ALLOCATION_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_COMPRESSION_INFORMATION>(), 16);
    assert_eq!(align_of::<FILE_COMPRESSION_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_DISPOSITION_INFORMATION>(), 1);
    assert_eq!(align_of::<FILE_DISPOSITION_INFORMATION>(), 1);
    assert_eq!(size_of::<FILE_END_OF_FILE_INFORMATION>(), 8);
    assert_eq!(align_of::<FILE_END_OF_FILE_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_VALID_DATA_LENGTH_INFORMATION>(), 8);
    assert_eq!(align_of::<FILE_VALID_DATA_LENGTH_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_LINK_INFORMATION>(), 24);
    assert_eq!(align_of::<FILE_LINK_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_MOVE_CLUSTER_INFORMATION>(), 24);
    assert_eq!(align_of::<FILE_MOVE_CLUSTER_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_RENAME_INFORMATION>(), 24);
    assert_eq!(align_of::<FILE_RENAME_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_STREAM_INFORMATION>(), 32);
    assert_eq!(align_of::<FILE_STREAM_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_TRACKING_INFORMATION>(), 16);
    assert_eq!(align_of::<FILE_TRACKING_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_COMPLETION_INFORMATION>(), 16);
    assert_eq!(align_of::<FILE_COMPLETION_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_PIPE_INFORMATION>(), 8);
    assert_eq!(align_of::<FILE_PIPE_INFORMATION>(), 4);
    assert_eq!(size_of::<FILE_PIPE_LOCAL_INFORMATION>(), 40);
    assert_eq!(align_of::<FILE_PIPE_LOCAL_INFORMATION>(), 4);
    assert_eq!(size_of::<FILE_PIPE_REMOTE_INFORMATION>(), 16);
    assert_eq!(align_of::<FILE_PIPE_REMOTE_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_MAILSLOT_QUERY_INFORMATION>(), 24);
    assert_eq!(align_of::<FILE_MAILSLOT_QUERY_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_MAILSLOT_SET_INFORMATION>(), 8);
    assert_eq!(align_of::<FILE_MAILSLOT_SET_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_REPARSE_POINT_INFORMATION>(), 16);
    assert_eq!(align_of::<FILE_REPARSE_POINT_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_LINK_ENTRY_INFORMATION>(), 24);
    assert_eq!(align_of::<FILE_LINK_ENTRY_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_LINKS_INFORMATION>(), 32);
    assert_eq!(align_of::<FILE_LINKS_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_NETWORK_PHYSICAL_NAME_INFORMATION>(), 8);
    assert_eq!(align_of::<FILE_NETWORK_PHYSICAL_NAME_INFORMATION>(), 4);
    assert_eq!(size_of::<FILE_STANDARD_LINK_INFORMATION>(), 12);
    assert_eq!(align_of::<FILE_STANDARD_LINK_INFORMATION>(), 4);
    assert_eq!(size_of::<FILE_SFIO_RESERVE_INFORMATION>(), 20);
    assert_eq!(align_of::<FILE_SFIO_RESERVE_INFORMATION>(), 4);
    assert_eq!(size_of::<FILE_SFIO_VOLUME_INFORMATION>(), 12);
    assert_eq!(align_of::<FILE_SFIO_VOLUME_INFORMATION>(), 4);
    assert_eq!(size_of::<FILE_IO_PRIORITY_HINT_INFORMATION>(), 4);
    assert_eq!(align_of::<FILE_IO_PRIORITY_HINT_INFORMATION>(), 4);
    assert_eq!(size_of::<FILE_IO_PRIORITY_HINT_INFORMATION_EX>(), 8);
    assert_eq!(align_of::<FILE_IO_PRIORITY_HINT_INFORMATION_EX>(), 4);
    assert_eq!(size_of::<FILE_IO_COMPLETION_NOTIFICATION_INFORMATION>(), 4);
    assert_eq!(align_of::<FILE_IO_COMPLETION_NOTIFICATION_INFORMATION>(), 4);
    assert_eq!(size_of::<FILE_PROCESS_IDS_USING_FILE_INFORMATION>(), 16);
    assert_eq!(align_of::<FILE_PROCESS_IDS_USING_FILE_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_IS_REMOTE_DEVICE_INFORMATION>(), 1);
    assert_eq!(align_of::<FILE_IS_REMOTE_DEVICE_INFORMATION>(), 1);
    assert_eq!(size_of::<FILE_NUMA_NODE_INFORMATION>(), 2);
    assert_eq!(align_of::<FILE_NUMA_NODE_INFORMATION>(), 2);
    assert_eq!(size_of::<FILE_IOSTATUSBLOCK_RANGE_INFORMATION>(), 16);
    assert_eq!(align_of::<FILE_IOSTATUSBLOCK_RANGE_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_REMOTE_PROTOCOL_INFORMATION>(), 116);
    assert_eq!(align_of::<FILE_REMOTE_PROTOCOL_INFORMATION>(), 4);
    assert_eq!(size_of::<FILE_INTEGRITY_STREAM_INFORMATION>(), 8);
    assert_eq!(align_of::<FILE_INTEGRITY_STREAM_INFORMATION>(), 4);
    assert_eq!(size_of::<FILE_VOLUME_NAME_INFORMATION>(), 8);
    assert_eq!(align_of::<FILE_VOLUME_NAME_INFORMATION>(), 4);
    assert_eq!(size_of::<FILE_ID_INFORMATION>(), 24);
    assert_eq!(align_of::<FILE_ID_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_ID_EXTD_DIR_INFORMATION>(), 96);
    assert_eq!(align_of::<FILE_ID_EXTD_DIR_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_LINK_ENTRY_FULL_ID_INFORMATION>(), 28);
    assert_eq!(align_of::<FILE_LINK_ENTRY_FULL_ID_INFORMATION>(), 4);
    assert_eq!(size_of::<FILE_ID_EXTD_BOTH_DIR_INFORMATION>(), 120);
    assert_eq!(align_of::<FILE_ID_EXTD_BOTH_DIR_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_STAT_INFORMATION>(), 72);
    assert_eq!(align_of::<FILE_STAT_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_MEMORY_PARTITION_INFORMATION>(), 16);
    assert_eq!(align_of::<FILE_MEMORY_PARTITION_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_STAT_LX_INFORMATION>(), 96);
    assert_eq!(align_of::<FILE_STAT_LX_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_CASE_SENSITIVE_INFORMATION>(), 4);
    assert_eq!(align_of::<FILE_CASE_SENSITIVE_INFORMATION>(), 4);
    assert_eq!(size_of::<FILE_DIRECTORY_INFORMATION>(), 72);
    assert_eq!(align_of::<FILE_DIRECTORY_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_FULL_DIR_INFORMATION>(), 72);
    assert_eq!(align_of::<FILE_FULL_DIR_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_ID_FULL_DIR_INFORMATION>(), 88);
    assert_eq!(align_of::<FILE_ID_FULL_DIR_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_BOTH_DIR_INFORMATION>(), 96);
    assert_eq!(align_of::<FILE_BOTH_DIR_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_ID_BOTH_DIR_INFORMATION>(), 112);
    assert_eq!(align_of::<FILE_ID_BOTH_DIR_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_NAMES_INFORMATION>(), 16);
    assert_eq!(align_of::<FILE_NAMES_INFORMATION>(), 4);
    assert_eq!(size_of::<FILE_ID_GLOBAL_TX_DIR_INFORMATION>(), 96);
    assert_eq!(align_of::<FILE_ID_GLOBAL_TX_DIR_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_OBJECTID_INFORMATION>(), 72);
    assert_eq!(align_of::<FILE_OBJECTID_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_FULL_EA_INFORMATION>(), 12);
    assert_eq!(align_of::<FILE_FULL_EA_INFORMATION>(), 4);
    assert_eq!(size_of::<FILE_GET_EA_INFORMATION>(), 8);
    assert_eq!(align_of::<FILE_GET_EA_INFORMATION>(), 4);
    assert_eq!(size_of::<FILE_GET_QUOTA_INFORMATION>(), 20);
    assert_eq!(align_of::<FILE_GET_QUOTA_INFORMATION>(), 4);
    assert_eq!(size_of::<FILE_QUOTA_INFORMATION>(), 56);
    assert_eq!(align_of::<FILE_QUOTA_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_FS_LABEL_INFORMATION>(), 8);
    assert_eq!(align_of::<FILE_FS_LABEL_INFORMATION>(), 4);
    assert_eq!(size_of::<FILE_FS_VOLUME_INFORMATION>(), 24);
    assert_eq!(align_of::<FILE_FS_VOLUME_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_FS_SIZE_INFORMATION>(), 24);
    assert_eq!(align_of::<FILE_FS_SIZE_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_FS_CONTROL_INFORMATION>(), 48);
    assert_eq!(align_of::<FILE_FS_CONTROL_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_FS_FULL_SIZE_INFORMATION>(), 32);
    assert_eq!(align_of::<FILE_FS_FULL_SIZE_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_FS_OBJECTID_INFORMATION>(), 64);
    assert_eq!(align_of::<FILE_FS_OBJECTID_INFORMATION>(), 1);
    assert_eq!(size_of::<FILE_FS_DEVICE_INFORMATION>(), 8);
    assert_eq!(align_of::<FILE_FS_DEVICE_INFORMATION>(), 4);
    assert_eq!(size_of::<FILE_FS_ATTRIBUTE_INFORMATION>(), 16);
    assert_eq!(align_of::<FILE_FS_ATTRIBUTE_INFORMATION>(), 4);
    assert_eq!(size_of::<FILE_FS_DRIVER_PATH_INFORMATION>(), 12);
    assert_eq!(align_of::<FILE_FS_DRIVER_PATH_INFORMATION>(), 4);
    assert_eq!(size_of::<FILE_FS_VOLUME_FLAGS_INFORMATION>(), 4);
    assert_eq!(align_of::<FILE_FS_VOLUME_FLAGS_INFORMATION>(), 4);
    assert_eq!(size_of::<FILE_FS_SECTOR_SIZE_INFORMATION>(), 28);
    assert_eq!(align_of::<FILE_FS_SECTOR_SIZE_INFORMATION>(), 4);
    assert_eq!(size_of::<FILE_FS_DATA_COPY_INFORMATION>(), 4);
    assert_eq!(align_of::<FILE_FS_DATA_COPY_INFORMATION>(), 4);
    assert_eq!(size_of::<FILE_FS_METADATA_SIZE_INFORMATION>(), 16);
    assert_eq!(align_of::<FILE_FS_METADATA_SIZE_INFORMATION>(), 8);
    assert_eq!(size_of::<FILE_FS_FULL_SIZE_INFORMATION_EX>(), 96);
    assert_eq!(align_of::<FILE_FS_FULL_SIZE_INFORMATION_EX>(), 8);
    assert_eq!(size_of::<IO_COMPLETION_BASIC_INFORMATION>(), 4);
    assert_eq!(align_of::<IO_COMPLETION_BASIC_INFORMATION>(), 4);
    assert_eq!(size_of::<REPARSE_DATA_BUFFER>(), 24);
    assert_eq!(align_of::<REPARSE_DATA_BUFFER>(), 4);
    assert_eq!(size_of::<FILE_PIPE_ASSIGN_EVENT_BUFFER>(), 16);
    assert_eq!(align_of::<FILE_PIPE_ASSIGN_EVENT_BUFFER>(), 8);
    assert_eq!(size_of::<FILE_PIPE_PEEK_BUFFER>(), 20);
    assert_eq!(align_of::<FILE_PIPE_PEEK_BUFFER>(), 4);
    assert_eq!(size_of::<FILE_PIPE_EVENT_BUFFER>(), 20);
    assert_eq!(align_of::<FILE_PIPE_EVENT_BUFFER>(), 4);
    assert_eq!(size_of::<FILE_PIPE_WAIT_FOR_BUFFER>(), 16);
    assert_eq!(align_of::<FILE_PIPE_WAIT_FOR_BUFFER>(), 8);
    assert_eq!(size_of::<FILE_PIPE_CLIENT_PROCESS_BUFFER>(), 16);
    assert_eq!(align_of::<FILE_PIPE_CLIENT_PROCESS_BUFFER>(), 8);
    assert_eq!(size_of::<FILE_PIPE_CLIENT_PROCESS_BUFFER_EX>(), 56);
    assert_eq!(align_of::<FILE_PIPE_CLIENT_PROCESS_BUFFER_EX>(), 8);
    assert_eq!(size_of::<FILE_MAILSLOT_PEEK_BUFFER>(), 12);
    assert_eq!(align_of::<FILE_MAILSLOT_PEEK_BUFFER>(), 4);
}
#[test]
fn ntldr() {
    use ntapi::ntldr::*;
    assert_eq!(size_of::<LDR_SERVICE_TAG_RECORD>(), 16);
    assert_eq!(align_of::<LDR_SERVICE_TAG_RECORD>(), 8);
    assert_eq!(size_of::<LDRP_CSLIST>(), 8);
    assert_eq!(align_of::<LDRP_CSLIST>(), 8);
    assert_eq!(size_of::<LDR_DDAG_NODE>(), 80);
    assert_eq!(align_of::<LDR_DDAG_NODE>(), 8);
    assert_eq!(size_of::<LDR_DEPENDENCY_RECORD>(), 32);
    assert_eq!(align_of::<LDR_DEPENDENCY_RECORD>(), 8);
    assert_eq!(size_of::<LDR_DATA_TABLE_ENTRY>(), 288);
    assert_eq!(align_of::<LDR_DATA_TABLE_ENTRY>(), 8);
    assert_eq!(size_of::<LDR_IMPORT_CALLBACK_INFO>(), 16);
    assert_eq!(align_of::<LDR_IMPORT_CALLBACK_INFO>(), 8);
    assert_eq!(size_of::<LDR_SECTION_INFO>(), 32);
    assert_eq!(align_of::<LDR_SECTION_INFO>(), 8);
    assert_eq!(size_of::<LDR_VERIFY_IMAGE_INFO>(), 64);
    assert_eq!(align_of::<LDR_VERIFY_IMAGE_INFO>(), 8);
    assert_eq!(size_of::<LDR_DLL_LOADED_NOTIFICATION_DATA>(), 40);
    assert_eq!(align_of::<LDR_DLL_LOADED_NOTIFICATION_DATA>(), 8);
    assert_eq!(size_of::<LDR_DLL_UNLOADED_NOTIFICATION_DATA>(), 40);
    assert_eq!(align_of::<LDR_DLL_UNLOADED_NOTIFICATION_DATA>(), 8);
    assert_eq!(size_of::<LDR_DLL_NOTIFICATION_DATA>(), 40);
    assert_eq!(align_of::<LDR_DLL_NOTIFICATION_DATA>(), 8);
    assert_eq!(size_of::<PS_MITIGATION_OPTIONS_MAP>(), 16);
    assert_eq!(align_of::<PS_MITIGATION_OPTIONS_MAP>(), 8);
    assert_eq!(size_of::<PS_MITIGATION_AUDIT_OPTIONS_MAP>(), 16);
    assert_eq!(align_of::<PS_MITIGATION_AUDIT_OPTIONS_MAP>(), 8);
    assert_eq!(size_of::<PS_SYSTEM_DLL_INIT_BLOCK>(), 224);
    assert_eq!(align_of::<PS_SYSTEM_DLL_INIT_BLOCK>(), 8);
    assert_eq!(size_of::<LDR_RESOURCE_INFO>(), 24);
    assert_eq!(align_of::<LDR_RESOURCE_INFO>(), 8);
    assert_eq!(size_of::<LDR_ENUM_RESOURCE_ENTRY>(), 40);
    assert_eq!(align_of::<LDR_ENUM_RESOURCE_ENTRY>(), 8);
    assert_eq!(size_of::<RTL_PROCESS_MODULE_INFORMATION>(), 296);
    assert_eq!(align_of::<RTL_PROCESS_MODULE_INFORMATION>(), 8);
    assert_eq!(size_of::<RTL_PROCESS_MODULES>(), 304);
    assert_eq!(align_of::<RTL_PROCESS_MODULES>(), 8);
    assert_eq!(size_of::<RTL_PROCESS_MODULE_INFORMATION_EX>(), 320);
    assert_eq!(align_of::<RTL_PROCESS_MODULE_INFORMATION_EX>(), 8);
    assert_eq!(size_of::<DELAYLOAD_PROC_DESCRIPTOR>(), 16);
    assert_eq!(align_of::<DELAYLOAD_PROC_DESCRIPTOR>(), 8);
    assert_eq!(size_of::<DELAYLOAD_INFO>(), 72);
    assert_eq!(align_of::<DELAYLOAD_INFO>(), 8);
}
#[test]
fn ntlpcapi() {
    use ntapi::ntlpcapi::*;
    assert_eq!(size_of::<PORT_MESSAGE>(), 40);
    assert_eq!(align_of::<PORT_MESSAGE>(), 8);
    assert_eq!(size_of::<PORT_DATA_ENTRY>(), 16);
    assert_eq!(align_of::<PORT_DATA_ENTRY>(), 8);
    assert_eq!(size_of::<PORT_DATA_INFORMATION>(), 24);
    assert_eq!(align_of::<PORT_DATA_INFORMATION>(), 8);
    assert_eq!(size_of::<LPC_CLIENT_DIED_MSG>(), 48);
    assert_eq!(align_of::<LPC_CLIENT_DIED_MSG>(), 8);
    assert_eq!(size_of::<PORT_VIEW>(), 48);
    assert_eq!(align_of::<PORT_VIEW>(), 8);
    assert_eq!(size_of::<REMOTE_PORT_VIEW>(), 24);
    assert_eq!(align_of::<REMOTE_PORT_VIEW>(), 8);
    assert_eq!(size_of::<PORT_MESSAGE64>(), 40);
    assert_eq!(align_of::<PORT_MESSAGE64>(), 8);
    assert_eq!(size_of::<LPC_CLIENT_DIED_MSG64>(), 48);
    assert_eq!(align_of::<LPC_CLIENT_DIED_MSG64>(), 8);
    assert_eq!(size_of::<PORT_VIEW64>(), 48);
    assert_eq!(align_of::<PORT_VIEW64>(), 8);
    assert_eq!(size_of::<REMOTE_PORT_VIEW64>(), 24);
    assert_eq!(align_of::<REMOTE_PORT_VIEW64>(), 8);
    assert_eq!(size_of::<ALPC_PORT_ATTRIBUTES>(), 72);
    assert_eq!(align_of::<ALPC_PORT_ATTRIBUTES>(), 8);
    assert_eq!(size_of::<ALPC_MESSAGE_ATTRIBUTES>(), 8);
    assert_eq!(align_of::<ALPC_MESSAGE_ATTRIBUTES>(), 4);
    assert_eq!(size_of::<ALPC_COMPLETION_LIST_STATE>(), 8);
    assert_eq!(align_of::<ALPC_COMPLETION_LIST_STATE>(), 8);
    assert_eq!(size_of::<ALPC_COMPLETION_LIST_HEADER>(), 768);
    assert_eq!(align_of::<ALPC_COMPLETION_LIST_HEADER>(), 128);
    assert_eq!(FIELD_OFFSET!(ALPC_COMPLETION_LIST_HEADER, StartMagic), 0);
    assert_eq!(FIELD_OFFSET!(ALPC_COMPLETION_LIST_HEADER, TotalSize), 8);
    assert_eq!(FIELD_OFFSET!(ALPC_COMPLETION_LIST_HEADER, ListOffset), 12);
    assert_eq!(FIELD_OFFSET!(ALPC_COMPLETION_LIST_HEADER, ListSize), 16);
    assert_eq!(FIELD_OFFSET!(ALPC_COMPLETION_LIST_HEADER, BitmapOffset), 20);
    assert_eq!(FIELD_OFFSET!(ALPC_COMPLETION_LIST_HEADER, BitmapSize), 24);
    assert_eq!(FIELD_OFFSET!(ALPC_COMPLETION_LIST_HEADER, DataOffset), 28);
    assert_eq!(FIELD_OFFSET!(ALPC_COMPLETION_LIST_HEADER, DataSize), 32);
    assert_eq!(
        FIELD_OFFSET!(ALPC_COMPLETION_LIST_HEADER, AttributeFlags),
        36
    );
    assert_eq!(
        FIELD_OFFSET!(ALPC_COMPLETION_LIST_HEADER, AttributeSize),
        40
    );
    assert_eq!(FIELD_OFFSET!(ALPC_COMPLETION_LIST_HEADER, State), 128);
    assert_eq!(
        FIELD_OFFSET!(ALPC_COMPLETION_LIST_HEADER, LastMessageId),
        136
    );
    assert_eq!(
        FIELD_OFFSET!(ALPC_COMPLETION_LIST_HEADER, LastCallbackId),
        140
    );
    assert_eq!(FIELD_OFFSET!(ALPC_COMPLETION_LIST_HEADER, PostCount), 256);
    assert_eq!(FIELD_OFFSET!(ALPC_COMPLETION_LIST_HEADER, ReturnCount), 384);
    assert_eq!(
        FIELD_OFFSET!(ALPC_COMPLETION_LIST_HEADER, LogSequenceNumber),
        512
    );
    assert_eq!(FIELD_OFFSET!(ALPC_COMPLETION_LIST_HEADER, UserLock), 640);
    assert_eq!(FIELD_OFFSET!(ALPC_COMPLETION_LIST_HEADER, EndMagic), 648);
    assert_eq!(size_of::<ALPC_CONTEXT_ATTR>(), 32);
    assert_eq!(align_of::<ALPC_CONTEXT_ATTR>(), 8);
    assert_eq!(size_of::<ALPC_HANDLE_ATTR32>(), 44);
    assert_eq!(align_of::<ALPC_HANDLE_ATTR32>(), 4);
    assert_eq!(size_of::<ALPC_HANDLE_ATTR>(), 64);
    assert_eq!(align_of::<ALPC_HANDLE_ATTR>(), 8);
    assert_eq!(size_of::<ALPC_SECURITY_ATTR>(), 24);
    assert_eq!(align_of::<ALPC_SECURITY_ATTR>(), 8);
    assert_eq!(size_of::<ALPC_DATA_VIEW_ATTR>(), 32);
    assert_eq!(align_of::<ALPC_DATA_VIEW_ATTR>(), 8);
    assert_eq!(size_of::<ALPC_BASIC_INFORMATION>(), 16);
    assert_eq!(align_of::<ALPC_BASIC_INFORMATION>(), 8);
    assert_eq!(size_of::<ALPC_PORT_ASSOCIATE_COMPLETION_PORT>(), 16);
    assert_eq!(align_of::<ALPC_PORT_ASSOCIATE_COMPLETION_PORT>(), 8);
    assert_eq!(size_of::<ALPC_SERVER_INFORMATION>(), 32);
    assert_eq!(align_of::<ALPC_SERVER_INFORMATION>(), 8);
    assert_eq!(size_of::<ALPC_PORT_MESSAGE_ZONE_INFORMATION>(), 16);
    assert_eq!(align_of::<ALPC_PORT_MESSAGE_ZONE_INFORMATION>(), 8);
    assert_eq!(size_of::<ALPC_PORT_COMPLETION_LIST_INFORMATION>(), 24);
    assert_eq!(align_of::<ALPC_PORT_COMPLETION_LIST_INFORMATION>(), 8);
    assert_eq!(size_of::<ALPC_MESSAGE_HANDLE_INFORMATION>(), 20);
    assert_eq!(align_of::<ALPC_MESSAGE_HANDLE_INFORMATION>(), 4);
}
#[test]
fn ntmmapi() {
    use ntapi::ntmmapi::*;
    assert_eq!(size_of::<MEMORY_WORKING_SET_BLOCK>(), 8);
    assert_eq!(align_of::<MEMORY_WORKING_SET_BLOCK>(), 8);
    assert_eq!(size_of::<MEMORY_WORKING_SET_INFORMATION>(), 16);
    assert_eq!(align_of::<MEMORY_WORKING_SET_INFORMATION>(), 8);
    assert_eq!(size_of::<MEMORY_REGION_INFORMATION>(), 32);
    assert_eq!(align_of::<MEMORY_REGION_INFORMATION>(), 8);
    assert_eq!(size_of::<MEMORY_WORKING_SET_EX_BLOCK>(), 8);
    assert_eq!(align_of::<MEMORY_WORKING_SET_EX_BLOCK>(), 8);
    assert_eq!(size_of::<MEMORY_WORKING_SET_EX_INFORMATION>(), 16);
    assert_eq!(align_of::<MEMORY_WORKING_SET_EX_INFORMATION>(), 8);
    assert_eq!(size_of::<MEMORY_SHARED_COMMIT_INFORMATION>(), 8);
    assert_eq!(align_of::<MEMORY_SHARED_COMMIT_INFORMATION>(), 8);
    assert_eq!(size_of::<MEMORY_IMAGE_INFORMATION>(), 24);
    assert_eq!(align_of::<MEMORY_IMAGE_INFORMATION>(), 8);
    assert_eq!(size_of::<MEMORY_ENCLAVE_IMAGE_INFORMATION>(), 88);
    assert_eq!(align_of::<MEMORY_ENCLAVE_IMAGE_INFORMATION>(), 8);
    assert_eq!(size_of::<MEMORY_FRAME_INFORMATION>(), 8);
    assert_eq!(align_of::<MEMORY_FRAME_INFORMATION>(), 8);
    assert_eq!(size_of::<FILEOFFSET_INFORMATION>(), 8);
    assert_eq!(align_of::<FILEOFFSET_INFORMATION>(), 8);
    assert_eq!(size_of::<PAGEDIR_INFORMATION>(), 8);
    assert_eq!(align_of::<PAGEDIR_INFORMATION>(), 8);
    assert_eq!(size_of::<UNIQUE_PROCESS_INFORMATION>(), 8);
    assert_eq!(align_of::<UNIQUE_PROCESS_INFORMATION>(), 8);
    assert_eq!(size_of::<MMPFN_IDENTITY>(), 24);
    assert_eq!(align_of::<MMPFN_IDENTITY>(), 8);
    assert_eq!(size_of::<MMPFN_MEMSNAP_INFORMATION>(), 16);
    assert_eq!(align_of::<MMPFN_MEMSNAP_INFORMATION>(), 8);
    assert_eq!(size_of::<SECTION_BASIC_INFORMATION>(), 24);
    assert_eq!(align_of::<SECTION_BASIC_INFORMATION>(), 8);
    assert_eq!(size_of::<SECTION_IMAGE_INFORMATION>(), 64);
    assert_eq!(align_of::<SECTION_IMAGE_INFORMATION>(), 8);
    assert_eq!(size_of::<SECTION_INTERNAL_IMAGE_INFORMATION>(), 72);
    assert_eq!(align_of::<SECTION_INTERNAL_IMAGE_INFORMATION>(), 8);
    assert_eq!(size_of::<MEMORY_RANGE_ENTRY>(), 16);
    assert_eq!(align_of::<MEMORY_RANGE_ENTRY>(), 8);
    assert_eq!(size_of::<CFG_CALL_TARGET_LIST_INFORMATION>(), 40);
    assert_eq!(align_of::<CFG_CALL_TARGET_LIST_INFORMATION>(), 8);
    assert_eq!(size_of::<MEMORY_PARTITION_CONFIGURATION_INFORMATION>(), 240);
    assert_eq!(align_of::<MEMORY_PARTITION_CONFIGURATION_INFORMATION>(), 8);
    assert_eq!(size_of::<MEMORY_PARTITION_TRANSFER_INFORMATION>(), 16);
    assert_eq!(align_of::<MEMORY_PARTITION_TRANSFER_INFORMATION>(), 8);
    assert_eq!(size_of::<MEMORY_PARTITION_PAGEFILE_INFORMATION>(), 40);
    assert_eq!(align_of::<MEMORY_PARTITION_PAGEFILE_INFORMATION>(), 8);
    assert_eq!(size_of::<MEMORY_PARTITION_PAGE_COMBINE_INFORMATION>(), 24);
    assert_eq!(align_of::<MEMORY_PARTITION_PAGE_COMBINE_INFORMATION>(), 8);
    assert_eq!(size_of::<MEMORY_PARTITION_PAGE_RANGE>(), 16);
    assert_eq!(align_of::<MEMORY_PARTITION_PAGE_RANGE>(), 8);
    assert_eq!(size_of::<MEMORY_PARTITION_INITIAL_ADD_INFORMATION>(), 32);
    assert_eq!(align_of::<MEMORY_PARTITION_INITIAL_ADD_INFORMATION>(), 8);
    assert_eq!(size_of::<MEMORY_PARTITION_MEMORY_EVENTS_INFORMATION>(), 40);
    assert_eq!(align_of::<MEMORY_PARTITION_MEMORY_EVENTS_INFORMATION>(), 8);
}
#[test]
fn ntnls() {
    use ntapi::ntnls::*;
    assert_eq!(size_of::<CPTABLEINFO>(), 64);
    assert_eq!(align_of::<CPTABLEINFO>(), 8);
    assert_eq!(size_of::<NLSTABLEINFO>(), 144);
    assert_eq!(align_of::<NLSTABLEINFO>(), 8);
}
#[test]
fn ntobapi() {
    use ntapi::ntobapi::*;
    assert_eq!(size_of::<OBJECT_BASIC_INFORMATION>(), 56);
    assert_eq!(align_of::<OBJECT_BASIC_INFORMATION>(), 8);
    assert_eq!(size_of::<OBJECT_NAME_INFORMATION>(), 16);
    assert_eq!(align_of::<OBJECT_NAME_INFORMATION>(), 8);
    assert_eq!(size_of::<OBJECT_TYPE_INFORMATION>(), 104);
    assert_eq!(align_of::<OBJECT_TYPE_INFORMATION>(), 8);
    assert_eq!(size_of::<OBJECT_TYPES_INFORMATION>(), 4);
    assert_eq!(align_of::<OBJECT_TYPES_INFORMATION>(), 4);
    assert_eq!(size_of::<OBJECT_HANDLE_FLAG_INFORMATION>(), 2);
    assert_eq!(align_of::<OBJECT_HANDLE_FLAG_INFORMATION>(), 1);
    assert_eq!(size_of::<OBJECT_DIRECTORY_INFORMATION>(), 32);
    assert_eq!(align_of::<OBJECT_DIRECTORY_INFORMATION>(), 8);
}
#[test]
fn ntpebteb() {
    use ntapi::ntpebteb::*;
    assert_eq!(size_of::<ACTIVATION_CONTEXT_STACK>(), 40);
    assert_eq!(align_of::<ACTIVATION_CONTEXT_STACK>(), 8);
    assert_eq!(size_of::<API_SET_NAMESPACE>(), 28);
    assert_eq!(align_of::<API_SET_NAMESPACE>(), 4);
    assert_eq!(size_of::<API_SET_HASH_ENTRY>(), 8);
    assert_eq!(align_of::<API_SET_HASH_ENTRY>(), 4);
    assert_eq!(size_of::<API_SET_NAMESPACE_ENTRY>(), 24);
    assert_eq!(align_of::<API_SET_NAMESPACE_ENTRY>(), 4);
    assert_eq!(size_of::<API_SET_VALUE_ENTRY>(), 20);
    assert_eq!(align_of::<API_SET_VALUE_ENTRY>(), 4);
    assert_eq!(size_of::<PEB>(), 1992);
    assert_eq!(align_of::<PEB>(), 8);
    assert_eq!(FIELD_OFFSET!(PEB, InheritedAddressSpace), 0);
    assert_eq!(FIELD_OFFSET!(PEB, ReadImageFileExecOptions), 1);
    assert_eq!(FIELD_OFFSET!(PEB, BeingDebugged), 2);
    assert_eq!(FIELD_OFFSET!(PEB, BitField), 3);
    assert_eq!(FIELD_OFFSET!(PEB, Mutant), 8);
    assert_eq!(FIELD_OFFSET!(PEB, ImageBaseAddress), 16);
    assert_eq!(FIELD_OFFSET!(PEB, Ldr), 24);
    assert_eq!(FIELD_OFFSET!(PEB, ProcessParameters), 32);
    assert_eq!(FIELD_OFFSET!(PEB, SubSystemData), 40);
    assert_eq!(FIELD_OFFSET!(PEB, ProcessHeap), 48);
    assert_eq!(FIELD_OFFSET!(PEB, FastPebLock), 56);
    assert_eq!(FIELD_OFFSET!(PEB, IFEOKey), 64);
    assert_eq!(FIELD_OFFSET!(PEB, AtlThunkSListPtr), 72);
    assert_eq!(FIELD_OFFSET!(PEB, CrossProcessFlags), 80);
    assert_eq!(FIELD_OFFSET!(PEB, u), 88);
    assert_eq!(FIELD_OFFSET!(PEB, SystemReserved), 96);
    assert_eq!(FIELD_OFFSET!(PEB, AtlThunkSListPtr32), 100);
    assert_eq!(FIELD_OFFSET!(PEB, ApiSetMap), 104);
    assert_eq!(FIELD_OFFSET!(PEB, TlsExpansionCounter), 112);
    assert_eq!(FIELD_OFFSET!(PEB, TlsBitmap), 120);
    assert_eq!(FIELD_OFFSET!(PEB, TlsBitmapBits), 128);
    assert_eq!(FIELD_OFFSET!(PEB, ReadOnlySharedMemoryBase), 136);
    assert_eq!(FIELD_OFFSET!(PEB, SharedData), 144);
    assert_eq!(FIELD_OFFSET!(PEB, AnsiCodePageData), 160);
    assert_eq!(FIELD_OFFSET!(PEB, OemCodePageData), 168);
    assert_eq!(FIELD_OFFSET!(PEB, UnicodeCaseTableData), 176);
    assert_eq!(FIELD_OFFSET!(PEB, NumberOfProcessors), 184);
    assert_eq!(FIELD_OFFSET!(PEB, NtGlobalFlag), 188);
    assert_eq!(FIELD_OFFSET!(PEB, CriticalSectionTimeout), 192);
    assert_eq!(FIELD_OFFSET!(PEB, HeapSegmentReserve), 200);
    assert_eq!(FIELD_OFFSET!(PEB, HeapSegmentCommit), 208);
    assert_eq!(FIELD_OFFSET!(PEB, HeapDeCommitTotalFreeThreshold), 216);
    assert_eq!(FIELD_OFFSET!(PEB, HeapDeCommitFreeBlockThreshold), 224);
    assert_eq!(FIELD_OFFSET!(PEB, NumberOfHeaps), 232);
    assert_eq!(FIELD_OFFSET!(PEB, MaximumNumberOfHeaps), 236);
    assert_eq!(FIELD_OFFSET!(PEB, GdiSharedHandleTable), 248);
    assert_eq!(FIELD_OFFSET!(PEB, ProcessStarterHelper), 256);
    assert_eq!(FIELD_OFFSET!(PEB, GdiDCAttributeList), 264);
    assert_eq!(FIELD_OFFSET!(PEB, LoaderLock), 272);
    assert_eq!(FIELD_OFFSET!(PEB, OSMajorVersion), 280);
    assert_eq!(FIELD_OFFSET!(PEB, OSMinorVersion), 284);
    assert_eq!(FIELD_OFFSET!(PEB, OSBuildNumber), 288);
    assert_eq!(FIELD_OFFSET!(PEB, OSCSDVersion), 290);
    assert_eq!(FIELD_OFFSET!(PEB, OSPlatformId), 292);
    assert_eq!(FIELD_OFFSET!(PEB, ImageSubsystem), 296);
    assert_eq!(FIELD_OFFSET!(PEB, ImageSubsystemMajorVersion), 300);
    assert_eq!(FIELD_OFFSET!(PEB, ImageSubsystemMinorVersion), 304);
    assert_eq!(FIELD_OFFSET!(PEB, ActiveProcessAffinityMask), 312);
    assert_eq!(FIELD_OFFSET!(PEB, GdiHandleBuffer), 320);
    assert_eq!(FIELD_OFFSET!(PEB, PostProcessInitRoutine), 560);
    assert_eq!(FIELD_OFFSET!(PEB, TlsExpansionBitmap), 568);
    assert_eq!(FIELD_OFFSET!(PEB, TlsExpansionBitmapBits), 576);
    assert_eq!(FIELD_OFFSET!(PEB, SessionId), 704);
    assert_eq!(FIELD_OFFSET!(PEB, AppCompatFlags), 712);
    assert_eq!(FIELD_OFFSET!(PEB, AppCompatFlagsUser), 720);
    assert_eq!(FIELD_OFFSET!(PEB, pShimData), 728);
    assert_eq!(FIELD_OFFSET!(PEB, AppCompatInfo), 736);
    assert_eq!(FIELD_OFFSET!(PEB, CSDVersion), 744);
    assert_eq!(FIELD_OFFSET!(PEB, ActivationContextData), 760);
    assert_eq!(FIELD_OFFSET!(PEB, ProcessAssemblyStorageMap), 768);
    assert_eq!(FIELD_OFFSET!(PEB, SystemDefaultActivationContextData), 776);
    assert_eq!(FIELD_OFFSET!(PEB, SystemAssemblyStorageMap), 784);
    assert_eq!(FIELD_OFFSET!(PEB, MinimumStackCommit), 792);
    assert_eq!(FIELD_OFFSET!(PEB, FlsListHead), 808);
    assert_eq!(FIELD_OFFSET!(PEB, FlsBitmap), 824);
    assert_eq!(FIELD_OFFSET!(PEB, FlsBitmapBits), 832);
    assert_eq!(FIELD_OFFSET!(PEB, FlsHighIndex), 848);
    assert_eq!(FIELD_OFFSET!(PEB, WerRegistrationData), 856);
    assert_eq!(FIELD_OFFSET!(PEB, WerShipAssertPtr), 864);
    assert_eq!(FIELD_OFFSET!(PEB, pUnused), 872);
    assert_eq!(FIELD_OFFSET!(PEB, pImageHeaderHash), 880);
    assert_eq!(FIELD_OFFSET!(PEB, TracingFlags), 888);
    assert_eq!(FIELD_OFFSET!(PEB, CsrServerReadOnlySharedMemoryBase), 896);
    assert_eq!(FIELD_OFFSET!(PEB, TppWorkerpListLock), 904);
    assert_eq!(FIELD_OFFSET!(PEB, TppWorkerpList), 912);
    assert_eq!(FIELD_OFFSET!(PEB, WaitOnAddressHashTable), 928);
    assert_eq!(FIELD_OFFSET!(PEB, TelemetryCoverageHeader), 1952);
    assert_eq!(FIELD_OFFSET!(PEB, CloudFileFlags), 1960);
    assert_eq!(FIELD_OFFSET!(PEB, CloudFileDiagFlags), 1964);
    assert_eq!(FIELD_OFFSET!(PEB, PlaceholderCompatibilityMode), 1968);
    assert_eq!(
        FIELD_OFFSET!(PEB, PlaceholderCompatibilityModeReserved),
        1969
    );
    assert_eq!(FIELD_OFFSET!(PEB, LeapSecondData), 1976);
    assert_eq!(FIELD_OFFSET!(PEB, LeapSecondFlags), 1984);
    assert_eq!(FIELD_OFFSET!(PEB, NtGlobalFlag2), 1988);
    assert_eq!(size_of::<GDI_TEB_BATCH>(), 1256);
    assert_eq!(align_of::<GDI_TEB_BATCH>(), 8);
    assert_eq!(size_of::<TEB_ACTIVE_FRAME_CONTEXT>(), 16);
    assert_eq!(align_of::<TEB_ACTIVE_FRAME_CONTEXT>(), 8);
    assert_eq!(size_of::<TEB_ACTIVE_FRAME>(), 24);
    assert_eq!(align_of::<TEB_ACTIVE_FRAME>(), 8);
    assert_eq!(size_of::<TEB>(), 6200);
    assert_eq!(align_of::<TEB>(), 8);
    assert_eq!(FIELD_OFFSET!(TEB, NtTib), 0);
    assert_eq!(FIELD_OFFSET!(TEB, EnvironmentPointer), 56);
    assert_eq!(FIELD_OFFSET!(TEB, ClientId), 64);
    assert_eq!(FIELD_OFFSET!(TEB, ActiveRpcHandle), 80);
    assert_eq!(FIELD_OFFSET!(TEB, ThreadLocalStoragePointer), 88);
    assert_eq!(FIELD_OFFSET!(TEB, ProcessEnvironmentBlock), 96);
    assert_eq!(FIELD_OFFSET!(TEB, LastErrorValue), 104);
    assert_eq!(FIELD_OFFSET!(TEB, CountOfOwnedCriticalSections), 108);
    assert_eq!(FIELD_OFFSET!(TEB, CsrClientThread), 112);
    assert_eq!(FIELD_OFFSET!(TEB, Win32ThreadInfo), 120);
    assert_eq!(FIELD_OFFSET!(TEB, User32Reserved), 128);
    assert_eq!(FIELD_OFFSET!(TEB, UserReserved), 232);
    assert_eq!(FIELD_OFFSET!(TEB, WOW32Reserved), 256);
    assert_eq!(FIELD_OFFSET!(TEB, CurrentLocale), 264);
    assert_eq!(FIELD_OFFSET!(TEB, FpSoftwareStatusRegister), 268);
    assert_eq!(FIELD_OFFSET!(TEB, ReservedForDebuggerInstrumentation), 272);
    assert_eq!(FIELD_OFFSET!(TEB, SystemReserved1), 400);
    assert_eq!(FIELD_OFFSET!(TEB, SystemReserved1), 400);
    assert_eq!(FIELD_OFFSET!(TEB, PlaceholderCompatibilityMode), 640);
    assert_eq!(FIELD_OFFSET!(TEB, PlaceholderReserved), 641);
    assert_eq!(FIELD_OFFSET!(TEB, ProxiedProcessId), 652);
    assert_eq!(FIELD_OFFSET!(TEB, ActivationStack), 656);
    assert_eq!(FIELD_OFFSET!(TEB, WorkingOnBehalfTicket), 696);
    assert_eq!(FIELD_OFFSET!(TEB, ExceptionCode), 704);
    assert_eq!(FIELD_OFFSET!(TEB, ActivationContextStackPointer), 712);
    assert_eq!(FIELD_OFFSET!(TEB, InstrumentationCallbackSp), 720);
    assert_eq!(FIELD_OFFSET!(TEB, InstrumentationCallbackPreviousPc), 728);
    assert_eq!(FIELD_OFFSET!(TEB, InstrumentationCallbackPreviousSp), 736);
    assert_eq!(FIELD_OFFSET!(TEB, TxFsContext), 744);
    assert_eq!(FIELD_OFFSET!(TEB, InstrumentationCallbackDisabled), 748);
    assert_eq!(FIELD_OFFSET!(TEB, TxFsContext), 744);
    assert_eq!(FIELD_OFFSET!(TEB, GdiTebBatch), 752);
    assert_eq!(FIELD_OFFSET!(TEB, RealClientId), 2008);
    assert_eq!(FIELD_OFFSET!(TEB, GdiCachedProcessHandle), 2024);
    assert_eq!(FIELD_OFFSET!(TEB, GdiClientPID), 2032);
    assert_eq!(FIELD_OFFSET!(TEB, GdiClientTID), 2036);
    assert_eq!(FIELD_OFFSET!(TEB, GdiThreadLocalInfo), 2040);
    assert_eq!(FIELD_OFFSET!(TEB, Win32ClientInfo), 2048);
    assert_eq!(FIELD_OFFSET!(TEB, glDispatchTable), 2544);
    assert_eq!(FIELD_OFFSET!(TEB, glReserved1), 4408);
    assert_eq!(FIELD_OFFSET!(TEB, glReserved2), 4640);
    assert_eq!(FIELD_OFFSET!(TEB, glSectionInfo), 4648);
    assert_eq!(FIELD_OFFSET!(TEB, glSection), 4656);
    assert_eq!(FIELD_OFFSET!(TEB, glTable), 4664);
    assert_eq!(FIELD_OFFSET!(TEB, glCurrentRC), 4672);
    assert_eq!(FIELD_OFFSET!(TEB, glContext), 4680);
    assert_eq!(FIELD_OFFSET!(TEB, LastStatusValue), 4688);
    assert_eq!(FIELD_OFFSET!(TEB, StaticUnicodeString), 4696);
    assert_eq!(FIELD_OFFSET!(TEB, StaticUnicodeBuffer), 4712);
    assert_eq!(FIELD_OFFSET!(TEB, DeallocationStack), 5240);
    assert_eq!(FIELD_OFFSET!(TEB, TlsSlots), 5248);
    assert_eq!(FIELD_OFFSET!(TEB, TlsLinks), 5760);
    assert_eq!(FIELD_OFFSET!(TEB, Vdm), 5776);
    assert_eq!(FIELD_OFFSET!(TEB, ReservedForNtRpc), 5784);
    assert_eq!(FIELD_OFFSET!(TEB, DbgSsReserved), 5792);
    assert_eq!(FIELD_OFFSET!(TEB, HardErrorMode), 5808);
    assert_eq!(FIELD_OFFSET!(TEB, Instrumentation), 5816);
    assert_eq!(FIELD_OFFSET!(TEB, Instrumentation), 5816);
    assert_eq!(FIELD_OFFSET!(TEB, ActivityId), 5904);
    assert_eq!(FIELD_OFFSET!(TEB, SubProcessTag), 5920);
    assert_eq!(FIELD_OFFSET!(TEB, PerflibData), 5928);
    assert_eq!(FIELD_OFFSET!(TEB, EtwTraceData), 5936);
    assert_eq!(FIELD_OFFSET!(TEB, WinSockData), 5944);
    assert_eq!(FIELD_OFFSET!(TEB, GdiBatchCount), 5952);
    assert_eq!(FIELD_OFFSET!(TEB, u), 5956);
    assert_eq!(FIELD_OFFSET!(TEB, u.s.ReservedPad0), 5956);
    assert_eq!(FIELD_OFFSET!(TEB, u.s.ReservedPad1), 5957);
    assert_eq!(FIELD_OFFSET!(TEB, u.s.ReservedPad2), 5958);
    assert_eq!(FIELD_OFFSET!(TEB, u.s.IdealProcessor), 5959);
    assert_eq!(FIELD_OFFSET!(TEB, GuaranteedStackBytes), 5960);
    assert_eq!(FIELD_OFFSET!(TEB, ReservedForPerf), 5968);
    assert_eq!(FIELD_OFFSET!(TEB, ReservedForOle), 5976);
    assert_eq!(FIELD_OFFSET!(TEB, WaitingOnLoaderLock), 5984);
    assert_eq!(FIELD_OFFSET!(TEB, SavedPriorityState), 5992);
    assert_eq!(FIELD_OFFSET!(TEB, ReservedForCodeCoverage), 6000);
    assert_eq!(FIELD_OFFSET!(TEB, ThreadPoolData), 6008);
    assert_eq!(FIELD_OFFSET!(TEB, DeallocationBStore), 6024);
    assert_eq!(FIELD_OFFSET!(TEB, BStoreLimit), 6032);
    assert_eq!(FIELD_OFFSET!(TEB, MuiGeneration), 6040);
    assert_eq!(FIELD_OFFSET!(TEB, IsImpersonating), 6044);
    assert_eq!(FIELD_OFFSET!(TEB, NlsCache), 6048);
    assert_eq!(FIELD_OFFSET!(TEB, pShimData), 6056);
    assert_eq!(FIELD_OFFSET!(TEB, HeapVirtualAffinity), 6064);
    assert_eq!(FIELD_OFFSET!(TEB, LowFragHeapDataSlot), 6066);
    assert_eq!(FIELD_OFFSET!(TEB, CurrentTransactionHandle), 6072);
    assert_eq!(FIELD_OFFSET!(TEB, ActiveFrame), 6080);
    assert_eq!(FIELD_OFFSET!(TEB, FlsData), 6088);
    assert_eq!(FIELD_OFFSET!(TEB, PreferredLanguages), 6096);
    assert_eq!(FIELD_OFFSET!(TEB, UserPrefLanguages), 6104);
    assert_eq!(FIELD_OFFSET!(TEB, MergedPrefLanguages), 6112);
    assert_eq!(FIELD_OFFSET!(TEB, MuiImpersonation), 6120);
    assert_eq!(FIELD_OFFSET!(TEB, CrossTebFlags), 6124);
    assert_eq!(FIELD_OFFSET!(TEB, SameTebFlags), 6126);
    assert_eq!(FIELD_OFFSET!(TEB, TxnScopeEnterCallback), 6128);
    assert_eq!(FIELD_OFFSET!(TEB, TxnScopeExitCallback), 6136);
    assert_eq!(FIELD_OFFSET!(TEB, TxnScopeContext), 6144);
    assert_eq!(FIELD_OFFSET!(TEB, LockCount), 6152);
    assert_eq!(FIELD_OFFSET!(TEB, WowTebOffset), 6156);
    assert_eq!(FIELD_OFFSET!(TEB, ResourceRetValue), 6160);
    assert_eq!(FIELD_OFFSET!(TEB, ReservedForWdf), 6168);
    assert_eq!(FIELD_OFFSET!(TEB, ReservedForCrt), 6176);
    assert_eq!(FIELD_OFFSET!(TEB, EffectiveContainerId), 6184);
}
#[test]
fn ntpfapi() {
    use ntapi::ntpfapi::*;
    assert_eq!(size_of::<PF_TRACE_LIMITS>(), 16);
    assert_eq!(align_of::<PF_TRACE_LIMITS>(), 8);
    assert_eq!(size_of::<PF_SYSTEM_PREFETCH_PARAMETERS>(), 368);
    assert_eq!(align_of::<PF_SYSTEM_PREFETCH_PARAMETERS>(), 8);
    assert_eq!(size_of::<PF_BOOT_CONTROL>(), 8);
    assert_eq!(align_of::<PF_BOOT_CONTROL>(), 4);
    assert_eq!(size_of::<PREFETCHER_INFORMATION>(), 32);
    assert_eq!(align_of::<PREFETCHER_INFORMATION>(), 8);
    assert_eq!(size_of::<PF_SYSTEM_SUPERFETCH_PARAMETERS>(), 24);
    assert_eq!(align_of::<PF_SYSTEM_SUPERFETCH_PARAMETERS>(), 4);
    assert_eq!(size_of::<PF_PFN_PRIO_REQUEST>(), 6336);
    assert_eq!(align_of::<PF_PFN_PRIO_REQUEST>(), 8);
    assert_eq!(size_of::<PFS_PRIVATE_PAGE_SOURCE>(), 24);
    assert_eq!(align_of::<PFS_PRIVATE_PAGE_SOURCE>(), 8);
    assert_eq!(size_of::<PF_PRIVSOURCE_INFO>(), 96);
    assert_eq!(align_of::<PF_PRIVSOURCE_INFO>(), 8);
    assert_eq!(size_of::<PF_PRIVSOURCE_QUERY_REQUEST>(), 112);
    assert_eq!(align_of::<PF_PRIVSOURCE_QUERY_REQUEST>(), 8);
    assert_eq!(size_of::<PF_SCENARIO_PHASE_INFO>(), 24);
    assert_eq!(align_of::<PF_SCENARIO_PHASE_INFO>(), 4);
    assert_eq!(size_of::<PF_MEMORY_LIST_NODE>(), 48);
    assert_eq!(align_of::<PF_MEMORY_LIST_NODE>(), 8);
    assert_eq!(size_of::<PF_MEMORY_LIST_INFO>(), 64);
    assert_eq!(align_of::<PF_MEMORY_LIST_INFO>(), 8);
    assert_eq!(size_of::<PF_PHYSICAL_MEMORY_RANGE>(), 16);
    assert_eq!(align_of::<PF_PHYSICAL_MEMORY_RANGE>(), 8);
    assert_eq!(size_of::<PF_PHYSICAL_MEMORY_RANGE_INFO>(), 24);
    assert_eq!(align_of::<PF_PHYSICAL_MEMORY_RANGE_INFO>(), 8);
    assert_eq!(size_of::<PF_REPURPOSED_BY_PREFETCH_INFO>(), 8);
    assert_eq!(align_of::<PF_REPURPOSED_BY_PREFETCH_INFO>(), 4);
    assert_eq!(size_of::<SUPERFETCH_INFORMATION>(), 32);
    assert_eq!(align_of::<SUPERFETCH_INFORMATION>(), 8);
}
#[test]
fn ntpnpapi() {
    use ntapi::ntpnpapi::*;
    assert_eq!(size_of::<PLUGPLAY_EVENT_BLOCK>(), 72);
    assert_eq!(align_of::<PLUGPLAY_EVENT_BLOCK>(), 8);
}
#[test]
fn ntpoapi() {
    use ntapi::ntpoapi::*;
    assert_eq!(size_of::<POWER_STATE>(), 4);
    assert_eq!(align_of::<POWER_STATE>(), 4);
    assert_eq!(size_of::<SYSTEM_POWER_STATE_CONTEXT>(), 4);
    assert_eq!(align_of::<SYSTEM_POWER_STATE_CONTEXT>(), 4);
    assert_eq!(size_of::<COUNTED_REASON_CONTEXT>(), 40);
    assert_eq!(align_of::<COUNTED_REASON_CONTEXT>(), 8);
    assert_eq!(size_of::<POWER_STATE_HANDLER>(), 24);
    assert_eq!(align_of::<POWER_STATE_HANDLER>(), 8);
    assert_eq!(size_of::<POWER_STATE_NOTIFY_HANDLER>(), 16);
    assert_eq!(align_of::<POWER_STATE_NOTIFY_HANDLER>(), 8);
    assert_eq!(size_of::<PROCESSOR_POWER_INFORMATION>(), 24);
    assert_eq!(align_of::<PROCESSOR_POWER_INFORMATION>(), 4);
    assert_eq!(size_of::<SYSTEM_POWER_INFORMATION>(), 16);
    assert_eq!(align_of::<SYSTEM_POWER_INFORMATION>(), 4);
}
#[test]
fn ntpsapi() {
    use ntapi::ntpsapi::*;
    assert_eq!(size_of::<PEB_LDR_DATA>(), 88);
    assert_eq!(align_of::<PEB_LDR_DATA>(), 8);
    assert_eq!(size_of::<INITIAL_TEB>(), 40);
    assert_eq!(align_of::<INITIAL_TEB>(), 8);
    assert_eq!(size_of::<WOW64_PROCESS>(), 8);
    assert_eq!(align_of::<WOW64_PROCESS>(), 8);
    assert_eq!(size_of::<PAGE_PRIORITY_INFORMATION>(), 4);
    assert_eq!(align_of::<PAGE_PRIORITY_INFORMATION>(), 4);
    assert_eq!(size_of::<PROCESS_BASIC_INFORMATION>(), 48);
    assert_eq!(align_of::<PROCESS_BASIC_INFORMATION>(), 8);
    assert_eq!(size_of::<PROCESS_EXTENDED_BASIC_INFORMATION>(), 64);
    assert_eq!(align_of::<PROCESS_EXTENDED_BASIC_INFORMATION>(), 8);
    assert_eq!(size_of::<VM_COUNTERS>(), 88);
    assert_eq!(align_of::<VM_COUNTERS>(), 8);
    assert_eq!(size_of::<VM_COUNTERS_EX>(), 96);
    assert_eq!(align_of::<VM_COUNTERS_EX>(), 8);
    assert_eq!(size_of::<VM_COUNTERS_EX2>(), 112);
    assert_eq!(align_of::<VM_COUNTERS_EX2>(), 8);
    assert_eq!(size_of::<KERNEL_USER_TIMES>(), 32);
    assert_eq!(align_of::<KERNEL_USER_TIMES>(), 8);
    assert_eq!(size_of::<POOLED_USAGE_AND_LIMITS>(), 72);
    assert_eq!(align_of::<POOLED_USAGE_AND_LIMITS>(), 8);
    assert_eq!(size_of::<PROCESS_EXCEPTION_PORT>(), 16);
    assert_eq!(align_of::<PROCESS_EXCEPTION_PORT>(), 8);
    assert_eq!(size_of::<PROCESS_ACCESS_TOKEN>(), 16);
    assert_eq!(align_of::<PROCESS_ACCESS_TOKEN>(), 8);
    assert_eq!(size_of::<PROCESS_LDT_INFORMATION>(), 16);
    assert_eq!(align_of::<PROCESS_LDT_INFORMATION>(), 4);
    assert_eq!(size_of::<PROCESS_LDT_SIZE>(), 4);
    assert_eq!(align_of::<PROCESS_LDT_SIZE>(), 4);
    assert_eq!(size_of::<PROCESS_WS_WATCH_INFORMATION>(), 16);
    assert_eq!(align_of::<PROCESS_WS_WATCH_INFORMATION>(), 8);
    assert_eq!(size_of::<PROCESS_WS_WATCH_INFORMATION_EX>(), 32);
    assert_eq!(align_of::<PROCESS_WS_WATCH_INFORMATION_EX>(), 8);
    assert_eq!(size_of::<PROCESS_PRIORITY_CLASS>(), 2);
    assert_eq!(align_of::<PROCESS_PRIORITY_CLASS>(), 1);
    assert_eq!(size_of::<PROCESS_FOREGROUND_BACKGROUND>(), 1);
    assert_eq!(align_of::<PROCESS_FOREGROUND_BACKGROUND>(), 1);
    assert_eq!(size_of::<PROCESS_DEVICEMAP_INFORMATION>(), 40);
    assert_eq!(align_of::<PROCESS_DEVICEMAP_INFORMATION>(), 8);
    assert_eq!(size_of::<PROCESS_DEVICEMAP_INFORMATION_EX>(), 48);
    assert_eq!(align_of::<PROCESS_DEVICEMAP_INFORMATION_EX>(), 8);
    assert_eq!(size_of::<PROCESS_SESSION_INFORMATION>(), 4);
    assert_eq!(align_of::<PROCESS_SESSION_INFORMATION>(), 4);
    assert_eq!(size_of::<PROCESS_HANDLE_TRACING_ENABLE>(), 4);
    assert_eq!(align_of::<PROCESS_HANDLE_TRACING_ENABLE>(), 4);
    assert_eq!(size_of::<PROCESS_HANDLE_TRACING_ENABLE_EX>(), 8);
    assert_eq!(align_of::<PROCESS_HANDLE_TRACING_ENABLE_EX>(), 4);
    assert_eq!(size_of::<PROCESS_HANDLE_TRACING_ENTRY>(), 160);
    assert_eq!(align_of::<PROCESS_HANDLE_TRACING_ENTRY>(), 8);
    assert_eq!(size_of::<PROCESS_HANDLE_TRACING_QUERY>(), 176);
    assert_eq!(align_of::<PROCESS_HANDLE_TRACING_QUERY>(), 8);
    assert_eq!(size_of::<THREAD_TLS_INFORMATION>(), 32);
    assert_eq!(align_of::<THREAD_TLS_INFORMATION>(), 8);
    assert_eq!(size_of::<PROCESS_TLS_INFORMATION>(), 56);
    assert_eq!(align_of::<PROCESS_TLS_INFORMATION>(), 8);
    assert_eq!(
        size_of::<PROCESS_INSTRUMENTATION_CALLBACK_INFORMATION>(),
        16
    );
    assert_eq!(
        align_of::<PROCESS_INSTRUMENTATION_CALLBACK_INFORMATION>(),
        8
    );
    assert_eq!(size_of::<PROCESS_STACK_ALLOCATION_INFORMATION>(), 24);
    assert_eq!(align_of::<PROCESS_STACK_ALLOCATION_INFORMATION>(), 8);
    assert_eq!(size_of::<PROCESS_STACK_ALLOCATION_INFORMATION_EX>(), 40);
    assert_eq!(align_of::<PROCESS_STACK_ALLOCATION_INFORMATION_EX>(), 8);
    assert_eq!(size_of::<PROCESS_AFFINITY_UPDATE_MODE>(), 4);
    assert_eq!(align_of::<PROCESS_AFFINITY_UPDATE_MODE>(), 4);
    assert_eq!(size_of::<PROCESS_MEMORY_ALLOCATION_MODE>(), 4);
    assert_eq!(align_of::<PROCESS_MEMORY_ALLOCATION_MODE>(), 4);
    assert_eq!(size_of::<PROCESS_HANDLE_INFORMATION>(), 8);
    assert_eq!(align_of::<PROCESS_HANDLE_INFORMATION>(), 4);
    assert_eq!(size_of::<PROCESS_CYCLE_TIME_INFORMATION>(), 16);
    assert_eq!(align_of::<PROCESS_CYCLE_TIME_INFORMATION>(), 8);
    assert_eq!(size_of::<PROCESS_WINDOW_INFORMATION>(), 8);
    assert_eq!(align_of::<PROCESS_WINDOW_INFORMATION>(), 4);
    assert_eq!(size_of::<PROCESS_HANDLE_TABLE_ENTRY_INFO>(), 40);
    assert_eq!(align_of::<PROCESS_HANDLE_TABLE_ENTRY_INFO>(), 8);
    assert_eq!(size_of::<PROCESS_HANDLE_SNAPSHOT_INFORMATION>(), 56);
    assert_eq!(align_of::<PROCESS_HANDLE_SNAPSHOT_INFORMATION>(), 8);
    assert_eq!(size_of::<PROCESS_MITIGATION_POLICY_INFORMATION>(), 8);
    assert_eq!(align_of::<PROCESS_MITIGATION_POLICY_INFORMATION>(), 4);
    assert_eq!(size_of::<PROCESS_KEEPALIVE_COUNT_INFORMATION>(), 8);
    assert_eq!(align_of::<PROCESS_KEEPALIVE_COUNT_INFORMATION>(), 4);
    assert_eq!(size_of::<PROCESS_REVOKE_FILE_HANDLES_INFORMATION>(), 16);
    assert_eq!(align_of::<PROCESS_REVOKE_FILE_HANDLES_INFORMATION>(), 8);
    assert_eq!(size_of::<PROCESS_WORKING_SET_CONTROL>(), 12);
    assert_eq!(align_of::<PROCESS_WORKING_SET_CONTROL>(), 4);
    assert_eq!(size_of::<PS_PROTECTION>(), 1);
    assert_eq!(align_of::<PS_PROTECTION>(), 1);
    assert_eq!(size_of::<PROCESS_FAULT_INFORMATION>(), 8);
    assert_eq!(align_of::<PROCESS_FAULT_INFORMATION>(), 4);
    assert_eq!(size_of::<PROCESS_TELEMETRY_ID_INFORMATION>(), 96);
    assert_eq!(align_of::<PROCESS_TELEMETRY_ID_INFORMATION>(), 8);
    assert_eq!(size_of::<PROCESS_COMMIT_RELEASE_INFORMATION>(), 32);
    assert_eq!(align_of::<PROCESS_COMMIT_RELEASE_INFORMATION>(), 8);
    assert_eq!(size_of::<PROCESS_JOB_MEMORY_INFO>(), 40);
    assert_eq!(align_of::<PROCESS_JOB_MEMORY_INFO>(), 8);
    assert_eq!(size_of::<PROCESS_CHILD_PROCESS_INFORMATION>(), 3);
    assert_eq!(align_of::<PROCESS_CHILD_PROCESS_INFORMATION>(), 1);
    assert_eq!(size_of::<PROCESS_WAKE_INFORMATION>(), 48);
    assert_eq!(align_of::<PROCESS_WAKE_INFORMATION>(), 8);
    assert_eq!(size_of::<PROCESS_ENERGY_TRACKING_STATE>(), 144);
    assert_eq!(align_of::<PROCESS_ENERGY_TRACKING_STATE>(), 4);
    assert_eq!(size_of::<MANAGE_WRITES_TO_EXECUTABLE_MEMORY>(), 4);
    assert_eq!(align_of::<MANAGE_WRITES_TO_EXECUTABLE_MEMORY>(), 4);
    assert_eq!(size_of::<PROCESS_READWRITEVM_LOGGING_INFORMATION>(), 1);
    assert_eq!(align_of::<PROCESS_READWRITEVM_LOGGING_INFORMATION>(), 1);
    assert_eq!(size_of::<PROCESS_UPTIME_INFORMATION>(), 56);
    assert_eq!(align_of::<PROCESS_UPTIME_INFORMATION>(), 8);
    assert_eq!(size_of::<PROCESS_SYSTEM_RESOURCE_MANAGEMENT>(), 4);
    assert_eq!(align_of::<PROCESS_SYSTEM_RESOURCE_MANAGEMENT>(), 4);
    assert_eq!(size_of::<PROCESS_SECURITY_DOMAIN_INFORMATION>(), 8);
    assert_eq!(align_of::<PROCESS_SECURITY_DOMAIN_INFORMATION>(), 8);
    assert_eq!(size_of::<PROCESS_COMBINE_SECURITY_DOMAINS_INFORMATION>(), 8);
    assert_eq!(
        align_of::<PROCESS_COMBINE_SECURITY_DOMAINS_INFORMATION>(),
        8
    );
    assert_eq!(size_of::<PROCESS_LOGGING_INFORMATION>(), 8);
    assert_eq!(align_of::<PROCESS_LOGGING_INFORMATION>(), 4);
    assert_eq!(size_of::<PROCESS_LEAP_SECOND_INFORMATION>(), 8);
    assert_eq!(align_of::<PROCESS_LEAP_SECOND_INFORMATION>(), 4);
    assert_eq!(size_of::<THREAD_BASIC_INFORMATION>(), 48);
    assert_eq!(align_of::<THREAD_BASIC_INFORMATION>(), 8);
    assert_eq!(size_of::<THREAD_LAST_SYSCALL_INFORMATION>(), 24);
    assert_eq!(align_of::<THREAD_LAST_SYSCALL_INFORMATION>(), 8);
    assert_eq!(size_of::<THREAD_CYCLE_TIME_INFORMATION>(), 16);
    assert_eq!(align_of::<THREAD_CYCLE_TIME_INFORMATION>(), 8);
    assert_eq!(size_of::<THREAD_TEB_INFORMATION>(), 16);
    assert_eq!(align_of::<THREAD_TEB_INFORMATION>(), 8);
    assert_eq!(size_of::<COUNTER_READING>(), 24);
    assert_eq!(align_of::<COUNTER_READING>(), 8);
    assert_eq!(size_of::<THREAD_PERFORMANCE_DATA>(), 448);
    assert_eq!(align_of::<THREAD_PERFORMANCE_DATA>(), 8);
    assert_eq!(size_of::<THREAD_PROFILING_INFORMATION>(), 24);
    assert_eq!(align_of::<THREAD_PROFILING_INFORMATION>(), 8);
    assert_eq!(size_of::<RTL_UMS_CONTEXT>(), 1024);
    assert_eq!(align_of::<RTL_UMS_CONTEXT>(), 16);
    assert_eq!(FIELD_OFFSET!(RTL_UMS_CONTEXT, Link), 0);
    assert_eq!(FIELD_OFFSET!(RTL_UMS_CONTEXT, Context), 16);
    assert_eq!(FIELD_OFFSET!(RTL_UMS_CONTEXT, Teb), 928);
    assert_eq!(FIELD_OFFSET!(RTL_UMS_CONTEXT, UserContext), 936);
    assert_eq!(FIELD_OFFSET!(RTL_UMS_CONTEXT, ScheduledThread), 944);
    assert_eq!(FIELD_OFFSET!(RTL_UMS_CONTEXT, Suspended), 948);
    assert_eq!(FIELD_OFFSET!(RTL_UMS_CONTEXT, VolatileContext), 952);
    assert_eq!(FIELD_OFFSET!(RTL_UMS_CONTEXT, Terminated), 956);
    assert_eq!(FIELD_OFFSET!(RTL_UMS_CONTEXT, DebugActive), 960);
    assert_eq!(FIELD_OFFSET!(RTL_UMS_CONTEXT, RunningOnSelfThread), 964);
    assert_eq!(FIELD_OFFSET!(RTL_UMS_CONTEXT, DenyRunningOnSelfThread), 968);
    assert_eq!(FIELD_OFFSET!(RTL_UMS_CONTEXT, Flags), 972);
    assert_eq!(FIELD_OFFSET!(RTL_UMS_CONTEXT, KernelUpdateLock), 976);
    assert_eq!(FIELD_OFFSET!(RTL_UMS_CONTEXT, PrimaryClientID), 984);
    assert_eq!(FIELD_OFFSET!(RTL_UMS_CONTEXT, ContextLock), 992);
    assert_eq!(FIELD_OFFSET!(RTL_UMS_CONTEXT, PrimaryUmsContext), 1000);
    assert_eq!(FIELD_OFFSET!(RTL_UMS_CONTEXT, SwitchCount), 1008);
    assert_eq!(FIELD_OFFSET!(RTL_UMS_CONTEXT, KernelYieldCount), 1012);
    assert_eq!(FIELD_OFFSET!(RTL_UMS_CONTEXT, MixedYieldCount), 1016);
    assert_eq!(FIELD_OFFSET!(RTL_UMS_CONTEXT, YieldCount), 1020);
    assert_eq!(size_of::<RTL_UMS_COMPLETION_LIST>(), 32);
    assert_eq!(align_of::<RTL_UMS_COMPLETION_LIST>(), 8);
    assert_eq!(size_of::<THREAD_UMS_INFORMATION>(), 32);
    assert_eq!(align_of::<THREAD_UMS_INFORMATION>(), 8);
    assert_eq!(size_of::<THREAD_NAME_INFORMATION>(), 16);
    assert_eq!(align_of::<THREAD_NAME_INFORMATION>(), 8);
    assert_eq!(size_of::<PS_ATTRIBUTE>(), 32);
    assert_eq!(align_of::<PS_ATTRIBUTE>(), 8);
    assert_eq!(size_of::<PS_ATTRIBUTE_LIST>(), 40);
    assert_eq!(align_of::<PS_ATTRIBUTE_LIST>(), 8);
    assert_eq!(size_of::<PS_MEMORY_RESERVE>(), 16);
    assert_eq!(align_of::<PS_MEMORY_RESERVE>(), 8);
    assert_eq!(size_of::<PS_STD_HANDLE_INFO>(), 8);
    assert_eq!(align_of::<PS_STD_HANDLE_INFO>(), 4);
    assert_eq!(size_of::<PS_BNO_ISOLATION_PARAMETERS>(), 40);
    assert_eq!(align_of::<PS_BNO_ISOLATION_PARAMETERS>(), 8);
    assert_eq!(size_of::<PS_CREATE_INFO>(), 88);
    assert_eq!(align_of::<PS_CREATE_INFO>(), 8);
    assert_eq!(size_of::<JOBOBJECT_EXTENDED_ACCOUNTING_INFORMATION>(), 432);
    assert_eq!(align_of::<JOBOBJECT_EXTENDED_ACCOUNTING_INFORMATION>(), 8);
    assert_eq!(size_of::<JOBOBJECT_WAKE_INFORMATION>(), 64);
    assert_eq!(align_of::<JOBOBJECT_WAKE_INFORMATION>(), 8);
    assert_eq!(size_of::<JOBOBJECT_WAKE_INFORMATION_V1>(), 40);
    assert_eq!(align_of::<JOBOBJECT_WAKE_INFORMATION_V1>(), 8);
    assert_eq!(size_of::<JOBOBJECT_INTERFERENCE_INFORMATION>(), 8);
    assert_eq!(align_of::<JOBOBJECT_INTERFERENCE_INFORMATION>(), 8);
    assert_eq!(size_of::<JOBOBJECT_WAKE_FILTER>(), 8);
    assert_eq!(align_of::<JOBOBJECT_WAKE_FILTER>(), 4);
    assert_eq!(size_of::<JOBOBJECT_FREEZE_INFORMATION>(), 16);
    assert_eq!(align_of::<JOBOBJECT_FREEZE_INFORMATION>(), 4);
    assert_eq!(size_of::<JOBOBJECT_MEMORY_USAGE_INFORMATION>(), 16);
    assert_eq!(align_of::<JOBOBJECT_MEMORY_USAGE_INFORMATION>(), 8);
    assert_eq!(size_of::<JOBOBJECT_MEMORY_USAGE_INFORMATION_V2>(), 40);
    assert_eq!(align_of::<JOBOBJECT_MEMORY_USAGE_INFORMATION_V2>(), 8);
    assert_eq!(size_of::<SILO_USER_SHARED_DATA>(), 592);
    assert_eq!(align_of::<SILO_USER_SHARED_DATA>(), 8);
    assert_eq!(size_of::<SILOOBJECT_ROOT_DIRECTORY>(), 24);
    assert_eq!(align_of::<SILOOBJECT_ROOT_DIRECTORY>(), 8);
    assert_eq!(size_of::<JOBOBJECT_ENERGY_TRACKING_STATE>(), 16);
    assert_eq!(align_of::<JOBOBJECT_ENERGY_TRACKING_STATE>(), 8);
}
#[test]
fn ntregapi() {
    use ntapi::ntregapi::*;
    assert_eq!(size_of::<KEY_BASIC_INFORMATION>(), 24);
    assert_eq!(align_of::<KEY_BASIC_INFORMATION>(), 8);
    assert_eq!(size_of::<KEY_NODE_INFORMATION>(), 32);
    assert_eq!(align_of::<KEY_NODE_INFORMATION>(), 8);
    assert_eq!(size_of::<KEY_FULL_INFORMATION>(), 48);
    assert_eq!(align_of::<KEY_FULL_INFORMATION>(), 8);
    assert_eq!(size_of::<KEY_NAME_INFORMATION>(), 8);
    assert_eq!(align_of::<KEY_NAME_INFORMATION>(), 4);
    assert_eq!(size_of::<KEY_CACHED_INFORMATION>(), 40);
    assert_eq!(align_of::<KEY_CACHED_INFORMATION>(), 8);
    assert_eq!(size_of::<KEY_FLAGS_INFORMATION>(), 4);
    assert_eq!(align_of::<KEY_FLAGS_INFORMATION>(), 4);
    assert_eq!(size_of::<KEY_VIRTUALIZATION_INFORMATION>(), 4);
    assert_eq!(align_of::<KEY_VIRTUALIZATION_INFORMATION>(), 4);
    assert_eq!(size_of::<KEY_TRUST_INFORMATION>(), 4);
    assert_eq!(align_of::<KEY_TRUST_INFORMATION>(), 4);
    assert_eq!(size_of::<KEY_LAYER_INFORMATION>(), 20);
    assert_eq!(align_of::<KEY_LAYER_INFORMATION>(), 4);
    assert_eq!(size_of::<KEY_WRITE_TIME_INFORMATION>(), 8);
    assert_eq!(align_of::<KEY_WRITE_TIME_INFORMATION>(), 8);
    assert_eq!(size_of::<KEY_WOW64_FLAGS_INFORMATION>(), 4);
    assert_eq!(align_of::<KEY_WOW64_FLAGS_INFORMATION>(), 4);
    assert_eq!(size_of::<KEY_HANDLE_TAGS_INFORMATION>(), 4);
    assert_eq!(align_of::<KEY_HANDLE_TAGS_INFORMATION>(), 4);
    assert_eq!(size_of::<KEY_SET_LAYER_INFORMATION>(), 4);
    assert_eq!(align_of::<KEY_SET_LAYER_INFORMATION>(), 4);
    assert_eq!(size_of::<KEY_CONTROL_FLAGS_INFORMATION>(), 4);
    assert_eq!(align_of::<KEY_CONTROL_FLAGS_INFORMATION>(), 4);
    assert_eq!(size_of::<KEY_SET_VIRTUALIZATION_INFORMATION>(), 4);
    assert_eq!(align_of::<KEY_SET_VIRTUALIZATION_INFORMATION>(), 4);
    assert_eq!(size_of::<KEY_VALUE_BASIC_INFORMATION>(), 16);
    assert_eq!(align_of::<KEY_VALUE_BASIC_INFORMATION>(), 4);
    assert_eq!(size_of::<KEY_VALUE_FULL_INFORMATION>(), 24);
    assert_eq!(align_of::<KEY_VALUE_FULL_INFORMATION>(), 4);
    assert_eq!(size_of::<KEY_VALUE_PARTIAL_INFORMATION>(), 16);
    assert_eq!(align_of::<KEY_VALUE_PARTIAL_INFORMATION>(), 4);
    assert_eq!(size_of::<KEY_VALUE_PARTIAL_INFORMATION_ALIGN64>(), 12);
    assert_eq!(align_of::<KEY_VALUE_PARTIAL_INFORMATION_ALIGN64>(), 4);
    assert_eq!(size_of::<KEY_VALUE_LAYER_INFORMATION>(), 8);
    assert_eq!(align_of::<KEY_VALUE_LAYER_INFORMATION>(), 4);
    assert_eq!(size_of::<KEY_VALUE_ENTRY>(), 24);
    assert_eq!(align_of::<KEY_VALUE_ENTRY>(), 8);
    assert_eq!(size_of::<REG_NOTIFY_INFORMATION>(), 16);
    assert_eq!(align_of::<REG_NOTIFY_INFORMATION>(), 4);
    assert_eq!(size_of::<KEY_PID_ARRAY>(), 24);
    assert_eq!(align_of::<KEY_PID_ARRAY>(), 8);
    assert_eq!(size_of::<KEY_OPEN_SUBKEYS_INFORMATION>(), 32);
    assert_eq!(align_of::<KEY_OPEN_SUBKEYS_INFORMATION>(), 8);
}
#[test]
fn ntrtl() {
    use ntapi::ntrtl::*;
    assert_eq!(size_of::<RTL_BALANCED_LINKS>(), 32);
    assert_eq!(align_of::<RTL_BALANCED_LINKS>(), 8);
    assert_eq!(size_of::<RTL_SPLAY_LINKS>(), 24);
    assert_eq!(align_of::<RTL_SPLAY_LINKS>(), 8);
    assert_eq!(size_of::<RTL_GENERIC_TABLE>(), 72);
    assert_eq!(align_of::<RTL_GENERIC_TABLE>(), 8);
    assert_eq!(size_of::<RTL_RB_TREE>(), 16);
    assert_eq!(align_of::<RTL_RB_TREE>(), 8);
    assert_eq!(size_of::<RTL_DYNAMIC_HASH_TABLE_ENTRY>(), 24);
    assert_eq!(align_of::<RTL_DYNAMIC_HASH_TABLE_ENTRY>(), 8);
    assert_eq!(size_of::<RTL_DYNAMIC_HASH_TABLE_CONTEXT>(), 24);
    assert_eq!(align_of::<RTL_DYNAMIC_HASH_TABLE_CONTEXT>(), 8);
    assert_eq!(size_of::<RTL_DYNAMIC_HASH_TABLE_ENUMERATOR>(), 40);
    assert_eq!(align_of::<RTL_DYNAMIC_HASH_TABLE_ENUMERATOR>(), 8);
    assert_eq!(size_of::<RTL_DYNAMIC_HASH_TABLE>(), 40);
    assert_eq!(align_of::<RTL_DYNAMIC_HASH_TABLE>(), 8);
    assert_eq!(size_of::<RTL_RESOURCE>(), 96);
    assert_eq!(align_of::<RTL_RESOURCE>(), 8);
    assert_eq!(size_of::<PREFIX_TABLE_ENTRY>(), 48);
    assert_eq!(align_of::<PREFIX_TABLE_ENTRY>(), 8);
    assert_eq!(size_of::<PREFIX_TABLE>(), 16);
    assert_eq!(align_of::<PREFIX_TABLE>(), 8);
    assert_eq!(size_of::<UNICODE_PREFIX_TABLE_ENTRY>(), 56);
    assert_eq!(align_of::<UNICODE_PREFIX_TABLE_ENTRY>(), 8);
    assert_eq!(size_of::<UNICODE_PREFIX_TABLE>(), 24);
    assert_eq!(align_of::<UNICODE_PREFIX_TABLE>(), 8);
    assert_eq!(size_of::<COMPRESSED_DATA_INFO>(), 12);
    assert_eq!(align_of::<COMPRESSED_DATA_INFO>(), 4);
    assert_eq!(size_of::<CURDIR>(), 24);
    assert_eq!(align_of::<CURDIR>(), 8);
    assert_eq!(size_of::<RTL_DRIVE_LETTER_CURDIR>(), 24);
    assert_eq!(align_of::<RTL_DRIVE_LETTER_CURDIR>(), 8);
    assert_eq!(size_of::<RTL_USER_PROCESS_PARAMETERS>(), 1040);
    assert_eq!(align_of::<RTL_USER_PROCESS_PARAMETERS>(), 8);
    assert_eq!(size_of::<RTL_USER_PROCESS_INFORMATION>(), 104);
    assert_eq!(align_of::<RTL_USER_PROCESS_INFORMATION>(), 8);
    assert_eq!(
        size_of::<RTLP_PROCESS_REFLECTION_REFLECTION_INFORMATION>(),
        32
    );
    assert_eq!(
        align_of::<RTLP_PROCESS_REFLECTION_REFLECTION_INFORMATION>(),
        8
    );
    assert_eq!(size_of::<CONTEXT_CHUNK>(), 8);
    assert_eq!(align_of::<CONTEXT_CHUNK>(), 4);
    assert_eq!(size_of::<CONTEXT_EX>(), 24);
    assert_eq!(align_of::<CONTEXT_EX>(), 4);
    assert_eq!(size_of::<DYNAMIC_FUNCTION_TABLE>(), 112);
    assert_eq!(align_of::<DYNAMIC_FUNCTION_TABLE>(), 8);
    assert_eq!(size_of::<RTLP_CURDIR_REF>(), 16);
    assert_eq!(align_of::<RTLP_CURDIR_REF>(), 8);
    assert_eq!(size_of::<RTL_RELATIVE_NAME_U>(), 32);
    assert_eq!(align_of::<RTL_RELATIVE_NAME_U>(), 8);
    assert_eq!(size_of::<GENERATE_NAME_CONTEXT>(), 36);
    assert_eq!(align_of::<GENERATE_NAME_CONTEXT>(), 4);
    assert_eq!(size_of::<RTL_HEAP_ENTRY>(), 32);
    assert_eq!(align_of::<RTL_HEAP_ENTRY>(), 8);
    assert_eq!(size_of::<RTL_HEAP_TAG>(), 72);
    assert_eq!(align_of::<RTL_HEAP_TAG>(), 8);
    assert_eq!(size_of::<RTL_HEAP_INFORMATION>(), 88);
    assert_eq!(align_of::<RTL_HEAP_INFORMATION>(), 8);
    assert_eq!(size_of::<RTL_PROCESS_HEAPS>(), 96);
    assert_eq!(align_of::<RTL_PROCESS_HEAPS>(), 8);
    assert_eq!(size_of::<RTL_HEAP_PARAMETERS>(), 96);
    assert_eq!(align_of::<RTL_HEAP_PARAMETERS>(), 8);
    assert_eq!(size_of::<RTL_HEAP_TAG_INFO>(), 16);
    assert_eq!(align_of::<RTL_HEAP_TAG_INFO>(), 8);
    assert_eq!(size_of::<RTL_HEAP_USAGE_ENTRY>(), 32);
    assert_eq!(align_of::<RTL_HEAP_USAGE_ENTRY>(), 8);
    assert_eq!(size_of::<RTL_HEAP_USAGE>(), 128);
    assert_eq!(align_of::<RTL_HEAP_USAGE>(), 8);
    assert_eq!(size_of::<RTL_HEAP_WALK_ENTRY>(), 48);
    assert_eq!(align_of::<RTL_HEAP_WALK_ENTRY>(), 8);
    assert_eq!(size_of::<PROCESS_HEAP_INFORMATION>(), 32);
    assert_eq!(align_of::<PROCESS_HEAP_INFORMATION>(), 8);
    assert_eq!(size_of::<HEAP_INFORMATION>(), 48);
    assert_eq!(align_of::<HEAP_INFORMATION>(), 8);
    assert_eq!(size_of::<HEAP_EXTENDED_INFORMATION>(), 88);
    assert_eq!(align_of::<HEAP_EXTENDED_INFORMATION>(), 8);
    assert_eq!(size_of::<HEAP_DEBUGGING_INFORMATION>(), 48);
    assert_eq!(align_of::<HEAP_DEBUGGING_INFORMATION>(), 8);
    assert_eq!(size_of::<RTL_MEMORY_ZONE_SEGMENT>(), 32);
    assert_eq!(align_of::<RTL_MEMORY_ZONE_SEGMENT>(), 8);
    assert_eq!(size_of::<RTL_MEMORY_ZONE>(), 56);
    assert_eq!(align_of::<RTL_MEMORY_ZONE>(), 8);
    assert_eq!(size_of::<RTL_PROCESS_VERIFIER_OPTIONS>(), 12);
    assert_eq!(align_of::<RTL_PROCESS_VERIFIER_OPTIONS>(), 4);
    assert_eq!(size_of::<RTL_DEBUG_INFORMATION>(), 208);
    assert_eq!(align_of::<RTL_DEBUG_INFORMATION>(), 8);
    assert_eq!(size_of::<PARSE_MESSAGE_CONTEXT>(), 40);
    assert_eq!(align_of::<PARSE_MESSAGE_CONTEXT>(), 8);
    assert_eq!(size_of::<TIME_FIELDS>(), 16);
    assert_eq!(align_of::<TIME_FIELDS>(), 2);
    assert_eq!(size_of::<RTL_TIME_ZONE_INFORMATION>(), 172);
    assert_eq!(align_of::<RTL_TIME_ZONE_INFORMATION>(), 4);
    assert_eq!(size_of::<RTL_BITMAP>(), 16);
    assert_eq!(align_of::<RTL_BITMAP>(), 8);
    assert_eq!(size_of::<RTL_BITMAP_RUN>(), 8);
    assert_eq!(align_of::<RTL_BITMAP_RUN>(), 4);
    assert_eq!(size_of::<RTL_BITMAP_EX>(), 16);
    assert_eq!(align_of::<RTL_BITMAP_EX>(), 8);
    assert_eq!(size_of::<RTL_HANDLE_TABLE_ENTRY>(), 8);
    assert_eq!(align_of::<RTL_HANDLE_TABLE_ENTRY>(), 8);
    assert_eq!(size_of::<RTL_HANDLE_TABLE>(), 48);
    assert_eq!(align_of::<RTL_HANDLE_TABLE>(), 8);
    assert_eq!(size_of::<RTL_QUERY_REGISTRY_TABLE>(), 56);
    assert_eq!(align_of::<RTL_QUERY_REGISTRY_TABLE>(), 8);
    assert_eq!(size_of::<RTL_ELEVATION_FLAGS>(), 4);
    assert_eq!(align_of::<RTL_ELEVATION_FLAGS>(), 4);
    assert_eq!(size_of::<RTL_UNLOAD_EVENT_TRACE>(), 104);
    assert_eq!(align_of::<RTL_UNLOAD_EVENT_TRACE>(), 8);
    assert_eq!(size_of::<RTL_UNLOAD_EVENT_TRACE32>(), 92);
    assert_eq!(align_of::<RTL_UNLOAD_EVENT_TRACE32>(), 4);
    assert_eq!(size_of::<RTL_IMAGE_MITIGATION_POLICY>(), 8);
    assert_eq!(align_of::<RTL_IMAGE_MITIGATION_POLICY>(), 8);
    assert_eq!(size_of::<RTL_IMAGE_MITIGATION_DEP_POLICY>(), 8);
    assert_eq!(align_of::<RTL_IMAGE_MITIGATION_DEP_POLICY>(), 8);
    assert_eq!(size_of::<RTL_IMAGE_MITIGATION_ASLR_POLICY>(), 24);
    assert_eq!(align_of::<RTL_IMAGE_MITIGATION_ASLR_POLICY>(), 8);
    assert_eq!(size_of::<RTL_IMAGE_MITIGATION_DYNAMIC_CODE_POLICY>(), 8);
    assert_eq!(align_of::<RTL_IMAGE_MITIGATION_DYNAMIC_CODE_POLICY>(), 8);
    assert_eq!(
        size_of::<RTL_IMAGE_MITIGATION_STRICT_HANDLE_CHECK_POLICY>(),
        8
    );
    assert_eq!(
        align_of::<RTL_IMAGE_MITIGATION_STRICT_HANDLE_CHECK_POLICY>(),
        8
    );
    assert_eq!(
        size_of::<RTL_IMAGE_MITIGATION_SYSTEM_CALL_DISABLE_POLICY>(),
        8
    );
    assert_eq!(
        align_of::<RTL_IMAGE_MITIGATION_SYSTEM_CALL_DISABLE_POLICY>(),
        8
    );
    assert_eq!(
        size_of::<RTL_IMAGE_MITIGATION_EXTENSION_POINT_DISABLE_POLICY>(),
        8
    );
    assert_eq!(
        align_of::<RTL_IMAGE_MITIGATION_EXTENSION_POINT_DISABLE_POLICY>(),
        8
    );
    assert_eq!(
        size_of::<RTL_IMAGE_MITIGATION_CONTROL_FLOW_GUARD_POLICY>(),
        16
    );
    assert_eq!(
        align_of::<RTL_IMAGE_MITIGATION_CONTROL_FLOW_GUARD_POLICY>(),
        8
    );
    assert_eq!(
        size_of::<RTL_IMAGE_MITIGATION_BINARY_SIGNATURE_POLICY>(),
        16
    );
    assert_eq!(
        align_of::<RTL_IMAGE_MITIGATION_BINARY_SIGNATURE_POLICY>(),
        8
    );
    assert_eq!(size_of::<RTL_IMAGE_MITIGATION_FONT_DISABLE_POLICY>(), 8);
    assert_eq!(align_of::<RTL_IMAGE_MITIGATION_FONT_DISABLE_POLICY>(), 8);
    assert_eq!(size_of::<RTL_IMAGE_MITIGATION_IMAGE_LOAD_POLICY>(), 24);
    assert_eq!(align_of::<RTL_IMAGE_MITIGATION_IMAGE_LOAD_POLICY>(), 8);
    assert_eq!(
        size_of::<RTL_IMAGE_MITIGATION_PAYLOAD_RESTRICTION_POLICY>(),
        48
    );
    assert_eq!(
        align_of::<RTL_IMAGE_MITIGATION_PAYLOAD_RESTRICTION_POLICY>(),
        8
    );
    assert_eq!(size_of::<RTL_IMAGE_MITIGATION_CHILD_PROCESS_POLICY>(), 8);
    assert_eq!(align_of::<RTL_IMAGE_MITIGATION_CHILD_PROCESS_POLICY>(), 8);
    assert_eq!(size_of::<RTL_IMAGE_MITIGATION_SEHOP_POLICY>(), 8);
    assert_eq!(align_of::<RTL_IMAGE_MITIGATION_SEHOP_POLICY>(), 8);
    assert_eq!(size_of::<RTL_IMAGE_MITIGATION_HEAP_POLICY>(), 8);
    assert_eq!(align_of::<RTL_IMAGE_MITIGATION_HEAP_POLICY>(), 8);
    assert_eq!(size_of::<PS_PKG_CLAIM>(), 16);
    assert_eq!(align_of::<PS_PKG_CLAIM>(), 8);
    assert_eq!(size_of::<RTL_BSD_ITEM>(), 24);
    assert_eq!(align_of::<RTL_BSD_ITEM>(), 8);
}
#[test]
fn ntsam() {
    use ntapi::ntsam::*;
    assert_eq!(size_of::<SAM_RID_ENUMERATION>(), 24);
    assert_eq!(align_of::<SAM_RID_ENUMERATION>(), 8);
    assert_eq!(size_of::<SAM_SID_ENUMERATION>(), 24);
    assert_eq!(align_of::<SAM_SID_ENUMERATION>(), 8);
    assert_eq!(size_of::<SAM_BYTE_ARRAY>(), 16);
    assert_eq!(align_of::<SAM_BYTE_ARRAY>(), 8);
    assert_eq!(size_of::<SAM_BYTE_ARRAY_32K>(), 16);
    assert_eq!(align_of::<SAM_BYTE_ARRAY_32K>(), 8);
    assert_eq!(size_of::<DOMAIN_GENERAL_INFORMATION>(), 88);
    assert_eq!(align_of::<DOMAIN_GENERAL_INFORMATION>(), 4);
    assert_eq!(FIELD_OFFSET!(DOMAIN_GENERAL_INFORMATION, ForceLogoff), 0);
    assert_eq!(FIELD_OFFSET!(DOMAIN_GENERAL_INFORMATION, OemInformation), 8);
    assert_eq!(FIELD_OFFSET!(DOMAIN_GENERAL_INFORMATION, DomainName), 24);
    assert_eq!(
        FIELD_OFFSET!(DOMAIN_GENERAL_INFORMATION, ReplicaSourceNodeName),
        40
    );
    assert_eq!(
        FIELD_OFFSET!(DOMAIN_GENERAL_INFORMATION, DomainModifiedCount),
        56
    );
    assert_eq!(
        FIELD_OFFSET!(DOMAIN_GENERAL_INFORMATION, DomainServerState),
        64
    );
    assert_eq!(
        FIELD_OFFSET!(DOMAIN_GENERAL_INFORMATION, DomainServerRole),
        68
    );
    assert_eq!(
        FIELD_OFFSET!(DOMAIN_GENERAL_INFORMATION, UasCompatibilityRequired),
        72
    );
    assert_eq!(FIELD_OFFSET!(DOMAIN_GENERAL_INFORMATION, UserCount), 76);
    assert_eq!(FIELD_OFFSET!(DOMAIN_GENERAL_INFORMATION, GroupCount), 80);
    assert_eq!(FIELD_OFFSET!(DOMAIN_GENERAL_INFORMATION, AliasCount), 84);
    assert_eq!(size_of::<DOMAIN_GENERAL_INFORMATION2>(), 108);
    assert_eq!(align_of::<DOMAIN_GENERAL_INFORMATION2>(), 4);
    assert_eq!(FIELD_OFFSET!(DOMAIN_GENERAL_INFORMATION2, I1), 0);
    assert_eq!(
        FIELD_OFFSET!(DOMAIN_GENERAL_INFORMATION2, LockoutDuration),
        88
    );
    assert_eq!(
        FIELD_OFFSET!(DOMAIN_GENERAL_INFORMATION2, LockoutObservationWindow),
        96
    );
    assert_eq!(
        FIELD_OFFSET!(DOMAIN_GENERAL_INFORMATION2, LockoutThreshold),
        104
    );
    assert_eq!(size_of::<DOMAIN_UAS_INFORMATION>(), 1);
    assert_eq!(align_of::<DOMAIN_UAS_INFORMATION>(), 1);
    assert_eq!(size_of::<DOMAIN_LOGOFF_INFORMATION>(), 8);
    assert_eq!(align_of::<DOMAIN_LOGOFF_INFORMATION>(), 8);
    assert_eq!(size_of::<DOMAIN_OEM_INFORMATION>(), 16);
    assert_eq!(align_of::<DOMAIN_OEM_INFORMATION>(), 8);
    assert_eq!(size_of::<DOMAIN_NAME_INFORMATION>(), 16);
    assert_eq!(align_of::<DOMAIN_NAME_INFORMATION>(), 8);
    assert_eq!(size_of::<DOMAIN_SERVER_ROLE_INFORMATION>(), 4);
    assert_eq!(align_of::<DOMAIN_SERVER_ROLE_INFORMATION>(), 4);
    assert_eq!(size_of::<DOMAIN_REPLICATION_INFORMATION>(), 16);
    assert_eq!(align_of::<DOMAIN_REPLICATION_INFORMATION>(), 8);
    assert_eq!(size_of::<DOMAIN_MODIFIED_INFORMATION>(), 16);
    assert_eq!(align_of::<DOMAIN_MODIFIED_INFORMATION>(), 8);
    assert_eq!(size_of::<DOMAIN_MODIFIED_INFORMATION2>(), 24);
    assert_eq!(align_of::<DOMAIN_MODIFIED_INFORMATION2>(), 8);
    assert_eq!(size_of::<DOMAIN_STATE_INFORMATION>(), 4);
    assert_eq!(align_of::<DOMAIN_STATE_INFORMATION>(), 4);
    assert_eq!(size_of::<DOMAIN_LOCKOUT_INFORMATION>(), 24);
    assert_eq!(align_of::<DOMAIN_LOCKOUT_INFORMATION>(), 8);
    assert_eq!(size_of::<DOMAIN_DISPLAY_USER>(), 64);
    assert_eq!(align_of::<DOMAIN_DISPLAY_USER>(), 8);
    assert_eq!(size_of::<DOMAIN_DISPLAY_MACHINE>(), 48);
    assert_eq!(align_of::<DOMAIN_DISPLAY_MACHINE>(), 8);
    assert_eq!(size_of::<DOMAIN_DISPLAY_GROUP>(), 48);
    assert_eq!(align_of::<DOMAIN_DISPLAY_GROUP>(), 8);
    assert_eq!(size_of::<DOMAIN_DISPLAY_OEM_USER>(), 24);
    assert_eq!(align_of::<DOMAIN_DISPLAY_OEM_USER>(), 8);
    assert_eq!(size_of::<DOMAIN_DISPLAY_OEM_GROUP>(), 24);
    assert_eq!(align_of::<DOMAIN_DISPLAY_OEM_GROUP>(), 8);
    assert_eq!(size_of::<DOMAIN_LOCALIZABLE_ACCOUNT_ENTRY>(), 40);
    assert_eq!(align_of::<DOMAIN_LOCALIZABLE_ACCOUNT_ENTRY>(), 8);
    assert_eq!(size_of::<DOMAIN_LOCALIZABLE_ACCOUNTS_BASIC>(), 16);
    assert_eq!(align_of::<DOMAIN_LOCALIZABLE_ACCOUNTS_BASIC>(), 8);
    assert_eq!(size_of::<DOMAIN_LOCALIZABLE_ACCOUNTS_INFO_BUFFER>(), 16);
    assert_eq!(align_of::<DOMAIN_LOCALIZABLE_ACCOUNTS_INFO_BUFFER>(), 8);
    assert_eq!(size_of::<GROUP_MEMBERSHIP>(), 8);
    assert_eq!(align_of::<GROUP_MEMBERSHIP>(), 4);
    assert_eq!(size_of::<GROUP_GENERAL_INFORMATION>(), 40);
    assert_eq!(align_of::<GROUP_GENERAL_INFORMATION>(), 8);
    assert_eq!(size_of::<GROUP_NAME_INFORMATION>(), 16);
    assert_eq!(align_of::<GROUP_NAME_INFORMATION>(), 8);
    assert_eq!(size_of::<GROUP_ATTRIBUTE_INFORMATION>(), 4);
    assert_eq!(align_of::<GROUP_ATTRIBUTE_INFORMATION>(), 4);
    assert_eq!(size_of::<GROUP_ADM_COMMENT_INFORMATION>(), 16);
    assert_eq!(align_of::<GROUP_ADM_COMMENT_INFORMATION>(), 8);
    assert_eq!(size_of::<ALIAS_GENERAL_INFORMATION>(), 40);
    assert_eq!(align_of::<ALIAS_GENERAL_INFORMATION>(), 8);
    assert_eq!(size_of::<ALIAS_NAME_INFORMATION>(), 16);
    assert_eq!(align_of::<ALIAS_NAME_INFORMATION>(), 8);
    assert_eq!(size_of::<ALIAS_ADM_COMMENT_INFORMATION>(), 16);
    assert_eq!(align_of::<ALIAS_ADM_COMMENT_INFORMATION>(), 8);
    assert_eq!(size_of::<ALIAS_EXTENDED_INFORMATION>(), 24);
    assert_eq!(align_of::<ALIAS_EXTENDED_INFORMATION>(), 8);
    assert_eq!(size_of::<USER_GENERAL_INFORMATION>(), 72);
    assert_eq!(align_of::<USER_GENERAL_INFORMATION>(), 8);
    assert_eq!(size_of::<USER_PREFERENCES_INFORMATION>(), 40);
    assert_eq!(align_of::<USER_PREFERENCES_INFORMATION>(), 8);
    assert_eq!(size_of::<USER_PARAMETERS_INFORMATION>(), 16);
    assert_eq!(align_of::<USER_PARAMETERS_INFORMATION>(), 8);
    assert_eq!(size_of::<USER_LOGON_INFORMATION>(), 184);
    assert_eq!(align_of::<USER_LOGON_INFORMATION>(), 4);
    assert_eq!(FIELD_OFFSET!(USER_LOGON_INFORMATION, UserName), 0);
    assert_eq!(FIELD_OFFSET!(USER_LOGON_INFORMATION, FullName), 16);
    assert_eq!(FIELD_OFFSET!(USER_LOGON_INFORMATION, UserId), 32);
    assert_eq!(FIELD_OFFSET!(USER_LOGON_INFORMATION, PrimaryGroupId), 36);
    assert_eq!(FIELD_OFFSET!(USER_LOGON_INFORMATION, HomeDirectory), 40);
    assert_eq!(
        FIELD_OFFSET!(USER_LOGON_INFORMATION, HomeDirectoryDrive),
        56
    );
    assert_eq!(FIELD_OFFSET!(USER_LOGON_INFORMATION, ScriptPath), 72);
    assert_eq!(FIELD_OFFSET!(USER_LOGON_INFORMATION, ProfilePath), 88);
    assert_eq!(FIELD_OFFSET!(USER_LOGON_INFORMATION, WorkStations), 104);
    assert_eq!(FIELD_OFFSET!(USER_LOGON_INFORMATION, LastLogon), 120);
    assert_eq!(FIELD_OFFSET!(USER_LOGON_INFORMATION, LastLogoff), 128);
    assert_eq!(FIELD_OFFSET!(USER_LOGON_INFORMATION, PasswordLastSet), 136);
    assert_eq!(
        FIELD_OFFSET!(USER_LOGON_INFORMATION, PasswordCanChange),
        144
    );
    assert_eq!(
        FIELD_OFFSET!(USER_LOGON_INFORMATION, PasswordMustChange),
        152
    );
    assert_eq!(FIELD_OFFSET!(USER_LOGON_INFORMATION, LogonHours), 160);
    assert_eq!(FIELD_OFFSET!(USER_LOGON_INFORMATION, BadPasswordCount), 176);
    assert_eq!(FIELD_OFFSET!(USER_LOGON_INFORMATION, LogonCount), 178);
    assert_eq!(
        FIELD_OFFSET!(USER_LOGON_INFORMATION, UserAccountControl),
        180
    );
    assert_eq!(size_of::<USER_ACCOUNT_INFORMATION>(), 192);
    assert_eq!(align_of::<USER_ACCOUNT_INFORMATION>(), 4);
    assert_eq!(FIELD_OFFSET!(USER_ACCOUNT_INFORMATION, UserName), 0);
    assert_eq!(FIELD_OFFSET!(USER_ACCOUNT_INFORMATION, FullName), 16);
    assert_eq!(FIELD_OFFSET!(USER_ACCOUNT_INFORMATION, UserId), 32);
    assert_eq!(FIELD_OFFSET!(USER_ACCOUNT_INFORMATION, PrimaryGroupId), 36);
    assert_eq!(FIELD_OFFSET!(USER_ACCOUNT_INFORMATION, HomeDirectory), 40);
    assert_eq!(
        FIELD_OFFSET!(USER_ACCOUNT_INFORMATION, HomeDirectoryDrive),
        56
    );
    assert_eq!(FIELD_OFFSET!(USER_ACCOUNT_INFORMATION, ScriptPath), 72);
    assert_eq!(FIELD_OFFSET!(USER_ACCOUNT_INFORMATION, ProfilePath), 88);
    assert_eq!(FIELD_OFFSET!(USER_ACCOUNT_INFORMATION, AdminComment), 104);
    assert_eq!(FIELD_OFFSET!(USER_ACCOUNT_INFORMATION, WorkStations), 120);
    assert_eq!(FIELD_OFFSET!(USER_ACCOUNT_INFORMATION, LastLogon), 136);
    assert_eq!(FIELD_OFFSET!(USER_ACCOUNT_INFORMATION, LastLogoff), 144);
    assert_eq!(FIELD_OFFSET!(USER_ACCOUNT_INFORMATION, LogonHours), 152);
    assert_eq!(
        FIELD_OFFSET!(USER_ACCOUNT_INFORMATION, BadPasswordCount),
        168
    );
    assert_eq!(FIELD_OFFSET!(USER_ACCOUNT_INFORMATION, LogonCount), 170);
    assert_eq!(
        FIELD_OFFSET!(USER_ACCOUNT_INFORMATION, PasswordLastSet),
        172
    );
    assert_eq!(FIELD_OFFSET!(USER_ACCOUNT_INFORMATION, AccountExpires), 180);
    assert_eq!(
        FIELD_OFFSET!(USER_ACCOUNT_INFORMATION, UserAccountControl),
        188
    );
    assert_eq!(size_of::<USER_ACCOUNT_NAME_INFORMATION>(), 16);
    assert_eq!(align_of::<USER_ACCOUNT_NAME_INFORMATION>(), 8);
    assert_eq!(size_of::<USER_FULL_NAME_INFORMATION>(), 16);
    assert_eq!(align_of::<USER_FULL_NAME_INFORMATION>(), 8);
    assert_eq!(size_of::<USER_NAME_INFORMATION>(), 32);
    assert_eq!(align_of::<USER_NAME_INFORMATION>(), 8);
    assert_eq!(size_of::<USER_PRIMARY_GROUP_INFORMATION>(), 4);
    assert_eq!(align_of::<USER_PRIMARY_GROUP_INFORMATION>(), 4);
    assert_eq!(size_of::<USER_HOME_INFORMATION>(), 32);
    assert_eq!(align_of::<USER_HOME_INFORMATION>(), 8);
    assert_eq!(size_of::<USER_SCRIPT_INFORMATION>(), 16);
    assert_eq!(align_of::<USER_SCRIPT_INFORMATION>(), 8);
    assert_eq!(size_of::<USER_PROFILE_INFORMATION>(), 16);
    assert_eq!(align_of::<USER_PROFILE_INFORMATION>(), 8);
    assert_eq!(size_of::<USER_ADMIN_COMMENT_INFORMATION>(), 16);
    assert_eq!(align_of::<USER_ADMIN_COMMENT_INFORMATION>(), 8);
    assert_eq!(size_of::<USER_WORKSTATIONS_INFORMATION>(), 16);
    assert_eq!(align_of::<USER_WORKSTATIONS_INFORMATION>(), 8);
    assert_eq!(size_of::<USER_SET_PASSWORD_INFORMATION>(), 24);
    assert_eq!(align_of::<USER_SET_PASSWORD_INFORMATION>(), 8);
    assert_eq!(size_of::<USER_CONTROL_INFORMATION>(), 4);
    assert_eq!(align_of::<USER_CONTROL_INFORMATION>(), 4);
    assert_eq!(size_of::<USER_EXPIRES_INFORMATION>(), 8);
    assert_eq!(align_of::<USER_EXPIRES_INFORMATION>(), 8);
    assert_eq!(size_of::<USER_LOGON_HOURS_INFORMATION>(), 16);
    assert_eq!(align_of::<USER_LOGON_HOURS_INFORMATION>(), 8);
    assert_eq!(size_of::<USER_EXTENDED_INFORMATION>(), 64);
    assert_eq!(align_of::<USER_EXTENDED_INFORMATION>(), 8);
    assert_eq!(size_of::<USER_LOGON_UI_INFORMATION>(), 2);
    assert_eq!(align_of::<USER_LOGON_UI_INFORMATION>(), 1);
    assert_eq!(size_of::<USER_PWD_CHANGE_FAILURE_INFORMATION>(), 24);
    assert_eq!(align_of::<USER_PWD_CHANGE_FAILURE_INFORMATION>(), 8);
    assert_eq!(size_of::<SAM_GROUP_MEMBER_ID>(), 4);
    assert_eq!(align_of::<SAM_GROUP_MEMBER_ID>(), 4);
    assert_eq!(size_of::<SAM_ALIAS_MEMBER_ID>(), 8);
    assert_eq!(align_of::<SAM_ALIAS_MEMBER_ID>(), 8);
    assert_eq!(size_of::<SAM_DELTA_DATA>(), 8);
    assert_eq!(align_of::<SAM_DELTA_DATA>(), 8);
    assert_eq!(size_of::<SAM_VALIDATE_PASSWORD_HASH>(), 16);
    assert_eq!(align_of::<SAM_VALIDATE_PASSWORD_HASH>(), 8);
    assert_eq!(size_of::<SAM_VALIDATE_PERSISTED_FIELDS>(), 48);
    assert_eq!(align_of::<SAM_VALIDATE_PERSISTED_FIELDS>(), 8);
    assert_eq!(size_of::<SAM_VALIDATE_STANDARD_OUTPUT_ARG>(), 56);
    assert_eq!(align_of::<SAM_VALIDATE_STANDARD_OUTPUT_ARG>(), 8);
    assert_eq!(size_of::<SAM_VALIDATE_AUTHENTICATION_INPUT_ARG>(), 56);
    assert_eq!(align_of::<SAM_VALIDATE_AUTHENTICATION_INPUT_ARG>(), 8);
    assert_eq!(size_of::<SAM_VALIDATE_PASSWORD_CHANGE_INPUT_ARG>(), 104);
    assert_eq!(align_of::<SAM_VALIDATE_PASSWORD_CHANGE_INPUT_ARG>(), 8);
    assert_eq!(size_of::<SAM_VALIDATE_PASSWORD_RESET_INPUT_ARG>(), 104);
    assert_eq!(align_of::<SAM_VALIDATE_PASSWORD_RESET_INPUT_ARG>(), 8);
    assert_eq!(size_of::<SAM_VALIDATE_INPUT_ARG>(), 104);
    assert_eq!(align_of::<SAM_VALIDATE_INPUT_ARG>(), 8);
    assert_eq!(size_of::<SAM_VALIDATE_OUTPUT_ARG>(), 56);
    assert_eq!(align_of::<SAM_VALIDATE_OUTPUT_ARG>(), 8);
    assert_eq!(size_of::<SAM_OPERATION_OBJCHG_INPUT>(), 24);
    assert_eq!(align_of::<SAM_OPERATION_OBJCHG_INPUT>(), 8);
    assert_eq!(size_of::<SAM_OPERATION_OBJCHG_OUTPUT>(), 4);
    assert_eq!(align_of::<SAM_OPERATION_OBJCHG_OUTPUT>(), 4);
    assert_eq!(size_of::<SAM_GENERIC_OPERATION_INPUT>(), 24);
    assert_eq!(align_of::<SAM_GENERIC_OPERATION_INPUT>(), 8);
    assert_eq!(size_of::<SAM_GENERIC_OPERATION_OUTPUT>(), 4);
    assert_eq!(align_of::<SAM_GENERIC_OPERATION_OUTPUT>(), 4);
}
#[test]
fn ntseapi() {
    use ntapi::ntseapi::*;
    assert_eq!(size_of::<TOKEN_SECURITY_ATTRIBUTE_FQBN_VALUE>(), 24);
    assert_eq!(align_of::<TOKEN_SECURITY_ATTRIBUTE_FQBN_VALUE>(), 8);
    assert_eq!(size_of::<TOKEN_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE>(), 16);
    assert_eq!(align_of::<TOKEN_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE>(), 8);
    assert_eq!(size_of::<TOKEN_SECURITY_ATTRIBUTE_V1>(), 40);
    assert_eq!(align_of::<TOKEN_SECURITY_ATTRIBUTE_V1>(), 8);
    assert_eq!(size_of::<TOKEN_SECURITY_ATTRIBUTES_INFORMATION>(), 16);
    assert_eq!(align_of::<TOKEN_SECURITY_ATTRIBUTES_INFORMATION>(), 8);
    assert_eq!(size_of::<TOKEN_PROCESS_TRUST_LEVEL>(), 8);
    assert_eq!(align_of::<TOKEN_PROCESS_TRUST_LEVEL>(), 8);
}
#[test]
fn ntwow64() {
    use ntapi::ntwow64::*;
    assert_eq!(size_of::<RTL_BALANCED_NODE32>(), 12);
    assert_eq!(align_of::<RTL_BALANCED_NODE32>(), 4);
    assert_eq!(FIELD_OFFSET!(RTL_BALANCED_NODE32, u), 0);
    assert_eq!(FIELD_OFFSET!(RTL_BALANCED_NODE32, u.s.Left), 0);
    assert_eq!(FIELD_OFFSET!(RTL_BALANCED_NODE32, u.s.Right), 4);
    assert_eq!(FIELD_OFFSET!(RTL_BALANCED_NODE32, ParentValue), 8);
    assert_eq!(size_of::<RTL_RB_TREE32>(), 8);
    assert_eq!(align_of::<RTL_RB_TREE32>(), 4);
    assert_eq!(FIELD_OFFSET!(RTL_RB_TREE32, Root), 0);
    assert_eq!(FIELD_OFFSET!(RTL_RB_TREE32, Min), 4);
    assert_eq!(size_of::<PEB_LDR_DATA32>(), 48);
    assert_eq!(align_of::<PEB_LDR_DATA32>(), 4);
    assert_eq!(FIELD_OFFSET!(PEB_LDR_DATA32, Length), 0);
    assert_eq!(FIELD_OFFSET!(PEB_LDR_DATA32, Initialized), 4);
    assert_eq!(FIELD_OFFSET!(PEB_LDR_DATA32, SsHandle), 8);
    assert_eq!(FIELD_OFFSET!(PEB_LDR_DATA32, InLoadOrderModuleList), 12);
    assert_eq!(FIELD_OFFSET!(PEB_LDR_DATA32, InMemoryOrderModuleList), 20);
    assert_eq!(
        FIELD_OFFSET!(PEB_LDR_DATA32, InInitializationOrderModuleList),
        28
    );
    assert_eq!(FIELD_OFFSET!(PEB_LDR_DATA32, EntryInProgress), 36);
    assert_eq!(FIELD_OFFSET!(PEB_LDR_DATA32, ShutdownInProgress), 40);
    assert_eq!(FIELD_OFFSET!(PEB_LDR_DATA32, ShutdownThreadId), 44);
    assert_eq!(size_of::<LDR_SERVICE_TAG_RECORD32>(), 8);
    assert_eq!(align_of::<LDR_SERVICE_TAG_RECORD32>(), 4);
    assert_eq!(FIELD_OFFSET!(LDR_SERVICE_TAG_RECORD32, Next), 0);
    assert_eq!(FIELD_OFFSET!(LDR_SERVICE_TAG_RECORD32, ServiceTag), 4);
    assert_eq!(size_of::<LDRP_CSLIST32>(), 4);
    assert_eq!(align_of::<LDRP_CSLIST32>(), 4);
    assert_eq!(FIELD_OFFSET!(LDRP_CSLIST32, Tail), 0);
    assert_eq!(size_of::<LDR_DDAG_NODE32>(), 44);
    assert_eq!(align_of::<LDR_DDAG_NODE32>(), 4);
    assert_eq!(FIELD_OFFSET!(LDR_DDAG_NODE32, Modules), 0);
    assert_eq!(FIELD_OFFSET!(LDR_DDAG_NODE32, ServiceTagList), 8);
    assert_eq!(FIELD_OFFSET!(LDR_DDAG_NODE32, LoadCount), 12);
    assert_eq!(FIELD_OFFSET!(LDR_DDAG_NODE32, LoadWhileUnloadingCount), 16);
    assert_eq!(FIELD_OFFSET!(LDR_DDAG_NODE32, LowestLink), 20);
    assert_eq!(FIELD_OFFSET!(LDR_DDAG_NODE32, u), 24);
    assert_eq!(FIELD_OFFSET!(LDR_DDAG_NODE32, IncomingDependencies), 28);
    assert_eq!(FIELD_OFFSET!(LDR_DDAG_NODE32, State), 32);
    assert_eq!(FIELD_OFFSET!(LDR_DDAG_NODE32, CondenseLink), 36);
    assert_eq!(FIELD_OFFSET!(LDR_DDAG_NODE32, PreorderNumber), 40);
    assert_eq!(size_of::<LDR_DATA_TABLE_ENTRY32>(), 168);
    assert_eq!(align_of::<LDR_DATA_TABLE_ENTRY32>(), 8);
    assert_eq!(FIELD_OFFSET!(LDR_DATA_TABLE_ENTRY32, InLoadOrderLinks), 0);
    assert_eq!(FIELD_OFFSET!(LDR_DATA_TABLE_ENTRY32, InMemoryOrderLinks), 8);
    assert_eq!(
        FIELD_OFFSET!(LDR_DATA_TABLE_ENTRY32, u1.InInitializationOrderLinks),
        16
    );
    assert_eq!(
        FIELD_OFFSET!(LDR_DATA_TABLE_ENTRY32, u1.InProgressLinks),
        16
    );
    assert_eq!(FIELD_OFFSET!(LDR_DATA_TABLE_ENTRY32, DllBase), 24);
    assert_eq!(FIELD_OFFSET!(LDR_DATA_TABLE_ENTRY32, EntryPoint), 28);
    assert_eq!(FIELD_OFFSET!(LDR_DATA_TABLE_ENTRY32, SizeOfImage), 32);
    assert_eq!(FIELD_OFFSET!(LDR_DATA_TABLE_ENTRY32, FullDllName), 36);
    assert_eq!(FIELD_OFFSET!(LDR_DATA_TABLE_ENTRY32, BaseDllName), 44);
    assert_eq!(FIELD_OFFSET!(LDR_DATA_TABLE_ENTRY32, u2.FlagGroup), 52);
    assert_eq!(FIELD_OFFSET!(LDR_DATA_TABLE_ENTRY32, u2.Flags), 52);
    assert_eq!(FIELD_OFFSET!(LDR_DATA_TABLE_ENTRY32, ObsoleteLoadCount), 56);
    assert_eq!(FIELD_OFFSET!(LDR_DATA_TABLE_ENTRY32, TlsIndex), 58);
    assert_eq!(FIELD_OFFSET!(LDR_DATA_TABLE_ENTRY32, HashLinks), 60);
    assert_eq!(FIELD_OFFSET!(LDR_DATA_TABLE_ENTRY32, TimeDateStamp), 68);
    assert_eq!(
        FIELD_OFFSET!(LDR_DATA_TABLE_ENTRY32, EntryPointActivationContext),
        72
    );
    assert_eq!(FIELD_OFFSET!(LDR_DATA_TABLE_ENTRY32, Lock), 76);
    assert_eq!(FIELD_OFFSET!(LDR_DATA_TABLE_ENTRY32, DdagNode), 80);
    assert_eq!(FIELD_OFFSET!(LDR_DATA_TABLE_ENTRY32, NodeModuleLink), 84);
    assert_eq!(FIELD_OFFSET!(LDR_DATA_TABLE_ENTRY32, LoadContext), 92);
    assert_eq!(FIELD_OFFSET!(LDR_DATA_TABLE_ENTRY32, ParentDllBase), 96);
    assert_eq!(
        FIELD_OFFSET!(LDR_DATA_TABLE_ENTRY32, SwitchBackContext),
        100
    );
    assert_eq!(
        FIELD_OFFSET!(LDR_DATA_TABLE_ENTRY32, BaseAddressIndexNode),
        104
    );
    assert_eq!(
        FIELD_OFFSET!(LDR_DATA_TABLE_ENTRY32, MappingInfoIndexNode),
        116
    );
    assert_eq!(FIELD_OFFSET!(LDR_DATA_TABLE_ENTRY32, OriginalBase), 128);
    assert_eq!(FIELD_OFFSET!(LDR_DATA_TABLE_ENTRY32, LoadTime), 136);
    assert_eq!(
        FIELD_OFFSET!(LDR_DATA_TABLE_ENTRY32, BaseNameHashValue),
        144
    );
    assert_eq!(FIELD_OFFSET!(LDR_DATA_TABLE_ENTRY32, LoadReason), 148);
    assert_eq!(
        FIELD_OFFSET!(LDR_DATA_TABLE_ENTRY32, ImplicitPathOptions),
        152
    );
    assert_eq!(FIELD_OFFSET!(LDR_DATA_TABLE_ENTRY32, ReferenceCount), 156);
    assert_eq!(
        FIELD_OFFSET!(LDR_DATA_TABLE_ENTRY32, DependentLoadFlags),
        160
    );
    assert_eq!(FIELD_OFFSET!(LDR_DATA_TABLE_ENTRY32, SigningLevel), 164);
    assert_eq!(size_of::<CURDIR32>(), 12);
    assert_eq!(align_of::<CURDIR32>(), 4);
    assert_eq!(FIELD_OFFSET!(CURDIR32, DosPath), 0);
    assert_eq!(FIELD_OFFSET!(CURDIR32, Handle), 8);
    assert_eq!(size_of::<RTL_DRIVE_LETTER_CURDIR32>(), 16);
    assert_eq!(align_of::<RTL_DRIVE_LETTER_CURDIR32>(), 4);
    assert_eq!(FIELD_OFFSET!(RTL_DRIVE_LETTER_CURDIR32, Flags), 0);
    assert_eq!(FIELD_OFFSET!(RTL_DRIVE_LETTER_CURDIR32, Length), 2);
    assert_eq!(FIELD_OFFSET!(RTL_DRIVE_LETTER_CURDIR32, TimeStamp), 4);
    assert_eq!(FIELD_OFFSET!(RTL_DRIVE_LETTER_CURDIR32, DosPath), 8);
    assert_eq!(size_of::<RTL_USER_PROCESS_PARAMETERS32>(), 676);
    assert_eq!(align_of::<RTL_USER_PROCESS_PARAMETERS32>(), 4);
    assert_eq!(
        FIELD_OFFSET!(RTL_USER_PROCESS_PARAMETERS32, MaximumLength),
        0
    );
    assert_eq!(FIELD_OFFSET!(RTL_USER_PROCESS_PARAMETERS32, Length), 4);
    assert_eq!(FIELD_OFFSET!(RTL_USER_PROCESS_PARAMETERS32, Flags), 8);
    assert_eq!(FIELD_OFFSET!(RTL_USER_PROCESS_PARAMETERS32, DebugFlags), 12);
    assert_eq!(
        FIELD_OFFSET!(RTL_USER_PROCESS_PARAMETERS32, ConsoleHandle),
        16
    );
    assert_eq!(
        FIELD_OFFSET!(RTL_USER_PROCESS_PARAMETERS32, ConsoleFlags),
        20
    );
    assert_eq!(
        FIELD_OFFSET!(RTL_USER_PROCESS_PARAMETERS32, StandardInput),
        24
    );
    assert_eq!(
        FIELD_OFFSET!(RTL_USER_PROCESS_PARAMETERS32, StandardOutput),
        28
    );
    assert_eq!(
        FIELD_OFFSET!(RTL_USER_PROCESS_PARAMETERS32, StandardError),
        32
    );
    assert_eq!(
        FIELD_OFFSET!(RTL_USER_PROCESS_PARAMETERS32, CurrentDirectory),
        36
    );
    assert_eq!(FIELD_OFFSET!(RTL_USER_PROCESS_PARAMETERS32, DllPath), 48);
    assert_eq!(
        FIELD_OFFSET!(RTL_USER_PROCESS_PARAMETERS32, ImagePathName),
        56
    );
    assert_eq!(
        FIELD_OFFSET!(RTL_USER_PROCESS_PARAMETERS32, CommandLine),
        64
    );
    assert_eq!(
        FIELD_OFFSET!(RTL_USER_PROCESS_PARAMETERS32, Environment),
        72
    );
    assert_eq!(FIELD_OFFSET!(RTL_USER_PROCESS_PARAMETERS32, StartingX), 76);
    assert_eq!(FIELD_OFFSET!(RTL_USER_PROCESS_PARAMETERS32, StartingY), 80);
    assert_eq!(FIELD_OFFSET!(RTL_USER_PROCESS_PARAMETERS32, CountX), 84);
    assert_eq!(FIELD_OFFSET!(RTL_USER_PROCESS_PARAMETERS32, CountY), 88);
    assert_eq!(
        FIELD_OFFSET!(RTL_USER_PROCESS_PARAMETERS32, CountCharsX),
        92
    );
    assert_eq!(
        FIELD_OFFSET!(RTL_USER_PROCESS_PARAMETERS32, CountCharsY),
        96
    );
    assert_eq!(
        FIELD_OFFSET!(RTL_USER_PROCESS_PARAMETERS32, FillAttribute),
        100
    );
    assert_eq!(
        FIELD_OFFSET!(RTL_USER_PROCESS_PARAMETERS32, WindowFlags),
        104
    );
    assert_eq!(
        FIELD_OFFSET!(RTL_USER_PROCESS_PARAMETERS32, ShowWindowFlags),
        108
    );
    assert_eq!(
        FIELD_OFFSET!(RTL_USER_PROCESS_PARAMETERS32, WindowTitle),
        112
    );
    assert_eq!(
        FIELD_OFFSET!(RTL_USER_PROCESS_PARAMETERS32, DesktopInfo),
        120
    );
    assert_eq!(FIELD_OFFSET!(RTL_USER_PROCESS_PARAMETERS32, ShellInfo), 128);
    assert_eq!(
        FIELD_OFFSET!(RTL_USER_PROCESS_PARAMETERS32, RuntimeData),
        136
    );
    assert_eq!(
        FIELD_OFFSET!(RTL_USER_PROCESS_PARAMETERS32, CurrentDirectories),
        144
    );
    assert_eq!(
        FIELD_OFFSET!(RTL_USER_PROCESS_PARAMETERS32, EnvironmentSize),
        656
    );
    assert_eq!(
        FIELD_OFFSET!(RTL_USER_PROCESS_PARAMETERS32, EnvironmentVersion),
        660
    );
    assert_eq!(
        FIELD_OFFSET!(RTL_USER_PROCESS_PARAMETERS32, PackageDependencyData),
        664
    );
    assert_eq!(
        FIELD_OFFSET!(RTL_USER_PROCESS_PARAMETERS32, ProcessGroupId),
        668
    );
    assert_eq!(
        FIELD_OFFSET!(RTL_USER_PROCESS_PARAMETERS32, LoaderThreads),
        672
    );
    assert_eq!(size_of::<PEB32>(), 1136);
    assert_eq!(align_of::<PEB32>(), 8);
    assert_eq!(FIELD_OFFSET!(PEB32, InheritedAddressSpace), 0);
    assert_eq!(FIELD_OFFSET!(PEB32, ReadImageFileExecOptions), 1);
    assert_eq!(FIELD_OFFSET!(PEB32, BeingDebugged), 2);
    assert_eq!(FIELD_OFFSET!(PEB32, BitField), 3);
    assert_eq!(FIELD_OFFSET!(PEB32, Mutant), 4);
    assert_eq!(FIELD_OFFSET!(PEB32, ImageBaseAddress), 8);
    assert_eq!(FIELD_OFFSET!(PEB32, Ldr), 12);
    assert_eq!(FIELD_OFFSET!(PEB32, ProcessParameters), 16);
    assert_eq!(FIELD_OFFSET!(PEB32, SubSystemData), 20);
    assert_eq!(FIELD_OFFSET!(PEB32, ProcessHeap), 24);
    assert_eq!(FIELD_OFFSET!(PEB32, FastPebLock), 28);
    assert_eq!(FIELD_OFFSET!(PEB32, AtlThunkSListPtr), 32);
    assert_eq!(FIELD_OFFSET!(PEB32, IFEOKey), 36);
    assert_eq!(FIELD_OFFSET!(PEB32, CrossProcessFlags), 40);
    assert_eq!(FIELD_OFFSET!(PEB32, u), 44);
    assert_eq!(FIELD_OFFSET!(PEB32, SystemReserved), 48);
    assert_eq!(FIELD_OFFSET!(PEB32, AtlThunkSListPtr32), 52);
    assert_eq!(FIELD_OFFSET!(PEB32, ApiSetMap), 56);
    assert_eq!(FIELD_OFFSET!(PEB32, TlsExpansionCounter), 60);
    assert_eq!(FIELD_OFFSET!(PEB32, TlsBitmap), 64);
    assert_eq!(FIELD_OFFSET!(PEB32, TlsBitmapBits), 68);
    assert_eq!(FIELD_OFFSET!(PEB32, ReadOnlySharedMemoryBase), 76);
    assert_eq!(FIELD_OFFSET!(PEB32, HotpatchInformation), 80);
    assert_eq!(FIELD_OFFSET!(PEB32, ReadOnlyStaticServerData), 84);
    assert_eq!(FIELD_OFFSET!(PEB32, AnsiCodePageData), 88);
    assert_eq!(FIELD_OFFSET!(PEB32, OemCodePageData), 92);
    assert_eq!(FIELD_OFFSET!(PEB32, UnicodeCaseTableData), 96);
    assert_eq!(FIELD_OFFSET!(PEB32, NumberOfProcessors), 100);
    assert_eq!(FIELD_OFFSET!(PEB32, NtGlobalFlag), 104);
    assert_eq!(FIELD_OFFSET!(PEB32, CriticalSectionTimeout), 112);
    assert_eq!(FIELD_OFFSET!(PEB32, HeapSegmentReserve), 120);
    assert_eq!(FIELD_OFFSET!(PEB32, HeapSegmentCommit), 124);
    assert_eq!(FIELD_OFFSET!(PEB32, HeapDeCommitTotalFreeThreshold), 128);
    assert_eq!(FIELD_OFFSET!(PEB32, HeapDeCommitFreeBlockThreshold), 132);
    assert_eq!(FIELD_OFFSET!(PEB32, NumberOfHeaps), 136);
    assert_eq!(FIELD_OFFSET!(PEB32, MaximumNumberOfHeaps), 140);
    assert_eq!(FIELD_OFFSET!(PEB32, ProcessHeaps), 144);
    assert_eq!(FIELD_OFFSET!(PEB32, GdiSharedHandleTable), 148);
    assert_eq!(FIELD_OFFSET!(PEB32, ProcessStarterHelper), 152);
    assert_eq!(FIELD_OFFSET!(PEB32, GdiDCAttributeList), 156);
    assert_eq!(FIELD_OFFSET!(PEB32, LoaderLock), 160);
    assert_eq!(FIELD_OFFSET!(PEB32, OSMajorVersion), 164);
    assert_eq!(FIELD_OFFSET!(PEB32, OSMinorVersion), 168);
    assert_eq!(FIELD_OFFSET!(PEB32, OSBuildNumber), 172);
    assert_eq!(FIELD_OFFSET!(PEB32, OSCSDVersion), 174);
    assert_eq!(FIELD_OFFSET!(PEB32, OSPlatformId), 176);
    assert_eq!(FIELD_OFFSET!(PEB32, ImageSubsystem), 180);
    assert_eq!(FIELD_OFFSET!(PEB32, ImageSubsystemMajorVersion), 184);
    assert_eq!(FIELD_OFFSET!(PEB32, ImageSubsystemMinorVersion), 188);
    assert_eq!(FIELD_OFFSET!(PEB32, ActiveProcessAffinityMask), 192);
    assert_eq!(FIELD_OFFSET!(PEB32, GdiHandleBuffer), 196);
    assert_eq!(FIELD_OFFSET!(PEB32, PostProcessInitRoutine), 332);
    assert_eq!(FIELD_OFFSET!(PEB32, TlsExpansionBitmap), 336);
    assert_eq!(FIELD_OFFSET!(PEB32, TlsExpansionBitmapBits), 340);
    assert_eq!(FIELD_OFFSET!(PEB32, SessionId), 468);
    assert_eq!(FIELD_OFFSET!(PEB32, AppCompatFlags), 472);
    assert_eq!(FIELD_OFFSET!(PEB32, AppCompatFlagsUser), 480);
    assert_eq!(FIELD_OFFSET!(PEB32, pShimData), 488);
    assert_eq!(FIELD_OFFSET!(PEB32, AppCompatInfo), 492);
    assert_eq!(FIELD_OFFSET!(PEB32, CSDVersion), 496);
    assert_eq!(FIELD_OFFSET!(PEB32, ActivationContextData), 504);
    assert_eq!(FIELD_OFFSET!(PEB32, ProcessAssemblyStorageMap), 508);
    assert_eq!(
        FIELD_OFFSET!(PEB32, SystemDefaultActivationContextData),
        512
    );
    assert_eq!(FIELD_OFFSET!(PEB32, SystemAssemblyStorageMap), 516);
    assert_eq!(FIELD_OFFSET!(PEB32, MinimumStackCommit), 520);
    assert_eq!(FIELD_OFFSET!(PEB32, FlsCallback), 524);
    assert_eq!(FIELD_OFFSET!(PEB32, FlsListHead), 528);
    assert_eq!(FIELD_OFFSET!(PEB32, FlsBitmap), 536);
    assert_eq!(FIELD_OFFSET!(PEB32, FlsBitmapBits), 540);
    assert_eq!(FIELD_OFFSET!(PEB32, FlsHighIndex), 556);
    assert_eq!(FIELD_OFFSET!(PEB32, WerRegistrationData), 560);
    assert_eq!(FIELD_OFFSET!(PEB32, WerShipAssertPtr), 564);
    assert_eq!(FIELD_OFFSET!(PEB32, pContextData), 568);
    assert_eq!(FIELD_OFFSET!(PEB32, pImageHeaderHash), 572);
    assert_eq!(FIELD_OFFSET!(PEB32, TracingFlags), 576);
    assert_eq!(FIELD_OFFSET!(PEB32, CsrServerReadOnlySharedMemoryBase), 584);
    assert_eq!(FIELD_OFFSET!(PEB32, TppWorkerpListLock), 592);
    assert_eq!(FIELD_OFFSET!(PEB32, TppWorkerpList), 596);
    assert_eq!(FIELD_OFFSET!(PEB32, WaitOnAddressHashTable), 604);
    assert_eq!(FIELD_OFFSET!(PEB32, TelemetryCoverageHeader), 1116);
    assert_eq!(FIELD_OFFSET!(PEB32, CloudFileFlags), 1120);
    assert_eq!(FIELD_OFFSET!(PEB32, CloudFileDiagFlags), 1124);
    assert_eq!(FIELD_OFFSET!(PEB32, PlaceholderCompatibilityMode), 1128);
    assert_eq!(
        FIELD_OFFSET!(PEB32, PlaceholderCompatibilityModeReserved),
        1129
    );
    assert_eq!(size_of::<GDI_TEB_BATCH32>(), 1248);
    assert_eq!(align_of::<GDI_TEB_BATCH32>(), 4);
    assert_eq!(FIELD_OFFSET!(GDI_TEB_BATCH32, Offset), 0);
    assert_eq!(FIELD_OFFSET!(GDI_TEB_BATCH32, HDC), 4);
    assert_eq!(FIELD_OFFSET!(GDI_TEB_BATCH32, Buffer), 8);
    assert_eq!(size_of::<TEB32>(), 4096);
    assert_eq!(align_of::<TEB32>(), 8);
    assert_eq!(FIELD_OFFSET!(TEB32, NtTib), 0);
    assert_eq!(FIELD_OFFSET!(TEB32, EnvironmentPointer), 28);
    assert_eq!(FIELD_OFFSET!(TEB32, ClientId), 32);
    assert_eq!(FIELD_OFFSET!(TEB32, ActiveRpcHandle), 40);
    assert_eq!(FIELD_OFFSET!(TEB32, ThreadLocalStoragePointer), 44);
    assert_eq!(FIELD_OFFSET!(TEB32, ProcessEnvironmentBlock), 48);
    assert_eq!(FIELD_OFFSET!(TEB32, LastErrorValue), 52);
    assert_eq!(FIELD_OFFSET!(TEB32, CountOfOwnedCriticalSections), 56);
    assert_eq!(FIELD_OFFSET!(TEB32, CsrClientThread), 60);
    assert_eq!(FIELD_OFFSET!(TEB32, Win32ThreadInfo), 64);
    assert_eq!(FIELD_OFFSET!(TEB32, User32Reserved), 68);
    assert_eq!(FIELD_OFFSET!(TEB32, UserReserved), 172);
    assert_eq!(FIELD_OFFSET!(TEB32, WOW32Reserved), 192);
    assert_eq!(FIELD_OFFSET!(TEB32, CurrentLocale), 196);
    assert_eq!(FIELD_OFFSET!(TEB32, FpSoftwareStatusRegister), 200);
    assert_eq!(
        FIELD_OFFSET!(TEB32, ReservedForDebuggerInstrumentation),
        204
    );
    assert_eq!(FIELD_OFFSET!(TEB32, SystemReserved1), 268);
    assert_eq!(FIELD_OFFSET!(TEB32, WorkingOnBehalfTicket), 412);
    assert_eq!(FIELD_OFFSET!(TEB32, ExceptionCode), 420);
    assert_eq!(FIELD_OFFSET!(TEB32, ActivationContextStackPointer), 424);
    assert_eq!(FIELD_OFFSET!(TEB32, InstrumentationCallbackSp), 428);
    assert_eq!(FIELD_OFFSET!(TEB32, InstrumentationCallbackPreviousPc), 432);
    assert_eq!(FIELD_OFFSET!(TEB32, InstrumentationCallbackPreviousSp), 436);
    assert_eq!(FIELD_OFFSET!(TEB32, InstrumentationCallbackDisabled), 440);
    assert_eq!(FIELD_OFFSET!(TEB32, SpareBytes), 441);
    assert_eq!(FIELD_OFFSET!(TEB32, TxFsContext), 464);
    assert_eq!(FIELD_OFFSET!(TEB32, GdiTebBatch), 468);
    assert_eq!(FIELD_OFFSET!(TEB32, RealClientId), 1716);
    assert_eq!(FIELD_OFFSET!(TEB32, GdiCachedProcessHandle), 1724);
    assert_eq!(FIELD_OFFSET!(TEB32, GdiClientPID), 1728);
    assert_eq!(FIELD_OFFSET!(TEB32, GdiClientTID), 1732);
    assert_eq!(FIELD_OFFSET!(TEB32, GdiThreadLocalInfo), 1736);
    assert_eq!(FIELD_OFFSET!(TEB32, Win32ClientInfo), 1740);
    assert_eq!(FIELD_OFFSET!(TEB32, glDispatchTable), 1988);
    assert_eq!(FIELD_OFFSET!(TEB32, glReserved1), 2920);
    assert_eq!(FIELD_OFFSET!(TEB32, glReserved2), 3036);
    assert_eq!(FIELD_OFFSET!(TEB32, glSectionInfo), 3040);
    assert_eq!(FIELD_OFFSET!(TEB32, glSection), 3044);
    assert_eq!(FIELD_OFFSET!(TEB32, glTable), 3048);
    assert_eq!(FIELD_OFFSET!(TEB32, glCurrentRC), 3052);
    assert_eq!(FIELD_OFFSET!(TEB32, glContext), 3056);
    assert_eq!(FIELD_OFFSET!(TEB32, LastStatusValue), 3060);
    assert_eq!(FIELD_OFFSET!(TEB32, StaticUnicodeString), 3064);
    assert_eq!(FIELD_OFFSET!(TEB32, StaticUnicodeBuffer), 3072);
    assert_eq!(FIELD_OFFSET!(TEB32, DeallocationStack), 3596);
    assert_eq!(FIELD_OFFSET!(TEB32, TlsSlots), 3600);
    assert_eq!(FIELD_OFFSET!(TEB32, TlsLinks), 3856);
    assert_eq!(FIELD_OFFSET!(TEB32, Vdm), 3864);
    assert_eq!(FIELD_OFFSET!(TEB32, ReservedForNtRpc), 3868);
    assert_eq!(FIELD_OFFSET!(TEB32, DbgSsReserved), 3872);
    assert_eq!(FIELD_OFFSET!(TEB32, HardErrorMode), 3880);
    assert_eq!(FIELD_OFFSET!(TEB32, Instrumentation), 3884);
    assert_eq!(FIELD_OFFSET!(TEB32, ActivityId), 3920);
    assert_eq!(FIELD_OFFSET!(TEB32, SubProcessTag), 3936);
    assert_eq!(FIELD_OFFSET!(TEB32, PerflibData), 3940);
    assert_eq!(FIELD_OFFSET!(TEB32, EtwTraceData), 3944);
    assert_eq!(FIELD_OFFSET!(TEB32, WinSockData), 3948);
    assert_eq!(FIELD_OFFSET!(TEB32, GdiBatchCount), 3952);
    assert_eq!(FIELD_OFFSET!(TEB32, u), 3956);
    assert_eq!(FIELD_OFFSET!(TEB32, u.s.ReservedPad0), 3956);
    assert_eq!(FIELD_OFFSET!(TEB32, u.s.ReservedPad1), 3957);
    assert_eq!(FIELD_OFFSET!(TEB32, u.s.ReservedPad2), 3958);
    assert_eq!(FIELD_OFFSET!(TEB32, u.s.IdealProcessor), 3959);
    assert_eq!(FIELD_OFFSET!(TEB32, GuaranteedStackBytes), 3960);
    assert_eq!(FIELD_OFFSET!(TEB32, ReservedForPerf), 3964);
    assert_eq!(FIELD_OFFSET!(TEB32, ReservedForOle), 3968);
    assert_eq!(FIELD_OFFSET!(TEB32, WaitingOnLoaderLock), 3972);
    assert_eq!(FIELD_OFFSET!(TEB32, SavedPriorityState), 3976);
    assert_eq!(FIELD_OFFSET!(TEB32, ReservedForCodeCoverage), 3980);
    assert_eq!(FIELD_OFFSET!(TEB32, ThreadPoolData), 3984);
    assert_eq!(FIELD_OFFSET!(TEB32, TlsExpansionSlots), 3988);
    assert_eq!(FIELD_OFFSET!(TEB32, MuiGeneration), 3992);
    assert_eq!(FIELD_OFFSET!(TEB32, IsImpersonating), 3996);
    assert_eq!(FIELD_OFFSET!(TEB32, NlsCache), 4000);
    assert_eq!(FIELD_OFFSET!(TEB32, pShimData), 4004);
    assert_eq!(FIELD_OFFSET!(TEB32, HeapVirtualAffinity), 4008);
    assert_eq!(FIELD_OFFSET!(TEB32, LowFragHeapDataSlot), 4010);
    assert_eq!(FIELD_OFFSET!(TEB32, CurrentTransactionHandle), 4012);
    assert_eq!(FIELD_OFFSET!(TEB32, ActiveFrame), 4016);
    assert_eq!(FIELD_OFFSET!(TEB32, FlsData), 4020);
    assert_eq!(FIELD_OFFSET!(TEB32, PreferredLanguages), 4024);
    assert_eq!(FIELD_OFFSET!(TEB32, UserPrefLanguages), 4028);
    assert_eq!(FIELD_OFFSET!(TEB32, MergedPrefLanguages), 4032);
    assert_eq!(FIELD_OFFSET!(TEB32, MuiImpersonation), 4036);
    assert_eq!(FIELD_OFFSET!(TEB32, CrossTebFlags), 4040);
    assert_eq!(FIELD_OFFSET!(TEB32, SameTebFlags), 4042);
    assert_eq!(FIELD_OFFSET!(TEB32, TxnScopeEnterCallback), 4044);
    assert_eq!(FIELD_OFFSET!(TEB32, TxnScopeExitCallback), 4048);
    assert_eq!(FIELD_OFFSET!(TEB32, TxnScopeContext), 4052);
    assert_eq!(FIELD_OFFSET!(TEB32, LockCount), 4056);
    assert_eq!(FIELD_OFFSET!(TEB32, WowTebOffset), 4060);
    assert_eq!(FIELD_OFFSET!(TEB32, ResourceRetValue), 4064);
    assert_eq!(FIELD_OFFSET!(TEB32, ReservedForWdf), 4068);
    assert_eq!(FIELD_OFFSET!(TEB32, ReservedForCrt), 4072);
    assert_eq!(FIELD_OFFSET!(TEB32, EffectiveContainerId), 4080);
}
#[test]
fn subprocesstag() {
    use ntapi::subprocesstag::*;
    assert_eq!(size_of::<TAG_INFO_NAME_FROM_TAG_IN_PARAMS>(), 8);
    assert_eq!(align_of::<TAG_INFO_NAME_FROM_TAG_IN_PARAMS>(), 4);
    assert_eq!(size_of::<TAG_INFO_NAME_FROM_TAG_OUT_PARAMS>(), 16);
    assert_eq!(align_of::<TAG_INFO_NAME_FROM_TAG_OUT_PARAMS>(), 8);
    assert_eq!(size_of::<TAG_INFO_NAME_FROM_TAG>(), 24);
    assert_eq!(align_of::<TAG_INFO_NAME_FROM_TAG>(), 8);
    assert_eq!(size_of::<TAG_INFO_NAMES_REFERENCING_MODULE_IN_PARAMS>(), 16);
    assert_eq!(align_of::<TAG_INFO_NAMES_REFERENCING_MODULE_IN_PARAMS>(), 8);
    assert_eq!(
        size_of::<TAG_INFO_NAMES_REFERENCING_MODULE_OUT_PARAMS>(),
        16
    );
    assert_eq!(
        align_of::<TAG_INFO_NAMES_REFERENCING_MODULE_OUT_PARAMS>(),
        8
    );
    assert_eq!(size_of::<TAG_INFO_NAMES_REFERENCING_MODULE>(), 32);
    assert_eq!(align_of::<TAG_INFO_NAMES_REFERENCING_MODULE>(), 8);
    assert_eq!(size_of::<TAG_INFO_NAME_TAG_MAPPING_IN_PARAMS>(), 4);
    assert_eq!(align_of::<TAG_INFO_NAME_TAG_MAPPING_IN_PARAMS>(), 4);
    assert_eq!(size_of::<TAG_INFO_NAME_TAG_MAPPING_ELEMENT>(), 24);
    assert_eq!(align_of::<TAG_INFO_NAME_TAG_MAPPING_ELEMENT>(), 8);
    assert_eq!(size_of::<TAG_INFO_NAME_TAG_MAPPING_OUT_PARAMS>(), 16);
    assert_eq!(align_of::<TAG_INFO_NAME_TAG_MAPPING_OUT_PARAMS>(), 8);
    assert_eq!(size_of::<TAG_INFO_NAME_TAG_MAPPING>(), 16);
    assert_eq!(align_of::<TAG_INFO_NAME_TAG_MAPPING>(), 8);
}
#[test]
fn winsta() {
    use ntapi::winsta::*;
    assert_eq!(size_of::<VARDATA_WIRE>(), 4);
    assert_eq!(align_of::<VARDATA_WIRE>(), 2);
    assert_eq!(FIELD_OFFSET!(VARDATA_WIRE, Size), 0);
    assert_eq!(FIELD_OFFSET!(VARDATA_WIRE, Offset), 2);
    assert_eq!(size_of::<SESSIONIDW>(), 76);
    assert_eq!(align_of::<SESSIONIDW>(), 4);
    assert_eq!(FIELD_OFFSET!(SESSIONIDW, u), 0);
    assert_eq!(FIELD_OFFSET!(SESSIONIDW, WinStationName), 4);
    assert_eq!(FIELD_OFFSET!(SESSIONIDW, State), 72);
    assert_eq!(size_of::<WINSTATIONCREATE>(), 8);
    assert_eq!(align_of::<WINSTATIONCREATE>(), 4);
    assert_eq!(FIELD_OFFSET!(WINSTATIONCREATE, Bitfields), 0);
    assert_eq!(FIELD_OFFSET!(WINSTATIONCREATE, MaxInstanceCount), 4);
    assert_eq!(size_of::<WINSTACONFIGWIRE>(), 134);
    assert_eq!(align_of::<WINSTACONFIGWIRE>(), 2);
    assert_eq!(FIELD_OFFSET!(WINSTACONFIGWIRE, Comment), 0);
    assert_eq!(FIELD_OFFSET!(WINSTACONFIGWIRE, OEMId), 122);
    assert_eq!(FIELD_OFFSET!(WINSTACONFIGWIRE, UserConfig), 126);
    assert_eq!(FIELD_OFFSET!(WINSTACONFIGWIRE, NewFields), 130);
    assert_eq!(size_of::<USERCONFIG>(), 2536);
    assert_eq!(align_of::<USERCONFIG>(), 4);
    assert_eq!(FIELD_OFFSET!(USERCONFIG, Bitfields), 0);
    assert_eq!(FIELD_OFFSET!(USERCONFIG, Bitfields2), 4);
    assert_eq!(FIELD_OFFSET!(USERCONFIG, UserName), 8);
    assert_eq!(FIELD_OFFSET!(USERCONFIG, Domain), 50);
    assert_eq!(FIELD_OFFSET!(USERCONFIG, Password), 86);
    assert_eq!(FIELD_OFFSET!(USERCONFIG, WorkDirectory), 116);
    assert_eq!(FIELD_OFFSET!(USERCONFIG, InitialProgram), 630);
    assert_eq!(FIELD_OFFSET!(USERCONFIG, CallbackNumber), 1144);
    assert_eq!(FIELD_OFFSET!(USERCONFIG, Callback), 1248);
    assert_eq!(FIELD_OFFSET!(USERCONFIG, Shadow), 1252);
    assert_eq!(FIELD_OFFSET!(USERCONFIG, MaxConnectionTime), 1256);
    assert_eq!(FIELD_OFFSET!(USERCONFIG, MaxDisconnectionTime), 1260);
    assert_eq!(FIELD_OFFSET!(USERCONFIG, MaxIdleTime), 1264);
    assert_eq!(FIELD_OFFSET!(USERCONFIG, KeyboardLayout), 1268);
    assert_eq!(FIELD_OFFSET!(USERCONFIG, MinEncryptionLevel), 1272);
    assert_eq!(FIELD_OFFSET!(USERCONFIG, NWLogonServer), 1274);
    assert_eq!(FIELD_OFFSET!(USERCONFIG, PublishedName), 1370);
    assert_eq!(FIELD_OFFSET!(USERCONFIG, WFProfilePath), 1500);
    assert_eq!(FIELD_OFFSET!(USERCONFIG, WFHomeDir), 2014);
    assert_eq!(FIELD_OFFSET!(USERCONFIG, WFHomeDirDrive), 2528);
    assert_eq!(size_of::<NETWORKCONFIG>(), 268);
    assert_eq!(align_of::<NETWORKCONFIG>(), 4);
    assert_eq!(FIELD_OFFSET!(NETWORKCONFIG, LanAdapter), 0);
    assert_eq!(FIELD_OFFSET!(NETWORKCONFIG, NetworkName), 4);
    assert_eq!(FIELD_OFFSET!(NETWORKCONFIG, Flags), 264);
    assert_eq!(size_of::<FLOWCONTROLCONFIG>(), 20);
    assert_eq!(align_of::<FLOWCONTROLCONFIG>(), 4);
    assert_eq!(FIELD_OFFSET!(FLOWCONTROLCONFIG, Bitfields), 0);
    assert_eq!(FIELD_OFFSET!(FLOWCONTROLCONFIG, XonChar), 4);
    assert_eq!(FIELD_OFFSET!(FLOWCONTROLCONFIG, XoffChar), 5);
    assert_eq!(FIELD_OFFSET!(FLOWCONTROLCONFIG, Type), 8);
    assert_eq!(FIELD_OFFSET!(FLOWCONTROLCONFIG, HardwareReceive), 12);
    assert_eq!(FIELD_OFFSET!(FLOWCONTROLCONFIG, HardwareTransmit), 16);
    assert_eq!(size_of::<CONNECTCONFIG>(), 8);
    assert_eq!(align_of::<CONNECTCONFIG>(), 4);
    assert_eq!(FIELD_OFFSET!(CONNECTCONFIG, Type), 0);
    assert_eq!(FIELD_OFFSET!(CONNECTCONFIG, Bitfields), 4);
    assert_eq!(size_of::<ASYNCCONFIG>(), 564);
    assert_eq!(align_of::<ASYNCCONFIG>(), 4);
    assert_eq!(FIELD_OFFSET!(ASYNCCONFIG, DeviceName), 0);
    assert_eq!(FIELD_OFFSET!(ASYNCCONFIG, ModemName), 258);
    assert_eq!(FIELD_OFFSET!(ASYNCCONFIG, BaudRate), 516);
    assert_eq!(FIELD_OFFSET!(ASYNCCONFIG, Parity), 520);
    assert_eq!(FIELD_OFFSET!(ASYNCCONFIG, StopBits), 524);
    assert_eq!(FIELD_OFFSET!(ASYNCCONFIG, ByteSize), 528);
    assert_eq!(FIELD_OFFSET!(ASYNCCONFIG, Bitfields), 532);
    assert_eq!(FIELD_OFFSET!(ASYNCCONFIG, FlowControl), 536);
    assert_eq!(FIELD_OFFSET!(ASYNCCONFIG, Connect), 556);
    assert_eq!(size_of::<NASICONFIG>(), 308);
    assert_eq!(align_of::<NASICONFIG>(), 2);
    assert_eq!(FIELD_OFFSET!(NASICONFIG, SpecificName), 0);
    assert_eq!(FIELD_OFFSET!(NASICONFIG, UserName), 30);
    assert_eq!(FIELD_OFFSET!(NASICONFIG, PassWord), 126);
    assert_eq!(FIELD_OFFSET!(NASICONFIG, SessionName), 176);
    assert_eq!(FIELD_OFFSET!(NASICONFIG, FileServer), 210);
    assert_eq!(FIELD_OFFSET!(NASICONFIG, GlobalSession), 306);
    assert_eq!(size_of::<OEMTDCONFIG>(), 268);
    assert_eq!(align_of::<OEMTDCONFIG>(), 4);
    assert_eq!(FIELD_OFFSET!(OEMTDCONFIG, Adapter), 0);
    assert_eq!(FIELD_OFFSET!(OEMTDCONFIG, DeviceName), 4);
    assert_eq!(FIELD_OFFSET!(OEMTDCONFIG, Flags), 264);
    assert_eq!(size_of::<PDPARAMS>(), 568);
    assert_eq!(align_of::<PDPARAMS>(), 4);
    assert_eq!(FIELD_OFFSET!(PDPARAMS, SdClass), 0);
    assert_eq!(FIELD_OFFSET!(PDPARAMS, u), 4);
    assert_eq!(size_of::<WDCONFIG>(), 300);
    assert_eq!(align_of::<WDCONFIG>(), 4);
    assert_eq!(FIELD_OFFSET!(WDCONFIG, WdName), 0);
    assert_eq!(FIELD_OFFSET!(WDCONFIG, WdDLL), 66);
    assert_eq!(FIELD_OFFSET!(WDCONFIG, WsxDLL), 132);
    assert_eq!(FIELD_OFFSET!(WDCONFIG, WdFlag), 200);
    assert_eq!(FIELD_OFFSET!(WDCONFIG, WdInputBufferLength), 204);
    assert_eq!(FIELD_OFFSET!(WDCONFIG, CfgDLL), 208);
    assert_eq!(FIELD_OFFSET!(WDCONFIG, WdPrefix), 274);
    assert_eq!(size_of::<PDCONFIG2>(), 168);
    assert_eq!(align_of::<PDCONFIG2>(), 4);
    assert_eq!(FIELD_OFFSET!(PDCONFIG2, PdName), 0);
    assert_eq!(FIELD_OFFSET!(PDCONFIG2, SdClass), 68);
    assert_eq!(FIELD_OFFSET!(PDCONFIG2, PdDLL), 72);
    assert_eq!(FIELD_OFFSET!(PDCONFIG2, PdFlag), 140);
    assert_eq!(FIELD_OFFSET!(PDCONFIG2, OutBufLength), 144);
    assert_eq!(FIELD_OFFSET!(PDCONFIG2, OutBufCount), 148);
    assert_eq!(FIELD_OFFSET!(PDCONFIG2, OutBufDelay), 152);
    assert_eq!(FIELD_OFFSET!(PDCONFIG2, InteractiveDelay), 156);
    assert_eq!(FIELD_OFFSET!(PDCONFIG2, PortNumber), 160);
    assert_eq!(FIELD_OFFSET!(PDCONFIG2, KeepAliveTimeout), 164);
    assert_eq!(size_of::<WINSTATIONCLIENT>(), 2296);
    assert_eq!(align_of::<WINSTATIONCLIENT>(), 4);
    assert_eq!(FIELD_OFFSET!(WINSTATIONCLIENT, Bitfields), 0);
    assert_eq!(FIELD_OFFSET!(WINSTATIONCLIENT, ClientName), 4);
    assert_eq!(FIELD_OFFSET!(WINSTATIONCLIENT, Domain), 46);
    assert_eq!(FIELD_OFFSET!(WINSTATIONCLIENT, UserName), 82);
    assert_eq!(FIELD_OFFSET!(WINSTATIONCLIENT, Password), 124);
    assert_eq!(FIELD_OFFSET!(WINSTATIONCLIENT, WorkDirectory), 154);
    assert_eq!(FIELD_OFFSET!(WINSTATIONCLIENT, InitialProgram), 668);
    assert_eq!(FIELD_OFFSET!(WINSTATIONCLIENT, SerialNumber), 1184);
    assert_eq!(FIELD_OFFSET!(WINSTATIONCLIENT, EncryptionLevel), 1188);
    assert_eq!(FIELD_OFFSET!(WINSTATIONCLIENT, ClientAddressFamily), 1192);
    assert_eq!(FIELD_OFFSET!(WINSTATIONCLIENT, ClientAddress), 1196);
    assert_eq!(FIELD_OFFSET!(WINSTATIONCLIENT, HRes), 1258);
    assert_eq!(FIELD_OFFSET!(WINSTATIONCLIENT, VRes), 1260);
    assert_eq!(FIELD_OFFSET!(WINSTATIONCLIENT, ColorDepth), 1262);
    assert_eq!(FIELD_OFFSET!(WINSTATIONCLIENT, ProtocolType), 1264);
    assert_eq!(FIELD_OFFSET!(WINSTATIONCLIENT, KeyboardLayout), 1268);
    assert_eq!(FIELD_OFFSET!(WINSTATIONCLIENT, KeyboardType), 1272);
    assert_eq!(FIELD_OFFSET!(WINSTATIONCLIENT, KeyboardSubType), 1276);
    assert_eq!(FIELD_OFFSET!(WINSTATIONCLIENT, KeyboardFunctionKey), 1280);
    assert_eq!(FIELD_OFFSET!(WINSTATIONCLIENT, ImeFileName), 1284);
    assert_eq!(FIELD_OFFSET!(WINSTATIONCLIENT, ClientDirectory), 1350);
    assert_eq!(FIELD_OFFSET!(WINSTATIONCLIENT, ClientLicense), 1864);
    assert_eq!(FIELD_OFFSET!(WINSTATIONCLIENT, ClientModem), 1930);
    assert_eq!(FIELD_OFFSET!(WINSTATIONCLIENT, ClientBuildNumber), 2012);
    assert_eq!(FIELD_OFFSET!(WINSTATIONCLIENT, ClientHardwareId), 2016);
    assert_eq!(FIELD_OFFSET!(WINSTATIONCLIENT, ClientProductId), 2020);
    assert_eq!(FIELD_OFFSET!(WINSTATIONCLIENT, OutBufCountHost), 2022);
    assert_eq!(FIELD_OFFSET!(WINSTATIONCLIENT, OutBufCountClient), 2024);
    assert_eq!(FIELD_OFFSET!(WINSTATIONCLIENT, OutBufLength), 2026);
    assert_eq!(FIELD_OFFSET!(WINSTATIONCLIENT, AudioDriverName), 2028);
    assert_eq!(FIELD_OFFSET!(WINSTATIONCLIENT, ClientTimeZone), 2048);
    assert_eq!(FIELD_OFFSET!(WINSTATIONCLIENT, ClientSessionId), 2220);
    assert_eq!(FIELD_OFFSET!(WINSTATIONCLIENT, ClientDigProductId), 2224);
    assert_eq!(FIELD_OFFSET!(WINSTATIONCLIENT, PerformanceFlags), 2288);
    assert_eq!(FIELD_OFFSET!(WINSTATIONCLIENT, ActiveInputLocale), 2292);
    assert_eq!(size_of::<TSHARE_COUNTERS>(), 4);
    assert_eq!(align_of::<TSHARE_COUNTERS>(), 4);
    assert_eq!(FIELD_OFFSET!(TSHARE_COUNTERS, Reserved), 0);
    assert_eq!(size_of::<PROTOCOLCOUNTERS>(), 460);
    assert_eq!(align_of::<PROTOCOLCOUNTERS>(), 4);
    assert_eq!(FIELD_OFFSET!(PROTOCOLCOUNTERS, WdBytes), 0);
    assert_eq!(FIELD_OFFSET!(PROTOCOLCOUNTERS, WdFrames), 4);
    assert_eq!(FIELD_OFFSET!(PROTOCOLCOUNTERS, WaitForOutBuf), 8);
    assert_eq!(FIELD_OFFSET!(PROTOCOLCOUNTERS, Frames), 12);
    assert_eq!(FIELD_OFFSET!(PROTOCOLCOUNTERS, Bytes), 16);
    assert_eq!(FIELD_OFFSET!(PROTOCOLCOUNTERS, CompressedBytes), 20);
    assert_eq!(FIELD_OFFSET!(PROTOCOLCOUNTERS, CompressFlushes), 24);
    assert_eq!(FIELD_OFFSET!(PROTOCOLCOUNTERS, Errors), 28);
    assert_eq!(FIELD_OFFSET!(PROTOCOLCOUNTERS, Timeouts), 32);
    assert_eq!(FIELD_OFFSET!(PROTOCOLCOUNTERS, AsyncFramingError), 36);
    assert_eq!(FIELD_OFFSET!(PROTOCOLCOUNTERS, AsyncOverrunError), 40);
    assert_eq!(FIELD_OFFSET!(PROTOCOLCOUNTERS, AsyncOverflowError), 44);
    assert_eq!(FIELD_OFFSET!(PROTOCOLCOUNTERS, AsyncParityError), 48);
    assert_eq!(FIELD_OFFSET!(PROTOCOLCOUNTERS, TdErrors), 52);
    assert_eq!(FIELD_OFFSET!(PROTOCOLCOUNTERS, ProtocolType), 56);
    assert_eq!(FIELD_OFFSET!(PROTOCOLCOUNTERS, Length), 58);
    assert_eq!(FIELD_OFFSET!(PROTOCOLCOUNTERS, Specific.TShareCounters), 60);
    assert_eq!(FIELD_OFFSET!(PROTOCOLCOUNTERS, Specific.Reserved), 60);
    assert_eq!(FIELD_OFFSET!(PROTOCOLCOUNTERS, Specific), 60);
    assert_eq!(size_of::<THINWIRECACHE>(), 8);
    assert_eq!(align_of::<THINWIRECACHE>(), 4);
    assert_eq!(FIELD_OFFSET!(THINWIRECACHE, CacheReads), 0);
    assert_eq!(FIELD_OFFSET!(THINWIRECACHE, CacheHits), 4);
    assert_eq!(size_of::<RESERVED_CACHE>(), 32);
    assert_eq!(align_of::<RESERVED_CACHE>(), 4);
    assert_eq!(FIELD_OFFSET!(RESERVED_CACHE, ThinWireCache), 0);
    assert_eq!(size_of::<TSHARE_CACHE>(), 4);
    assert_eq!(align_of::<TSHARE_CACHE>(), 4);
    assert_eq!(FIELD_OFFSET!(TSHARE_CACHE, Reserved), 0);
    assert_eq!(size_of::<CACHE_STATISTICS>(), 84);
    assert_eq!(align_of::<CACHE_STATISTICS>(), 4);
    assert_eq!(FIELD_OFFSET!(CACHE_STATISTICS, ProtocolType), 0);
    assert_eq!(FIELD_OFFSET!(CACHE_STATISTICS, Length), 2);
    assert_eq!(
        FIELD_OFFSET!(CACHE_STATISTICS, Specific.ReservedCacheStats),
        4
    );
    assert_eq!(
        FIELD_OFFSET!(CACHE_STATISTICS, Specific.TShareCacheStats),
        4
    );
    assert_eq!(FIELD_OFFSET!(CACHE_STATISTICS, Specific.Reserved), 4);
    assert_eq!(FIELD_OFFSET!(CACHE_STATISTICS, Specific), 4);
    assert_eq!(size_of::<PROTOCOLSTATUS>(), 1012);
    assert_eq!(align_of::<PROTOCOLSTATUS>(), 4);
    assert_eq!(FIELD_OFFSET!(PROTOCOLSTATUS, Output), 0);
    assert_eq!(FIELD_OFFSET!(PROTOCOLSTATUS, Input), 460);
    assert_eq!(FIELD_OFFSET!(PROTOCOLSTATUS, Cache), 920);
    assert_eq!(FIELD_OFFSET!(PROTOCOLSTATUS, AsyncSignal), 1004);
    assert_eq!(FIELD_OFFSET!(PROTOCOLSTATUS, AsyncSignalMask), 1008);
    assert_eq!(size_of::<WINSTATIONINFORMATION>(), 1216);
    assert_eq!(align_of::<WINSTATIONINFORMATION>(), 8);
    assert_eq!(FIELD_OFFSET!(WINSTATIONINFORMATION, ConnectState), 0);
    assert_eq!(FIELD_OFFSET!(WINSTATIONINFORMATION, WinStationName), 4);
    assert_eq!(FIELD_OFFSET!(WINSTATIONINFORMATION, LogonId), 72);
    assert_eq!(FIELD_OFFSET!(WINSTATIONINFORMATION, ConnectTime), 80);
    assert_eq!(FIELD_OFFSET!(WINSTATIONINFORMATION, DisconnectTime), 88);
    assert_eq!(FIELD_OFFSET!(WINSTATIONINFORMATION, LastInputTime), 96);
    assert_eq!(FIELD_OFFSET!(WINSTATIONINFORMATION, LogonTime), 104);
    assert_eq!(FIELD_OFFSET!(WINSTATIONINFORMATION, Status), 112);
    assert_eq!(FIELD_OFFSET!(WINSTATIONINFORMATION, Domain), 1124);
    assert_eq!(FIELD_OFFSET!(WINSTATIONINFORMATION, UserName), 1160);
    assert_eq!(FIELD_OFFSET!(WINSTATIONINFORMATION, CurrentTime), 1208);
    assert_eq!(size_of::<WINSTATIONUSERTOKEN>(), 24);
    assert_eq!(align_of::<WINSTATIONUSERTOKEN>(), 8);
    assert_eq!(FIELD_OFFSET!(WINSTATIONUSERTOKEN, ProcessId), 0);
    assert_eq!(FIELD_OFFSET!(WINSTATIONUSERTOKEN, ThreadId), 8);
    assert_eq!(FIELD_OFFSET!(WINSTATIONUSERTOKEN, UserToken), 16);
    assert_eq!(size_of::<WINSTATIONVIDEODATA>(), 6);
    assert_eq!(align_of::<WINSTATIONVIDEODATA>(), 2);
    assert_eq!(FIELD_OFFSET!(WINSTATIONVIDEODATA, HResolution), 0);
    assert_eq!(FIELD_OFFSET!(WINSTATIONVIDEODATA, VResolution), 2);
    assert_eq!(FIELD_OFFSET!(WINSTATIONVIDEODATA, fColorDepth), 4);
    assert_eq!(size_of::<CDCONFIG>(), 140);
    assert_eq!(align_of::<CDCONFIG>(), 4);
    assert_eq!(FIELD_OFFSET!(CDCONFIG, CdClass), 0);
    assert_eq!(FIELD_OFFSET!(CDCONFIG, CdName), 4);
    assert_eq!(FIELD_OFFSET!(CDCONFIG, CdDLL), 70);
    assert_eq!(FIELD_OFFSET!(CDCONFIG, CdFlag), 136);
    assert_eq!(size_of::<WINSTATIONCLIENTDATA>(), 9);
    assert_eq!(align_of::<WINSTATIONCLIENTDATA>(), 1);
    assert_eq!(FIELD_OFFSET!(WINSTATIONCLIENTDATA, DataName), 0);
    assert_eq!(FIELD_OFFSET!(WINSTATIONCLIENTDATA, fUnicodeData), 8);
    assert_eq!(size_of::<WINSTATIONLOADINDICATORDATA>(), 72);
    assert_eq!(align_of::<WINSTATIONLOADINDICATORDATA>(), 8);
    assert_eq!(
        FIELD_OFFSET!(WINSTATIONLOADINDICATORDATA, RemainingSessionCapacity),
        0
    );
    assert_eq!(FIELD_OFFSET!(WINSTATIONLOADINDICATORDATA, LoadFactor), 4);
    assert_eq!(FIELD_OFFSET!(WINSTATIONLOADINDICATORDATA, TotalSessions), 8);
    assert_eq!(
        FIELD_OFFSET!(WINSTATIONLOADINDICATORDATA, DisconnectedSessions),
        12
    );
    assert_eq!(FIELD_OFFSET!(WINSTATIONLOADINDICATORDATA, IdleCPU), 16);
    assert_eq!(FIELD_OFFSET!(WINSTATIONLOADINDICATORDATA, TotalCPU), 24);
    assert_eq!(
        FIELD_OFFSET!(WINSTATIONLOADINDICATORDATA, RawSessionCapacity),
        32
    );
    assert_eq!(FIELD_OFFSET!(WINSTATIONLOADINDICATORDATA, reserved), 36);
    assert_eq!(size_of::<WINSTATIONSHADOW>(), 16);
    assert_eq!(align_of::<WINSTATIONSHADOW>(), 4);
    assert_eq!(FIELD_OFFSET!(WINSTATIONSHADOW, ShadowState), 0);
    assert_eq!(FIELD_OFFSET!(WINSTATIONSHADOW, ShadowClass), 4);
    assert_eq!(FIELD_OFFSET!(WINSTATIONSHADOW, SessionId), 8);
    assert_eq!(FIELD_OFFSET!(WINSTATIONSHADOW, ProtocolType), 12);
    assert_eq!(size_of::<WINSTATIONPRODID>(), 204);
    assert_eq!(align_of::<WINSTATIONPRODID>(), 4);
    assert_eq!(FIELD_OFFSET!(WINSTATIONPRODID, DigProductId), 0);
    assert_eq!(FIELD_OFFSET!(WINSTATIONPRODID, ClientDigProductId), 64);
    assert_eq!(FIELD_OFFSET!(WINSTATIONPRODID, OuterMostDigProductId), 128);
    assert_eq!(FIELD_OFFSET!(WINSTATIONPRODID, CurrentSessionId), 192);
    assert_eq!(FIELD_OFFSET!(WINSTATIONPRODID, ClientSessionId), 196);
    assert_eq!(FIELD_OFFSET!(WINSTATIONPRODID, OuterMostSessionId), 200);
    assert_eq!(size_of::<WINSTATIONREMOTEADDRESS>(), 32);
    assert_eq!(align_of::<WINSTATIONREMOTEADDRESS>(), 4);
    assert_eq!(FIELD_OFFSET!(WINSTATIONREMOTEADDRESS, sin_family), 0);
    assert_eq!(FIELD_OFFSET!(WINSTATIONREMOTEADDRESS, u.ipv4.sin_port), 4);
    assert_eq!(FIELD_OFFSET!(WINSTATIONREMOTEADDRESS, u.ipv4.sin_addr), 8);
    assert_eq!(FIELD_OFFSET!(WINSTATIONREMOTEADDRESS, u.ipv4.sin_zero), 12);
    assert_eq!(FIELD_OFFSET!(WINSTATIONREMOTEADDRESS, u.ipv6.sin6_port), 4);
    assert_eq!(
        FIELD_OFFSET!(WINSTATIONREMOTEADDRESS, u.ipv6.sin6_flowinfo),
        8
    );
    assert_eq!(FIELD_OFFSET!(WINSTATIONREMOTEADDRESS, u.ipv6.sin6_addr), 12);
    assert_eq!(
        FIELD_OFFSET!(WINSTATIONREMOTEADDRESS, u.ipv6.sin6_scope_id),
        28
    );
    assert_eq!(size_of::<WINSTATIONINFORMATIONEX_LEVEL1>(), 1216);
    assert_eq!(align_of::<WINSTATIONINFORMATIONEX_LEVEL1>(), 8);
    assert_eq!(FIELD_OFFSET!(WINSTATIONINFORMATIONEX_LEVEL1, SessionId), 0);
    assert_eq!(
        FIELD_OFFSET!(WINSTATIONINFORMATIONEX_LEVEL1, SessionState),
        4
    );
    assert_eq!(
        FIELD_OFFSET!(WINSTATIONINFORMATIONEX_LEVEL1, SessionFlags),
        8
    );
    assert_eq!(
        FIELD_OFFSET!(WINSTATIONINFORMATIONEX_LEVEL1, WinStationName),
        12
    );
    assert_eq!(FIELD_OFFSET!(WINSTATIONINFORMATIONEX_LEVEL1, UserName), 78);
    assert_eq!(
        FIELD_OFFSET!(WINSTATIONINFORMATIONEX_LEVEL1, DomainName),
        120
    );
    assert_eq!(
        FIELD_OFFSET!(WINSTATIONINFORMATIONEX_LEVEL1, LogonTime),
        160
    );
    assert_eq!(
        FIELD_OFFSET!(WINSTATIONINFORMATIONEX_LEVEL1, ConnectTime),
        168
    );
    assert_eq!(
        FIELD_OFFSET!(WINSTATIONINFORMATIONEX_LEVEL1, DisconnectTime),
        176
    );
    assert_eq!(
        FIELD_OFFSET!(WINSTATIONINFORMATIONEX_LEVEL1, LastInputTime),
        184
    );
    assert_eq!(
        FIELD_OFFSET!(WINSTATIONINFORMATIONEX_LEVEL1, CurrentTime),
        192
    );
    assert_eq!(
        FIELD_OFFSET!(WINSTATIONINFORMATIONEX_LEVEL1, ProtocolStatus),
        200
    );
    assert_eq!(size_of::<WINSTATIONINFORMATIONEX_LEVEL2>(), 2240);
    assert_eq!(align_of::<WINSTATIONINFORMATIONEX_LEVEL2>(), 8);
    assert_eq!(FIELD_OFFSET!(WINSTATIONINFORMATIONEX_LEVEL2, SessionId), 0);
    assert_eq!(
        FIELD_OFFSET!(WINSTATIONINFORMATIONEX_LEVEL2, SessionState),
        4
    );
    assert_eq!(
        FIELD_OFFSET!(WINSTATIONINFORMATIONEX_LEVEL2, SessionFlags),
        8
    );
    assert_eq!(
        FIELD_OFFSET!(WINSTATIONINFORMATIONEX_LEVEL2, WinStationName),
        12
    );
    assert_eq!(
        FIELD_OFFSET!(WINSTATIONINFORMATIONEX_LEVEL2, SamCompatibleUserName),
        78
    );
    assert_eq!(
        FIELD_OFFSET!(WINSTATIONINFORMATIONEX_LEVEL2, SamCompatibleDomainName),
        120
    );
    assert_eq!(
        FIELD_OFFSET!(WINSTATIONINFORMATIONEX_LEVEL2, LogonTime),
        160
    );
    assert_eq!(
        FIELD_OFFSET!(WINSTATIONINFORMATIONEX_LEVEL2, ConnectTime),
        168
    );
    assert_eq!(
        FIELD_OFFSET!(WINSTATIONINFORMATIONEX_LEVEL2, DisconnectTime),
        176
    );
    assert_eq!(
        FIELD_OFFSET!(WINSTATIONINFORMATIONEX_LEVEL2, LastInputTime),
        184
    );
    assert_eq!(
        FIELD_OFFSET!(WINSTATIONINFORMATIONEX_LEVEL2, CurrentTime),
        192
    );
    assert_eq!(
        FIELD_OFFSET!(WINSTATIONINFORMATIONEX_LEVEL2, ProtocolStatus),
        200
    );
    assert_eq!(
        FIELD_OFFSET!(WINSTATIONINFORMATIONEX_LEVEL2, UserName),
        1212
    );
    assert_eq!(
        FIELD_OFFSET!(WINSTATIONINFORMATIONEX_LEVEL2, DomainName),
        1726
    );
    assert_eq!(size_of::<WINSTATIONINFORMATIONEX>(), 2248);
    assert_eq!(align_of::<WINSTATIONINFORMATIONEX>(), 8);
    assert_eq!(FIELD_OFFSET!(WINSTATIONINFORMATIONEX, Level), 0);
    assert_eq!(FIELD_OFFSET!(WINSTATIONINFORMATIONEX, Data), 8);
    assert_eq!(size_of::<TS_PROCESS_INFORMATION_NT4>(), 24);
    assert_eq!(align_of::<TS_PROCESS_INFORMATION_NT4>(), 8);
    assert_eq!(FIELD_OFFSET!(TS_PROCESS_INFORMATION_NT4, MagicNumber), 0);
    assert_eq!(FIELD_OFFSET!(TS_PROCESS_INFORMATION_NT4, LogonId), 4);
    assert_eq!(FIELD_OFFSET!(TS_PROCESS_INFORMATION_NT4, ProcessSid), 8);
    assert_eq!(FIELD_OFFSET!(TS_PROCESS_INFORMATION_NT4, Pad), 16);
    assert_eq!(size_of::<TS_SYS_PROCESS_INFORMATION>(), 184);
    assert_eq!(align_of::<TS_SYS_PROCESS_INFORMATION>(), 8);
    assert_eq!(
        FIELD_OFFSET!(TS_SYS_PROCESS_INFORMATION, NextEntryOffset),
        0
    );
    assert_eq!(
        FIELD_OFFSET!(TS_SYS_PROCESS_INFORMATION, NumberOfThreads),
        4
    );
    assert_eq!(FIELD_OFFSET!(TS_SYS_PROCESS_INFORMATION, SpareLi1), 8);
    assert_eq!(FIELD_OFFSET!(TS_SYS_PROCESS_INFORMATION, SpareLi2), 16);
    assert_eq!(FIELD_OFFSET!(TS_SYS_PROCESS_INFORMATION, SpareLi3), 24);
    assert_eq!(FIELD_OFFSET!(TS_SYS_PROCESS_INFORMATION, CreateTime), 32);
    assert_eq!(FIELD_OFFSET!(TS_SYS_PROCESS_INFORMATION, UserTime), 40);
    assert_eq!(FIELD_OFFSET!(TS_SYS_PROCESS_INFORMATION, KernelTime), 48);
    assert_eq!(FIELD_OFFSET!(TS_SYS_PROCESS_INFORMATION, ImageName), 56);
    assert_eq!(FIELD_OFFSET!(TS_SYS_PROCESS_INFORMATION, BasePriority), 72);
    assert_eq!(
        FIELD_OFFSET!(TS_SYS_PROCESS_INFORMATION, UniqueProcessId),
        76
    );
    assert_eq!(
        FIELD_OFFSET!(TS_SYS_PROCESS_INFORMATION, InheritedFromUniqueProcessId),
        80
    );
    assert_eq!(FIELD_OFFSET!(TS_SYS_PROCESS_INFORMATION, HandleCount), 84);
    assert_eq!(FIELD_OFFSET!(TS_SYS_PROCESS_INFORMATION, SessionId), 88);
    assert_eq!(FIELD_OFFSET!(TS_SYS_PROCESS_INFORMATION, SpareUl3), 92);
    assert_eq!(
        FIELD_OFFSET!(TS_SYS_PROCESS_INFORMATION, PeakVirtualSize),
        96
    );
    assert_eq!(FIELD_OFFSET!(TS_SYS_PROCESS_INFORMATION, VirtualSize), 104);
    assert_eq!(
        FIELD_OFFSET!(TS_SYS_PROCESS_INFORMATION, PageFaultCount),
        112
    );
    assert_eq!(
        FIELD_OFFSET!(TS_SYS_PROCESS_INFORMATION, PeakWorkingSetSize),
        116
    );
    assert_eq!(
        FIELD_OFFSET!(TS_SYS_PROCESS_INFORMATION, WorkingSetSize),
        120
    );
    assert_eq!(
        FIELD_OFFSET!(TS_SYS_PROCESS_INFORMATION, QuotaPeakPagedPoolUsage),
        128
    );
    assert_eq!(
        FIELD_OFFSET!(TS_SYS_PROCESS_INFORMATION, QuotaPagedPoolUsage),
        136
    );
    assert_eq!(
        FIELD_OFFSET!(TS_SYS_PROCESS_INFORMATION, QuotaPeakNonPagedPoolUsage),
        144
    );
    assert_eq!(
        FIELD_OFFSET!(TS_SYS_PROCESS_INFORMATION, QuotaNonPagedPoolUsage),
        152
    );
    assert_eq!(
        FIELD_OFFSET!(TS_SYS_PROCESS_INFORMATION, PagefileUsage),
        160
    );
    assert_eq!(
        FIELD_OFFSET!(TS_SYS_PROCESS_INFORMATION, PeakPagefileUsage),
        168
    );
    assert_eq!(
        FIELD_OFFSET!(TS_SYS_PROCESS_INFORMATION, PrivatePageCount),
        176
    );
    assert_eq!(size_of::<TS_ALL_PROCESSES_INFO>(), 24);
    assert_eq!(align_of::<TS_ALL_PROCESSES_INFO>(), 8);
    assert_eq!(FIELD_OFFSET!(TS_ALL_PROCESSES_INFO, pTsProcessInfo), 0);
    assert_eq!(FIELD_OFFSET!(TS_ALL_PROCESSES_INFO, SizeOfSid), 8);
    assert_eq!(FIELD_OFFSET!(TS_ALL_PROCESSES_INFO, pSid), 16);
    assert_eq!(size_of::<TS_COUNTER_HEADER>(), 8);
    assert_eq!(align_of::<TS_COUNTER_HEADER>(), 4);
    assert_eq!(FIELD_OFFSET!(TS_COUNTER_HEADER, dwCounterID), 0);
    assert_eq!(FIELD_OFFSET!(TS_COUNTER_HEADER, bResult), 4);
    assert_eq!(size_of::<TS_COUNTER>(), 24);
    assert_eq!(align_of::<TS_COUNTER>(), 8);
    assert_eq!(FIELD_OFFSET!(TS_COUNTER, CounterHead), 0);
    assert_eq!(FIELD_OFFSET!(TS_COUNTER, dwValue), 8);
    assert_eq!(FIELD_OFFSET!(TS_COUNTER, StartTime), 16);
}
