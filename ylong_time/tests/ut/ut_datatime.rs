use crate::base::date::BaseDate;
use crate::base::datetime::BaseDateTime;
use crate::base::time::BaseTime;
use crate::datetime::DateTime;
use crate::offset::fixed::FixedOffset;
use crate::offset::local::Local;
use crate::offset::utc::Utc;
use crate::offset::{Offset, TimeZone};
use crate::{Datelike, Weekday};

/*
 * @title  datetime_format() 函数UT测试
 * @design 无入参
 * @precon 需要预先创建好 `DateTime` 结构
 * @brief  描述测试用例执行
 *         1、手动创建具有偏移量的日期时间，然后将其转换为指定的某个单独格式，校验结果
 *         2、手动创建具有偏移量的日期时间，然后将其转换为指定的多个格式组合，校验结果
 * @expect 所得结果应当一一对应
 * @auto   true
 */
#[test]
fn ut_datetime_format() {
    let base_date = BaseDate::from_ymd(2021, 6, 8).unwrap();
    let base_time = BaseTime::from_hms_nano(1, 2, 59, 1_999_000_001).unwrap();
    let base_datetime = BaseDateTime::new(base_date, base_time);
    let offset = FixedOffset::east(0).unwrap();
    let datetime: DateTime<FixedOffset> = DateTime::from_utc(base_datetime, offset);
    assert_eq!(datetime.format("%Y").to_string(), String::from("2021"));
    assert_eq!(datetime.format("%C").to_string(), String::from("20"));
    assert_eq!(datetime.format("%y").to_string(), String::from("21"));
    assert_eq!(datetime.format("%m").to_string(), String::from("06"));
    assert_eq!(datetime.format("%a").to_string(), String::from("Tue"));
    assert_eq!(datetime.format("%A").to_string(), String::from("Tuesday"));
    assert_eq!(datetime.format("%b").to_string(), String::from("Jun"));
    assert_eq!(datetime.format("%B").to_string(), String::from("June"));
    assert_eq!(datetime.format("%h").to_string(), String::from("Jun"));
    assert_eq!(datetime.format("%d").to_string(), String::from("08"));
    assert_eq!(datetime.format("%e").to_string(), String::from(" 8"));
    assert_eq!(datetime.format("%j").to_string(), String::from("159"));
    assert_eq!(datetime.format("%D").to_string(), String::from("06/08/21"));
    assert_eq!(
        datetime.format("%F").to_string(),
        String::from("2021-06-08")
    );
    assert_eq!(
        datetime.format("%v").to_string(),
        String::from(" 8-Jun-2021")
    );
    assert_eq!(datetime.format("%H").to_string(), String::from("01"));
    assert_eq!(datetime.format("%k").to_string(), String::from(" 1"));
    assert_eq!(datetime.format("%M").to_string(), String::from("02"));
    assert_eq!(datetime.format("%S").to_string(), String::from("60"));
    assert_eq!(datetime.format("%f").to_string(), String::from("999000001"));
    assert_eq!(
        datetime.format("%.f").to_string(),
        String::from(".999000001")
    );
    assert_eq!(datetime.format("%.3f").to_string(), String::from(".999"));
    assert_eq!(datetime.format("%.6f").to_string(), String::from(".999000"));
    assert_eq!(
        datetime.format("%.9f").to_string(),
        String::from(".999000001")
    );
    assert_eq!(datetime.format("%R").to_string(), String::from("01:02"));
    assert_eq!(datetime.format("%T").to_string(), String::from("01:02:60"));
    assert_eq!(datetime.format("%Z").to_string(), String::from("+00:00"));
    assert_eq!(datetime.format("%z").to_string(), String::from("+0000"));
    assert_eq!(datetime.format("%:z").to_string(), String::from("+00:00"));
    assert_eq!(
        datetime.format("%+").to_string(),
        String::from("2021-06-08T01:02:60.999000001+00:00")
    );
    assert_eq!(
        datetime.format("%s").to_string(),
        String::from("1623114180")
    );
    assert_eq!(datetime.format("%t").to_string(), String::from("\t"));
    assert_eq!(datetime.format("%n").to_string(), String::from("\n"));
    assert_eq!(datetime.format("%%").to_string(), String::from("%"));

    // 校验本地时间与世界协调时情况的时区情况
    let local_time = Local::now().unwrap();
    assert_eq!(local_time.format("%Z").to_string(), String::from("+08:00"));
    assert_eq!(local_time.format("%z").to_string(), String::from("+0800"));
    assert_eq!(local_time.format("%:z").to_string(), String::from("+08:00"));
    let utc_time = Utc::now().unwrap();
    assert_eq!(utc_time.format("%Z").to_string(), String::from("UTC"));
    assert_eq!(utc_time.format("%z").to_string(), String::from("+0000"));
    assert_eq!(utc_time.format("%:z").to_string(), String::from("+00:00"));

    // 校验组合情况
    let base_date = BaseDate::from_ymd(2021, 6, 8).unwrap();
    let base_time = BaseTime::from_hms_nano(1, 2, 59, 1_999_000_001).unwrap();
    let base_datetime = BaseDateTime::new(base_date, base_time);
    let offset = FixedOffset::east(0).unwrap();
    let datetime: DateTime<FixedOffset> = DateTime::from_utc(base_datetime, offset);
    assert_eq!(
        datetime.format("%b %e %T %Y").to_string(),
        String::from("Jun  8 01:02:60 2021")
    );
}

/*
 * @title  datetime_to_rfc3339() 函数UT测试
 * @design 无入参
 * @precon 需要预先创建好 `DateTime` 结构
 * @brief  描述测试用例执行
 *         1、调用 `Local::now` 得到日期时间，然后将其转换为 `rfc3339` 的格式，校验结果
 *         2、调用 `Utc::now` 得到日期时间，然后将其转换为 `rfc3339` 的格式，校验结果
 *         3、手动创建具有偏移量的日期时间，然后将其转换为 `rfc3339` 的格式，校验结果
 * @expect 所得结果应当一一对应
 * @auto   true
 */
#[test]
fn ut_datetime_to_rfc3339() {
    let utc = Utc::now().unwrap();
    let rfc3339 = DateTime::parse_from_rfc3339(utc.to_rfc3339().as_str()).unwrap();
    assert_eq!(utc.datetime, rfc3339.datetime);
    assert_eq!(utc.offset.fix(), rfc3339.offset);

    let local = Local::now().unwrap();
    let rfc3339 = DateTime::parse_from_rfc3339(local.to_rfc3339().as_str()).unwrap();
    assert_eq!(local.datetime, rfc3339.datetime);
    assert_eq!(local.offset, rfc3339.offset);

    let base_date = BaseDate::from_ymd(2021, 6, 15).unwrap();
    let base_time = BaseTime::from_hms_nano(0, 0, 0, 999_999_999).unwrap();
    let base_datetime = BaseDateTime::new(base_date, base_time);
    let offset = FixedOffset::east(60).unwrap();

    let datetime: DateTime<FixedOffset> = DateTime::from_utc(base_datetime, offset);
    // rfc3339 偏移量部分精准度只到分钟
    let rfc3339 = DateTime::parse_from_rfc3339(datetime.to_rfc3339().as_str()).unwrap();
    assert_eq!(datetime.datetime, rfc3339.datetime);
    assert_eq!(datetime.offset, rfc3339.offset);
}

/*
 * @title  datetime_parse_from_rfc3339() 函数UT测试
 * @design 入参1: s
 *             有效值范围: 符合 `rfc3339` 格式
 *             无效值范围: 不符合 `rfc3339` 格式
 * @precon 无
 * @brief  描述测试用例执行
 *         1、传入符合格式要求的字符串，校验返回值是否正确
 *         2、传入符合格式要求的字符串，拥有纳秒部分，检验返回值是否正确
 *         3、传入不符合格式要求的字符串，校验返回值是否正确
 * @expect 所得结果应当一一对应
 * @auto   true
 */
#[test]
fn ut_datetime_parse_from_rfc3339() {
    let s = "2021-06-15T17:01:51.954585101+08:00";
    let datetime = DateTime::parse_from_rfc3339(s).unwrap();
    let base_date = BaseDate::from_ymd(2021, 6, 15).unwrap();
    let base_time = BaseTime::from_hms_nano(17, 1, 51, 954_585_101).unwrap();
    let base_datetime = BaseDateTime::new(base_date, base_time);
    let offset = FixedOffset::east(8 * 3600).unwrap();
    assert_eq!(datetime.datetime + offset.fix(), base_datetime);
    assert_eq!(datetime.offset, offset);

    let s = "2021-06-15T17:01:51.954585101+23:59";
    let datetime = DateTime::parse_from_rfc3339(s).unwrap();
    let base_date = BaseDate::from_ymd(2021, 6, 15).unwrap();
    let base_time = BaseTime::from_hms_nano(17, 1, 51, 954_585_101).unwrap();
    let base_datetime = BaseDateTime::new(base_date, base_time);
    let offset = FixedOffset::east(23 * 3600 + 59 * 60).unwrap();
    assert_eq!(datetime.datetime + offset.fix(), base_datetime);
    assert_eq!(datetime.offset, offset);

    let s = "2021-06-15T17:01:51.954585101-23:59";
    let datetime = DateTime::parse_from_rfc3339(s).unwrap();
    let base_date = BaseDate::from_ymd(2021, 6, 15).unwrap();
    let base_time = BaseTime::from_hms_nano(17, 1, 51, 954_585_101).unwrap();
    let base_datetime = BaseDateTime::new(base_date, base_time);
    let offset = FixedOffset::west(23 * 3600 + 59 * 60).unwrap();
    assert_eq!(datetime.datetime + offset.fix(), base_datetime);
    assert_eq!(datetime.offset, offset);

    let s = "2021-06-15T17:01:51.954585101Utc";
    let datetime = DateTime::parse_from_rfc3339(s).unwrap();
    let base_date = BaseDate::from_ymd(2021, 6, 15).unwrap();
    let base_time = BaseTime::from_hms_nano(17, 1, 51, 954_585_101).unwrap();
    let base_datetime = BaseDateTime::new(base_date, base_time);
    let offset = FixedOffset::east(0).unwrap();
    assert_eq!(datetime.datetime + offset.fix(), base_datetime);
    assert_eq!(datetime.offset, offset);

    let s = "2021-06-15T17:01:51.954585101UTC";
    let datetime = DateTime::parse_from_rfc3339(s).unwrap();
    let base_date = BaseDate::from_ymd(2021, 6, 15).unwrap();
    let base_time = BaseTime::from_hms_nano(17, 1, 51, 954_585_101).unwrap();
    let base_datetime = BaseDateTime::new(base_date, base_time);
    let offset = FixedOffset::east(0).unwrap();
    assert_eq!(datetime.datetime + offset.fix(), base_datetime);
    assert_eq!(datetime.offset, offset);

    let s = "2021-06-15T17:01:51.954585101Z";
    let datetime = DateTime::parse_from_rfc3339(s).unwrap();
    let base_date = BaseDate::from_ymd(2021, 6, 15).unwrap();
    let base_time = BaseTime::from_hms_nano(17, 1, 51, 954_585_101).unwrap();
    let base_datetime = BaseDateTime::new(base_date, base_time);
    let offset = FixedOffset::east(0).unwrap();
    assert_eq!(datetime.datetime + offset.fix(), base_datetime);
    assert_eq!(datetime.offset, offset);

    let s = "2021-06-15T17:01:51Z";
    let datetime = DateTime::parse_from_rfc3339(s).unwrap();
    let base_date = BaseDate::from_ymd(2021, 6, 15).unwrap();
    let base_time = BaseTime::from_hms(17, 1, 51).unwrap();
    let base_datetime = BaseDateTime::new(base_date, base_time);
    let offset = FixedOffset::east(0).unwrap();
    assert_eq!(datetime.datetime + offset.fix(), base_datetime);
    assert_eq!(datetime.offset, offset);

    let s = "2021-06-15T17:01:51z";
    let datetime = DateTime::parse_from_rfc3339(s).unwrap();
    let base_date = BaseDate::from_ymd(2021, 6, 15).unwrap();
    let base_time = BaseTime::from_hms(17, 1, 51).unwrap();
    let base_datetime = BaseDateTime::new(base_date, base_time);
    let offset = FixedOffset::east(0).unwrap();
    assert_eq!(datetime.datetime + offset.fix(), base_datetime);
    assert_eq!(datetime.offset, offset);

    let s = "2021-06-15T17:1:51.954585101Z";
    let datetime = DateTime::parse_from_rfc3339(s);
    assert!(datetime.is_err());

    let s = "2021-06-15T17:1:51.954585101Z12312312";
    let datetime = DateTime::parse_from_rfc3339(s);
    assert!(datetime.is_err());

    let s = "2021-06-15T17:01:51.954585101  Z";
    let datetime = DateTime::parse_from_rfc3339(s);
    assert!(datetime.is_err());

    let s = "2021-06-15T127:01:51.954585101Z";
    let datetime = DateTime::parse_from_rfc3339(s);
    assert!(datetime.is_err());

    let s = "2021-06-15T127:01:51.954585d101Z";
    let datetime = DateTime::parse_from_rfc3339(s);
    assert!(datetime.is_err());

    let s = "";
    let datetime = DateTime::parse_from_rfc3339(s);
    assert!(datetime.is_err());

    let s = "20021-06-15T17:01:51.954585101Z";
    let datetime = DateTime::parse_from_rfc3339(s);
    assert!(datetime.is_err());

    let s = "2021-06-15 17:01:51.954585101Z";
    let datetime = DateTime::parse_from_rfc3339(s);
    assert!(datetime.is_err());
}

/*
 * @title  datetime_from_str 函数UT测试
 * @design 入参1: s，不同的时间字符，与参数 fmt 相对应即可
 *         入参2: fmt, 不同的匹配格式，当前支持基本的日期、时间匹配格式
 * @precon 无
 * @brief  描述测试用例执行
 *         1、传入符合格式要求的字符串，校验返回值是否正确
 *         2、传入不符合格式要求的字符串，校验返回值是否正确
 * @expect 所得结果应当一一对应
 * @auto   true
 */
#[test]
fn ut_datetime_from_str() {
    // 正常的匹配格式进行测试
    let time = "2022-02-26 14:32:23 .043214321";
    let fmt = "%Y-%m-%d %H:%M:%S %.f";
    let result = FixedOffset::east(8 * 3600)
        .unwrap()
        .datetime_from_str(time, fmt)
        .unwrap();
    let base_date = BaseDate::from_ymd(2022, 2, 26).unwrap();
    let base_time = BaseTime::from_hms_nano(14, 32, 23, 43_214_321).unwrap();
    let base_datetime = BaseDateTime::new(base_date, base_time);
    let datetime = DateTime::from_local(base_datetime, FixedOffset::east(8 * 3600).unwrap());
    assert_eq!(result, datetime);

    let time = "2022-02-26 14:32:23 .043214321";
    let fmt = "%Y-%m-%d %H:%M:%S %.3f";
    let result = FixedOffset::east(8 * 3600)
        .unwrap()
        .datetime_from_str(time, fmt)
        .unwrap();
    let base_date = BaseDate::from_ymd(2022, 2, 26).unwrap();
    let base_time = BaseTime::from_hms_nano(14, 32, 23, 43_000_000).unwrap();
    let base_datetime = BaseDateTime::new(base_date, base_time);
    let datetime = DateTime::from_local(base_datetime, FixedOffset::east(8 * 3600).unwrap());
    assert_eq!(result, datetime);

    let time = "2022-02-26 14:32:23 .043214321";
    let fmt = "%Y-%m-%d %H:%M:%S %.6f";
    let result = FixedOffset::east(8 * 3600)
        .unwrap()
        .datetime_from_str(time, fmt)
        .unwrap();
    let base_date = BaseDate::from_ymd(2022, 2, 26).unwrap();
    let base_time = BaseTime::from_hms_nano(14, 32, 23, 43_214_000).unwrap();
    let base_datetime = BaseDateTime::new(base_date, base_time);
    let datetime = DateTime::from_local(base_datetime, FixedOffset::east(8 * 3600).unwrap());
    assert_eq!(result, datetime);

    let time = "2022-02-26 14:32:23 .043214321";
    let fmt = "%Y-%m-%d %H:%M:%S %.9f";
    let result = FixedOffset::east(8 * 3600)
        .unwrap()
        .datetime_from_str(time, fmt)
        .unwrap();
    let base_date = BaseDate::from_ymd(2022, 2, 26).unwrap();
    let base_time = BaseTime::from_hms_nano(14, 32, 23, 43_214_321).unwrap();
    let base_datetime = BaseDateTime::new(base_date, base_time);
    let datetime = DateTime::from_local(base_datetime, FixedOffset::east(8 * 3600).unwrap());
    assert_eq!(result, datetime);

    let time = "2022-2-26 14:32:23 .043214321";
    let fmt = "%Y-%m-%d %H:%M:%S %.f";
    let result = FixedOffset::east(8 * 3600)
        .unwrap()
        .datetime_from_str(time, fmt)
        .unwrap();
    let base_date = BaseDate::from_ymd(2022, 2, 26).unwrap();
    let base_time = BaseTime::from_hms_nano(14, 32, 23, 43_214_321).unwrap();
    let base_datetime = BaseDateTime::new(base_date, base_time);
    let datetime = DateTime::from_local(base_datetime, FixedOffset::east(8 * 3600).unwrap());
    assert_eq!(result, datetime);

    let time = "2022-02-26 14:32:23.043214321";
    let fmt = "%Y-%m-%d %H:%M:%S%.f";
    let result = FixedOffset::east(8 * 3600)
        .unwrap()
        .datetime_from_str(time, fmt)
        .unwrap();
    let base_date = BaseDate::from_ymd(2022, 2, 26).unwrap();
    let base_time = BaseTime::from_hms_nano(14, 32, 23, 43_214_321).unwrap();
    let base_datetime = BaseDateTime::new(base_date, base_time);
    let datetime = DateTime::from_local(base_datetime, FixedOffset::east(8 * 3600).unwrap());
    assert_eq!(result, datetime);

    let time = "2022-02-26 14:32:23";
    let fmt = "%Y-%m-%d %H:%M:%S";
    let result = FixedOffset::east(8 * 3600)
        .unwrap()
        .datetime_from_str(time, fmt)
        .unwrap();
    let base_date = BaseDate::from_ymd(2022, 2, 26).unwrap();
    let base_time = BaseTime::from_hms(14, 32, 23).unwrap();
    let base_datetime = BaseDateTime::new(base_date, base_time);
    let datetime = DateTime::from_local(base_datetime, FixedOffset::east(8 * 3600).unwrap());
    assert_eq!(result, datetime);

    let time = "2022-02-26 14:32";
    let fmt = "%Y-%m-%d %H:%M";
    let result = FixedOffset::east(8 * 3600)
        .unwrap()
        .datetime_from_str(time, fmt)
        .unwrap();
    let base_date = BaseDate::from_ymd(2022, 2, 26).unwrap();
    let base_time = BaseTime::from_hms(14, 32, 0).unwrap();
    let base_datetime = BaseDateTime::new(base_date, base_time);
    let datetime = DateTime::from_local(base_datetime, FixedOffset::east(8 * 3600).unwrap());
    assert_eq!(result, datetime);

    // 非正常的匹配格式进行测试
    let time = "2022-02-26 14:32:23";
    let fmt = "%Y-%m-%d %H:%M:%S,";
    let result = FixedOffset::east(8 * 3600)
        .unwrap()
        .datetime_from_str(time, fmt);
    assert!(result.is_err());
}

/// UT test for datetime_weekday.
///
/// # Title
/// ut_datetime_weekday
///
/// # Brief
/// 1. testing the week of leap year.
/// 2. Check if the test results are correct.
#[test]
fn ut_datetime_weekday() {
    let s = "2020-02-29T17:01:51Z";
    let datetime = DateTime::parse_from_rfc3339(s).unwrap();
    let weekday = datetime.weekday();
    assert_eq!(weekday, Weekday::Saturday);

    let s = "2016-02-29T17:01:51Z";
    let datetime = DateTime::parse_from_rfc3339(s).unwrap();
    let weekday = datetime.weekday();
    assert_eq!(weekday, Weekday::Monday);
}
