use std::thread::sleep;
use std::time::Duration;
use ylong_sysinfo::CpusInfo;

fn main() {
    let mut cpus_info = CpusInfo::new().unwrap();
    sleep(Duration::new(1, 0));
    cpus_info.update_all().unwrap();
    println!(
        "global_cpu_usage: {:?}%",
        cpus_info.global_cpu().usage() as f32 / 100.0
    );
    for cpu in cpus_info.cpus() {
        println!("cpu_usage: {:?}%", cpu.usage() as f32 / 100.0)
    }
}
