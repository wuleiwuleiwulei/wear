use std::convert::TryInto;
use ylong_time::datetime::DateTime;
use ylong_time::offset::fixed::FixedOffset;
use ylong_time::offset::TimeZone;
use ylong_time::{BaseDate, BaseDateTime, BaseTime, Datelike, Duration, Local, Utc, Weekday};

fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && year % 100 != 0 || year % 400 == 0
}

/// SDV test for ylong_time::BaseDate::from_ymd()
///
/// # Title
/// sdv_basedate_from_ymd_valid_inputs
///
/// # Brief
/// Test whether the output is as expected and desired when valid input is provided.
#[test]
fn sdv_basedate_from_ymd_valid_inputs() {
    for y in 1870..2200 {
        for m in 1..13 {
            for d in 1..32 {
                if m == 2 && (is_leap_year(y) && d > 29 || !is_leap_year(y) && d > 28) {
                    continue;
                }
                if (m == 4 || m == 6 || m == 9 || m == 11) && d > 30 {
                    continue;
                }
                let date = BaseDate::from_ymd(y, m, d);
                assert!(date.is_ok());
                let date_str = date.unwrap().to_string();
                let date_str_to_compare = format!("{y}-{m:02}-{d:02}");
                assert_eq!(date_str, date_str_to_compare);
            }
        }
    }
}

/// SDV test for ylong_time::BaseDate::from_ymd()
///
/// # Title
/// sdv_basedate_from_ymd_invalid_inputs
///
/// # Brief
/// Test whether the method throws an error when invalid output is provided.
#[test]
fn sdv_basedate_from_ymd_invalid_inputs() {
    for y in 1870..2200 {
        for m in 1..13 {
            for d in 28..32 {
                if m == 2 && (is_leap_year(y) && d > 29 || !is_leap_year(y) && d > 28) {
                    let date = BaseDate::from_ymd(y, m, d);
                    assert!(date.is_err());
                }
                if (m == 4 || m == 6 || m == 9 || m == 11) && d > 30 {
                    let date = BaseDate::from_ymd(y, m, d);
                    assert!(date.is_err());
                }
            }
        }
    }

    for y in 1870..2200 {
        for m in 13..25 {
            let d = 2;
            let date = BaseDate::from_ymd(y, m, d);
            assert!(date.is_err());
        }
        for m in 1..13 {
            for d in 32..52 {
                let date = BaseDate::from_ymd(y, m, d);
                assert!(date.is_err());
            }
        }
    }
}

/// SDV test for ylong_time::BaseDate::from_yo()
///
/// # Title
/// sdv_basedate_from_yo_handle_leap_year
///
/// # Brief
/// Test if the method handles leap year properly.
#[test]
fn sdv_basedate_from_yo_handle_leap_year() {
    for y in 1870..2200 {
        if is_leap_year(y) {
            let date = BaseDate::from_yo(y, 60);
            assert!(date.is_ok());
            let date_str = date.unwrap().to_string();
            let date_str_to_compare = format!("{y}-02-29");
            assert_eq!(date_str, date_str_to_compare);
            let date = BaseDate::from_yo(y, 366);
            assert!(date.is_ok());
            let date_str = date.unwrap().to_string();
            let date_str_to_compare = format!("{y}-12-31");
            assert_eq!(date_str, date_str_to_compare);
            let date = BaseDate::from_yo(y, 365);
            assert!(date.is_ok());
            let date_str = date.unwrap().to_string();
            let date_str_to_compare = format!("{y}-12-30");
            assert_eq!(date_str, date_str_to_compare);
            let date = BaseDate::from_yo(y, 367);
            assert!(date.is_err());
        } else {
            let date = BaseDate::from_yo(y, 60);
            assert!(date.is_ok());
            let date_str = date.unwrap().to_string();
            let date_str_to_compare = format!("{y}-03-01");
            assert_eq!(date_str, date_str_to_compare);
            let date = BaseDate::from_yo(y, 365);
            assert!(date.is_ok());
            let date_str = date.unwrap().to_string();
            let date_str_to_compare = format!("{y}-12-31");
            assert_eq!(date_str, date_str_to_compare);
            let date = BaseDate::from_yo(y, 366);
            assert!(date.is_err());
        }
    }
}

/// SDV test for ylong_time::BaseDate::succ()
///
/// # Title
/// sdv_basedate_succ
///
/// # Brief
/// Test whether the method performs as expected.
#[test]
fn sdv_basedate_succ() {
    for y in 1870..2200 {
        for m in 1..13 {
            for d in 1..32 {
                if is_leap_year(y) && m == 2 && d == 29 {
                    let date = BaseDate::from_ymd(y, m, d).unwrap();
                    let next = format!("{y}-03-01");
                    assert!(date.succ().unwrap().to_string().eq(&next));
                } else if is_leap_year(y) && m == 2 && d == 28 {
                    let date = BaseDate::from_ymd(y, m, d).unwrap();
                    let next = format!("{y}-02-29");
                    assert!(date.succ().unwrap().to_string().eq(&next));
                } else if m == 12 && d == 31 {
                    let date = BaseDate::from_ymd(y, m, d).unwrap();
                    let next = format!("{}-01-01", y + 1);
                    assert!(date.succ().unwrap().to_string().eq(&next));
                } else if (m == 1 || m == 3 || m == 5 || m == 7 || m == 8 || m == 10) && d == 31 {
                    let date = BaseDate::from_ymd(y, m, d).unwrap();
                    let next = format!("{}-{:02}-01", y, m + 1);
                    assert!(date.succ().unwrap().to_string().eq(&next));
                }
            }
        }
    }
}

/// SDV test for ylong_time::BaseDate::pred()
///
/// # Title
/// sdv_basedate_pred
///
/// # Brief
/// Test whether the method performs as expected.
#[test]
fn sdv_basedate_pred() {
    for y in 1870..2200 {
        for m in 1..13 {
            for d in 1..32 {
                if is_leap_year(y) && m == 3 && d == 1 {
                    let date = BaseDate::from_ymd(y, m, d).unwrap();
                    let prev = format!("{y}-02-29");
                    assert!(date.pred().unwrap().to_string().eq(&prev));
                } else if m == 1 && d == 1 {
                    let date = BaseDate::from_ymd(y, m, d).unwrap();
                    let prev = format!("{}-12-31", y - 1);
                    assert!(date.pred().unwrap().to_string().eq(&prev));
                } else if !is_leap_year(y) && m == 3 && d == 1 {
                    let date = BaseDate::from_ymd(y, m, d).unwrap();
                    let prev = format!("{y}-02-28");
                    assert!(date.pred().unwrap().to_string().eq(&prev));
                } else if (m == 4 || m == 6 || m == 9 || m == 11) && d == 1 {
                    let date = BaseDate::from_ymd(y, m, d).unwrap();
                    let prev = format!("{}-{:02}-31", y, m - 1);
                    assert!(date.pred().unwrap().to_string().eq(&prev));
                }
            }
        }
    }
}

/// SDV test for ylong_time::BaseDate::checked_add_signed()
///
/// # Title
/// sdv_basedate_checked_add_signed
///
/// # Brief
/// Test whether the method performs as expected.
#[test]
fn sdv_basedate_checked_add_signed() {
    for y in 1870..2200 {
        let m = 2;
        let d = 1;
        if is_leap_year(y) {
            let date = BaseDate::from_ymd(y, m, d).unwrap();
            let date_str_to_compare = format!("{y}-02-29");
            assert!(date
                .checked_add_signed(Duration::days(28).unwrap())
                .unwrap()
                .to_string()
                .eq(&date_str_to_compare));
        }
        for n in 0..10000 {
            let date = BaseDate::from_ymd(y, m, d).unwrap();
            assert!(date.checked_add_signed(Duration::days(n).unwrap()).is_ok());
        }
    }
}

/// SDV test for ylong_time::BaseDate::checked_sub_signed()
///
/// # Title
/// sdv_basedate_checked_sub_signed
///
/// # Brief
/// Test whether the method performs as expected.
#[test]
fn sdv_basedate_checked_sub_signed() {
    for y in 1870..2200 {
        let m = 2;
        let d = 29;
        if is_leap_year(y) {
            let date = BaseDate::from_ymd(y, m, d).unwrap();
            let date_str_to_compare = format!("{y}-01-31");
            assert!(date
                .checked_sub_signed(Duration::days(29).unwrap())
                .unwrap()
                .to_string()
                .eq(&date_str_to_compare));
        }
        let d = 2;
        for n in 0..10000 {
            let date = BaseDate::from_ymd(y, m, d).unwrap();
            assert!(date.checked_sub_signed(Duration::days(n).unwrap()).is_ok());
        }
    }
}

/// SDV test for ylong_time::BaseDateTime::new()
///
/// # Title
/// sdv_leap_year_test_for_basedatetime_new
///
/// # Brief
/// Test if the method handles leap year properly.
#[test]
fn sdv_leap_year_test_for_basedatetime_new() {
    for y in 1870..2200 {
        let m = 2;
        let d = 29;
        if is_leap_year(y) {
            let base_date = BaseDate::from_ymd(y, m, d).unwrap();
            let base_time = BaseTime::from_hms(11, 12, 13).unwrap();
            let datetime = BaseDateTime::new(base_date, base_time);
            let str_to_compare = format!("{y}-02-29 11:12:13");
            assert_eq!(datetime.to_string(), str_to_compare);
        }
    }
}

/// SDV test for ylong_time::DateTime::parse_from_rfc3339()
///
/// # Title
/// sdv_parse_invalid_date_string
///
/// # Brief
/// Test whether the method throws an error when invalid output is provided.
#[test]
fn sdv_parse_invalid_date_string() {
    // invalid month
    let s = "2222-13-31T23:59:59Z";
    let datetime = DateTime::parse_from_rfc3339(s);
    assert!(datetime.is_err());

    // invalid day
    let s = "2222-12-32T23:59:59Z";
    let datetime = DateTime::parse_from_rfc3339(s);
    assert!(datetime.is_err());

    // invalid hour
    let s = "2222-12-31T24:59:59Z";
    let datetime = DateTime::parse_from_rfc3339(s);
    assert!(datetime.is_err());

    // invalid minute
    let s = "2222-12-31T23:60:59Z";
    let datetime = DateTime::parse_from_rfc3339(s);
    assert!(datetime.is_err());

    // invalid second
    let s = "2222-12-31T23:59:60Z";
    let datetime = DateTime::parse_from_rfc3339(s);
    assert!(datetime.is_err());

    let s = "2019-10-12T14:20:50.954585101Z12312312+07:00";
    let datetime = DateTime::parse_from_rfc3339(s);
    assert!(datetime.is_err());

    let s = "2019-10-12T14:20:50.hh954585101Z12312312+07:00";
    let datetime = DateTime::parse_from_rfc3339(s);
    assert!(datetime.is_err());

    let s = "  ";
    let datetime = DateTime::parse_from_rfc3339(s);
    assert!(datetime.is_err());

    let s = "22t2-31T23:59:20Z";
    let datetime = DateTime::parse_from_rfc3339(s);
    assert!(datetime.is_err());

    for year in 1870..2200 {
        if !is_leap_year(year) {
            let s = &format!("{year}-02-29T08:59:51Z");
            let datetime = DateTime::parse_from_rfc3339(s);
            assert!(datetime.is_err());
        }
    }
}

/// SDV test for ylong_time::DateTime::parse_from_rfc3339()
///
/// # Title
/// sdv_parse_valid_date_string
///
/// # Brief
/// Test whether the output is as expected and desired when valid input is provided.
#[test]
fn sdv_parse_valid_date_string() {
    for y in 1870..2200 {
        for m in 1..13 {
            for d in 1..32 {
                if m == 2 && (is_leap_year(y) && d > 29 || !is_leap_year(y) && d > 28) {
                    continue;
                }
                if (m == 4 || m == 6 || m == 9 || m == 11) && d > 30 {
                    continue;
                }
                let s = &format!("{y}-{m:02}-{d:02}T17:01:51Z");
                let datetime = DateTime::parse_from_rfc3339(s);
                assert!(datetime.is_ok());
            }
        }
    }
}

/// SDV test for ylong_time::DateTime::to_rfc3339()
///
/// # Title
/// sdv_to_rfc3339
///
/// # Brief
/// Test whether the output is as expected and desired when valid input is provided.
#[test]
fn sdv_to_rfc3339() {
    for y in 2001..2200 {
        for m in 1..13 {
            for d in 1..32 {
                if m == 2 && (is_leap_year(y) && d > 29 || !is_leap_year(y) && d > 28) {
                    continue;
                }
                if (m == 4 || m == 6 || m == 9 || m == 11) && d > 30 {
                    continue;
                }
                let s = &format!("{y}-{m:02}-{d:02}T17:01:51+03:00");
                let datetime = DateTime::parse_from_rfc3339(s).unwrap();
                assert_eq!(s, datetime.to_rfc3339().as_str());
            }
        }
    }
}

/// SDV test for ylong_time::fixed::FixedOffset
///
/// # Title
/// sdv_fixedoffest_test
///
/// # Brief
/// Test whether the method performs as expected.
#[test]
fn sdv_fixedoffest_test() {
    for i in 0..86400 {
        let fixed_offset = FixedOffset::east(i).unwrap();
        assert_eq!(fixed_offset.local_minus_utc(), i);
        assert_eq!(fixed_offset.utc_minus_local(), -i);
        let fixed_offset = FixedOffset::west(i).unwrap();
        assert_eq!(fixed_offset.local_minus_utc(), -i);
        assert_eq!(fixed_offset.utc_minus_local(), i);
    }
}

/// SDV test for ylong_time::WeekDay
///
/// # Title
/// sdv_datetime_weekday_leap_year_test
///
/// # Brief
/// Test if the method handles leap year properly.
#[test]
fn sdv_datetime_weekday_leap_year_test() {
    let leap_years = [
        1980, 1984, 1988, 1992, 1996, 2000, 2004, 2008, 2012, 2016, 2020, 2024, 2028, 2032, 2036,
        2040, 2044, 2048, 2052, 2056, 2060, 2064, 2068, 2072, 2076, 2080, 2084, 2088, 2092, 2096,
    ];

    let weekdays = [
        Weekday::Friday,    // 1980
        Weekday::Wednesday, // 1984
        Weekday::Monday,    // 1988
        Weekday::Saturday,  // 1992
        Weekday::Thursday,  // 1996
        Weekday::Tuesday,   // 2000
        Weekday::Sunday,    // 2004
        Weekday::Friday,    // 2008
        Weekday::Wednesday, // 2012
        Weekday::Monday,    // 2016
        Weekday::Saturday,  // 2020
        Weekday::Thursday,  // 2024
        Weekday::Tuesday,   // 2028
        Weekday::Sunday,    // 2032
        Weekday::Friday,    // 2036
        Weekday::Wednesday, // 2040
        Weekday::Monday,    // 2044
        Weekday::Saturday,  // 2048
        Weekday::Thursday,  // 2052
        Weekday::Tuesday,   // 2056
        Weekday::Sunday,    // 2060
        Weekday::Friday,    // 2064
        Weekday::Wednesday, // 2068
        Weekday::Monday,    // 2072
        Weekday::Saturday,  // 2076
        Weekday::Thursday,  // 2080
        Weekday::Tuesday,   // 2084
        Weekday::Sunday,    // 2088
        Weekday::Friday,    // 2092
        Weekday::Wednesday, // 2096
    ];

    for (i, year) in leap_years.iter().enumerate() {
        let s = &format!("{year}-02-29T17:01:51Z");
        let datetime = DateTime::parse_from_rfc3339(s).unwrap();
        assert_eq!(datetime.weekday(), weekdays[i])
    }
}

/// SDV test for ylong_time::datetime::DateTime::from_utc()
///
/// # Title
/// sdv_datetime_from_utc_leap_year_test
///
/// # Brief
/// Test if the method handles leap year properly.
#[test]
fn sdv_datetime_from_utc_leap_year_test() {
    for y in 1870..2200 {
        let m = 2;
        let d = 29;
        if is_leap_year(y) {
            let basedatetime = BaseDateTime::new(
                BaseDate::from_ymd(y, m, d).unwrap(),
                BaseTime::from_hms(11, 12, 13).unwrap(),
            );
            let datetime = DateTime::<Utc>::from_utc(basedatetime, Utc);
            let datetime_to_compare = Utc.from_local_datetime(&basedatetime);
            assert_eq!(datetime, datetime_to_compare);
        }
    }
}

/// SDV test for ylong_time::datetime::DateTime::timestamp()
///
/// # Title
/// sdv_datetime_timestamp
///
/// # Brief
/// Test whether the output is as expected and desired when valid input is provided.
#[test]
fn sdv_datetime_timestamp() {
    for sec in 0..10000000 {
        let datetime: DateTime<Utc> =
            DateTime::from_utc(BaseDateTime::from_timestamp(sec, 0).unwrap(), Utc);
        assert_eq!(datetime.timestamp().unwrap(), sec);
    }
}

/// SDV test for ylong_time::datetime::DateTime::timestamp_millis()
///
/// # Title
/// sdv_datetime_timestamp_millis
///
/// # Brief
/// Test whether the output is as expected and desired when valid input is provided.
#[test]
fn sdv_datetime_timestamp_millis() {
    for sec in 0..10000000 {
        let datetime: DateTime<Utc> =
            DateTime::from_utc(BaseDateTime::from_timestamp(sec, 0).unwrap(), Utc);

        assert_eq!(
            datetime.timestamp().unwrap() * 1000,
            datetime.timestamp_millis().unwrap()
        );
    }
}

/// SDV test for ylong_time::datetime::DateTime::timestamp_micros()
///
/// # Title
/// sdv_datetime_timestamp_micros
///
/// # Brief
/// Test whether the output is as expected and desired when valid input is provided.
#[test]
fn sdv_datetime_timestamp_micros() {
    for sec in 0..10000000 {
        let datetime: DateTime<Utc> =
            DateTime::from_utc(BaseDateTime::from_timestamp(sec, 0).unwrap(), Utc);

        assert_eq!(
            datetime.timestamp_millis().unwrap() * 1000,
            datetime.timestamp_micros().unwrap()
        );
    }
}

/// SDV test for ylong_time::datetime::DateTime::date_base()
///
/// # Title
/// sdv_datetime_date_base
///
/// # Brief
/// Test whether the output is as expected and desired when valid input is provided.
#[test]
fn sdv_datetime_date_base() {
    for y in 1870..2200 {
        for m in 1..13 {
            for d in 1..32 {
                if m == 2 && (is_leap_year(y) && d > 29 || !is_leap_year(y) && d > 28) {
                    continue;
                }
                if (m == 4 || m == 6 || m == 9 || m == 11) && d > 30 {
                    continue;
                }
                let date = Utc.ymd(y, m, d).unwrap();
                let datetime = date.and_hms_nano(0, 0, 0, 0).unwrap();
                let datetime_to_compare = FixedOffset::east(0)
                    .unwrap()
                    .ymd(y, m, d)
                    .unwrap()
                    .and_hms(0, 0, 0)
                    .unwrap();
                assert_eq!(datetime.date_base(), datetime_to_compare.date_base());
            }
        }
    }
}

/// SDV test for ylong_time::datetime::DateTime::timezone()
///
/// # Title
/// sdv_datetime_timezone
///
/// # Brief
/// Test whether the output is as expected and desired when valid input is provided.
#[test]
fn sdv_datetime_timezone() {
    for sec in 0..86400 {
        let datetime: DateTime<FixedOffset> = DateTime::from_utc(
            BaseDateTime::from_timestamp(0, 0).unwrap(),
            FixedOffset::east(sec).unwrap(),
        );
        assert_eq!(datetime.timezone(), FixedOffset::east(sec).unwrap());
    }
}

/// SDV test for ylong_time::datetime::DateTime::from_local()
///
/// # Title
/// sdv_datetime_from_local_leap_year_test
///
/// # Brief
/// Test if the method handles leap year properly.
#[test]
fn sdv_datetime_from_local_leap_year_test() {
    for y in 1870..2200 {
        let m = 2;
        let d = 29;
        if is_leap_year(y) {
            let basedatetime = BaseDateTime::new(
                BaseDate::from_ymd(y, m, d).unwrap(),
                BaseTime::from_hms(11, 12, 13).unwrap(),
            );
            let datetime = DateTime::<Utc>::from_local(basedatetime, Utc);
            let datetime_to_compare = Utc.from_local_datetime(&basedatetime);
            assert_eq!(datetime, datetime_to_compare);
        }
    }
}

/// SDV test for ylong_time::datetime::DateTime::time()
///
/// # Title
/// sdv_datetime_time
///
/// # Brief
/// Test whether the output is as expected and desired when valid input is provided.
#[test]
fn sdv_datetime_time() {
    for sec in 0..86400 {
        let base_datetime = BaseDateTime::from_timestamp(sec, 0).unwrap();
        let datetime: DateTime<FixedOffset> = DateTime::from_utc(
            BaseDateTime::from_timestamp(0, 0).unwrap(),
            FixedOffset::east(sec.try_into().unwrap()).unwrap(),
        );
        assert_eq!(datetime.time(), base_datetime.time());
    }
}

/// SDV test for ylong_time::datetime::DateTime::base_local()
///
/// # Title
/// sdv_datetime_from_local
///
/// # Brief
/// Test whether the output is as expected and desired when valid input is provided.
#[test]
fn sdv_datetime_base_local() {
    for y in 2001..2200 {
        for m in 1..13 {
            for d in 1..32 {
                if m == 2 && (is_leap_year(y) && d > 29 || !is_leap_year(y) && d > 28) {
                    continue;
                }
                if (m == 4 || m == 6 || m == 9 || m == 11) && d > 30 {
                    continue;
                }
                let basedatetime = BaseDateTime::new(
                    BaseDate::from_ymd(y, m, d).unwrap(),
                    BaseTime::from_hms(11, 12, 13).unwrap(),
                );
                let datetime = DateTime::<Utc>::from_local(basedatetime, Utc);
                assert_eq!(datetime.base_local(), basedatetime);
            }
        }
    }
}

/// SDV test for ylong_time::datetime::DateTime::date()
///
/// # Title
/// sdv_datetime_date_leap_year_test
///
/// # Brief
/// Test if the method handles leap year properly.
#[test]
fn sdv_datetime_date_leap_year_test() {
    for y in 2001..2200 {
        for m in 1..13 {
            for d in 28..32 {
                if m == 2 && (is_leap_year(y) && d > 29 || !is_leap_year(y) && d > 28) {
                    continue;
                }
                if (m == 4 || m == 6 || m == 9 || m == 11) && d > 30 {
                    continue;
                }
                let date = Utc.ymd(y, m, d).unwrap();
                let datetime = date.and_hms_nano(0, 0, 0, 0).unwrap();
                assert_eq!(date, datetime.date());
            }
        }
    }
}

/// SDV test for ylong_time::offset::utc::Utc::now()
/// SDV test for ylong_time::offset::utc::Utc::today()
///
/// # Title
/// sdv_utc_now_and_today
///
/// # Brief
/// The date_base of now and today should be the same.
#[test]
fn sdv_utc_now_and_today() {
    let now = Utc::now().unwrap();
    let today = Utc::today().unwrap().and_hms(0, 0, 0).unwrap();
    assert_eq!(now.date_base(), today.date_base());
}

/// SDV test for ylong_time::offset::local::Local::now()
/// SDV test for ylong_time::offset::local::Local::today()
///
/// # Title
/// sdv_local_now_and_today
///
/// # Brief
/// The date_base of now and today should be the same.
#[test]
fn sdv_local_now_and_today() {
    let now = Local::now().unwrap();
    let today = Local::today().unwrap().and_hms(0, 0, 0).unwrap();
    assert_eq!(now.date_base(), today.date_base());
}

/// SDV test for ylong_time::datetime::DateTime::offset()
///
/// # Title
/// sdv_datetime_offset
///
/// # Brief
/// Test whether the output is as expected and desired when valid input is provided.
#[test]
fn sdv_datetime_offset() {
    for sec in 0..86400 {
        let datetime: DateTime<FixedOffset> = DateTime::from_utc(
            BaseDateTime::from_timestamp(0, 0).unwrap(),
            FixedOffset::east(sec).unwrap(),
        );
        assert_eq!(*datetime.offset(), FixedOffset::east(sec).unwrap());
    }
}
