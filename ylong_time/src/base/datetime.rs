use crate::base::date::{BaseDate, MAX_DATE, MIN_DATE};
use crate::base::duration::Duration;
use crate::base::time::{BaseTime, MIN_TIME};
use crate::error::TimeError;
use crate::{Datelike, Timelike, Weekday};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Sub, SubAssign};

/// 秒中的纳秒数
const NANOS_OF_SEC: u32 = 1_000_000_000;
/// 一天中的秒数
const SECS_OF_DAY: i64 = 86400;
/// 从第零年开始到 1970.1.1 所经历的天数
const UNIX_BEGIN_DAYS: i64 = 719_163;

/// 最小的可能日期时间
pub const MIN_DATETIME: BaseDateTime = BaseDateTime {
    date: MIN_DATE,
    time: MIN_TIME,
};

/// 最大的可能日期时间
pub const MAX_DATETIME: BaseDateTime = BaseDateTime {
    date: MAX_DATE,
    time: MIN_TIME,
};

/// 不包含时区的日期与时间基础结构
#[derive(Copy, Clone, PartialOrd, PartialEq, Eq)]
pub struct BaseDateTime {
    date: BaseDate,
    time: BaseTime,
}

impl BaseDateTime {
    /// 创建一个 `BaseDateTime` 结构，根据已有的 `BaseDate`、`BaseTime` 创建
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::date::BaseDate;
    /// use ylong_time::base::time::BaseTime;
    /// use ylong_time::base::datetime::BaseDateTime;
    ///
    /// let base_date = BaseDate::from_ymd(2021, 6, 9).unwrap();
    /// let base_time = BaseTime::from_hms(11, 12, 13).unwrap();
    /// let base_datetime = BaseDateTime::new(base_date, base_time);
    /// println!("{}", base_datetime);
    /// ```
    pub fn new(date: BaseDate, time: BaseTime) -> BaseDateTime {
        BaseDateTime { date, time }
    }

    /// 创建一个从 `UNIX` 时间戳时间开始的 `BaseDateTime`
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::datetime::BaseDateTime;
    /// use ylong_time::base::date::BaseDate;
    /// use ylong_time::base::time::BaseTime;
    ///
    /// let base_date = BaseDate::from_ymd(1970, 1, 1).unwrap();
    /// let base_time = BaseTime::from_hms(0, 0, 0).unwrap();
    /// let base_datetime = BaseDateTime::from_timestamp(0, 0).unwrap();
    /// assert_eq!(base_datetime.date(), base_date);
    /// assert_eq!(base_datetime.time(), base_time);
    /// ```
    pub fn from_timestamp(secs: i64, nano: u32) -> Result<BaseDateTime, TimeError> {
        let mut day: i64;
        let mut sec: i64;
        if secs < 0 {
            day = secs / SECS_OF_DAY - 1;
            sec = SECS_OF_DAY - (secs % SECS_OF_DAY).abs();
            if sec == SECS_OF_DAY {
                sec %= SECS_OF_DAY;
                day += 1;
            }
        } else {
            day = secs / SECS_OF_DAY;
            sec = secs % SECS_OF_DAY
        }
        let date = BaseDate::from_num_days_from_ce(day.add(UNIX_BEGIN_DAYS) as i32);
        let time = BaseTime::from_num_seconds_from_midnight(sec as u32, nano);
        match (date, time) {
            (Ok(date), Ok(time)) => Ok(BaseDateTime { date, time }),
            (Err(date_err), _) => Err(date_err),
            (_, Err(time_err)) => Err(time_err),
        }
    }

    /// 检索 `BaseDateTime` 中的日期
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::datetime::BaseDateTime;
    /// use ylong_time::base::date::BaseDate;
    ///
    /// let base_datetime = BaseDateTime::from_timestamp(0, 0).unwrap();
    /// let base_date = BaseDate::from_ymd(1970, 1, 1).unwrap();
    /// assert_eq!(base_datetime.date(), base_date);
    /// ```
    pub fn date(&self) -> BaseDate {
        self.date
    }

    /// 检索 `BaseDateTime` 中的时间
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::datetime::BaseDateTime;
    /// use ylong_time::base::time::BaseTime;
    ///
    /// let base_datetime = BaseDateTime::from_timestamp(0, 0).unwrap();
    /// let base_time = BaseTime::from_hms(0, 0, 0).unwrap();
    /// assert_eq!(base_datetime.time(), base_time);
    /// ```
    pub fn time(&self) -> BaseTime {
        self.time
    }

    /// 返回从 1970.1.1 开始的时间戳秒数，不包含闰秒
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::datetime::BaseDateTime;
    ///
    /// let base_datetime = BaseDateTime::from_timestamp(0, 0).unwrap();
    /// assert_eq!(base_datetime.timestamp().unwrap(), 0);
    /// ```
    pub fn timestamp(&self) -> Result<i64, TimeError> {
        let seconds = i64::from(self.time.num_seconds_from_midnight());
        let days = self.date.num_days_from_ce();
        match days {
            Ok(days) => Ok((days as i64 - UNIX_BEGIN_DAYS) * SECS_OF_DAY
                + seconds
                + (self.time.nanosecond() / NANOS_OF_SEC) as i64),
            Err(err) => Err(err),
        }
    }

    /// 返回从 1970.1.1 开始的时间戳毫秒，不包含闰秒
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::BaseDateTime;
    ///
    /// let base_datetime = BaseDateTime::from_timestamp(0, 1_000_000).unwrap();
    /// assert_eq!(base_datetime.timestamp_millis().unwrap(), 1);
    /// ```
    pub fn timestamp_millis(&self) -> Result<i64, TimeError> {
        let as_ms = self.timestamp()? * 1000;
        Ok(as_ms + i64::from(self.nanosecond() / 1_000_000))
    }

    /// 返回从 1970.1.1 开始的时间戳微秒，不包含闰秒
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::BaseDateTime;
    ///
    /// let base_datetime = BaseDateTime::from_timestamp(0, 1_000).unwrap();
    /// assert_eq!(base_datetime.timestamp_micros().unwrap(), 1);
    /// ```
    pub fn timestamp_micros(&self) -> Result<i64, TimeError> {
        let as_us = self.timestamp()? * 1_000_000;
        Ok(as_us + i64::from(self.nanosecond() / 1000))
    }

    /// 返回从 1970.1.1 开始的时间戳纳秒，不包含闰秒
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::BaseDateTime;
    ///
    /// let base_datetime = BaseDateTime::from_timestamp(0, 1).unwrap();
    /// assert_eq!(base_datetime.timestamp_nanos().unwrap(), 1);
    /// ```
    pub fn timestamp_nanos(&self) -> Result<i128, TimeError> {
        let as_ns = i128::from(self.timestamp()?) * 1_000_000_000;
        Ok(as_ns + i128::from(self.nanosecond()))
    }

    /// 将当前 `BaseDateTime` 加上某个 `Duration`
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::datetime::BaseDateTime;
    /// use ylong_time::base::duration::Duration;
    /// use ylong_time::base::date::BaseDate;
    /// use ylong_time::base::time::BaseTime;
    ///
    /// let base_datetime = BaseDateTime::from_timestamp(0, 0).unwrap();
    /// let duration = Duration::days(1).unwrap();
    /// let base_date = BaseDate::from_ymd(1970, 1, 2).unwrap();
    /// let base_time = BaseTime::from_hms(0, 0, 0).unwrap();
    /// assert_eq!(base_datetime + duration, BaseDateTime::new(base_date, base_time));
    /// ```
    pub fn checked_add_signed(self, rhs: Duration) -> Result<BaseDateTime, TimeError> {
        let (time, day_secs) = self.time.overflowing_add_signed(rhs);

        let date = self
            .date
            .checked_add_signed(Duration::seconds(day_secs).unwrap());
        match date {
            Ok(date) => Ok(BaseDateTime { date, time }),
            Err(err) => Err(err),
        }
    }

    /// 将当前 `BaseDateTime` 减去某个 `Duration`
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::datetime::BaseDateTime;
    /// use ylong_time::base::duration::Duration;
    /// use ylong_time::base::date::BaseDate;
    /// use ylong_time::base::time::BaseTime;
    ///
    /// let base_datetime = BaseDateTime::from_timestamp(0, 0).unwrap();
    /// let duration = Duration::days(1).unwrap();
    /// let base_date = BaseDate::from_ymd(1969, 12, 31).unwrap();
    /// let base_time = BaseTime::from_hms(0, 0, 0).unwrap();
    /// assert_eq!(base_datetime - duration, BaseDateTime::new(base_date, base_time));
    /// ```
    pub fn checked_sub_signed(self, rhs: Duration) -> Result<BaseDateTime, TimeError> {
        let (time, day_secs) = self.time.overflowing_sub_signed(rhs);

        let date = self
            .date
            .checked_sub_signed(Duration::seconds(day_secs).unwrap());
        match date {
            Ok(date) => Ok(BaseDateTime { date, time }),
            Err(err) => Err(err),
        }
    }

    /// 将当前 `BaseDateTime` 减去某个 `BaseDateTime`
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::datetime::BaseDateTime;
    /// use ylong_time::base::duration::Duration;
    /// use ylong_time::base::date::BaseDate;
    /// use ylong_time::base::time::BaseTime;
    ///
    /// let base_datetime_one = BaseDateTime::from_timestamp(0, 0).unwrap();
    /// let base_datetime_two = BaseDateTime::from_timestamp(86400, 0).unwrap();
    /// let duration = Duration::days(1).unwrap();
    /// assert_eq!((base_datetime_two - base_datetime_one), duration);
    /// ```
    pub fn signed_duration_since(self, rhs: BaseDateTime) -> Result<Duration, TimeError> {
        match self.date.signed_duration_since(rhs.date) {
            Ok(duration) => Ok(duration + self.time.signed_duration_since(rhs.time)),
            Err(err) => Err(err),
        }
    }
}

impl Datelike for BaseDateTime {
    /// 返回当前 `BaseDateTime` 的年份
    fn year(&self) -> i32 {
        self.date.year()
    }

    /// 返回当前 `BaseDateTime` 的月份（从一月开始）
    fn month(&self) -> u32 {
        self.date.month()
    }

    /// 返回当前 `BaseDateTime` 的月份（从零月开始）
    fn month0(&self) -> u32 {
        self.date.month0()
    }

    /// 返回当前日期的天数（每月的第几天，从第一天开始计算）
    fn day(&self) -> u32 {
        self.date.day()
    }

    /// 返回当前日期的天数（每月的第几天，从第零天开始计算）
    fn day0(&self) -> u32 {
        self.date.day0()
    }

    /// 返回当前日期的天数（每年的第几天，从第一天开始计算）
    fn ordinal(&self) -> u32 {
        self.date.ordinal()
    }

    /// 返回当前日期的天数（每年的第几天，从第零天开始计算）
    fn ordinal0(&self) -> u32 {
        self.date.ordinal0()
    }

    fn weekday(&self) -> Weekday {
        self.date.weekday()
    }

    /// 将一个 BaseDateTime 替换年份，并且可以判断替换年份后的日期是否有效
    fn with_year(&self, year: i32) -> Result<Self, TimeError> {
        let date = self.date.with_year(year);
        match date {
            Ok(date) => Ok(BaseDateTime {
                date,
                time: self.time,
            }),
            Err(err) => Err(err),
        }
    }

    /// 将一个 BaseDateTime 替换月份（从一月份开始计算），并且可以判断替换月份后的日期是否有效
    fn with_month(&self, month: u32) -> Result<Self, TimeError> {
        let date = self.date.with_month(month);
        match date {
            Ok(date) => Ok(BaseDateTime {
                date,
                time: self.time,
            }),
            Err(err) => Err(err),
        }
    }

    /// 将一个 BaseDateTime 替换月份（从零月份开始计算），并且可以判断替换月份后的日期是否有效
    fn with_month0(&self, month0: u32) -> Result<Self, TimeError> {
        let date = self.date.with_month0(month0);
        match date {
            Ok(date) => Ok(BaseDateTime {
                date,
                time: self.time,
            }),
            Err(err) => Err(err),
        }
    }

    /// 将一个 BaseDateTime 替换天数（每月第几天，从第一天开始计算），并且可以判断替换后的日期是否有效
    fn with_day(&self, day: u32) -> Result<Self, TimeError> {
        let date = self.date.with_day(day);
        match date {
            Ok(date) => Ok(BaseDateTime {
                date,
                time: self.time,
            }),
            Err(err) => Err(err),
        }
    }

    /// 将一个 BaseDateTime 替换天数（每月第几天，从第零天开始计算），并且可以判断替换后的日期是否有效
    fn with_day0(&self, day0: u32) -> Result<Self, TimeError> {
        let date = self.date.with_day0(day0);
        match date {
            Ok(date) => Ok(BaseDateTime {
                date,
                time: self.time,
            }),
            Err(err) => Err(err),
        }
    }

    /// 将一个 BaseDateTime 替换天数（每年第几天，从第一天开始计算），并且可以判断替换后的日期是否有效
    fn with_ordinal(&self, ordinal: u32) -> Result<Self, TimeError> {
        let date = self.date.with_ordinal(ordinal);
        match date {
            Ok(date) => Ok(BaseDateTime {
                date,
                time: self.time,
            }),
            Err(err) => Err(err),
        }
    }

    /// 将一个 BaseDateTime 替换天数（每年第几天，从第零天开始计算），并且可以判断替换后的日期是否有效
    fn with_ordinal0(&self, ordinal0: u32) -> Result<Self, TimeError> {
        let date = self.date.with_ordinal0(ordinal0);
        match date {
            Ok(date) => Ok(BaseDateTime {
                date,
                time: self.time,
            }),
            Err(err) => Err(err),
        }
    }
}

impl Timelike for BaseDateTime {
    /// 返回当前 `BaseDateTime` 代表的小时数
    fn hour(&self) -> u32 {
        self.time.hour()
    }

    /// 返回当前 `BaseDateTime` 代表的分钟数
    fn minute(&self) -> u32 {
        self.time.minute()
    }

    /// 返回当前 `BaseDateTime` 代表的秒数，但是就算存在闰秒也不会返回六十秒的情况
    fn second(&self) -> u32 {
        self.time.second()
    }

    /// 返回当前 `BaseDateTime` 代表的纳秒数
    fn nanosecond(&self) -> u32 {
        self.time.nanosecond()
    }

    /// 将当前 `BaseDateTime` 代表的小时替换为新的小时数
    fn with_hour(&self, hour: u32) -> Result<Self, TimeError> {
        let time = self.time.with_hour(hour);
        match time {
            Ok(time) => Ok(BaseDateTime {
                date: self.date,
                time,
            }),
            Err(err) => Err(err),
        }
    }

    /// 将当前 `BaseDateTime` 代表的分钟替换为新的分钟数
    fn with_minute(&self, minute: u32) -> Result<Self, TimeError> {
        let time = self.time.with_minute(minute);
        match time {
            Ok(time) => Ok(BaseDateTime {
                date: self.date,
                time,
            }),
            Err(err) => Err(err),
        }
    }

    /// 将当前 `BaseDateTime` 所代表的秒数替换为新的秒数，不包含闰秒的情况，因此传入的秒数范围也在 0 ~ 59
    fn with_second(&self, second: u32) -> Result<Self, TimeError> {
        let time = self.time.with_second(second);
        match time {
            Ok(time) => Ok(BaseDateTime {
                date: self.date,
                time,
            }),
            Err(err) => Err(err),
        }
    }

    /// 将当前 `BaseDateTime` 所代表的纳秒替换为新的纳秒数
    fn with_nanosecond(&self, nanosecond: u32) -> Result<Self, TimeError> {
        let time = self.time.with_nanosecond(nanosecond);
        match time {
            Ok(time) => Ok(BaseDateTime {
                date: self.date,
                time,
            }),
            Err(err) => Err(err),
        }
    }
}

impl Add<Duration> for BaseDateTime {
    type Output = BaseDateTime;

    fn add(self, rhs: Duration) -> Self::Output {
        self.checked_add_signed(rhs)
            .expect("`BaseDateTime + Duration` overflowed")
    }
}

impl AddAssign<Duration> for BaseDateTime {
    fn add_assign(&mut self, rhs: Duration) {
        *self = self.add(rhs);
    }
}

impl Sub<Duration> for BaseDateTime {
    type Output = BaseDateTime;

    fn sub(self, rhs: Duration) -> Self::Output {
        self.checked_sub_signed(rhs)
            .expect("`BaseDateTime - Duration` overflowed")
    }
}

impl SubAssign<Duration> for BaseDateTime {
    fn sub_assign(&mut self, rhs: Duration) {
        *self = self.sub(rhs)
    }
}

impl Sub<BaseDateTime> for BaseDateTime {
    type Output = Duration;

    fn sub(self, rhs: BaseDateTime) -> Self::Output {
        self.signed_duration_since(rhs)
            .expect("`BaseDateTime - BaseDateTime` overflowed")
    }
}

impl Debug for BaseDateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}T{:?}", self.date, self.time)
    }
}

impl Display for BaseDateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.date, self.time)
    }
}

#[cfg(test)]
mod test {
    include!("../../tests/ut/base/ut_datetime.rs");
}
