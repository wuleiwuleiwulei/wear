use crate::base::date::BaseDate;
use crate::base::duration::Duration;
use crate::base::time::BaseTime;
use crate::datetime::DateTime;
use crate::error::TimeError;
use crate::offset::utc::Utc;
use crate::offset::TimeZone;
use std::fmt::{Debug, Display, Formatter};

/// 包含时区的时期结构
#[derive(Clone, PartialOrd, PartialEq)]
pub struct Date<Tz: TimeZone> {
    date: BaseDate,
    offset: Tz::Offset,
}

/// 最小可能的感知时区的日期
#[allow(dead_code)]
pub const MIN_DATE: Date<Utc> = Date {
    date: crate::base::date::MIN_DATE,
    offset: Utc,
};
/// 最大可能的感知时区的日期
#[allow(dead_code)]
pub const MAX_DATE: Date<Utc> = Date {
    date: crate::base::date::MAX_DATE,
    offset: Utc,
};

impl<Tz: TimeZone> Date<Tz> {
    /// 通过给定的 `UTC` 日期以及偏移量，创建新的 `Date`
    pub fn from_utc(date: BaseDate, offset: Tz::Offset) -> Date<Tz> {
        Date { date, offset }
    }

    /// 将一个给定的 `BaseTime` 转换为 `DateTime`，并且保留相关的时区偏移信息
    pub fn and_time(&self, time: BaseTime) -> DateTime<Tz> {
        let local_datetime = self.date.and_time(time);
        self.timezone().from_local_datetime(&local_datetime)
    }

    /// 通过当前的 `date`、`hour`、`minute`、`second` 创建 `DateTime`，并且保留相关的时区偏移信息
    pub fn and_hms(&self, hour: u32, min: u32, sec: u32) -> Result<DateTime<Tz>, TimeError> {
        let basetime = BaseTime::from_hms(hour, min, sec);
        match basetime {
            Ok(basetime) => Ok(self.and_time(basetime)),
            Err(err) => Err(err),
        }
    }

    /// 通过当前的 `date`、`hour`、`minute`、`second`、`millisecond` 创建 `DateTime`，并且保留相关的时区偏移信息
    pub fn and_hms_milli(
        &self,
        hour: u32,
        min: u32,
        sec: u32,
        milli: u32,
    ) -> Result<DateTime<Tz>, TimeError> {
        let basetime = BaseTime::from_hms_milli(hour, min, sec, milli);
        match basetime {
            Ok(basetime) => Ok(self.and_time(basetime)),
            Err(err) => Err(err),
        }
    }

    /// 通过当前的 `date`、`hour`、`minute`、`second`、`microsecond` 创建 `DateTime`，并且保留相关的时区偏移信息
    pub fn and_hms_micro(
        &self,
        hour: u32,
        min: u32,
        sec: u32,
        micro: u32,
    ) -> Result<DateTime<Tz>, TimeError> {
        let basetime = BaseTime::from_hms_micro(hour, min, sec, micro);
        match basetime {
            Ok(basetime) => Ok(self.and_time(basetime)),
            Err(err) => Err(err),
        }
    }

    /// 通过当前的 `date`、`hour`、`minute`、`second`、`nanosecond` 创建 `DateTime`，并且保留相关的时区偏移信息
    pub fn and_hms_nano(
        &self,
        hour: u32,
        min: u32,
        sec: u32,
        nano: u32,
    ) -> Result<DateTime<Tz>, TimeError> {
        let basetime = BaseTime::from_hms_nano(hour, min, sec, nano);
        match basetime {
            Ok(basetime) => Ok(self.and_time(basetime)),
            Err(err) => Err(err),
        }
    }

    /// 创建当前日期的下一个日期，结构为 `Date`
    pub fn succ(&self) -> Result<Date<Tz>, TimeError> {
        let basedate = self.date.succ();
        match basedate {
            Ok(basedate) => Ok(Date::from_utc(basedate, self.offset.clone())),
            Err(err) => Err(err),
        }
    }

    /// 创建当前日期的上一个日期，结构为 `Date`
    pub fn pred(&self) -> Result<Date<Tz>, TimeError> {
        let basedate = self.date.pred();
        match basedate {
            Ok(basedate) => Ok(Date::from_utc(basedate, self.offset.clone())),
            Err(err) => Err(err),
        }
    }

    /// 返回当前日期 `Date` 的偏移量
    pub fn offset(&self) -> &Tz::Offset {
        &self.offset
    }

    /// 返回当前 `Date` 相关的时区
    pub fn timezone(&self) -> Tz {
        TimeZone::from_offset(&self.offset)
    }

    /// 将指定的 `Duration` 增加至当前日期
    pub fn checked_add_signed(self, rhs: Duration) -> Result<Date<Tz>, TimeError> {
        let basedate = self.date.checked_add_signed(rhs);
        match basedate {
            Ok(basedate) => Ok(Date {
                date: basedate,
                offset: self.offset,
            }),
            Err(err) => Err(err),
        }
    }

    /// 当前日期减去指定的 `Duration`
    pub fn checked_sub_signed(self, rhs: Duration) -> Result<Date<Tz>, TimeError> {
        let basedate = self.date.checked_sub_signed(rhs);
        match basedate {
            Ok(basedate) => Ok(Date {
                date: basedate,
                offset: self.offset,
            }),
            Err(err) => Err(err),
        }
    }

    /// 当前日期减去指定的 `Date`，并返回其 `Duration`
    pub fn signed_duration_since(self, rhs: Date<Tz>) -> Result<Duration, TimeError> {
        self.date.signed_duration_since(rhs.date)
    }
}

impl<Tz: TimeZone> Display for Date<Tz> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{:?}", self.date, self.offset)
    }
}

impl<Tz: TimeZone> Debug for Date<Tz> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.date, self.offset)
    }
}
