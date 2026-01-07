use crate::error::{find_error, read_error, Error};
use crate::linux::process::*;
use std::str::FromStr;

#[derive(Clone)]
pub struct ProcessMemoryInfo {
    occupied_mem: u64,
    physical_mem: u64,
    swap_mem: u64,
}

impl ReadFrom for ProcessMemoryInfo {}

impl ProcessMemoryInfo {
    /// Gets memory data by pid under process.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::ProcessMemoryInfo;
    ///
    /// let mut process_memory_info = ProcessMemoryInfo::from_pid(1).unwrap();
    /// ```
    pub fn from_pid(pid: u64) -> Result<ProcessMemoryInfo, Error> {
        let proc_mem_info_map = ProcessMemoryInfo::read_proc_file_full_content(&pid, "status")?;
        let mut value_vec = Vec::new();
        let mut nofind_vec = Vec::new();
        let to_find = ["VmSize", "VmRSS", "VmSwap"];

        for &key in &to_find {
            match proc_mem_info_map.get(key) {
                Some(content) => {
                    let content_value = content
                        .split_whitespace()
                        .collect::<Vec<&str>>()
                        .first()
                        .map(|c| c.trim().to_string())
                        .unwrap_or_default();
                    let value_u64 = u64::from_str(&content_value)
                        .map_err(|e| read_error("proc", "memory", e))?;
                    value_vec.push(value_u64);
                }
                None => {
                    nofind_vec.push(key);
                }
            }
        }

        if !nofind_vec.is_empty() {
            return Err(find_error(&nofind_vec, "status"));
        }
        let mut process_mem = ProcessMemoryInfo {
            occupied_mem: 0,
            physical_mem: 0,
            swap_mem: 0,
        };

        process_mem.swap_mem = value_vec.pop().unwrap().wrapping_mul(1024);
        process_mem.physical_mem = value_vec.pop().unwrap().wrapping_mul(1024);
        process_mem.occupied_mem = value_vec.pop().unwrap().wrapping_mul(1024);

        Ok(process_mem)
    }
}

#[derive(Clone)]
pub struct MemoryInfo {
    total_phys: u64,
    avail_phys: u64,
}

impl ReadFrom for MemoryInfo {}

impl MemoryInfo {
    /// Gets information about system memory.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::MemoryInfo;
    ///
    /// let mut sys_mem_info = MemoryInfo::new().unwrap();
    /// ```
    pub fn new() -> Result<MemoryInfo, Error> {
        let sys_mem_info_map = MemoryInfo::read_sys_file_content("meminfo")?;
        let mut value_vec = Vec::new();
        let mut nofind_vec = Vec::new();
        let to_find = ["MemTotal", "MemAvailable"];
        for &key in &to_find {
            match sys_mem_info_map.get(key) {
                Some(content) => {
                    let content_value = content
                        .split(':')
                        .collect::<Vec<&str>>()
                        .get(1)
                        .map(|c| c.split_whitespace().collect::<Vec<&str>>())
                        .and_then(|v| v.first().cloned())
                        .map(|s| s.to_string())
                        .unwrap_or_default();
                    let value_u64 = u64::from_str(&content_value)
                        .map_err(|e| read_error("sys", "memory", e))?;

                    value_vec.push(value_u64);
                }
                None => {
                    nofind_vec.push(key);
                }
            }
        }

        if !nofind_vec.is_empty() {
            return Err(find_error(&nofind_vec, "meminfo"));
        }

        let mut sys_mem = MemoryInfo {
            total_phys: 0,
            avail_phys: 0,
        };
        sys_mem.avail_phys = value_vec.pop().unwrap().wrapping_mul(1024);
        sys_mem.total_phys = value_vec.pop().unwrap().wrapping_mul(1024);

        Ok(sys_mem)
    }

    /// Gets the amount of actual physical memory, in bytes.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::MemoryInfo;
    ///
    /// let memory_info = MemoryInfo::new().unwrap();
    /// let total_phys = memory_info.total_phys();
    /// ```
    pub fn total_phys(&self) -> u64 {
        self.total_phys
    }

    /// Gets the amount of physical memory currently available, in bytes.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::MemoryInfo;
    ///
    /// let memory_info = MemoryInfo::new().unwrap();
    /// let avail_phys = memory_info.avail_phys();
    /// ```
    pub fn avail_phys(&self) -> u64 {
        self.avail_phys
    }
}

#[cfg(test)]
mod ut_memory {
    use crate::error::Error;
    use crate::linux::memory::{MemoryInfo, ProcessMemoryInfo};
    use crate::linux::process::ReadFrom;
    use std::collections::HashMap;
    use ylong_mock::stub_sync_func;

    /*
     * @test   ut_process_mem_file_block
     * @title  from_pid UT test
     * @brief  Set hashmap output to test whether fn can get correct content
     */
    #[test]
    fn ut_process_mem_file_block() {
        fn mock_read_proc_file_full_content(
            _pid: &u64,
            _file: &str,
        ) -> Result<HashMap<String, String>, Error> {
            let mut content_name_map = HashMap::new();
            content_name_map.insert("VmSize".to_string(), "226524 kB".to_string());
            content_name_map.insert("VmRSS".to_string(), "5320 kB".to_string());
            content_name_map.insert("VmSwap".to_string(), "0 kB".to_string());
            Ok(content_name_map)
        }

        let mock = stub_sync_func!(
            ProcessMemoryInfo::read_proc_file_full_content,
            mock_read_proc_file_full_content
        );
        let mock = mock.unwrap();
        let res = ProcessMemoryInfo::from_pid(1).unwrap();
        mock.stub_remove();
        assert_eq!(res.occupied_mem, 226524_u64.wrapping_mul(1024));
        assert_eq!(res.physical_mem, 5320_u64.wrapping_mul(1024));
        assert_eq!(res.swap_mem, 0);
    }

    /*
     * @test   ut_sys_mem_file_block
     * @title  update_sys_mem_file_info UT test
     * @brief  Set hashmap output to test whether fn can get correct content
     */
    #[test]
    fn ut_sys_mem_file_block() {
        fn mock_read_sys_file_content(_file: &str) -> Result<HashMap<String, String>, Error> {
            let mut content_name_map = HashMap::new();
            content_name_map.insert("MemTotal".to_string(), "MemTotal: 8166416 kB".to_string());
            content_name_map.insert(
                "MemAvailable".to_string(),
                "MemAvailable: 2856012 kB".to_string(),
            );
            Ok(content_name_map)
        }

        let mock = stub_sync_func!(
            MemoryInfo::read_sys_file_content,
            mock_read_sys_file_content
        );
        let mock = mock.unwrap();
        let res = MemoryInfo::new().unwrap();
        mock.stub_remove();
        assert_eq!(res.total_phys, 8166416_u64.wrapping_mul(1024));
        assert_eq!(res.avail_phys, 2856012_u64.wrapping_mul(1024));
    }
}
