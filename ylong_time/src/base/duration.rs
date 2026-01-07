use crate::error::{BaseErrorKind, TimeError};
use std::ops::{Add, Div, Mul, Neg, Sub};

/// 微秒中的纳秒数
const NANOS_OF_MICRO: i32 = 1000;
/// 毫秒中的纳秒数
const NANOS_OF_MILLI: i32 = 1_000_000;
/// 秒中的纳秒数
const NANOS_OF_SEC: i32 = 1_000_000_000;
/// 秒中的微秒数
const MICROS_OF_SEC: i64 = 1_000_000;
/// 秒中的毫秒数
const MILLIS_OF_SEC: i64 = 1000;
/// 分钟中的秒数
const SECS_OF_MINUTE: i64 = 60;
/// 小时中的秒数
const SECS_OF_HOUR: i64 = 3600;
/// 一天中的秒数
const SECS_OF_DAY: i64 = 86400;
/// 一周中的秒数
const SECS_OF_WEEK: i64 = 604800;

/// 有着纳秒精准度的时间段结构
#[derive(PartialOrd, PartialEq, Eq, Debug, Copy, Clone)]
pub struct Duration {
    secs: i64,
    nanos: i32,
}

/// 最小的可能时间段
pub const MIN: Duration = Duration {
    // MIN_DATE.num_days_from_ce().unwrap() as i64 * SECS_PER_DAY as i64 + 0
    secs: -8272497168000,
    nanos: 0,
};

/// 最大的可能时间段
pub const MAX: Duration = Duration {
    // MAX_DATE.num_days_from_ce().unwrap() as i64 * SECS_PER_DAY as i64 + 86399
    secs: 8272434095999,
    nanos: NANOS_OF_SEC - 1,
};

impl Duration {
    /// 判断当前的 `Duration` 是否满足 `ISO` 格式的时间段范围
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::duration::Duration;
    ///
    /// let duration = Duration::seconds(10).unwrap();
    /// assert_eq!(duration.valid(), true);
    /// ```
    pub fn valid(&self) -> bool {
        let d = Duration {
            secs: self.secs,
            nanos: self.nanos,
        };
        !(d < MIN || d > MAX)
    }

    /// 通过给定的周数创建新的 `Duration` 结构
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::duration::Duration;
    ///
    /// let duration = Duration::weeks(1).unwrap();
    /// println!("{:?}", duration);
    /// ```
    pub fn weeks(weeks: i64) -> Result<Duration, TimeError> {
        let secs = weeks.mul(SECS_OF_WEEK);
        Duration::seconds(secs)
    }

    /// 通过指定的天数创建新的 `Duration` 结构
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::duration::Duration;
    ///
    /// let duration = Duration::days(1).unwrap();
    /// println!("{:?}", duration);
    /// ```
    pub fn days(days: i64) -> Result<Duration, TimeError> {
        let secs = days.mul(SECS_OF_DAY);
        Duration::seconds(secs)
    }

    /// 通过指定的小时数创建新的 `Duration` 结构
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::duration::Duration;
    ///
    /// let duration = Duration::hours(1).unwrap();
    /// println!("{:?}", duration);
    /// ```
    pub fn hours(hours: i64) -> Result<Duration, TimeError> {
        let secs = hours.mul(SECS_OF_HOUR);
        Duration::seconds(secs)
    }

    /// 通过指定的分钟数创建新的 `Duration` 结构
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::duration::Duration;
    ///
    /// let duration = Duration::minutes(1).unwrap();
    /// println!("{:?}", duration);
    /// ```
    pub fn minutes(minutes: i64) -> Result<Duration, TimeError> {
        let secs = minutes.mul(SECS_OF_MINUTE);
        Duration::seconds(secs)
    }

    /// 通过给定的秒数创建新的 `Duration` 结构
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::duration::Duration;
    ///
    /// let duration = Duration::seconds(1).unwrap();
    /// println!("{:?}", duration);
    /// ```
    pub fn seconds(seconds: i64) -> Result<Duration, TimeError> {
        let d = Duration {
            secs: seconds,
            nanos: 0,
        };
        if d < MIN || d > MAX {
            Err(TimeError::from(BaseErrorKind::InvalidDuration))
        } else {
            Ok(d)
        }
    }

    /// 通过给定的毫秒数创建新的 `Duration` 结构
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::duration::Duration;
    ///
    /// let duration = Duration::milliseconds(1).unwrap();
    /// println!("{:?}", duration);
    /// ```
    pub fn milliseconds(milliseconds: i64) -> Result<Duration, TimeError> {
        let secs = milliseconds / MILLIS_OF_SEC;
        let nanos = (milliseconds % MILLIS_OF_SEC) as i32 * NANOS_OF_MILLI;
        let d = Duration { secs, nanos };
        if d < MIN || d > MAX {
            Err(TimeError::from(BaseErrorKind::InvalidDuration))
        } else {
            Ok(d)
        }
    }

    /// 通过给定的微秒数创建新的 `Duration` 结构
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::duration::Duration;
    ///
    /// let duration = Duration::microseconds(1).unwrap();
    /// println!("{:?}", duration);
    /// ```
    pub fn microseconds(microseconds: i64) -> Result<Duration, TimeError> {
        let secs = microseconds / MICROS_OF_SEC;
        let nanos = (microseconds % MICROS_OF_SEC) as i32 * NANOS_OF_MICRO;
        let d = Duration { secs, nanos };
        if d < MIN || d > MAX {
            Err(TimeError::from(BaseErrorKind::InvalidDuration))
        } else {
            Ok(d)
        }
    }

    /// 通过给定的纳秒数创建新的 `Duration` 结构
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::duration::Duration;
    ///
    /// let duration = Duration::nanoseconds(1).unwrap();
    /// println!("{:?}", duration);
    /// ```
    pub fn nanoseconds(nanoseconds: i64) -> Result<Duration, TimeError> {
        let secs = nanoseconds / NANOS_OF_SEC as i64;
        let nanos = (nanoseconds % NANOS_OF_SEC as i64) as i32;
        let d = Duration { secs, nanos };
        if d < MIN || d > MAX {
            Err(TimeError::from(BaseErrorKind::InvalidDuration))
        } else {
            Ok(d)
        }
    }

    /// 返回在 `Duration` 之中拥有的周数
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::duration::Duration;
    ///
    /// let duration = Duration::days(14).unwrap();
    /// assert_eq!(duration.num_weeks(), 2);
    /// ```
    pub fn num_weeks(&self) -> i64 {
        self.num_days() / 7
    }

    /// 返回在 `Duration` 之中拥有的天数
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::duration::Duration;
    ///
    /// let duration = Duration::days(1).unwrap();
    /// assert_eq!(duration.num_days(), 1);
    /// ```
    pub fn num_days(&self) -> i64 {
        self.num_seconds() / SECS_OF_DAY
    }

    /// 返回在 `Duration` 之中拥有的小时数
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::duration::Duration;
    ///
    /// let duration = Duration::days(1).unwrap();
    /// assert_eq!(duration.num_days(), 1);
    /// ```
    pub fn num_hours(&self) -> i64 {
        self.num_seconds() / SECS_OF_HOUR
    }

    /// 返回在 `Duration` 之中拥有的分钟数
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::duration::Duration;
    ///
    /// let duration = Duration::minutes(1).unwrap();
    /// assert_eq!(duration.num_minutes(), 1);
    /// ```
    pub fn num_minutes(&self) -> i64 {
        self.num_seconds() / SECS_OF_MINUTE
    }

    /// 返回在 `Duration` 之中拥有的秒数
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::duration::Duration;
    ///
    /// let duration = Duration::minutes(1).unwrap();
    /// assert_eq!(duration.num_seconds(), 60);
    /// ```
    pub fn num_seconds(&self) -> i64 {
        // 当秒数处于负数的情况下，我们挪用了纳秒的数据，因此之后相同条件下纳秒需要减去 `1_000_000_000`
        if self.secs < 0 && self.nanos > 0 {
            self.secs + 1
        } else {
            self.secs
        }
    }

    fn nanos_mod_sec(&self) -> i32 {
        if self.secs < 0 && self.nanos > 0 {
            self.nanos - NANOS_OF_SEC
        } else {
            self.nanos
        }
    }

    /// 返回在 `Duration` 之中拥有的毫秒数
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::duration::Duration;
    ///
    /// let duration = Duration::minutes(1).unwrap();
    /// assert_eq!(duration.num_minutes(), 1);
    /// ```
    pub fn num_milliseconds(&self) -> i64 {
        let secs_part = self.num_seconds() * MILLIS_OF_SEC;
        let nanos_part = (self.nanos_mod_sec() / NANOS_OF_MILLI) as i64;
        secs_part + nanos_part
    }

    /// 返回在 `Duration` 之中拥有的微秒数
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::duration::Duration;
    ///
    /// let duration = Duration::microseconds(1).unwrap();
    /// assert_eq!(duration.num_microseconds(), 1);
    /// ```
    pub fn num_microseconds(&self) -> i64 {
        let secs_part = self.num_seconds() * MICROS_OF_SEC;
        let nano_part = (self.nanos_mod_sec() / NANOS_OF_MICRO) as i64;
        secs_part + nano_part
    }

    /// 返回在 `Duration` 之中拥有的纳秒数
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::duration::Duration;
    ///
    /// let duration = Duration::nanoseconds(1).unwrap();
    /// assert_eq!(duration.num_nanoseconds(), 1);
    /// ```
    pub fn num_nanoseconds(&self) -> i64 {
        let secs_part = self.num_seconds() * NANOS_OF_SEC as i64;
        let nano_part = self.nanos_mod_sec() as i64;
        secs_part + nano_part
    }

    /// 返回当前 `Duration` 的最小有效时间段
    pub fn min_duration() -> Duration {
        MIN
    }

    /// 返回当前 `Duration` 的最大有效时间段
    pub fn max_duration() -> Duration {
        MAX
    }
}

impl Neg for Duration {
    type Output = Duration;

    /// 将 `Duration` 转为负数
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::duration::Duration;
    ///
    /// let duration = Duration::seconds(1).unwrap();
    /// let neg_duration = Duration::seconds(-1).unwrap();
    /// assert_eq!(-duration, neg_duration);
    /// ```
    fn neg(self) -> Self::Output {
        if self.nanos == 0 {
            Duration {
                secs: -self.secs,
                nanos: 0,
            }
        } else {
            Duration {
                secs: -self.secs - 1,
                nanos: NANOS_OF_SEC - self.nanos,
            }
        }
    }
}

impl Add for Duration {
    type Output = Duration;

    /// `Duration` 加法运算
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::duration::Duration;
    ///
    /// let duration = Duration::seconds(1).unwrap();
    /// let add_duration = Duration::seconds(2).unwrap();
    /// assert_eq!(duration + duration, add_duration);
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        let mut secs = self.secs + rhs.secs;
        let mut nanos = self.nanos + rhs.nanos;
        if nanos >= NANOS_OF_SEC {
            nanos -= NANOS_OF_SEC;
            secs += 1
        }
        Duration { secs, nanos }
    }
}

impl Sub for Duration {
    type Output = Duration;

    /// `Duration` 减法运算
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::duration::Duration;
    ///
    /// let duration = Duration::seconds(1).unwrap();
    /// let sub_duration = Duration::seconds(0).unwrap();
    /// assert_eq!(duration - duration, sub_duration);
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        let mut secs = self.secs - rhs.secs;
        let mut nanos = self.nanos - rhs.nanos;
        if nanos < 0 {
            nanos += NANOS_OF_SEC;
            secs -= 1;
        }
        Duration { secs, nanos }
    }
}

impl Mul<i32> for Duration {
    type Output = Duration;

    /// `Duration` 乘法运算
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::duration::Duration;
    ///
    /// let duration = Duration::seconds(1).unwrap();
    /// let mul_duration = Duration::seconds(10).unwrap();
    /// assert_eq!(duration * 10, mul_duration);
    /// ```
    fn mul(self, rhs: i32) -> Self::Output {
        let total_nanos = self.nanos * rhs;
        let extra_secs = total_nanos / NANOS_OF_SEC;
        let nanos = total_nanos % NANOS_OF_SEC;
        let secs = self.secs * rhs as i64 + extra_secs as i64;
        Duration { secs, nanos }
    }
}

impl Div<i32> for Duration {
    type Output = Duration;

    /// `Duration` 除法运算
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::duration::Duration;
    ///
    /// let duration = Duration::seconds(3).unwrap();
    /// let div_duration = Duration::seconds(1).unwrap();
    /// assert_eq!(duration / 3, div_duration);
    /// ```
    fn div(self, rhs: i32) -> Self::Output {
        let mut secs = self.secs / rhs as i64;
        // 直接进行除法运算后剩余无法被整除的秒数
        let carry = self.secs % rhs as i64;
        // 将无法被整除的秒数作为纳秒部分
        let extra_nanos = carry * NANOS_OF_SEC as i64 / rhs as i64;
        let mut nanos = self.nanos / rhs + extra_nanos as i32;
        // 如果此时的纳秒部分可以进位为秒数部分
        if nanos >= NANOS_OF_SEC {
            nanos -= NANOS_OF_SEC;
            secs += 1;
        }
        // 如果此时的纳秒部分为负数
        if nanos < 0 {
            nanos += NANOS_OF_SEC;
            secs -= 1;
        }
        Duration { secs, nanos }
    }
}

#[cfg(test)]
mod test {
    include!("../../tests/ut/base/ut_duration.rs");
}
