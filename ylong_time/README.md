## ylong_time 介绍

ylong_time 时间管理模块，提供时间管理基础功能

## 功能介绍：

### 日期与时间（Date and Time）

ylong_time 提供不同时区的日期与时间管理功能，时区包含 `UTC`、`Local`

- `UTC` 时区的时间获取与简单使用

  ```rust
  use ylong_time::Utc;
  
  let utc = Utc::now().unwrap();
  println!("{}", utc);
  println!("{:?}", utc);
  ```

  输出：

  ```rust
  2021-06-10 12:13:57.812409522 UTC
  2021-06-10T12:13:57.812409522Z
  ```

- `Local` 时区的时间获取与简单使用

  ```rust
  use ylong_time::Local;
  
  let local = Local::now().unwrap();
  println!("{}", utc);
  println!("{:?}", utc);
  ```

  输出：

  ```rust
  2021-06-10 20:16:21.011359385 +0800
  2021-06-10T20:16:21.011359385+0800
  ```

### 时间段（Duration）

代表当前时间所经历过的时间段

- `Duration` 所代表的时间段，基本使用方法

  ```rust
  use ylong_time::Duration;
  
  let local = Local::now().unwrap();
  println!("local: {}", local);
  let duration = Duration::days(1).unwrap();
  let new_datetime = local + duration;
  println!("new_datetime: {}", new_datetime);
  ```

  输出：
  
  ```rust
  local: 2021-06-10 20:16:21.011359385 +0800
  new_datetime: 2021-06-11 20:16:21.011359385 +0800
  ```


### 格式化操作

- 将满足 `rfc3339` 格式的字符串转换为 `DateTime` 格式

  ```rust
  use ylong_time::datetime::DateTime;
  use ylong_time::BaseDate;
  use ylong_time::BaseTime;
  use ylong_time::BaseDateTime;
  use ylong_time::FixedOffset;
  
  // 满足 `rfc3339` 格式的字符串
  let s = "2021-06-15T17:01:51.954585101+08:00";
  // 通过字符串创建出 `DateTime` 结构
  let datetime = DateTime::parse_from_rfc3339(s).unwrap();
  // 用于校验的数据
  let base_date = BaseDate::from_ymd(2021, 6, 15).unwrap();
  let base_time = BaseTime::from_hms_nano(17, 1, 51, 954585101).unwrap();
  let base_datetime = BaseDateTime::new(base_date, base_time);
  let offset = FixedOffset::east(8 * 3600).unwrap();
  assert_eq!(datetime.datetime, base_datetime);
  assert_eq!(datetime.offset, offset);
  ```

- 将 `DateTime` 结构转换为满足 `rfc3339` 格式的字符串

  ```rust
  use ylong_time::datetime::DateTime;
  use ylong_time::Utc;
  
  let utc = Utc::now().unwrap();
  let rfc3339 = DateTime::parse_from_rfc3339(utc.to_rfc3339().as_str()).unwrap();
  assert_eq!(utc.datetime, rfc3339.datetime);
  assert_eq!(utc.offset.fix(), rfc3339.offset);
  ```