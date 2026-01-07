//! 日历和顺序日期的核心实现

use crate::error::{BaseErrorKind, TimeError};
use crate::Weekday;

/// 日期最大天数
const MAX_DAYS_FROM_YEAR_0: i32 = 95745764;
/// 日期最小天数
const MIN_DAYS_FROM_YEAR_0: i32 = -95746495;
/// 日期最大年份
pub(crate) const MAX_YEAR: i32 = i32::MAX >> 13;
/// 日期最小年份
pub(crate) const MIN_YEAR: i32 = i32::MIN >> 13;

/// 主日字母
///
/// 在格里历之中，对年份来说，拥有 14 种类型，但无论是平年还是闰年，都是从周一至周日。
/// `SundayLetter` 将会把信息存储在 4 位二进制数之中，格式为 `abbb`。
/// `a` 在等于 1 的情况下代表当前年份为平年，否则为闰年。
/// `bbb` 代表这一年的最后一天代表星期几（星期一对应的数字为 1，其它同理直至 7）。
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq)]
pub struct SundayLetter(u8);

// 平年，该年最后一天是周五
pub const A: SundayLetter = SundayLetter(0b1101);
// 平年，该年最后一天是周四
pub const B: SundayLetter = SundayLetter(0b1100);
// 平年，该年最后一天是周三
pub const C: SundayLetter = SundayLetter(0b1011);
// 平年，该年最后一天是周二
pub const D: SundayLetter = SundayLetter(0b1010);
// 平年，该年最后一天是周一
pub const E: SundayLetter = SundayLetter(0b1001);
// 平年，该年最后一天是周日
pub const F: SundayLetter = SundayLetter(0b1111);
// 平年，该年最后一天是周六
pub const G: SundayLetter = SundayLetter(0b1110);
// 闰年，该年最后一天是周五
pub const AG: SundayLetter = SundayLetter(0b0101);
// 闰年，该年最后一天是周四
pub const BA: SundayLetter = SundayLetter(0b0100);
// 闰年，该年最后一天是周三
pub const CB: SundayLetter = SundayLetter(0b0011);
// 闰年，该年最后一天是周二
pub const DC: SundayLetter = SundayLetter(0b0010);
// 闰年，该年最后一天是周一
pub const ED: SundayLetter = SundayLetter(0b0001);
// 闰年，该年最后一天是周日
pub const FE: SundayLetter = SundayLetter(0b0111);
// 闰年，该年最后一天是周六
pub const GF: SundayLetter = SundayLetter(0b0110);

/// 一个周期四百年内所经过的天数
const DAYS_OF_FOUR_HUNDRED: i32 = 146_097;

// 每一年对应的主日字母，四百年为一个周期
static YEAR_TO_SUN_FLAGS: [SundayLetter; 400] = [
    BA, G, F, E, DC, B, A, G, FE, D, C, B, AG, F, E, D, CB, A, G, F, ED, C, B, A, GF, E, D, C, BA,
    G, F, E, DC, B, A, G, FE, D, C, B, AG, F, E, D, CB, A, G, F, ED, C, B, A, GF, E, D, C, BA, G,
    F, E, DC, B, A, G, FE, D, C, B, AG, F, E, D, CB, A, G, F, ED, C, B, A, GF, E, D, C, BA, G, F,
    E, DC, B, A, G, FE, D, C, B, AG, F, E, D, // 100
    C, B, A, G, FE, D, C, B, AG, F, E, D, CB, A, G, F, ED, C, B, A, GF, E, D, C, BA, G, F, E, DC,
    B, A, G, FE, D, C, B, AG, F, E, D, CB, A, G, F, ED, C, B, A, GF, E, D, C, BA, G, F, E, DC, B,
    A, G, FE, D, C, B, AG, F, E, D, CB, A, G, F, ED, C, B, A, GF, E, D, C, BA, G, F, E, DC, B, A,
    G, FE, D, C, B, AG, F, E, D, CB, A, G, F, // 200
    E, D, C, B, AG, F, E, D, CB, A, G, F, ED, C, B, A, GF, E, D, C, BA, G, F, E, DC, B, A, G, FE,
    D, C, B, AG, F, E, D, CB, A, G, F, ED, C, B, A, GF, E, D, C, BA, G, F, E, DC, B, A, G, FE, D,
    C, B, AG, F, E, D, CB, A, G, F, ED, C, B, A, GF, E, D, C, BA, G, F, E, DC, B, A, G, FE, D, C,
    B, AG, F, E, D, CB, A, G, F, ED, C, B, A, // 300
    G, F, E, D, CB, A, G, F, ED, C, B, A, GF, E, D, C, BA, G, F, E, DC, B, A, G, FE, D, C, B, AG,
    F, E, D, CB, A, G, F, ED, C, B, A, GF, E, D, C, BA, G, F, E, DC, B, A, G, FE, D, C, B, AG, F,
    E, D, CB, A, G, F, ED, C, B, A, GF, E, D, C, BA, G, F, E, DC, B, A, G, FE, D, C, B, AG, F, E,
    D, CB, A, G, F, ED, C, B, A, GF, E, D, C, // 400
];

// 公元前一年为 0 年，由于闰年一年的天数变化，从而需要对一年的天数进行调整
static YEAR_DELTAS: [u8; 401] = [
    0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7, 8, 8, 8,
    8, 9, 9, 9, 9, 10, 10, 10, 10, 11, 11, 11, 11, 12, 12, 12, 12, 13, 13, 13, 13, 14, 14, 14, 14,
    15, 15, 15, 15, 16, 16, 16, 16, 17, 17, 17, 17, 18, 18, 18, 18, 19, 19, 19, 19, 20, 20, 20, 20,
    21, 21, 21, 21, 22, 22, 22, 22, 23, 23, 23, 23, 24, 24, 24, 24, 25, 25, 25, // 100
    25, 25, 25, 25, 25, 26, 26, 26, 26, 27, 27, 27, 27, 28, 28, 28, 28, 29, 29, 29, 29, 30, 30, 30,
    30, 31, 31, 31, 31, 32, 32, 32, 32, 33, 33, 33, 33, 34, 34, 34, 34, 35, 35, 35, 35, 36, 36, 36,
    36, 37, 37, 37, 37, 38, 38, 38, 38, 39, 39, 39, 39, 40, 40, 40, 40, 41, 41, 41, 41, 42, 42, 42,
    42, 43, 43, 43, 43, 44, 44, 44, 44, 45, 45, 45, 45, 46, 46, 46, 46, 47, 47, 47, 47, 48, 48, 48,
    48, 49, 49, 49, // 200
    49, 49, 49, 49, 49, 50, 50, 50, 50, 51, 51, 51, 51, 52, 52, 52, 52, 53, 53, 53, 53, 54, 54, 54,
    54, 55, 55, 55, 55, 56, 56, 56, 56, 57, 57, 57, 57, 58, 58, 58, 58, 59, 59, 59, 59, 60, 60, 60,
    60, 61, 61, 61, 61, 62, 62, 62, 62, 63, 63, 63, 63, 64, 64, 64, 64, 65, 65, 65, 65, 66, 66, 66,
    66, 67, 67, 67, 67, 68, 68, 68, 68, 69, 69, 69, 69, 70, 70, 70, 70, 71, 71, 71, 71, 72, 72, 72,
    72, 73, 73, 73, // 300
    73, 73, 73, 73, 73, 74, 74, 74, 74, 75, 75, 75, 75, 76, 76, 76, 76, 77, 77, 77, 77, 78, 78, 78,
    78, 79, 79, 79, 79, 80, 80, 80, 80, 81, 81, 81, 81, 82, 82, 82, 82, 83, 83, 83, 83, 84, 84, 84,
    84, 85, 85, 85, 85, 86, 86, 86, 86, 87, 87, 87, 87, 88, 88, 88, 88, 89, 89, 89, 89, 90, 90, 90,
    90, 91, 91, 91, 91, 92, 92, 92, 92, 93, 93, 93, 93, 94, 94, 94, 94, 95, 95, 95, 95, 96, 96, 96,
    96, 97, 97, 97, 97, // 400+1
];

/// 将已经存在的天数转换为年份及天数（当前年份的第几天）
///
/// # Examples
///
/// ```
/// use ylong_time::base::kernel::days_to_yo;
///
/// let day_zero = 0;
/// let day_one = 1;
///
/// assert_eq!(days_to_yo(day_zero).unwrap(), (0, 366));
/// assert_eq!(days_to_yo(day_one).unwrap(), (1, 1));
/// ```
pub fn days_to_yo(days: i32) -> Result<(i32, u32), TimeError> {
    if !(MIN_DAYS_FROM_YEAR_0..=MAX_DAYS_FROM_YEAR_0).contains(&days) {
        Err(TimeError::from(BaseErrorKind::InvalidDay))
    } else {
        // 用于保证 `12.31 1 BCE` 对应于第 0 天
        let mut days = days + 365;
        if days < 0 {
            days -= 365;
        }
        let year_div_400 = days.abs() / DAYS_OF_FOUR_HUNDRED * 400;
        let mut year_mod_400 = days.abs() % DAYS_OF_FOUR_HUNDRED / 365;
        let mut ordinal = (days.abs() % DAYS_OF_FOUR_HUNDRED % 365) as u32;
        // 因为计算时天数是按照 `365` 天进行计算的，闰年的天数不同需要进行修正，计算修正天数
        let delta = u32::from(YEAR_DELTAS[year_mod_400 as usize]);
        // 天数不足，倒退一年，否则正常减去闰年多的天数
        if ordinal < delta {
            year_mod_400 -= 1;
            ordinal += 365 - u32::from(YEAR_DELTAS[year_mod_400 as usize]);
        } else {
            ordinal -= delta;
        }

        let year = year_div_400 + year_mod_400;
        if days < 0 {
            Ok((-year, 366 - ordinal))
        } else {
            Ok((year, ordinal + 1))
        }
    }
}

/// 将年份及天数（当前年份的第几天）转换为天数
///
/// # Examples
///
/// ```
/// use ylong_time::base::kernel::yo_to_days;
///
/// let (year_zero, ordinal_last) = (0, 366);
/// let (year_one, ordinal_first) = (1, 1);
///
/// assert_eq!(yo_to_days(year_zero, ordinal_last).unwrap(), 0);
/// assert_eq!(yo_to_days(year_one, ordinal_first).unwrap(), 1);
/// ```
pub fn yo_to_days(year: i32, ordinal: u32) -> Result<i32, TimeError> {
    if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
        Err(TimeError::from(BaseErrorKind::InvalidYear))
    } else if ordinal < 1 || ordinal > SundayLetter::from_year(year).ndays() {
        Err(TimeError::from(BaseErrorKind::InvalidOrdinal))
    } else {
        let mut ndays = -365;
        let mut year = year;

        if year < 0 {
            let excess = 1 + (-year) / 400;
            year += excess * 400;
            ndays -= excess * DAYS_OF_FOUR_HUNDRED;
        }
        let year_div_400 = year / 400;
        let year_mod_400 = year % 400;

        // 将对应的年数转换为对应的天数
        Ok(ndays
            + year_div_400 * DAYS_OF_FOUR_HUNDRED
            + year_mod_400 * 365
            + YEAR_DELTAS[year_mod_400 as usize] as i32
            + ordinal as i32
            - 1)
    }
}

impl SundayLetter {
    /// 找到对应年份的主日字母
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::kernel::{ SundayLetter, G, F };
    ///
    /// let year_g = 2001;
    /// let year_f = 2002;
    ///
    /// assert_eq!(SundayLetter::from_year(year_g), G);
    /// assert_eq!(SundayLetter::from_year(year_f), F);
    /// ```
    pub fn from_year(year: i32) -> SundayLetter {
        let year = (year % 400).abs();
        YEAR_TO_SUN_FLAGS[year as usize]
    }

    /// 判断拥有当前主日字母的年份天数有多少天
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::kernel::{ A, AG };
    ///
    /// let leap_year = AG;
    /// let no_leap_year = A;
    ///
    /// assert_eq!(no_leap_year.ndays(), 365);
    /// assert_eq!(leap_year.ndays(), 366);
    /// ```
    pub fn ndays(&self) -> u32 {
        let SundayLetter(flags) = *self;
        366 - u32::from(flags >> 3)
    }

    /// ISO 周日历计算，根据当前年份的主日字母，计算当前年份有多少周
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::kernel::D;
    ///
    /// assert_eq!(D.isoweeknums(), 53);
    /// ```
    pub fn isoweeknums(&self) -> u32 {
        let SundayLetter(flags) = *self;
        //主日字母为 `D`、`DC`、`ED` 的年份拥有 53 周
        52 + ((0b0000_0100_0000_0110 >> flags as usize) & 1)
    }
}

// `O` 意味着某个年份的第几天，`L` 意味着该年份是否是闰年
pub(crate) const MIN_OL: u32 = 1 << 1;
pub(crate) const MAX_OL: u32 = 366 << 1;

// `XX` 的情况为无效值，为负数即可，其它值用于对应的月份及当月第几天对应的当年第几天
const XX: i8 = -1;
static MDL_TO_OL: [i8; MAX_MDL as usize + 1] = [
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, // 0
    XX, XX, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, // 1
    XX, XX, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66,
    66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66,
    66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, XX, XX, XX, XX, XX, // 2
    XX, XX, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74,
    72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74,
    72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, // 3
    XX, XX, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76,
    74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76,
    74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, XX, XX, // 4
    XX, XX, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80,
    78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80,
    78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, // 5
    XX, XX, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82,
    80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82,
    80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, XX, XX, // 6
    XX, XX, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86,
    84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86,
    84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, // 7
    XX, XX, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88,
    86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88,
    86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, // 8
    XX, XX, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90,
    88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90,
    88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, XX, XX, // 9
    XX, XX, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94,
    92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94,
    92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, // 10
    XX, XX, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96,
    94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96,
    94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, XX, XX, // 11
    XX, XX, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98,
    100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100,
    98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98,
    100, // 12
];

/// 某个年份的天数（该年的第几天）以及该年份的主日字母。
///
/// 表示为 `(ordinal << 4) | flags`，`flags` 作为主日数字最大为四位，因此一年的天数左移四位。
#[derive(Copy, Clone, PartialOrd, PartialEq, Debug, Eq)]
pub struct Of(u32);

impl Of {
    /// 创建一个新的 `Of` 结构体
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::kernel::{ Of, A };
    ///
    /// let of = Of::new(1, A).unwrap();
    /// ```
    pub fn new(ordinal: u32, flags: SundayLetter) -> Result<Of, TimeError> {
        if ordinal == 0 || ordinal > flags.ndays() {
            Err(TimeError::from(BaseErrorKind::InvalidOrdinal))
        } else {
            Ok(Of((ordinal << 4) | u32::from(flags.0)))
        }
    }

    /// 判断当前 `Of` 结构是否为有效值
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::kernel::{Of, AG};
    ///
    /// let of = Of::new(1, AG).unwrap();
    /// let of_two = Of::new(366, AG).unwrap().succ();
    ///
    /// assert_eq!(of.valid(), true);
    /// assert_eq!(of_two.valid(), false);
    /// ```
    pub fn valid(&self) -> bool {
        let Of(of) = *self;
        let ol = of >> 3;
        (MIN_OL..=MAX_OL).contains(&ol)
    }

    /// 返回当前 `Of` 结构的年份的天数（该年的第几天）
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::kernel::{Of, AG};
    ///
    /// let of = Of::new(366, AG).unwrap();
    /// assert_eq!(of.ordinal(), 366);
    /// ```
    pub fn ordinal(&self) -> u32 {
        let Of(of) = *self;
        of >> 4
    }

    /// 将当前 `Of` 结构的年份的天数替换为新的天数（该年的第几天），
    /// 并返回新的 `Of` 结构体
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::kernel::{ Of, A };
    ///
    /// let of = Of::new(1, A).unwrap();
    /// let of_two = Of::new(2, A).unwrap();
    ///
    /// assert_eq!(of.with_ordinal(2).unwrap(), of_two);
    /// ```
    pub fn with_ordinal(&self, ordinal: u32) -> Result<Of, TimeError> {
        if ordinal > self.flags().ndays() {
            Err(TimeError::from(BaseErrorKind::InvalidOrdinal))
        } else {
            let Of(of) = *self;
            Ok(Of((ordinal << 4) | of & 0b1111))
        }
    }

    /// Returns the day of the week corresponding to the current `Of` structure.
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::{BaseDate, Datelike, Weekday};
    ///
    /// let base_date = BaseDate::from_ymd(2022, 12, 15).unwrap();
    /// assert_eq!(base_date.weekday(), Weekday::Thursday);
    /// ```
    pub fn weekday(&self) -> Weekday {
        let of = self.0;
        // Sunday Letter Calculation Method:
        //
        // By the day of the year corresponding to the Of,
        // and the day of the week on which the SundayLetter corresponds to the first day of the year,
        // Calculate the day of the week for the corresponding date.
        Weekday::weekday0_from_u8((((of >> 4) + (of & 0b111)) % 7) as u8).unwrap()
    }

    /// 返回当前 `Of` 结构的年份主日字母
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::kernel::{ Of, A };
    ///
    /// let of = Of::new(1, A).unwrap();
    /// assert_eq!(of.flags(), A);
    /// ```
    pub fn flags(&self) -> SundayLetter {
        let Of(of) = *self;
        SundayLetter((of & 0b1111) as u8)
    }

    /// 将当前 `Of` 结构的年份主日字母替换为新的主日字母
    /// 并返回新的 `Of` 结构体
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::kernel::{ Of, A, B };
    ///
    /// let of = Of::new(1, A).unwrap();
    /// let of_two = Of::new(1, B).unwrap();
    /// assert_eq!(of.with_flags(B).unwrap(), of_two);
    /// ```
    pub fn with_flags(&self, flags: SundayLetter) -> Result<Of, TimeError> {
        if self.ordinal() > flags.ndays() {
            Err(TimeError::from(BaseErrorKind::InvalidOrdinal))
        } else {
            let Of(of) = *self;
            Ok(Of((of & !0b1111) | u32::from(flags.0)))
        }
    }

    /// 当前日期的下一个日期
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::kernel::{ Of, A };
    ///
    /// let of = Of::new(1, A).unwrap();
    /// let of_two = Of::new(2, A).unwrap();
    ///
    /// assert_eq!(of.succ(), of_two);
    /// ```
    pub fn succ(&self) -> Of {
        // 此处生成的 `Of` 无法保证有效性，但这是为了上一层接口的功能
        let Of(of) = *self;
        Of(of + (1 << 4))
    }

    /// 当前日期的上一个日期
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::kernel::{ Of, A };
    ///
    /// let of = Of::new(2, A).unwrap();
    /// let of_two = Of::new(1, A).unwrap();
    ///
    /// assert_eq!(of.pred(), of_two);
    /// ```
    pub fn pred(&self) -> Of {
        let Of(of) = *self;
        Of(of - (1 << 4))
    }

    /// 将 `Mdf` 结构转换为 `Of` 结构
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::kernel::{ Of, Mdf, A };
    ///
    /// let of = Of::new(1, A).unwrap();
    /// let mdf = Mdf::new(1, 1, A).unwrap();
    /// assert_eq!(Of::from_mdf(mdf), of);
    /// ```
    pub fn from_mdf(Mdf(mdf): Mdf) -> Of {
        let mdl = mdf >> 3;
        match MDL_TO_OL.get(mdl as usize) {
            Some(&temp) => Of(mdf.wrapping_sub((i32::from(temp) as u32 & 0x3ff) << 3)),
            None => Of(0),
        }
    }

    /// 将当前 `Of` 结构转换为 `Mdf` 结构
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::kernel::{ Of, Mdf, A };
    ///
    /// let of = Of::new(1, A).unwrap();
    /// let mdf = Mdf::new(1, 1, A).unwrap();
    /// assert_eq!(of.as_mdf(), mdf);
    /// ```
    pub fn as_mdf(&self) -> Mdf {
        Mdf::from_of(*self)
    }
}

// `M` 意味着月份、`D` 意味着该月份的天数（第几天）、`L` 代表当前年份是否是闰年
#[allow(dead_code)]
pub(crate) const MIN_MDL: u32 = (1 << 6) | (1 << 1);
pub(crate) const MAX_MDL: u32 = (12 << 6) | (31 << 1) | 1;

// 用于计算该年第几天对应的月份及该月第几天
static OL_TO_MDL: [u8; MAX_OL as usize + 1] = [
    0, 0, // 0
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, // 1
    66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66,
    66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66,
    66, 66, 66, 66, 66, 66, 66, 66, 66, // 2
    74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72,
    74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72,
    74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, 74, 72, // 3
    76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74,
    76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74,
    76, 74, 76, 74, 76, 74, 76, 74, 76, 74, 76, 74, // 4
    80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78,
    80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78,
    80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, 80, 78, // 5
    82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80,
    82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80,
    82, 80, 82, 80, 82, 80, 82, 80, 82, 80, 82, 80, // 6
    86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84,
    86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84,
    86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, 86, 84, // 7
    88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86,
    88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86,
    88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, 88, 86, // 8
    90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88,
    90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88,
    90, 88, 90, 88, 90, 88, 90, 88, 90, 88, 90, 88, // 9
    94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92,
    94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92,
    94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, 94, 92, // 10
    96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94,
    96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94,
    96, 94, 96, 94, 96, 94, 96, 94, 96, 94, 96, 94, // 11
    100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100,
    98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98,
    100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100, 98, 100,
    98, // 12
];

/// 月份、天数（月份的第几天）以及年份主日字母
///
/// 表示为 `(month << 9) | (day << 4) | flags`，`flags` 作为主日字母最大为四位，
/// 因此天数左移四位，天数最大为五位，因此月份需左移九位。
#[derive(Copy, Clone, PartialOrd, PartialEq, Eq, Debug)]
pub struct Mdf(pub u32);

impl Mdf {
    /// 创建一个新的 `Mdf` 结构体
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::kernel::{ Mdf, A };
    ///
    /// let mdf = Mdf::new(1, 1, A).unwrap();
    /// ```
    pub fn new(month: u32, day: u32, SundayLetter(flags): SundayLetter) -> Result<Mdf, TimeError> {
        if month == 0 || month > 12 {
            Err(TimeError::from(BaseErrorKind::InvalidMonth))
        } else if day == 0 || day > 31 {
            Err(TimeError::from(BaseErrorKind::InvalidDay))
        } else {
            Ok(Mdf((month << 9) | (day << 4) | u32::from(flags)))
        }
    }

    /// 判断当前 `Mdf` 结构是否为有效值
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::kernel::{ Mdf, A };
    ///
    /// let mdf = Mdf::new(1, 1, A).unwrap();
    /// assert_eq!(mdf.valid(), true);
    /// ```
    pub fn valid(&self) -> bool {
        let Mdf(mdf) = *self;
        let mdl = mdf >> 3;
        match MDL_TO_OL.get(mdl as usize) {
            Some(&delta) => delta >= 0,
            None => false,
        }
    }

    /// 返回当前 `Mdf` 结构的月份
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::kernel::{ Mdf, A };
    ///
    /// let mdf = Mdf::new(1, 1, A).unwrap();
    /// assert_eq!(mdf.month(), 1);
    /// ```
    pub fn month(&self) -> u32 {
        let Mdf(mdf) = *self;
        mdf >> 9
    }

    /// 将当前 `Mdf` 结构的月份替换为新的月份，
    /// 并返回新的 `Mdf` 结构体
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::kernel::{ Mdf, A };
    ///
    /// let mdf = Mdf::new(1, 1, A).unwrap();
    /// let mdf_two = Mdf::new(2, 1, A).unwrap();
    ///
    /// assert_eq!(mdf.with_month(2).unwrap(), mdf_two);
    /// ```
    pub fn with_month(&self, month: u32) -> Result<Mdf, TimeError> {
        if month == 0 || month > 12 {
            Err(TimeError::from(BaseErrorKind::InvalidMonth))
        } else {
            let Mdf(mdf) = *self;
            Ok(Mdf((month << 9) | (mdf & 0b1_1111_1111)))
        }
    }

    /// 返回当前 `Mdf` 结构的天数（该月的第几天）
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::kernel::{ Mdf, A };
    ///
    /// let mdf = Mdf::new(1, 2, A).unwrap();
    /// assert_eq!(mdf.day(), 2);
    /// ```
    pub fn day(&self) -> u32 {
        let Mdf(mdf) = *self;
        (mdf >> 4) & 0b1_1111
    }

    /// 将当前 `Mdf` 结构的天数（该月的第几天）替换为新的天数，
    /// 并返回新的 `Mdf` 结构体
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::kernel::{ Mdf, A };
    ///
    /// let mdf = Mdf::new(1, 1, A).unwrap();
    /// let mdf_two = Mdf::new(1, 2, A).unwrap();
    ///
    /// assert_eq!(mdf.with_day(2).unwrap(), mdf_two);
    /// ```
    pub fn with_day(&self, day: u32) -> Result<Mdf, TimeError> {
        if day == 0 || day > 31 {
            Err(TimeError::from(BaseErrorKind::InvalidDay))
        } else {
            let Mdf(mdf) = *self;
            Ok(Mdf((day << 4) | (mdf & !0b1_1111_0000)))
        }
    }

    /// 返回当前 `Mdf` 结构的年份主日字母
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::kernel::{ Mdf, A, B };
    ///
    /// let mdf = Mdf::new(1, 1, A).unwrap();
    /// assert_eq!(mdf.flags(), A);
    /// ```
    pub fn flags(&self) -> SundayLetter {
        let Mdf(mdf) = *self;
        SundayLetter((mdf & 0b1111) as u8)
    }

    /// 将当前 `Mdf` 结构的年份主日字母替换为新的主日字母
    /// 并返回新的 `Mdf` 结构体
    ///
    /// ```
    /// use ylong_time::base::kernel::{ Mdf, A, B };
    ///
    /// let mdf = Mdf::new(1, 1, A).unwrap();
    /// let mdf_two = Mdf::new(1, 1, B).unwrap();
    ///
    /// assert_eq!(mdf.with_flags(B), mdf_two);
    /// ```
    pub fn with_flags(&self, SundayLetter(flags): SundayLetter) -> Mdf {
        let Mdf(mdf) = *self;
        Mdf((mdf & !0b1111) | u32::from(flags))
    }

    /// 将 `Of` 结构转换为 `Mdf` 结构
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::kernel::{ Of, Mdf, A };
    ///
    /// let of = Of::new(1, A).unwrap();
    /// let mdf = Mdf::new(1, 1, A).unwrap();
    /// assert_eq!(Mdf::from_of(of), mdf);
    /// ```
    pub fn from_of(Of(of): Of) -> Mdf {
        let ol = of >> 3;
        match OL_TO_MDL.get(ol as usize) {
            Some(&temp) => Mdf(of + (u32::from(temp) << 3)),
            None => Mdf(0),
        }
    }

    /// 将当前 `Mdf` 结构转换为 `Of` 结构
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::base::kernel::{ Of, Mdf, A };
    ///
    /// let mdf = Mdf::new(1, 1, A).unwrap();
    /// let of = Of::new(1, A).unwrap();
    /// assert_eq!(mdf.as_of(), of);
    /// ```
    pub fn as_of(&self) -> Of {
        Of::from_mdf(*self)
    }
}

#[cfg(test)]
mod test {
    include!("../../tests/ut/base/ut_kernel.rs");
}
