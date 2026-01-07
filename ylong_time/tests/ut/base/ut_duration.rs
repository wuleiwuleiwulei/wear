use crate::base::duration::Duration;

/*
 * @title  duration_valid() 函数UT测试
 * @design 无入参
 * @precon 无
 * @brief  描述测试用例执行
 *         1、创建有效的 `Duration`，校验结果是否正确
 *         2、创建无效的 `Duration`，校验结果是否正确
 * @expect 所产生的结果与测试用例描述对应
 * @auto   true
 */
#[test]
fn ut_duration_valid() {
    let duration = Duration::seconds(1).unwrap();
    assert!(duration.valid());

    let duration = Duration::weeks(1).unwrap() * 1_000_000_000;
    assert!(!duration.valid());
}

/*
 * @title  duration_weeks() 函数UT测试
 * @design 入参1: weeks
 *               有效值范围: 使得产生的 `Duration` 不超过有效值范围
 *               无效值范围: 使得产生的 `Duration` 超过有效值范围
 * @precon 无
 * @brief  描述测试用例执行
 *         1、有效值范围内的周数，从而创建 `Duration`，该 `Duration` 是有效值
 *         2、无效值范围内的周数，从而创建 `Duration`，返回 `Err(_)`
 * @expect 所产生的结果与测试用例描述对应
 * @auto   true
 */
#[test]
fn ut_duration_weeks() {
    let duration = Duration::weeks(1);
    assert!(duration.is_ok());

    let duration = Duration::weeks(1_000_000_000);
    assert!(duration.is_err());
}

/*
 * @title  duration_days() 函数UT测试
 * @design 入参1: days
 *               有效值范围: 使得产生的 `Duration` 不超过有效值范围
 *               无效值范围: 使得产生的 `Duration` 超过有效值范围
 * @precon 无
 * @brief  描述测试用例执行
 *         1、有效值范围内的天数，从而创建 `Duration`，该 `Duration` 是有效值
 *         2、无效值范围内的天数，从而创建 `Duration`，返回 `Err(_)`
 * @expect 所产生的结果与测试用例描述对应
 * @auto   true
 */
#[test]
fn ut_duration_days() {
    let duration = Duration::days(1);
    assert!(duration.is_ok());

    let duration = Duration::days(1_000_000_000);
    assert!(duration.is_err());
}

/*
 * @title  duration_hours() 函数UT测试
 * @design 入参1: hours
 *               有效值范围: 使得产生的 `Duration` 不超过有效值范围
 *               无效值范围: 使得产生的 `Duration` 超过有效值范围
 * @precon 无
 * @brief  描述测试用例执行
 *         1、有效值范围内的小时数，从而创建 `Duration`，该 `Duration` 是有效值
 *         2、无效值范围内的小时数，从而创建 `Duration`，返回 `Err(_)`
 * @expect 所产生的结果与测试用例描述对应
 * @auto   true
 */
#[test]
fn ut_duration_hours() {
    let duration = Duration::hours(1);
    assert!(duration.is_ok());

    let duration = Duration::hours(1_000_000_000_000);
    assert!(duration.is_err());
}

/*
 * @title  duration_minutes() 函数UT测试
 * @design 入参1: minutes
 *               有效值范围: 使得产生的 `Duration` 不超过有效值范围
 *               无效值范围: 使得产生的 `Duration` 超过有效值范围
 * @precon 无
 * @brief  描述测试用例执行
 *         1、有效值范围内的分钟数，从而创建 `Duration`，该 `Duration` 是有效值
 *         2、无效值范围内的分钟数，从而创建 `Duration`，返回 `Err(_)`
 * @expect 所产生的结果与测试用例描述对应
 * @auto   true
 */
#[test]
fn ut_duration_minutes() {
    let duration = Duration::minutes(1);
    assert!(duration.is_ok());

    let duration = Duration::minutes(1_000_000_000_000);
    assert!(duration.is_err());
}

/*
 * @title  duration_seconds() 函数UT测试
 * @design 入参1: seconds
 *               有效值范围: seconds >= -8272497168000 && seconds <= 8272434095999
 *               无效值范围: seconds < -8272497168000 || seconds > 8272434095999
 * @precon 无
 * @brief  描述测试用例执行
 *         1、有效值范围内的分钟数，从而创建 `Duration`，该 `Duration` 是有效值
 *         2、无效值范围内的分钟数，从而创建 `Duration`，返回 `Err(_)`
 * @expect 所产生的结果与测试用例描述对应
 * @auto   true
 */
#[test]
fn ut_duration_seconds() {
    let seconds = -8272497168000;
    let duration = Duration::seconds(seconds);
    assert!(duration.is_ok());

    let seconds = 8272434095999;
    let duration = Duration::seconds(seconds);
    assert!(duration.is_ok());

    let seconds = -8272497168001;
    let duration = Duration::seconds(seconds);
    assert!(duration.is_err());

    let seconds = 8272434096000;
    let duration = Duration::seconds(seconds);
    assert!(duration.is_err());
}

/*
 * @title  duration_milliseconds() 函数UT测试
 * @design 入参1: milliseconds
 *               有效值范围: 使得产生的 `Duration` 不超过有效值范围
 *               无效值范围: 使得产生的 `Duration` 超过有效值范围
 * @precon 无
 * @brief  描述测试用例执行
 *         1、有效值范围内的毫秒数，从而创建 `Duration`，该 `Duration` 是有效值
 *         2、无效值范围内的毫秒数，从而创建 `Duration`，返回 `Err(_)`
 * @expect 所产生的结果与测试用例描述对应
 * @auto   true
 */
#[test]
fn ut_duration_milliseconds() {
    let milliseconds = -8272497168000000;
    let duration = Duration::milliseconds(milliseconds);
    assert!(duration.is_ok());

    let milliseconds = 8272434095999000;
    let duration = Duration::milliseconds(milliseconds);
    assert!(duration.is_ok());

    let milliseconds = -8272497168001000;
    let duration = Duration::milliseconds(milliseconds);
    assert!(duration.is_err());

    let milliseconds = 8272434096000000;
    let duration = Duration::milliseconds(milliseconds);
    assert!(duration.is_err());

    let milliseconds = 1_000;
    let duration = Duration::milliseconds(milliseconds).unwrap();
    assert_eq!(duration, Duration::seconds(1).unwrap());
}

/*
 * @title  duration_microseconds() 函数UT测试
 * @design 入参1: microseconds
 *               有效值范围: 使得产生的 `Duration` 不超过有效值范围
 *               无效值范围: 使得产生的 `Duration` 超过有效值范围
 * @precon 无
 * @brief  描述测试用例执行
 *         1、有效值范围内的微秒数，从而创建 `Duration`，该 `Duration` 是有效值
 *         2、无效值范围内的微秒数，从而创建 `Duration`，返回 `Err(_)`
 * @expect 所产生的结果与测试用例描述对应
 * @auto   true
 */
#[test]
fn ut_duration_microseconds() {
    let microseconds = -8272497168000000000;
    let duration = Duration::microseconds(microseconds);
    assert!(duration.is_ok());

    let microseconds = 8272434095999000000;
    let duration = Duration::microseconds(microseconds);
    assert!(duration.is_ok());

    let microseconds = -8272497168001000000;
    let duration = Duration::microseconds(microseconds);
    assert!(duration.is_err());

    let microseconds = 8272434096000000000;
    let duration = Duration::microseconds(microseconds);
    assert!(duration.is_err());

    let microseconds = 1_000_000;
    let duration = Duration::microseconds(microseconds).unwrap();
    assert_eq!(duration, Duration::seconds(1).unwrap());
}

/*
 * @title  duration_nanoseconds() 函数UT测试
 * @design 入参1: nanoseconds，由于精度问题，不会触及边界
 * @precon 无
 * @brief  描述测试用例执行
 *         1、测试纳秒与秒之间的关系，确认无误即可
 * @expect 所产生的结果与测试用例描述对应
 * @auto   true
 */
#[test]
fn ut_duration_nanoseconds() {
    let nanoseconds = 1_000_000_000;
    let duration = Duration::nanoseconds(nanoseconds).unwrap();
    assert_eq!(duration, Duration::seconds(1).unwrap());
}

/*
 * @title  duration_num_weeks() 函数UT测试
 * @design 无入参
 * @precon 无
 * @brief  描述测试用例执行
 *         1、测试不同情况下的周数，校验返回值是否正确
 * @expect 所产生的结果与测试用例描述对应
 * @auto   true
 */
#[test]
fn ut_duration_num_weeks() {
    let weeks = 1;
    let duration = Duration::weeks(weeks).unwrap();
    assert_eq!(duration.num_weeks(), weeks);

    let days = 7;
    let duration = Duration::days(days).unwrap();
    assert_eq!(duration.num_weeks(), 1);

    let hours = 24 * 7;
    let duration = Duration::hours(hours).unwrap();
    assert_eq!(duration.num_weeks(), 1);

    let minutes = 24 * 7 * 60;
    let duration = Duration::minutes(minutes).unwrap();
    assert_eq!(duration.num_weeks(), 1);

    let seconds = 24 * 7 * 60 * 60;
    let duration = Duration::seconds(seconds).unwrap();
    assert_eq!(duration.num_weeks(), 1);

    let milliseconds = 24 * 7 * 60 * 60 * 1_000;
    let duration = Duration::milliseconds(milliseconds).unwrap();
    assert_eq!(duration.num_weeks(), 1);

    let microseconds = 24 * 7 * 60 * 60 * 1_000_000;
    let duration = Duration::microseconds(microseconds).unwrap();
    assert_eq!(duration.num_weeks(), 1);

    let nanoseconds = 24 * 7 * 60 * 60 * 1_000_000_000;
    let duration = Duration::nanoseconds(nanoseconds).unwrap();
    assert_eq!(duration.num_weeks(), 1);
}

/*
 * @title  duration_num_days() 函数UT测试
 * @design 无入参
 * @precon 无
 * @brief  描述测试用例执行
 *         1、测试不同情况下的天数，校验返回值是否正确
 * @expect 所产生的结果与测试用例描述对应
 * @auto   true
 */
#[test]
fn ut_duration_num_days() {
    let days = 1;
    let duration = Duration::days(days).unwrap();
    assert_eq!(duration.num_days(), 1);

    let hours = 24;
    let duration = Duration::hours(hours).unwrap();
    assert_eq!(duration.num_days(), 1);

    let minutes = 24 * 60;
    let duration = Duration::minutes(minutes).unwrap();
    assert_eq!(duration.num_days(), 1);

    let seconds = 24 * 60 * 60;
    let duration = Duration::seconds(seconds).unwrap();
    assert_eq!(duration.num_days(), 1);

    let milliseconds = 24 * 60 * 60 * 1_000;
    let duration = Duration::milliseconds(milliseconds).unwrap();
    assert_eq!(duration.num_days(), 1);

    let microseconds = 24 * 60 * 60 * 1_000_000;
    let duration = Duration::microseconds(microseconds).unwrap();
    assert_eq!(duration.num_days(), 1);

    let nanoseconds = 24 * 60 * 60 * 1_000_000_000;
    let duration = Duration::nanoseconds(nanoseconds).unwrap();
    assert_eq!(duration.num_days(), 1);
}

/*
 * @title  duration_num_hours() 函数UT测试
 * @design 无入参
 * @precon 无
 * @brief  描述测试用例执行
 *         1、测试不同情况下的小时数，校验返回值是否正确
 * @expect 所产生的结果与测试用例描述对应
 * @auto   true
 */
#[test]
fn ut_duration_num_hours() {
    let hours = 1;
    let duration = Duration::hours(hours).unwrap();
    assert_eq!(duration.num_hours(), 1);

    let minutes = 60;
    let duration = Duration::minutes(minutes).unwrap();
    assert_eq!(duration.num_hours(), 1);

    let seconds = 60 * 60;
    let duration = Duration::seconds(seconds).unwrap();
    assert_eq!(duration.num_hours(), 1);

    let milliseconds = 60 * 60 * 1_000;
    let duration = Duration::milliseconds(milliseconds).unwrap();
    assert_eq!(duration.num_hours(), 1);

    let microseconds = 60 * 60 * 1_000_000;
    let duration = Duration::microseconds(microseconds).unwrap();
    assert_eq!(duration.num_hours(), 1);

    let nanoseconds = 60 * 60 * 1_000_000_000;
    let duration = Duration::nanoseconds(nanoseconds).unwrap();
    assert_eq!(duration.num_hours(), 1);
}

/*
 * @title  duration_num_minutes() 函数UT测试
 * @design 无入参
 * @precon 无
 * @brief  描述测试用例执行
 *         1、测试不同情况下的分钟数，校验返回值是否正确
 * @expect 所产生的结果与测试用例描述对应
 * @auto   true
 */
#[test]
fn ut_duration_num_minutes() {
    let minutes = 1;
    let duration = Duration::minutes(minutes).unwrap();
    assert_eq!(duration.num_minutes(), 1);

    let seconds = 60;
    let duration = Duration::seconds(seconds).unwrap();
    assert_eq!(duration.num_minutes(), 1);

    let milliseconds = 60 * 1_000;
    let duration = Duration::milliseconds(milliseconds).unwrap();
    assert_eq!(duration.num_minutes(), 1);

    let microseconds = 60 * 1_000_000;
    let duration = Duration::microseconds(microseconds).unwrap();
    assert_eq!(duration.num_minutes(), 1);

    let nanoseconds = 60 * 1_000_000_000;
    let duration = Duration::nanoseconds(nanoseconds).unwrap();
    assert_eq!(duration.num_minutes(), 1);
}

/*
 * @title  duration_num_seconds() 函数UT测试
 * @design 无入参
 * @precon 无
 * @brief  描述测试用例执行
 *         1、测试不同情况下的秒数，校验返回值是否正确
 * @expect 所产生的结果与测试用例描述对应
 * @auto   true
 */
#[test]
fn ut_duration_num_seconds() {
    let seconds = 1;
    let duration = Duration::seconds(seconds).unwrap();
    assert_eq!(duration.num_seconds(), 1);

    let milliseconds = 1_000;
    let duration = Duration::milliseconds(milliseconds).unwrap();
    assert_eq!(duration.num_seconds(), 1);

    let microseconds = 1_000_000;
    let duration = Duration::microseconds(microseconds).unwrap();
    assert_eq!(duration.num_seconds(), 1);

    let nanoseconds = 1_000_000_000;
    let duration = Duration::nanoseconds(nanoseconds).unwrap();
    assert_eq!(duration.num_seconds(), 1);
}

/*
 * @title  duration_num_milliseconds() 函数UT测试
 * @design 无入参
 * @precon 无
 * @brief  描述测试用例执行
 *         1、测试不同情况下的毫秒数，校验返回值是否正确
 * @expect 所产生的结果与测试用例描述对应
 * @auto   true
 */
#[test]
fn ut_duration_num_milliseconds() {
    let milliseconds = 1;
    let duration = Duration::milliseconds(milliseconds).unwrap();
    assert_eq!(duration.num_milliseconds(), 1);

    let microseconds = 1_000;
    let duration = Duration::microseconds(microseconds).unwrap();
    assert_eq!(duration.num_milliseconds(), 1);

    let nanoseconds = 1_000_000;
    let duration = Duration::nanoseconds(nanoseconds).unwrap();
    assert_eq!(duration.num_milliseconds(), 1);
}

/*
 * @title  duration_num_microseconds() 函数UT测试
 * @design 无入参
 * @precon 无
 * @brief  描述测试用例执行
 *         1、测试不同情况下的微秒数，校验返回值是否正确
 * @expect 所产生的结果与测试用例描述对应
 * @auto   true
 */
#[test]
fn ut_duration_num_microseconds() {
    let microseconds = 1;
    let duration = Duration::microseconds(microseconds).unwrap();
    assert_eq!(duration.num_microseconds(), 1);

    let nanoseconds = 1_000;
    let duration = Duration::nanoseconds(nanoseconds).unwrap();
    assert_eq!(duration.num_microseconds(), 1);
}

/*
 * @title  duration_num_nanoseconds() 函数UT测试
 * @design 无入参
 * @precon 无
 * @brief  描述测试用例执行
 *         1、测试不同情况下的纳秒数，校验返回值是否正确
 * @expect 所产生的结果与测试用例描述对应
 * @auto   true
 */
#[test]
fn ut_duration_num_nanoseconds() {
    let nanoseconds = 1;
    let duration = Duration::nanoseconds(nanoseconds).unwrap();
    assert_eq!(duration.num_nanoseconds(), 1);
}
