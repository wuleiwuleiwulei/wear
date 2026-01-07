## Ylong_sysinfo 简明使用教程



### CPU 信息获取及使用方式

1. 创建一个 Cpu 的相关信息结构体，并使用得到的相关信息（示例不对 Error 情况做处理，请用户根据场景进行处理）。

   ```rust
   use ylong_sysinfo::CpusInfo;
   
   // 创建一个 Cpu 相关的信息结构体
   let mut cpus_info = CpusInfo::new().unwrap();
   
   // 获取 Cpu 相关信息时，最好经过一段时间间隔，否则获取到的 Cpu 占用率可能为 0
   sleep(Duration::new(1, 0));
   
   // 更新 Cpu 所有相关信息
   cpus_info.update_all().unwrap();
   
   // 获取全局 Cpu 的占用率（万分比）
   println!(
       "global_cpu_usage: {:?}%",
       cpus_info.global_cpu().usage() as f32 / 100.0
   );
   
   // 获取逻辑 Cpu 的占用率（万分比）
   for cpu in cpus_info.cpus() {
       println!("cpu_usage: {:?}%", cpu.usage() as f32 / 100.0)
   }
   ```



### Disk 信息获取及使用方式

1. 创建一个 Disk 的相关信息结构体，并使用得到的相关信息（示例不对 Error 情况做处理，请用户根据场景进行处理）。

   ```rust
   use ylong_sysinfo::DisksInfo;
   
   // 创建一个 Disk 相关的信息结构体
   let disks_info = DisksInfo::new().unwrap();
   
   // 获取所有 disk 的数组
   let mut mounts = disks_info.disks();
   // 将 disk 的数组按照 mount_point 的长度进行排序
   mounts.sort_by_key(|m2| std::cmp::Reverse(m2.mount_point().len()));
   println!("{mounts:?}");
   
   #[cfg(target_os = "windows")]
   let path = "D:\\";
   #[cfg(target_os = "linux")]
   let path = "/dev/mqueue";
   
   // 找到 mount_point 包含 path 的 disk
   let first_matched = mounts.iter().find(|m| path.starts_with(m.mount_point()));
   println!("{first_matched:?}");
   
   // 找到挂载在 path 的 disk
   let disk = disks_info.disk_at(path).unwrap();
   println!("{disk:?}");
   ```



### Memory 信息获取及使用方式

1. 创建一个 Memory 的相关信息结构体，并使用得到的相关信息（示例不对 Error 情况做处理，请用户根据场景进行处理）。

   ```rust
   use ylong_sysinfo::MemoryInfo;
   
   // 创建一个 Memory 相关的信息结构体
   let memory_info = MemoryInfo::new().unwrap();
   
   // 获取当前总物理内存
   let total_memory = memory_info.total_phys();
   
   // 获取当前可用物理内存
   let avail_memory = memory_info.avail_phys();
   println!("{total_memory:?} {avail_memory:?}");
   ```



### Network 信息获取及使用方式

1. 创建一个 Network 的相关信息结构体，并使用得到的相关信息（示例不对 Error 情况做处理，请用户根据场景进行处理）。

   ```rust
   use ylong_sysinfo::NetworksInfo;
   
   // 创建一个 Network 相关的信息结构体
   let networks_info = NetworksInfo::new().unwrap();
   
   // 迭代获取 Network 的相关信息
   for network in networks_info.iter() {
       println!("{:?}", network.0);
   }
   
   #[cfg(target_os = "windows")]
   let network_name = "以太网";
   #[cfg(target_os = "linux")]
   let network_name = "eth0";
   
   let mut iter = networks_info.iter();
   // 找到 name 包含 netwrork_name 并且不是 loopback 的 Network 设备
   let network = iter
   .find(|(name, ip_addr)| name.contains(network_name) && !ip_addr.is_loopback())
   .unwrap();
   println!("{:?}", network.0);
   ```



### Process 信息获取及使用方式

1. 创建一个 Process 的相关信息结构体，并使用得到的相关信息（示例不对 Error 情况做处理，请用户根据场景进行处理）。

   ```rust
   use ylong_sysinfo::ProcessesInfo;
   
   // 创建一个 Process 相关的信息结构体
   let processes_info = ProcessesInfo::new().unwrap();
   
   // 获取一个进程 name 包含 chrome 的进程迭代器
   let iter = processes_info
   	.processes()
   	.values()
   	.filter(|val| val.name().contains("chrome"));
   
   // 查看有哪些 process 名字包含 chrome
   for process in iter {
       println!("{process:?}");
   }
   ```

   

