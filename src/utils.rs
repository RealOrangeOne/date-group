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
    if !is_between(date_time.year(), 1960, Local::today().year()) {
        return None;
    }
    return Some(date_time);
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_is_between() {
        assert!(is_between(2, 1, 3));
        assert!(is_between(200, 100, 300));
        assert!(is_between(100, 100, 100));
        assert!(is_between(-10, -100, 100));
    }
}
