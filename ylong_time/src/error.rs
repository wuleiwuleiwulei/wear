use std::fmt::{Debug, Display, Formatter};

/// 时间错误总结构
pub struct TimeError {
    repr: Repr,
}

/// 模块错误，其中再分为细粒度的错误类型
pub enum Repr {
    Base(BaseErrorKind),
    Parse(ParseErrorKind),
}

impl Debug for TimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.repr, f)
    }
}

impl Display for TimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.repr {
            Repr::Base(kind) => write!(f, "{}", kind.as_str()),
            Repr::Parse(kind) => write!(f, "{}", kind.as_str()),
        }
    }
}

impl Debug for Repr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Repr::Base(kind) => f.debug_tuple("Kind").field(&kind).finish(),
            Repr::Parse(kind) => f.debug_tuple("Kind").field(&kind).finish(),
        }
    }
}

/// Base 模块中的细粒度错误类型
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum BaseErrorKind {
    /// 当前总天数属于无效天数
    InvalidDays,
    /// 当前年份属于无效年份，范围应当为 i32::MAX >> 13 ~ i32::MIN >> 13
    InvalidYear,
    /// 当前年份的第几天属于无效天数，范围应当为 1 ~ 366 天
    InvalidOrdinal,
    /// 当前月份属于无效月份，范围应当为 1 ~ 12 月
    InvalidMonth,
    /// 当前月份的第几天属于无效天数，范围应当为 1 ~ 31 天
    InvalidDay,
    /// The number corresponding to the current weekday is an invalid value.
    InvalidWeekday,
    /// 当前 `Mdf` 结构为无效结构，应当是使用上一个日期或下一个日期所产生的无效结构
    InvalidMdf,
    /// 当前小时属于无效小时，范围应当为 0 ~ 23
    InvalidHour,
    /// 当前分钟属于无效分钟，范围应当为 0 ~ 59
    InvalidMin,
    /// 当前秒属于无效秒数，范围应当为 0 ~ 59
    InvalidSec,
    /// 当前纳秒属于无效纳秒，范围应当为 0 ~ 1_999_999_999
    InvalidNano,
    /// 当前 `Duration` 结构为无效结构
    InvalidDuration,
}

impl BaseErrorKind {
    pub(crate) fn as_str(&self) -> &'static str {
        match *self {
            BaseErrorKind::InvalidDays => "invalid days",
            BaseErrorKind::InvalidYear => "invalid year",
            BaseErrorKind::InvalidOrdinal => "invalid ordinal",
            BaseErrorKind::InvalidMonth => "invalid month",
            BaseErrorKind::InvalidWeekday => "invalid weekday",
            BaseErrorKind::InvalidDay => "invalid day",
            BaseErrorKind::InvalidMdf => "invalid mdf",
            BaseErrorKind::InvalidHour => "invalid hour",
            BaseErrorKind::InvalidMin => "invalid minute",
            BaseErrorKind::InvalidSec => "invalid second",
            BaseErrorKind::InvalidNano => "invalid nanosecond",
            BaseErrorKind::InvalidDuration => "invalid duration",
        }
    }
}

impl From<BaseErrorKind> for TimeError {
    fn from(kind: BaseErrorKind) -> Self {
        TimeError {
            repr: Repr::Base(kind),
        }
    }
}

/// Format 模块中的细粒度错误类型
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum ParseErrorKind {
    /// 错误的字符长度范围
    InvalidRange,
    /// 给定的字符串存在无效字符序列
    Invalid,
    /// 给定的字符串没有可能的日期与时间数据
    Impossible,
    /// 给定的字符串太短了
    TooShort,
    /// 给定的字符串太长了
    TooLong,
    /// 给定的数字超过了数值范围
    Overflow,
}

impl ParseErrorKind {
    pub(crate) fn as_str(&self) -> &'static str {
        match *self {
            ParseErrorKind::Invalid => "invalid character sequence",
            ParseErrorKind::Impossible => "impossible date or time",
            ParseErrorKind::TooShort => "specified string is too short",
            ParseErrorKind::TooLong => "specified string is too long",
            ParseErrorKind::Overflow => "Digital overflow range",
            ParseErrorKind::InvalidRange => "min > max, invalid range",
        }
    }
}

impl From<ParseErrorKind> for TimeError {
    fn from(kind: ParseErrorKind) -> Self {
        TimeError {
            repr: Repr::Parse(kind),
        }
    }
}
