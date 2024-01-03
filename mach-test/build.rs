extern crate ctest;

#[derive(Eq, Ord, PartialEq, PartialOrd, Copy, Clone, Debug)]
struct Xcode(pub u32, pub u32);

impl Xcode {
    fn version() -> Xcode {
        use std::process::Command;
        let out = Command::new("/usr/bin/xcodebuild")
            .arg("-version")
            .output()
            .expect("failed to execute xcodebuild");
        let stdout = ::std::str::from_utf8(&out.stdout).expect("couldn't parse stdout as UTF8");
        let stderr = ::std::str::from_utf8(&out.stderr).expect("couldn't parse stderr as UTF8");

        if !out.status.success() {
            eprintln!("stdout: {}", stdout);
            eprintln!("stderr: {}", stderr);
            panic!("xcodebuild -version failed");
        }

        // xcodebuild -version output looks like:
        //
        // Xcode 9.2
        // Build version 9C40b
        let mut iter = stdout
            .split(|c: char| c.is_whitespace() || c == '.')
            .skip(1)
            .map(|c| {
                c.parse()
                    .expect("failed to parse Xcode version into number")
            });
        let major: u32 = iter.next().expect("failed to parse Xcode major version");
        let minor: u32 = iter.next().expect("failed to parse Xcode minor version");

        Xcode(major, minor)
    }
}

fn main() {
    let xcode = Xcode::version();
    // kept on purpose for debugging:
    // println!("cargo:warning=\"Xcode version: {:?}\"", xcode);

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
        .header("mach/memory_object_types.h")
        .header("mach/message.h");

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

    // The below doesn't exist in Xcode 14:
    if xcode < Xcode(14, 0) {
        cfg.header("mach/lock_set.h");
    }

    cfg.skip_struct(move |s| {
        match s {
            // TODO: this type is a bitfield and must be verified by hand
            "mach_msg_type_descriptor_t" |

            // TODO: this type is a bitfield and must be verified by hand
            "mach_msg_port_descriptor_t" |

            // TODO: this type is a bitfield and must be verified by hand
            "mach_msg_ool_descriptor_t" |

            // TODO: this type is a bitfield and must be verified by hand
            "mach_msg_ool_ports_descriptor_t" |

            // FIXME: this type is not exposed in /usr/include/mach
            // but seems to be exposed in
            // SDKs/MacOSX.sdk/System/Library/Frameworks/Kernel.framework/Versions/A/Headers/mach
            "ipc_port" => true,

            // FIXME: Changed in XCode 11, figure out what's changed.
            "vm_region_submap_info_64" if xcode >= Xcode(11, 0) => true,

            _ => false,
        }
    });

    cfg.skip_type(move |s| {
        match s {
            // FIXME: this type is not exposed in /usr/include/mach
            // but seems to be exposed in
            // SDKs/MacOSX.sdk/System/Library/Frameworks/Kernel.framework/Versions/A/Headers/mach
            "ipc_port_t" => true,

            // FIXME: Changed in XCode 11, see `vm_region_submap_info_data_64`'s comment.
            "vm_region_submap_info_data_64_t" if xcode >= Xcode(11, 0) => true,

            // FIXME: Unavailable since Xcode 14:
            "io_master_t" if xcode >= Xcode(14, 0) => true,

            _ => false,
        }
    });

    cfg.skip_fn(move |s| {
        // FIXME: The return type of these functions are different in Xcode 13 or higher.
        if s.starts_with("semaphore") {
            return true;
        }
        match s {
            // mac_task_self and current_tasl are not functions, but macro that map to the
            // mask_task_self_ static variable:
            "mach_task_self" | "current_task" => true,

            _ => false,
        }
    });

    cfg.skip_const(move |s| match s {
        "MACH_RCV_NOTIFY" | "MACH_RCV_OVERWRITE" if xcode <= Xcode(11, 0) => true,

        // FIXME: Somehow it fails, like:
        // bad VM_PROT_NO_CHANGE value at byte 0: rust: 8 (0x8) != c 0 (0x0)
        // bad VM_PROT_NO_CHANGE value at byte 3: rust: 0 (0x0) != c 1 (0x1)
        "VM_PROT_NO_CHANGE" if xcode >= Xcode(13, 0) => true,

        // FIXME: Unavailable since Xcode 14:
        "EXC_CORPSE_VARIANT_BIT" if xcode >= Xcode(14, 0) => true,
        _ => false,
    });

    cfg.fn_cname(|rust, _link_name| match rust {
        v => v.to_string(),
    });

    cfg.skip_signededness(|c| {
        // signededness test does not make sense for these:
        match c {
            // struct types:
            "mach_timespec_t" |
            "ipc_port_t"  |
            "vm_statistics_data_t" |
            "fsid_t" |
            "fsobj_id_t" |
            "dyld_kernel_image_info_t" |
            "dyld_kernel_process_info_t" |

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

    cfg.type_name(move |ty, is_struct, is_union| match ty {
        t if is_union => format!("union {}", t),
        t if t.ends_with("_t") => t.to_string(),
        t if is_struct => format!("struct {}", t),
        t => t.to_string(),
    });

    cfg.skip_roundtrip(move |s| match s {
        // FIXME: TODO
        "name_t" | "uuid_t" | "vm_region_info_data_t" | "cmd_t" | "mach_vm_read_entry_t" => true,
        _ => false,
    });

    // Include the directory where the header files are defined
    cfg.include("/usr/include");

    // Generate the tests, passing the path to the `*-sys` library as well as
    // the module to generate.
    cfg.generate("../src/lib.rs", "all.rs");
}
