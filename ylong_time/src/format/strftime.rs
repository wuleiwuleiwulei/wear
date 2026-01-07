/*!
日期与时间格式语法的

## 说明符

以下说明符既可用于格式化也可用于解析

| 说明符 |   样例   |     描述说明                                                                |
|-------|----------|----------------------------------------------------------------------------|
|       |          | **日期说明符:**                                                            |
| `%Y`  | `2001`   | 完整的格里公历年份，可用 0 填充至 4 位数字。[^1]                               |
| `%C`  | `20`     | 完整的格里公历年份除以 100，可用 0 填充为 2 位数字。[^2]                       |
| `%y`  | `01`     | 完整的格里公历年份取余 100，可用 0 填充为 2 位数字。[^2]                       |
|       |          |                                                                            |
| `%m`  | `07`     | 月份 (01 -- 12)，可用 0 填充为 2 位数字。                                    |
| `%b`  | `Jul`    | 月份的缩写名称，总是为 3 个字母。                                             |
| `%B`  | `July`   | 月份的全名，在解析中也接受相应的缩写。                                         |
| `%h`  | `Jul`    | 与 `%b` 相同。                                                              |
|       |          |                                                                            |
| `%d`  | `08`     | 日数 (01 -- 31)，可用 0 填充为 2 位数字。                                    |
| `%e`  | ` 8`     | 与 `%d` 相同，但是通过空格填充，与 `%_d` 相同。                               |
|       |          |                                                                         |
| `%a`  | `Mon`    | Abbreviations for the days of the week.                                    |
| `%A`  | `Monday` | Full name for the days of the week.                                        |
|       |          |                                                                            |
| `%j`  | `189`    | 某年份的第几天，可用 0 填充至 3 个数字。                                      |
|       |          |                                                                            |
| `%D`  | `07/08/01`    | 月份-该月第几天-年份。与 `%m/%d/%y` 相同。                               | |
| `%F`  | `2001-07-08`  | 年份-月份-该月第几天 (ISO 8601)。与 `%Y-%m-%d` 相同。                    |
| `%v`  | ` 8-Jul-2001` | 该月第几天-月份缩略名-年份。与 `%e-%b-%Y` 相同。                         |
|       |          |                                                                            |
|       |          | **时间说明符:**                                                            |
| `%H`  | `00`     | 小时数 (00--23)，可用 0 填充为 2 位数字。                                    |
| `%k`  | ` 0`     | 与 `%H` 相同，但是通过空格进行填充。与 `%_H` 相同。                           |
|       |          |                                                                            |
| `%M`  | `34`     | 分钟数 (00--59)，可用 0 填充为 2 位数字。                                    |
| `%S`  | `60`     | 秒数 (00--60)，可用 0 填充为 2 位数字。[^3]                                  |
| `%f`  | `026490000`   | 纳秒数，以纳秒为单位 [^6]                                               |
| `%.f` | `.026490`| 与 `.%f` 类似但是左对齐。这些都会消耗前面的 `.` [^6]                          |
| `%.3f`| `.026`        | 与 `.%f` 类似但是左对齐。并且强制长度为 3。[^6]                          |
| `%.6f`| `.026490`     | 与 `.%f` 类似但是左对齐。并且强制长度为 6。 [^6]                         |
| `%.9f`| `.026490000`  | 与 `.%f` 类似但是左对齐。并且强制长度为 9。 [^6]                         | |
|       |               |                                                                      |
| `%R`  | `00:34`       | 小时-分钟。与 `%H:%M` 相同。                                           |
| `%T`  | `00:34:60`    | 小时-分钟-秒。与 `%H:%M:%S` 相同。                                     | |
|       |          |
|       |          | **时区说明符:**                                                            |
| `%Z`  | `ACST`   | 本地时区名字。在解析期间跳过所有非空白字符。[^7]                               |
| `%z`  | `+0930`  | 本地时间到 `UTC` 的偏移量 (`UTC` 是 `+0000`)。                               |
| `%:z` | `+09:30` | 与 `%z` 相同，但是有冒号。                                                   | |
|       |          |                                                                            |
|       |          | **日期与时间说明符:**                                                       | |
| `%+`  | `2001-07-08T00:34:60.026490+09:30` | ISO 8601 / RFC 3339 日期与时间格式。 [^4]          |
|       |               |                                                                       |
| `%s`  | `994518299`   | UNIX 时间戳, 从 1970-01-01 00:00 UTC 开始的秒数。 [^5]                  |
|       |          |                               `                                             |
|       |          | **特殊说明符:**                                                            |
| `%t`  |          | 转义 tab (`\t`)。                                                          |
| `%n`  |          | 转义 换行 (`\n`)。                                                          |
| `%%`  |          | 转义 百分号。                                                               |

It is possible to override the default padding behavior of numeric specifiers `%?`.
This is not allowed for other specifiers and will result in the `BAD_FORMAT` error.

修饰语    | 描述说明
-------- | -----------
`%-?`    | 禁止空格和零的任何填充。(例如 `%j` = `012`, `%-j` = `12`)
`%_?`    | 使用空格作为填充项。(例如 `%j` = `012`, `%_j` = ` 12`)
`%0?`    | 使用 0 作为填充项。(e.g. `%e` = ` 9`, `%0e` = `09`)

Notes:

[^1]: `%Y`:
   为负数的年份在格式化中运行使用，但解析过程中不支持

[^2]: `%C`, `%y`:
   这是 `floor` 除法，因此 公元前 100 年 (-99 年) 将会分别打印 `-1` 和 `99`

[^3]: `%S`:
   将会统计闰秒，因此 `60` 是有可能的。

[^4]: `%+`: 与 `%Y-%m-%dT%H:%M:%S%.f%:z` 相同，例如 0, 3, 6 or 9 位的纳秒数，以及在时区偏移量中的冒号。
   <br>
   <br>
   典型的 `strftime` 实现具有此说明符的不同（和区域设置相关）格式。
   虽然 `%+` 的格式要稳定得多，但如果要控制确切的输出，最好避免使用此说明符。

[^5]: `%s`:
   这不是可填充的，可以是负数。
   由于我们只考虑非闰秒，因此它与 ISO C `strftime` 的行为不同。

[^6]: `%f`, `%.f`, `%.3f`, `%.6f`, `%.9f`:
   <br>
   默认的 `%f` 是右对齐的，并且始终通过 0 填充至 9 个数字以兼容 glibc 或其它。
   因此它始终计算自最后一整秒以来的纳秒数。例如，最后一秒后 7 毫秒将打印 `007000000`，
   解析 `7000000` 将产生相同的结果。
   <br>
   <br>
   变量 `%.f` 左对齐，并且根据精度打印 0、3、6、9 位的纳秒数。
   例如，在 `%.f` 下的最后一秒后 70ms 将打印 `.070`（注意：不是`.07`），
   解析 `.07`、`.070000` 等将产生相同的结果。请注意，如果小数部分为零或下一个字符不是 `.`，他们可以不打印或读取任何内容。
   <br>
   <br>
   变体 `%.3f`、`%.6f` 和 `%.9f` 左对齐，并根据 `f` 前面的数字打印 3、6 或 9 位小数字。
   例如，在 `%.3f`下的最后一秒后 70ms 将打印 `.070`（注意：不是`.07`），
   解析`.07`、`.070000`等将产生相同的结果。请注意，如果小数部分为零或下一个字符不是 `.`，则它们不能读取任何内容。但是将以指定的长度打印。
   <br>

[^7]: `%Z`:
   偏移量将不会从解析的数据中填充，也不会对其进行验证。时区被完全忽略。类似于此格式代码的 glibc `strptime`处理。
   <br>
   <br>
   无法可靠地从缩写转换为偏移量，例如 CDT 可以表示中部夏令时（北美）或中国夏令时。
 */

use super::{Item, Numeric, Specific};
use crate::format::Pad;
use crate::{lit, num, num0, nums, sp, spec};

/// 解析类似 `strftime` 格式字符串的迭代器。
#[derive(Clone, Debug)]
pub struct StrftimeItems<'a> {
    /// 字符串的剩余部分
    remainder: &'a str,
    /// 如果当前说明符由多个格式化项组成（例如 `%+`），则解析器引用它们的静态重建切片。
    /// 如果 `recons' 不为空，则必须在 `remainder` 之前返回。
    recons: &'static [Item<'static>],
}

impl<'a> StrftimeItems<'a> {
    /// 创建一个新的解析类似 `strftime` 格式的字符串迭代器。
    pub fn new(s: &'a str) -> StrftimeItems<'a> {
        static FMT_NONE: &[Item<'static>; 0] = &[];

        StrftimeItems {
            remainder: s,
            recons: FMT_NONE,
        }
    }
}

impl<'a> Iterator for StrftimeItems<'a> {
    type Item = Item<'a>;

    fn next(&mut self) -> Option<Item<'a>> {
        if !self.recons.is_empty() {
            let item = self.recons[0].clone();
            self.recons = &self.recons[1..];
            return Some(item);
        }

        match self.remainder.chars().next() {
            // 已经结束
            None => None,

            // 下一项是说明符
            Some('%') => {
                self.remainder = &self.remainder[1..];

                macro_rules! next {
                    () => {
                        match self.remainder.chars().next() {
                            Some(x) => {
                                self.remainder = &self.remainder[x.len_utf8()..];
                                x
                            }
                            // 字符串过早结束
                            None => return Some(Item::Error),
                        }
                    };
                }

                // 获取下一个说明符
                let spec = next!();
                // 判断下一个说明符是否是填充项
                let pad_override = match spec {
                    '0' => Some(Pad::Zero),
                    '_' => Some(Pad::Space),
                    '-' => Some(Pad::None),
                    _ => None,
                };
                // 为填充项时获取下一个说明符，否则不变
                let spec = if pad_override.is_some() {
                    next!()
                } else {
                    spec
                };

                macro_rules! recons {
                    [$head: expr, $($tail: expr),+ $(,)*] => ({
                        const RECONS: &'static [Item<'static>] = &[$($tail),+];
                        self.recons = RECONS;
                        $head
                    })
                }

                let item = match spec {
                    'A' => spec!(LongWeekdayName),
                    'B' => spec!(LongMonthName),
                    'C' => num0!(YearDiv100),
                    'D' => recons![
                        num0!(Month),
                        lit!("/"),
                        num0!(Day),
                        lit!("/"),
                        num0!(YearMod100)
                    ],
                    'F' => recons![num0!(Year), lit!("-"), num0!(Month), lit!("-"), num0!(Day)],
                    'H' => num0!(Hour),
                    'M' => num0!(Minute),
                    'R' => recons![num0!(Hour), lit!(":"), num0!(Minute)],
                    'S' => num0!(Second),
                    'T' => recons![
                        num0!(Hour),
                        lit!(":"),
                        num0!(Minute),
                        lit!(":"),
                        num0!(Second)
                    ],
                    'Y' => num0!(Year),
                    'Z' => spec!(TimezoneName),
                    'a' => spec!(ShortWeekdayName),
                    'b' | 'h' => spec!(ShortMonthName),
                    'd' => num0!(Day),
                    'e' => nums!(Day),
                    'f' => num0!(Nanosecond),
                    'j' => num0!(Ordinal),
                    'k' => nums!(Hour),
                    'm' => num0!(Month),
                    'n' => sp!("\n"),
                    's' => num!(Timestamp),
                    't' => sp!("\t"),
                    'v' => {
                        recons![
                            nums!(Day),
                            lit!("-"),
                            spec!(ShortMonthName),
                            lit!("-"),
                            num0!(Year)
                        ]
                    }
                    'y' => num0!(YearMod100),
                    ':' => match next!() {
                        'z' => spec!(TimezoneOffsetColon),
                        _ => Item::Error,
                    },
                    'z' => spec!(TimezoneOffset),
                    '.' => match next!() {
                        'f' => spec!(Nanosecond),
                        '3' => match next!() {
                            'f' => spec!(NanosecondThree),
                            _ => Item::Error,
                        },
                        '6' => match next!() {
                            'f' => spec!(NanosecondSix),
                            _ => Item::Error,
                        },
                        '9' => match next!() {
                            'f' => spec!(NanosecondNine),
                            _ => Item::Error,
                        },
                        _ => Item::Error,
                    },
                    '%' => lit!("%"),
                    '+' => spec!(RFC3339),
                    _ => Item::Error, // no such specifier
                };

                // 如果拥有填充项，调整 `item`
                if let Some(new_pad) = pad_override {
                    match item {
                        Item::Numeric(ref kind, _pad) if self.recons.is_empty() => {
                            Some(Item::Numeric(kind.clone(), new_pad))
                        }
                        _ => Some(Item::Error),
                    }
                } else {
                    Some(item)
                }
            }

            // 下一项为空格
            Some(c) if c.is_whitespace() => {
                let nextspec = self
                    .remainder
                    .find(|c: char| !c.is_whitespace())
                    .unwrap_or(self.remainder.len());
                assert!(nextspec > 0);
                let item = sp!(&self.remainder[..nextspec]);
                self.remainder = &self.remainder[nextspec..];
                Some(item)
            }

            // 下一项为文本
            _ => {
                let nextspec = self
                    .remainder
                    .find(|c: char| c.is_whitespace() || c == '%')
                    .unwrap_or(self.remainder.len());
                assert!(nextspec > 0);
                let item = lit!(&self.remainder[..nextspec]);
                self.remainder = &self.remainder[nextspec..];
                Some(item)
            }
        }
    }
}
