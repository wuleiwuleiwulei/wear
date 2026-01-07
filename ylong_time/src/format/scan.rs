use crate::error::{ParseErrorKind, TimeError};
use crate::Weekday;

/// 尝试解析从 `min` 到 `max` 范围的非负数字
pub fn number(s: &str, min: usize, max: usize) -> Result<(&str, i64), TimeError> {
    if min > max {
        return Err(TimeError::from(ParseErrorKind::InvalidRange));
    }
    // 将当前字符串转换为 `ASCII` 的数组
    let bytes = s.as_bytes();
    // 当前字符串长度以及小于最小值，返回错误
    if bytes.len() < min {
        return Err(TimeError::from(ParseErrorKind::TooShort));
    }

    let mut n = 0_i64;
    // 'i' 为当前 `ASCII` 所在位置，'c' 为 `ASCII` 码
    for (i, c) in bytes.iter().take(max).cloned().enumerate() {
        if !c.is_ascii_digit() {
            if i < min {
                return Err(TimeError::from(ParseErrorKind::Invalid));
            } else {
                // 此处为 `min` != `max` 的情况，将会从非数字字符处进行分割
                return Ok((&s[i..], n));
            }
        }

        // 将字符转换为数字
        n = match n
            .checked_mul(10)
            .and_then(|n| n.checked_add((c - b'0') as i64))
        {
            Some(n) => n,
            None => return Err(TimeError::from(ParseErrorKind::Overflow)),
        };
    }

    // 返回字符剩余值以及匹配的数字
    Ok((&s[core::cmp::min(max, bytes.len())..], n))
}

/// 尝试解析某个给定的字符
pub fn char(s: &str, c1: u8) -> Result<&str, TimeError> {
    match s.as_bytes().first() {
        Some(&c) if c == c1 => Ok(&s[1..]),
        Some(_) => Err(TimeError::from(ParseErrorKind::Invalid)),
        None => Err(TimeError::from(ParseErrorKind::TooShort)),
    }
}

/// Attempt to parse a given string as the abbreviated day of the week (retaining only the first three characters).
pub fn short_weekday(s: &str) -> Result<(&str, Weekday), TimeError> {
    // only three characters.
    if s.len() < 3 {
        return Err(TimeError::from(ParseErrorKind::TooShort));
    }
    let data = &s.as_bytes()[0..3];
    let weekday = match data {
        b"Mon" => Weekday::Monday,
        b"Tue" => Weekday::Tuesday,
        b"Wed" => Weekday::Wednesday,
        b"Thu" => Weekday::Thursday,
        b"Fri" => Weekday::Friday,
        b"Sat" => Weekday::Saturday,
        b"Sun" => Weekday::Sunday,
        _ => return Err(TimeError::from(ParseErrorKind::Invalid)),
    };
    Ok((&s[3..], weekday))
}

/// Attempt to parse a given string as the full name of the day of the week.
pub fn long_weekday(s: &str) -> Result<(&str, Weekday), TimeError> {
    // All full names should be at least 6 characters long.
    if s.len() < 6 {
        return Err(TimeError::from(ParseErrorKind::TooShort));
    }

    static LONG_WEEKDAY_TAIL: [&str; 7] = ["day", "sday", "nesday", "rsday", "day", "urday", "day"];

    let (mut s, weekday) = short_weekday(s)?;
    let tail = LONG_WEEKDAY_TAIL[weekday.weekday0_to_u8() as usize];
    if s.len() >= tail.len() && &s[..tail.len()] == tail {
        s = &s[tail.len()..];
    }

    Ok((s, weekday))
}

/// 尝试解析某个给定的数字作为纳秒部分
pub fn nanosecond(s: &str) -> Result<(&str, i64), TimeError> {
    let origlen = s.len();
    // `s` 为剩余的字符，`v` 为已经选择的作为纳秒的部分
    let (s, v) = number(s, 1, 9)?;
    // 将被作为纳秒的位数
    let consumed = origlen - s.len();

    static SCALE: [i64; 10] = [
        0,
        100_000_000,
        10_000_000,
        1_000_000,
        100_000,
        10_000,
        1_000,
        100,
        10,
        1,
    ];
    let v = v
        .checked_mul(SCALE[consumed])
        .ok_or_else(|| TimeError::from(ParseErrorKind::Overflow))?;

    // 把下一个给定的 `char` 之前多余的数字去除
    let s = s.trim_start_matches(char::is_numeric);

    Ok((s, v))
}

/// 与 `timezone_offset` 功能相同但允许使用 `z`/`Z` 等效于 `+00::00`
pub fn timezone_offset_zulu<F>(s: &str, colon: F) -> Result<(&str, i32), TimeError>
where
    F: FnMut(&str) -> Result<&str, TimeError>,
{
    let bytes = s.as_bytes();
    match bytes.first() {
        // 满足 `z`/'Z'
        Some(&b'z') | Some(&b'Z') => Ok((&s[1..], 0)),
        // 判断是否满足 `utc`/'Utc'
        Some(&b'u') | Some(&b'U') => {
            if bytes.len() >= 3 {
                let (b, c) = (bytes[1], bytes[2]);
                match (b | 32, c | 32) {
                    (b't', b'c') => Ok((&s[3..], 0)),
                    _ => Err(TimeError::from(ParseErrorKind::Invalid)),
                }
            } else {
                Err(TimeError::from(ParseErrorKind::Invalid))
            }
        }
        _ => timezone_offset(s, colon),
    }
}

/// 解析时区偏移量部分
pub fn timezone_offset<F>(mut s: &str, mut colon: F) -> Result<(&str, i32), TimeError>
where
    F: FnMut(&str) -> Result<&str, TimeError>,
{
    fn digits(s: &str) -> Result<(u8, u8), TimeError> {
        let b = s.as_bytes();
        if b.len() < 2 {
            Err(TimeError::from(ParseErrorKind::TooShort))
        } else {
            Ok((b[0], b[1]))
        }
    }

    // 偏移量正负符号部分
    let negative = match s.as_bytes().first() {
        Some(&b'+') => false,
        Some(&b'-') => true,
        Some(_) => return Err(TimeError::from(ParseErrorKind::Invalid)),
        None => return Err(TimeError::from(ParseErrorKind::TooShort)),
    };
    s = &s[1..];

    // 偏移量小时部分
    let hours = match digits(s)? {
        (h1 @ b'0'..=b'9', h2 @ b'0'..=b'9') => i32::from((h1 - b'0') * 10 + (h2 - b'0')),
        _ => return Err(TimeError::from(ParseErrorKind::Invalid)),
    };
    s = &s[2..];

    // 其它符号部分，通过传入的函数进行判断
    s = colon(s)?;

    // 偏移量分钟部分
    let minutes = match digits(s)? {
        (m1 @ b'0'..=b'5', m2 @ b'0'..=b'9') => i32::from((m1 - b'0') * 10 + (m2 - b'0')),
        (b'6'..=b'9', b'0'..=b'9') => return Err(TimeError::from(ParseErrorKind::Overflow)),
        _ => return Err(TimeError::from(ParseErrorKind::Invalid)),
    };
    s = &s[2..];

    let seconds = hours * 3600 + minutes * 60;
    Ok((s, if negative { -seconds } else { seconds }))
}

#[cfg(test)]
mod test {
    include!("../../tests/ut/format/ut_scan.rs");
}
