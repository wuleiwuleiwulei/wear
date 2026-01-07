pub mod parse;
pub mod scan;
pub mod strftime;

use crate::base::date::BaseDate;
use crate::base::time::BaseTime;
use crate::offset::fixed::FixedOffset;
use crate::offset::Offset;
use crate::{Datelike, Timelike};
use std::borrow::Borrow;
use std::fmt::{Display, Formatter};

/// 用于 `Numeric` 的填充字符
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Pad {
    /// 无填充项
    None,
    /// `0` 填充项
    Zero,
    /// ` ` 填充项
    Space,
}

/// 数字项类型
/// 格式化宽度 (FW) 以及解析宽度 (PW)
///
/// `FW` 指的是能被格式化的最小宽度。
/// 如果宽度不够，并且填充项不是 [`Pad::None`]，那么将会进行左填充。
///
/// `PW` 指的是能被解析的最大宽度。
/// 解析器只尝试消耗从一个到给定的位数（贪婪地）。它还会修剪前面的空白（如果有）。
/// 它无法解析负数，因此无法格式化某些日期和时间，然后使用相同的格式化项解析。
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Numeric {
    /// 完整的格里公历年份 (FW=4, PW=∞)
    /// 如果给定初始化信号，那么可能接受 1 BCE 之前的年份以及 9999 CE 之后的年份
    Year,
    /// 格里公历年份除以 100 (世纪数，FW=PW=2)。表示非负年份。
    YearDiv100,
    /// 格里公历年份取余 100 (FW=PW=2)。不能为负数。
    YearMod100,
    /// 月份
    Month,
    /// 该月份的第几天 (FW=PW=2)
    Day,
    /// 该年份的第几天 (FW=PW=3)
    Ordinal,
    /// 二十四小时制的小时数 (FW=PW=2)
    Hour,
    /// 一整个小时的分钟数 (FW=PW=2)
    Minute,
    /// 一整个分钟的秒数
    Second,
    /// 一整个秒数的纳秒数 (FW=PW=9)
    /// 不会进行左对齐
    Nanosecond,
    /// 自 1970 年 1 月 1 日午夜 UTC 以来的非闰秒数 (FW=1, PW=∞)。
    /// 对于格式化，它假定在没有时区偏移时为 UTC。
    Timestamp,
}

/// 固定格式项
///
/// 拥有自己的格式化和解析规则。
/// 否则，他们将在指定的大小写中打印，但不区分大小写进行解析。
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Specific {
    /// abbreviating the name of the week,
    /// Print a three-letter-length name with the first letter of the word capitalized.
    ShortWeekdayName,
    /// Full name of the week,
    /// Print the complete name with the first letter of the word capitalized.
    LongWeekdayName,
    /// 缩写月份名称
    /// 在词首字母大写情况下打印三个字母长度的名字，在任何情况下读取相同的名字
    ShortMonthName,
    /// 完整月份名字
    /// 在词首字母大写的情况下打印完整的名字，在任何情况下读取缩写或全名的月份
    LongMonthName,
    /// 左对齐纳秒的可选点加一个或多个数字。
    /// 根据可用的精度，可以不打印，或打印 3、6 或 9 位数字。
    Nanosecond,
    /// 与 `Nanosecond` 相同，但是精度修改为 3
    NanosecondThree,
    /// 与 `Nanosecond` 相同，但是精度修改为 6
    NanosecondSix,
    /// 与 `Nanosecond` 相同，但是精度修改为 9
    NanosecondNine,
    /// 时区名字
    /// 不支持被解析，在解析中使用它会立即导致错误
    TimezoneName,
    /// 本地时间到世界协调时之间的偏移量
    /// 在解析器中，冒号可以省略或用任何数量的空格包围。
    TimezoneOffsetColon,
    /// 本地时间到世界协调时之间的偏移量
    /// 在解析器中，冒号可以省略或用任何数量的空格包围，`Z`可以是大写字母，也可以是小写字母。
    TimezoneOffsetColonZ,
    /// 与 [`Specific::TimezoneOffsetColon`] 相同，但是不打印冒号。
    /// 解析时允许选择是否存在冒号
    TimezoneOffset,
    /// 与 [`Specific::TimezoneOffsetColonZ`] 相同，但是不打印冒号。
    /// 解析时允许选择是否存在冒号
    TimezoneOffsetZ,
    /// `RFC 3339` 或 `ISO 8601` 日期与时间格式语法
    RFC3339,
}

/// 既用于格式化也用于解析格式化的结构
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Item<'a> {
    /// 字面上打印和解析的文本
    Literal(&'a str),
    /// 空格，按字面意思打印，但读取 0 个或多个的空格。
    Space(&'a str),
    /// 数字项。格式化时，可以选择填充到最大长度（如果有）
    /// 解析器只需忽略任何填充的空白和零。
    Numeric(Numeric, Pad),
    /// 固定格式项
    Specific(Specific),
    /// 错误情况
    Error,
}

#[macro_export]
macro_rules! lit {
    ($x: expr) => {
        Item::Literal($x)
    };
}

#[macro_export]
macro_rules! sp {
    ($x: expr) => {
        Item::Space($x)
    };
}

#[macro_export]
macro_rules! num {
    ($x: ident) => {
        Item::Numeric(Numeric::$x, Pad::None)
    };
}

#[macro_export]
macro_rules! num0 {
    ($x: ident) => {
        Item::Numeric(Numeric::$x, Pad::Zero)
    };
}

#[macro_export]
macro_rules! nums {
    ($x: ident) => {
        Item::Numeric(Numeric::$x, Pad::Space)
    };
}

#[macro_export]
macro_rules! spec {
    ($x: ident) => {
        Item::Specific(Specific::$x)
    };
}

/// 用于格式化的临时结构
#[derive(Debug)]
pub struct DelayedFormat<I> {
    date: BaseDate,
    time: BaseTime,
    off: (String, FixedOffset),
    items: I,
}

impl<'a, I: Iterator<Item = B> + Clone, B: Borrow<Item<'a>>> DelayedFormat<I> {
    pub fn new_with_offset<Off>(
        date: BaseDate,
        time: BaseTime,
        offset: &Off,
        items: I,
    ) -> DelayedFormat<I>
    where
        Off: Offset + std::fmt::Display,
    {
        let name_and_diff = (offset.to_string(), offset.fix());
        DelayedFormat {
            date,
            time,
            off: name_and_diff,
            items,
        }
    }
}

impl<'a, I: Iterator<Item = B> + Clone, B: Borrow<Item<'a>>> Display for DelayedFormat<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        format(f, &self.date, &self.time, &self.off, self.items.clone())
    }
}

/// 尝试按照指定格式格式化给定的参数
pub fn format<'a, I, B>(
    w: &mut std::fmt::Formatter,
    date: &BaseDate,
    time: &BaseTime,
    off: &(String, FixedOffset),
    items: I,
) -> std::fmt::Result
where
    I: Iterator<Item = B> + Clone,
    B: Borrow<Item<'a>>,
{
    let mut result = String::new();
    for item in items {
        format_inner(&mut result, date, time, off, item.borrow())?;
    }
    w.pad(&result)
}

fn format_inner(
    result: &mut String,
    date: &BaseDate,
    time: &BaseTime,
    off: &(String, FixedOffset),
    item: &Item,
) -> std::fmt::Result {
    use core::fmt::Write;

    fn div_rem(this: i32, other: i32) -> (i32, i32) {
        (this / other, this % other)
    }

    fn div_floor(this: i32, other: i32) -> i32 {
        match div_rem(this, other) {
            (d, r) if (r > 0 && other < 0) || (r < 0 && other > 0) => d - 1,
            (d, _) => d,
        }
    }

    fn mod_floor(this: i32, other: i32) -> i32 {
        match this % other {
            r if (r > 0 && other < 0) || (r < 0 && other > 0) => r + other,
            r => r,
        }
    }

    let (short_months, long_months) = {
        (
            &[
                "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
            ],
            &[
                "January",   // 一月
                "February",  // 二月
                "March",     // 三月
                "April",     // 四月
                "May",       // 五月
                "June",      // 六月
                "July",      // 七月
                "August",    // 八月
                "September", // 九月
                "October",   // 十月
                "November",  // 十一月
                "December",  // 十二月
            ],
        )
    };

    match *item {
        Item::Literal(s) | Item::Space(s) => result.push_str(s),
        Item::Numeric(ref spec, ref pad) => {
            use self::Numeric::*;

            let (width, v) = match *spec {
                Year => (4, date.year() as i64),
                YearDiv100 => (2, div_floor(date.year(), 100) as i64),
                YearMod100 => (2, mod_floor(date.year(), 100) as i64),
                Month => (2, date.month() as i64),
                Day => (2, date.day() as i64),
                Ordinal => (3, date.ordinal() as i64),
                Hour => (2, time.hour() as i64),
                Minute => (2, time.minute() as i64),
                Second => (
                    2,
                    time.second() as i64 + time.nanosecond() as i64 / 1_000_000_000,
                ),
                Nanosecond => (9, time.nanosecond() as i64 % 1_000_000_000),
                Timestamp => (1, {
                    let (_, off) = *off;
                    (date.and_time(*time) - off).timestamp().unwrap()
                }),
            };

            if (spec == &Year) && !(0..10_000).contains(&v) {
                match *pad {
                    Pad::None => write!(result, "{v:+}"),
                    Pad::Zero => write!(result, "{:+01$}", v, width + 1),
                    Pad::Space => write!(result, "{:+1$}", v, width + 1),
                }
            } else {
                match *pad {
                    Pad::None => write!(result, "{v}"),
                    Pad::Zero => write!(result, "{v:0width$}"),
                    Pad::Space => write!(result, "{v:width$}"),
                }
            }?
        }

        Item::Specific(ref spec) => {
            use self::Specific::*;

            /// 以 `+HHMM` 或者 `+HH:MM` 的格式打印偏移量
            fn write_local_minus_utc(
                out: &mut String,
                offset: FixedOffset,
                allow_zulu: bool,
                use_colon: bool,
            ) -> std::fmt::Result {
                let offset_minus = offset.local_minus_utc();
                if !allow_zulu || offset_minus != 0 {
                    let (sign, off) = if offset_minus < 0 {
                        ('-', -offset_minus)
                    } else {
                        ('+', offset_minus)
                    };
                    if use_colon {
                        write!(out, "{}{:02}:{:02}", sign, off / 3600, off / 60 % 60)
                    } else {
                        write!(out, "{}{:02}{:02}", sign, off / 3600, off / 60 % 60)
                    }
                } else {
                    out.push('Z');
                    Ok(())
                }
            }

            let ret = match *spec {
                ShortMonthName => {
                    result.push_str(short_months[date.month0() as usize]);
                    Ok(())
                }
                LongMonthName => {
                    result.push_str(long_months[date.month0() as usize]);
                    Ok(())
                }
                ShortWeekdayName => {
                    write!(result, "{}", date.weekday())
                }
                LongWeekdayName => {
                    write!(result, "{}", date.weekday().full_name())
                }
                Nanosecond => {
                    let nano = time.nanosecond() % 1_000_000_000;
                    if nano == 0 {
                        Ok(())
                    } else if nano % 1_000_000 == 0 {
                        write!(result, ".{:03}", nano / 1_000_000)
                    } else if nano % 1_000 == 0 {
                        write!(result, ".{:06}", nano / 1_000)
                    } else {
                        write!(result, ".{nano:09}")
                    }
                }
                NanosecondThree => {
                    let nano = time.nanosecond() % 1_000_000_000;
                    write!(result, ".{:03}", nano / 1_000_000)
                }
                NanosecondSix => {
                    let nano = time.nanosecond() % 1_000_000_000;
                    write!(result, ".{:06}", nano / 1_000)
                }
                NanosecondNine => {
                    let nano = time.nanosecond() % 1_000_000_000;
                    write!(result, ".{nano:09}")
                }
                TimezoneName => {
                    let (ref name, _) = *off;
                    result.push_str(name);
                    Ok(())
                }
                TimezoneOffsetColon => {
                    let (_, off) = *off;
                    write_local_minus_utc(result, off, false, true)
                }
                TimezoneOffsetColonZ => {
                    let (_, off) = *off;
                    write_local_minus_utc(result, off, true, true)
                }
                TimezoneOffset => {
                    let (_, off) = *off;
                    write_local_minus_utc(result, off, false, false)
                }
                TimezoneOffsetZ => {
                    let (_, off) = *off;
                    write_local_minus_utc(result, off, true, false)
                }
                RFC3339 => {
                    let (d, t, &(_, off)) = (date, time, off);
                    write!(result, "{d:?}T{t:?}")?;
                    write_local_minus_utc(result, off, false, true)
                }
            };

            match ret {
                Ok(_) => (),
                Err(err) => return Err(err),
            }
        }

        Item::Error => return Err(std::fmt::Error),
    }
    Ok(())
}
