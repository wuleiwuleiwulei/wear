use crate::error::Error;
use crate::error::InnerError::GetProcessesFailed;
use crate::sys::ffi::{NtQuerySystemInformation, SystemInformationClass, SystemProcessInformation};
use std::collections::HashMap;
use std::ffi::{c_long, c_ulong};

// The specified information record length does not match the length that is required for the specified information class.
// It is used by crossing the complementary code, and c_long is also by official design.
#[allow(overflowing_literals)]
const STATUS_INFO_LENGTH_MISMATCH: c_long = 0xC0000004;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Pid(usize);

impl Pid {
    /// Used to create `Pid` structure.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::Pid;
    ///
    /// let pid = Pid::new(0);
    /// ```
    pub fn new(pid: usize) -> Self {
        Self(pid)
    }
}

pub struct ProcessesInfo {
    processes: HashMap<Pid, Process>,
}

impl ProcessesInfo {
    /// Gets the current system process information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{ProcessesInfo, Error};
    ///
    /// let processes_info = ProcessesInfo::new()?;
    /// # Ok::<(), Error>(())
    /// ```
    pub fn new() -> Result<Self, Error> {
        let processes = HashMap::new();
        let mut result = Self { processes };
        result.update()?;
        Ok(result)
    }

    /// Updates the current system process information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{ProcessesInfo, Error};
    ///
    /// let mut processes_info = ProcessesInfo::new()?;
    /// processes_info.update()?;
    /// # Ok::<(), Error>(())
    /// ```
    pub fn update(&mut self) -> Result<(), Error> {
        // TODO!: Subsequent updates based on the existing state of the process will be considered.
        let mut processes = HashMap::new();

        // Windows 10 notebook requires at least 512KiB of memory to make it in one go.
        let mut buffer_size: usize = 512 * 1024;

        loop {
            let mut process_information = Vec::with_capacity(buffer_size);
            let mut cb_needed = 0;

            unsafe {
                process_information.set_len(buffer_size);
                let status = NtQuerySystemInformation(
                    SystemInformationClass::SystemProcessInformation,
                    process_information.as_mut_ptr(),
                    buffer_size as c_ulong,
                    &mut cb_needed,
                );

                if status != STATUS_INFO_LENGTH_MISMATCH {
                    if status < 0 {
                        return Err(Error::Internal(GetProcessesFailed));
                    }

                    // The number of windows processes in normal use is less than 200.
                    // So here is a little more allocation.
                    let mut process_ids = Vec::with_capacity(300);
                    let mut process_information_offset = 0;
                    loop {
                        let pi = &*(process_information
                            .as_ptr()
                            .offset(process_information_offset)
                            as *const SystemProcessInformation);
                        process_ids.push(pi);
                        if pi.next_entry_offset == 0 {
                            break;
                        }

                        process_information_offset += pi.next_entry_offset as isize;
                    }

                    for process in process_ids {
                        let pid = Pid::new(process.unique_process_id as usize);
                        let name = get_process_name(process, &pid);

                        processes.entry(pid).or_insert_with(|| {
                            let mut process = Process::new();
                            process.set_name(name);
                            process.set_pid(pid);
                            process
                        });
                    }

                    break;
                }

                if cb_needed == 0 {
                    // If the allocated memory is not enough, double.
                    buffer_size *= 2;
                    continue;
                }
                // After calling `NtQuerySystemInformation`, allocate a few kilobytes in case some new process in.
                buffer_size = (cb_needed + (1024 * 10)) as usize;
            }
        }

        self.processes = processes;
        Ok(())
    }

    /// Returns a hash table of current process information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{ProcessesInfo, Error};
    ///
    /// let processes_info = ProcessesInfo::new()?;
    /// let processes = processes_info.processes();
    /// # Ok::<(), Error>(())
    /// ```
    pub fn processes(&self) -> &HashMap<Pid, Process> {
        &self.processes
    }

    /// Returns an vector of process containing the given `name`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{ProcessesInfo, Error};
    ///
    /// let processes_info = ProcessesInfo::new()?;
    /// let processes = processes_info.processes_by_name("IDLE");
    /// # Ok::<(), Error>(())
    /// ```
    pub fn processes_by_name(&self, name: &str) -> Vec<&Process> {
        self.processes()
            .values()
            .filter(|value| value.name.contains(name))
            .collect::<Vec<&Process>>()
    }

    /// Returns an vector of processes with exactly the given `name`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{ProcessesInfo, Error};
    ///
    /// let processes_info = ProcessesInfo::new()?;
    /// let processes = processes_info.processes_by_exact_name("IDLE");
    /// # Ok::<(), Error>(())
    /// ```
    pub fn processes_by_exact_name(&self, name: &str) -> Vec<&Process> {
        self.processes()
            .values()
            .filter(|value| value.name == name)
            .collect::<Vec<&Process>>()
    }

    /// Finds the corresponding process information based on the pid.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{Pid, ProcessesInfo, Error};
    ///
    /// let processes_info = ProcessesInfo::new()?;
    /// let pid = Pid::new(0);
    /// let process = processes_info.process(&pid).unwrap();
    /// # Ok::<(), Error>(())
    /// ```
    pub fn process(&self, pid: &Pid) -> Option<&Process> {
        self.processes.get(pid)
    }
}

#[derive(Debug)]
pub struct Process {
    name: String,
    pid: Pid,
}

impl Process {
    fn new() -> Self {
        Self {
            name: "".to_string(),
            pid: Pid(0),
        }
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn set_pid(&mut self, pid: Pid) {
        self.pid = pid;
    }

    /// Gets the name of the current process.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{Pid, ProcessesInfo, Error};
    ///
    /// let processes_info = ProcessesInfo::new()?;
    /// let pid = Pid::new(0);
    /// let process = processes_info.process(&pid).unwrap();
    /// let name = process.name();
    /// # Ok::<(), Error>(())
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Gets the pid of the current process.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{Pid, ProcessesInfo, Error};
    ///
    /// let processes_info = ProcessesInfo::new()?;
    /// let pid = Pid::new(0);
    /// let process = processes_info.process(&pid).unwrap();
    /// let pid = process.pid();
    /// # Ok::<(), Error>(())
    /// ```
    pub fn pid(&self) -> &Pid {
        &self.pid
    }
}

fn get_process_name(process: &SystemProcessInformation, pid: &Pid) -> String {
    let image_name = &process.image_name;
    if image_name.buffer.is_null() {
        match pid.0 {
            0 => "Idle".to_string(),
            4 => "System".to_string(),
            _ => format!("<no name> Process {pid:?}"),
        }
    } else {
        unsafe {
            let name = std::slice::from_raw_parts(
                image_name.buffer,
                image_name.length as usize / std::mem::size_of::<u16>(),
            );

            String::from_utf16_lossy(name)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Pid, ProcessesInfo};

    /// UT test for the basic `ProcessesInfo`.
    ///
    /// # Title
    /// ut_processes_info_new
    ///
    /// # Brief
    /// 1. Gets the basic `ProcessesInfo`.
    /// 2. Verifies data value.
    #[test]
    fn ut_processes_info_new() {
        let processes_info = ProcessesInfo::new().unwrap();
        let processes = processes_info.processes();
        // PID 0 is the System Idle Process.
        assert_eq!(processes.get(&Pid::new(0)).unwrap().name(), "Idle");
    }
}
