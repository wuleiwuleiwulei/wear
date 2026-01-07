use crate::base::date::BaseDate;
use crate::base::datetime::BaseDateTime;
use crate::base::kernel::{MAX_YEAR, MIN_YEAR};
use crate::base::time::BaseTime;

/*
 * @title  basedatetime_from_timestamp() 函数UT测试
 * @design 入参1: secs
 *             有效值范围: secs >= -8334632851200 && secs <= 8210298412799
 *             无效值范围: secs < -8334632851200 || secs > 8210298412799
 *         入参2: nano
 *             有效值范围: nano >= 0 && nano < 2_000_000_000
 *             无效值范围: nano < 0 || nano >= 2_000_000_000
 * @precon 无
 * @brief  描述测试用例执行
 *         1、在有效范围上创建 `BaseDateTime` 结构
 *         2、在无效范围上创建 `BaseDateTime` 结构
 * @expect 只要在有效值范围内时，均会返回正确 `Duration` 结构，
 *         其他情况下，会返回 `Err(_)`
 * @auto   true
 */
#[test]
fn ut_basedatetime_from_timestamp() {
    let base_datetime = BaseDateTime::from_timestamp(8210298412799, 1_999_999_999).unwrap();
    let base_date = BaseDate::from_ymd(MAX_YEAR, 12, 31).unwrap();
    let base_time = BaseTime::from_hms_nano(23, 59, 59, 1_999_999_999).unwrap();
    assert_eq!(base_datetime, BaseDateTime::new(base_date, base_time));

    let base_datetime = BaseDateTime::from_timestamp(8210298412800, 1_999_999_999);
    assert!(base_datetime.is_err());

    let base_datetime = BaseDateTime::from_timestamp(-8334632851200, 0).unwrap();
    let base_date = BaseDate::from_ymd(MIN_YEAR, 1, 1).unwrap();
    let base_time = BaseTime::from_hms(0, 0, 0).unwrap();
    assert_eq!(base_datetime, BaseDateTime::new(base_date, base_time));

    let base_datetime = BaseDateTime::from_timestamp(-8334632851201, 0);
    assert!(base_datetime.is_err());
}

/*
 * @title  basedatetime_timestamp() 函数UT测试
 * @design 无入参，无异常分支
 * @precon 无
 * @brief  描述测试用例执行
 *         1、在有效范围上创建 `BaseDateTime` 结构
 *         2、创建完成后校验时间戳基本功能
 * @expect 校验成功
 * @auto   true
 */
#[test]
fn ut_basedatetime_timestamp() {
    let base_datetime = BaseDateTime::from_timestamp(0, 0).unwrap();
    assert_eq!(base_datetime.timestamp().unwrap(), 0);

    let base_datetime = BaseDateTime::from_timestamp(8210298412799, 1_999_999_999).unwrap();
    assert_eq!(base_datetime.timestamp().unwrap(), 8210298412800);
}

/*
 * @title  basedatetime_timestamp_millis() 函数UT测试
 * @design 无入参，无异常分支
 * @precon 无
 * @brief  描述测试用例执行
 *         1、在有效范围上创建 `BaseDateTime` 结构
 *         2、创建完成后校验时间戳基本功能
 * @expect 校验成功
 * @auto   true
 */
#[test]
fn ut_basedatetime_timestamp_millis() {
    let base_datetime = BaseDateTime::from_timestamp(0, 0).unwrap();
    assert_eq!(base_datetime.timestamp_millis().unwrap(), 0);

    let base_datetime = BaseDateTime::from_timestamp(8210298412799, 1_999_999_999).unwrap();
    assert_eq!(base_datetime.timestamp_millis().unwrap(), 8210298412801999);
}

/*
 * @title  basedatetime_timestamp_micros() 函数UT测试
 * @design 无入参，无异常分支
 * @precon 无
 * @brief  描述测试用例执行
 *         1、在有效范围上创建 `BaseDateTime` 结构
 *         2、创建完成后校验时间戳基本功能
 * @expect 校验成功
 * @auto   true
 */
#[test]
fn ut_basedatetime_timestamp_micros() {
    let base_datetime = BaseDateTime::from_timestamp(0, 0).unwrap();
    assert_eq!(base_datetime.timestamp_micros().unwrap(), 0);

    let base_datetime = BaseDateTime::from_timestamp(8210298412799, 1_999_999_999).unwrap();
    assert_eq!(
        base_datetime.timestamp_micros().unwrap(),
        8210298412801999999
    );
}

/*
 * @title  basedatetime_timestamp_nanos() 函数UT测试
 * @design 无入参，无异常分支
 * @precon 无
 * @brief  描述测试用例执行
 *         1、在有效范围上创建 `BaseDateTime` 结构
 *         2、创建完成后校验时间戳基本功能
 * @expect 校验成功
 * @auto   true
 */
#[test]
fn ut_basedatetime_timestamp_nanos() {
    let base_datetime = BaseDateTime::from_timestamp(0, 0).unwrap();
    assert_eq!(base_datetime.timestamp_nanos().unwrap(), 0);

    let base_datetime = BaseDateTime::from_timestamp(8210298412799, 1_999_999_999).unwrap();
    assert_eq!(
        base_datetime.timestamp_nanos().unwrap(),
        8210298412801999999999
    );
}
