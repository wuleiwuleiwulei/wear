use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(target_os = "linux")]
#[path = "sys/unix.rs"]
mod inner;

#[cfg(target_os = "windows")]
#[path = "sys/windows.rs"]
mod inner;

/// 将秒数与纳秒数记录为标准时间格式的结构
#[derive(Debug)]
pub struct Timespec {
    pub sec: i64,
    pub nano: i32,
}

impl Timespec {
    /// 通过系统时间获取到当前时间并记录为标准格式
    pub fn now() -> Timespec {
        let system_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before Unix epoch");

        Timespec {
            sec: system_time.as_secs() as i64,
            nano: system_time.subsec_nanos() as i32,
        }
    }

    /// 将当前 `Timespec` 结构转换为系统本地时间
    pub fn local(self) -> Tm {
        let mut tm = Tm {
            tm_second: 0,
            tm_minute: 0,
            tm_hour: 0,
            tm_mday: 0,
            tm_month: 0,
            tm_year: 0,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: 0,
            tm_utcoffset: 0,
            tm_nano: 0,
        };
        inner::time_to_local_tm(self.sec, &mut tm);
        tm.tm_nano = self.nano;
        tm
    }
}

/// 将历法日期和时间拆分成多个部分的结构
#[derive(Debug)]
pub struct Tm {
    /// 一秒钟的纳秒数范围： 0 ~ 1_000_000_000 - 1
    pub tm_nano: i32,
    /// 一分钟内的秒数范围： 0 ~ 60，存在闰秒的情况
    pub tm_second: i32,
    /// 一小时内的分钟范围： 0 ~ 59
    pub tm_minute: i32,
    /// 一天内的小时范围： 0 ~ 23
    pub tm_hour: i32,
    /// 一个月内的天数范围： 1 ~ 31
    pub tm_mday: i32,
    /// 一年内的月份范围： 0 ~ 11
    pub tm_month: i32,
    /// 年份范围从 1900 年开始
    pub tm_year: i32,
    /// 一周内的天数范围： 0 ~ 6
    pub tm_wday: i32,
    /// 一年内的天数范围： 0 ~ 365
    pub tm_yday: i32,
    /// 判断是否使用夏令时
    pub tm_isdst: i32,
    /// 转换世界协调时的变化
    pub tm_utcoffset: i32,
}

impl Tm {
    /// 将 `Tm` 中的数据转换为从 `Unix Epoch` 开始所经过的秒数，
    /// `Unix Epoch` 是 `UTC` 还是 `LOCAL` 由 `Tm.tm_utcoffset` 决定
    #[allow(dead_code)]
    pub fn to_timespec(&self) -> Timespec {
        let sec = match self.tm_utcoffset {
            0 => inner::utc_tm_to_time(self),
            _ => inner::local_tm_to_time(self),
        };
        Timespec {
            sec,
            nano: self.tm_nano,
        }
    }
}
