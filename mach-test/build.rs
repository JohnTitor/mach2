fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../src");
    println!("cargo:rerun-if-env-changed=SDKROOT");
    println!("cargo:rerun-if-env-changed=DEVELOPER_DIR");

    let mut cfg = ctest::TestGenerator::new();

    cfg.flag("-Wno-unknown-warning-option");

    cfg.header("mach-o/loader.h").header("mach-o/dyld.h");

    // Include the header files where the C APIs are defined
    cfg.header("mach/boolean.h")
        .header("bootstrap.h")
        .header("mach/bootstrap.h")
        .header("mach/clock.h")
        .header("mach/clock_priv.h")
        .header("mach/clock_reply.h")
        .header("mach/clock_types.h")
        .header("mach/dyld_kernel.h");

    cfg.header("mach/error.h")
        .header("mach/exc.h")
        .header("mach/exception.h")
        .header("mach/exception_types.h")
        .header("mach/host_info.h")
        .header("mach/host_notify.h")
        .header("mach/host_priv.h")
        .header("mach/host_reboot.h")
        .header("mach/host_security.h")
        .header("mach/host_special_ports.h")
        .header("mach/kern_return.h")
        .header("mach/kmod.h")
        .header("mach/mach.h")
        .header("mach/mach_error.h")
        .header("mach/mach_host.h")
        .header("mach/mach_init.h")
        .header("mach/mach_interface.h")
        .header("mach/mach_param.h")
        .header("mach/mach_port.h")
        .header("mach/mach_syscalls.h")
        .header("mach/mach_time.h")
        .header("mach/mach_traps.h")
        .header("mach/mach_types.h")
        .header("mach/mach_vm.h")
        .header("mach/mach_voucher.h")
        .header("mach/mach_voucher_types.h")
        .header("mach/machine.h")
        .header("mach/memory_entry.h")
        .header("mach/memory_object_types.h")
        .header("mach/message.h")
        .header("mach/mig.h");

    cfg.header("mach/ndr.h")
        .header("mach/notify.h")
        .header("mach/policy.h")
        .header("mach/port.h")
        .header("mach/port_obj.h")
        .header("mach/processor.h")
        .header("mach/processor_info.h")
        .header("mach/processor_set.h")
        .header("mach/rpc.h")
        .header("mach/sdt.h")
        .header("mach/semaphore.h")
        .header("mach/shared_region.h")
        .header("mach/std_types.h")
        .header("mach/sync.h")
        .header("mach/sync_policy.h")
        .header("mach/task.h")
        .header("mach/task_info.h")
        .header("mach/task_policy.h")
        .header("mach/task_special_ports.h")
        .header("mach/thread_act.h")
        .header("mach/thread_info.h")
        .header("mach/thread_policy.h")
        .header("mach/thread_special_ports.h");

    cfg.header("mach/thread_state.h")
        .header("mach/thread_status.h")
        .header("mach/thread_switch.h")
        .header("mach/time_value.h")
        .header("mach/vm_attributes.h")
        .header("mach/vm_behavior.h")
        .header("mach/vm_inherit.h")
        .header("mach/vm_map.h")
        .header("mach/vm_page_size.h")
        .header("mach/vm_param.h")
        .header("mach/vm_prot.h")
        .header("mach/vm_purgable.h")
        .header("mach/vm_region.h")
        .header("mach/vm_statistics.h")
        .header("mach/vm_sync.h")
        .header("mach/vm_task.h")
        .header("mach/vm_types.h");

    cfg.skip_struct(move |s| {
        match s.ident() {
            // TODO: these types are bitfields and must be verified by hand
            "mach_msg_type_descriptor_t"
            | "mach_msg_port_descriptor_t"
            | "mach_msg_ool_descriptor_t"
            | "mach_msg_ool_ports_descriptor_t" => true,
            _ => false,
        }
    });

    cfg.skip_fn(move |s| {
        match s.ident() {
            // These macros map to the mach_task_self_ static variable.
            "mach_task_self" | "current_task" => true,

            _ => false,
        }
    });

    cfg.skip_signededness(|c| {
        // signededness test does not make sense for these:
        match c {
            // struct types:
            "mach_timespec_t" |
            "vm_statistics_data_t" |
            "fsid_t" |
            "fsobj_id_t" |
            "dyld_kernel_image_info_t" |
            "dyld_kernel_process_info_t" |
            "host_cpu_counters_info_data_t" |
            "processor_cpu_counters_info_data_t" |
            "task_vm_shared_cache_info_data_t" |
            "task_shared_region_info_data_t" |
            "vm_page_info_extended_data_t" |

            // array types:
            "vm_region_info_data_t" |
            "mach_vm_read_entry_t" |
            "name_t" |
            "cmd_t" |
            "name_array_t" |
            "bootstrap_status_array_t" |
            "bootstrap_property_array_t" |
            "bool_array_t" |
            "uuid_t" |

            // pointer types:
            "clock_attr_t" |
            "host_info64_t" |
            "host_cpu_counters_info_t" |
            "processor_info_array_t" |
            "processor_cpu_counters_info_t" |
            "task_vm_shared_cache_info_t" |
            "task_shared_region_info_t" |
            "vm_page_info_extended_t" |
            "dyld_kernel_image_info_array_t" |
            "memory_object_fault_info_t" |
            "exception_data_t" |
            "exception_flavor_array_t" |
            "exception_mask_array_t" |
            "exception_port_arrary_t" |
            "exception_handler_array_t" |
            "thread_state_t" |
            "thread_array_t" |
            "thread_port_array_t" |
            "thread_act_array_t" |
            "thread_act_port_array_t" |
            "ledger_array_t" |
            "ledger_port_array_t" |
            "mach_exception_data_t" |
            "exception_behavior_array_t" |
            "exception_port_array_t" |
            "task_info_t" |
            "task_array_t" |
            "task_port_array_t" |
            "processor_array_t" |
            "processor_port_array_t" |
            "processor_set_array_t" |
            "processor_set_name_array_t" |
            "processor_set_name_port_array_t" |
            "vm_region_t" |
            "vm_region_info_t" |
            "vm_region_info_64_t" |
            "vm_region_recurse_info_t" |
            "vm_region_recurse_info_64_t" |
            "vm_region_basic_info_64_t" |
            "vm_region_basic_info_t" |
            "vm_region_basic_info_data_t" |
            "vm_region_basic_info_data_64_t" |
            "vm_region_extended_info_t" |
            "vm_region_extended_info_data_t" |
            "vm_region_top_info_t" |
            "vm_region_top_info_data_t" |
            "vm_region_submap_info_t" |
            "vm_region_submap_info_64_t" |
            "vm_region_submap_info_data_t" |
            "vm_region_submap_info_data_64_t" |
            "vm_region_submap_short_info_64_t" |
            "vm_region_submap_short_info_data_64_t" |
            "vm_page_info_t" |
            "vm_page_info_basic_t" |
            "vm_page_info_basic_data_t" |
            "vm_statistics_t"
                => true,
            _ => false,
        }
    });

    cfg.rename_struct_ty(move |ty| match ty {
        t if t.ends_with("_t") => Some(t.to_string()),
        t @ "gpu_energy_data" => Some(t.to_string()),
        t => Some(format!("struct {}", t)),
    });

    cfg.skip_roundtrip(move |s| match s {
        // FIXME: TODO
        "name_t" | "uuid_t" | "vm_region_info_data_t" | "cmd_t" | "mach_vm_read_entry_t" => true,
        _ => false,
    });

    // Generate the tests, passing the path to the `*-sys` library as well as
    // the module to generate.
    ctest::generate_test(&mut cfg, "../src/lib.rs", "all.rs").unwrap();
}
