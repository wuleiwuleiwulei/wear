use super::scan;
use crate::base::date::BaseDate;
use crate::base::datetime::BaseDateTime;
use crate::base::time::BaseTime;
use crate::datetime::DateTime;
use crate::error::{ParseErrorKind, TimeError};
use crate::format::scan::{long_weekday, short_weekday};
use crate::format::{Item, Numeric};
use crate::offset::fixed::FixedOffset;
use crate::offset::{Offset, TimeZone};
use crate::Weekday;
use std::borrow::Borrow;

/// 日期与时间的解析器
#[derive(Default)]
pub struct Parsed {
    /// 年份
    pub year: Option<i32>,

    /// 年份（mod 100）
    pub year_mod_100: Option<i32>,

    /// 月份（1--12）
    pub month: Option<u32>,

    /// Day of the week.
    pub weekday: Option<Weekday>,

    /// 当前年份的第几天（1--365 或者 1--366）
    pub ordinal: Option<u32>,

    /// 当前月份的第几天（1--28，1--29，1--30，1--31）
    pub day: Option<u32>,

    /// 小时数（0--23）
    pub hour: Option<u32>,

    /// 分钟数（0--59）
    pub minute: Option<u32>,

    /// 秒数（0--60，将闰秒也统计其中）
    pub second: Option<u32>,

    /// 纳秒数（0--999_999_999）
    pub nanosecond: Option<u32>,

    /// 从本地时间到世界协调时的偏移量（以秒数为单位）
    pub offset: Option<i32>,
}

/// 检查旧值是否为空或与新值相同
fn set_if_consistent<T: PartialEq>(old: &mut Option<T>, new: T) -> Result<(), TimeError> {
    if let Some(ref old) = *old {
        if *old == new {
            Ok(())
        } else {
            Err(TimeError::from(ParseErrorKind::Impossible))
        }
    } else {
        *old = Some(new);
        Ok(())
    }
}

fn i64_as_i32(value: i64) -> Option<i32> {
    if value >= i32::MIN as i64 && value <= i32::MAX as i64 {
        Some(value as i32)
    } else {
        None
    }
}

fn i64_as_u32(value: i64) -> Option<u32> {
    if value >= u32::MIN as i64 && value <= u32::MAX as i64 {
        Some(value as u32)
    } else {
        None
    }
}

impl Parsed {
    /// 返回数值都是初始值的解析器结构
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::Parsed;
    ///
    /// // 创建解析器
    /// let mut parsed = Parsed::new();
    /// ```
    pub fn new() -> Parsed {
        Parsed::default()
    }

    /// 设定当前年份，
    /// 同时设置 year 和 year_mod_100 将会报错，两者不可共存
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::Parsed;
    ///
    /// // 创建解析器
    /// let mut parsed = Parsed::new();
    ///
    /// // 设定解析器的年份
    /// parsed.set_year(1999);
    /// assert_eq!(parsed.year.unwrap(), 1999);
    /// ```
    pub fn set_year(&mut self, value: i64) -> Result<(), TimeError> {
        set_if_consistent(
            &mut self.year,
            i64_as_i32(value).ok_or_else(|| TimeError::from(ParseErrorKind::Overflow))?,
        )
    }

    /// 设定当前年份（mod 100），
    /// 同时设置 year 和 year_mod_100 将会报错，两者不可共存
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::Parsed;
    ///
    /// // 创建解析器
    /// let mut parsed = Parsed::new();
    ///
    /// // 设定解析器的年份（mod 100）
    /// parsed.set_year_mod_100(1999 % 100);
    /// assert_eq!(parsed.year_mod_100.unwrap(), 1999 % 100);
    /// ```
    pub fn set_year_mod_100(&mut self, value: i64) -> Result<(), TimeError> {
        set_if_consistent(
            &mut self.year_mod_100,
            i64_as_i32(value).ok_or_else(|| TimeError::from(ParseErrorKind::Overflow))?,
        )
    }

    /// 设定当前月份
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::Parsed;
    ///
    /// // 创建解析器
    /// let mut parsed = Parsed::new();
    ///
    /// // 设定解析器的月份
    /// parsed.set_month(1);
    /// assert_eq!(parsed.month.unwrap(), 1);
    /// ```
    pub fn set_month(&mut self, value: i64) -> Result<(), TimeError> {
        set_if_consistent(
            &mut self.month,
            i64_as_u32(value).ok_or_else(|| TimeError::from(ParseErrorKind::Overflow))?,
        )
    }

    /// Set the current day of the week.
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::{Parsed, Weekday};
    ///
    /// // Creating a parser.
    /// let mut parsed = Parsed::new();
    ///
    /// // Set the day of the week for the parser.
    /// parsed.set_weekday(Weekday::Sunday);
    /// assert_eq!(parsed.weekday.unwrap(), Weekday::Sunday);
    /// ```
    pub fn set_weekday(&mut self, weekday: Weekday) -> Result<(), TimeError> {
        set_if_consistent(&mut self.weekday, weekday)
    }

    /// 设定当前天数
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::Parsed;
    ///
    /// // 创建解析器
    /// let mut parsed = Parsed::new();
    ///
    /// // 设定解析器的天数
    /// parsed.set_day(10);
    /// assert_eq!(parsed.day.unwrap(), 10);
    /// ```
    pub fn set_day(&mut self, value: i64) -> Result<(), TimeError> {
        set_if_consistent(
            &mut self.day,
            i64_as_u32(value).ok_or_else(|| TimeError::from(ParseErrorKind::Overflow))?,
        )
    }

    /// 设定当前小时数
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::Parsed;
    ///
    /// // 创建解析器
    /// let mut parsed = Parsed::new();
    ///
    /// // 设定解析器的小时数
    /// parsed.set_hour(1);
    /// assert_eq!(parsed.hour.unwrap(), 1);
    pub fn set_hour(&mut self, value: i64) -> Result<(), TimeError> {
        set_if_consistent(
            &mut self.hour,
            i64_as_u32(value).ok_or_else(|| TimeError::from(ParseErrorKind::Overflow))?,
        )
    }

    /// 设定当前分钟数
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::Parsed;
    ///
    /// // 创建解析器
    /// let mut parsed = Parsed::new();
    ///
    /// // 设定解析器的分钟数
    /// parsed.set_minute(10);
    /// assert_eq!(parsed.minute.unwrap(), 10);
    pub fn set_minute(&mut self, value: i64) -> Result<(), TimeError> {
        set_if_consistent(
            &mut self.minute,
            i64_as_u32(value).ok_or_else(|| TimeError::from(ParseErrorKind::Overflow))?,
        )
    }

    /// 设定当前秒数
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::Parsed;
    ///
    /// // 创建解析器
    /// let mut parsed = Parsed::new();
    ///
    /// // 设定解析器的秒数
    /// parsed.set_second(10);
    /// assert_eq!(parsed.second.unwrap(), 10);
    pub fn set_second(&mut self, value: i64) -> Result<(), TimeError> {
        set_if_consistent(
            &mut self.second,
            i64_as_u32(value).ok_or_else(|| TimeError::from(ParseErrorKind::Overflow))?,
        )
    }

    /// 设定当前纳秒数
    /// # Examples
    ///
    /// ```
    /// use ylong_time::Parsed;
    ///
    /// // 创建解析器
    /// let mut parsed = Parsed::new();
    ///
    /// // 设定解析器的纳秒数
    /// parsed.set_nanosecond(10);
    /// assert_eq!(parsed.nanosecond.unwrap(), 10);
    #[inline]
    pub fn set_nanosecond(&mut self, value: i64) -> Result<(), TimeError> {
        set_if_consistent(
            &mut self.nanosecond,
            i64_as_u32(value).ok_or_else(|| TimeError::from(ParseErrorKind::Overflow))?,
        )
    }

    /// 设定当前偏移量
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::Parsed;
    ///
    /// // 创建解析器
    /// let mut parsed = Parsed::new();
    ///
    /// // 设定解析器的偏移量
    /// parsed.set_offset(10);
    /// assert_eq!(parsed.offset.unwrap(), 10);
    pub fn set_offset(&mut self, value: i64) -> Result<(), TimeError> {
        set_if_consistent(
            &mut self.offset,
            i64_as_i32(value).ok_or_else(|| TimeError::from(ParseErrorKind::Overflow))?,
        )
    }

    /// 根据当前 'Parsed' 中的数据创建 'BaseDate'
    /// 当前解析器支持分析的情况通过 year, month, day 构造日期
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::{parse, Parsed};
    ///
    /// // 创建解析器
    /// let mut parsed = Parsed::new();
    ///
    /// // 使用 parse() 函数对解析器进行解析后得到日期，
    /// // 直接使用的情况下，解析格式由用户自定义，
    /// // 此处直接设置解析器便于得到结果。
    /// parsed.set_year(1999);
    /// parsed.set_month(1);
    /// parsed.set_day(10);
    ///
    /// // 解析得到日期
    /// let base_date = parsed.as_base_date().unwrap();
    /// ```
    pub fn as_base_date(&self) -> Result<BaseDate, TimeError> {
        match (self.year, self.year_mod_100, self.month, self.day) {
            (Some(year), None, Some(month), Some(day)) => BaseDate::from_ymd(year, month, day),
            (None, Some(year_mod_100 @ 0..=99), Some(month), Some(day)) => BaseDate::from_ymd(
                year_mod_100 + if year_mod_100 < 70 { 2000 } else { 1900 },
                month,
                day,
            ),
            _ => Err(TimeError::from(ParseErrorKind::Impossible)),
        }
    }

    /// 根据当前 'Parsed' 中的数据创建 'BaseTime'
    /// 当前解析器支持分析的情况通过 hour, minute, second, nanosecond/_ 构造时间
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::{parse, Parsed};
    ///
    /// // 创建解析器
    /// let mut parsed = Parsed::new();
    ///
    /// // 使用 parse() 函数对解析器进行解析后得到日期，
    /// // 直接使用的情况下，解析格式由用户自定义，
    /// // 此处直接设置解析器便于得到结果。
    /// parsed.set_hour(1);
    /// parsed.set_minute(10);
    /// parsed.set_second(10);
    ///
    /// // 解析得到日期
    /// let base_time = parsed.as_base_time().unwrap();
    /// ```
    pub fn as_base_time(&self) -> Result<BaseTime, TimeError> {
        match (self.hour, self.minute, self.second, self.nanosecond) {
            (Some(hour), Some(minute), Some(second), Some(nanosecond)) => {
                BaseTime::from_hms_nano(hour, minute, second, nanosecond)
            }
            (Some(hour), Some(minute), Some(second), None) => {
                BaseTime::from_hms(hour, minute, second)
            }
            (Some(hour), Some(minute), None, None) => BaseTime::from_hms(hour, minute, 0),
            _ => Err(TimeError::from(ParseErrorKind::Impossible)),
        }
    }

    /// 根据当前的 'Parsed' 中的数据创建 'BaseDateTime'
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::{parse, Parsed};
    ///
    /// // 创建解析器
    /// let mut parsed = Parsed::new();
    ///
    /// // 使用 parse() 函数对解析器进行解析后得到日期，
    /// // 直接使用的情况下，解析格式由用户自定义，
    /// // 此处直接设置解析器便于得到结果。
    /// parsed.set_year(1999);
    /// parsed.set_month(1);
    /// parsed.set_day(10);
    /// parsed.set_hour(1);
    /// parsed.set_minute(10);
    /// parsed.set_second(10);
    ///
    /// // 解析得到日期
    /// let base_datetime = parsed.as_base_datetime().unwrap();
    /// ```
    pub fn as_base_datetime(&self) -> Result<BaseDateTime, TimeError> {
        let base_date = self.as_base_date();
        let base_time = self.as_base_time();

        match (base_date, base_time) {
            (Ok(base_date), Ok(base_time)) => Ok(BaseDateTime::new(base_date, base_time)),
            (Err(err), _) => Err(err),
            (_, Err(err)) => Err(err),
        }
    }

    /// 根据当前的 `Parsed` 中的数据创建 `DateTime`
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::{parse, Parsed};
    ///
    /// // 创建解析器
    /// let mut parsed = Parsed::new();
    ///
    /// // 使用 parse() 函数对解析器进行解析后得到日期，
    /// // 直接使用的情况下，解析格式由用户自定义，
    /// // 此处直接设置解析器便于得到结果。
    /// parsed.set_year(1999);
    /// parsed.set_month(1);
    /// parsed.set_day(10);
    /// parsed.set_hour(1);
    /// parsed.set_minute(10);
    /// parsed.set_second(10);
    /// parsed.set_offset(0);
    ///
    /// // 解析得到日期
    /// let datetime = parsed.as_datetime().unwrap();
    /// ```
    pub fn as_datetime(&self) -> Result<DateTime<FixedOffset>, TimeError> {
        let offset = self
            .offset
            .ok_or_else(|| TimeError::from(ParseErrorKind::Impossible))?;
        let base_datetime = self.as_base_datetime()?;
        let offset = FixedOffset::east(offset)?;

        let datetime: DateTime<FixedOffset> =
            DateTime::from_utc(base_datetime - offset.fix(), offset);
        Ok(datetime)
    }

    /// 根据当前的 'Parsed' 中的数据创建带有时区的 `DateTime<Tz>`
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::{FixedOffset, parse, Parsed};
    ///
    /// // 创建解析器
    /// let mut parsed = Parsed::new();
    ///
    /// // 使用 parse() 函数对解析器进行解析后得到日期，
    /// // 直接使用的情况下，解析格式由用户自定义，
    /// // 此处直接设置解析器便于得到结果。
    /// parsed.set_year(1999);
    /// parsed.set_month(1);
    /// parsed.set_day(10);
    /// parsed.set_hour(1);
    /// parsed.set_minute(10);
    /// parsed.set_second(10);
    /// parsed.set_offset(0);
    ///
    /// // 解析得到日期
    /// let datetime_with_timezone = parsed.as_datetime_with_timezone(&FixedOffset::east(0).unwrap()).unwrap();
    /// ```
    pub fn as_datetime_with_timezone<Tz: TimeZone>(
        &self,
        tz: &Tz,
    ) -> Result<DateTime<Tz>, TimeError> {
        let base_datetime = self.as_base_datetime()?;
        Ok(tz.from_local_datetime(&base_datetime))
    }
}

pub fn parse<'a, I, B>(parsed: &mut Parsed, s: &str, items: I) -> Result<(), TimeError>
where
    I: Iterator<Item = B>,
    B: Borrow<Item<'a>>,
{
    parse_internal(parsed, s, items)
        .map(|_| ())
        .map_err(|(_s, e)| e)
}

fn parse_internal<'a, 'b, I, B>(
    parsed: &mut Parsed,
    mut s: &'b str,
    items: I,
) -> Result<&'b str, (&'b str, TimeError)>
where
    I: Iterator<Item = B>,
    B: Borrow<Item<'a>>,
{
    macro_rules! try_parse {
        ($e:expr) => {{
            match $e {
                Ok((left_s, v)) => {
                    s = left_s;
                    v
                }
                Err(e) => return Err((s, e)),
            }
        }};
    }

    for item in items {
        match *item.borrow() {
            // 该分支匹配解析器中的空格情况
            Item::Space(_) => s = s.trim_start(),

            // 该分支匹配解析器中的字符情况
            Item::Literal(literal) => {
                // 待匹配字符长度比整体长
                if s.len() < literal.len() {
                    return Err((s, TimeError::from(ParseErrorKind::TooShort)));
                }
                // 待匹配字符匹配失败
                if !s.starts_with(literal) {
                    return Err((s, TimeError::from(ParseErrorKind::Invalid)));
                }
                s = &s[literal.len()..]
            }

            // 该分支匹配解析器中的数字情况
            Item::Numeric(ref num, _) => {
                type Func = fn(&mut Parsed, i64) -> Result<(), TimeError>;

                let (width, signed, func): (usize, bool, Func) = match *num {
                    Numeric::Year => (4, true, Parsed::set_year),
                    Numeric::YearMod100 => (2, false, Parsed::set_year_mod_100),
                    Numeric::Month => (2, false, Parsed::set_month),
                    Numeric::Day => (2, false, Parsed::set_day),
                    Numeric::Hour => (2, false, Parsed::set_hour),
                    Numeric::Minute => (2, false, Parsed::set_minute),
                    Numeric::Second => (2, false, Parsed::set_second),
                    Numeric::Nanosecond => (9, false, Parsed::set_nanosecond),
                    _ => {
                        panic!("now not supported for parsing")
                    }
                };

                // 防止出现空格的情况出现
                s = s.trim_start();

                // 判断是否存在 '+'/'-'，然后根据符号进行匹配
                let temp = if signed {
                    if s.starts_with('+') {
                        try_parse!(scan::number(&s[1..], 1, width))
                    } else if s.starts_with('-') {
                        let temp = try_parse!(scan::number(&s[1..], 1, width));
                        0_i64
                            .checked_sub(temp)
                            .ok_or((s, TimeError::from(ParseErrorKind::Overflow)))?
                    } else {
                        try_parse!(scan::number(s, 1, width))
                    }
                } else {
                    try_parse!(scan::number(s, 1, width))
                };

                // 完成解析器内部数值更新
                func(parsed, temp).map_err(|e| (s, e))?;
            }

            // 该分支匹配解析器中的特殊格式情况
            Item::Specific(ref spec) => {
                use super::Specific::*;

                match spec {
                    &ShortWeekdayName => {
                        let weekday = try_parse!(short_weekday(s));
                        parsed.set_weekday(weekday).map_err(|e| (s, e))?
                    }

                    &LongWeekdayName => {
                        let weekday = try_parse!(long_weekday(s));
                        parsed.set_weekday(weekday).map_err(|e| (s, e))?
                    }

                    &Nanosecond => {
                        if s.starts_with('.') {
                            let nanosecond = try_parse!(scan::nanosecond(&s[1..]));
                            parsed.set_nanosecond(nanosecond).map_err(|e| (s, e))?
                        }
                    }

                    &NanosecondThree => {
                        if s.starts_with('.') {
                            let nanosecond = try_parse!(scan::nanosecond(&s[1..4]));
                            parsed.set_nanosecond(nanosecond).map_err(|e| (s, e))?
                        }
                    }

                    &NanosecondSix => {
                        if s.starts_with('.') {
                            let nanosecond = try_parse!(scan::nanosecond(&s[1..7]));
                            parsed.set_nanosecond(nanosecond).map_err(|e| (s, e))?
                        }
                    }

                    &NanosecondNine => {
                        if s.starts_with('.') {
                            let nanosecond = try_parse!(scan::nanosecond(&s[1..10]));
                            parsed.set_nanosecond(nanosecond).map_err(|e| (s, e))?
                        }
                    }

                    RFC3339 => try_parse!(parse_rfc3339(parsed, s)),
                    _ => {
                        panic!("now not supported for parsing")
                    }
                }
            }

            // 该分支为解析器解析错误
            Item::Error => return Err((s, TimeError::from(ParseErrorKind::Invalid))),
        }
    }

    // 如果拥有多余的字符，是错误的
    if !s.is_empty() {
        Err((s, TimeError::from(ParseErrorKind::TooLong)))
    } else {
        Ok(s)
    }
}

fn parse_rfc3339<'a>(parsed: &mut Parsed, mut s: &'a str) -> Result<(&'a str, ()), TimeError> {
    macro_rules! try_parse {
        ($e:expr) => {{
            let (s_, v) = $e?;
            s = s_;
            v
        }};
    }

    // 年月日
    parsed.set_year(try_parse!(scan::number(s, 4, 4)))?;
    s = scan::char(s, b'-')?;
    parsed.set_month(try_parse!(scan::number(s, 2, 2)))?;
    s = scan::char(s, b'-')?;
    parsed.set_day(try_parse!(scan::number(s, 2, 2)))?;

    s = match s.as_bytes().first() {
        Some(&b't') | Some(&b'T') => &s[1..],
        Some(_) => return Err(TimeError::from(ParseErrorKind::Invalid)),
        None => return Err(TimeError::from(ParseErrorKind::TooShort)),
    };

    // 时分秒
    parsed.set_hour(try_parse!(scan::number(s, 2, 2)))?;
    s = scan::char(s, b':')?;
    parsed.set_minute(try_parse!(scan::number(s, 2, 2)))?;
    s = scan::char(s, b':')?;
    parsed.set_second(try_parse!(scan::number(s, 2, 2)))?;
    // 可能拥有的纳秒部分
    if s.starts_with('.') {
        parsed.set_nanosecond(try_parse!(scan::nanosecond(&s[1..])))?;
    }

    let offset = try_parse!(scan::timezone_offset_zulu(s, |s| scan::char(s, b':')));
    if offset <= -86400 || offset >= 86400 {
        return Err(TimeError::from(ParseErrorKind::Overflow));
    }
    parsed.set_offset(i64::from(offset))?;

    Ok((s, ()))
}
