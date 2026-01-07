//! <https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/>

use std::ffi::{
    c_char, c_double, c_int, c_long, c_longlong, c_uchar, c_uint, c_ulong, c_ulonglong, c_ushort,
    c_void,
};

#[repr(C)]
struct SystemInfoU([c_ulong; 1]);

#[repr(C)]
pub(crate) struct SystemInfo {
    u: SystemInfoU,
    dw_page_size: c_ulong,
    lp_minimum_application_address: *mut c_void,
    lp_maximum_application_address: *mut c_void,
    dw_active_processor_mask: usize,
    pub(crate) dw_number_of_processors: c_ulong,
    dw_processor_type: c_ulong,
    dw_allocation_granularity: c_ulong,
    w_processor_level: c_ushort,
    w_processor_revision: c_ushort,
}

#[repr(C)]
pub(crate) union PdhFmtCounterValueU {
    pub(crate) long_value: c_long,
    double_value: c_double,
    large_value: c_longlong,
    ansi_string_value: *const c_char,
    wide_string_value: *const c_char,
}

#[repr(C)]
pub(crate) struct PdhFmtCounterValue {
    c_status: c_ulong,
    pub(crate) u: PdhFmtCounterValueU,
}

#[link(name = "pdh")]
extern "system" {
    pub(crate) fn GetSystemInfo(lpSystemInfo: *mut SystemInfo);

    pub(crate) fn PdhOpenQueryA(
        szDataSource: *const c_char,
        dwUserData: usize,
        phQuery: *mut *mut c_void,
    ) -> c_long;

    pub(crate) fn PdhAddEnglishCounterW(
        hQuery: *mut c_void,
        szFullCounterPath: *const c_ushort,
        dwUserData: usize,
        phCounter: *mut *mut c_void,
    ) -> c_long;

    pub(crate) fn PdhRemoveCounter(hCounter: *mut c_void) -> c_long;

    pub(crate) fn PdhGetFormattedCounterValue(
        hCounter: *mut c_void,
        dwFormat: c_ulong,
        lpdwType: *mut c_ulong,
        pValue: *mut PdhFmtCounterValue,
    ) -> c_long;

    pub(crate) fn PdhCollectQueryData(hQuery: *mut c_void) -> c_long;

    pub(crate) fn PdhCloseQuery(hQuery: *mut c_void) -> c_long;

    pub(crate) fn GetLogicalDrives() -> c_ulong;

    pub(crate) fn GetDriveTypeW(lpRootPathName: *const c_ushort) -> c_uint;

    pub(crate) fn GetVolumeInformationW(
        lpRootPathName: *const c_ushort,
        lpVolumeNameBuffer: *const c_ushort,
        nVolumeNameSize: c_ulong,
        lpVolumeSerialNumber: *mut c_ulong,
        lpMaximumComponentLength: *mut c_ulong,
        lpFileSystemFlags: *mut c_ulong,
        lpFileSystemNameBuffer: *mut c_ushort,
        nFileSystemNameSize: c_ulong,
    ) -> c_int;

    pub(crate) fn GetDiskFreeSpaceExW(
        lpDirectoryName: *const c_ushort,
        lpFreeBytesAvailableToCaller: *mut c_ulonglong,
        lpTotalNumberOfBytes: *mut c_ulonglong,
        lpTotalNumberOfFreeBytes: *mut c_ulonglong,
    ) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct IpAdapterAddressesLh00 {
    length: c_ulong,
    if_index: c_ulong,
}

#[repr(C)]
pub(crate) union IpAdapterAddressesLh0 {
    alignment: c_ulonglong,
    anonymous: IpAdapterAddressesLh00,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct IpAdapterUnicastAddressLh00 {
    length: c_ulong,
    flags: c_ulong,
}

#[repr(C)]
pub(crate) union IpAdapterUnicastAddressLh0 {
    alignment: c_ulonglong,
    anonymous: IpAdapterUnicastAddressLh00,
}

#[repr(C)]
pub struct Sockaddr {
    pub(crate) sa_family: c_ushort,
    pub(crate) sa_data: [c_uchar; 14],
}

#[repr(C)]
pub(crate) struct SocketAddress {
    pub(crate) lp_sockaddr: *mut Sockaddr,
    i_sockaddr_length: c_int,
}

#[repr(C)]
pub(crate) struct IpAdapterUnicastAddressLh {
    anonymous: IpAdapterUnicastAddressLh0,
    next: *mut IpAdapterUnicastAddressLh,
    pub(crate) address: SocketAddress,
    prefix_origin: c_int,
    suffix_origin: c_int,
    dad_state: c_int,
    valid_lifetime: c_ulong,
    preferred_lifetime: c_ulong,
    lease_lifetime: c_ulong,
    on_link_prefix_length: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct IpAdapterAnycastAddressXp00 {
    length: c_ulong,
    flags: c_ulong,
}

#[repr(C)]
pub(crate) union IpAdapterAnycastAddressXp0 {
    alignment: c_ulonglong,
    anonymous: IpAdapterAnycastAddressXp00,
}

#[repr(C)]
pub(crate) struct IpAdapterAnycastAddressXp {
    anonymous: IpAdapterAnycastAddressXp0,
    next: *mut IpAdapterAnycastAddressXp,
    address: SocketAddress,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct IpAdapterMulticastAddressXp00 {
    length: c_ulong,
    flags: c_ulong,
}

#[repr(C)]
pub(crate) union IpAdapterMulticastAddressXp0 {
    alignment: c_ulonglong,
    anonymous: IpAdapterMulticastAddressXp00,
}

#[repr(C)]
pub(crate) struct IpAdapterMulticastAddressXp {
    anonymous: IpAdapterMulticastAddressXp0,
    next: *mut IpAdapterMulticastAddressXp,
    address: SocketAddress,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct IpAdapterDnsServerAddressXp00 {
    length: c_ulong,
    reserved: c_ulong,
}

#[repr(C)]
pub(crate) union IpAdapterDnsServerAddressXp0 {
    alignment: c_ulonglong,
    anonymous: IpAdapterDnsServerAddressXp00,
}

#[repr(C)]
pub(crate) struct IpAdapterDnsServerAddressXp {
    anonymous: IpAdapterDnsServerAddressXp0,
    next: *mut IpAdapterDnsServerAddressXp,
    address: SocketAddress,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct IpAdapterAddressesLh10 {
    pub(crate) _bitfield: c_ulong,
}

#[repr(C)]
pub(crate) union IpAdapterAddressesLh1 {
    pub(crate) flags: c_ulong,
    pub(crate) anonymous: IpAdapterAddressesLh10,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct IpAdapterPrefixXp00 {
    length: c_ulong,
    flags: c_ulong,
}

#[repr(C)]
pub(crate) union IpAdapterPrefixXp0 {
    alignment: c_ulonglong,
    anonymous: IpAdapterPrefixXp00,
}

#[repr(C)]
pub(crate) struct IpAdapterPrefixXp {
    anonymous: IpAdapterPrefixXp0,
    next: *mut IpAdapterPrefixXp,
    address: SocketAddress,
    prefix_length: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct IpAdapterWinsServerAddressLh00 {
    length: c_ulong,
    reserved: c_ulong,
}

#[repr(C)]
pub(crate) union IpAdapterWinsServerAddressLh0 {
    alignment: c_ulonglong,
    anonymous: IpAdapterWinsServerAddressLh00,
}

#[repr(C)]
pub(crate) struct IpAdapterWinsServerAddressLh {
    anonymous: IpAdapterWinsServerAddressLh0,
    next: *mut IpAdapterWinsServerAddressLh,
    address: SocketAddress,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct IpAdapterGatewayAddressLh00 {
    length: c_ulong,
    reserved: c_ulong,
}

#[repr(C)]
pub(crate) union IpAdapterGatewayAddressLh0 {
    alignment: c_ulonglong,
    anonymous: IpAdapterGatewayAddressLh00,
}

#[repr(C)]
pub(crate) struct IpAdapterGatewayAddressLh {
    anonymous: IpAdapterGatewayAddressLh0,
    next: *mut IpAdapterGatewayAddressLh,
    address: SocketAddress,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct NetLuidLh0 {
    _bitfield: c_ulonglong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union NetLuidLh {
    pub(crate) value: c_ulonglong,
    info: NetLuidLh0,
}

impl NetLuidLh {
    pub(crate) fn new(value: c_ulonglong) -> Self {
        Self { value }
    }
}

#[repr(C)]
pub(crate) struct Guid {
    data1: c_ulong,
    data2: c_ushort,
    data3: c_ushort,
    data4: [c_uchar; 8],
}

#[repr(C)]
pub(crate) struct IpAdapterDnsSuffix {
    next: *mut IpAdapterDnsSuffix,
    string: [c_ushort; 256],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct InAddr00 {
    pub(crate) s_b1: c_uchar,
    pub(crate) s_b2: c_uchar,
    pub(crate) s_b3: c_uchar,
    pub(crate) s_b4: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct InAddr01 {
    pub(crate) s_w1: c_ushort,
    pub(crate) s_w2: c_ushort,
}

#[repr(C)]
pub(crate) union InAddr0 {
    pub(crate) s_un_b: InAddr00,
    pub(crate) s_un_w: InAddr01,
    pub(crate) s_addr: c_ulong,
}

#[repr(C)]
pub(crate) struct InAddr {
    pub(crate) s_un: InAddr0,
}

#[repr(C)]
pub(crate) struct SockaddrIn {
    pub(crate) sin_family: c_ushort,
    pub(crate) sin_port: c_ushort,
    pub(crate) sin_addr: InAddr,
    pub(crate) sin_zero: [c_uchar; 8],
}

#[repr(C)]
pub(crate) union In6Addr0 {
    pub(crate) byte: [c_uchar; 16],
    pub(crate) word: [c_ushort; 8],
}

#[repr(C)]
pub(crate) struct In6Addr {
    pub(crate) u: In6Addr0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct ScopeId00 {
    pub(crate) _bitfield: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union ScopeId0 {
    pub(crate) anonymous: ScopeId00,
    pub(crate) value: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct ScopeId {
    pub(crate) anonymous: ScopeId0,
}

#[repr(C)]
pub(crate) union SockaddrIn60 {
    pub(crate) sin6_scope_id: c_ulong,
    pub(crate) sin6_scope_struct: ScopeId,
}

#[repr(C)]
pub(crate) struct SockaddrIn6 {
    pub(crate) sin6_family: c_ushort,
    pub(crate) sin6_port: c_ushort,
    pub(crate) sin6_flowinfo: c_ulong,
    pub(crate) sin6_addr: In6Addr,
    pub(crate) anonymous: SockaddrIn60,
}

#[repr(C)]
pub(crate) struct IpAdapterAddressesLh {
    pub(crate) anonymous1: IpAdapterAddressesLh0,
    pub(crate) next: *mut IpAdapterAddressesLh,
    pub(crate) adapter_name: *mut c_uchar,
    pub(crate) first_unicast_address: *mut IpAdapterUnicastAddressLh,
    pub(crate) first_anycast_address: *mut IpAdapterAnycastAddressXp,
    pub(crate) first_multicast_address: *mut IpAdapterMulticastAddressXp,
    pub(crate) first_dns_server_address: *mut IpAdapterDnsServerAddressXp,
    pub(crate) dns_suffix: *mut c_ushort,
    pub(crate) description: *mut c_ushort,
    pub(crate) friendly_name: *mut c_ushort,
    pub(crate) physical_address: [c_uchar; 8],
    pub(crate) physical_address_length: c_ulong,
    pub(crate) anonymous2: IpAdapterAddressesLh1,
    pub(crate) mtu: c_ulong,
    pub(crate) if_type: c_ulong,
    pub(crate) oper_status: c_int,
    pub(crate) ipv6if_index: c_ulong,
    pub(crate) zone_indices: [c_ulong; 16],
    pub(crate) first_prefix: *mut IpAdapterPrefixXp,
    pub(crate) transmit_link_speed: c_ulonglong,
    pub(crate) receive_link_speed: c_ulonglong,
    pub(crate) first_wins_server_address: *mut IpAdapterWinsServerAddressLh,
    pub(crate) first_gateway_address: *mut IpAdapterGatewayAddressLh,
    pub(crate) ipv4metric: c_ulong,
    pub(crate) ipv6metric: c_ulong,
    pub(crate) luid: NetLuidLh,
    pub(crate) dhcpv4server: SocketAddress,
    pub(crate) compartment_id: c_ulong,
    pub(crate) network_guid: Guid,
    pub(crate) connection_type: c_int,
    pub(crate) tunnel_type: c_int,
    pub(crate) dhcpv6server: SocketAddress,
    pub(crate) dhcpv6client_duid: [c_uchar; 130],
    pub(crate) dhcpv6client_duid_length: c_ulong,
    pub(crate) dhcpv6iaid: c_ulong,
    pub(crate) first_dns_suffix: *mut IpAdapterDnsSuffix,
}

#[link(name = "iphlpapi")]
extern "system" {
    pub(crate) fn GetAdaptersAddresses(
        Family: c_ulong,
        Flags: c_ulong,
        Reserved: *mut c_void,
        AdapterAddresses: *mut IpAdapterAddressesLh,
        SizePointer: *mut c_ulong,
    ) -> c_uint;
}

const IF_MAX_STRING_SIZE: usize = 256;
const IF_MAX_PHYS_ADDRESS_LENGTH: usize = 32;

#[repr(C)]
struct MibIfRow2InterfaceAndOperStatusFlags {
    bitfield: c_uchar,
}

#[repr(C)]
pub struct MibIfRow2 {
    pub(crate) interface_luid: NetLuidLh,
    pub(crate) interface_index: c_ulong,
    interface_guid: Guid,
    alias: [c_ushort; IF_MAX_STRING_SIZE + 1],
    description: [c_ushort; IF_MAX_STRING_SIZE + 1],
    physical_address_length: c_ulong,
    physical_address: [c_uchar; IF_MAX_PHYS_ADDRESS_LENGTH],
    permanent_physical_address: [c_uchar; IF_MAX_PHYS_ADDRESS_LENGTH],
    mtu: c_ulong,
    typ: c_ulong,
    tunnel_type: c_ulong,
    media_type: c_int,
    physical_medium_type: c_int,
    access_type: c_ulong,
    direction_type: c_ulong,
    interface_and_oper_status_flags: MibIfRow2InterfaceAndOperStatusFlags,
    oper_status: c_ulong,
    admin_status: c_ulong,
    media_connect_state: c_ulong,
    network_guid: Guid,
    connection_type: c_ulong,
    transmit_link_speed: c_ulonglong,
    receive_link_speed: c_ulonglong,
    pub(crate) in_octets: c_ulonglong,
    pub(crate) in_ucast_pkts: c_ulonglong,
    pub(crate) in_nucast_pkts: c_ulonglong,
    in_discards: c_ulonglong,
    in_errors: c_ulonglong,
    in_unknown_protos: c_ulonglong,
    in_ucast_octets: c_ulonglong,
    in_multicast_octets: c_ulonglong,
    in_broadcast_octets: c_ulonglong,
    pub(crate) out_octets: c_ulonglong,
    pub(crate) out_ucast_pkts: c_ulonglong,
    pub(crate) out_nucast_pkts: c_ulonglong,
    out_discards: c_ulonglong,
    out_errors: c_ulonglong,
    out_ucast_octets: c_ulonglong,
    out_multicast_octets: c_ulonglong,
    out_broadcast_octets: c_ulonglong,
    out_qlen: c_ulonglong,
}

extern "system" {
    pub(crate) fn GetIfEntry2(Row: *mut MibIfRow2) -> c_ulong;
}

#[repr(C)]
pub(crate) struct MemoryStatusEx {
    pub(crate) dw_length: c_ulong,
    pub(crate) dw_memory_load: c_ulong,
    pub(crate) ull_total_phys: c_ulonglong,
    pub(crate) ull_avail_phys: c_ulonglong,
    pub(crate) ull_total_page_file: c_ulonglong,
    pub(crate) ull_avail_page_file: c_ulonglong,
    pub(crate) ull_total_virtual: c_ulonglong,
    pub(crate) ull_avail_virtual: c_ulonglong,
    pub(crate) ull_avail_extended_virtual: c_ulonglong,
}

extern "system" {
    pub(crate) fn GlobalMemoryStatusEx(lpBuffer: *mut MemoryStatusEx) -> c_int;
}

#[repr(C)]
pub(crate) enum SystemInformationClass {
    SystemProcessInformation = 5,
}

#[repr(C)]
pub(crate) struct LargeInteger([c_longlong; 1]);

#[repr(C)]
pub(crate) struct UnicodeString {
    pub(crate) length: c_ushort,
    pub(crate) maximum_length: c_ushort,
    pub(crate) buffer: *mut c_ushort,
}

#[repr(C)]
pub(crate) struct ClientId {
    unique_process: *mut c_void,
    unique_thread: *mut c_void,
}

// Structure internal type, only for enum types not using internal values.
#[repr(C)]
#[allow(dead_code)]
pub(crate) enum KthreadState {
    Initialized = 0,
    Ready = 1,
    Running = 2,
    Standby = 3,
    Terminated = 4,
    Waiting = 5,
    Transition = 6,
    DeferredReady = 7,
    GateWaitObsolete = 8,
    WaitingForProcessInSwap = 9,
    MaximumThreadState = 10,
}

// Structure internal type, only for enum types not using internal values.
#[repr(C)]
#[allow(dead_code)]
pub(crate) enum KwaitReason {
    Executive = 0,
    FreePage = 1,
    PageIn = 2,
    PoolAllocation = 3,
    DelayExecution = 4,
    Suspended = 5,
    UserRequest = 6,
    WrExecutive = 7,
    WrFreePage = 8,
    WrPageIn = 9,
    WrPoolAllocation = 10,
    WrDelayExecution = 11,
    WrSuspended = 12,
    WrUserRequest = 13,
    WrEventPair = 14,
    WrQueue = 15,
    WrLpcReceive = 16,
    WrLpcReply = 17,
    WrVirtualMemory = 18,
    WrPageOut = 19,
    WrRendezvous = 20,
    WrKeyedEvent = 21,
    WrTerminated = 22,
    WrProcessInSwap = 23,
    WrCpuRateControl = 24,
    WrCalloutStack = 25,
    WrKernel = 26,
    WrResource = 27,
    WrPushLock = 28,
    WrMutex = 29,
    WrQuantumEnd = 30,
    WrDispatchInt = 31,
    WrPreempted = 32,
    WrYieldExecution = 33,
    WrFastMutex = 34,
    WrGuardedMutex = 35,
    WrRundown = 36,
    WrAlertByThreadId = 37,
    WrDeferredPreempt = 38,
    MaximumWaitReason = 39,
}

#[repr(C)]
pub(crate) struct SystemThreadInformation {
    kernel_time: LargeInteger,
    user_time: LargeInteger,
    create_time: LargeInteger,
    wait_time: c_ulong,
    start_address: *mut c_void,
    client_id: ClientId,
    priority: c_long,
    base_priority: c_long,
    context_switches: c_ulong,
    thread_state: KthreadState,
    wait_reason: KwaitReason,
}

#[repr(C)]
pub(crate) struct SystemProcessInformation {
    pub(crate) next_entry_offset: c_ulong,
    pub(crate) number_of_threads: c_ulong,
    pub(crate) working_set_private_size: LargeInteger,
    pub(crate) hard_fault_count: c_ulong,
    pub(crate) number_of_threads_high_watermark: c_ulong,
    pub(crate) cycle_time: c_ulonglong,
    pub(crate) create_time: LargeInteger,
    pub(crate) user_time: LargeInteger,
    pub(crate) kernel_time: LargeInteger,
    pub(crate) image_name: UnicodeString,
    pub(crate) base_priority: c_long,
    pub(crate) unique_process_id: *mut c_void,
    pub(crate) inherited_from_unique_process_id: *mut c_void,
    pub(crate) handle_count: c_ulong,
    pub(crate) session_id: c_ulong,
    pub(crate) unique_process_key: usize,
    pub(crate) peak_virtual_size: usize,
    pub(crate) virtual_size: usize,
    pub(crate) page_fault_count: c_ulong,
    pub(crate) peak_working_set_size: usize,
    pub(crate) working_set_size: usize,
    pub(crate) quota_peak_paged_pool_usage: usize,
    pub(crate) quota_paged_pool_usage: usize,
    pub(crate) quota_peak_non_paged_pool_usage: usize,
    pub(crate) quota_non_paged_pool_usage: usize,
    pub(crate) pagefile_usage: usize,
    pub(crate) peak_pagefile_usage: usize,
    pub(crate) private_page_count: usize,
    pub(crate) read_operation_count: LargeInteger,
    pub(crate) write_operation_count: LargeInteger,
    pub(crate) other_operation_count: LargeInteger,
    pub(crate) read_transfer_count: LargeInteger,
    pub(crate) write_transfer_count: LargeInteger,
    pub(crate) other_transfer_count: LargeInteger,
    pub(crate) threads: [SystemThreadInformation; 1],
}

#[link(name = "ntdll")]
extern "system" {
    pub(crate) fn NtQuerySystemInformation(
        SystemInformationClass: SystemInformationClass,
        SystemInformation: *mut c_void,
        SystemInformationLength: c_ulong,
        ReturnLength: *mut c_ulong,
    ) -> c_long;
}
