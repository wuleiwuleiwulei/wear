use ylong_sysinfo::NetworksInfo;

fn main() {
    let networks_info = NetworksInfo::new().unwrap();
    for network in networks_info.iter() {
        println!("{:?}", network.0);
    }

    #[cfg(target_os = "windows")]
    let network_name = "以太网";
    #[cfg(target_os = "linux")]
    let network_name = "eth0";

    let mut iter = networks_info.iter();
    let network = iter
        .find(|(name, ip_addr)| name.contains(network_name) && !ip_addr.is_loopback())
        .unwrap();
    println!("{:?}", network.0);
}
