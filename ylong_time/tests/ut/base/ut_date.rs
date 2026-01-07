use crate::base::date::BaseDate;
use crate::base::duration::Duration;
use crate::base::kernel::{Mdf, Of, SundayLetter, A, MAX_YEAR, MIN_YEAR};

/*
 * @title  basedate_from_mdf() 函数UT测试
 * @design 入参1: year
 *               有效值范围: year >= MIN_YEAR && year <= MAX_YEAR
 *               无效值范围: year < MIN_YEAR || year > MAX_YEAR
 *         入参2: mdf
 *               有效值范围: mdf.valid() == true
 *               无效值范围: mdf.valid() == false
 * @precon 无
 * @brief  描述测试用例执行
 *         1、year 与 mdf 均为有效值
 *         2、year 为有效值，mdf 为无效值
 *         3、year 为无效值，mdf 为有效值
 *         4、year 与 mdf 均为无效值
 * @expect 只有均为有效值的情况下会创建 `BaseDate` 结构，
 *         其他情况下，只会返回 `Err(_)`
 * @auto   true
 */
#[test]
fn ut_basedate_from_mdf() {
    let year = MAX_YEAR;
    let mdf = Mdf::new(1, 1, A).unwrap();
    let date = BaseDate::from_mdf(year, mdf);
    assert!(date.is_ok());

    let year = MIN_YEAR;
    let mdf = Mdf::new(1, 1, A).unwrap();
    let date = BaseDate::from_mdf(year, mdf);
    assert!(date.is_ok());

    let year = MAX_YEAR + 1;
    let mdf = Mdf::new(1, 1, A).unwrap();
    let date = BaseDate::from_mdf(year, mdf);
    assert!(date.is_err());

    let year = MIN_YEAR - 1;
    let mdf = Mdf::new(1, 1, A).unwrap();
    let date = BaseDate::from_mdf(year, mdf);
    assert!(date.is_err());

    let year = 2020;
    let mdf = Mdf::from_of(Of::new(1, A).unwrap().pred());
    let date = BaseDate::from_mdf(year, mdf);
    assert!(date.is_err());

    let year = MIN_YEAR - 1;
    let mdf = Mdf::from_of(Of::new(1, A).unwrap().pred());
    let date = BaseDate::from_mdf(year, mdf);
    assert!(date.is_err());
}

/*
 * @title  basedate_from_of() 函数UT测试
 * @design 入参1: year
 *               有效值范围: year >= MIN_YEAR && year <= MAX_YEAR
 *               无效值范围: year < MIN_YEAR || year > MAX_YEAR
 *         入参2: of
 *               有效值范围: of.valid() == true
 *               无效值范围: of.valid() == false
 * @precon 无
 * @brief  描述测试用例执行
 *         1、year 与 of 均为有效值
 *         2、year 为有效值，of 为无效值
 *         3、year 为无效值，of 为有效值
 *         4、year 与 of 均为无效值
 * @expect 只有均为有效值的情况下会创建 `BaseDate` 结构，
 *         其他情况下，只会返回 `Err(_)`
 * @auto   true
 */
#[test]
fn ut_basedate_from_of() {
    let year = MAX_YEAR;
    let of = Of::new(1, A).unwrap();
    let date = BaseDate::from_of(year, of);
    assert!(date.is_ok());

    let year = MIN_YEAR;
    let of = Of::new(1, A).unwrap();
    let date = BaseDate::from_of(year, of);
    assert!(date.is_ok());

    let year = MIN_YEAR - 1;
    let of = Of::new(1, A).unwrap();
    let date = BaseDate::from_of(year, of);
    assert!(date.is_err());

    let year = MAX_YEAR + 1;
    let of = Of::new(1, A).unwrap();
    let date = BaseDate::from_of(year, of);
    assert!(date.is_err());

    let year = 2021;
    let of = Of::new(1, A).unwrap().pred();
    let date = BaseDate::from_of(year, of);
    assert!(date.is_err());

    let year = MIN_YEAR - 1;
    let of = Of::new(1, A).unwrap().pred();
    let date = BaseDate::from_of(year, of);
    assert!(date.is_err());
}

/*
 * @title  basedate_mdf() 函数UT测试
 * @design 无入参
 * @precon 无
 * @brief  描述测试用例执行
 *         1、根据创建的不同 `BaseDate`，测试其返回结果
 * @expect 返回结果应当与真实 `Mdf` 构成一一对应
 * @auto   true
 */
#[test]
fn ut_basedate_mdf() {
    let basedate = BaseDate::from_ymd(2021, 1, 1).unwrap();
    assert_eq!(
        basedate.mdf(),
        Mdf::new(1, 1, SundayLetter::from_year(2021)).unwrap()
    );

    let basedate = BaseDate::from_ymd(2020, 12, 31).unwrap();
    assert_eq!(
        basedate.mdf(),
        Mdf::new(12, 31, SundayLetter::from_year(2020)).unwrap()
    );
}

/*
 * @title  basedate_of() 函数UT测试
 * @design 无入参
 * @precon 无
 * @brief  描述测试用例执行
 *         1、根据创建的不同 `BaseDate`，测试其返回结果
 * @expect 返回结果应当与真实 `Of` 构成一一对应
 * @auto   true
 */
#[test]
fn ut_basedate_of() {
    let basedate = BaseDate::from_ymd(2021, 1, 1).unwrap();
    assert_eq!(
        basedate.of(),
        Of::new(1, SundayLetter::from_year(2021)).unwrap()
    );

    let basedate = BaseDate::from_ymd(2021, 12, 31).unwrap();
    assert_eq!(
        basedate.of(),
        Of::new(365, SundayLetter::from_year(2021)).unwrap()
    );
}

/*
 * @title  basedate_with_mdf() 函数UT测试
 * @design 入参1: mdf
 *               有效值范围: mdf.valid() == true
 *               无效值范围: mdf.valid() == false
 * @precon 无
 * @brief  描述测试用例执行
 *         1、mdf 为有效值
 *         2、mdf 为无效值
 * @expect 只有为有效值的情况下会创建 `BaseDate` 结构，
 *         其他情况下，只会返回 `Err(_)`
 * @auto   true
 */
#[test]
fn ut_basedate_with_mdf() {
    let mdf = Mdf::new(1, 1, A).unwrap();
    let change_mdf = Mdf::new(12, 31, A).unwrap();
    let basedate = BaseDate::from_mdf(2021, mdf).unwrap();
    assert!(basedate.with_mdf(change_mdf).is_ok());

    let mdf = Mdf::new(1, 1, A).unwrap();
    let change_mdf = Mdf::from_of(Of::new(1, A).unwrap().pred());
    let basedate = BaseDate::from_mdf(2021, mdf).unwrap();
    assert!(basedate.with_mdf(change_mdf).is_err());
}

/*
 * @title  basedate_with_of() 函数UT测试
 * @design 入参1: of
 *               有效值范围: of.valid() == true
 *               无效值范围: of.valid() == false
 * @precon 无
 * @brief  描述测试用例执行
 *         1、of 为有效值
 *         2、of 为无效值
 * @expect 只有为有效值的情况下会创建 `BaseDate` 结构，
 *         其他情况下，只会返回 `Err(_)`
 * @auto   true
 */
#[test]
fn ut_basedate_with_of() {
    let mdf = Mdf::new(1, 1, A).unwrap();
    let of = Of::new(1, A).unwrap();
    let basedate = BaseDate::from_mdf(2021, mdf).unwrap();
    assert!(basedate.with_of(of).is_ok());

    let mdf = Mdf::new(1, 1, A).unwrap();
    let of = Of::new(1, A).unwrap().pred();
    let basedate = BaseDate::from_mdf(2021, mdf).unwrap();
    assert!(basedate.with_of(of).is_err());
}

/*
 * @title  basedate_from_ymd() 函数UT测试
 * @design 入参1: year
 *               有效值范围: year >= MIN_YEAR && year <= MAX_YEAR
 *               无效值范围: year < MIN_YEAR || year > MAX_YEAR
 *         入参2: month
 *               有效值范围: month != 0 && month <= 12
 *               无效值范围: month == 0 || month > 12
 *         入参3: day
 *               有效值范围: day != 0 && day <= 31
 *               无效值范围: day == 0 || day > 31
 * @precon 无
 * @brief  描述测试用例执行
 *         1、year、month、day 均为有效值
 *         2、year、month 为有效值，day 为无效值
 *         3、year、day 为有效值，month 为无效值
 *         4、month、day 为有效值，year 为无效值
 *         5、year 为有效值，month、day 为无效值
 *         6、month 为有效值，year、day 为无效值
 *         7、day 为有效值，year、month 为无效值
 *         8、year、month、day 均为无效值
 * @expect 只有均为有效值的情况下会创建 `BaseDate` 结构，
 *         其他情况下，只会返回 `Err(_)`
 * @auto   true
 */
#[test]
fn ut_basedate_from_ymd() {
    let (year, month, day) = (2021, 1, 1);
    assert!(BaseDate::from_ymd(year, month, day).is_ok());

    let (year, month, day) = (2021, 1, 32);
    assert!(BaseDate::from_ymd(year, month, day).is_err());

    let (year, month, day) = (2021, 13, 1);
    assert!(BaseDate::from_ymd(year, month, day).is_err());

    let (year, month, day) = (MAX_YEAR + 1, 1, 1);
    assert!(BaseDate::from_ymd(year, month, day).is_err());

    let (year, month, day) = (2021, 13, 32);
    assert!(BaseDate::from_ymd(year, month, day).is_err());

    let (year, month, day) = (MAX_YEAR + 1, 1, 32);
    assert!(BaseDate::from_ymd(year, month, day).is_err());

    let (year, month, day) = (MIN_YEAR - 1, 13, 31);
    assert!(BaseDate::from_ymd(year, month, day).is_err());

    let (year, month, day) = (MIN_YEAR - 1, 13, 32);
    assert!(BaseDate::from_ymd(year, month, day).is_err());
}

/*
 * @title  basedate_from_yo() 函数UT测试
 * @design 入参1: year
 *               有效值范围: year >= MIN_YEAR && year <= MAX_YEAR
 *               无效值范围: year < MIN_YEAR || year > MAX_YEAR
 *         入参2: ordinal
 *               有效值范围: ordinal != 0 || ordinal <= Sunflags::from_year(year).ndays()
 *               无效值范围: ordinal == 0 || ordinal > Sunflags::from_year(year).ndays()
 * @precon 无
 * @brief  描述测试用例执行
 *         1、year、ordinal 均为有效值
 *         2、year 为有效值，ordinal 为无效值
 *         3、year 为无效值，ordinal 为有效值
 *         4、year、ordinal 均为无效值
 * @expect 只有均为有效值的情况下会创建 `BaseDate` 结构，
 *         其他情况下，只会返回 `Err(_)`
 * @auto   true
 */
#[test]
fn ut_basedate_from_yo() {
    let (year, ordinal) = (2021, 1);
    assert!(BaseDate::from_yo(year, ordinal).is_ok());

    let (year, ordinal) = (2020, 367);
    assert!(BaseDate::from_yo(year, ordinal).is_err());

    let (year, ordinal) = (MIN_YEAR - 1, 1);
    assert!(BaseDate::from_yo(year, ordinal).is_err());

    let (year, ordinal) = (MIN_YEAR - 1, 367);
    assert!(BaseDate::from_yo(year, ordinal).is_err());
}

/*
 * @title  basedate_succ() 函数UT测试
 * @design 无入参
 * @precon 无
 * @brief  描述测试用例执行
 *         1、日期不在边界时
 *         2、日期在边界上时
 * @expect 只要不出现边界的情况，均会返回正确 `BaseDate` 结构，
 *         边界情况下，会返回 `Err(_)`
 * @auto   true
 */
#[test]
fn ut_basedate_succ() {
    let basedate = BaseDate::from_ymd(2021, 12, 31).unwrap();
    let change_basedate = BaseDate::from_ymd(2022, 1, 1).unwrap();
    assert_eq!(basedate.succ().unwrap(), change_basedate);

    let basedate = BaseDate::from_ymd(MAX_YEAR, 12, 31).unwrap();
    assert!(basedate.succ().is_err());
}

/*
 * @title  basedate_pred() 函数UT测试
 * @design 无入参
 * @precon 无
 * @brief  描述测试用例执行
 *         1、日期不在边界时
 *         2、日期在边界上时
 * @expect 只要不出现边界的情况，均会返回正确 `BaseDate` 结构，
 *         边界情况下，会返回 `Err(_)`
 * @auto   true
 */
#[test]
fn ut_basedate_pred() {
    let basedate = BaseDate::from_ymd(2021, 1, 1).unwrap();
    let change_basedate = BaseDate::from_ymd(2020, 12, 31).unwrap();
    assert_eq!(basedate.pred().unwrap(), change_basedate);

    let basedate = BaseDate::from_ymd(MIN_YEAR, 1, 1).unwrap();
    assert!(basedate.pred().is_err());
}

/*
 * @title  basedate_from_num_days_from_ce() 函数UT测试
 * @design 入参1: days
 *             有效值范围: days >= -95746495 && days <= 95745764
 *             无效值范围: days < -95746495 || days > 95745764
 * @precon 无
 * @brief  描述测试用例执行
 *         1、days 在有效值范围内时，将会创建正确的 `BaseDate` 结构
 *         2、days 在无效值范围内时，将会返回错误 `Err(_)`
 * @expect 只要在有效值范围内时，均会返回正确 `BaseDate` 结构，
 *         其他情况下，会返回 `Err(_)`
 * @auto   true
 */
#[test]
fn ut_basedate_from_num_days_from_ce() {
    let days = 0;
    let basedate = BaseDate::from_num_days_from_ce(days).unwrap();
    assert_eq!(basedate, BaseDate::from_ymd(0, 12, 31).unwrap());

    let days = -1;
    let basedate = BaseDate::from_num_days_from_ce(days).unwrap();
    assert_eq!(basedate, BaseDate::from_ymd(0, 12, 30).unwrap());

    let days = 1;
    let basedate = BaseDate::from_num_days_from_ce(days).unwrap();
    assert_eq!(basedate, BaseDate::from_ymd(1, 1, 1).unwrap());

    let days = -95746495;
    let basedate = BaseDate::from_num_days_from_ce(days).unwrap();
    assert_eq!(basedate, BaseDate::from_ymd(MIN_YEAR, 1, 1).unwrap());

    let days = 95745764;
    let basedate = BaseDate::from_num_days_from_ce(days).unwrap();
    assert_eq!(basedate, BaseDate::from_ymd(MAX_YEAR, 12, 31).unwrap());

    let days = -95746495 - 1;
    let basedate = BaseDate::from_num_days_from_ce(days);
    assert!(basedate.is_err());

    let days = 95745764 + 1;
    let basedate = BaseDate::from_num_days_from_ce(days);
    assert!(basedate.is_err());
}

/*
 * @title  basedate_checked_add_signed() 函数UT测试
 * @design 入参1: rhs，无无效值
 * @precon 无
 * @brief  描述测试用例执行
 *         1、本身的 `BaseDate` 所表示的日期不超过范围，增加一定时间后也不超过范围
 *         2、本身的 `BaseDate` 所表示的日期不超过范围，增加一定时间后超过范围
 * @expect 只要在有效值范围内时，均会返回正确 `BaseDate` 结构，
 *         其他情况下，会返回 `Err(_)`
 * @auto   true
 */
#[test]
fn ut_basedate_checked_add_signed() {
    let basedate = BaseDate::from_ymd(2021, 1, 1).unwrap();
    let change_basedate = BaseDate::from_ymd(2021, 2, 1).unwrap();
    assert_eq!(basedate + Duration::days(31).unwrap(), change_basedate);

    let basedate = BaseDate::from_ymd(2021, 12, 31).unwrap();
    let change_basedate = BaseDate::from_ymd(2022, 1, 1).unwrap();
    assert_eq!(basedate + Duration::days(1).unwrap(), change_basedate);

    let basedate = BaseDate::from_ymd(2020, 1, 1).unwrap();
    let change_basedate = BaseDate::from_ymd(2019, 12, 31).unwrap();
    assert_eq!(basedate + Duration::days(-1).unwrap(), change_basedate);

    let basedate = BaseDate::from_ymd(0, 12, 31).unwrap();
    let change_basedate = BaseDate::from_ymd(MAX_YEAR, 12, 31).unwrap();
    assert_eq!(basedate + Duration::max_duration(), change_basedate);

    let basedate = BaseDate::from_ymd(0, 12, 31).unwrap();
    let change_basedate = BaseDate::from_ymd(MIN_YEAR, 1, 1).unwrap();
    assert_eq!(basedate + Duration::min_duration(), change_basedate);
}

/*
 * @title  basedate_checked_sub_signed() 函数UT测试
 * @design 入参1: rhs，无无效值
 * @precon 无
 * @brief  描述测试用例执行
 *         1、本身的 `BaseDate` 所表示的日期不超过范围，减少一定时间后也不超过范围
 *         2、本身的 `BaseDate` 所表示的日期不超过范围，减少一定时间后超过范围
 * @expect 只要在有效值范围内时，均会返回正确 `BaseDate` 结构，
 *         其他情况下，会返回 `Err(_)`
 * @auto   true
 */
#[test]
fn ut_basedate_checked_sub_signed() {
    let basedate = BaseDate::from_ymd(2021, 1, 1).unwrap();
    let change_basedate = BaseDate::from_ymd(2021, 2, 1).unwrap();
    assert_eq!(basedate - Duration::days(-31).unwrap(), change_basedate);

    let basedate = BaseDate::from_ymd(2021, 12, 31).unwrap();
    let change_basedate = BaseDate::from_ymd(2022, 1, 1).unwrap();
    assert_eq!(basedate - Duration::days(-1).unwrap(), change_basedate);

    let basedate = BaseDate::from_ymd(2020, 1, 1).unwrap();
    let change_basedate = BaseDate::from_ymd(2019, 12, 31).unwrap();
    assert_eq!(basedate - Duration::days(1).unwrap(), change_basedate);
}

/*
 * @title  basedate_signed_duration_since() 函数UT测试
 * @design 入参1: rhs，无无效值
 * @precon 无
 * @brief  描述测试用例执行
 *         1、本身的 `BaseDate` 所表示的日期不超过范围，
 *         减少指定的 `BaseDate` 也不超过范围，多次测试不同数据。
 * @expect 只要在有效值范围内时，均会返回正确 `Duration` 结构，
 *         其他情况下，会返回 `Err(_)`
 * @auto   true
 */
#[test]
fn ut_basedate_signed_duration_since() {
    let basedate_one = BaseDate::from_ymd(2021, 12, 31).unwrap();
    let basedate_two = BaseDate::from_ymd(2021, 12, 30).unwrap();
    assert_eq!(basedate_one - basedate_two, Duration::days(1).unwrap());

    let basedate_one = BaseDate::from_ymd(2021, 12, 30).unwrap();
    let basedate_two = BaseDate::from_ymd(2021, 12, 31).unwrap();
    assert_eq!(basedate_one - basedate_two, Duration::days(-1).unwrap());

    let basedate_one = BaseDate::from_ymd(2021, 12, 31).unwrap();
    let basedate_two = BaseDate::from_ymd(2021, 12, 31).unwrap();
    assert_eq!(basedate_one - basedate_two, Duration::days(0).unwrap());
}
