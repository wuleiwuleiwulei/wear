use crate::error::Error;
use crate::error::InnerError::GetMemoryFailed;
use crate::sys::ffi::{GlobalMemoryStatusEx, MemoryStatusEx};
use std::ffi::c_ulong;
use std::mem::{size_of, zeroed};

pub struct MemoryInfo {
    memory_load: u32,
    total_phys: u64,
    avail_phys: u64,
}

impl MemoryInfo {
    /// Gets information about system memory.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{MemoryInfo, Error};
    ///
    /// let memory_info = MemoryInfo::new()?;
    /// # Ok::<(), Error>(())
    /// ```
    pub fn new() -> Result<Self, Error> {
        let mut mem_info: MemoryStatusEx = unsafe { zeroed() };
        mem_info.dw_length = size_of::<MemoryStatusEx>() as c_ulong;
        if unsafe { GlobalMemoryStatusEx(&mut mem_info) } == 0 {
            return Err(Error::Internal(GetMemoryFailed));
        }
        Ok(Self {
            memory_load: mem_info.dw_memory_load,
            total_phys: mem_info.ull_total_phys,
            avail_phys: mem_info.ull_avail_phys,
        })
    }

    /// Updates information about system memory.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::thread::sleep;
    /// use std::time::Duration;
    /// use ylong_sysinfo::{MemoryInfo, Error};
    ///
    /// let mut memory_info = MemoryInfo::new()?;
    /// sleep(Duration::new(1, 0));
    /// assert!(memory_info.update().is_ok());
    /// # Ok::<(), Error>(())
    /// ```
    pub fn update(&mut self) -> Result<(), Error> {
        let mut mem_info: MemoryStatusEx = unsafe { zeroed() };
        mem_info.dw_length = size_of::<MemoryStatusEx>() as c_ulong;
        if unsafe { GlobalMemoryStatusEx(&mut mem_info) } == 0 {
            return Err(Error::Internal(GetMemoryFailed));
        }
        self.memory_load = mem_info.dw_memory_load;
        self.total_phys = mem_info.ull_total_phys;
        self.avail_phys = mem_info.ull_avail_phys;
        Ok(())
    }

    /// Gets system memory load.
    /// A number between 0 and 100 that specifies the approximate percentage of physical memory that is in use.
    /// (0 indicates no memory use and 100 indicates full memory use).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{MemoryInfo, Error};
    ///
    /// let memory_info = MemoryInfo::new()?;
    /// let memory_load = memory_info.memory_load();
    /// # Ok::<(), Error>(())
    /// ```
    pub fn memory_load(&self) -> u32 {
        self.memory_load
    }

    /// Gets the amount of actual physical memory, in bytes.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{MemoryInfo, Error};
    ///
    /// let memory_info = MemoryInfo::new()?;
    /// let total_phys = memory_info.total_phys();
    /// # Ok::<(), Error>(())
    /// ```
    pub fn total_phys(&self) -> u64 {
        self.total_phys
    }

    /// Gets the amount of physical memory currently available, in bytes.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{MemoryInfo, Error};
    ///
    /// let memory_info = MemoryInfo::new()?;
    /// let avail_phys = memory_info.avail_phys();
    /// # Ok::<(), Error>(())
    /// ```
    pub fn avail_phys(&self) -> u64 {
        self.avail_phys
    }
}

#[cfg(test)]
mod test {
    use crate::MemoryInfo;
    use sysinfo::{System, SystemExt};

    /// UT test for the basic `MemoryInfo`.
    ///
    /// # Title
    /// ut_memory_cross_verify
    ///
    /// # Brief
    /// 1. Gets the basic `MemoryInfo`.
    /// 2. Verifies data value with third-party sysinfo.
    #[test]
    fn ut_memory_cross_verify() {
        let memory_info = MemoryInfo::new().unwrap();
        let total_phys = memory_info.total_phys();

        let mut system = System::new_all();
        system.refresh_memory();
        let total_memory = system.total_memory();
        assert_eq!(total_memory, total_phys);
    }
}
