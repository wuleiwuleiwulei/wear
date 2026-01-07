use ylong_sysinfo::MemoryInfo;

fn main() {
    let memory_info = MemoryInfo::new().unwrap();
    let total_memory = memory_info.total_phys();
    let avail_memory = memory_info.avail_phys();
    println!("{total_memory:?} {avail_memory:?}");
}
