use crate::base::duration::Duration;
use crate::base::time::BaseTime;

/*
 * @title  from_hms() 函数UT测试
 * @design 入参1: hour
 *               有效值范围: hour < 24
 *               无效值范围: hour >= 24
 *         入参2: minute
 *               有效值范围: minute < 60
 *               无效值范围: minute >= 60
 *         入参3: second
 *               有效值范围: second < 60
 *               无效值范围: second >= 60
 * @precon 无
 * @brief  描述测试用例执行
 *         1、hour、minute、second 均为有效值
 *         2、hour、minute 为有效值，second 为无效值
 *         3、hour、second 为有效值，minute 为无效值
 *         4、minute、second 为有效值，hour 为无效值
 *         5、hour 为有效值，minute、second 为无效值
 *         6、minute 为有效值，hour、second 为无效值
 *         7、second 为有效值，hour、minute 为无效值
 *         8、hour、minute、second 均为无效值
 * @expect 只有均为有效值的情况下会创建 `BaseTime` 结构，
 *         其他情况下，只会返回 `Err(_)`
 * @auto   true
 */
#[test]
fn ut_basetime_from_hms() {
    let (hour, minute, second) = (23, 59, 59);
    let basetime = BaseTime::from_hms(hour, minute, second);
    assert!(basetime.is_ok());

    let (hour, minute, second) = (23, 59, 60);
    let basetime = BaseTime::from_hms(hour, minute, second);
    assert!(basetime.is_err());

    let (hour, minute, second) = (23, 60, 59);
    let basetime = BaseTime::from_hms(hour, minute, second);
    assert!(basetime.is_err());

    let (hour, minute, second) = (24, 59, 59);
    let basetime = BaseTime::from_hms(hour, minute, second);
    assert!(basetime.is_err());

    let (hour, minute, second) = (24, 60, 60);
    let basetime = BaseTime::from_hms(hour, minute, second);
    assert!(basetime.is_err());

    let (hour, minute, second) = (24, 59, 60);
    let basetime = BaseTime::from_hms(hour, minute, second);
    assert!(basetime.is_err());

    let (hour, minute, second) = (24, 60, 59);
    let basetime = BaseTime::from_hms(hour, minute, second);
    assert!(basetime.is_err());

    let (hour, minute, second) = (24, 60, 60);
    let basetime = BaseTime::from_hms(hour, minute, second);
    assert!(basetime.is_err());
}

/*
 * @title  from_hms_milli() 函数UT测试
 * @design 入参1: hour
 *               有效值范围: hour < 24
 *               无效值范围: hour >= 24
 *         入参2: minute
 *               有效值范围: minute < 60
 *               无效值范围: minute >= 60
 *         入参3: second
 *               有效值范围: second < 60
 *               无效值范围: second >= 60
 *         入参4: milli
 *               有效值范围: milli < 2000
 *               无效值范围: milli >= 2000
 * @precon 无
 * @brief  描述测试用例执行
 *         1、hour、minute、second、milli 均为有效值
 *         2、hour、minute、second 为有效值，milli 为无效值
 *         3、hour、minute、milli 为有效值，second 为无效值
 *         4、hour、minute、milli 为有效值，minute 为无效值
 *         5、minute、second、milli 为有效值、hour 为无效值
 *         6、hour、minute 为有效值，second、milli 为无效值
 *         7、hour、second 为有效值，minute、milli 为无效值
 *         8、hour、milli 为有效值，minute、second 为无效值
 *         9、minute、second 为有效值，hour、milli 为无效值
 *         10、minute、milli 为有效值，hour、second 为无效值
 *         11、second、milli 为有效值，hour、minute 为无效值
 *         12、hour、minute、second 为无效值，milli 为有效值
 *         13、hour、minute、milli 为无效值，second 为有效值
 *         14、hour、minute、milli 为无效值，minute 为有效值
 *         15、minute、second、milli 为无效值、hour 为有效值
 *         16、hour、minute、second、milli 均为无效值
 * @expect 只有均为有效值的情况下会创建 `BaseTime` 结构，
 *         其他情况下，只会返回 `Err(_)`
 * @auto   true
 */
#[test]
fn ut_basetime_from_hms_milli() {
    let (hour, minute, second, milli) = (23, 59, 59, 1999);
    let basetime = BaseTime::from_hms_milli(hour, minute, second, milli);
    assert!(basetime.is_ok());

    let (hour, minute, second, milli) = (23, 59, 59, 2000);
    let basetime = BaseTime::from_hms_milli(hour, minute, second, milli);
    assert!(basetime.is_err());

    let (hour, minute, second, milli) = (23, 59, 60, 1999);
    let basetime = BaseTime::from_hms_milli(hour, minute, second, milli);
    assert!(basetime.is_err());

    let (hour, minute, second, milli) = (23, 60, 59, 1999);
    let basetime = BaseTime::from_hms_milli(hour, minute, second, milli);
    assert!(basetime.is_err());

    let (hour, minute, second, milli) = (24, 59, 59, 1999);
    let basetime = BaseTime::from_hms_milli(hour, minute, second, milli);
    assert!(basetime.is_err());

    let (hour, minute, second, milli) = (23, 59, 60, 2000);
    let basetime = BaseTime::from_hms_milli(hour, minute, second, milli);
    assert!(basetime.is_err());

    let (hour, minute, second, milli) = (23, 60, 59, 2000);
    let basetime = BaseTime::from_hms_milli(hour, minute, second, milli);
    assert!(basetime.is_err());

    let (hour, minute, second, milli) = (23, 60, 60, 1999);
    let basetime = BaseTime::from_hms_milli(hour, minute, second, milli);
    assert!(basetime.is_err());

    let (hour, minute, second, milli) = (24, 59, 59, 2000);
    let basetime = BaseTime::from_hms_milli(hour, minute, second, milli);
    assert!(basetime.is_err());

    let (hour, minute, second, milli) = (24, 59, 60, 1999);
    let basetime = BaseTime::from_hms_milli(hour, minute, second, milli);
    assert!(basetime.is_err());

    let (hour, minute, second, milli) = (24, 60, 59, 1999);
    let basetime = BaseTime::from_hms_milli(hour, minute, second, milli);
    assert!(basetime.is_err());

    let (hour, minute, second, milli) = (24, 60, 60, 1999);
    let basetime = BaseTime::from_hms_milli(hour, minute, second, milli);
    assert!(basetime.is_err());

    let (hour, minute, second, milli) = (24, 60, 59, 2000);
    let basetime = BaseTime::from_hms_milli(hour, minute, second, milli);
    assert!(basetime.is_err());

    let (hour, minute, second, milli) = (24, 59, 60, 2000);
    let basetime = BaseTime::from_hms_milli(hour, minute, second, milli);
    assert!(basetime.is_err());

    let (hour, minute, second, milli) = (23, 60, 60, 2000);
    let basetime = BaseTime::from_hms_milli(hour, minute, second, milli);
    assert!(basetime.is_err());

    let (hour, minute, second, milli) = (24, 60, 60, 2000);
    let basetime = BaseTime::from_hms_milli(hour, minute, second, milli);
    assert!(basetime.is_err());
}

/*
 * @title  from_hms_micro() 函数UT测试
 * @design 入参1: hour
 *               有效值范围: hour < 24
 *               无效值范围: hour >= 24
 *         入参2: minute
 *               有效值范围: minute < 60
 *               无效值范围: minute >= 60
 *         入参3: second
 *               有效值范围: second < 60
 *               无效值范围: second >= 60
 *         入参4: micro
 *               有效值范围: micro < 2_000_000
 *               无效值范围: micro >= 2_000_000
 * @precon 无
 * @brief  描述测试用例执行
 *         1、hour、minute、second、micro 均为有效值
 *         2、hour、minute、second 为有效值，micro 为无效值
 *         3、hour、minute、micro 为有效值，second 为无效值
 *         4、hour、minute、micro 为有效值，minute 为无效值
 *         5、minute、second、micro 为有效值、hour 为无效值
 *         6、hour、minute 为有效值，second、micro 为无效值
 *         7、hour、second 为有效值，minute、micro 为无效值
 *         8、hour、micro 为有效值，minute、second 为无效值
 *         9、minute、second 为有效值，hour、micro 为无效值
 *         10、minute、micro 为有效值，hour、second 为无效值
 *         11、second、micro 为有效值，hour、minute 为无效值
 *         12、hour、minute、second 为无效值，micro 为有效值
 *         13、hour、minute、micro 为无效值，second 为有效值
 *         14、hour、minute、micro 为无效值，minute 为有效值
 *         15、minute、second、micro 为无效值、hour 为有效值
 *         16、hour、minute、second、micro 均为无效值
 * @expect 只有均为有效值的情况下会创建 `BaseTime` 结构，
 *         其他情况下，只会返回 `Err(_)`
 * @auto   true
 */
#[test]
fn ut_basetime_from_hms_micro() {
    let (hour, minute, second, micro) = (23, 59, 59, 1_999_999);
    let basetime = BaseTime::from_hms_micro(hour, minute, second, micro);
    assert!(basetime.is_ok());

    let (hour, minute, second, micro) = (23, 59, 59, 2_000_000);
    let basetime = BaseTime::from_hms_micro(hour, minute, second, micro);
    assert!(basetime.is_err());

    let (hour, minute, second, micro) = (23, 59, 60, 1_999_999);
    let basetime = BaseTime::from_hms_micro(hour, minute, second, micro);
    assert!(basetime.is_err());

    let (hour, minute, second, micro) = (23, 60, 59, 1_999_999);
    let basetime = BaseTime::from_hms_micro(hour, minute, second, micro);
    assert!(basetime.is_err());

    let (hour, minute, second, micro) = (24, 59, 59, 1_999_999);
    let basetime = BaseTime::from_hms_micro(hour, minute, second, micro);
    assert!(basetime.is_err());

    let (hour, minute, second, micro) = (23, 59, 60, 2_000_000);
    let basetime = BaseTime::from_hms_micro(hour, minute, second, micro);
    assert!(basetime.is_err());

    let (hour, minute, second, micro) = (23, 60, 59, 2_000_000);
    let basetime = BaseTime::from_hms_micro(hour, minute, second, micro);
    assert!(basetime.is_err());

    let (hour, minute, second, micro) = (23, 60, 60, 1_999_999);
    let basetime = BaseTime::from_hms_micro(hour, minute, second, micro);
    assert!(basetime.is_err());

    let (hour, minute, second, micro) = (24, 59, 59, 2_000_000);
    let basetime = BaseTime::from_hms_micro(hour, minute, second, micro);
    assert!(basetime.is_err());

    let (hour, minute, second, micro) = (24, 59, 60, 1_999_999);
    let basetime = BaseTime::from_hms_micro(hour, minute, second, micro);
    assert!(basetime.is_err());

    let (hour, minute, second, micro) = (24, 60, 59, 1_999_999);
    let basetime = BaseTime::from_hms_micro(hour, minute, second, micro);
    assert!(basetime.is_err());

    let (hour, minute, second, micro) = (24, 60, 60, 1_999_999);
    let basetime = BaseTime::from_hms_micro(hour, minute, second, micro);
    assert!(basetime.is_err());

    let (hour, minute, second, micro) = (24, 60, 59, 2_000_000);
    let basetime = BaseTime::from_hms_micro(hour, minute, second, micro);
    assert!(basetime.is_err());

    let (hour, minute, second, micro) = (24, 59, 60, 2_000_000);
    let basetime = BaseTime::from_hms_micro(hour, minute, second, micro);
    assert!(basetime.is_err());

    let (hour, minute, second, micro) = (23, 60, 60, 2_000_000);
    let basetime = BaseTime::from_hms_micro(hour, minute, second, micro);
    assert!(basetime.is_err());

    let (hour, minute, second, micro) = (24, 60, 60, 2_000_000);
    let basetime = BaseTime::from_hms_micro(hour, minute, second, micro);
    assert!(basetime.is_err());
}

/*
 * @title  from_hms_nano() 函数UT测试
 * @design 入参1: hour
 *               有效值范围: hour < 24
 *               无效值范围: hour >= 24
 *         入参2: minute
 *               有效值范围: minute < 60
 *               无效值范围: minute >= 60
 *         入参3: second
 *               有效值范围: second < 60
 *               无效值范围: second >= 60
 *         入参4: nano
 *               有效值范围: nano < 2_000_000_000
 *               无效值范围: nano >= 2_000_000_000
 * @precon 无
 * @brief  描述测试用例执行
 *         1、hour、minute、second、nano 均为有效值
 *         2、hour、minute、second 为有效值，nano 为无效值
 *         3、hour、minute、nano 为有效值，second 为无效值
 *         4、hour、minute、nano 为有效值，minute 为无效值
 *         5、minute、second、nano 为有效值、hour 为无效值
 *         6、hour、minute 为有效值，second、nano 为无效值
 *         7、hour、second 为有效值，minute、nano 为无效值
 *         8、hour、nano 为有效值，minute、second 为无效值
 *         9、minute、second 为有效值，hour、nano 为无效值
 *         10、minute、nano 为有效值，hour、second 为无效值
 *         11、second、nano 为有效值，hour、minute 为无效值
 *         12、hour、minute、second 为无效值，nano 为有效值
 *         13、hour、minute、nano 为无效值，second 为有效值
 *         14、hour、minute、nano 为无效值，minute 为有效值
 *         15、minute、second、nano 为无效值、hour 为有效值
 *         16、hour、minute、second、nano 均为无效值
 * @expect 只有均为有效值的情况下会创建 `BaseTime` 结构，
 *         其他情况下，只会返回 `Err(_)`
 * @auto   true
 */
#[test]
fn ut_basetime_from_hms_nano() {
    let (hour, minute, second, nano) = (23, 59, 59, 1_999_999_999);
    let basetime = BaseTime::from_hms_nano(hour, minute, second, nano);
    assert!(basetime.is_ok());

    let (hour, minute, second, nano) = (23, 59, 59, 2_000_000_000);
    let basetime = BaseTime::from_hms_nano(hour, minute, second, nano);
    assert!(basetime.is_err());

    let (hour, minute, second, nano) = (23, 59, 60, 1_999_999_999);
    let basetime = BaseTime::from_hms_nano(hour, minute, second, nano);
    assert!(basetime.is_err());

    let (hour, minute, second, nano) = (23, 60, 59, 1_999_999_999);
    let basetime = BaseTime::from_hms_nano(hour, minute, second, nano);
    assert!(basetime.is_err());

    let (hour, minute, second, nano) = (24, 59, 59, 1_999_999_999);
    let basetime = BaseTime::from_hms_nano(hour, minute, second, nano);
    assert!(basetime.is_err());

    let (hour, minute, second, nano) = (23, 59, 60, 2_000_000_000);
    let basetime = BaseTime::from_hms_nano(hour, minute, second, nano);
    assert!(basetime.is_err());

    let (hour, minute, second, nano) = (23, 60, 59, 2_000_000_000);
    let basetime = BaseTime::from_hms_nano(hour, minute, second, nano);
    assert!(basetime.is_err());

    let (hour, minute, second, nano) = (23, 60, 60, 1_999_999_999);
    let basetime = BaseTime::from_hms_nano(hour, minute, second, nano);
    assert!(basetime.is_err());

    let (hour, minute, second, nano) = (24, 59, 59, 2_000_000_000);
    let basetime = BaseTime::from_hms_nano(hour, minute, second, nano);
    assert!(basetime.is_err());

    let (hour, minute, second, nano) = (24, 59, 60, 1_999_999_999);
    let basetime = BaseTime::from_hms_nano(hour, minute, second, nano);
    assert!(basetime.is_err());

    let (hour, minute, second, nano) = (24, 60, 59, 1_999_999_999);
    let basetime = BaseTime::from_hms_nano(hour, minute, second, nano);
    assert!(basetime.is_err());

    let (hour, minute, second, nano) = (24, 60, 60, 1_999_999_999);
    let basetime = BaseTime::from_hms_nano(hour, minute, second, nano);
    assert!(basetime.is_err());

    let (hour, minute, second, nano) = (24, 60, 59, 2_000_000_000);
    let basetime = BaseTime::from_hms_nano(hour, minute, second, nano);
    assert!(basetime.is_err());

    let (hour, minute, second, nano) = (24, 59, 60, 2_000_000_000);
    let basetime = BaseTime::from_hms_nano(hour, minute, second, nano);
    assert!(basetime.is_err());

    let (hour, minute, second, nano) = (23, 60, 60, 2_000_000_000);
    let basetime = BaseTime::from_hms_nano(hour, minute, second, nano);
    assert!(basetime.is_err());

    let (hour, minute, second, nano) = (24, 60, 60, 2_000_000_000);
    let basetime = BaseTime::from_hms_nano(hour, minute, second, nano);
    assert!(basetime.is_err());
}

/*
 * @title  hms() 函数UT测试
 * @design 无入参
 * @precon 无
 * @brief  描述测试用例执行
 *         1、通过 from_hms 接口创建 `BaseTime`，校验返回值是否正确
 *         2、通过 from_hms_milli 接口创建 `BaseTime`，校验返回值是否正确
 *         3、通过 from_hms_micro 接口创建 `BaseTime`，校验返回值是否正确
 *         4、通过 from_hms_nano 接口创建 `BaseTime`，校验返回值是否正确
 * @expect 返回正确的 (hour, minute, second) 元组结果
 * @auto   true
 */
#[test]
fn ut_basetime_hms() {
    let basetime = BaseTime::from_hms(12, 13, 14).unwrap();
    assert_eq!((12, 13, 14), basetime.hms());

    let basetime = BaseTime::from_hms_milli(12, 13, 14, 15).unwrap();
    assert_eq!((12, 13, 14), basetime.hms());

    let basetime = BaseTime::from_hms_micro(12, 13, 14, 15).unwrap();
    assert_eq!((12, 13, 14), basetime.hms());

    let basetime = BaseTime::from_hms_nano(12, 13, 14, 15).unwrap();
    assert_eq!((12, 13, 14), basetime.hms());
}

/*
 * @title  from_num_seconds_from_midnight() 函数UT测试
 * @design 入参1: secs
 *             有效值范围: secs < 86400
 *             无效值范围: secs >= 86400
 *         入参2: nanos
 *             有效值范围: nanos < 2_000_000_000
 *             无效值范围: nanos >= 2_000_000_000
 * @precon 无
 * @brief  描述测试用例执行
 *         1、secs、nanos 均为有效值
 *         2、secs 为有效值，nanos 为无效值
 *         3、secs 为无效值，nanos 为有效值
 *         4、secs、nanos 均为无效值
 * @expect 只有均为有效值的情况下会创建 `BaseTime` 结构，
 *         其他情况下，只会返回 `Err(_)`
 * @auto   true
 */
#[test]
fn ut_basetime_from_num_seconds_from_midnight() {
    let (secs, nanos) = (86399, 1_999_999_999);
    let basetime = BaseTime::from_num_seconds_from_midnight(secs, nanos);
    assert!(basetime.is_ok());

    let (secs, nanos) = (86400, 1_999_999_999);
    let basetime = BaseTime::from_num_seconds_from_midnight(secs, nanos);
    assert!(basetime.is_err());

    let (secs, nanos) = (86399, 2_000_000_000);
    let basetime = BaseTime::from_num_seconds_from_midnight(secs, nanos);
    assert!(basetime.is_err());

    let (secs, nanos) = (86400, 2_000_000_000);
    let basetime = BaseTime::from_num_seconds_from_midnight(secs, nanos);
    assert!(basetime.is_err());
}

/*
 * @title  overflowing_add_signed() 函数UT测试
 * @design 入参1: rhs，无无效值
 * @precon 无
 * @brief  描述测试用例执行
 *         1、通过 `Duration::weeks()` 进行时间变化，校验返回值是否正确
 *         2、通过 `Duration::days()` 进行时间变化，校验返回值是否正确
 *         3、通过 `Duration::hours()` 进行时间变化，校验返回值是否正确
 *         4、通过 `Duration::minutes()` 进行时间变化，校验返回值是否正确
 *         5、通过 `Duration::seconds()` 进行时间变化，校验返回值是否正确
 *         6、通过 `Duration::milliseconds()` 进行时间变化，校验返回值是否正确
 *         7、通过 `Duration::microseconds()` 进行时间变化，校验返回值是否正确
 *         8、通过 `Duration::nanoseconds()` 进行时间变化，校验返回值是否正确
 *         9、溢出场景测试
 * @expect 只有均为有效值的情况下会创建 `BaseTime` 结构，
 *         其他情况下，只会返回 `Err(_)`
 * @auto   true
 */
#[test]
fn ut_basetime_overflowing_add_signed() {
    let basetime = BaseTime::from_hms_milli(3, 4, 5, 678).unwrap();
    assert_eq!(
        basetime.overflowing_add_signed(Duration::weeks(1).unwrap()),
        (BaseTime::from_hms_milli(3, 4, 5, 678).unwrap(), 86400 * 7)
    );

    let basetime = BaseTime::from_hms_milli(3, 4, 5, 678).unwrap();
    assert_eq!(
        basetime.overflowing_add_signed(Duration::days(1).unwrap()),
        (BaseTime::from_hms_milli(3, 4, 5, 678).unwrap(), 86400)
    );

    let basetime = BaseTime::from_hms_milli(3, 4, 5, 678).unwrap();
    assert_eq!(
        basetime.overflowing_add_signed(Duration::hours(23).unwrap()),
        (BaseTime::from_hms_milli(2, 4, 5, 678).unwrap(), 86400)
    );

    let basetime = BaseTime::from_hms_milli(3, 4, 5, 678).unwrap();
    assert_eq!(
        basetime.overflowing_add_signed(Duration::minutes(59).unwrap()),
        (BaseTime::from_hms_milli(4, 3, 5, 678).unwrap(), 0)
    );

    let basetime = BaseTime::from_hms_milli(3, 4, 5, 678).unwrap();
    assert_eq!(
        basetime.overflowing_add_signed(Duration::seconds(59).unwrap()),
        (BaseTime::from_hms_milli(3, 5, 4, 678).unwrap(), 0)
    );

    let basetime = BaseTime::from_hms_milli(3, 4, 5, 678).unwrap();
    assert_eq!(
        basetime.overflowing_add_signed(Duration::milliseconds(2000).unwrap()),
        (BaseTime::from_hms_milli(3, 4, 7, 678).unwrap(), 0)
    );

    let basetime = BaseTime::from_hms_milli(3, 4, 5, 678).unwrap();
    assert_eq!(
        basetime.overflowing_add_signed(Duration::microseconds(2000).unwrap()),
        (BaseTime::from_hms_milli(3, 4, 5, 680).unwrap(), 0)
    );

    let basetime = BaseTime::from_hms_milli(3, 4, 5, 678).unwrap();
    assert_eq!(
        basetime.overflowing_add_signed(Duration::nanoseconds(2_000_000).unwrap()),
        (BaseTime::from_hms_milli(3, 4, 5, 680).unwrap(), 0)
    );

    let basetime = BaseTime::from_hms_nano(23, 59, 59, 1_999_999_999).unwrap();
    assert_eq!(
        basetime.overflowing_add_signed(Duration::nanoseconds(1).unwrap()),
        (BaseTime::from_hms_nano(0, 0, 0, 0).unwrap(), 86400),
    )
}

/*
 * @title  overflowing_sub_signed() 函数UT测试
 * @design 入参1: rhs，无无效值
 * @precon 无
 * @brief  描述测试用例执行
 *         1、通过 `Duration::weeks()` 进行时间变化，校验返回值是否正确
 *         2、通过 `Duration::days()` 进行时间变化，校验返回值是否正确
 *         3、通过 `Duration::hours()` 进行时间变化，校验返回值是否正确
 *         4、通过 `Duration::minutes()` 进行时间变化，校验返回值是否正确
 *         5、通过 `Duration::seconds()` 进行时间变化，校验返回值是否正确
 *         6、通过 `Duration::milliseconds()` 进行时间变化，校验返回值是否正确
 *         7、通过 `Duration::microseconds()` 进行时间变化，校验返回值是否正确
 *         8、通过 `Duration::nanoseconds()` 进行时间变化，校验返回值是否正确
 * @expect 只有均为有效值的情况下会创建 `BaseTime` 结构，
 *         其他情况下，只会返回 `Err(_)`
 * @auto   true
 */
#[test]
fn ut_overflowing_sub_signed() {
    let basetime = BaseTime::from_hms_milli(3, 4, 5, 678).unwrap();
    assert_eq!(
        basetime.overflowing_sub_signed(Duration::weeks(-1).unwrap()),
        (BaseTime::from_hms_milli(3, 4, 5, 678).unwrap(), -86400 * 7)
    );

    let basetime = BaseTime::from_hms_milli(3, 4, 5, 678).unwrap();
    assert_eq!(
        basetime.overflowing_sub_signed(Duration::days(-1).unwrap()),
        (BaseTime::from_hms_milli(3, 4, 5, 678).unwrap(), -86400)
    );

    let basetime = BaseTime::from_hms_milli(3, 4, 5, 678).unwrap();
    assert_eq!(
        basetime.overflowing_sub_signed(Duration::hours(-23).unwrap()),
        (BaseTime::from_hms_milli(2, 4, 5, 678).unwrap(), -86400)
    );

    let basetime = BaseTime::from_hms_milli(3, 4, 5, 678).unwrap();
    assert_eq!(
        basetime.overflowing_sub_signed(Duration::minutes(-59).unwrap()),
        (BaseTime::from_hms_milli(4, 3, 5, 678).unwrap(), 0)
    );

    let basetime = BaseTime::from_hms_milli(3, 4, 5, 678).unwrap();
    assert_eq!(
        basetime.overflowing_sub_signed(Duration::seconds(-59).unwrap()),
        (BaseTime::from_hms_milli(3, 5, 4, 678).unwrap(), 0)
    );

    let basetime = BaseTime::from_hms_milli(3, 4, 5, 678).unwrap();
    assert_eq!(
        basetime.overflowing_sub_signed(Duration::milliseconds(-2000).unwrap()),
        (BaseTime::from_hms_milli(3, 4, 7, 678).unwrap(), 0)
    );

    let basetime = BaseTime::from_hms_milli(3, 4, 5, 678).unwrap();
    assert_eq!(
        basetime.overflowing_sub_signed(Duration::microseconds(-2000).unwrap()),
        (BaseTime::from_hms_milli(3, 4, 5, 680).unwrap(), 0)
    );

    let basetime = BaseTime::from_hms_milli(3, 4, 5, 678).unwrap();
    assert_eq!(
        basetime.overflowing_sub_signed(Duration::nanoseconds(-2_000_000).unwrap()),
        (BaseTime::from_hms_milli(3, 4, 5, 680).unwrap(), 0)
    );
}

/*
 * @title  signed_duration_since() 函数UT测试
 * @design 入参1: rhs，无无效值
 * @precon 无
 * @brief  描述测试用例执行
 *         1、当前 `BaseTime` 大于所减的 `BaseTime`
 *         2、当前 `BaseTime` 小于所减的 `BaseTime`
 *         3、当前 `BaseTime` 等于所减的 `BaseTIme`
 *         4、不同精度的 `BaseTime` 相减
 * @expect 所得结果应当一一对应
 * @auto   true
 */
#[test]
fn ut_basetime_signed_duration_since() {
    let basetime_one = BaseTime::from_hms(3, 4, 5).unwrap();
    let basetime_two = BaseTime::from_hms(3, 4, 5).unwrap();
    assert_eq!(basetime_one - basetime_two, Duration::seconds(0).unwrap());

    let basetime_one = BaseTime::from_hms(3, 4, 6).unwrap();
    let basetime_two = BaseTime::from_hms(3, 4, 5).unwrap();
    assert_eq!(basetime_one - basetime_two, Duration::seconds(1).unwrap());

    let basetime_one = BaseTime::from_hms(3, 4, 4).unwrap();
    let basetime_two = BaseTime::from_hms(3, 4, 5).unwrap();
    assert_eq!(basetime_one - basetime_two, Duration::seconds(-1).unwrap());

    let basetime_one = BaseTime::from_hms(3, 4, 5).unwrap();
    let basetime_two = BaseTime::from_hms_nano(3, 4, 5, 1_000_000_000).unwrap();
    assert_eq!(basetime_one - basetime_two, Duration::seconds(-1).unwrap());
}
