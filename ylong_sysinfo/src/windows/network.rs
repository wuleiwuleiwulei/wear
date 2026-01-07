use crate::error::Error;
use crate::error::InnerError::{GetAdaptersAddressesFailed, GetIPAddressFailed, NotEnoughMemory};
use crate::sys::ffi::{
    GetAdaptersAddresses, GetIfEntry2, MibIfRow2, NetLuidLh, SockaddrIn, SockaddrIn6,
};
use crate::windows::ffi::IpAdapterAddressesLh;
use std::alloc::{alloc, dealloc, Layout};
use std::collections::hash_map::{Iter, IterMut};
use std::collections::HashMap;
use std::ffi::{c_ulong, c_ushort};
use std::mem;

const ERROR_SUCCESS: u32 = 0;
const ERROR_BUFFER_OVERFLOW: u32 = 111;

pub struct NetworksInfo {
    networks: HashMap<String, Network>,
}

impl NetworksInfo {
    /// Creates `NetworksInfo` Structure.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{NetworksInfo, Error};
    ///
    /// let networks_info = NetworksInfo::new()?;
    /// # Ok::<(), Error>(())
    /// ```
    pub fn new() -> Result<Self, Error> {
        let mut networks = HashMap::new();

        let adapters_addresses = get_adapters_addresses()?;
        for adapter_address in adapters_addresses {
            let name = get_name(adapter_address);

            if let std::collections::hash_map::Entry::Vacant(e) = networks.entry(name.clone()) {
                let mut network = Network::new(name.clone());
                match network._update_baseinfo(adapter_address) {
                    Ok(_) => {
                        network.update_flow();
                        e.insert(network);
                    }
                    Err(_err) => {
                        #[cfg(feature = "log")]
                        log::error!("update baseinfo failed:{}, name is {}", _err, name);
                    }
                }
            }
        }

        Ok(Self { networks })
    }

    /// Gets references iterations of networks information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{NetworksInfo, Error};
    ///
    /// let networks_info = NetworksInfo::new()?;
    /// let iter = networks_info.iter();
    /// for (name, _network) in iter {
    ///     println!("name: {:?}", name);
    /// }
    /// # Ok::<(), Error>(())
    /// ```
    pub fn iter(&self) -> Iter<String, Network> {
        self.networks.iter()
    }

    /// Gets mutable references iterations of networks information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{NetworksInfo, Error};
    ///
    /// let mut networks_info = NetworksInfo::new()?;
    /// let iter_mut = networks_info.iter_mut();
    /// for (name, _network) in iter_mut {
    ///     println!("name: {:?}", name);
    /// }
    /// # Ok::<(), Error>(())
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<String, Network> {
        self.networks.iter_mut()
    }

    /// Gets references to network information by name.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{NetworksInfo, Error};
    ///
    /// let networks_info = NetworksInfo::new()?;
    /// let name = "name";
    /// let network = networks_info.get(name);
    /// assert!(network.is_some());
    /// # Ok::<(), Error>(())
    /// ```
    pub fn get(&self, name: &str) -> Option<&Network> {
        self.networks.get(name)
    }

    /// Gets mutable references to network information by name.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{NetworksInfo, Error};
    ///
    /// let mut networks_info = NetworksInfo::new()?;
    /// let name = "name";
    /// let network = networks_info.get_mut(name);
    /// assert!(network.is_some());
    /// # Ok::<(), Error>(())
    /// ```
    pub fn get_mut(&mut self, name: &str) -> Option<&mut Network> {
        self.networks.get_mut(name)
    }
}

enum IpAddr {
    V4(String),
    V6(String),
}

pub struct Network {
    id: NetLuidLh,

    name: String,

    ipv4: String,

    ipv6: String,

    mac: String,

    rx_packets: u64,

    tx_packets: u64,

    rx_bytes: u64,

    tx_bytes: u64,
}

impl Network {
    fn new(name: String) -> Self {
        Self {
            id: NetLuidLh::new(0),
            name,
            ipv4: "".to_string(),
            ipv6: "".to_string(),
            mac: "".to_string(),
            rx_packets: 0,
            tx_packets: 0,
            rx_bytes: 0,
            tx_bytes: 0,
        }
    }

    /// Updates the `Network` flow.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{NetworksInfo, Error};
    ///
    /// let mut networks = NetworksInfo::new().unwrap();
    /// let mut iter_mut = networks.iter_mut();
    /// for info in iter_mut {
    ///     info.1.update_flow();
    /// }
    /// # Ok::<(), Error>(())
    /// ```
    pub fn update_flow(&mut self) {
        let result = std::mem::MaybeUninit::<MibIfRow2>::zeroed();

        unsafe {
            let mut result = result.assume_init();
            result.interface_luid = self.id;
            result.interface_index = 0;
            if GetIfEntry2(&mut result) != ERROR_SUCCESS {
                return;
            }
            self.rx_bytes = result.in_octets;
            self.tx_bytes = result.out_octets;
            self.rx_packets = result.in_ucast_pkts.wrapping_add(result.in_nucast_pkts);
            self.tx_packets = result.out_ucast_pkts.wrapping_add(result.out_nucast_pkts);
        }
    }

    /// Updates the 'Network' base information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{NetworksInfo, Error};
    ///
    /// let mut networks = NetworksInfo::new()?;
    /// let mut iter_mut = networks.iter_mut();
    /// for info in iter_mut {
    ///     info.1.update_baseinfo().unwrap();
    /// }
    /// # Ok::<(), Error>(())
    /// ```
    pub fn update_baseinfo(&mut self) -> Result<(), Error> {
        let adapters_addresses = get_adapters_addresses()?;
        for adapter_address in adapters_addresses {
            let name = get_name(adapter_address);
            if name == self.name {
                self._update_baseinfo(adapter_address)?;
            }
        }
        Ok(())
    }

    fn _update_baseinfo(
        &mut self,
        adapter_address: *mut IpAdapterAddressesLh,
    ) -> Result<(), Error> {
        if adapter_address.is_null() {
            return Err(Error::Internal(GetIPAddressFailed));
        }
        let id = get_id(adapter_address);
        let mac = get_mac_address(adapter_address);
        let ip_addr = get_ip_address(adapter_address)?;
        self.id = id;
        self.mac = mac;
        match ip_addr {
            IpAddr::V4(ipv4) => self.ipv4 = ipv4,
            IpAddr::V6(ipv6) => self.ipv6 = ipv6,
        }
        Ok(())
    }

    /// Updates the 'Network' all information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{NetworksInfo, Error};
    ///
    /// let mut networks = NetworksInfo::new()?;
    /// let mut iter_mut = networks.iter_mut();
    /// for info in iter_mut {
    ///     info.1.update().unwrap();
    /// }
    /// # Ok::<(), Error>(())
    /// ```
    pub fn update(&mut self) -> Result<(), Error> {
        self.update_flow();
        self.update_baseinfo()
    }

    /// Returns the name of the current network device.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{NetworksInfo, Error};
    ///
    /// let mut networks = NetworksInfo::new()?;
    /// let mut iter_mut = networks.iter_mut();
    /// for info in iter_mut {
    ///     let _ = info.1.name();
    /// }
    /// # Ok::<(), Error>(())
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the ipv4 address of the current network device.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{NetworksInfo, Error};
    ///
    /// let mut networks = NetworksInfo::new()?;
    /// let mut iter_mut = networks.iter_mut();
    /// for info in iter_mut {
    ///     let _ = info.1.ipv4();
    /// }
    /// # Ok::<(), Error>(())
    /// ```
    pub fn ipv4(&self) -> &str {
        &self.ipv4
    }

    /// Returns the ipv6 address of the current network device.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{NetworksInfo, Error};
    ///
    /// let mut networks = NetworksInfo::new()?;
    /// let mut iter_mut = networks.iter_mut();
    /// for info in iter_mut {
    ///     let _ = info.1.ipv6();
    /// }
    /// # Ok::<(), Error>(())
    /// ```
    pub fn ipv6(&self) -> &str {
        &self.ipv6
    }

    /// Returns the mac address of the current network device.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{NetworksInfo, Error};
    ///
    /// let mut networks = NetworksInfo::new()?;
    /// let mut iter_mut = networks.iter_mut();
    /// for info in iter_mut {
    ///     let _ = info.1.mac();
    /// }
    /// # Ok::<(), Error>(())
    /// ```
    pub fn mac(&self) -> &str {
        &self.mac
    }

    /// Determines if the current state is `LOOPBACK` state.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{NetworksInfo, Error};
    ///
    /// let mut networks = NetworksInfo::new()?;
    /// let mut iter_mut = networks.iter_mut();
    /// for info in iter_mut {
    ///     info.1.is_loopback();
    /// }
    /// # Ok::<(), Error>(())
    /// ```
    pub fn is_loopback(&self) -> bool {
        if !self.ipv4.is_empty() && self.ipv4.contains("127") {
            return true;
        }

        if !self.ipv6.is_empty() && self.ipv6.eq("0:0:0:0:0:0:0:1") {
            return true;
        }

        false
    }

    /// Returns the number of packets received by the current network device.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{NetworksInfo, Error};
    ///
    /// let mut networks = NetworksInfo::new()?;
    /// let mut iter_mut = networks.iter_mut();
    /// for info in iter_mut {
    ///     let _ = info.1.rx_packets();
    /// }
    /// # Ok::<(), Error>(())
    /// ```
    pub fn rx_packets(&self) -> u64 {
        self.rx_packets
    }

    /// Returns the number of packets sent by the current network device.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{NetworksInfo, Error};
    ///
    /// let mut networks = NetworksInfo::new()?;
    /// let mut iter_mut = networks.iter_mut();
    /// for info in iter_mut {
    ///     let _ = info.1.tx_packets();
    /// }
    /// # Ok::<(), Error>(())
    /// ```
    pub fn tx_packets(&self) -> u64 {
        self.tx_packets
    }

    /// Returns the number of bytes received by the current network device.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{NetworksInfo, Error};
    ///
    /// let mut networks = NetworksInfo::new()?;
    /// let mut iter_mut = networks.iter_mut();
    /// for info in iter_mut {
    ///     let _ = info.1.rx_bytes();
    /// }
    /// # Ok::<(), Error>(())
    /// ```
    pub fn rx_bytes(&self) -> u64 {
        self.rx_bytes
    }

    /// Returns the number of bytes sent by the current network device.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{NetworksInfo, Error};
    ///
    /// let mut networks = NetworksInfo::new()?;
    /// let mut iter_mut = networks.iter_mut();
    /// for info in iter_mut {
    ///     let _ = info.1.tx_bytes();
    /// }
    /// # Ok::<(), Error>(())
    /// ```
    pub fn tx_bytes(&self) -> u64 {
        self.tx_bytes
    }
}

struct AdaptersAddresses {
    ptr: *mut IpAdapterAddressesLh,
    layout: Layout,
    next: *mut IpAdapterAddressesLh,
}

impl AdaptersAddresses {
    fn new(size: usize) -> Result<Self, Error> {
        let layout = Layout::from_size_align(size, mem::align_of::<IpAdapterAddressesLh>())
            .map_err(|_| Error::Internal(NotEnoughMemory))?;
        let ptr: *mut IpAdapterAddressesLh = unsafe { alloc(layout).cast() };
        Ok(Self {
            ptr,
            layout,
            next: ptr,
        })
    }
}

impl Iterator for AdaptersAddresses {
    type Item = *mut IpAdapterAddressesLh;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next;
        if !next.is_null() {
            self.next = unsafe { (*next).next };
            return Some(next);
        }
        None
    }
}

impl Drop for AdaptersAddresses {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.ptr.cast(), self.layout);
        }
    }
}

fn get_adapters_addresses() -> Result<AdaptersAddresses, Error> {
    const AF_UNSPEC: c_ulong = 0;
    const FLAGS: c_ulong = 0;
    const INITIAL_BUFFER_SIZE: u32 = 15000;

    let mut size: u32 = INITIAL_BUFFER_SIZE;

    loop {
        let adapters_addresses = AdaptersAddresses::new(size as usize)?;

        let result = unsafe {
            GetAdaptersAddresses(
                AF_UNSPEC,
                FLAGS,
                std::ptr::null_mut(),
                adapters_addresses.ptr,
                &mut size,
            )
        };

        break match result {
            ERROR_SUCCESS => Ok(adapters_addresses),
            ERROR_BUFFER_OVERFLOW => continue,
            _ => Err(Error::Internal(GetAdaptersAddressesFailed)),
        };
    }
}

fn get_name(adapter_address: *mut IpAdapterAddressesLh) -> String {
    let len = unsafe {
        let mut ptr = (*adapter_address).friendly_name;
        while *ptr != 0 {
            ptr = ptr.offset(1);
        }
        match ptr.offset_from((*adapter_address).friendly_name).try_into() {
            Ok(size) => size,
            Err(_) => return "unknown".to_string(),
        }
    };
    String::from_utf16_lossy(unsafe {
        std::slice::from_raw_parts((*adapter_address).friendly_name, len)
    })
}

fn get_ip_address(adapter_address: *mut IpAdapterAddressesLh) -> Result<IpAddr, Error> {
    let first_unicast_address = unsafe { (*adapter_address).first_unicast_address };
    if first_unicast_address.is_null() {
        return Err(Error::Internal(GetIPAddressFailed));
    }
    let socket_address = unsafe { (*first_unicast_address).address.lp_sockaddr };
    if socket_address.is_null() {
        return Err(Error::Internal(GetIPAddressFailed));
    }

    const AF_INET: c_ushort = 2;
    const AF_INET6: c_ushort = 23;

    let socket_address_family = unsafe { (*socket_address).sa_family };
    if socket_address_family == AF_INET {
        let socket_address = socket_address.cast::<SockaddrIn>();
        if socket_address.is_null() {
            return Err(Error::Internal(GetIPAddressFailed));
        }
        let address = unsafe { (*socket_address).sin_addr.s_un.s_addr };
        let ipv4 = address.to_ne_bytes();
        Ok(IpAddr::V4(format!(
            "{}.{}.{}.{}",
            ipv4[0], ipv4[1], ipv4[2], ipv4[3]
        )))
    } else if socket_address_family == AF_INET6 {
        let socket_address = socket_address.cast::<SockaddrIn6>();
        if socket_address.is_null() {
            return Err(Error::Internal(GetIPAddressFailed));
        }
        let [a, b, c, d, e, f, g, h] = unsafe { (*socket_address).sin6_addr.u.word };
        let ipv6 = [
            c_ushort::from_be(a),
            c_ushort::from_be(b),
            c_ushort::from_be(c),
            c_ushort::from_be(d),
            c_ushort::from_be(e),
            c_ushort::from_be(f),
            c_ushort::from_be(g),
            c_ushort::from_be(h),
        ];
        Ok(IpAddr::V6(format!(
            "{}:{}:{}:{}:{}:{}:{}:{}",
            ipv6[0], ipv6[1], ipv6[2], ipv6[3], ipv6[4], ipv6[5], ipv6[6], ipv6[7]
        )))
    } else {
        Err(Error::Internal(GetIPAddressFailed))
    }
}

fn get_mac_address(adapter_address: *mut IpAdapterAddressesLh) -> String {
    let physical_address = unsafe { (*adapter_address).physical_address };
    format!(
        "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        physical_address[0],
        physical_address[1],
        physical_address[2],
        physical_address[3],
        physical_address[4],
        physical_address[5],
        physical_address[6],
        physical_address[7]
    )
}

fn get_id(adapter_address: *mut IpAdapterAddressesLh) -> NetLuidLh {
    unsafe { (*adapter_address).luid }
}

#[cfg(test)]
mod test {
    use crate::NetworksInfo;
    use local_ip_address::list_afinet_netifas;

    /// UT test for the basic `NetworksInfo`.
    ///
    /// # Title
    /// ut_networks_info_cross_verify
    ///
    /// # Brief
    /// 1. Gets the basic `NetworksInfo`.
    /// 2. Verifies data value with third-party local_ip_address.
    #[test]
    fn ut_networks_info_cross_verify() {
        let networks_info = NetworksInfo::new().unwrap();
        for (name, id_addr) in list_afinet_netifas().unwrap() {
            let network = networks_info.get(&name).unwrap();
            if !network.is_loopback() && id_addr.is_ipv4() {
                assert_eq!(network.ipv4, id_addr.to_string());
            }
            if !network.is_loopback() && id_addr.is_ipv6() {
                assert_eq!(network.ipv6, id_addr.to_string());
            }
        }
    }
}
