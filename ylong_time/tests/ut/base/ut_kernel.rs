use crate::base::kernel::{
    days_to_yo, yo_to_days, Mdf, Of, SundayLetter, A, AG, B, BA, C, CB, D, DC, E, ED, F, FE, G, GF,
    MAX_DAYS_FROM_YEAR_0, MAX_YEAR, MIN_DAYS_FROM_YEAR_0, MIN_YEAR,
};

/*
 * @title  days_to_yo() 函数UT测试
 * @design 入参1: days
 *               有效值范围: days >= MIN_DAYS_FROM_YEAR_0 && days <= MAX_DAYS_FROM_YEAR_0
 *               无效值范围: days < MIN_DAYS_FROM_YEAR_0 || days > MAX_DAYS_FROM_YEAR_0
 * @precon 无
 * @brief  描述测试用例执行
 *         1、days 设置为 0，检验返回值是否正确，公元前一年为第零年
 *         2、days 设置为 1，检验返回值是否正确
 *         3、days 设置为 -365，检验返回值是否正确
 *         4、days 设置为 MIN_DAYS_FROM_YEAR_0，检验返回值是否正确
 *         5、days 设置为 MAX_DAYS_FROM_YEAR_0，检验返回值是否正确
 *         6、days 设置为无效值部分，检验返回值是否正确
 * @expect 修改不同 days 所得到的年份以及当前年份第几天属性
 *         在有效值范围内将为 `Ok(_)`
 *         无效值范围内将为 `Err(_)`
 * @auto   true
 */
#[test]
fn ut_days_to_yo() {
    let days = 0;
    assert_eq!(days_to_yo(days).unwrap(), (0, 366));

    let days = 1;
    assert_eq!(days_to_yo(days).unwrap(), (1, 1));

    let days = -365;
    assert_eq!(days_to_yo(days).unwrap(), (0, 1));

    let days = MIN_DAYS_FROM_YEAR_0;
    assert_eq!(days_to_yo(days).unwrap(), (MIN_YEAR, 1));

    let days = MAX_DAYS_FROM_YEAR_0;
    assert_eq!(days_to_yo(days).unwrap(), (MAX_YEAR, 365));

    let days = -95746496;
    assert!(days_to_yo(days).is_err());

    let days = 95745765;
    assert!(days_to_yo(days).is_err());
}

/*
 * @title  yo_to_days() 函数UT测试
 * @design 入参1: year
 *               有效值范围: year >= MIN_YEAR && year <= MAX_YEAR
 *               无效值范围: year < MIN_YEAR || year > MAX_YEAR
 *         入参2: ordinal
 *               有效值范围: ordinal >= 1 || ordinal <= SundayLetter::from_year(year).ndays()
 *               无效值范围: ordinal < 1 || ordinal > SundayLetter::from_year(year).ndays()
 * @precon 无
 * @brief  描述测试用例执行
 *         1、将 (year, ordinal) 设置为 (0, 366)，检验返回值是否正确
 *         2、将 (year, ordinal) 设置为 (1, 1)，检验返回值是否正确
 *         3、将 (year, ordinal) 设置为 (0, 1), 检验返回值是否正确
 *         4、将 (year, ordinal) 设置为 (MIN_YEAR, 1)，检验返回值是否正确
 *         5、将 (year, ordinal) 设置为 (MAX_YEAR, 365)，检验返回值是否正确
 *         6、将 (year, ordinal) 设置为无效值返回内，检验返回值是否正确
 * @expect 修改不同 (year, ordinal) 所得到的年份以及当前年份第几天属性
 *         在有效值范围内将为 `Ok(_)`
 *         无效值范围内将为 `Err(_)`
 * @auto   true
 */
#[test]
fn ut_yo_to_days() {
    let days = 0;
    assert_eq!(yo_to_days(0, 366).unwrap(), days);

    let days = 1;
    assert_eq!(yo_to_days(1, 1).unwrap(), days);

    let days = -365;
    assert_eq!(yo_to_days(0, 1).unwrap(), days);

    let days = MIN_DAYS_FROM_YEAR_0;
    assert_eq!(yo_to_days(MIN_YEAR, 1).unwrap(), days);

    let days = MAX_DAYS_FROM_YEAR_0;
    assert_eq!(yo_to_days(MAX_YEAR, 365).unwrap(), days);

    assert!(yo_to_days(MIN_YEAR - 1, 1).is_err());

    assert!(yo_to_days(MAX_YEAR + 1, 365).is_err());
}

/*
 * @title  sunday_letter_from_year() 函数UT测试
 * @design 入参1: year，只要在 i32 范围之中，无无效值
 * @precon 无
 * @brief  描述测试用例执行
 *         1、测试从 2001 年至 2021 年的主日字母是否正确
 * @expect 所得主日字母与历法之中的一一对应
 * @auto   true
 */
#[test]
fn ut_sunday_letter_from_year() {
    assert_eq!(SundayLetter::from_year(2001), G);
    assert_eq!(SundayLetter::from_year(2002), F);
    assert_eq!(SundayLetter::from_year(2003), E);
    assert_eq!(SundayLetter::from_year(2004), DC);
    assert_eq!(SundayLetter::from_year(2005), B);
    assert_eq!(SundayLetter::from_year(2006), A);
    assert_eq!(SundayLetter::from_year(2007), G);
    assert_eq!(SundayLetter::from_year(2008), FE);
    assert_eq!(SundayLetter::from_year(2009), D);
    assert_eq!(SundayLetter::from_year(2010), C);
    assert_eq!(SundayLetter::from_year(2011), B);
    assert_eq!(SundayLetter::from_year(2012), AG);
    assert_eq!(SundayLetter::from_year(2013), F);
    assert_eq!(SundayLetter::from_year(2014), E);
    assert_eq!(SundayLetter::from_year(2015), D);
    assert_eq!(SundayLetter::from_year(2016), CB);
    assert_eq!(SundayLetter::from_year(2017), A);
    assert_eq!(SundayLetter::from_year(2018), G);
    assert_eq!(SundayLetter::from_year(2019), F);
    assert_eq!(SundayLetter::from_year(2020), ED);
    assert_eq!(SundayLetter::from_year(2021), C);
}

/*
 * @title  sunday_letter_ndays() 函数UT测试
 * @design 无入参
 * @precon 创建好某个主日字母结构
 * @brief  描述测试用例执行
 *         1、拥有不同主日字母的年份，该年份的总天数（是否是闰年）
 * @expect 所得年份总天数正确
 * @auto   true
 */
#[test]
fn ut_sunday_letter_ndays() {
    assert_eq!(SundayLetter::from_year(2014).ndays(), 365);
    assert_eq!(SundayLetter::from_year(2012).ndays(), 366);
    assert_eq!(SundayLetter::from_year(2000).ndays(), 366);
    assert_eq!(SundayLetter::from_year(1900).ndays(), 365);
    assert_eq!(SundayLetter::from_year(1600).ndays(), 366);
    assert_eq!(SundayLetter::from_year(1).ndays(), 365);
    assert_eq!(SundayLetter::from_year(0).ndays(), 366); // 1 BCE
    assert_eq!(SundayLetter::from_year(-1).ndays(), 365); // 2 BCE
    assert_eq!(SundayLetter::from_year(-4).ndays(), 366); // 5 BCE
    assert_eq!(SundayLetter::from_year(-99).ndays(), 365); // 100 BCE
    assert_eq!(SundayLetter::from_year(-100).ndays(), 365); // 101 BCE
    assert_eq!(SundayLetter::from_year(-399).ndays(), 365); // 400 BCE
    assert_eq!(SundayLetter::from_year(-400).ndays(), 366); // 401 BCE
}

/*
 * @title  sunday_letter_isoweeknums() 函数UT测试
 * @design 无入参
 * @precon 创建好某个主日字母结构
 * @brief  描述测试用例执行
 *         1、拥有不同主日字母的年份，该年份的总周数
 * @expect 所得年份总周数正确
 * @auto   true
 */
#[test]
fn ut_sunday_letter_isoweeknums() {
    assert_eq!(A.isoweeknums(), 52);
    assert_eq!(AG.isoweeknums(), 52);
    assert_eq!(B.isoweeknums(), 52);
    assert_eq!(BA.isoweeknums(), 52);
    assert_eq!(C.isoweeknums(), 52);
    assert_eq!(CB.isoweeknums(), 52);
    assert_eq!(D.isoweeknums(), 53);
    assert_eq!(DC.isoweeknums(), 53);
    assert_eq!(E.isoweeknums(), 52);
    assert_eq!(ED.isoweeknums(), 53);
    assert_eq!(F.isoweeknums(), 52);
    assert_eq!(FE.isoweeknums(), 52);
    assert_eq!(G.isoweeknums(), 52);
    assert_eq!(GF.isoweeknums(), 52);
}

/*
 * @title  of_new() 函数UT测试
 * @design 入参1: ordinal
 *               有效值范围: ordinal != 0 && ordinal <= flags.ndays()
 *               无效值范围: ordinal == 0 || ordinal > flags.ndays()
 *         入参2: flags, 无无效值范围
 * @precon 无
 * @brief  描述测试用例执行
 *         1、ordinal 设置为 365，检验返回值是否正确，主日字母为非闰年主日字母
 *         2、ordinal 设置为 366，检验返回值是否正确，主日字母为非闰年主日字母
 *         3、ordinal 设置为 366，检验返回值是否正确，主日字母为闰年主日字母
 *         4、ordinal 设置为 367，检验返回值是否正确，主日字母为闰年主日字母
 * @expect 修改不同 days 所得到的年份以及当前年份第几天属性
 *         在有效值范围内将为 `Ok(_)`
 *         无效值范围内将为 `Err(_)`
 * @auto   true
 */
#[test]
fn ut_of_new() {
    let of = Of::new(365, A);
    assert!(of.is_ok());

    let of = Of::new(366, A);
    assert!(of.is_err());

    let of = Of::new(366, AG);
    assert!(of.is_ok());

    let of = Of::new(367, AG);
    assert!(of.is_err());
}

/*
 * @title  of_valid() 函数UT测试
 * @design 无入参
 * @precon 通过不同的 ordinal 创建不同的 `Of` 结构
 * @brief  描述测试用例执行
 *         1、ordinal 设置为 366 的 `Of` 结构，校验其本身及下一天的有效性
 *         2、ordinal 设置为 1 的 `Of` 结构，校验其本身及上一天的有效性
 * @expect 直接创建的 `Of` 结构将是有效的，该用例通过调用 `succ`/`pred` 函数创建的 `Of` 是无效的
 * @auto   true
 */
#[test]
fn ut_of_vaild() {
    let of = Of::new(366, AG).unwrap();
    assert!(of.valid());
    assert!(!of.succ().valid());

    let of = Of::new(1, A).unwrap();
    assert!(of.valid());
    assert!(!of.pred().valid());
}

/*
 * @title  of_ordinal() 函数UT测试
 * @design 无入参
 * @precon 通过不同的 ordinal 创建不同的 `Of` 结构
 * @brief  描述测试用例执行
 *         1、ordinal 设置为 366 的 `Of` 结构，检验其返回值是否正确
 *         2、ordinal 设置为 1 的 `Of` 结构，校验其返回值是否正确
 * @expect 将会返回正确的 ordinal 值
 * @auto   true
 */
#[test]
fn ut_of_ordinal() {
    let of = Of::new(1, A).unwrap();
    assert_eq!(of.ordinal(), 1);

    let of = Of::new(366, AG).unwrap();
    assert_eq!(of.ordinal(), 366);
}

/*
 * @title  of_with_ordinal() 函数UT测试
 * @design 入参1: ordinal
 *               有效值范围: ordinal <= self.flags().ndays()
 *               无效值范围: ordinal > self.flags().ndays()
 * @precon 无
 * @brief  描述测试用例执行
 *         1、主日字母非闰年情况下，将 ordinal = 365 的情况替换为 366
 *         2、主日字母非闰年情况下，将 ordinal = 365 的情况替换为 1
 *         3、主日字母为闰年情况下，将 ordinal = 365 的情况替换为 366
 *         4、主日字母为闰年情况下，将 ordinal = 365 的情况替换为 1
 * @expect 无效的 ordinal 将产生 `Err(err)`
 * @auto   true
 */
#[test]
fn ut_of_with_ordinal() {
    let of = Of::new(365, A).unwrap();
    assert_eq!(of.with_ordinal(1).unwrap(), Of::new(1, A).unwrap());
    assert!(of.with_ordinal(366).is_err());

    let of = Of::new(365, AG).unwrap();
    assert_eq!(of.with_ordinal(366).unwrap(), Of::new(366, AG).unwrap());
    assert_eq!(of.with_ordinal(1).unwrap(), Of::new(1, AG).unwrap());
}

/*
 * @title  of_flags() 函数UT测试
 * @design 无入参
 * @precon 无
 * @brief  描述测试用例执行
 *         1、主日字母非闰年情况下
 *         2、主日字母为闰年情况下
 * @expect 创建时输入的主日字母，与该函数结果一一对应
 * @auto   true
 */
#[test]
fn ut_of_flags() {
    let of = Of::new(1, AG).unwrap();
    assert_eq!(of.flags(), AG);

    let of = Of::new(1, A).unwrap();
    assert_eq!(of.flags(), A);
}

/*
 * @title  of_with_flags() 函数UT测试
 * @design 入参1: flags:
 *               有效值范围: self.ordinal() <= flags.ndays()
 *               无效值范围: self.ordinal() > flags.ndays()
 * @precon 无
 * @brief  描述测试用例执行
 *         1、ordinal 设置为 365，主日字母非闰年主日字母，将主日字母替换为闰年主日字母
 *         2、ordinal 设置为 366，主日字母为闰年主日字母，将主日字母替换非闰年主日字母
 * @expect 可以将非闰年替换为闰年，但闰年替换为非闰年在某些日期会报错
 *         在有效值范围内将为 `Ok(_)`
 *         无效值范围内将为 `Err(_)`
 * @auto   true
 */
#[test]
fn ut_of_with_flags() {
    let of = Of::new(1, A).unwrap();
    assert_eq!(of.with_flags(AG).unwrap(), Of::new(1, AG).unwrap());

    let of = Of::new(366, AG).unwrap();
    assert!(of.with_flags(A).is_err());
}

/*
 * @title  of_succ_pred() 函数UT测试
 * @design 无入参
 * @precon 无
 * @brief  描述测试用例执行
 *         1、ordinal 设置为 365，主日字母非闰年主日字母，获取下一天
 *         2、ordinal 设置为 366，主日字母为闰年主日字母，获取下一天
 *         3、ordinal 设置为 1，获取前一天
 *         4、ordinal 设置为 365，主日字母为闰年主日字母，获取前一天或下一天
 * @expect 此函数不会出现报错，但会产生无效 `Of` 值
 * @auto   true
 */
#[test]
fn ut_of_succ_pred() {
    let of = Of::new(365, A).unwrap();
    assert!(!of.succ().valid());

    let of = Of::new(366, AG).unwrap();
    assert!(!of.succ().valid());

    let of = Of::new(1, A).unwrap();
    assert!(!of.pred().valid());

    let of = Of::new(365, AG).unwrap();
    assert_eq!(of.succ(), Of::new(366, AG).unwrap());
    assert_eq!(of.pred(), Of::new(364, AG).unwrap());
}

/*
 * @title  of_from_mdf() 函数UT测试
 * @design 入参1: mdf，无无效值
 * @precon 无
 * @brief  描述测试用例执行
 *         1、1.1 对应的 `Of` 结构为第 1 天
 *         2、12.31，且非闰年的情况，对应的 `Of` 结构为第 365 天
 *         3、12.31，且为闰年的情况，对应的 `Of` 结构为第 366 天
 *         4、1.1 的前一天，对应的 `Of` 结构存在但无效
 *         5、12.31 的后一天，对应的 `Of` 结构存在但无效
 * @expect 所产生的结果与测试用例描述对应
 * @auto   true
 */
#[test]
fn ut_of_from_mdf() {
    let mdf = Mdf::new(1, 1, A).unwrap();
    assert_eq!(Of::from_mdf(mdf), Of::new(1, A).unwrap());

    let mdf = Mdf::new(12, 31, A).unwrap();
    assert_eq!(Of::from_mdf(mdf), Of::new(365, A).unwrap());

    let mdf = Mdf::new(12, 31, AG).unwrap();
    assert_eq!(Of::from_mdf(mdf), Of::new(366, AG).unwrap());

    let of = Of::new(1, A).unwrap().pred();
    let mdf = Mdf::from_of(of);
    let of = Of::from_mdf(mdf);
    assert!(!of.valid());

    let of = Of::new(365, A).unwrap().succ();
    let mdf = Mdf::from_of(of);
    let of = Of::from_mdf(mdf);
    assert!(!of.valid());
}

/*
 * @title  of_as_mdf() 函数UT测试
 * @design 无入参
 * @precon 无
 * @brief  描述测试用例执行
 *         1、`Of` 结构为第 1 天对应的 1.1
 *         2、`Of` 结构为第 365 天，且非闰年的情况，对应的 12.31
 *         3、`Of` 结构为第 366 天，且为闰年的情况，对应的 12.31
 *         4、1.1 的前一天，对应的 `Of` 结构存在但无效
 *         5、12.31 的后一天，对应的 `Of` 结构存在但无效
 * @expect 所产生的结果与测试用例描述对应
 * @auto   true
 */
#[test]
fn ut_of_as_mdf() {
    let of = Of::new(1, A).unwrap();
    let mdf = Mdf::new(1, 1, A).unwrap();
    assert_eq!(of.as_mdf(), mdf);

    let of = Of::new(365, A).unwrap();
    let mdf = Mdf::new(12, 31, A).unwrap();
    assert_eq!(of.as_mdf(), mdf);

    let of = Of::new(366, AG).unwrap();
    let mdf = Mdf::new(12, 31, AG).unwrap();
    assert_eq!(of.as_mdf(), mdf);

    let of = Of::new(1, A).unwrap().pred();
    assert!(!of.as_mdf().valid());

    let of = Of::new(365, A).unwrap().succ();
    assert!(!of.as_mdf().valid());
}

/*
 * @title  mdf_new() 函数UT测试
 * @design 入参1: month
 *               有效值范围: month != 0 && month <= 12
 *               无效值范围: month == 0 || month > 12
 *         入参2: day
 *               有效值范围: day != 0 && day <= 31
 *               无效值范围: day == 0 || day > 31
 *         入参3: SundayLetter(flags)，无无效值范围
 * @precon 无
 * @brief  描述测试用例执行
 *         1、1.1 创建的 `Mdf` 结构，成功
 *         2、12.31 创建的 `Mdf` 结构，成功
 *         3、0.1 创建的 `Mdf` 结构，失败
 *         4、1.32 创建的 `Mdf` 结构，失败
 * @expect 所产生的结果与测试用例描述对应
 * @auto   true
 */
#[test]
fn ut_mdf_new() {
    let mdf = Mdf::new(1, 1, A);
    assert!(mdf.is_ok());

    let mdf = Mdf::new(12, 31, A);
    assert!(mdf.is_ok());

    let mdf = Mdf::new(0, 31, A);
    assert!(mdf.is_err());

    let mdf = Mdf::new(1, 32, A);
    assert!(mdf.is_err());
}

/*
 * @title  mdf_valid() 函数UT测试
 * @design 无入参
 * @precon 无
 * @brief  描述测试用例执行
 *         1、1.1 创建的 `Mdf` 结构，为有效值
 *         2、12.31 创建的 `Mdf` 结构，为有效值
 *         3、2.30 创建的 `Mdf` 结构，为无效值
 *         4、1.32 创建的 `Mdf` 结构，为无效值
 * @expect 所产生的结果与测试用例描述对应
 * @auto   true
 */
#[test]
fn ut_mdf_valid() {
    let mdf = Mdf::new(1, 1, A).unwrap();
    assert!(mdf.valid());

    let mdf = Mdf::new(12, 31, A).unwrap();
    assert!(mdf.valid());

    let mdf = Mdf::new(2, 30, A).unwrap();
    assert!(!mdf.valid());

    let mdf = Mdf::new(4, 31, A).unwrap();
    assert!(!mdf.valid());
}

/*
 * @title  mdf_month() 函数UT测试
 * @design 无入参
 * @precon 无
 * @brief  描述测试用例执行
 *         1、1.1 创建的 `Mdf` 结构，返回 1 月
 *         2、12.31 创建的 `Mdf` 结构，返回 12 月
 * @expect 所产生的结果与测试用例描述对应
 * @auto   true
 */
#[test]
fn ut_mdf_month() {
    let mdf = Mdf::new(1, 1, A).unwrap();
    assert_eq!(mdf.month(), 1);

    let mdf = Mdf::new(12, 31, A).unwrap();
    assert_eq!(mdf.month(), 12);
}

/*
 * @title  mdf_with_month() 函数UT测试
 * @design 入参1: month
 *               有效值范围: month != 0 && month <= 12
 *               无效值范围: month == 0 || month > 12
 * @precon 无
 * @brief  描述测试用例执行
 *         1、月份为有效值，替换成功
 *         2、月份为无效值，替换失败
 * @expect 所产生的结果与测试用例描述对应
 * @auto   true
 */
#[test]
fn ut_mdf_with_month() {
    let mdf = Mdf::new(1, 1, A).unwrap();

    assert_eq!(mdf.with_month(12).unwrap(), Mdf::new(12, 1, A).unwrap());
    assert!(mdf.with_month(13).is_err());
}

/*
 * @title  mdf_day() 函数UT测试
 * @design 无入参
 * @precon 无
 * @brief  描述测试用例执行
 *         1、1.1 创建的 `Mdf` 结构，返回 1 天
 *         2、12.31 创建的 `Mdf` 结构，返回 31 天
 * @expect 所产生的结果与测试用例描述对应
 * @auto   true
 */
#[test]
fn ut_mdf_day() {
    let mdf = Mdf::new(1, 1, A).unwrap();
    assert_eq!(mdf.day(), 1);

    let mdf = Mdf::new(12, 31, A).unwrap();
    assert_eq!(mdf.day(), 31);
}

/*
 * @title  mdf_with_day() 函数UT测试
 * @design 入参1: day
 *               有效值范围: day != 0 && day <= 31
 *               无效值范围: day == 0 || day > 31
 * @precon 无
 * @brief  描述测试用例执行
 *         1、天数为有效值，替换成功
 *         2、天数为无效值，替换失败
 * @expect 所产生的结果与测试用例描述对应
 * @auto   true
 */
#[test]
fn ut_mdf_with_day() {
    let mdf = Mdf::new(1, 1, A).unwrap();

    assert_eq!(mdf.with_day(31).unwrap(), Mdf::new(1, 31, A).unwrap());
    assert!(mdf.with_day(32).is_err());
}

/*
 * @title  mdf_flags() 函数UT测试
 * @design 无入参
 * @precon 无
 * @brief  描述测试用例执行
 *         1、该函数返回的主日字母与创建的相同
 * @expect 所产生的结果与测试用例描述对应
 * @auto   true
 */
#[test]
fn ut_mdf_flags() {
    let mdf = Mdf::new(1, 1, A).unwrap();
    assert_eq!(mdf.flags(), A);

    let mdf = Mdf::new(1, 1, AG).unwrap();
    assert_eq!(mdf.flags(), AG);
}

/*
 * @title  mdf_with_flags() 函数UT测试
 * @design 入参1: flags，无无效值
 * @precon 无
 * @brief  描述测试用例执行
 *         1、该函数将返回替换主日字母后的 `Mdf` 结构
 * @expect 所产生的结果与测试用例描述对应
 * @auto   true
 */
#[test]
fn ut_mdf_with_flags() {
    let mdf = Mdf::new(1, 1, A).unwrap();
    assert_eq!(mdf.with_flags(AG), Mdf::new(1, 1, AG).unwrap());

    let mdf = Mdf::new(1, 1, AG).unwrap();
    assert_eq!(mdf.with_flags(A), Mdf::new(1, 1, A).unwrap());
}

/*
 * @title  mdf_from_of() 函数UT测试
 * @design 入参1: of
 *               有效值范围: 正常创建均为有效值
 *               无效值范围: 在临界部分时，调用 pred/succ 函数时会产生无效值
 * @precon 无
 * @brief  描述测试用例执行
 *         1、有效值范围内创建出有效的 `Mdf` 结构
 *         2、无效值范围内创建出无效的 `Mdf` 结构
 * @expect 所产生的结果与测试用例描述对应
 * @auto   true
 */
#[test]
fn ut_mdf_from_of() {
    let of = Of::new(1, A).unwrap();
    assert_eq!(Mdf::from_of(of), Mdf::new(1, 1, A).unwrap());

    let of = Of::new(365, A).unwrap();
    assert_eq!(Mdf::from_of(of), Mdf::new(12, 31, A).unwrap());

    let of = Of::new(366, AG).unwrap();
    assert_eq!(Mdf::from_of(of), Mdf::new(12, 31, AG).unwrap());

    let of = Of::new(365, A).unwrap().succ();
    assert!(!Mdf::from_of(of).valid());

    let of = Of::new(1, A).unwrap().pred();
    assert!(!Mdf::from_of(of).valid());
}

/*
 * @title  mdf_as_of() 函数UT测试
 * @design 无入参
 * @precon 无
 * @brief  描述测试用例执行
 *         1、`Mdf` 将会被转换成 `Of` 结构
 * @expect 所产生的结果与测试用例描述对应
 * @auto   true
 */
#[test]
fn ut_mdf_as_of() {
    let of = Of::new(1, A).unwrap();
    let mdf = Mdf::new(1, 1, A).unwrap();
    assert_eq!(mdf.as_of(), of);

    let of = Of::new(365, A).unwrap();
    let mdf = Mdf::new(12, 31, A).unwrap();
    assert_eq!(mdf.as_of(), of);

    let of = Of::new(366, AG).unwrap();
    let mdf = Mdf::new(12, 31, AG).unwrap();
    assert_eq!(mdf.as_of(), of);
}
