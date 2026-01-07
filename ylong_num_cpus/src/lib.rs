use std::os::raw::c_long;
#[cfg(target_os = "linux")]
#[path = "sys/linux.rs"]
pub mod sys;

#[cfg(target_os = "windows")]
#[path = "sys/windows.rs"]
pub mod sys;

/// get_cpu_num函数是对外接口，会对应不同操作系统自动调用底层函数，
/// Linux 下对应 sysconf 函数，默认获取可用状态的 cpu 核数，
/// Windows 下对应 GetSystemInfo 函数，默认获取可用状态的 cpu 核数。
/// # Example
///
/// ```rust
/// use ylong_num_cpus;
///
/// let cpus = ylong_num_cpus::get_cpu_num();
/// ```
pub fn get_cpu_num() -> c_long {
    sys::get_cpu_num_online()
}
