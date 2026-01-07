use libc::{sysconf, _SC_NPROCESSORS_CONF, _SC_NPROCESSORS_ONLN};
use std::os::raw::c_long;

/// Linux 平台下可调用底层的函数，获取目前启用状态的 cpu 核数
///
/// # Example 2
///
/// ```rust
/// use ylong_num_cpus;
///
/// #[cfg(target_os = "linux")]
/// let cpus = ylong_num_cpus::sys::get_cpu_num_online();
///```
pub fn get_cpu_num_online() -> c_long {
    unsafe { sysconf(_SC_NPROCESSORS_ONLN) }
}

/// Linux 平台下可调用底层的函数，获取 cpu 核数，包括被禁用状态的 cpu
///
/// # Example 2
///
/// ```rust
/// use ylong_num_cpus;
///
/// #[cfg(target_os = "linux")]
/// let cpus = ylong_num_cpus::sys::get_cpu_num_configured();
///```
pub fn get_cpu_num_configured() -> c_long {
    unsafe { sysconf(_SC_NPROCESSORS_CONF) }
}
