#[test]
fn standard_date() {
    let dt = crate::Utc::from_ymdhms(2024, 1, 5, 12, 0, 0_f64);
    assert_eq!("2024-01-05T12:00:00", dt.as_rfc3339());
}

#[test]
fn on_leap_day() {
    let dt = crate::Utc::from_ymdhms(2000, 2, 29, 12, 0, 0_f64);
    assert_eq!("2000-02-29T12:00:00", dt.as_rfc3339());
}

#[test]
fn on_leap_day_non_leap_year() {
    let dt = crate::Utc::from_ymdhms(2001, 2, 29, 12, 0, 0_f64);
    assert_eq!("2001-03-01T12:00:00", dt.as_rfc3339());
}

#[test]
fn underflow_month() {
    let dt = crate::Utc::from_ymd(2020, 0, 1);
    assert_eq!("2019-12-01T00:00:00", dt.as_rfc3339());
}

#[test]
fn underflow_day() {
    let dt = crate::Utc::from_ymd(2020, 1, 0);
    assert_eq!("2019-12-31T00:00:00", dt.as_rfc3339());
}

#[test]
fn underflow_month_and_day() {
    let dt = crate::Utc::from_ymd(2020, 0, 0);
    assert_eq!("2019-11-30T00:00:00", dt.as_rfc3339());
}

#[test]
fn overflow_month() {
    let dt = crate::Utc::from_ymd(2020, 13, 1);
    assert_eq!("2021-01-01T00:00:00", dt.as_rfc3339());
}

#[test]
fn overflow_day() {
    let dt = crate::Utc::from_ymd(2020, 12, 32);
    assert_eq!("2021-01-01T00:00:00", dt.as_rfc3339());
}

#[test]
fn overflow_day_and_month() {
    let dt = crate::Utc::from_ymd(2020, 13, 32);
    assert_eq!("2021-02-01T00:00:00", dt.as_rfc3339());
}

#[test]
fn overflow_hour() {
    let dt = crate::Utc::from_ymdhms(2018, 11, 11, 24, 0, 0_f64);
    assert_eq!("2018-11-12T00:00:00", dt.as_rfc3339());
}

#[test]
fn overflow_hour_and_day() {
    let dt = crate::Utc::from_ymdhms(2018, 11, 31, 24, 0, 0_f64);
    assert_eq!("2018-12-02T00:00:00", dt.as_rfc3339());
}

#[test]
fn overflow_hour_day_and_month() {
    let dt = crate::Utc::from_ymdhms(2018, 13, 32, 24, 0, 0_f64);
    assert_eq!("2019-02-02T00:00:00", dt.as_rfc3339());
}

#[test]
fn overflow_minute() {
    let dt = crate::Utc::from_ymdhms(2017, 12, 10, 12, 60, 0_f64);
    assert_eq!("2017-12-10T13:00:00", dt.as_rfc3339());
}

#[test]
fn overflow_second() {
    let dt = crate::Utc::from_ymdhms(2017, 12, 10, 12, 60, 0_f64);
    assert_eq!("2017-12-10T13:00:00", dt.as_rfc3339());
}

#[test]
fn day_of_week() {
    let dt = crate::Utc::from_ymd(2024, 1, 5);
    assert_eq!("Friday", dt.day_of_week());
}

#[test]
fn day_of_week_of_epoch() {
    let dt = crate::Utc::from_ymd(1970, 1, 1);
    assert_eq!("Thursday", dt.day_of_week());
}

#[test]
fn day_of_week_day_after_epoch() {
    let dt = crate::Utc::from_ymd(1970, 1, 2);
    assert_eq!("Friday", dt.day_of_week());
}

#[test]
fn day_of_week_two_days_after_epoch() {
    let dt = crate::Utc::from_ymd(1970, 1, 3);
    assert_eq!("Saturday", dt.day_of_week());
}

#[test]
fn day_of_week_six_days_after_epoch() {
    let dt = crate::Utc::from_ymd(1970, 1, 7);
    assert_eq!("Wednesday", dt.day_of_week());
}

#[test]
fn day_of_week_last_day_of_month_of_epoch() {
    let dt = crate::Utc::from_ymd(1970, 1, 31);
    assert_eq!("Saturday", dt.day_of_week());
}

#[test]
fn day_of_week_one_month_after_epoch() {
    let dt = crate::Utc::from_ymd(1970, 2, 1);
    assert_eq!("Sunday", dt.day_of_week());
}

#[test]
fn day_of_week_one_year_after_epoch() {
    let dt = crate::Utc::from_ymd(1971, 1, 1);
    assert_eq!("Friday", dt.day_of_week());
}

#[test]
fn month_of_year() {
    let dt = crate::Utc::from_ymd(2024, 1, 5);
    assert_eq!("January", dt.month_of_year());
}

#[test]
fn month_of_year_one_month_after_epoch() {
    let dt = crate::Utc::from_ymd(2024, 2, 1);
    println!("{}", dt.as_rfc3339());
    assert_eq!("February", dt.month_of_year());
}

#[test]
fn with_nanos_2_precision() {
    let dt = crate::Utc::from_ymdhms(2024, 1, 5, 5, 34, 12.324_f64);
    assert_eq!("2024-01-05T05:34:12.32", dt.as_rfc3339_with_nano(2));
}

#[test]
fn rfc7231() {
    let dt = crate::Utc::from_ymdhms(2024, 1, 5, 11, 44, 58.0_f64);
    assert_eq!("Fri, 05 Jan 2024 11:44:58 GMT", dt.as_rfc7231());
}

#[test]
#[should_panic]
fn before_epoch() {
    crate::Utc::from_ymdhms(1969, 12, 31, 11, 59, 59.99_f64);
}

#[test]
#[should_panic]
fn before_epoch_underflow_month() {
    crate::Utc::from_ymd(1970, 0, 31);
}

#[test]
#[should_panic]
fn before_epoch_underflow_day() {
    crate::Utc::from_ymd(1970, 1, 0);
}

#[test]
fn all_days_since_epoch() {
    let start = crate::Utc::epoch();
    let mut day = start.day();
    let now = crate::Utc::now();
    let end = crate::Utc::from_ymd(now.year(), now.month(), now.day());

    loop {
        let curr = crate::Utc::from_ymd(start.year(), start.month(), day);

        if curr == end {
            break;
        }

        day += 1;
    }
}

