use crate::Weekday;

/// UT test for weekday_prev.
///
/// # Title
/// ut_weekday_prev
///
/// # Brief
/// 1. Coverage testing, testing every day of the week.
/// 2. Check if the test results are correct.
#[test]
fn ut_weekday_prev() {
    let monday = Weekday::Monday;
    let tuesday = Weekday::Tuesday;
    let wednesday = Weekday::Wednesday;
    let thursday = Weekday::Thursday;
    let friday = Weekday::Friday;
    let saturday = Weekday::Saturday;
    let sunday = Weekday::Sunday;

    assert_eq!(monday.prev(), Weekday::Sunday);
    assert_eq!(tuesday.prev(), Weekday::Monday);
    assert_eq!(wednesday.prev(), Weekday::Tuesday);
    assert_eq!(thursday.prev(), Weekday::Wednesday);
    assert_eq!(friday.prev(), Weekday::Thursday);
    assert_eq!(saturday.prev(), Weekday::Friday);
    assert_eq!(sunday.prev(), Weekday::Saturday);
}

/// UT test for weekday_next.
///
/// # Title
/// ut_weekday_next
///
/// # Brief
/// 1. Coverage testing, testing every day of the week.
/// 2. Check if the test results are correct.
#[test]
fn ut_weekday_next() {
    let monday = Weekday::Monday;
    let tuesday = Weekday::Tuesday;
    let wednesday = Weekday::Wednesday;
    let thursday = Weekday::Thursday;
    let friday = Weekday::Friday;
    let saturday = Weekday::Saturday;
    let sunday = Weekday::Sunday;

    assert_eq!(monday.next(), Weekday::Tuesday);
    assert_eq!(tuesday.next(), Weekday::Wednesday);
    assert_eq!(wednesday.next(), Weekday::Thursday);
    assert_eq!(thursday.next(), Weekday::Friday);
    assert_eq!(friday.next(), Weekday::Saturday);
    assert_eq!(saturday.next(), Weekday::Sunday);
    assert_eq!(sunday.next(), Weekday::Monday);
}

/// UT test for weekday0_to_u8.
///
/// # Title
/// ut_weekday0_to_u8
///
/// # Brief
/// 1. Coverage testing, testing every day of the week.
/// 2. Check if the test results are correct.
#[test]
fn ut_weekday0_to_u8() {
    assert_eq!(Weekday::Monday.weekday0_to_u8(), 0);
    assert_eq!(Weekday::Tuesday.weekday0_to_u8(), 1);
    assert_eq!(Weekday::Wednesday.weekday0_to_u8(), 2);
    assert_eq!(Weekday::Thursday.weekday0_to_u8(), 3);
    assert_eq!(Weekday::Friday.weekday0_to_u8(), 4);
    assert_eq!(Weekday::Saturday.weekday0_to_u8(), 5);
    assert_eq!(Weekday::Sunday.weekday0_to_u8(), 6);
}

/// UT test for weekday0_from_u8.
///
/// # Title
/// ut_weekday0_from_u8
///
/// # Brief
/// 1. Coverage testing, testing every day of the week.
/// 2. Check if the test results are correct.
#[test]
fn ut_weekday0_from_u8() {
    assert_eq!(Weekday::Monday, Weekday::weekday0_from_u8(0).unwrap());
    assert_eq!(Weekday::Tuesday, Weekday::weekday0_from_u8(1).unwrap());
    assert_eq!(Weekday::Wednesday, Weekday::weekday0_from_u8(2).unwrap());
    assert_eq!(Weekday::Thursday, Weekday::weekday0_from_u8(3).unwrap());
    assert_eq!(Weekday::Friday, Weekday::weekday0_from_u8(4).unwrap());
    assert_eq!(Weekday::Saturday, Weekday::weekday0_from_u8(5).unwrap());
    assert_eq!(Weekday::Sunday, Weekday::weekday0_from_u8(6).unwrap());
    assert!(Weekday::weekday0_from_u8(7).is_err());
}

/// UT test for weekday_to_u8.
///
/// # Title
/// ut_weekday_to_u8
///
/// # Brief
/// 1. Coverage testing, testing every day of the week.
/// 2. Check if the test results are correct.
#[test]
fn ut_weekday_to_u8() {
    assert_eq!(Weekday::Monday.weekday_to_u8(), 1);
    assert_eq!(Weekday::Tuesday.weekday_to_u8(), 2);
    assert_eq!(Weekday::Wednesday.weekday_to_u8(), 3);
    assert_eq!(Weekday::Thursday.weekday_to_u8(), 4);
    assert_eq!(Weekday::Friday.weekday_to_u8(), 5);
    assert_eq!(Weekday::Saturday.weekday_to_u8(), 6);
    assert_eq!(Weekday::Sunday.weekday_to_u8(), 7);
}

/// UT test for weekday_from_u8.
///
/// # Title
/// ut_weekday_from_u8
///
/// # Brief
/// 1. Coverage testing, testing every day of the week.
/// 2. Check if the test results are correct.
#[test]
fn ut_weekday_from_u8() {
    assert_eq!(Weekday::Monday, Weekday::weekday_from_u8(1).unwrap());
    assert_eq!(Weekday::Tuesday, Weekday::weekday_from_u8(2).unwrap());
    assert_eq!(Weekday::Wednesday, Weekday::weekday_from_u8(3).unwrap());
    assert_eq!(Weekday::Thursday, Weekday::weekday_from_u8(4).unwrap());
    assert_eq!(Weekday::Friday, Weekday::weekday_from_u8(5).unwrap());
    assert_eq!(Weekday::Saturday, Weekday::weekday_from_u8(6).unwrap());
    assert_eq!(Weekday::Sunday, Weekday::weekday_from_u8(7).unwrap());
    assert!(Weekday::weekday_from_u8(8).is_err());
}
