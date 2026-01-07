use crate::base::date::BaseDate;
use crate::base::datetime::BaseDateTime;
use crate::date::Date;
use crate::datetime::DateTime;
use crate::error::TimeError;
use crate::offset::fixed::FixedOffset;
use crate::offset::{Offset, TimeZone};
use std::fmt::{Debug, Display, Formatter};
use std::time::{SystemTime, UNIX_EPOCH};

/// 世界协调时时区
#[derive(Clone, Copy, PartialOrd, PartialEq, Eq)]
pub struct Utc;

impl Utc {
    /// 返回当前的日期，返回 `Date`
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::offset::utc::Utc;
    ///
    /// let today_date = Utc::today();
    /// match today_date {
    ///     Ok(today_date) => println!("{}", today_date),
    ///     Err(err) => println!("{}", err),
    /// }
    /// ```
    pub fn today() -> Result<Date<Utc>, TimeError> {
        let now = Utc::now();
        match now {
            Ok(now) => Ok(now.date()),
            Err(err) => Err(err),
        }
    }

    /// 返回当前的日期时间，返回 `DateTime`
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::offset::utc::Utc;
    ///
    /// let now = Utc::now();
    /// match now {
    ///     Ok(now) => println!("{}", now),
    ///     Err(err) => println!("{}", err),
    /// }
    /// ```
    pub fn now() -> Result<DateTime<Utc>, TimeError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before Unix epoch");
        let base_datetime = BaseDateTime::from_timestamp(now.as_secs() as i64, now.subsec_nanos());
        match base_datetime {
            Ok(base_datetime) => Ok(DateTime::from_utc(base_datetime, Utc)),
            Err(err) => Err(err),
        }
    }
}

impl TimeZone for Utc {
    type Offset = Utc;

    fn from_offset(_offset: &Self::Offset) -> Self {
        Utc
    }

    fn offset_from_local_date(&self, _local: &BaseDate) -> Self::Offset {
        Utc
    }

    fn offset_from_local_datetime(&self, _local: &BaseDateTime) -> Self::Offset {
        Utc
    }

    fn offset_from_utc_date(&self, _utc: &BaseDate) -> Self::Offset {
        Utc
    }

    fn offset_from_utc_datetime(&self, _utc: &BaseDateTime) -> Self::Offset {
        Utc
    }
}

impl Offset for Utc {
    fn fix(&self) -> FixedOffset {
        FixedOffset::east(0).unwrap()
    }
}

impl Debug for Utc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Z")
    }
}

impl Display for Utc {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "UTC")
    }
}
