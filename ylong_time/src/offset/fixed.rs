use crate::base::date::BaseDate;
use crate::base::datetime::BaseDateTime;
use crate::base::duration::Duration;
use crate::base::time::BaseTime;
use crate::datetime::DateTime;
use crate::error::{BaseErrorKind, TimeError};
use crate::offset::{Offset, TimeZone};
use crate::Timelike;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Sub};

/// 由于时区所产生的时间偏移，并且范围是从 UTC-23:59:59 到 UTC+23:59:59
#[derive(Copy, Clone, PartialOrd, PartialEq, Eq)]
pub struct FixedOffset {
    local_minus_utc: i32,
}

impl FixedOffset {
    /// 为东半球的时区创建新的 `FixedOffset`
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::offset::fixed::FixedOffset;
    ///
    /// let fixed_offset = FixedOffset::east(86399).unwrap();
    /// assert_eq!(fixed_offset.local_minus_utc(), 86399);
    /// ```
    pub fn east(secs: i32) -> Result<FixedOffset, TimeError> {
        if secs > -86400 && secs < 86400 {
            Ok(FixedOffset {
                local_minus_utc: secs,
            })
        } else {
            Err(TimeError::from(BaseErrorKind::InvalidSec))
        }
    }

    /// 为西半球的时区创建新的 `FixedOffset`
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::offset::fixed::FixedOffset;
    ///
    /// let fixed_offset = FixedOffset::west(86399).unwrap();
    /// assert_eq!(fixed_offset.local_minus_utc(), -86399);
    /// ```
    pub fn west(secs: i32) -> Result<FixedOffset, TimeError> {
        if secs > -86400 && secs < 86400 {
            Ok(FixedOffset {
                local_minus_utc: -secs,
            })
        } else {
            Err(TimeError::from(BaseErrorKind::InvalidSec))
        }
    }

    /// 返回从 `UTC` 时区转换至 `LOCAL` 时所变化的秒数
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::offset::fixed::FixedOffset;
    ///
    /// let fixed_offset = FixedOffset::west(86399).unwrap();
    /// assert_eq!(fixed_offset.local_minus_utc(), -86399);
    /// ```
    pub fn local_minus_utc(&self) -> i32 {
        self.local_minus_utc
    }

    /// 返回从 `LOCAL` 时区转换至 `UTC` 时所变化的秒数
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::offset::fixed::FixedOffset;
    ///
    /// let fixed_offset = FixedOffset::west(86399).unwrap();
    /// assert_eq!(fixed_offset.utc_minus_local(), 86399);
    /// ```
    pub fn utc_minus_local(&self) -> i32 {
        -self.local_minus_utc
    }
}

impl TimeZone for FixedOffset {
    type Offset = FixedOffset;

    fn from_offset(offset: &Self::Offset) -> Self {
        *offset
    }

    fn offset_from_local_date(&self, _local: &BaseDate) -> Self::Offset {
        *self
    }

    fn offset_from_local_datetime(&self, _local: &BaseDateTime) -> Self::Offset {
        *self
    }

    fn offset_from_utc_date(&self, _utc: &BaseDate) -> Self::Offset {
        *self
    }

    fn offset_from_utc_datetime(&self, _utc: &BaseDateTime) -> Self::Offset {
        *self
    }
}

impl Offset for FixedOffset {
    fn fix(&self) -> FixedOffset {
        *self
    }
}

impl Debug for FixedOffset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let offset = self.local_minus_utc;
        let (sign, offset) = if offset < 0 {
            ('-', -offset)
        } else {
            ('+', offset)
        };
        let (mins, sec) = (offset / 60, offset % 60);
        let (hour, min) = (mins / 60, mins % 60);
        if sec == 0 {
            write!(f, "{sign}{hour:02}:{min:02}")
        } else {
            write!(f, "{sign}{hour:02}:{min:02}:{sec:02}")
        }
    }
}

impl Display for FixedOffset {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

fn add_with_leapsecond<T>(lhs: &T, rhs: i32) -> T
where
    T: Timelike + Add<Duration, Output = T>,
{
    // 获取当前被加数据的纳秒数，然后保存其不包含纳秒的部分
    let nanos = lhs.nanosecond();
    let lhs = lhs.with_nanosecond(0).unwrap();
    // 将当前数据增加一段时间，并将之前的纳秒部分还原，返回变化后数据
    (lhs + Duration::seconds(i64::from(rhs)).unwrap())
        .with_nanosecond(nanos)
        .unwrap()
}

impl Add<FixedOffset> for BaseTime {
    type Output = BaseTime;

    fn add(self, rhs: FixedOffset) -> Self::Output {
        add_with_leapsecond(&self, rhs.local_minus_utc)
    }
}

impl Sub<FixedOffset> for BaseTime {
    type Output = BaseTime;

    fn sub(self, rhs: FixedOffset) -> Self::Output {
        add_with_leapsecond(&self, -rhs.local_minus_utc)
    }
}

impl Add<FixedOffset> for BaseDateTime {
    type Output = BaseDateTime;

    fn add(self, rhs: FixedOffset) -> Self::Output {
        add_with_leapsecond(&self, rhs.local_minus_utc)
    }
}

impl Sub<FixedOffset> for BaseDateTime {
    type Output = BaseDateTime;

    fn sub(self, rhs: FixedOffset) -> Self::Output {
        add_with_leapsecond(&self, -rhs.local_minus_utc)
    }
}

impl<Tz: TimeZone> Add<FixedOffset> for DateTime<Tz> {
    type Output = DateTime<Tz>;

    fn add(self, rhs: FixedOffset) -> DateTime<Tz> {
        add_with_leapsecond(&self, rhs.local_minus_utc)
    }
}

impl<Tz: TimeZone> Sub<FixedOffset> for DateTime<Tz> {
    type Output = DateTime<Tz>;

    fn sub(self, rhs: FixedOffset) -> DateTime<Tz> {
        add_with_leapsecond(&self, -rhs.local_minus_utc)
    }
}

#[cfg(test)]
mod test {
    include!("../../tests/ut/offset/ut_fixed.rs");
}
