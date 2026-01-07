use crate::error::Error;
use crate::error::InnerError::{AddCounterFailed, CreateQueryFailed, UpdateQueryFailed};
use crate::sys::ffi::{
    GetSystemInfo, PdhAddEnglishCounterW, PdhFmtCounterValue, PdhGetFormattedCounterValue,
    PdhOpenQueryA, SystemInfo,
};
use crate::windows::ffi::{PdhCloseQuery, PdhCollectQueryData, PdhRemoveCounter};
use std::ffi::{c_ulong, c_void};
use std::mem::zeroed;
use std::ptr::null_mut;

const ERROR_SUCCESS: i32 = 0;
const PDH_FMT_LONG: c_ulong = 0x00000100;

struct Query(*mut c_void);

impl Query {
    fn new() -> Result<Self, Error> {
        let mut query = null_mut();
        unsafe {
            if PdhOpenQueryA(null_mut(), 0, &mut query) == ERROR_SUCCESS {
                Ok(Self(query))
            } else {
                Err(Error::Internal(CreateQueryFailed))
            }
        }
    }
}

impl Drop for Query {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                PdhCloseQuery(self.0);
            }
        }
    }
}

pub struct CpusInfo {
    query: Query,
    global_cpu: Cpu,
    cpus: Vec<Cpu>,
}

impl CpusInfo {
    /// Gets the basic information of the current CPU.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{CpusInfo, Error};
    ///
    /// let cpus_info = CpusInfo::new()?;
    /// # Ok::<(), Error>(())
    /// ```
    pub fn new() -> Result<Self, Error> {
        let query = Query::new()?;

        let mut sys_info: SystemInfo = unsafe { zeroed() };
        unsafe {
            GetSystemInfo(&mut sys_info);
        }
        let number = sys_info.dw_number_of_processors as usize;

        let mut counter: *mut c_void = unsafe { zeroed() };
        let mut sz_full_counter_path = r"\Processor(_Total)\% Processor Time"
            .encode_utf16()
            .collect::<Vec<_>>();
        sz_full_counter_path.push(0);
        unsafe {
            if PdhAddEnglishCounterW(query.0, sz_full_counter_path.as_ptr(), 0, &mut counter)
                != ERROR_SUCCESS
            {
                return Err(Error::Internal(AddCounterFailed));
            }
        }
        let global_cpu = Cpu::new(counter);

        let mut cpus = Vec::with_capacity(number);
        for index in 0..number {
            let mut counter: *mut c_void = unsafe { zeroed() };
            let mut sz_full_counter_path = format!(r"\Processor({index})\% Processor Time")
                .encode_utf16()
                .collect::<Vec<_>>();
            sz_full_counter_path.push(0);
            unsafe {
                if PdhAddEnglishCounterW(query.0, sz_full_counter_path.as_ptr(), 0, &mut counter)
                    != ERROR_SUCCESS
                {
                    return Err(Error::Internal(AddCounterFailed));
                }
            }
            cpus.push(Cpu::new(counter));
        }

        unsafe {
            if PdhCollectQueryData(query.0) != ERROR_SUCCESS {
                return Err(Error::Internal(UpdateQueryFailed));
            }
        }

        Ok(Self {
            query,
            global_cpu,
            cpus,
        })
    }

    /// Updates all CPU information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{CpusInfo, Error};
    ///
    /// let mut cpus_info = CpusInfo::new()?;
    /// cpus_info.update_all()?;
    /// # Ok::<(), Error>(())
    /// ```
    pub fn update_all(&mut self) -> Result<(), Error> {
        unsafe {
            if PdhCollectQueryData(self.query.0) != ERROR_SUCCESS {
                return Err(Error::Internal(UpdateQueryFailed));
            }
        }

        Ok(())
    }

    /// Gets the reference of global cpu.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{CpusInfo, Error};
    ///
    /// let mut cpus_info = CpusInfo::new()?;
    /// cpus_info.update_all()?;
    /// let global_cpu = cpus_info.global_cpu();
    /// # Ok::<(), Error>(())
    /// ```
    pub fn global_cpu(&self) -> &Cpu {
        &self.global_cpu
    }

    /// Gets the reference of cpus.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{CpusInfo, Error};
    ///
    /// let mut cpus_info = CpusInfo::new()?;
    /// cpus_info.update_all()?;
    /// let cpus = cpus_info.cpus();
    /// # Ok::<(), Error>(())
    /// ```
    pub fn cpus(&self) -> &[Cpu] {
        &self.cpus
    }
}

impl Drop for CpusInfo {
    fn drop(&mut self) {
        unsafe {
            PdhRemoveCounter(self.global_cpu.counter);
            for cpu in &self.cpus {
                PdhRemoveCounter(cpu.counter);
            }
        }
    }
}

pub struct Cpu {
    counter: *mut c_void,
}

impl Cpu {
    fn new(counter: *mut c_void) -> Self {
        Self { counter }
    }

    /// Gets the current CPU usage (Ten Thousand Ratio).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{CpusInfo, Error};
    ///
    /// let mut cpus_info = CpusInfo::new()?;
    /// cpus_info.update_all()?;
    /// let global_cpu = cpus_info.global_cpu();
    /// let usage = global_cpu.usage();
    /// # Ok::<(), Error>(())
    /// ```
    pub fn usage(&self) -> u16 {
        unsafe {
            let mut value = std::mem::MaybeUninit::<PdhFmtCounterValue>::uninit();
            let ret = PdhGetFormattedCounterValue(
                self.counter,
                PDH_FMT_LONG,
                null_mut(),
                value.as_mut_ptr(),
            );
            let value = value.assume_init();
            if ret == ERROR_SUCCESS {
                let mut usage = value.u.long_value.checked_mul(100).unwrap();
                if usage > 10000 {
                    usage = 10000;
                }
                return usage as u16;
            }
        }

        0
    }
}

#[cfg(test)]
mod test {
    use crate::CpusInfo;
    use std::thread::sleep;
    use std::time::Duration;
    use sysinfo::{System, SystemExt};

    /// UT test for get cpus information.
    ///
    /// # Title
    /// ut_cpus_info_new
    ///
    /// # Brief
    /// 1. Cross-validate with the third-party library sys-info.
    /// 2. The data obtained on the same device should be the same.
    #[test]
    fn ut_cpus_info_new() {
        let cpus_info = CpusInfo::new().unwrap();
        let num_one = cpus_info.cpus().len();

        let system = System::new_all();
        let num_two = system.cpus().len();
        assert_eq!(num_one, num_two);
    }

    /// UT test for the basic `CpusInfo` and update it.
    ///
    /// # Title
    /// ut_cpus_info_update_all
    ///
    /// # Brief
    /// 1. Gets the basic `CpusInfo` and update it.
    /// 2. Verifies data value changes.
    #[test]
    fn ut_cpus_info_update_all() {
        let mut cpus_info = CpusInfo::new().unwrap();
        let usage = cpus_info.global_cpu().usage();
        assert_eq!(usage, 0);

        sleep(Duration::from_millis(500));
        cpus_info.update_all().unwrap();
        let usage = cpus_info.global_cpu().usage();
        assert_ne!(usage, 0);
    }
}
