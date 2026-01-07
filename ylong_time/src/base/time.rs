use crate::base::duration::Duration;
use crate::error::{BaseErrorKind, TimeError};
use crate::Timelike;
use core::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

/// 可能的最小时间
pub const MIN_TIME: BaseTime = BaseTime { secs: 0, frac: 0 };

/// 可能的最大时间
pub const MAX_TIME: BaseTime = BaseTime {
    secs: 23 * 3600 + 59 * 60 + 59,
    frac: 1_999_999_999,
};

/// 基础时间结构，承认闰秒的存在，但不主动使用
#[derive(Copy, Clone, PartialOrd, PartialEq, Eq)]
pub struct BaseTime {
    secs: u32,
    frac: u32,
}

impl BaseTime {
    /// 通过指定的时分秒创建 `BaseTime` 结构
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::time::BaseTime;
    ///
    /// let time = BaseTime::from_hms(1, 1, 1).unwrap();
    /// println!("{}", time);
    /// let time = BaseTime::from_hms(25, 1, 1);
    /// assert_eq!(time.is_err(), true);
    /// ```
    pub fn from_hms(hour: u32, min: u32, sec: u32) -> Result<BaseTime, TimeError> {
        BaseTime::from_hms_nano(hour, min, sec, 0)
    }

    /// 通过指定的时分秒、毫秒创建 `BaseTime` 结构
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::time::BaseTime;
    ///
    /// let time = BaseTime::from_hms_milli(1, 1, 1, 1).unwrap();
    /// println!("{}", time);
    /// let time = BaseTime::from_hms_milli(1, 1, 1, 2_000);
    /// assert_eq!(time.is_err(), true);
    /// ```
    pub fn from_hms_milli(
        hour: u32,
        min: u32,
        sec: u32,
        milli: u32,
    ) -> Result<BaseTime, TimeError> {
        BaseTime::from_hms_nano(hour, min, sec, milli.mul(1_000_000))
    }

    /// 通过指定的时分秒、微秒创建 `BaseTime` 结构
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::time::BaseTime;
    ///
    /// let time = BaseTime::from_hms_micro(1, 1, 1, 1).unwrap();
    /// println!("{}", time);
    /// let time = BaseTime::from_hms_micro(1, 1, 1, 2_000_000);
    /// assert_eq!(time.is_err(), true);
    /// ```
    pub fn from_hms_micro(
        hour: u32,
        min: u32,
        sec: u32,
        micro: u32,
    ) -> Result<BaseTime, TimeError> {
        BaseTime::from_hms_nano(hour, min, sec, micro.mul(1_000))
    }

    /// 通过指定的时分秒、纳秒创建 `BaseTime` 结构，其中的纳秒是用来表示闰秒的
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::time::BaseTime;
    ///
    /// let time = BaseTime::from_hms_nano(1, 1, 1, 1_000_000_000).unwrap();
    /// println!("{}", time);
    /// let time = BaseTime::from_hms_nano(1, 1, 1, 2_000_000_000);
    /// assert_eq!(time.is_err(), true);
    /// ```
    pub fn from_hms_nano(hour: u32, min: u32, sec: u32, nano: u32) -> Result<BaseTime, TimeError> {
        // 即使存在闰秒的情况下，纳秒也不应该超过 `2_000_000_000`
        if hour >= 24 {
            Err(TimeError::from(BaseErrorKind::InvalidHour))
        } else if min >= 60 {
            Err(TimeError::from(BaseErrorKind::InvalidMin))
        } else if sec >= 60 {
            Err(TimeError::from(BaseErrorKind::InvalidSec))
        } else if nano >= 2_000_000_000 {
            Err(TimeError::from(BaseErrorKind::InvalidNano))
        } else {
            let secs = hour * 3600 + min * 60 + sec;
            Ok(BaseTime { secs, frac: nano })
        }
    }

    /// 将当前的时分秒作为元组数据返回
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::time::BaseTime;
    ///
    /// let time = BaseTime::from_hms(1, 1, 1).unwrap();
    /// assert_eq!(time.hms(), (1, 1, 1));
    /// ```
    pub fn hms(&self) -> (u32, u32, u32) {
        let temp_min = self.secs / 60;
        let sec = self.secs % 60;
        let min = temp_min % 60;
        let hour = temp_min / 60;
        (hour, min, sec)
    }

    /// 创建一个根据从午夜开始计算的秒以及纳秒来计算 `BaseTime` 的结构，纳秒用于表示闰秒操作
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::time::BaseTime;
    ///
    /// let secs_from_midnight = BaseTime::from_num_seconds_from_midnight;
    ///
    /// assert_eq!(secs_from_midnight(0, 0).is_ok(), true);
    /// assert_eq!(secs_from_midnight(86400, 0).is_err(), true);
    /// assert_eq!(secs_from_midnight(86399, 1_999_999_999).is_ok(), true);
    /// ```
    pub fn from_num_seconds_from_midnight(secs: u32, nanos: u32) -> Result<BaseTime, TimeError> {
        if secs >= 86400 {
            Err(TimeError::from(BaseErrorKind::InvalidSec))
        } else if nanos >= 2_000_000_000 {
            Err(TimeError::from(BaseErrorKind::InvalidNano))
        } else {
            Ok(BaseTime { secs, frac: nanos })
        }
    }

    /// 将当前 `BaseTime` 增加 `Duration` 的时间，
    /// 由于 `BaseTime` 无法表现出天数的变化，一并返回天数变化的秒数
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::time::BaseTime;
    /// use ylong_time::base::duration::Duration;
    ///
    /// let time = BaseTime::from_hms(3, 4, 5).unwrap();
    /// assert_eq!(time.overflowing_add_signed(Duration::hours(11).unwrap()), (BaseTime::from_hms(14, 4, 5).unwrap(), 0));
    /// assert_eq!(time.overflowing_add_signed(Duration::hours(-7).unwrap()), (BaseTime::from_hms(20, 4, 5).unwrap(), -86400));
    /// ```
    pub fn overflowing_add_signed(&self, mut rhs: Duration) -> (BaseTime, i64) {
        let mut secs = self.secs;
        let mut frac = self.frac;

        // 判断是否存在闰秒的情况，一般情况下闰秒部分将被忽略
        if frac >= 1_000_000_000 {
            let rfrac = 2_000_000_000 - frac;
            if rhs >= Duration::nanoseconds(i64::from(rfrac)).unwrap() {
                rhs = rhs - Duration::nanoseconds(i64::from(rfrac)).unwrap();
                secs += 1;
                frac = 0;
            } else if rhs < Duration::nanoseconds(-i64::from(frac)).unwrap() {
                rhs = rhs + Duration::nanoseconds(i64::from(frac)).unwrap();
                frac = 0;
            } else {
                frac = (i64::from(frac) + rhs.num_nanoseconds()) as u32;
                return (BaseTime { secs, frac }, 0);
            }
        }

        let rhssecs = rhs.num_seconds();
        let rhsfrac = (rhs - Duration::seconds(rhssecs).unwrap()).num_nanoseconds();

        let rhssecsinday = rhssecs % 86_400;
        let mut morerhssecs = rhssecs - rhssecsinday;
        let rhssecs = rhssecsinday as i32;
        let rhsfrac = rhsfrac as i32;

        let mut secs = secs as i32 + rhssecs;
        let mut frac = frac as i32 + rhsfrac;

        if frac < 0 {
            // 纳秒部分如果小于零的情况下，需要将其补全，秒数减少
            frac += 1_000_000_000;
            secs -= 1;
        } else if frac >= 1_000_000_000 {
            // 纳秒部分如果大于1_000_000_000的情况下，需要将其进位，秒数增加
            frac -= 1_000_000_000;
            secs += 1;
        }

        if secs < 0 {
            // 秒数部分如果为负数，那么从天数经过秒数处减少一天的秒数进行补全
            secs += 86_400;
            morerhssecs -= 86_400;
        } else if secs >= 86_400 {
            // 秒数部分如果大于 86_400，那么将其减少一天的秒数，将其补全至天数所经过秒数
            secs -= 86_400;
            morerhssecs += 86_400;
        }

        (
            BaseTime {
                secs: secs as u32,
                frac: frac as u32,
            },
            morerhssecs,
        )
    }

    /// 将当前 `BaseTime` 减少 `Duration` 的时间，并且返回天数变化的秒数
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::time::BaseTime;
    /// use ylong_time::base::duration::Duration;
    ///
    /// let time = BaseTime::from_hms(3, 4, 5).unwrap();
    /// assert_eq!(time.overflowing_sub_signed(Duration::hours(2).unwrap()), (BaseTime::from_hms(1, 4, 5).unwrap(), 0));
    /// assert_eq!(time.overflowing_sub_signed(Duration::hours(17).unwrap()), (BaseTime::from_hms(10, 4, 5).unwrap(), 86400));
    /// ```
    pub fn overflowing_sub_signed(&self, rhs: Duration) -> (BaseTime, i64) {
        let (time, days) = self.overflowing_add_signed(-rhs);
        (time, -days)
    }

    /// 将当前 `BaseTime` 减少 `BaseTime` 的时间，并且返回变化的秒数
    /// 如果包含闰秒，闰秒情况将会被考虑在其中
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::time::BaseTime;
    /// use ylong_time::base::duration::Duration;
    ///
    /// let basetime_one = BaseTime::from_hms(3, 4, 5).unwrap();
    /// let basetime_two = BaseTime::from_hms(3, 4, 4).unwrap();
    ///
    /// assert_eq!(basetime_one - basetime_two, Duration::seconds(1).unwrap());
    /// ```
    pub fn signed_duration_since(self, rhs: BaseTime) -> Duration {
        // 两者秒数、纳秒数相减
        let secs = i64::from(self.secs) - i64::from(rhs.secs);
        let frac = i64::from(self.frac) - i64::from(rhs.frac);

        // 存在闰秒的情况时，闰秒将会被考虑在内，并作为变化量体现在秒中
        let delta = match self.secs.cmp(&rhs.secs) {
            Ordering::Equal => 0,
            Ordering::Greater => i64::from(self.frac >= 1_000_000_000),
            Ordering::Less => {
                if rhs.frac >= 1_000_000_000 {
                    -1
                } else {
                    0
                }
            }
        };

        Duration::seconds(secs + delta).unwrap() + Duration::nanoseconds(frac).unwrap()
    }
}

impl Timelike for BaseTime {
    /// 返回当前 `BaseTime` 代表的小时数
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::time::BaseTime;
    /// use ylong_time::Timelike;
    ///
    /// let basetime = BaseTime::from_hms(3, 4, 5).unwrap();
    /// assert_eq!(basetime.hour(), 3);
    /// ```
    fn hour(&self) -> u32 {
        self.hms().0
    }

    /// 返回当前 `BaseTime` 代表的分钟数
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::time::BaseTime;
    /// use ylong_time::Timelike;
    ///
    /// let basetime = BaseTime::from_hms(3, 4, 5).unwrap();
    /// assert_eq!(basetime.minute(), 4);
    /// ```
    fn minute(&self) -> u32 {
        self.hms().1
    }

    /// 返回当前 `BaseTime` 代表的秒数，但是就算存在闰秒也不会返回六十秒的情况
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::time::BaseTime;
    /// use ylong_time::Timelike;
    ///
    /// let basetime = BaseTime::from_hms(3, 4, 5).unwrap();
    /// assert_eq!(basetime.second(), 5);
    /// ```
    fn second(&self) -> u32 {
        self.hms().2
    }

    /// 返回当前 `BaseTime` 代表的纳秒数
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::time::BaseTime;
    /// use ylong_time::Timelike;
    ///
    /// let basetime = BaseTime::from_hms_nano(3, 4, 5, 1_000_000_000).unwrap();
    /// assert_eq!(basetime.nanosecond(), 1_000_000_000);
    /// ```
    fn nanosecond(&self) -> u32 {
        self.frac
    }

    /// 将当前 `BaseTime` 代表的小时替换为新的小时数
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::time::BaseTime;
    /// use ylong_time::Timelike;
    ///
    /// let basetime = BaseTime::from_hms(3, 4, 5).unwrap();
    /// assert_eq!(basetime.with_hour(4).unwrap(), BaseTime::from_hms(4, 4, 5).unwrap());
    /// ```
    fn with_hour(&self, hour: u32) -> Result<Self, TimeError> {
        if hour >= 24 {
            Err(TimeError::from(BaseErrorKind::InvalidHour))
        } else {
            // 将当前 `BaseTime` 结构中的 `hour` 通过取余去除
            let secs = hour * 3600 + self.secs % 3600;
            let frac = self.frac;
            Ok(BaseTime { secs, frac })
        }
    }

    /// 将当前 `BaseTime` 代表的分钟替换为新的分钟数
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::time::BaseTime;
    /// use ylong_time::Timelike;
    ///
    /// let basetime = BaseTime::from_hms(3, 4, 5).unwrap();
    /// assert_eq!(basetime.with_hour(4).unwrap(), BaseTime::from_hms(4, 4, 5).unwrap());
    /// ```
    fn with_minute(&self, minute: u32) -> Result<Self, TimeError> {
        if minute >= 60 {
            Err(TimeError::from(BaseErrorKind::InvalidMin))
        } else {
            // 将当前 `BaseTime` 结构中的 `min` 通过取余去除，并保留 `hour`、`second`等时间
            let secs = minute * 60 + self.secs / 3600 * 3600 + self.secs % 60;
            let frac = self.frac;
            Ok(BaseTime { secs, frac })
        }
    }

    /// 将当前 `BaseTime` 所代表的秒数替换为新的秒数，不包含闰秒的情况，因此传入的秒数范围也在 0 ~ 59
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::time::BaseTime;
    /// use ylong_time::Timelike;
    ///
    /// let basetime = BaseTime::from_hms(3, 4, 5).unwrap();
    /// assert_eq!(basetime.with_second(4).unwrap(), BaseTime::from_hms(3, 4, 4).unwrap());
    /// ```
    fn with_second(&self, second: u32) -> Result<Self, TimeError> {
        if second >= 60 {
            Err(TimeError::from(BaseErrorKind::InvalidSec))
        } else {
            // 将当前 `BaseTime` 结构中除了秒数的其它数据保留
            let secs = self.secs / 60 * 60 + second;
            let frac = self.frac;
            Ok(BaseTime { secs, frac })
        }
    }

    /// 将当前 `BaseTime` 所代表的纳秒替换为新的纳秒数
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::time::BaseTime;
    /// use ylong_time::Timelike;
    ///
    /// let basetime = BaseTime::from_hms_nano(3, 4, 5, 1_000_000_000).unwrap();
    /// assert_eq!(basetime.with_nanosecond(1_999_999_999).unwrap(), BaseTime::from_hms_nano(3, 4, 5, 1_999_999_999).unwrap());
    /// ```
    fn with_nanosecond(&self, nanosecond: u32) -> Result<Self, TimeError> {
        if nanosecond >= 2_000_000_000 {
            Err(TimeError::from(BaseErrorKind::InvalidNano))
        } else {
            let secs = self.secs;
            Ok(BaseTime {
                secs,
                frac: nanosecond,
            })
        }
    }
}

impl Add<Duration> for BaseTime {
    type Output = BaseTime;

    fn add(self, rhs: Duration) -> Self::Output {
        self.overflowing_add_signed(rhs).0
    }
}

impl AddAssign<Duration> for BaseTime {
    fn add_assign(&mut self, rhs: Duration) {
        *self = self.add(rhs)
    }
}

impl Sub<Duration> for BaseTime {
    type Output = BaseTime;

    fn sub(self, rhs: Duration) -> Self::Output {
        self.overflowing_sub_signed(rhs).0
    }
}

impl SubAssign<Duration> for BaseTime {
    fn sub_assign(&mut self, rhs: Duration) {
        *self = self.sub(rhs)
    }
}

impl Sub<BaseTime> for BaseTime {
    type Output = Duration;

    fn sub(self, rhs: BaseTime) -> Self::Output {
        self.signed_duration_since(rhs)
    }
}

impl Debug for BaseTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (hour, min, sec) = self.hms();
        let (sec, nano) = if self.frac >= 1_000_000_000 {
            (sec + 1, self.frac - 1_000_000_000)
        } else {
            (sec, self.frac)
        };

        write!(f, "{hour:02}:{min:02}:{sec:02}")?;

        if nano == 0 {
            Ok(())
        } else if nano % 1_000_000 == 0 {
            // 将纳秒之中的微秒部分直接输出
            write!(f, ".{:03}", nano / 1_000_000)
        } else if nano % 1_000 == 0 {
            // 将纳秒中的毫秒部分直接输出
            write!(f, ".{:06}", nano / 1_000)
        } else {
            // 纳秒部分直接输出
            write!(f, ".{nano:09}")
        }
    }
}

impl Display for BaseTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

#[cfg(test)]
mod test {
    include!("../../tests/ut/base/ut_time.rs");
}
