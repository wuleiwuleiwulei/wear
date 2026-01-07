use ylong_num_cpus::get_cpu_num;

#[cfg(target_os = "linux")]
#[test]
fn sdv_linux_test() {
    use ylong_num_cpus::sys;

    let cpus = get_cpu_num();
    assert!(cpus > 0);
    let cpus = sys::get_cpu_num_configured();
    assert!(cpus > 0);
}

#[cfg(target_os = "windows")]
#[test]
fn sdv_windows_test() {
    let cpus = get_cpu_num();
    assert!(cpus > 0);
}
