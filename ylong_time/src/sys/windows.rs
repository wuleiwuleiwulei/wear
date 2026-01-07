use crate::sys::Tm;
use core::mem;
use std::os::raw::{c_int, c_long, c_ulong, c_ushort};

const HECTONANOSECS_IN_SEC: i64 = 10_000_000;

// windows epoch 从 1601-01-01T00:00:00Z 开始。
// 它比 UNIX/Linux 纪元 (1970-01-01T00:00:00Z) 早 11_644_473_600 秒
const HECTONANOSEC_TO_UNIX_EPOCH: i64 = 11_644_473_600 * HECTONANOSECS_IN_SEC;

/// 包含一个 64 位值，表示自 1601 年 1 月 1 日 (UTC) 以来的 100 纳秒间隔数。
/// 可以通过 FileTimeToSystemTime 函数转换为使用者易懂的系统时间。
#[repr(C)]
pub struct Filetime {
    dw_low_date_time: c_ulong,
    dw_high_date_time: c_ulong,
}

/// 指定日期和时间，使用单个成员表示月、日、年、工作日、小时、分钟、秒和毫秒。
/// 时间采用协调世界时 (UTC) 或本地时间 (LOCAL)，具体取决于要调用的函数。
#[repr(C)]
pub struct Systemtime {
    w_year: c_ushort,
    w_month: c_ushort,
    w_day_of_week: c_ushort,
    w_day: c_ushort,
    w_hour: c_ushort,
    w_minute: c_ushort,
    w_second: c_ushort,
    w_milliseconds: c_ushort,
}

/// 指定时区的设置
#[repr(C)]
pub struct TIME_ZONE_INFORMATION {
    bias: c_long,
    standard_name: [u16; 32],
    standard_date: Systemtime,
    standard_bias: c_long,
    daylight_name: [u16; 32],
    daylight_date: Systemtime,
    daylight_bias: c_long,
}

fn time_to_file_time(sec: i64) -> Filetime {
    let t = ((sec * HECTONANOSECS_IN_SEC) + HECTONANOSEC_TO_UNIX_EPOCH) as u64;
    Filetime {
        dw_low_date_time: t as c_ulong,
        dw_high_date_time: (t >> 32) as c_ulong,
    }
}

fn file_time_as_u64(ft: &Filetime) -> u64 {
    ((ft.dw_high_date_time as u64) << 32) | (ft.dw_low_date_time as u64)
}

fn file_time_to_unix_seconds(ft: &Filetime) -> i64 {
    let t = file_time_as_u64(ft) as i64;
    (t - HECTONANOSEC_TO_UNIX_EPOCH) / HECTONANOSECS_IN_SEC
}

extern "system" {
    pub fn SystemTimeToFileTime(
        lpSystemTime: *const Systemtime,
        lpFileTime: *mut Filetime,
    ) -> c_int;

    pub fn FileTimeToSystemTime(
        lpFileTime: *const Filetime,
        lpSystemTime: *mut Systemtime,
    ) -> c_int;

    pub fn SystemTimeToTzSpecificLocalTime(
        lpTimeZoneInformation: *const TIME_ZONE_INFORMATION,
        lpUniversalTime: *const Systemtime,
        lpLocalTime: *mut Systemtime,
    ) -> c_int;

    pub fn GetTimeZoneInformation(lpTimeZoneInformation: *mut TIME_ZONE_INFORMATION) -> c_ulong;

    pub fn TzSpecificLocalTimeToSystemTime(
        lpTimeZoneInformation: *const TIME_ZONE_INFORMATION,
        lpLocalTime: *const Systemtime,
        lpUniversalTime: *mut Systemtime,
    ) -> c_int;
}

fn system_time_to_file_time(sys: &Systemtime) -> Filetime {
    unsafe {
        let mut ft = mem::zeroed();
        SystemTimeToFileTime(sys, &mut ft);
        ft
    }
}

fn tm_to_system_time(tm: &Tm) -> Systemtime {
    let mut sys: Systemtime = unsafe { mem::zeroed() };
    sys.w_second = tm.tm_second as c_ushort;
    sys.w_minute = tm.tm_minute as c_ushort;
    sys.w_hour = tm.tm_hour as c_ushort;
    sys.w_day = tm.tm_mday as c_ushort;
    sys.w_day_of_week = tm.tm_wday as c_ushort;
    sys.w_month = (tm.tm_month + 1) as c_ushort;
    sys.w_year = (tm.tm_year + 1900) as c_ushort;
    sys
}

fn system_time_to_tm(sys: &Systemtime, tm: &mut Tm) {
    // 计算当前年月日是该年份中的第几天
    fn yday(year: i32, month: i32, day: i32) -> i32 {
        let leap = if month > 2 {
            if year % 4 == 0 {
                1
            } else {
                2
            }
        } else {
            0
        };
        let july = i32::from(month > 7);

        (month - 1) * 30 + month / 2 + (day - 1) - leap + july
    }

    tm.tm_second = sys.w_second as i32;
    tm.tm_minute = sys.w_minute as i32;
    tm.tm_hour = sys.w_hour as i32;
    tm.tm_mday = sys.w_day as i32;
    tm.tm_wday = sys.w_day_of_week as i32;
    tm.tm_month = (sys.w_month - 1) as i32;
    tm.tm_year = (sys.w_year - 1900) as i32;
    tm.tm_yday = yday(tm.tm_year, tm.tm_month + 1, tm.tm_mday);
}

/// 将从 `Unix Epoch LOCAL` 开始所经过的秒数保存在 `libc::tm` 结构中，
/// 随后将 `libc::tm` 转换为  rust 的 `Tm` 结构。
pub fn time_to_local_tm(sec: i64, tm: &mut Tm) {
    let ft = time_to_file_time(sec);
    unsafe {
        let mut utc = mem::zeroed();
        let mut local = mem::zeroed();
        FileTimeToSystemTime(&ft, &mut utc);
        SystemTimeToTzSpecificLocalTime(std::ptr::null(), &utc, &mut local);
        system_time_to_tm(&local, tm);

        let local = system_time_to_file_time(&local);
        let local_sec = file_time_to_unix_seconds(&local);

        let mut tz = mem::zeroed();
        GetTimeZoneInformation(&mut tz);

        tm.tm_utcoffset = (local_sec - sec) as i32;
        tm.tm_isdst = if tm.tm_utcoffset == -60 * (tz.bias + tz.standard_bias) {
            0
        } else {
            1
        };
    }
}

/// 将 `Tm` 中的数据转换为从 `Unix Epoch UTC` 开始所经过的秒数
pub fn utc_tm_to_time(tm: &Tm) -> i64 {
    unsafe {
        let mut ft = mem::zeroed();
        let sys_time = tm_to_system_time(tm);
        SystemTimeToFileTime(&sys_time, &mut ft);
        file_time_to_unix_seconds(&ft)
    }
}

/// 将 `Tm` 中的数据转换为从 `Unix Epoch LOCAL` 开始所经过的秒数
pub fn local_tm_to_time(tm: &Tm) -> i64 {
    unsafe {
        let mut ft = mem::zeroed();
        let mut utc = mem::zeroed();
        let sys_time = tm_to_system_time(tm);
        TzSpecificLocalTimeToSystemTime(std::ptr::null_mut(), &sys_time, &mut utc);
        SystemTimeToFileTime(&utc, &mut ft);
        file_time_to_unix_seconds(&ft)
    }
}
