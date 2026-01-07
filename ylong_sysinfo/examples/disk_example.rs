use ylong_sysinfo::DisksInfo;

fn main() {
    let disks_info = DisksInfo::new().unwrap();
    let mut mounts = disks_info.disks();
    mounts.sort_by_key(|m2| std::cmp::Reverse(m2.mount_point().len()));
    println!("{mounts:?}");

    #[cfg(target_os = "windows")]
    let path = "C:\\";
    #[cfg(target_os = "linux")]
    let path = "/dev/mqueue";

    let first_matched = mounts.iter().find(|m| path.starts_with(m.mount_point()));
    println!("{first_matched:?}");

    let disk = disks_info.disk_at(path).unwrap();
    println!("{disk:?}");
    println!("{:?}", disk.total_space());
}
