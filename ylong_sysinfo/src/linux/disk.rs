use crate::error::InnerError::Overflow;
use crate::error::{Error, InnerError};
use libc::statvfs;
use std::ffi::CString;
use std::fs::File;
use std::io::{Error as OsError, Read};
use std::path::Path;
use std::slice::{Iter, IterMut};

#[derive(Debug)]
pub struct DisksInfo {
    // 所有磁盘的信息列表
    disks: Vec<Disk>,
}

impl DisksInfo {
    /// 创建所有磁盘的信息列表结构
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::DisksInfo;
    ///
    /// // 获得磁盘列表的基本信息
    /// let disks_info = DisksInfo::new();
    /// assert!(disks_info.is_ok());
    /// ```
    pub fn new() -> Result<Self, Error> {
        let mut string = String::new();
        if let Err(e) = File::open("/proc/mounts").and_then(|mut f| f.read_to_string(&mut string)) {
            return Err(Error::Os(e));
        }

        let disks = string
            .lines()
            .map(|line| {
                // 仅获取所需数据，调用 next() 获取，也防止调用 split(" ")
                // 找不到 index 整个进程 panic 掉
                let mut temp = line.split_whitespace();
                let fs_spec = temp.next().unwrap_or("");
                let fs_file = temp.next().unwrap_or("");
                let fs_vfstype = temp.next().unwrap_or("");
                (fs_spec, fs_file, fs_vfstype)
            })
            .filter(|(_fs_spec, fs_file, fs_vfstype)| {
                // 部分 fs_vfstype 不是我们要找的磁盘信息，去除掉
                let flag = matches!(
                    *fs_vfstype,
                    "rootfs"
                        | "sysfs"
                        | "proc"
                        | "tmpfs"
                        | "devtmpfs"
                        | "cgroup"
                        | "cgroup2"
                        | "pstore"
                        | "squashfs"
                        | "rpc_pipefs"
                        | "iso9660"
                );

                // 部分 fs_file 不是我们要找的磁盘信息，去除掉
                !(flag
                    || fs_file.starts_with("/proc")
                    || fs_file.starts_with("/sys")
                    || fs_file.starts_with("/run")
                    || fs_file.starts_with("sunrpc"))
            })
            .filter_map(|(fs_spec, fs_file, fs_vfstype)| {
                Disk::new(fs_spec, fs_file, fs_vfstype).ok()
            })
            .collect();

        Ok(Self { disks })
    }

    /// Gets disks in `DisksInfo`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::DisksInfo;
    ///
    /// let disks_info = DisksInfo::new().unwrap();
    /// let mut disks = disks_info.disks();
    /// disks.sort();
    /// for disk in disks {
    ///     println!("{:?}", disk);
    /// }
    /// ```
    pub fn disks(&self) -> Vec<Disk> {
        self.disks.clone()
    }

    /// Gets disk information at a given path.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::DisksInfo;
    ///
    /// let disks_info = DisksInfo::new().unwrap();
    /// let disk = disks_info.disk_at("D:\\").unwrap();
    /// println!("{:?}", disk);
    /// ```
    pub fn disk_at<P: AsRef<Path>>(&self, path: P) -> Result<Disk, Error> {
        self.disks()
            .into_iter()
            .find(|disk| Path::new(&disk.mount_point) == path.as_ref())
            .ok_or(Error::Internal(InnerError::NotFound))
    }

    /// 获取到所有磁盘信息的迭代器
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::DisksInfo;
    ///
    /// let disks_info = DisksInfo::new().unwrap();
    /// for disk in disks_info.iter() {
    ///     println!("{:?}", disk);
    /// }
    /// ```
    pub fn iter(&self) -> Iter<Disk> {
        self.disks.iter()
    }

    /// 获取到所有磁盘信息的可变迭代器
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::DisksInfo;
    ///
    /// let mut disks_info = DisksInfo::new().unwrap();
    /// for disk in disks_info.iter_mut() {
    ///     println!("{:?}", disk);
    /// }
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<Disk> {
        self.disks.iter_mut()
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct DiskUsage {
    /// 当前磁盘总空间
    total_space: u64,
    /// 当前磁盘剩余空间
    free_space: u64,
    /// 当前磁盘可用空间
    avail_space: u64,
    /// 当前磁盘已使用空间
    used_space: u64,

    /// 当前磁盘总索引节点数目
    total_inodes: u64,
    /// 当前磁盘剩余索引节点数目
    free_inodes: u64,
    /// 当前磁盘可用索引节点数目
    avail_inodes: u64,
    /// 当前磁盘已使用索引节点数目
    used_inodes: u64,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Disk {
    /// 磁盘设备名称
    device_name: String,
    /// 磁盘所使用文件系统类型
    file_system: String,
    /// 磁盘挂载点
    mount_point: String,
    /// 磁盘使用情况
    disk_usage: DiskUsage,
}

pub(crate) fn checked_mul(a: u64, b: u64) -> Result<u64, Error> {
    if let Some(result) = a.checked_mul(b) {
        return Ok(result);
    }
    Err(Error::Internal(Overflow))
}

pub(crate) fn checked_sub(a: u64, b: u64) -> Result<u64, Error> {
    if let Some(result) = a.checked_sub(b) {
        return Ok(result);
    }
    Err(Error::Internal(Overflow))
}

impl Disk {
    fn new(fs_spec: &str, fs_file: &str, fs_vfstype: &str) -> Result<Self, Error> {
        // 将路径转换为调用 libc 所需的形式
        let path = CString::new(fs_file).unwrap();

        let error;
        let mut buf;
        unsafe {
            buf = std::mem::zeroed();
            error = statvfs(path.as_ptr() as *const _, &mut buf);
        }
        if error == 0 {
            // 所有的单位都为字节 (Bytes)
            let total_space = checked_mul(buf.f_blocks, buf.f_frsize)?;
            let free_space = checked_mul(buf.f_bfree, buf.f_frsize)?;
            let avail_space = checked_mul(buf.f_bavail, buf.f_frsize)?;
            let used_space = checked_sub(total_space, free_space)?;

            let total_inodes = buf.f_files;
            let free_inodes = buf.f_ffree;
            let avail_inodes = buf.f_favail;
            let used_inodes = checked_sub(total_inodes, free_inodes)?;

            return Ok(Self {
                device_name: fs_spec.to_string(),
                file_system: fs_vfstype.to_string(),
                mount_point: fs_file.to_string(),
                disk_usage: DiskUsage {
                    total_space,
                    free_space,
                    avail_space,
                    used_space,
                    total_inodes,
                    free_inodes,
                    avail_inodes,
                    used_inodes,
                },
            });
        }
        Err(Error::Os(OsError::last_os_error()))
    }

    /// 更新当前 Disk 的信息
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::DisksInfo;
    ///
    /// let mut disks_info = DisksInfo::new().unwrap();
    /// for disk in disks_info.iter_mut() {
    ///     disk.update();
    /// }
    /// ```
    pub fn update(&mut self) -> Result<(), Error> {
        let path = CString::new(self.mount_point()).unwrap();

        let error;
        let mut buf;
        unsafe {
            buf = std::mem::zeroed();
            error = statvfs(path.as_ptr() as *const _, &mut buf);
        }

        if error == 0 {
            // 所有的单位都为字节 (Bytes)
            let total_space = checked_mul(buf.f_blocks, buf.f_frsize)?;
            let free_space = checked_mul(buf.f_bfree, buf.f_frsize)?;
            let avail_space = checked_mul(buf.f_bavail, buf.f_frsize)?;
            let used_space = checked_sub(total_space, free_space)?;

            let total_inodes = buf.f_files;
            let free_inodes = buf.f_ffree;
            let avail_inodes = buf.f_favail;
            let used_inodes = checked_sub(total_inodes, free_inodes)?;

            self.disk_usage = DiskUsage {
                total_space,
                free_space,
                avail_space,
                used_space,
                total_inodes,
                free_inodes,
                avail_inodes,
                used_inodes,
            };
            return Ok(());
        }
        Err(Error::Os(OsError::last_os_error()))
    }

    /// 返回当前磁盘设备名字
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::DisksInfo;
    ///
    /// let mut disks_info = DisksInfo::new().unwrap();
    /// let mut name = vec![];
    /// for disk in disks_info.iter() {
    ///     name.push(disk.device_name());
    /// }
    /// ```
    pub fn device_name(&self) -> &str {
        &self.device_name
    }

    /// 返回当前磁盘所使用文件系统类型
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::DisksInfo;
    ///
    /// let mut disks_info = DisksInfo::new().unwrap();
    /// let mut file_system = vec![];
    /// for disk in disks_info.iter() {
    ///     file_system.push(disk.file_system());
    /// }
    /// ```
    pub fn file_system(&self) -> &str {
        &self.file_system
    }

    /// 返回当前磁盘挂载点
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::DisksInfo;
    ///
    /// let mut disks_info = DisksInfo::new().unwrap();
    /// let mut mount_point = vec![];
    /// for disk in disks_info.iter() {
    ///     mount_point.push(disk.mount_point());
    /// }
    /// ```
    pub fn mount_point(&self) -> &str {
        &self.mount_point
    }

    /// 返回当前磁盘总空间
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::DisksInfo;
    ///
    /// let mut disks_info = DisksInfo::new().unwrap();
    /// let mut total_space = vec![];
    /// for disk in disks_info.iter() {
    ///     total_space.push(disk.total_space());
    /// }
    /// ```
    pub fn total_space(&self) -> u64 {
        self.disk_usage.total_space
    }

    /// 返回当前磁盘剩余空间
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::DisksInfo;
    ///
    /// let mut disks_info = DisksInfo::new().unwrap();
    /// let mut free_space = vec![];
    /// for disk in disks_info.iter() {
    ///     free_space.push(disk.free_space());
    /// }
    /// ```
    pub fn free_space(&self) -> u64 {
        self.disk_usage.free_space
    }

    /// 返回当前磁盘可用空间
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::DisksInfo;
    ///
    /// let mut disks_info = DisksInfo::new().unwrap();
    /// let mut avail_space = vec![];
    /// for disk in disks_info.iter() {
    ///     avail_space.push(disk.avail_space());
    /// }
    /// ```
    pub fn avail_space(&self) -> u64 {
        self.disk_usage.avail_space
    }

    /// 返回当前磁盘已使用空间
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::DisksInfo;
    ///
    /// let mut disks_info = DisksInfo::new().unwrap();
    /// let mut used_space = vec![];
    /// for disk in disks_info.iter() {
    ///     used_space.push(disk.used_space());
    /// }
    /// ```
    pub fn used_space(&self) -> u64 {
        self.disk_usage.used_space
    }

    /// 返回当前磁盘总索引节点数目
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::DisksInfo;
    ///
    /// let mut disks_info = DisksInfo::new().unwrap();
    /// let mut total_inodes = vec![];
    /// for disk in disks_info.iter() {
    ///     total_inodes.push(disk.total_inodes());
    /// }
    /// ```
    pub fn total_inodes(&self) -> u64 {
        self.disk_usage.total_inodes
    }

    /// 返回当前磁盘剩余索引节点数目
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::DisksInfo;
    ///
    /// let mut disks_info = DisksInfo::new().unwrap();
    /// let mut free_inodes = vec![];
    /// for disk in disks_info.iter() {
    ///     free_inodes.push(disk.free_inodes());
    /// }
    /// ```
    pub fn free_inodes(&self) -> u64 {
        self.disk_usage.free_inodes
    }

    /// 返回当前磁盘可用索引节点数目
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::DisksInfo;
    ///
    /// let mut disks_info = DisksInfo::new().unwrap();
    /// let mut avail_inodes = vec![];
    /// for disk in disks_info.iter() {
    ///     avail_inodes.push(disk.avail_inodes());
    /// }
    /// ```
    pub fn avail_inodes(&self) -> u64 {
        self.disk_usage.avail_inodes
    }

    /// 返回当前磁盘已使用索引节点数目
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::DisksInfo;
    ///
    /// let mut disks_info = DisksInfo::new().unwrap();
    /// let mut used_inodes = vec![];
    /// for disk in disks_info.iter() {
    ///     used_inodes.push(disk.used_inodes());
    /// }
    /// ```
    pub fn used_inodes(&self) -> u64 {
        self.disk_usage.used_inodes
    }
}

#[cfg(test)]
mod test {
    use crate::DisksInfo;

    /*
     * @test ut_disks_info_test
     * @title  DisksInfo ut 测试用例
     * @brief  1.校验是否能够成功获取 DisksInfo 信息结构
     */
    #[test]
    fn ut_disks_info_test() {
        let disks_info = DisksInfo::new();
        assert!(disks_info.is_ok());
    }

    /*
     * @test ut_disk_test
     * @title  Disk ut 测试用例
     * @brief  1.校验是否能够成功获取 Disk 信息结构
     *         2.校验 Disk 内部数据是否正确
     */
    #[test]
    fn ut_disk_test() {
        let disks_info = DisksInfo::new().unwrap();
        let disks = disks_info.disks;
        assert_ne!(disks.len(), 0);
    }

    /*
     * @test ut_disk_iter_test
     * @title  iter() ut 测试用例
     * @brief  1.校验是否能够成功获取 Disk 的迭代器
     *         2.校验 Disk 内部数据是否正确
     */
    #[test]
    fn ut_disk_iter_test() {
        let disk_info = DisksInfo::new().unwrap();
        for disk in disk_info.iter() {
            assert!(!(disk.device_name.is_empty()));
            assert!(!(disk.file_system.is_empty()));
            assert!(!(disk.mount_point.is_empty()));
        }
    }

    /*
     * @test ut_disk_iter_mut_test
     * @title  iter_mut() ut 测试用例
     * @brief  1.校验是否能够成功获取 Disk 信息结构
     *         2.校验 Disk 内部数据是否正确
     */
    #[test]
    fn ut_disk_iter_mut_test() {
        let mut disk_info = DisksInfo::new().unwrap();
        for disk in disk_info.iter_mut() {
            assert!(!(disk.device_name.is_empty()));
            assert!(!(disk.file_system.is_empty()));
            assert!(!(disk.mount_point.is_empty()));
        }
    }

    /*
     * @test ut_update_test
     * @title  update() ut 测试用例
     * @brief  1.校验是否能够成功更新 Disk 信息结构
     *         2.校验 Disk 内部数据是否正确
     */
    #[test]
    fn ut_update_test() {
        let mut disk_info = DisksInfo::new().unwrap();
        let mut before = vec![];
        for disk in disk_info.iter_mut() {
            before.push(disk.device_name.clone());
        }

        let mut after = vec![];
        for disk in disk_info.iter_mut() {
            let ret = disk.update();
            assert!(ret.is_ok())
        }
        for disk in disk_info.iter_mut() {
            after.push(disk.device_name.clone());
        }
        assert_eq!(before, after);
    }
}
