use crate::error::Error;
use crate::error::InnerError::{NotFound, Overflow};
use std::fs::File;
use std::io::Read;
use std::str::Lines;

pub struct CpusInfo {
    global_cpu: Cpu,
    cpus: Vec<Cpu>,
}

impl CpusInfo {
    /// 获取当前 CPU 的基本信息
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::CpusInfo;
    ///
    /// let cpus_info = CpusInfo::new();
    /// assert!(cpus_info.is_ok());
    /// ```
    pub fn new() -> Result<Self, Error> {
        let string = read_proc_stat()?;
        let mut cpus = vec![];
        let mut lines = string.trim().lines();
        let global_cpu = if let Ok(temp) = parse_proc_stat_line("cpu  ", lines.next().unwrap()) {
            Cpu::new(work_time(&temp), total_time(&temp))
        } else {
            Cpu::new(0, 0)
        };
        for (id, line) in lines.enumerate() {
            if let Ok(temp) = parse_proc_stat_line(&format!("cpu{id} "), line) {
                cpus.push(Cpu::new(work_time(&temp), total_time(&temp)));
            } else {
                break;
            }
        }

        Ok(Self { global_cpu, cpus })
    }

    fn _update_global_cpu(&mut self, lines: &mut Lines) -> Result<(), Error> {
        if let Some(first_line) = lines.next() {
            if let Ok(temp) = parse_proc_stat_line("cpu  ", first_line) {
                self.global_cpu
                    .update(work_time(&temp), total_time(&temp))?;
                return Ok(());
            }
        }

        Err(Error::Internal(NotFound))
    }

    /// 仅更新整体 CPU 的数据
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::CpusInfo;
    ///
    /// let mut cpus_info = CpusInfo::new().unwrap();
    /// cpus_info.update_global_cpu().unwrap();
    /// ```
    pub fn update_global_cpu(&mut self) -> Result<(), Error> {
        let string = read_proc_stat()?;
        let mut lines = string.trim().lines();
        self._update_global_cpu(&mut lines)
    }

    fn _update_cpus(&mut self, lines: &mut Lines) -> Result<(), Error> {
        for (id, line) in lines.take(self.cpus.len()).enumerate() {
            if let Ok(temp) = parse_proc_stat_line(&format!("cpu{id} "), line) {
                self.cpus[id].update(work_time(&temp), total_time(&temp))?;
            }
        }

        Ok(())
    }

    /// 仅更新逻辑 CPU 列表的数据
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::CpusInfo;
    ///
    /// let mut cpus_info = CpusInfo::new().unwrap();
    /// cpus_info.update_cpus().unwrap();
    /// ```
    pub fn update_cpus(&mut self) -> Result<(), Error> {
        let string = read_proc_stat()?;
        let mut lines = string.trim().lines();
        lines.next().unwrap();
        self._update_cpus(&mut lines)
    }

    /// 更新所有 CPU 信息
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::CpusInfo;
    ///
    /// let mut cpus_info = CpusInfo::new().unwrap();
    /// cpus_info.update_all().unwrap();
    /// ```
    pub fn update_all(&mut self) -> Result<(), Error> {
        let string = read_proc_stat()?;
        let mut lines = string.trim().lines();
        self._update_global_cpu(&mut lines)?;
        self._update_cpus(&mut lines)?;
        Ok(())
    }

    /// 获取整体 CPU 的数据引用
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::CpusInfo;
    ///
    /// let cpus_info = CpusInfo::new().unwrap();
    /// let global_cpu = cpus_info.global_cpu();
    /// ```
    pub fn global_cpu(&self) -> &Cpu {
        &self.global_cpu
    }

    /// 获取逻辑 CPU 列表的数据引用
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::CpusInfo;
    ///
    /// let cpus_info = CpusInfo::new().unwrap();
    /// let cpus = cpus_info.cpus();
    /// ```
    pub fn cpus(&self) -> &[Cpu] {
        &self.cpus
    }
}

// 获取 CPU 某个时刻的工作时间
fn work_time(data: &[u64]) -> u64 {
    // 对应于 /proc/stat 之中的数据位置，该位置固定，如下：
    // user + nice + system + irq + softirq + steal
    data[0] + data[1] + data[2] + data[5] + data[6] + data[7]
}

// 获取 CPU 某个时刻的总时间
fn total_time(data: &[u64]) -> u64 {
    // 对应于 /proc/stat 之中的数据位置，该位置固定，如下：
    // work_time + idle + iowait
    work_time(data) + data[3] + data[4]
}

// 获取 /proc/stat 之中的数据
fn read_proc_stat() -> Result<String, Error> {
    let mut string = String::new();
    if let Err(e) = File::open("/proc/stat").and_then(|mut f| f.read_to_string(&mut string)) {
        return Err(Error::Os(e));
    }

    Ok(string)
}

// 获取 CPU 占用率基本数据
fn parse_proc_stat_line(head: &str, line: &str) -> Result<Vec<u64>, Error> {
    let mut data = vec![];
    if !line.contains(head) {
        return Err(Error::Internal(NotFound));
    }
    for i in line.trim_start_matches(head).split(' ') {
        data.push(i.parse::<u64>().unwrap());
    }
    Ok(data)
}

pub struct Cpu {
    // 旧的 CPU 工作时间
    old_work_time: u64,
    // 旧的 CPU 总时间
    old_total_time: u64,

    // 新的 CPU 工作时间
    new_work_time: u64,
    // 新的 CPU 总时间
    new_total_time: u64,

    // 占用率(万分比)，不可能超过 10000
    usage: u16,
}

macro_rules! sub {
    ($a:expr, $b:expr, $c:expr) => {
        if $a > $b {
            ($a - $b) as u64
        } else {
            !$c as u64
        }
    };
}

impl Cpu {
    fn new(work_time: u64, total_time: u64) -> Self {
        Self {
            old_work_time: 0,
            old_total_time: 0,
            new_work_time: work_time,
            new_total_time: total_time,
            usage: 0,
        }
    }

    fn update(&mut self, work_time: u64, total_time: u64) -> Result<(), Error> {
        self.old_work_time = self.new_work_time;
        self.old_total_time = self.new_total_time;
        self.new_work_time = work_time;
        self.new_total_time = total_time;

        let mut usage = sub!(self.new_work_time, self.old_work_time, true)
            .checked_mul(10000)
            .ok_or(Error::Internal(Overflow))?
            / sub!(self.new_total_time, self.old_total_time, false);

        if usage > 10000 {
            usage = 10000;
        }
        self.usage = usage as u16;
        Ok(())
    }

    /// 获取 CPU 占用率（万分比）
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::CpusInfo;
    ///
    /// let mut cpus_info = CpusInfo::new().unwrap();
    /// cpus_info.update_global_cpu().unwrap();
    /// let global_cpu = cpus_info.global_cpu();
    /// let usage = global_cpu.usage();
    /// ```
    pub fn usage(&self) -> u16 {
        self.usage
    }
}

#[cfg(test)]
mod test {
    use crate::CpusInfo;
    use sysinfo::{System, SystemExt};

    /*
     * @test ut_cpus_info_new
     * @title  CpusInfo ut 测试用例
     * @brief  1. 和三方库 sysinfo 进行交叉验证
     *         2. 相同设备上得到的数据应当相同
     */
    #[test]
    fn ut_cpus_info_new() {
        let cpus_info = CpusInfo::new().unwrap();
        let num_one = cpus_info.cpus().len();

        let system = System::new_all();
        let num_two = system.cpus().len();
        assert_eq!(num_one, num_two);
    }

    /*
     * @test ut_cpus_info_update_global_cpu
     * @title  CpusInfo ut 测试用例
     * @brief  1. 获取基本的 CpusInfo 后进行更新
     *         2. 数据数值变化
     */
    #[test]
    fn ut_cpus_info_update_global_cpu() {
        let mut cpus_info = CpusInfo::new().unwrap();
        let old_work_time = cpus_info.global_cpu().old_work_time;
        let old_total_time = cpus_info.global_cpu().old_total_time;
        assert_eq!(old_work_time, 0);
        assert_eq!(old_total_time, 0);

        cpus_info.update_global_cpu().unwrap();
        let old_work_time = cpus_info.global_cpu().old_work_time;
        let old_total_time = cpus_info.global_cpu().old_total_time;
        assert_ne!(old_work_time, 0);
        assert_ne!(old_total_time, 0);
    }

    /*
     * @test ut_cpus_info_update_cpus
     * @title  CpusInfo ut 测试用例
     * @brief  1. 获取基本的 CpusInfo 后进行更新
     *         2. 数据数值变化
     */
    #[test]
    fn ut_cpus_info_update_cpus() {
        let mut cpus_info = CpusInfo::new().unwrap();
        let old_work_time = cpus_info.cpus()[0].old_work_time;
        let old_total_time = cpus_info.cpus()[0].old_total_time;
        assert_eq!(old_work_time, 0);
        assert_eq!(old_total_time, 0);

        cpus_info.update_cpus().unwrap();
        let old_work_time = cpus_info.cpus()[0].old_work_time;
        let old_total_time = cpus_info.cpus()[0].old_total_time;
        assert_ne!(old_work_time, 0);
        assert_ne!(old_total_time, 0);
    }

    /*
     * @test ut_cpus_info_update_all
     * @title  CpusInfo::update_all() ut 测试用例
     * @brief  1. 获取基本的 CpusInfo 后进行更新
     *         2. 整体cpu与逻辑cpu的信息都有变化
     */
    #[test]
    fn ut_cpus_info_update_all() {
        let mut cpus_info = CpusInfo::new().unwrap();
        cpus_info.update_all().unwrap();
        let global_old_work_time = cpus_info.global_cpu().old_work_time;
        let global_old_total_time = cpus_info.global_cpu().old_total_time;
        let cpus_old_work_time = cpus_info.cpus()[0].old_work_time;
        let cpus_old_total_time = cpus_info.cpus()[0].old_total_time;
        assert_ne!(global_old_work_time, 0);
        assert_ne!(global_old_total_time, 0);
        assert_ne!(cpus_old_work_time, 0);
        assert_ne!(cpus_old_total_time, 0);
    }
}
