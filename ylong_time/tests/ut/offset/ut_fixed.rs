use crate::base::datetime::BaseDateTime;
use crate::base::time::BaseTime;
use crate::datetime::DateTime;
use crate::offset::fixed::FixedOffset;
use crate::offset::utc::Utc;

/*
 * @title  add_fixedoffset_for_basetime() 函数UT测试
 * @design 入参1: rhs，无无效值
 * @precon 无
 * @brief  描述测试用例执行
 *         1、`FixedOffset` 为东时区
 *         2、`FixedOffset` 为西时区
 *         3、`FixedOffset` 偏移量为 0
 * @expect 通过不同偏移量计算后所得 `BaseTime` 结果正确
 * @auto   true
 */
#[test]
fn ut_add_fixedoffset_for_basetime() {
    let fixed_offset = FixedOffset::east(86399).unwrap();
    let base_time = BaseTime::from_hms(0, 0, 0).unwrap();
    let change_basetime = base_time + fixed_offset;
    assert_eq!(change_basetime, BaseTime::from_hms(23, 59, 59).unwrap());

    let fixed_offset = FixedOffset::west(86399).unwrap();
    let base_time = BaseTime::from_hms(23, 59, 59).unwrap();
    let change_basetime = base_time + fixed_offset;
    assert_eq!(change_basetime, BaseTime::from_hms(0, 0, 0).unwrap());

    let fixed_offset = FixedOffset::east(0).unwrap();
    let base_time = BaseTime::from_hms(0, 0, 0).unwrap();
    let change_basetime = base_time + fixed_offset;
    assert_eq!(change_basetime, BaseTime::from_hms(0, 0, 0).unwrap());

    let fixed_offset = FixedOffset::east(0).unwrap();
    let base_time = BaseTime::from_hms(0, 0, 0).unwrap();
    let change_basetime = base_time + fixed_offset;
    assert_eq!(change_basetime, BaseTime::from_hms(0, 0, 0).unwrap());
}

/*
 * @title  add_fixedoffset_for_basedatetime() 函数UT测试
 * @design 入参1: rhs，无无效值
 * @precon 无
 * @brief  描述测试用例执行
 *         1、`FixedOffset` 为东时区
 *         2、`FixedOffset` 为西时区
 *         3、`FixedOffset` 偏移量为 0
 * @expect 通过不同偏移量计算后所得 `BaseTime` 结果正确
 * @auto   true
 */
#[test]
fn ut_add_fixedoffset_for_basedatetime() {
    let fixed_offset = FixedOffset::east(86399).unwrap();
    let base_datetime = BaseDateTime::from_timestamp(1, 0).unwrap();
    assert_eq!(
        base_datetime + fixed_offset,
        BaseDateTime::from_timestamp(86400, 0).unwrap()
    );

    let fixed_offset = FixedOffset::west(86399).unwrap();
    let base_datetime = BaseDateTime::from_timestamp(86399, 0).unwrap();
    assert_eq!(
        base_datetime + fixed_offset,
        BaseDateTime::from_timestamp(0, 0).unwrap()
    );

    let fixed_offset = FixedOffset::east(0).unwrap();
    let base_datetime = BaseDateTime::from_timestamp(0, 0).unwrap();
    assert_eq!(
        base_datetime + fixed_offset,
        BaseDateTime::from_timestamp(0, 0).unwrap()
    );

    let fixed_offset = FixedOffset::west(0).unwrap();
    let base_datetime = BaseDateTime::from_timestamp(0, 0).unwrap();
    assert_eq!(
        base_datetime + fixed_offset,
        BaseDateTime::from_timestamp(0, 0).unwrap()
    );
}

/*
 * @title  add_fixedoffset_for_datetime() 函数UT测试
 * @design 入参1: rhs，无无效值
 * @precon 无
 * @brief  描述测试用例执行
 *         1、`FixedOffset` 为东时区
 *         2、`FixedOffset` 为西时区
 *         3、`FixedOffset` 偏移量为 0
 * @expect 通过不同偏移量计算后所得 `BaseTime` 结果正确
 * @auto   true
 */
#[test]
fn ut_add_fixedoffset_for_datetime() {
    let fixed_offset = FixedOffset::east(86399).unwrap();
    let base_datetime = BaseDateTime::from_timestamp(1, 0).unwrap();
    let datetime: DateTime<Utc> = DateTime::from_utc(base_datetime, Utc);
    let change_datetime: DateTime<Utc> =
        DateTime::from_utc(BaseDateTime::from_timestamp(86400, 0).unwrap(), Utc);
    assert_eq!(datetime + fixed_offset, change_datetime);

    let fixed_offset = FixedOffset::west(86399).unwrap();
    let base_datetime = BaseDateTime::from_timestamp(86399, 0).unwrap();
    let datetime: DateTime<Utc> = DateTime::from_utc(base_datetime, Utc);
    let change_datetime: DateTime<Utc> =
        DateTime::from_utc(BaseDateTime::from_timestamp(0, 0).unwrap(), Utc);
    assert_eq!(datetime + fixed_offset, change_datetime);

    let fixed_offset = FixedOffset::east(0).unwrap();
    let base_datetime = BaseDateTime::from_timestamp(0, 0).unwrap();
    let datetime: DateTime<Utc> = DateTime::from_utc(base_datetime, Utc);
    let change_datetime: DateTime<Utc> =
        DateTime::from_utc(BaseDateTime::from_timestamp(0, 0).unwrap(), Utc);
    assert_eq!(datetime + fixed_offset, change_datetime);

    let fixed_offset = FixedOffset::west(0).unwrap();
    let base_datetime = BaseDateTime::from_timestamp(0, 0).unwrap();
    let datetime: DateTime<Utc> = DateTime::from_utc(base_datetime, Utc);
    let change_datetime: DateTime<Utc> =
        DateTime::from_utc(BaseDateTime::from_timestamp(0, 0).unwrap(), Utc);
    assert_eq!(datetime + fixed_offset, change_datetime);
}
