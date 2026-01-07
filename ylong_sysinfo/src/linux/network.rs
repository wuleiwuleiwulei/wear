use crate::error::Error;
use crate::error::InnerError::ParseIntError;
use libc::{
    freeifaddrs, getnameinfo, ifaddrs, sockaddr, sockaddr_in, sockaddr_in6, AF_INET, AF_INET6,
    AF_PACKET, IFF_ALLMULTI, IFF_AUTOMEDIA, IFF_BROADCAST, IFF_DEBUG, IFF_DYNAMIC, IFF_LOOPBACK,
    IFF_MASTER, IFF_MULTICAST, IFF_NOARP, IFF_NOTRAILERS, IFF_POINTOPOINT, IFF_PORTSEL,
    IFF_PROMISC, IFF_RUNNING, IFF_SLAVE, IFF_UP, NI_MAXHOST, NI_NUMERICHOST,
};
use std::collections::hash_map::{Iter, IterMut};
use std::collections::HashMap;
use std::ffi::CStr;
use std::fs::File;
use std::io::{Error as OsError, Read};
use std::mem;
use std::os::raw::c_char;

pub struct IfaddrIterator {
    base: *mut ifaddrs,
    next: *mut ifaddrs,
}

impl Drop for IfaddrIterator {
    fn drop(&mut self) {
        unsafe {
            freeifaddrs(self.base);
        }
    }
}

impl Iterator for IfaddrIterator {
    type Item = *mut ifaddrs;

    fn next(&mut self) -> Option<Self::Item> {
        match unsafe { self.next.as_ref() } {
            Some(ifaddr) => {
                self.next = ifaddr.ifa_next;
                Some(ifaddr as *const _ as *mut _)
            }
            None => None,
        }
    }
}

fn get_ifaddrs() -> Result<IfaddrIterator, Error> {
    let mut addrs: *mut libc::ifaddrs = std::ptr::null_mut();
    unsafe {
        let result = libc::getifaddrs(&mut addrs);
        if result == 0 {
            return Ok(IfaddrIterator {
                base: addrs,
                next: addrs,
            });
        }
        Err(Error::Os(OsError::last_os_error()))
    }
}

fn read_devs_info(name: &str) -> Result<(u64, u64, u64, u64), Error> {
    fn read_file(path: String) -> Result<u64, Error> {
        let mut string = String::new();
        if let Err(e) = File::open(path).and_then(|mut f| f.read_to_string(&mut string)) {
            return Err(Error::Os(e));
        }
        string
            .trim_end()
            .parse::<u64>()
            .map_err(|_| Error::Internal(ParseIntError))
    }

    let rx_bytes_path = format!("/sys/class/net/{name}/statistics/rx_bytes");
    let tx_bytes_path = format!("/sys/class/net/{name}/statistics/tx_bytes");
    let rx_packets_path = format!("/sys/class/net/{name}/statistics/rx_packets");
    let tx_packets_path = format!("/sys/class/net/{name}/statistics/tx_packets");

    let rx_bytes = read_file(rx_bytes_path)?;
    let tx_bytes = read_file(tx_bytes_path)?;
    let rx_packets = read_file(rx_packets_path)?;
    let tx_packets = read_file(tx_packets_path)?;

    Ok((rx_bytes, tx_bytes, rx_packets, tx_packets))
}

pub struct NetworksInfo {
    networks: HashMap<String, Network>,
}

impl NetworksInfo {
    /// 创建 Networks 基本信息结构
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::NetworksInfo;
    ///
    /// let networks = NetworksInfo::new();
    /// assert!(networks.is_ok());
    /// ```
    pub fn new() -> Result<Self, Error> {
        let mut networks = HashMap::new();
        let ifaddrs = get_ifaddrs()?;

        for ifaddr in ifaddrs {
            unsafe {
                // 此处 unwrap 必定成功
                let name = CStr::from_ptr((*ifaddr).ifa_name).to_str().unwrap();
                if !networks.contains_key(name) {
                    let mut network = Network::new(name.to_string());
                    network._update_flow()?;
                    networks.insert(name.to_string(), network);
                }
                networks.get_mut(name).unwrap()._update_baseinfo(ifaddr);
            }
        }

        Ok(Self { networks })
    }

    /// 获取所有网络信息的非可变引用
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::NetworksInfo;
    ///
    /// let networks = NetworksInfo::new().unwrap();
    /// let iter = networks.iter();
    /// for info in iter {
    ///     println!("{:?}", info.0);
    /// }
    /// ```
    pub fn iter(&self) -> Iter<String, Network> {
        self.networks.iter()
    }

    /// 获取所有网络信息的可变引用
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::NetworksInfo;
    ///
    /// let mut networks = NetworksInfo::new().unwrap();
    /// let mut iter_mut = networks.iter_mut();
    /// for info in iter_mut {
    ///     info.1.update().unwrap();
    /// }
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<String, Network> {
        self.networks.iter_mut()
    }

    /// 根据名称找到对应的网络信息引用
    ///
    /// # Examples
    ///
    /// ```not work
    /// use ylong_sysinfo::NetworksInfo;
    ///
    /// let networks = NetworksInfo::new().unwrap();
    /// let network = networks.get("lo");
    /// assert!(network.is_some());
    /// ```
    pub fn get(&self, name: &str) -> Option<&Network> {
        self.networks.get(name)
    }

    /// 根据名称找到对应的网络信息可变引用
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::NetworksInfo;
    ///
    /// let mut networks = NetworksInfo::new().unwrap();
    /// let network = networks.get_mut("lo");
    /// assert!(network.is_some());
    /// ```
    pub fn get_mut(&mut self, name: &str) -> Option<&mut Network> {
        self.networks.get_mut(name)
    }
}

pub struct Network {
    // 当前网络设备名字
    name: String,

    // IPv4 地址
    ipv4: String,

    // IPv6 地址
    ipv6: String,

    // MAC 地址
    mac: String,

    // Network 状态
    state: State,

    // 接收数据包数
    rx_packets: u64,

    // 发送数据包数
    tx_packets: u64,

    // 接收字节数
    rx_bytes: u64,

    // 发送字节数
    tx_bytes: u64,
}

pub struct State {
    inner: i32,
}

impl State {
    fn new(inner: i32) -> Self {
        Self { inner }
    }

    // 判断当前状态是否为 UP 状态
    fn is_up(&self) -> bool {
        (self.inner & IFF_UP) != 0
    }

    // 判断当前状态是否为 BROADCAST 状态
    fn is_broadcast(&self) -> bool {
        (self.inner & IFF_BROADCAST) != 0
    }

    // 判断当前状态是否为 DEBUG 状态
    fn is_debug(&self) -> bool {
        (self.inner & IFF_DEBUG) != 0
    }

    // 判断当前状态是否为 LOOPBACK 状态
    fn is_loopback(&self) -> bool {
        (self.inner & IFF_LOOPBACK) != 0
    }

    // 判断当前状态是否为 POINTOPOINT 状态
    fn is_point_to_point(&self) -> bool {
        (self.inner & IFF_POINTOPOINT) != 0
    }

    // 判断当前状态是否为 NOTRAILERS 状态
    fn is_no_trailers(&self) -> bool {
        (self.inner & IFF_NOTRAILERS) != 0
    }

    // 判断当前状态是否为 RUNNING 状态
    fn is_running(&self) -> bool {
        (self.inner & IFF_RUNNING) != 0
    }

    // 判断当前状态是否为 NOARP 状态
    fn is_no_arp(&self) -> bool {
        (self.inner & IFF_NOARP) != 0
    }

    // 判断当前状态是否为 PROMISC 状态
    fn is_promisc(&self) -> bool {
        (self.inner & IFF_PROMISC) != 0
    }

    // 判断当前状态是否为 ALLMULTI 状态
    fn is_all_multi(&self) -> bool {
        (self.inner & IFF_ALLMULTI) != 0
    }

    // 判断当前状态是否为 MASTER 状态
    fn is_master(&self) -> bool {
        (self.inner & IFF_MASTER) != 0
    }

    // 判断当前状态是否为 SLAVE 状态
    fn is_slave(&self) -> bool {
        (self.inner & IFF_SLAVE) != 0
    }

    // 判断当前状态是否为 MULTICAST 状态
    fn is_multicast(&self) -> bool {
        (self.inner & IFF_MULTICAST) != 0
    }

    // 判断当前状态是否为 PORTSEL 状态
    fn is_port_sel(&self) -> bool {
        (self.inner & IFF_PORTSEL) != 0
    }

    // 判断当前状态是否为 AUTOMEDIA 状态
    fn is_auto_media(&self) -> bool {
        (self.inner & IFF_AUTOMEDIA) != 0
    }

    // 判断当前状态是否为 DYNAMIC 状态
    fn is_dynamic(&self) -> bool {
        (self.inner & IFF_DYNAMIC) != 0
    }
}

impl Network {
    fn new(name: String) -> Self {
        Self {
            name,
            ipv4: "".to_string(),
            ipv6: "".to_string(),
            mac: "".to_string(),
            state: State::new(0),
            rx_bytes: 0,
            tx_bytes: 0,
            rx_packets: 0,
            tx_packets: 0,
        }
    }

    fn _update_flow(&mut self) -> Result<(), Error> {
        let (rx_bytes, tx_bytes, rx_packets, tx_packets) = read_devs_info(&self.name)?;
        self.rx_bytes = rx_bytes;
        self.tx_bytes = tx_bytes;
        self.rx_packets = rx_packets;
        self.tx_packets = tx_packets;
        Ok(())
    }

    // Safety: buf 不可为空指针，该函数仅内部使用，可保证指针不为空
    unsafe fn _update_baseinfo(&mut self, buf: *mut ifaddrs) {
        if !(*buf).ifa_addr.is_null() {
            let sa_family = (*(*buf).ifa_addr).sa_family;
            if sa_family == AF_INET as u16 || sa_family == AF_INET6 as u16 {
                let size = if sa_family == AF_INET as u16 {
                    mem::size_of::<sockaddr_in>() as u32
                } else {
                    mem::size_of::<sockaddr_in6>() as u32
                };
                let mut data = [0 as c_char; NI_MAXHOST as usize];
                getnameinfo(
                    (*buf).ifa_addr,
                    size,
                    &mut data as *mut c_char,
                    NI_MAXHOST,
                    std::ptr::null_mut(),
                    0,
                    NI_NUMERICHOST,
                );
                let host = data.split(|num| *num == 0).next().unwrap();
                let host = std::str::from_utf8(mem::transmute::<&[c_char], &[u8]>(host))
                    .unwrap()
                    .to_string();

                if sa_family == AF_INET as u16 {
                    self.ipv4 = host;
                } else {
                    self.ipv6 = host;
                }
            } else if sa_family == AF_PACKET as u16 {
                let sockaddr_ll: libc::sockaddr_ll =
                    std::ptr::read_unaligned(&(*(*buf).ifa_addr) as *const sockaddr as *const _);
                let string = format!(
                    "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
                    sockaddr_ll.sll_addr[0],
                    sockaddr_ll.sll_addr[1],
                    sockaddr_ll.sll_addr[2],
                    sockaddr_ll.sll_addr[3],
                    sockaddr_ll.sll_addr[4],
                    sockaddr_ll.sll_addr[5],
                );
                self.mac = string;
            }
        }

        self.state = State::new((*buf).ifa_flags as i32);
    }

    /// 更新当前网络设备的流量数据
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::NetworksInfo;
    ///
    /// let mut networks = NetworksInfo::new().unwrap();
    /// let mut iter_mut = networks.iter_mut();
    /// for info in iter_mut {
    ///     info.1.update().unwrap();
    /// }
    /// ```
    pub fn update_flow(&mut self) -> Result<(), Error> {
        self._update_flow()
    }

    /// 更新当前网络设备的基础信息
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::NetworksInfo;
    ///
    /// let mut networks = NetworksInfo::new().unwrap();
    /// let mut iter_mut = networks.iter_mut();
    /// for info in iter_mut {
    ///     info.1.update_baseinfo().unwrap();
    /// }
    /// ```
    pub fn update_baseinfo(&mut self) -> Result<(), Error> {
        let ifaddrs = get_ifaddrs()?;

        for ifaddr in ifaddrs {
            unsafe {
                // 此处 unwrap 必定成功
                let name = CStr::from_ptr((*ifaddr).ifa_name).to_str().unwrap();
                if name == self.name {
                    self._update_baseinfo(ifaddr);
                }
            }
        }

        Ok(())
    }

    /// 更新当前网络设备所有信息
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::NetworksInfo;
    ///
    /// let mut networks = NetworksInfo::new().unwrap();
    /// let mut iter_mut = networks.iter_mut();
    /// for info in iter_mut {
    ///     info.1.update().unwrap();
    /// }
    /// ```
    pub fn update(&mut self) -> Result<(), Error> {
        self.update_flow()?;
        self.update_baseinfo()
    }

    /// 返回当前网络设备的名字
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::NetworksInfo;
    ///
    /// let mut networks = NetworksInfo::new().unwrap();
    /// let mut iter_mut = networks.iter_mut();
    /// for info in iter_mut {
    ///     let _ = info.1.name();
    /// }
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }

    /// 返回当前网络设备的 ipv4 地址
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::NetworksInfo;
    ///
    /// let mut networks = NetworksInfo::new().unwrap();
    /// let mut iter_mut = networks.iter_mut();
    /// for info in iter_mut {
    ///     let _ = info.1.ipv4();
    /// }
    /// ```
    pub fn ipv4(&self) -> &str {
        &self.ipv4
    }

    /// 返回当前网络设备的 ipv6 地址
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::NetworksInfo;
    ///
    /// let mut networks = NetworksInfo::new().unwrap();
    /// let mut iter_mut = networks.iter_mut();
    /// for info in iter_mut {
    ///     let _ = info.1.ipv6();
    /// }
    /// ```
    pub fn ipv6(&self) -> &str {
        &self.ipv6
    }

    /// 返回当前网络设备的 mac 地址
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::NetworksInfo;
    ///
    /// let mut networks = NetworksInfo::new().unwrap();
    /// let mut iter_mut = networks.iter_mut();
    /// for info in iter_mut {
    ///     let _ = info.1.mac();
    /// }
    /// ```
    pub fn mac(&self) -> &str {
        &self.mac
    }

    /// 判断当前状态是否为 UP 状态
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::NetworksInfo;
    ///
    /// let mut networks = NetworksInfo::new().unwrap();
    /// let mut iter_mut = networks.iter_mut();
    /// for info in iter_mut {
    ///     info.1.is_up();
    /// }
    /// ```
    pub fn is_up(&self) -> bool {
        self.state.is_up()
    }

    /// 判断当前状态是否为 BROADCAST 状态
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::NetworksInfo;
    ///
    /// let mut networks = NetworksInfo::new().unwrap();
    /// let mut iter_mut = networks.iter_mut();
    /// for info in iter_mut {
    ///     info.1.is_broadcast();
    /// }
    /// ```
    pub fn is_broadcast(&self) -> bool {
        self.state.is_broadcast()
    }

    /// 判断当前状态是否为 DEBUG 状态
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::NetworksInfo;
    ///
    /// let mut networks = NetworksInfo::new().unwrap();
    /// let mut iter_mut = networks.iter_mut();
    /// for info in iter_mut {
    ///     info.1.is_debug();
    /// }
    /// ```
    pub fn is_debug(&self) -> bool {
        self.state.is_debug()
    }

    /// 判断当前状态是否为 LOOPBACK 状态
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::NetworksInfo;
    ///
    /// let mut networks = NetworksInfo::new().unwrap();
    /// let mut iter_mut = networks.iter_mut();
    /// for info in iter_mut {
    ///     info.1.is_loopback();
    /// }
    /// ```
    pub fn is_loopback(&self) -> bool {
        self.state.is_loopback()
    }

    /// 判断当前状态是否为 POINTOPOINT 状态
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::NetworksInfo;
    ///
    /// let mut networks = NetworksInfo::new().unwrap();
    /// let mut iter_mut = networks.iter_mut();
    /// for info in iter_mut {
    ///     info.1.is_point_to_point();
    /// }
    /// ```
    pub fn is_point_to_point(&self) -> bool {
        self.state.is_point_to_point()
    }

    /// 判断当前状态是否为 NOTRAILERS 状态
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::NetworksInfo;
    ///
    /// let mut networks = NetworksInfo::new().unwrap();
    /// let mut iter_mut = networks.iter_mut();
    /// for info in iter_mut {
    ///     info.1.is_no_trailers();
    /// }
    /// ```
    pub fn is_no_trailers(&self) -> bool {
        self.state.is_no_trailers()
    }

    /// 判断当前状态是否为 RUNNING 状态
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::NetworksInfo;
    ///
    /// let mut networks = NetworksInfo::new().unwrap();
    /// let mut iter_mut = networks.iter_mut();
    /// for info in iter_mut {
    ///     info.1.is_running();
    /// }
    /// ```
    pub fn is_running(&self) -> bool {
        self.state.is_running()
    }

    /// 判断当前状态是否为 NOARP 状态
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::NetworksInfo;
    ///
    /// let mut networks = NetworksInfo::new().unwrap();
    /// let mut iter_mut = networks.iter_mut();
    /// for info in iter_mut {
    ///     info.1.is_no_arp();
    /// }
    /// ```
    pub fn is_no_arp(&self) -> bool {
        self.state.is_no_arp()
    }

    /// 判断当前状态是否为 PROMISC 状态
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::NetworksInfo;
    ///
    /// let mut networks = NetworksInfo::new().unwrap();
    /// let mut iter_mut = networks.iter_mut();
    /// for info in iter_mut {
    ///     info.1.is_promisc();
    /// }
    /// ```
    pub fn is_promisc(&self) -> bool {
        self.state.is_promisc()
    }

    /// 判断当前状态是否为 ALLMULTI 状态
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::NetworksInfo;
    ///
    /// let mut networks = NetworksInfo::new().unwrap();
    /// let mut iter_mut = networks.iter_mut();
    /// for info in iter_mut {
    ///     info.1.is_all_multi();
    /// }
    /// ```
    pub fn is_all_multi(&self) -> bool {
        self.state.is_all_multi()
    }

    /// 判断当前状态是否为 MASTER 状态
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::NetworksInfo;
    ///
    /// let mut networks = NetworksInfo::new().unwrap();
    /// let mut iter_mut = networks.iter_mut();
    /// for info in iter_mut {
    ///     info.1.is_master();
    /// }
    /// ```
    pub fn is_master(&self) -> bool {
        self.state.is_master()
    }

    /// 判断当前状态是否为 SLAVE 状态
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::NetworksInfo;
    ///
    /// let mut networks = NetworksInfo::new().unwrap();
    /// let mut iter_mut = networks.iter_mut();
    /// for info in iter_mut {
    ///     info.1.is_slave();
    /// }
    /// ```
    pub fn is_slave(&self) -> bool {
        self.state.is_slave()
    }

    /// 判断当前状态是否为 MULTICAST 状态
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::NetworksInfo;
    ///
    /// let mut networks = NetworksInfo::new().unwrap();
    /// let mut iter_mut = networks.iter_mut();
    /// for info in iter_mut {
    ///     info.1.is_multicast();
    /// }
    /// ```
    pub fn is_multicast(&self) -> bool {
        self.state.is_multicast()
    }

    /// 判断当前状态是否为 PORTSEL 状态
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::NetworksInfo;
    ///
    /// let mut networks = NetworksInfo::new().unwrap();
    /// let mut iter_mut = networks.iter_mut();
    /// for info in iter_mut {
    ///     info.1.is_port_sel();
    /// }
    /// ```
    pub fn is_port_sel(&self) -> bool {
        self.state.is_port_sel()
    }

    /// 判断当前状态是否为 AUTOMEDIA 状态
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::NetworksInfo;
    ///
    /// let mut networks = NetworksInfo::new().unwrap();
    /// let mut iter_mut = networks.iter_mut();
    /// for info in iter_mut {
    ///     info.1.is_auto_media();
    /// }
    /// ```
    pub fn is_auto_media(&self) -> bool {
        self.state.is_auto_media()
    }

    /// 判断当前状态是否为 DYNAMIC 状态
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::NetworksInfo;
    ///
    /// let mut networks = NetworksInfo::new().unwrap();
    /// let mut iter_mut = networks.iter_mut();
    /// for info in iter_mut {
    ///     info.1.is_dynamic();
    /// }
    /// ```
    pub fn is_dynamic(&self) -> bool {
        self.state.is_dynamic()
    }

    /// 返回当前网络设备接收数据包数目
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::NetworksInfo;
    ///
    /// let mut networks = NetworksInfo::new().unwrap();
    /// let mut iter_mut = networks.iter_mut();
    /// for info in iter_mut {
    ///     let _ = info.1.rx_packets();
    /// }
    /// ```
    pub fn rx_packets(&self) -> u64 {
        self.rx_packets
    }

    /// 返回当前网络设备发送数据包数目
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::NetworksInfo;
    ///
    /// let mut networks = NetworksInfo::new().unwrap();
    /// let mut iter_mut = networks.iter_mut();
    /// for info in iter_mut {
    ///     let _ = info.1.tx_packets();
    /// }
    /// ```
    pub fn tx_packets(&self) -> u64 {
        self.tx_packets
    }

    /// 返回当前网络设备接收字节数目
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::NetworksInfo;
    ///
    /// let mut networks = NetworksInfo::new().unwrap();
    /// let mut iter_mut = networks.iter_mut();
    /// for info in iter_mut {
    ///     let _ = info.1.rx_bytes();
    /// }
    /// ```
    pub fn rx_bytes(&self) -> u64 {
        self.rx_bytes
    }

    /// 返回当前网络设备发送数据包数目
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::NetworksInfo;
    ///
    /// let mut networks = NetworksInfo::new().unwrap();
    /// let mut iter_mut = networks.iter_mut();
    /// for info in iter_mut {
    ///     let _ = info.1.tx_bytes();
    /// }
    /// ```
    pub fn tx_bytes(&self) -> u64 {
        self.tx_bytes
    }
}

#[cfg(test)]
mod test {
    use crate::error::{Error, InnerError};
    use crate::sys::network::{Network, State};
    use crate::NetworksInfo;
    use ylong_mock::stub_sync_func;

    #[test]
    fn ut_networks_test() {
        ut_networks_info_new();
        ut_networks_info_iter();
        ut_networks_info_iter_mut();
        ut_state();
    }

    /*
     * @test ut_networks_info_new
     * @title  NetworksInfo ut 测试用例
     * @brief  1. 获取基本的 NetworksInfo 信息
     *         2. 通过打桩使其进入不同的分支，并校验结果是否符合预期
     */
    fn ut_networks_info_new() {
        let networks_info = NetworksInfo::new();
        assert!(networks_info.is_ok());

        fn mock_update_flow() -> Result<(), Error> {
            Err(Error::Internal(InnerError::NotFound))
        }
        let mock = stub_sync_func!(Network::_update_flow, mock_update_flow).unwrap();
        let networks_info = NetworksInfo::new();
        mock.stub_remove();
        assert!(networks_info.is_err());
    }

    /*
     * @test ut_networks_info_iter
     * @title  NetworksInfo ut 测试用例
     * @brief  1. 获取基本的 NetworksInfo 信息
     *         2. 获取该结构的非可变引用迭代
     *         3. 校验结果是否符合预期
     */
    fn ut_networks_info_iter() {
        let networks_info = NetworksInfo::new().unwrap();
        for network in networks_info.iter() {
            assert_eq!(network.0, network.1.name());
        }
    }

    /*
     * @test ut_networks_info_iter_mut
     * @title  NetworksInfo ut 测试用例
     * @brief  1. 获取基本的 NetworksInfo 信息
     *         2. 获取该结构的可变引用迭代
     *         3. 校验结果是否符合预期
     */
    fn ut_networks_info_iter_mut() {
        let mut networks_info = NetworksInfo::new().unwrap();
        for network in networks_info.iter_mut() {
            assert_eq!(network.0, network.1.name());
        }
    }

    /*
     * @test ut_state
     * @title  State ut 测试用例
     * @brief  1. 获取基本的 State 信息
     *         2. 校验结果是否符合预期
     */
    fn ut_state() {
        let state = State::new(4099);
        assert!(state.is_up());
        assert!(state.is_broadcast());
        assert!(state.is_multicast());
        assert!(!state.is_debug());
        assert!(!state.is_loopback());
        assert!(!state.is_point_to_point());
        assert!(!state.is_no_trailers());
        assert!(!state.is_running());
        assert!(!state.is_no_arp());
        assert!(!state.is_promisc());
        assert!(!state.is_all_multi());
        assert!(!state.is_master());
        assert!(!state.is_slave());
        assert!(!state.is_port_sel());
        assert!(!state.is_auto_media());
        assert!(!state.is_dynamic());
    }
}
