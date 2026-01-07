use super::Tm;
use libc::time_t;
use std::mem;

extern "C" {
    fn tzset();
}

// 将 rust 中的 `Tm` 数据变更至为 libc 中的 `Tm` 数据
fn rust_tm_to_tm(rust_tm: &Tm, tm: &mut libc::tm) {
    tm.tm_sec = rust_tm.tm_second;
    tm.tm_min = rust_tm.tm_minute;
    tm.tm_hour = rust_tm.tm_hour;
    tm.tm_mday = rust_tm.tm_mday;
    tm.tm_mon = rust_tm.tm_month;
    tm.tm_year = rust_tm.tm_year;
    tm.tm_wday = rust_tm.tm_wday;
    tm.tm_yday = rust_tm.tm_yday;
    tm.tm_isdst = rust_tm.tm_isdst;
}

// 将 libc 中的 `Tm` 数据变更至为 rust 中的 `Tm` 数据
fn tm_to_rust_tm(tm: &libc::tm, rust_tm: &mut Tm, utcoff: i32) {
    rust_tm.tm_second = tm.tm_sec;
    rust_tm.tm_minute = tm.tm_min;
    rust_tm.tm_hour = tm.tm_hour;
    rust_tm.tm_mday = tm.tm_mday;
    rust_tm.tm_month = tm.tm_mon;
    rust_tm.tm_year = tm.tm_year;
    rust_tm.tm_wday = tm.tm_wday;
    rust_tm.tm_yday = tm.tm_yday;
    rust_tm.tm_isdst = tm.tm_isdst;
    rust_tm.tm_utcoffset = utcoff;
}

/// 将从 `Unix Epoch LOCAL` 开始所经过的秒数保存在 `libc::tm` 结构中，
/// 随后将 `libc::tm` 转换为  rust 的 `Tm` 结构。
pub fn time_to_local_tm(sec: i64, tm: &mut Tm) {
    unsafe {
        let sec = sec as time_t;
        let mut out = mem::zeroed();
        // tzset 在实现的时候是通过内部的 tzset_internal 函数来完成的，
        // 显式的调用 tzset 会以显式的方式告知 tzset_internal，
        // 而单独调用 localtime 的时候是以隐式的方式告知 tzset_internal，
        // 前者将强制 tzset 不管何种情况一律重新加载 TZ 信息或者 /etc/localtime，
        // 而后者则是只有在 TZ 发生变化，或者加载文件名发生变化的时候才会再次加载时区信息。
        // 在使用 localtime_r 之前显式调用强制刷新时区。
        tzset();
        libc::localtime_r(&sec, &mut out);
        let gmtoff = out.tm_gmtoff;
        tm_to_rust_tm(&out, tm, gmtoff as i32);
    }
}

/// 将 `Tm` 中的数据转换为从 `Unix Epoch UTC` 开始所经过的秒数
pub fn utc_tm_to_time(rust_tm: &Tm) -> i64 {
    use libc::timegm;

    // 此时创建未分配空间的 `tm` 结构
    let mut tm = unsafe { mem::zeroed() };
    // 进行转换操作
    rust_tm_to_tm(rust_tm, &mut tm);
    unsafe { timegm(&mut tm) }
}

/// 将 `Tm` 中的数据转换为从 `Unix Epoch LOCAL` 开始所经过的秒数
pub fn local_tm_to_time(rust_tm: &Tm) -> i64 {
    use libc::mktime;

    // 此时创建未分配空间的 `tm` 结构
    let mut tm = unsafe { mem::zeroed() };
    // 进行转换操作
    rust_tm_to_tm(rust_tm, &mut tm);
    unsafe { mktime(&mut tm) }
}
