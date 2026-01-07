use std::os::raw::c_long;

// Windows 平台下获取当前可用 cpu 核数
pub(crate) fn get_cpu_num_online() -> c_long {
    #[repr(C)]
    struct SYSTEM_INFO {
        w_processor_architecture: u16,
        w_reserved: u16,
        dw_page_size: u32,
        lp_minimum_application_address: *mut u8,
        lp_maximum_application_address: *mut u8,
        dw_active_processor_mask: *mut u8,
        dw_number_of_processors: u32,
        dw_processor_type: u32,
        dw_allocation_granularity: u32,
        w_processor_level: u16,
        w_processor_revision: u16,
    }

    extern "system" {
        fn GetSystemInfo(lpSystemInfo: *mut SYSTEM_INFO);
    }

    unsafe {
        let mut sysinfo: SYSTEM_INFO = std::mem::zeroed();
        GetSystemInfo(&mut sysinfo);
        sysinfo.dw_number_of_processors as c_long
    }
}
