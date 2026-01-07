use crate::format::scan::{char, long_weekday, number, short_weekday};
use crate::Weekday;

/*
 * @title  number() 函数UT测试
 * @design 入参1: s
 *             有效值范围: 符合 `rfc3339` 格式
 *             无效值范围: 不符合 `rfc3339` 格式
 *         入参2: min
 *             有效值范围: min <= max
 *             无效值范围: min > max
 *         入参3: max
 *             有效值范围: max >= min
 *             无效值范围: max < min
 * @precon 无
 * @brief  描述测试用例执行
 *         1、传入无效的 `min` 或 `max` 范围
 *         2、传入不符合格式要求的字符串，校验返回值是否正确
 *         3、传入符合格式要求的字符串，校验返回值是否正确
 * @expect 所得结果应当一一对应
 * @auto   true
 */
#[test]
fn ut_number() {
    // min > max
    let min = 4;
    let max = 3;
    let s = "abcde";
    assert!(number(s, min, max).is_err());

    // s 长度范围错误
    let min = 4;
    let max = 5;
    let s = "123";
    assert!(number(s, min, max).is_err());

    // s 之中拥有不正确的序列
    let min = 4;
    let max = 4;
    let s = "123a1234";
    assert!(number(s, min, max).is_err());

    // s 之中所包含的数字溢出了
    let min = 1;
    let max = 32;
    let s = "9223372036854775808";
    assert!(number(s, min, max).is_err());

    // s 是符合要求的字符串
    let min = 3;
    let max = 5;
    let s = "1234";
    assert_eq!(number(s, min, max).unwrap(), ("", 1234));

    // s 是符合要求的字符串
    let min = 3;
    let max = 5;
    let s = "12345";
    assert_eq!(number(s, min, max).unwrap(), ("", 12345));

    // s 是符合要求的字符串
    let min = 4;
    let max = 4;
    let s = "12345";
    assert_eq!(number(s, min, max).unwrap(), ("5", 1234));
}

/*
 * @title  char() 函数UT测试
 * @design 入参1: s
 *             有效值范围: 符合 `rfc3339` 格式
 *             无效值范围: 不符合 `rfc3339` 格式
 *         入参2: c，无无效值
 * @precon 无
 * @brief  描述测试用例执行
 *         1、使得 s 的第一个字符能够与 c 匹配，校验返回值是否正确
 *         2、使得 s 的第一个字符与 c 不匹配，校验返回值是否正确
 *         3、使得 s 的长度不够，校验返回值是否正确
 * @expect 所得结果应当一一对应
 * @auto   true
 */
#[test]
fn ut_char() {
    let s = "+123";
    let c = b'+';
    assert_eq!(char(s, c).unwrap(), "123");

    let s = "+123";
    let c = b'-';
    assert!(char(s, c).is_err());

    let s = "";
    let c = b'-';
    assert!(char(s, c).is_err());
}

/// UT test for short_weekday.
///
/// # Title
/// ut_short_weekday
///
/// # Brief
/// 1. Coverage testing.
/// 2. Check if the test results are correct.
#[test]
fn ut_short_weekday() {
    let s = "Mon";
    let result = short_weekday(s).unwrap();
    assert_eq!(result, ("", Weekday::Monday));

    let s = "Monabcd";
    let result = short_weekday(s).unwrap();
    assert_eq!(result, ("abcd", Weekday::Monday));

    let s = "Mod";
    let result = short_weekday(s);
    assert!(result.is_err());

    let s = "aMon";
    let result = short_weekday(s);
    assert!(result.is_err());
}

/// UT test for long_weekday.
///
/// # Title
/// ut_long_weekday
///
/// # Brief
/// 1. Coverage testing.
/// 2. Check if the test results are correct.
#[test]
fn ut_long_weekday() {
    let s = "Monday";
    let result = long_weekday(s).unwrap();
    assert_eq!(result, ("", Weekday::Monday));

    let s = "Tuesday";
    let result = long_weekday(s).unwrap();
    assert_eq!(result, ("", Weekday::Tuesday));

    let s = "Monday1";
    let result = long_weekday(s).unwrap();
    assert_eq!(result, ("1", Weekday::Monday));

    let s = "Monda";
    let result = long_weekday(s);
    assert!(result.is_err());

    let s = "aMonday";
    let result = long_weekday(s);
    assert!(result.is_err());
}
