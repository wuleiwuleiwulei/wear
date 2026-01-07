use crate::base::date::BaseDate;
use crate::base::datetime::BaseDateTime;
use crate::base::duration::Duration;
use crate::base::time::BaseTime;
use crate::date::Date;
use crate::error::TimeError;
use crate::format::parse::{parse, Parsed};
use crate::format::strftime::StrftimeItems;
use crate::format::Item;
use crate::format::{DelayedFormat, Specific};
use crate::offset::fixed::FixedOffset;
use crate::offset::utc::Utc;
use crate::offset::{Offset, TimeZone};
use crate::{Datelike, Timelike, Weekday};
use std::borrow::Borrow;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Sub};

/// 感知时区的日期时间结构
#[derive(Clone, Copy, PartialOrd, PartialEq)]
pub struct DateTime<Tz: TimeZone> {
    datetime: BaseDateTime,
    offset: Tz::Offset,
}

/// 最小的可能 `DateTime`
#[allow(dead_code)]
pub const MIN_DATETIME: DateTime<Utc> = DateTime {
    datetime: crate::base::datetime::MIN_DATETIME,
    offset: Utc,
};
/// 最大的可能 `DateTime`
#[allow(dead_code)]
pub const MAX_DATETIME: DateTime<Utc> = DateTime {
    datetime: crate::base::datetime::MAX_DATETIME,
    offset: Utc,
};

impl<Tz: TimeZone> DateTime<Tz> {
    /// 通过给定的 `UTC` 日期时间以及偏移量，创建新的 `DateTime`
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::datetime::DateTime;
    /// use ylong_time::offset::utc::Utc;
    /// use ylong_time::base::datetime::BaseDateTime;
    /// use ylong_time::offset::TimeZone;
    ///
    /// let datetime = DateTime::<Utc>::from_utc(BaseDateTime::from_timestamp(61, 0).unwrap(), Utc);
    /// assert_eq!(Utc.timestamp(61, 0).unwrap(), datetime);
    /// ```
    pub fn from_utc(datetime: BaseDateTime, offset: Tz::Offset) -> DateTime<Tz> {
        DateTime { datetime, offset }
    }

    /// 通过给定的 'Local' 日期时间以及偏移量，创建新的 'DateTime'
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::datetime::DateTime;
    /// use ylong_time::Utc;
    /// use ylong_time::base::datetime::BaseDateTime;
    /// use ylong_time::offset::TimeZone;
    ///
    /// let datetime = DateTime::<Utc>::from_local(BaseDateTime::from_timestamp(61, 0).unwrap(), Utc);
    /// assert_eq!(Utc.timestamp(61, 0).unwrap(), datetime);
    /// ```
    pub fn from_local(datetime: BaseDateTime, offset: Tz::Offset) -> DateTime<Tz> {
        DateTime::from_utc(datetime - offset.fix(), offset)
    }

    /// 返回当前日期时间的 `Date` 组成部分
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::offset::utc::Utc;
    /// use ylong_time::offset::TimeZone;
    ///
    /// let date = Utc.ymd(2021, 1, 1).unwrap();
    /// let datetime = date.and_hms(0, 0, 0).unwrap();
    ///
    /// assert_eq!(datetime.date(), date);
    /// assert_eq!(datetime.date().and_hms(1, 1, 1).unwrap(), date.and_hms(1, 1, 1).unwrap());
    /// ```
    pub fn date(&self) -> Date<Tz> {
        Date::from_utc(self.base_local().date(), self.offset.clone())
    }

    /// 返回当前日期时间的 `BaseDate` 组成部分
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::offset::utc::Utc;
    /// use ylong_time::offset::TimeZone;
    /// use ylong_time::offset::fixed::FixedOffset;
    ///
    /// let datetime = Utc.ymd(2021, 1, 1).unwrap().and_hms(0, 0, 0).unwrap();
    /// let other_datetime = FixedOffset::east(23).unwrap().ymd(2021, 1, 1).unwrap().and_hms(0, 0, 0).unwrap();
    /// assert_eq!(datetime.date_base(), other_datetime.date_base());
    /// ```
    pub fn date_base(&self) -> BaseDate {
        self.base_local().date()
    }

    /// 返回当前日期时间的本地日期时间 `BaseDateTime`
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::offset::TimeZone;
    /// use ylong_time::datetime::DateTime;
    /// use ylong_time::base::datetime::BaseDateTime;
    /// use ylong_time::offset::fixed::FixedOffset;
    ///
    /// let base_datetime = BaseDateTime::from_timestamp(86399, 0).unwrap();
    /// let datetime: DateTime<FixedOffset> = DateTime::from_utc(BaseDateTime::from_timestamp(0, 0).unwrap(), FixedOffset::east(86399).unwrap());
    /// assert_eq!(datetime.base_local(), base_datetime);
    /// ```
    pub fn base_local(&self) -> BaseDateTime {
        self.datetime + self.offset.fix()
    }

    /// 返回当前日期时间的 `Time` 组成部分
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::offset::TimeZone;
    /// use ylong_time::datetime::DateTime;
    /// use ylong_time::base::datetime::BaseDateTime;
    /// use ylong_time::offset::fixed::FixedOffset;
    ///
    /// let base_datetime = BaseDateTime::from_timestamp(86399, 0).unwrap();
    /// let datetime: DateTime<FixedOffset> = DateTime::from_utc(
    ///     BaseDateTime::from_timestamp(0, 0).unwrap(),
    ///     FixedOffset::east(86399).unwrap()
    /// );
    /// assert_eq!(datetime.time(), base_datetime.time());
    /// ```
    pub fn time(&self) -> BaseTime {
        self.datetime.time() + self.offset.fix()
    }

    /// 返回从 UTC 1970.1.1 开始的时间戳秒数，不包含闰秒
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::datetime::DateTime;
    /// use ylong_time::base::datetime::BaseDateTime;
    /// use ylong_time::offset::fixed::FixedOffset;
    ///
    /// let datetime: DateTime<FixedOffset> = DateTime::from_utc(
    ///     BaseDateTime::from_timestamp(0, 0).unwrap(),
    ///     FixedOffset::east(86399).unwrap()
    /// );
    /// assert_eq!(
    ///     datetime.timestamp().unwrap(),
    ///     BaseDateTime::from_timestamp(0, 0).unwrap().timestamp().unwrap()
    /// );
    /// ```
    pub fn timestamp(&self) -> Result<i64, TimeError> {
        self.datetime.timestamp()
    }

    /// 返回从 UTC 1970.1.1 开始的时间戳毫秒数，不包含闰秒
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::datetime::DateTime;
    /// use ylong_time::{FixedOffset, BaseDateTime};
    ///
    /// let datetime: DateTime<FixedOffset> = DateTime::from_utc(
    ///     BaseDateTime::from_timestamp(0, 0).unwrap(),
    ///     FixedOffset::east(86399).unwrap()
    /// );
    /// assert_eq!(
    ///     datetime.timestamp_millis().unwrap(),
    ///     BaseDateTime::from_timestamp(0, 0).unwrap().timestamp_millis().unwrap()
    /// );
    /// ```
    pub fn timestamp_millis(&self) -> Result<i64, TimeError> {
        self.datetime.timestamp_millis()
    }

    /// 返回从 UTC 1970.1.1 开始的时间戳微秒数，不包含闰秒
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::datetime::DateTime;
    /// use ylong_time::{FixedOffset, BaseDateTime};
    ///
    /// let datetime: DateTime<FixedOffset> = DateTime::from_utc(
    ///     BaseDateTime::from_timestamp(0, 0).unwrap(),
    ///     FixedOffset::east(86399).unwrap()
    /// );
    /// assert_eq!(
    ///     datetime.timestamp_micros().unwrap(),
    ///     BaseDateTime::from_timestamp(0, 0).unwrap().timestamp_micros().unwrap()
    /// );
    /// ```
    pub fn timestamp_micros(&self) -> Result<i64, TimeError> {
        self.datetime.timestamp_micros()
    }

    /// 返回从 UTC 1970.1.1 开始的时间戳纳秒数，不包含闰秒
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::datetime::DateTime;
    /// use ylong_time::{FixedOffset, BaseDateTime};
    ///
    /// let datetime: DateTime<FixedOffset> = DateTime::from_utc(
    ///     BaseDateTime::from_timestamp(0, 0).unwrap(),
    ///     FixedOffset::east(86399).unwrap()
    /// );
    /// assert_eq!(
    ///     datetime.timestamp_nanos().unwrap(),
    ///     BaseDateTime::from_timestamp(0, 0).unwrap().timestamp_nanos().unwrap()
    /// );
    /// ```
    pub fn timestamp_nanos(&self) -> Result<i128, TimeError> {
        self.datetime.timestamp_nanos()
    }

    /// 返回当前日期时间 `DateTime` 相关的偏移量
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::datetime::DateTime;
    /// use ylong_time::base::datetime::BaseDateTime;
    /// use ylong_time::offset::fixed::FixedOffset;
    ///
    /// let datetime: DateTime<FixedOffset> = DateTime::from_utc(
    ///     BaseDateTime::from_timestamp(0, 0).unwrap(),
    ///     FixedOffset::east(86399).unwrap()
    /// );
    /// assert_eq!(*datetime.offset(), FixedOffset::east(86399).unwrap());
    /// ```
    pub fn offset(&self) -> &Tz::Offset {
        &self.offset
    }

    /// 返回当前日期时间相关的时区
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::datetime::DateTime;
    /// use ylong_time::offset::fixed::FixedOffset;
    /// use ylong_time::base::datetime::BaseDateTime;
    ///
    /// let datetime: DateTime<FixedOffset> = DateTime::from_utc(
    ///     BaseDateTime::from_timestamp(0, 0).unwrap(),
    ///     FixedOffset::east(86399).unwrap()
    /// );
    /// assert_eq!(datetime.timezone(), FixedOffset::east(86399).unwrap());
    /// ```
    pub fn timezone(&self) -> Tz {
        TimeZone::from_offset(&self.offset)
    }

    /// 当前日期时间加上指定的 `Duration`
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::datetime::DateTime;
    /// use ylong_time::base::datetime::BaseDateTime;
    /// use ylong_time::offset::utc::Utc;
    /// use ylong_time::base::duration::Duration;
    ///
    /// let datetime: DateTime<Utc> = DateTime::from_utc(
    ///     BaseDateTime::from_timestamp(0, 0).unwrap(),
    ///     Utc
    /// );
    /// let other_datetime: DateTime<Utc> = DateTime::from_utc(
    ///     BaseDateTime::from_timestamp(86400, 0).unwrap(),
    ///     Utc
    /// );
    /// let duration = Duration::days(1).unwrap();
    /// assert_eq!(datetime + duration, other_datetime);
    /// ```
    pub fn checked_add_signed(self, rhs: Duration) -> Result<DateTime<Tz>, TimeError> {
        let base_datetime = self.datetime.checked_add_signed(rhs);
        match base_datetime {
            Ok(base_datetime) => Ok(DateTime {
                datetime: base_datetime,
                offset: self.offset,
            }),
            Err(err) => Err(err),
        }
    }

    /// 当前日期时间减去指定的 `Duration`
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::datetime::DateTime;
    /// use ylong_time::base::datetime::BaseDateTime;
    /// use ylong_time::offset::utc::Utc;
    /// use ylong_time::base::duration::Duration;
    ///
    /// let datetime: DateTime<Utc> = DateTime::from_utc(
    ///     BaseDateTime::from_timestamp(0, 0).unwrap(),
    ///     Utc
    /// );
    /// let other_datetime: DateTime<Utc> = DateTime::from_utc(
    ///     BaseDateTime::from_timestamp(-86400, 0).unwrap(),
    ///     Utc
    /// );
    /// let duration = Duration::days(1).unwrap();
    /// assert_eq!(datetime - duration, other_datetime);
    /// ```
    pub fn checked_sub_signed(self, rhs: Duration) -> Result<DateTime<Tz>, TimeError> {
        let base_datetime = self.datetime.checked_sub_signed(rhs);
        match base_datetime {
            Ok(base_datetime) => Ok(DateTime {
                datetime: base_datetime,
                offset: self.offset,
            }),
            Err(err) => Err(err),
        }
    }

    /// 当前日期时间减去指定的 `DateTime`
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::datetime::DateTime;
    /// use ylong_time::base::datetime::BaseDateTime;
    /// use ylong_time::offset::utc::Utc;
    /// use ylong_time::base::duration::Duration;
    ///
    /// let datetime: DateTime<Utc> = DateTime::from_utc(
    ///     BaseDateTime::from_timestamp(0, 0).unwrap(),
    ///     Utc
    /// );
    /// let other_datetime: DateTime<Utc> = DateTime::from_utc(
    ///     BaseDateTime::from_timestamp(-86400, 0).unwrap(),
    ///     Utc
    /// );
    ///
    /// assert_eq!(datetime - other_datetime, Duration::days(1).unwrap());
    /// ```
    pub fn signed_duration_since(self, rhs: DateTime<Tz>) -> Result<Duration, TimeError> {
        self.datetime.signed_duration_since(rhs.datetime)
    }
}

/// 将本地日期时间通过给定的函数转换成其他的日期时间
fn map_local<Tz: TimeZone, F>(dt: &DateTime<Tz>, mut f: F) -> Result<DateTime<Tz>, TimeError>
where
    F: FnMut(BaseDateTime) -> Result<BaseDateTime, TimeError>,
{
    f(dt.base_local()).map(|datetime| dt.timezone().from_local_datetime(&datetime))
}

impl<Tz: TimeZone> Datelike for DateTime<Tz> {
    fn year(&self) -> i32 {
        self.base_local().year()
    }

    fn month(&self) -> u32 {
        self.base_local().month()
    }

    fn month0(&self) -> u32 {
        self.base_local().month0()
    }

    fn day(&self) -> u32 {
        self.base_local().day()
    }

    fn day0(&self) -> u32 {
        self.base_local().day0()
    }

    fn ordinal(&self) -> u32 {
        self.base_local().ordinal()
    }

    fn ordinal0(&self) -> u32 {
        self.base_local().ordinal0()
    }

    fn weekday(&self) -> Weekday {
        self.base_local().weekday()
    }

    fn with_year(&self, year: i32) -> Result<Self, TimeError> {
        map_local(self, |datetime| datetime.with_year(year))
    }

    fn with_month(&self, month: u32) -> Result<Self, TimeError> {
        map_local(self, |datetime| datetime.with_month(month))
    }

    fn with_month0(&self, month0: u32) -> Result<Self, TimeError> {
        map_local(self, |datetime| datetime.with_month0(month0))
    }

    fn with_day(&self, day: u32) -> Result<Self, TimeError> {
        map_local(self, |datetime| datetime.with_day(day))
    }

    fn with_day0(&self, day0: u32) -> Result<Self, TimeError> {
        map_local(self, |datetime| datetime.with_day0(day0))
    }

    fn with_ordinal(&self, ordinal: u32) -> Result<Self, TimeError> {
        map_local(self, |datetime| datetime.with_ordinal(ordinal))
    }

    fn with_ordinal0(&self, ordinal0: u32) -> Result<Self, TimeError> {
        map_local(self, |datetime| datetime.with_ordinal0(ordinal0))
    }
}

impl<Tz: TimeZone> Timelike for DateTime<Tz> {
    fn hour(&self) -> u32 {
        self.base_local().hour()
    }

    fn minute(&self) -> u32 {
        self.base_local().minute()
    }

    fn second(&self) -> u32 {
        self.base_local().second()
    }

    fn nanosecond(&self) -> u32 {
        self.base_local().nanosecond()
    }

    fn with_hour(&self, hour: u32) -> Result<Self, TimeError> {
        map_local(self, |datetime| datetime.with_hour(hour))
    }

    fn with_minute(&self, minute: u32) -> Result<Self, TimeError> {
        map_local(self, |datetime| datetime.with_minute(minute))
    }

    fn with_second(&self, second: u32) -> Result<Self, TimeError> {
        map_local(self, |datetime| datetime.with_second(second))
    }

    fn with_nanosecond(&self, nanosecond: u32) -> Result<Self, TimeError> {
        map_local(self, |datetime| datetime.with_nanosecond(nanosecond))
    }
}

impl<Tz: TimeZone> Add<Duration> for DateTime<Tz> {
    type Output = DateTime<Tz>;

    fn add(self, rhs: Duration) -> Self::Output {
        self.checked_add_signed(rhs).unwrap()
    }
}

impl<Tz: TimeZone> Sub<Duration> for DateTime<Tz> {
    type Output = DateTime<Tz>;

    fn sub(self, rhs: Duration) -> Self::Output {
        self.checked_sub_signed(rhs).unwrap()
    }
}

impl<Tz: TimeZone> Sub<DateTime<Tz>> for DateTime<Tz> {
    type Output = Duration;

    fn sub(self, rhs: DateTime<Tz>) -> Self::Output {
        self.signed_duration_since(rhs).unwrap()
    }
}

impl<Tz: TimeZone> Debug for DateTime<Tz> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{:?}", self.base_local(), self.offset)
    }
}

impl<Tz: TimeZone> Display for DateTime<Tz> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.base_local(), self.offset)
    }
}

impl DateTime<FixedOffset> {
    /// 解析 `RFC 3339` 或 `ISO 8601` 格式的日期与时间字符串，例如 `1996-12-19T16:39:57-08:00`，
    /// 并且返回新的且拥有解析后 `FixedOffset` 的 `DateTime` 结构
    pub fn parse_from_rfc3339(s: &str) -> Result<DateTime<FixedOffset>, TimeError> {
        const ITEMS: &[Item] = &[Item::Specific(Specific::RFC3339)];
        let mut parsed = Parsed::new();
        parse(&mut parsed, s, ITEMS.iter())?;
        parsed.as_datetime()
    }

    /// 根据指定的格式解析日期与时间字符串，
    /// 并返回新的且拥有解析后 'FixedOffset' 的 'DateTime' 结构
    pub fn parse_from_str(s: &str, fmt: &str) -> Result<DateTime<FixedOffset>, TimeError> {
        let mut parsed = Parsed::new();
        parse(&mut parsed, s, StrftimeItems::new(fmt))?;
        parsed.as_datetime()
    }
}

impl<Tz: TimeZone> DateTime<Tz>
where
    Tz::Offset: std::fmt::Display,
{
    /// 将 `DateTime` 格式化为 `RFC 3339` 或 `ISO 8601` 的日期与时间字符串，例如 `1996-12-19T16:39:57-08:00`
    pub fn to_rfc3339(&self) -> String {
        const ITEMS: &[Item<'static>] = &[Item::Specific(Specific::RFC3339)];
        self.format_with_items(ITEMS.iter()).to_string()
    }

    /// 已指定的格式格式化合并后的日期与时间
    fn format_with_items<'a, I, B>(&self, items: I) -> DelayedFormat<I>
    where
        I: Iterator<Item = B> + Clone,
        B: Borrow<Item<'a>>,
    {
        let local = self.base_local();
        DelayedFormat::new_with_offset(local.date(), local.time(), &self.offset, items)
    }

    /// 按照自己制定的格式格式化日期与时间
    pub fn format<'a>(&self, fmt: &'a str) -> DelayedFormat<StrftimeItems<'a>> {
        self.format_with_items(StrftimeItems::new(fmt))
    }
}

#[cfg(test)]
mod test {
    include!("../tests/ut/ut_datatime.rs");
}
