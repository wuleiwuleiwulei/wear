pub mod offset;

pub mod base {
    pub mod date;
    pub mod datetime;
    pub mod duration;
    pub mod kernel;
    pub mod time;
}

pub mod date;
pub mod datetime;
pub mod error;
pub mod format;
mod sys;
mod weekday;

pub use base::date::BaseDate;
pub use base::datetime::BaseDateTime;
pub use base::duration::Duration;
pub use base::time::BaseTime;
pub use format::parse::{parse, Parsed};
pub use offset::fixed::FixedOffset;
pub use offset::local::Local;
pub use offset::utc::Utc;
pub use weekday::Weekday;

use crate::base::kernel::yo_to_days;
use crate::error::TimeError;

pub trait Datelike: Sized {
    /// 返回当前日期的年份
    fn year(&self) -> i32;

    /// 返回当前日期的月份（从一月份开始）
    fn month(&self) -> u32;

    /// 返回当前日期的月份（从零月份开始）
    fn month0(&self) -> u32;

    /// 返回当前日期的天数（每月的第几天，从第一天开始计算）
    fn day(&self) -> u32;

    /// 返回当前日期的天数（每月的第几天，从第零天开始计算）
    fn day0(&self) -> u32;

    /// 返回当前日期的天数（每年的第几天，从第一天开始计算）
    fn ordinal(&self) -> u32;

    /// 返回当前日期的天数（每年的第几天，从第零天开始计算）
    fn ordinal0(&self) -> u32;

    /// Return the day of the week of the current date.
    fn weekday(&self) -> Weekday;

    /// 替换年份，并且可以判断替换年份后的日期是否有效
    fn with_year(&self, year: i32) -> Result<Self, TimeError>;

    /// 替换月份（从一月份开始计算），并且可以判断替换月份后的日期是否有效
    fn with_month(&self, month: u32) -> Result<Self, TimeError>;

    /// 替换月份（从零月份开始计算），并且可以判断替换月份后的日期是否有效
    fn with_month0(&self, month0: u32) -> Result<Self, TimeError>;

    /// 替换天数（每月第几天，从第一天开始计算），并且可以判断替换后的日期是否有效
    fn with_day(&self, day: u32) -> Result<Self, TimeError>;

    /// 替换天数（每月第几天，从第零天开始计算），并且可以判断替换后的日期是否有效
    fn with_day0(&self, day0: u32) -> Result<Self, TimeError>;

    /// 替换天数（每年第几天，从第一天开始计算），并且可以判断替换后的日期是否有效
    fn with_ordinal(&self, ordinal: u32) -> Result<Self, TimeError>;

    /// 替换天数（每年第几天，从第零天开始计算），并且可以判断替换后的日期是否有效
    fn with_ordinal0(&self, ordinal0: u32) -> Result<Self, TimeError>;

    /// 通过当前日期计算出从公历初始值已经经过了多少天
    fn num_days_from_ce(&self) -> Result<i32, TimeError> {
        yo_to_days(self.year(), self.ordinal())
    }
}

pub trait Timelike: Sized {
    /// 返回当前时间代表的小时数
    fn hour(&self) -> u32;

    /// 返回当前时间代表的分钟数
    fn minute(&self) -> u32;

    /// 返回当前时间代表的秒数，即使存在闰秒的情况下也不会返回六十秒的情况
    fn second(&self) -> u32;

    /// 返回当前时间代表的纳秒数
    fn nanosecond(&self) -> u32;

    /// 将当前时间代表的小时替换为新的小时数
    fn with_hour(&self, hour: u32) -> Result<Self, TimeError>;

    /// 将当前时间代表的分钟替换为新的分钟数
    fn with_minute(&self, minute: u32) -> Result<Self, TimeError>;

    /// 将当前时间所代表的秒数替换为新的秒数，不包含闰秒的情况，因此传入的秒数范围也在 `0 ~ 59`
    fn with_second(&self, second: u32) -> Result<Self, TimeError>;

    /// 将当前时间所代表的纳秒替换为新的纳秒数
    fn with_nanosecond(&self, nanosecond: u32) -> Result<Self, TimeError>;

    /// 从一天零点开始，计算经过的秒数，不包含闰秒的情况
    fn num_seconds_from_midnight(&self) -> u32 {
        self.hour() * 3600 + self.minute() * 60 + self.second()
    }
}
