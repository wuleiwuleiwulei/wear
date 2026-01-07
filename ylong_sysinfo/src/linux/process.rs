use crate::error::InnerError::{LenSizeError, ReadError};
use crate::error::{find_error, read_error, Error};
use libc::getpid;
use std::collections::HashMap;
use std::str::FromStr;
use std::{fs, io};

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
    /// use ylong_sysinfo::ProcessesInfo;
    ///
    /// let processes_info = ProcessesInfo::new().unwrap();
    /// ```
    pub fn new() -> Result<Self, Error> {
        let mut processes = HashMap::new();

        let pids = get_pids();
        for pid in pids {
            let process = Process::from_pid(u64::from_str(&pid).unwrap())?;
            let pid = Pid::new(usize::from_str(&pid).unwrap());
            processes.entry(pid).or_insert(process);
        }

        Ok(Self { processes })
    }

    /// Updates the current system process information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::ProcessesInfo;
    ///
    /// let mut processes_info = ProcessesInfo::new().unwrap();
    /// processes_info.update().unwrap();
    /// ```
    pub fn update(&mut self) -> Result<(), Error> {
        // TODO!: Subsequent updates based on the existing state of the process will be considered.
        let mut processes = HashMap::new();

        let pids = get_pids();
        for pid in pids {
            let process = Process::from_pid(u64::from_str(&pid).unwrap())?;
            let pid = Pid::new(usize::from_str(&pid).unwrap());
            processes.entry(pid).or_insert(process);
        }

        self.processes = processes;
        Ok(())
    }

    /// Returns a hash table of current process information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::ProcessesInfo;
    ///
    /// let processes_info = ProcessesInfo::new().unwrap();
    /// let processes = processes_info.processes();
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
    /// use ylong_sysinfo::{Pid, ProcessesInfo};
    ///
    /// let processes_info = ProcessesInfo::new().unwrap();
    /// let pid = Pid::new(0);
    /// let process = processes_info.process(&pid).unwrap();
    /// ```
    pub fn process(&self, pid: &Pid) -> Option<&Process> {
        self.processes.get(pid)
    }
}

#[derive(Clone)]
pub struct StatusFile {
    ppid: u64,
    threads: u64,
    uid: u64,
    gid: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Process {
    pid: Pid,
    name: String,
    state: Status,
    ppid: u64,
    utime: u64,
    stime: u64,
    cutime: u64,
    cstime: u64,
    start_time: u64,
    total_time: u64,
}

#[derive(Clone)]
pub struct Limits {
    max_stack_size: u64,
    max_processes: u64,
    max_open_files: u64,
}

#[derive(Clone)]
pub struct Cgroup {
    pid_cgroup: String,
}
// https://man7.org/linux/man-pages/man5/procfs.5.html
// Idle Waiting in uninterruptible disk sleep.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Dead,
    Idle,
    Parked,
    Running,
    Sleeping,
    Stopped,
    Tracing,
    Waiting,
    Wakekill,
    Waking,
    Zombie,
}

impl TryFrom<char> for Status {
    type Error = Error;
    fn try_from(value: char) -> Result<Status, Error> {
        match value {
            'D' => Ok(Status::Waiting),
            'I' => Ok(Status::Idle),
            'K' => Ok(Status::Wakekill),
            'P' => Ok(Status::Parked),
            'R' => Ok(Status::Running),
            'S' => Ok(Status::Sleeping),
            'T' => Ok(Status::Stopped),
            't' => Ok(Status::Tracing),
            'W' => Ok(Status::Waking),
            'X' | 'x' => Ok(Status::Dead),
            'Z' => Ok(Status::Zombie),
            _ => Err(Error::Internal(ReadError(format!(
                "Unknown process state {value}"
            )))),
        }
    }
}

pub(crate) trait ReadFrom {
    fn read_proc_file(pid: &u64, file: &str) -> Result<String, Error> {
        let file_path = format!("/proc/{pid}/{file}");
        fs::read_to_string(file_path).map_err(|e| read_error("file", &pid.to_string(), e))
    }

    fn read_sys_file(file: &str) -> Result<String, Error> {
        let file_path = format!("/proc/{file}");
        fs::read_to_string(file_path).map_err(|e| read_error("System", file, e))
    }

    fn read_proc_file_full_content(
        pid: &u64,
        file: &str,
    ) -> Result<HashMap<String, String>, Error> {
        let content = Self::read_proc_file(pid, file)?;
        let mut content_name_map = HashMap::new();
        let content_lines = content.lines();
        for line in content_lines {
            let line_vec = line.split(':').collect::<Vec<&str>>();
            let line_key = line_vec
                .first()
                .map(|c| c.trim().to_string())
                .unwrap_or_default();
            let line_value = line_vec
                .get(1)
                .map(|c| c.trim().to_string())
                .unwrap_or_default();
            content_name_map.insert(line_key, line_value);
        }
        Ok(content_name_map)
    }

    fn read_sys_file_content(file: &str) -> Result<HashMap<String, String>, Error> {
        let content = Self::read_sys_file(file)?;
        let mut content_name_map = HashMap::new();
        let mut content_lines = content.lines();
        let total_mem = content_lines
            .find(|iter| iter.contains("MemTotal:"))
            .map(|iter| iter.to_string())
            .unwrap();
        let _free_mem = content_lines
            .find(|iter| iter.contains("MemFree:"))
            .map(|iter| iter.to_string())
            .unwrap();
        let avail_mem = content_lines
            .find(|iter| iter.contains("MemAvailable:"))
            .map(|iter| iter.to_string())
            .unwrap();
        content_name_map.insert("MemTotal".to_string(), total_mem);
        content_name_map.insert("MemAvailable".to_string(), avail_mem);
        Ok(content_name_map)
    }
}

impl ReadFrom for StatusFile {}

impl StatusFile {
    /// get status data by pid under process
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::StatusFile;
    ///
    /// let mut status_file_info = StatusFile::from_pid(1).unwrap();
    /// ```
    pub fn from_pid(pid: u64) -> Result<StatusFile, Error> {
        let status_info_map = StatusFile::read_proc_file_full_content(&pid, "status")?;
        let mut value_vec = Vec::new();
        let mut nofind_vec = Vec::new();
        let to_find = ["PPid", "Threads"];
        for &key in &to_find {
            match status_info_map.get(key) {
                Some(content) => {
                    let value_u64 =
                        u64::from_str(content).map_err(|e| read_error("status", key, e))?;
                    value_vec.push(value_u64);
                }
                None => {
                    nofind_vec.push(key);
                }
            }
        }
        let mut id_vec = Vec::new();
        let to_find_id = ["Uid", "Gid"];
        for &key in &to_find_id {
            match status_info_map.get(key) {
                Some(content) => {
                    let content_value = content
                        .split('\t')
                        .collect::<Vec<&str>>()
                        .first()
                        .map(|c| c.trim().to_string())
                        .unwrap_or_default();
                    let value_u64 =
                        u64::from_str(&content_value).map_err(|e| read_error("status", key, e))?;
                    id_vec.push(value_u64);
                }
                None => {
                    nofind_vec.push(key);
                }
            }
        }
        if !nofind_vec.is_empty() {
            return Err(find_error(&nofind_vec, "status"));
        }

        let mut status_file = StatusFile {
            ppid: 0,
            threads: 0,
            uid: 0,
            gid: 0,
        };
        status_file.ppid = value_vec[0];
        status_file.threads = value_vec[1];
        status_file.uid = id_vec[0];
        status_file.gid = id_vec[1];

        Ok(status_file)
    }
}

impl ReadFrom for Process {}

impl Process {
    /// get stat data by pid under process
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::Process;
    ///
    /// let mut stat_info = Process::from_pid(1).unwrap();
    /// ```
    pub fn from_pid(pid: u64) -> Result<Process, Error> {
        let content = Process::read_proc_file(&pid, "stat")?;
        let parts: Vec<&str> = content.splitn(2, ' ').collect();
        if parts.len() < 2 {
            return Err(Error::Internal(LenSizeError));
        }

        let mut stat = Process {
            pid: Pid::new(0),
            name: "default".to_string(),
            state: Status::Sleeping,
            ppid: 0,
            utime: 0,
            stime: 0,
            cutime: 0,
            cstime: 0,
            start_time: 0,
            total_time: 0,
        };

        stat.pid = Pid::new(usize::from_str(parts[0]).map_err(|e| read_error("pid", parts[0], e))?);
        let leftover = parts[1];
        let comm_end = leftover
            .rfind(')')
            .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidData))
            .map_err(|e| read_error(")", parts[0], e))?;

        stat.name = leftover[1..comm_end].to_string();
        let parts: Vec<&str> = leftover[comm_end + 2..].split_whitespace().collect();
        if parts.len() < 20 {
            return Err(Error::Internal(ReadError(format!(
                "Parts for status and ppid len less than 20, stat file is {}.",
                &content
            ))));
        }

        let status_info =
            char::from_str(parts[0]).map_err(|e| read_error("status", parts[0], e))?;
        stat.state = Status::try_from(status_info)?;
        stat.ppid = u64::from_str(parts[1]).map_err(|e| read_error("ppid", parts[1], e))?;

        stat.utime = u64::from_str(parts[11]).map_err(|e| read_error("utime", parts[11], e))?;

        stat.stime = u64::from_str(parts[12]).map_err(|e| read_error("stime", parts[12], e))?;

        stat.cutime = u64::from_str(parts[13]).map_err(|e| read_error("cutime", parts[13], e))?;

        stat.cstime = u64::from_str(parts[14]).map_err(|e| read_error("cstime", parts[14], e))?;

        stat.start_time =
            u64::from_str(parts[19]).map_err(|e| read_error("start_time", parts[19], e))?;

        stat.total_time = stat.utime + stat.stime + stat.cutime + stat.cstime;
        Ok(stat)
    }

    /// Gets the name of the current process.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{Pid, ProcessesInfo};
    ///
    /// let processes_info = ProcessesInfo::new().unwrap();
    /// let pid = Pid::new(0);
    /// let process = processes_info.process(&pid).unwrap();
    /// let name = process.name();
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Gets the pid of the current process.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{Pid, ProcessesInfo};
    ///
    /// let processes_info = ProcessesInfo::new().unwrap();
    /// let pid = Pid::new(0);
    /// let process = processes_info.process(&pid).unwrap();
    /// let pid = process.pid();
    /// ```
    pub fn pid(&self) -> &Pid {
        &self.pid
    }
}

impl ReadFrom for Limits {
    fn read_proc_file_full_content(
        pid: &u64,
        file: &str,
    ) -> Result<HashMap<String, String>, Error> {
        let content = Self::read_proc_file(pid, file)?;
        let mut content_name_map = HashMap::new();
        let mut content_lines = content.lines();
        let max_stack_size_content = content_lines
            .find(|iter| iter.contains("Max stack size"))
            .map(|iter| iter.to_string())
            .unwrap();
        let max_process_content = content_lines
            .find(|iter| iter.contains("Max processes"))
            .map(|iter| iter.to_string())
            .unwrap();
        let max_file_content = content_lines
            .find(|iter| iter.contains("Max open files"))
            .map(|iter| iter.to_string())
            .unwrap();
        content_name_map.insert("max_stack_size".to_string(), max_stack_size_content);
        content_name_map.insert("max_processes".to_string(), max_process_content);
        content_name_map.insert("max_open_files".to_string(), max_file_content);
        Ok(content_name_map)
    }
}

impl Limits {
    /// get limits data by pid under process
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::Limits;
    ///
    /// let mut limits_info = Limits::from_pid(1).unwrap();
    /// ```
    pub fn from_pid(pid: u64) -> Result<Limits, Error> {
        let limits_info_map = Limits::read_proc_file_full_content(&pid, "limits")?;
        let mut value_vec = Vec::new();
        let mut nofind_vec = Vec::new();
        let to_find = ["max_open_files", "max_processes", "max_stack_size"];
        for &key in &to_find {
            let index = key.split('_').count();
            match limits_info_map.get(key) {
                Some(content) => {
                    let content_value = content
                        .split_whitespace()
                        .collect::<Vec<&str>>()
                        .get(index)
                        .map(|c| c.trim().to_string())
                        .unwrap_or_default();
                    let value_u64 = u64::from_str(&content_value)
                        .map_err(|e| read_error("limits", &pid.to_string(), e))?;

                    value_vec.push(value_u64);
                }
                None => {
                    nofind_vec.push(key);
                }
            }
        }
        if !nofind_vec.is_empty() {
            return Err(find_error(&nofind_vec, "limits"));
        }

        let mut limits_flie = Limits {
            max_stack_size: 0,
            max_processes: 0,
            max_open_files: 0,
        };
        limits_flie.max_open_files = value_vec[0];
        limits_flie.max_processes = value_vec[1];
        limits_flie.max_stack_size = value_vec[2];

        Ok(limits_flie)
    }
}

impl ReadFrom for Cgroup {
    fn read_proc_file_full_content(
        pid: &u64,
        file: &str,
    ) -> Result<HashMap<String, String>, Error> {
        let content = Self::read_proc_file(pid, file)?;
        let mut content_name_map = HashMap::new();
        let mut content_lines = content.lines();
        let pid_cgroup_content = content_lines
            .find(|iter| iter.contains(":pids:"))
            .map(|iter| iter.to_string())
            .unwrap();
        content_name_map.insert("pid_cgroup".to_string(), pid_cgroup_content);
        Ok(content_name_map)
    }
}

impl Cgroup {
    /// get cgroup data by pid under process
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::Cgroup;
    ///
    /// let mut cgroup_info = Cgroup::from_pid(1).unwrap();
    /// ```
    pub fn from_pid(pid: u64) -> Result<Cgroup, Error> {
        let cgroup_info_map = Cgroup::read_proc_file_full_content(&pid, "cgroup")?;
        let mut value_vec = Vec::new();
        let mut nofind_vec = Vec::new();
        let to_find = ["pid_cgroup"];
        for &key in &to_find {
            match cgroup_info_map.get(key) {
                Some(content) => {
                    let content_value = content
                        .split(":pids:")
                        .collect::<Vec<&str>>()
                        .get(1)
                        .map(|c| c.split('/').collect::<Vec<&str>>())
                        .and_then(|v| v.last().cloned())
                        .map(|s| s.to_string())
                        .unwrap_or_default();
                    value_vec.push(content_value);
                }
                None => {
                    nofind_vec.push(key);
                }
            }
        }
        if !nofind_vec.is_empty() {
            return Err(find_error(&nofind_vec, "cgroup"));
        }

        let mut cgroup_file = Cgroup {
            pid_cgroup: "default".to_string(),
        };
        cgroup_file.pid_cgroup = value_vec.pop().unwrap();
        Ok(cgroup_file)
    }
}

/// get all pids
///
/// # Examples
///
/// ```no_run
/// use ylong_sysinfo::get_pids;
///
/// let all_pids = get_pids();
/// ```
pub fn get_pids() -> Vec<String> {
    let mut pids = vec![];
    for entry in fs::read_dir("/proc").unwrap() {
        let file_name = entry.as_ref().unwrap().file_name();
        let name_to_string = file_name.into_string().unwrap();
        let first_pos = unsafe { *name_to_string.as_ptr().add(0) as char };
        if !first_pos.is_numeric() {
            continue;
        }
        pids.push(name_to_string);
    }
    pids
}

/// get current pid
///
/// # Examples
///
/// ```no_run
/// use ylong_sysinfo::get_current_pid;
///
/// let current_pid = get_current_pid();
/// ```
pub fn get_current_pid() -> i32 {
    let current_pid: i32 = unsafe { getpid() };
    current_pid
}

#[cfg(test)]
mod ut_process {
    use crate::error::Error;
    use crate::sys::process::Status::Sleeping;
    use crate::sys::process::{Cgroup, Limits, Pid, Process, ReadFrom, StatusFile};
    use std::collections::HashMap;
    use ylong_mock::stub_sync_func;

    /*
     * @test   ut_process_status_file_block
     * @title  from_pid UT test
     * @brief  Set hashmap output to test whether fn can get correct content
     */
    #[test]
    fn ut_process_status_file_block() {
        fn mock_read_proc_file_full_content(
            _pid: &u64,
            _file: &str,
        ) -> Result<HashMap<String, String>, Error> {
            let mut content_name_map = HashMap::new();
            content_name_map.insert("PPid".to_string(), "2".to_string());
            content_name_map.insert("Threads".to_string(), "1".to_string());
            content_name_map.insert("Uid".to_string(), "0".to_string());
            content_name_map.insert("Gid".to_string(), "0".to_string());
            Ok(content_name_map)
        }

        let mock = stub_sync_func!(
            StatusFile::read_proc_file_full_content,
            mock_read_proc_file_full_content
        );
        let mock = mock.unwrap();
        let res = StatusFile::from_pid(1).unwrap();
        mock.stub_remove();
        assert_eq!(res.ppid, 2);
        assert_eq!(res.threads, 1);
        assert_eq!(res.uid, 0);
        assert_eq!(res.gid, 0);
    }

    /*
     * @test   ut_process_stat_block
     * @title  from_pid UT test
     * @brief  Set readfile output to test whether fn can get correct content
     */
    #[test]
    fn ut_process_stat_block() {
        fn mock_read_proc_file(_pid: &u64, _file: &str) -> Result<String, Error> {
            Ok("1 (systemd) S 0 1 1 0 -1 4194560 380340 39032217944 23952 19571924 2350 4377 444947274 58018846 20 0 1 0 0 231960576 1170 18446744073709551615 93875271913472 93875273288024 140735533252432 0 0 0 671173123 4096 1260 1 0 0 17 2 0 0 17768 0 0 93875275386128 93875275624768 93875304677376 140735533256464 140735533256508 140735533256508 140735533256685 0".to_string()
            )
        }
        let mock = stub_sync_func!(Process::read_proc_file, mock_read_proc_file);
        let mock = mock.unwrap();
        let res = Process::from_pid(1).unwrap();
        let expect_res = Process {
            pid: Pid::new(1),
            name: "systemd".to_string(),
            state: Sleeping,
            ppid: 0,
            utime: 2350,
            stime: 4377,
            cutime: 444947274,
            cstime: 58018846,
            start_time: 0,
            total_time: 502972847,
        };
        mock.stub_remove();
        assert_eq!(res, expect_res);
    }

    /*
     * @test   ut_process_limits_block
     * @title  from_pid UT test
     * @brief  Set hashmap output to test whether fn can get correct content
     */
    #[test]
    fn ut_process_limits_block() {
        fn mock_read_proc_file_full_content(
            _pid: &u64,
            _file: &str,
        ) -> Result<HashMap<String, String>, Error> {
            let mut content_name_map = HashMap::new();
            content_name_map.insert(
                "max_stack_size".to_string(),
                "Max stack size 8388608 unlimited bytes".to_string(),
            );
            content_name_map.insert(
                "max_processes".to_string(),
                "Max processes 31705 31705 processes".to_string(),
            );
            content_name_map.insert(
                "max_open_files".to_string(),
                "Max open files 1048576 1048576 files".to_string(),
            );
            Ok(content_name_map)
        }
        let mock = stub_sync_func!(
            Limits::read_proc_file_full_content,
            mock_read_proc_file_full_content
        );
        let mock = mock.unwrap();
        let res = Limits::from_pid(1).unwrap();
        mock.stub_remove();
        assert_eq!(res.max_stack_size, 8388608);
        assert_eq!(res.max_processes, 31705);
        assert_eq!(res.max_open_files, 1048576);
    }

    /*
     * @test   ut_process_cgroup_block
     * @title  from_pid UT test
     * @brief  Set hashmap output to test whether fn can get correct content
     */
    #[test]
    fn ut_process_cgroup_block() {
        fn mock_read_proc_file_full_content(
            _pid: &u64,
            _file: &str,
        ) -> Result<HashMap<String, String>, Error> {
            let mut content_name_map = HashMap::new();
            content_name_map.insert("pid_cgroup".to_string(), "7:pids:/".to_string());
            Ok(content_name_map)
        }

        let mock = stub_sync_func!(
            Cgroup::read_proc_file_full_content,
            mock_read_proc_file_full_content
        );
        let mock = mock.unwrap();
        let res = Cgroup::from_pid(1).unwrap();
        mock.stub_remove();
        assert_eq!(res.pid_cgroup, "");
    }
}
