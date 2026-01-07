use crate::base::datetime::BaseDateTime;
use crate::base::duration::Duration;
use crate::base::kernel;
use crate::base::kernel::{days_to_yo, yo_to_days, Mdf, Of, SundayLetter};
use crate::base::time::BaseTime;
use crate::error::{BaseErrorKind, TimeError};
use crate::format::strftime::StrftimeItems;
use crate::format::{DelayedFormat, Item};
use crate::{Datelike, Utc, Weekday};
use std::borrow::Borrow;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::result::Result::Err;

/// 日期最小年份
const MIN_YEAR: i32 = kernel::MIN_YEAR;
/// 日期最大年份
const MAX_YEAR: i32 = kernel::MAX_YEAR;

/// 不含时区的基础日期结构
/// 日历使用公历，格林乔治历法，该历法的年份从 `BCE 262145-1-1` 至 `CE 262143-12-31`
#[derive(Copy, Clone, PartialOrd, PartialEq, Eq)]
pub struct BaseDate {
    // (year << 13) | mdf
    ymdf: i32,
}

/// 最小的 `BaseDate` 日期值为公元前 `262145-1-1`，并且该年主日字母为 `FE`。
pub const MIN_DATE: BaseDate = BaseDate {
    ymdf: (MIN_YEAR << 13) | (1 << 9) | (1 << 4) | 0b0111, /*FE*/
};
/// 最大的 `BaseDate` 日期值为公元 `262143-12-31`，并且该年主日字母为 `F`。
pub const MAX_DATE: BaseDate = BaseDate {
    ymdf: (MAX_YEAR << 13) | (12 << 9) | (31 << 4) | 0b1111, /*F*/
};

impl BaseDate {
    /// 通过 `Mdf` 结构创建 `BaseDate` 结构
    fn from_mdf(year: i32, mdf: Mdf) -> Result<BaseDate, TimeError> {
        if (MIN_YEAR..=MAX_YEAR).contains(&year) && mdf.valid() {
            let Mdf(mdf) = mdf;
            Ok(BaseDate {
                ymdf: (year << 13) | mdf as i32,
            })
        } else {
            Err(TimeError::from(BaseErrorKind::InvalidYear))
        }
    }

    /// 通过 `Of` 结构创建 `BaseDate` 结构
    fn from_of(year: i32, of: Of) -> Result<BaseDate, TimeError> {
        BaseDate::from_mdf(year, of.as_mdf())
    }

    /// 返回当前结构所代表的 `Mdf` 结构
    fn mdf(&self) -> Mdf {
        Mdf((self.ymdf & 0b1_1111_1111_1111) as u32)
    }

    /// 返回当前结构所代表的 `Of` 结构
    fn of(&self) -> Of {
        self.mdf().as_of()
    }

    /// 将当前 `BaseDate` 结构中的 `mdf` 信息替换为新的
    fn with_mdf(&self, mdf: Mdf) -> Result<BaseDate, TimeError> {
        if mdf.valid() {
            let Mdf(mdf) = mdf;
            Ok(BaseDate {
                ymdf: (self.ymdf & !0b1_1111_1111_1111) | mdf as i32,
            })
        } else {
            Err(TimeError::from(BaseErrorKind::InvalidMdf))
        }
    }

    /// 将当前 `BaseDate` 结构中的 `of` 信息替换为新的
    fn with_of(&self, of: Of) -> Result<BaseDate, TimeError> {
        self.with_mdf(of.as_mdf())
    }

    /// 通过年月日直接创建 `BaseDate`
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::kernel::{Mdf, A};
    /// use ylong_time::base::date::BaseDate;
    ///
    /// let date = BaseDate::from_ymd(2021, 6, 3).unwrap();
    /// println!("{}", date);
    /// ```
    pub fn from_ymd(year: i32, month: u32, day: u32) -> Result<BaseDate, TimeError> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            Err(TimeError::from(BaseErrorKind::InvalidYear))
        } else {
            let flags = SundayLetter::from_year(year);
            let mdf = Mdf::new(month, day, flags);
            match mdf {
                Ok(mdf) => BaseDate::from_mdf(year, mdf),
                Err(err) => Err(err),
            }
        }
    }

    /// 通过年份和当前年份的天数创建 `BaseDate`
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::date::BaseDate;
    ///
    /// let date = BaseDate::from_yo(2021, 1).unwrap();
    /// println!("{}", date);
    /// ```
    pub fn from_yo(year: i32, ordinal: u32) -> Result<BaseDate, TimeError> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            Err(TimeError::from(BaseErrorKind::InvalidYear))
        } else {
            let flags = SundayLetter::from_year(year);
            let of = Of::new(ordinal, flags);
            match of {
                Ok(of) => BaseDate::from_of(year, of),
                Err(err) => Err(err),
            }
        }
    }

    /// 创建当前 `BaseDate` 的下一个日历日期
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::date::BaseDate;
    ///
    /// let date = BaseDate::from_ymd(2021, 2, 28).unwrap();
    /// assert_eq!(date.succ().unwrap(), BaseDate::from_ymd(2021, 3, 1).unwrap());
    /// ```
    pub fn succ(&self) -> Result<BaseDate, TimeError> {
        let basedate = self.with_of(self.of().succ());
        match basedate {
            Ok(basedate) => Ok(basedate),
            _ => match BaseDate::from_ymd(self.year() + 1, 1, 1) {
                Ok(basedate) => Ok(basedate),
                Err(err) => Err(err),
            },
        }
    }

    /// 创建当前 `BaseDate` 的上一个日历日期
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::date::BaseDate;
    ///
    /// let date = BaseDate::from_ymd(2021, 3, 1).unwrap();
    /// assert_eq!(date.pred().unwrap(), BaseDate::from_ymd(2021, 2, 28).unwrap());
    /// ```
    pub fn pred(&self) -> Result<BaseDate, TimeError> {
        let basedate = self.with_of(self.of().pred());
        match basedate {
            Ok(basedate) => Ok(basedate),
            _ => match BaseDate::from_ymd(self.year() - 1, 12, 31) {
                Ok(basedate) => Ok(basedate),
                Err(err) => Err(err),
            },
        }
    }

    /// 通过从公历初始值经过了多少天创建 `BaseDate`
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::date::BaseDate;
    ///
    /// let date = BaseDate::from_num_days_from_ce(-1).unwrap();
    /// assert_eq!(date, BaseDate::from_ymd(0, 12, 30).unwrap());
    /// ```
    pub fn from_num_days_from_ce(days: i32) -> Result<BaseDate, TimeError> {
        // 公元前一年为第零年，公元前一年的 12.31 日将被看作第零天
        let yo = days_to_yo(days);
        match yo {
            Ok((year, ordinal)) => {
                let flags = SundayLetter::from_year(year);
                let of = Of::new(ordinal, flags);
                match of {
                    Ok(of) => BaseDate::from_of(year, of),
                    Err(err) => Err(err),
                }
            }
            Err(err) => Err(err),
        }
    }

    /// 将指定的 `Duration` 天数部分加入到当前的日期
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::date::BaseDate;
    /// use ylong_time::base::duration::Duration;
    ///
    /// let date = BaseDate::from_ymd(2015, 9, 5).unwrap();
    /// assert_eq!(date.checked_add_signed(Duration::days(40).unwrap()).unwrap(),
    ///     BaseDate::from_ymd(2015, 10, 15).unwrap());
    /// ```
    pub fn checked_add_signed(self, rhs: Duration) -> Result<BaseDate, TimeError> {
        let days = yo_to_days(self.year(), self.ordinal());
        match days {
            Ok(mut days) => {
                let delta_days = rhs.num_days();
                days += delta_days as i32;
                let (year, ordinal) = days_to_yo(days)?;
                BaseDate::from_yo(year, ordinal)
            }
            Err(err) => Err(err),
        }
    }

    /// 将当前的日期减去指定的 `Duration` 天数部分
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::date::BaseDate;
    /// use ylong_time::base::duration::Duration;
    ///
    /// let date = BaseDate::from_ymd(2015, 9, 5).unwrap();
    /// assert_eq!(date.checked_sub_signed(Duration::days(40).unwrap()).unwrap(),
    ///     BaseDate::from_ymd(2015, 7, 27).unwrap());
    /// ```
    pub fn checked_sub_signed(self, rhs: Duration) -> Result<BaseDate, TimeError> {
        let days = yo_to_days(self.year(), self.ordinal());
        match days {
            Ok(mut days) => {
                let delta_days = rhs.num_days();
                days -= delta_days as i32;
                let (year, ordinal) = days_to_yo(days)?;
                BaseDate::from_yo(year, ordinal)
            }
            Err(err) => Err(err),
        }
    }

    /// 将当前的日期减去指定的日期，返回 `Duration` 结构
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::date::BaseDate;
    /// use ylong_time::base::duration::Duration;
    ///
    /// let from_ymd = BaseDate::from_ymd;
    /// let since = BaseDate::signed_duration_since;
    ///
    /// assert_eq!(since(from_ymd(2014, 1, 1).unwrap(), from_ymd(2014, 1, 1).unwrap()).unwrap(), Duration::days(0).unwrap());
    /// assert_eq!(since(from_ymd(2014, 1, 1).unwrap(), from_ymd(2013, 12, 31).unwrap()).unwrap(), Duration::days(1).unwrap());
    /// assert_eq!(since(from_ymd(2014, 1, 1).unwrap(), from_ymd(2014, 1, 2).unwrap()).unwrap(), Duration::days(-1).unwrap());
    /// ```
    pub fn signed_duration_since(self, rhs: BaseDate) -> Result<Duration, TimeError> {
        let days1 = self.num_days_from_ce()?;
        let days2 = rhs.num_days_from_ce()?;
        let days = days1 - days2;
        Duration::days(days as i64)
    }

    /// 通过拼装指定的 `BaseTime`，创建新的 `BaseDateTime`
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::date::BaseDate;
    /// use ylong_time::base::time::BaseTime;
    /// use ylong_time::base::datetime::BaseDateTime;
    ///
    /// let date = BaseDate::from_ymd(2021, 1, 1).unwrap();
    /// let time = BaseTime::from_hms(1, 1, 1).unwrap();
    /// let datetime = BaseDateTime::new(date, time);
    ///
    /// assert_eq!(date.and_time(time), datetime);
    /// ```
    pub fn and_time(&self, time: BaseTime) -> BaseDateTime {
        BaseDateTime::new(*self, time)
    }

    /// 已指定的格式格式化合并后的日期与时间
    fn format_with_items<'a, I, B>(&self, items: I) -> DelayedFormat<I>
    where
        I: Iterator<Item = B> + Clone,
        B: Borrow<Item<'a>>,
    {
        // Safety: The BaseTime will always be OK.
        let base_time = BaseTime::from_hms(0, 0, 0).unwrap();
        DelayedFormat::new_with_offset(*self, base_time, &Utc, items)
    }

    /// 按照自己制定的格式格式化日期
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::date::BaseDate;
    ///
    /// let date = BaseDate::from_ymd(2021, 1, 1).unwrap();
    /// let format = date.format("%Y/%-m/%-d").to_string();
    ///
    /// assert_eq!(format, "2021/1/1");
    /// ```
    pub fn format<'a>(&self, fmt: &'a str) -> DelayedFormat<StrftimeItems<'a>> {
        self.format_with_items(StrftimeItems::new(fmt))
    }
}

impl Datelike for BaseDate {
    /// 返回当前日期的年份
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::date::BaseDate;
    /// use ylong_time::Datelike;
    ///
    /// let date = BaseDate::from_ymd(2021, 1, 1).unwrap();
    /// assert_eq!(date.year(), 2021);
    /// ```
    fn year(&self) -> i32 {
        self.ymdf >> 13
    }

    /// 返回当前日期的月份（从一月份开始）
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::date::BaseDate;
    /// use ylong_time::Datelike;
    ///
    /// let date = BaseDate::from_ymd(2021, 1, 1).unwrap();
    /// assert_eq!(date.month(), 1);
    /// ```
    fn month(&self) -> u32 {
        self.mdf().month()
    }

    /// 返回当前日期的月份（从零月份开始）
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::date::BaseDate;
    /// use ylong_time::Datelike;
    ///
    /// let date = BaseDate::from_ymd(2021, 1, 1).unwrap();
    /// assert_eq!(date.month0(), 0);
    /// ```
    fn month0(&self) -> u32 {
        self.mdf().month() - 1
    }

    /// 返回当前日期的天数（每月的第几天，从第一天开始计算）
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::date::BaseDate;
    /// use ylong_time::Datelike;
    ///
    /// let date = BaseDate::from_ymd(2021, 1, 1).unwrap();
    /// assert_eq!(date.day(), 1);
    /// ```
    fn day(&self) -> u32 {
        self.mdf().day()
    }

    /// 返回当前日期的天数（每月的第几天，从第零天开始计算）
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::date::BaseDate;
    /// use ylong_time::Datelike;
    ///
    /// let date = BaseDate::from_ymd(2021, 1, 1).unwrap();
    /// assert_eq!(date.day0(), 0);
    /// ```
    fn day0(&self) -> u32 {
        self.mdf().day() - 1
    }

    /// 返回当前日期的天数（每年的第几天，从第一天开始计算）
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::date::BaseDate;
    /// use ylong_time::Datelike;
    ///
    /// let date = BaseDate::from_ymd(2021, 12, 31).unwrap();
    /// assert_eq!(date.ordinal(), 365);
    /// ```
    fn ordinal(&self) -> u32 {
        self.of().ordinal()
    }

    /// 返回当前日期的天数（每年的第几天，从第零天开始计算）
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::date::BaseDate;
    /// use ylong_time::Datelike;
    ///
    /// let date = BaseDate::from_ymd(2021, 12, 31).unwrap();
    /// assert_eq!(date.ordinal0(), 364);
    /// ```
    fn ordinal0(&self) -> u32 {
        self.of().ordinal() - 1
    }

    /// Return the day of the week of the current date.
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::{BaseDate, Datelike, Weekday};
    ///
    /// let base_date = BaseDate::from_ymd(2022, 12, 15).unwrap();
    /// assert_eq!(base_date.weekday(), Weekday::Thursday);
    /// ```
    fn weekday(&self) -> Weekday {
        self.of().weekday()
    }

    /// 将一个 `BaseDate` 替换年份，并且可以判断替换年份后的日期是否有效
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::date::BaseDate;
    /// use ylong_time::Datelike;
    ///
    /// let date = BaseDate::from_ymd(2020, 2, 29).unwrap();
    /// assert_eq!(date.with_year(2016).unwrap(), BaseDate::from_ymd(2016, 2, 29).unwrap());
    /// assert_eq!(date.with_year(2021).is_err(), true);
    /// ```
    fn with_year(&self, year: i32) -> Result<Self, TimeError> {
        let mdf = self.mdf();
        let flags = SundayLetter::from_year(year);
        let mdf = mdf.with_flags(flags);

        BaseDate::from_mdf(year, mdf)
    }

    /// 将一个 `BaseDate` 替换月份（从一月份开始计算），并且可以判断替换月份后的日期是否有效
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::date::BaseDate;
    /// use ylong_time::Datelike;
    ///
    /// let date = BaseDate::from_ymd(2020, 1, 31).unwrap();
    /// assert_eq!(date.with_month(2).is_err(), true);
    /// assert_eq!(date.with_month(3).unwrap(), BaseDate::from_ymd(2020, 3, 31).unwrap());
    /// ```
    fn with_month(&self, month: u32) -> Result<Self, TimeError> {
        let mdf = self.mdf().with_month(month);
        match mdf {
            Ok(mdf) => self.with_mdf(mdf),
            Err(err) => Err(err),
        }
    }

    /// 将一个 `BaseDate` 替换月份（从零月份开始计算），并且可以判断替换月份后的日期是否有效
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::date::BaseDate;
    /// use ylong_time::Datelike;
    ///
    /// let date = BaseDate::from_ymd(2020, 1, 31).unwrap();
    /// assert_eq!(date.with_month0(1).is_err(), true);
    /// assert_eq!(date.with_month0(2).unwrap(), BaseDate::from_ymd(2020, 3, 31).unwrap());
    /// ```
    fn with_month0(&self, month0: u32) -> Result<Self, TimeError> {
        let mdf = self.mdf().with_month(month0 + 1);
        match mdf {
            Ok(mdf) => self.with_mdf(mdf),
            Err(err) => Err(err),
        }
    }

    /// 将一个 `BaseDate` 替换天数（每月第几天，从第一天开始计算），并且可以判断替换后的日期是否有效
    ///
    /// # Examplse
    ///
    /// ```
    /// use ylong_time::base::date::BaseDate;
    /// use ylong_time::Datelike;
    ///
    /// let date = BaseDate::from_ymd(2020, 2, 29).unwrap();
    /// assert_eq!(date.with_day(31).is_err(), true);
    /// assert_eq!(date.with_day(1).unwrap(), BaseDate::from_ymd(2020, 2, 1).unwrap());
    /// ```
    fn with_day(&self, day: u32) -> Result<Self, TimeError> {
        let mdf = self.mdf().with_day(day);
        match mdf {
            Ok(mdf) => self.with_mdf(mdf),
            Err(err) => Err(err),
        }
    }

    /// 将一个 `BaseDate` 替换天数（每月第几天，从第零天开始计算），并且可以判断替换后的日期是否有效
    ///
    /// # Examplse
    ///
    /// ```
    /// use ylong_time::base::date::BaseDate;
    /// use ylong_time::Datelike;
    ///
    /// let date = BaseDate::from_ymd(2020, 2, 29).unwrap();
    /// assert_eq!(date.with_day0(30).is_err(), true);
    /// assert_eq!(date.with_day0(0).unwrap(), BaseDate::from_ymd(2020, 2, 1).unwrap());
    /// ```
    fn with_day0(&self, day0: u32) -> Result<Self, TimeError> {
        let mdf = self.mdf().with_day(day0 + 1);
        match mdf {
            Ok(mdf) => self.with_mdf(mdf),
            Err(err) => Err(err),
        }
    }

    /// 将一个 `BaseDate` 替换天数（每年第几天，从第一天开始计算），并且可以判断替换后的日期是否有效
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::date::BaseDate;
    /// use ylong_time::Datelike;
    ///
    /// let date = BaseDate::from_yo(2020, 1).unwrap();
    /// assert_eq!(date.with_ordinal(2).unwrap(), BaseDate::from_yo(2020, 2).unwrap());
    /// ```
    fn with_ordinal(&self, ordinal: u32) -> Result<Self, TimeError> {
        let mdf = self.of().with_ordinal(ordinal);
        match mdf {
            Ok(mdf) => self.with_mdf(mdf.as_mdf()),
            Err(err) => Err(err),
        }
    }

    /// 将一个 `BaseDate` 替换天数（每年第几天，从第零天开始计算），并且可以判断替换后的日期是否有效
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::date::BaseDate;
    /// use ylong_time::Datelike;
    ///
    /// let date = BaseDate::from_yo(2020, 1).unwrap();
    /// assert_eq!(date.with_ordinal0(1).unwrap(), BaseDate::from_yo(2020, 2).unwrap());
    /// ```
    fn with_ordinal0(&self, ordinal0: u32) -> Result<Self, TimeError> {
        let mdf = self.of().with_ordinal(ordinal0 + 1);
        match mdf {
            Ok(mdf) => self.with_mdf(mdf.as_mdf()),
            Err(err) => Err(err),
        }
    }
}

/// 使得当前的 `BaseDate` 结构拥有增加 `Duration` 中整数天数的功能
impl Add<Duration> for BaseDate {
    type Output = BaseDate;

    fn add(self, rhs: Duration) -> Self::Output {
        self.checked_add_signed(rhs)
            .expect("`BaseDate + Duration` overflowed")
    }
}

impl AddAssign<Duration> for BaseDate {
    fn add_assign(&mut self, rhs: Duration) {
        *self = self.add(rhs);
    }
}

/// 使得当前的 `BaseDate` 结构拥有减少 `Duration` 中整数天数的功能
impl Sub<Duration> for BaseDate {
    type Output = BaseDate;

    fn sub(self, rhs: Duration) -> Self::Output {
        self.checked_sub_signed(rhs)
            .expect("`BaseDate - Duration` overflowed")
    }
}

impl SubAssign<Duration> for BaseDate {
    fn sub_assign(&mut self, rhs: Duration) {
        *self = self.sub(rhs)
    }
}

/// 将当前 `BaseDate` 日期减去指定 `BaseDate`，并返回 `Duration`
impl Sub<BaseDate> for BaseDate {
    type Output = Duration;

    fn sub(self, rhs: BaseDate) -> Self::Output {
        self.signed_duration_since(rhs)
            .expect("BaseDate - BaseDate overflowed")
    }
}

impl Debug for BaseDate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let year = self.year();
        let mdf = self.mdf();
        if (0..=9999).contains(&year) {
            write!(f, "{:04}-{:02}-{:02}", year, mdf.month(), mdf.day())
        } else {
            write!(f, "{:+05}-{:02}-{:02}", year, mdf.month(), mdf.day())
        }
    }
}

impl Display for BaseDate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

#[cfg(test)]
mod test {
    include!("../../tests/ut/base/ut_date.rs");
}
