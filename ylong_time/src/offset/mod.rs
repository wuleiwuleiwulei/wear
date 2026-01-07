#![allow(clippy::wrong_self_convention)] // TODO: 修改该 clippy 问题。

use crate::base::date::BaseDate;
use crate::base::datetime::BaseDateTime;
use crate::date::Date;
use crate::datetime::DateTime;
use crate::error::TimeError;
use crate::format::parse::{parse, Parsed};
use crate::format::strftime::StrftimeItems;
use crate::offset::fixed::FixedOffset;
use std::fmt::{Debug, Display};

pub mod fixed;
pub mod local;
pub mod utc;

pub trait Offset: Sized + Clone + Debug + Display {
    /// 返回 `UTC` 到 `LOCAL` 的偏移量大小
    fn fix(&self) -> FixedOffset;
}

/// 时区结构
pub trait TimeZone: Sized {
    /// 时区相关的偏移类型
    type Offset: Offset;

    /// 通过年月日以及当前时区，创建新的 `Date`
    fn ymd(&self, year: i32, month: u32, day: u32) -> Result<Date<Self>, TimeError> {
        let base_date = BaseDate::from_ymd(year, month, day);
        match base_date {
            Ok(basedate) => Ok(self.from_local_date(&basedate)),
            Err(err) => Err(err),
        }
    }

    /// 通过指定年份以及当前年份的第几天、当前时区，创建 `Date` 结构
    fn yo(&self, year: i32, ordinal: u32) -> Result<Date<Self>, TimeError> {
        let base_date = BaseDate::from_yo(year, ordinal);
        match base_date {
            Ok(basedate) => Ok(self.from_local_date(&basedate)),
            Err(err) => Err(err),
        }
    }

    /// 通过不包含闰秒的秒数创建时间戳
    fn timestamp(&self, secs: i64, nanos: u32) -> Result<DateTime<Self>, TimeError> {
        let base_datetime = BaseDateTime::from_timestamp(secs, nanos);
        match base_datetime {
            Ok(base_datetime) => Ok(self.from_utc_datetime(&base_datetime)),
            Err(err) => Err(err),
        }
    }

    /// 通过不包含闰秒的毫秒数创建时间戳
    fn timestamp_millis(&self, millis: i64) -> Result<DateTime<Self>, TimeError> {
        let (mut secs, mut millis) = (millis / 1000, millis % 1000);
        if millis < 0 {
            secs -= 1;
            millis += 1000;
        }
        self.timestamp(secs, millis as u32 * 1_000_000)
    }

    /// 通过不包含闰秒的纳秒数创建时间戳
    fn timestamp_nanos(&self, nanos: i64) -> Result<DateTime<Self>, TimeError> {
        let (mut secs, mut nanos) = (nanos / 1_000_000_000, nanos % 1_000_000_000);
        if nanos < 0 {
            secs -= 1;
            nanos += 1_000_000_000;
        }
        self.timestamp(secs, nanos as u32)
    }

    /// 通过偏移量创建新的 `TimeZone`
    fn from_offset(offset: &Self::Offset) -> Self;

    /// 通过指定的本地 `BaseDate` 创建新的 `Offset`
    fn offset_from_local_date(&self, local: &BaseDate) -> Self::Offset;

    /// 通过指定的本地 `BaseDateTime` 创建新的 `Offset`
    fn offset_from_local_datetime(&self, local: &BaseDateTime) -> Self::Offset;

    /// 将当前本地日期 `BaseDate` 转换为感知时区的 `Date`
    fn from_local_date(&self, local: &BaseDate) -> Date<Self> {
        let offset = self.offset_from_local_date(local);
        Date::from_utc(*local, offset)
    }

    /// 将当前本地日期时间 `BaseDateTime` 转换为感知时区的 `DateTime`
    fn from_local_datetime(&self, local: &BaseDateTime) -> DateTime<Self> {
        let offset = self.offset_from_local_datetime(local);
        DateTime::from_utc(*local - offset.fix(), offset)
    }

    /// 通过指定的世界协调时 `BaseDate` 创建新的 `Offset`
    fn offset_from_utc_date(&self, utc: &BaseDate) -> Self::Offset;

    /// 通过指定的世界协调时 `BaseDateTime` 创建新的 `Offset`
    fn offset_from_utc_datetime(&self, utc: &BaseDateTime) -> Self::Offset;

    /// 将当前世界协调时日期 `BaseDate` 转换为本地时间
    fn from_utc_date(&self, utc: &BaseDate) -> Date<Self> {
        Date::from_utc(*utc, self.offset_from_utc_date(utc))
    }

    /// 将当前世界协调时日期时间 `BaseDateTime` 转换为本地时间
    fn from_utc_datetime(&self, utc: &BaseDateTime) -> DateTime<Self> {
        let offset = self.offset_from_utc_datetime(utc);
        DateTime::from_utc(*utc, offset)
    }

    /// 将指定了自定义格式的字符串解析为 'DateTime' 结构体
    fn datetime_from_str(&self, s: &str, fmt: &str) -> Result<DateTime<Self>, TimeError> {
        let mut parsed = Parsed::new();
        parse(&mut parsed, s, StrftimeItems::new(fmt))?;
        parsed.as_datetime_with_timezone(self)
    }
}
