use crate::base::date::BaseDate;
use crate::base::datetime::BaseDateTime;
use crate::base::time::BaseTime;
use crate::date::Date;
use crate::datetime::DateTime;
use crate::error::TimeError;
use crate::offset::fixed::FixedOffset;
use crate::offset::TimeZone;
use crate::sys::Timespec;
use crate::{sys, Datelike, Timelike};

/// 将 `sys::Tm` 结构转换为时区感知的 `DateTime` 结构
fn tm_to_datetime(mut tm: sys::Tm) -> Result<DateTime<Local>, TimeError> {
    // 存在闰秒的情况下
    if tm.tm_second >= 60 {
        tm.tm_nano += (tm.tm_second - 59) * 1_000_000_000;
        tm.tm_second = 59;
    }

    fn tm_to_base_date(tm: &sys::Tm) -> Result<BaseDate, TimeError> {
        BaseDate::from_ymd(tm.tm_year + 1900, tm.tm_month as u32 + 1, tm.tm_mday as u32)
    }

    let date = tm_to_base_date(&tm);
    let time = BaseTime::from_hms_nano(
        tm.tm_hour as u32,
        tm.tm_minute as u32,
        tm.tm_second as u32,
        tm.tm_nano as u32,
    );
    let offset = FixedOffset::east(tm.tm_utcoffset);
    match (date, time, offset) {
        // 我们获取到的 datetime 实际上是本地时间，因此需要减去偏移量并将偏移量保存至结构体之中
        (Ok(date), Ok(time), Ok(offset)) => {
            Ok(DateTime::from_utc(date.and_time(time) - offset, offset))
        }
        (Err(err), _, _) => Err(err),
        (_, Err(err), _) => Err(err),
        (_, _, Err(err)) => Err(err),
    }
}

/// 将 'BaseDateTime' 转换为 'Timespec'
fn base_datetime_to_timespec(base_datetime: &BaseDateTime, local: bool) -> Timespec {
    let tm_utcoff = i32::from(local);

    let tm = sys::Tm {
        tm_nano: 0,
        tm_second: base_datetime.second() as i32,
        tm_minute: base_datetime.minute() as i32,
        tm_hour: base_datetime.hour() as i32,
        tm_mday: base_datetime.day() as i32,
        tm_month: base_datetime.month0() as i32,
        tm_year: base_datetime.year() - 1900,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: -1,
        tm_utcoffset: tm_utcoff,
    };

    tm.to_timespec()
}

/// 本地时区结构
#[derive(Copy, Clone, Debug)]
pub struct Local;

impl Local {
    /// 根据当前日期返回 `Date`
    pub fn today() -> Result<Date<Local>, TimeError> {
        let now = Local::now();
        match now {
            Ok(now) => Ok(now.date()),
            Err(err) => Err(err),
        }
    }

    /// 根据当前日期返回 `DateTime`
    pub fn now() -> Result<DateTime<Local>, TimeError> {
        tm_to_datetime(Timespec::now().local())
    }
}

impl TimeZone for Local {
    type Offset = FixedOffset;

    fn from_offset(_offset: &Self::Offset) -> Self {
        Local
    }

    fn offset_from_local_date(&self, local: &BaseDate) -> Self::Offset {
        *self.from_local_date(local).offset()
    }

    fn offset_from_local_datetime(&self, local: &BaseDateTime) -> Self::Offset {
        *self.from_local_datetime(local).offset()
    }

    fn from_local_date(&self, local: &BaseDate) -> Date<Self> {
        let datetime =
            self.from_local_datetime(&local.and_time(BaseTime::from_hms(0, 0, 0).unwrap()));
        Date::from_utc(*local, *datetime.offset())
    }

    fn from_local_datetime(&self, local: &BaseDateTime) -> DateTime<Self> {
        let timespec = base_datetime_to_timespec(local, true);

        let mut tm = timespec.local();
        tm.tm_nano = local.nanosecond() as i32;

        tm_to_datetime(tm).unwrap()
    }

    fn offset_from_utc_date(&self, utc: &BaseDate) -> Self::Offset {
        *self.from_utc_date(utc).offset()
    }

    fn offset_from_utc_datetime(&self, utc: &BaseDateTime) -> Self::Offset {
        *self.from_local_datetime(utc).offset()
    }
}
