use ylong_sysinfo::ProcessesInfo;

fn main() {
    let processes_info = ProcessesInfo::new().unwrap();
    let iter = processes_info
        .processes()
        .values()
        .filter(|val| val.name().contains("chrome"));
    for process in iter {
        println!("{process:?}");
    }

    println!("---------------");

    let processes_info = ProcessesInfo::new().unwrap();
    let iter = processes_info.processes_by_name("chrome");
    for process in iter {
        println!("{process:?}");
    }

    let processes_info = ProcessesInfo::new().unwrap();
    println!("{:?}", processes_info.processes().keys().len());
}
