use chrono::{Datelike, Local, NaiveDateTime};
use dtparse::Parser;
use std::cmp::Ord;
use std::collections::HashMap;

#[inline]
fn is_between<T: Ord>(i: T, min: T, max: T) -> bool {
    return i >= min && i <= max;
}

pub fn parse_datetime(date_time: String) -> Option<NaiveDateTime> {
    return Parser::default()
        .parse(
            &date_time,
            None,
            None,
            true,
            false,
            None,
            true,
            &HashMap::new(),
        )
        .map(|(dt, _, _)| dt)
        .ok()
        .and_then(validate_datetime);
}

fn validate_datetime(date_time: NaiveDateTime) -> Option<NaiveDateTime> {
    if !is_between(date_time.year(), 1900, Local::today().year()) {
        return None;
    }
    return Some(date_time);
}

#[cfg(test)]
mod tests {

    use super::*;
    use chrono::NaiveDate;

    fn make_datetime(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> NaiveDateTime {
        return NaiveDate::from_ymd(year, month, day).and_hms(hour, minute, second);
    }

    #[test]
    fn test_is_between() {
        assert!(is_between(2, 1, 3));
        assert!(is_between(200, 100, 300));
        assert!(is_between(100, 100, 100));
        assert!(is_between(-10, -100, 100));
    }

    #[test]
    fn test_parse_datetime() {
        assert!(parse_datetime("foobar".to_string()).is_none());
        assert_eq!(
            parse_datetime("2020-01-01".to_string()).unwrap(),
            make_datetime(2020, 1, 1, 0, 0, 0)
        );
        assert_eq!(
            parse_datetime("image 2020-01-01 03:04:05".to_string()).unwrap(),
            make_datetime(2020, 1, 1, 3, 4, 5)
        );
    }

    #[test]
    fn test_validate_datetime() {
        assert!(validate_datetime(make_datetime(2020, 1, 1, 3, 4, 5)).is_some());
        assert!(validate_datetime(make_datetime(1820, 1, 1, 3, 4, 5)).is_none());
    }
}
