use chrono::NaiveDateTime;
use dtparse::Parser;
use std::collections::HashMap;

pub fn parse_datetime(date_time: String) -> Option<NaiveDateTime> {
    return match Parser::default().parse(
        &date_time,
        None,
        None,
        true,
        false,
        None,
        false,
        &HashMap::new(),
    ) {
        Ok((dt, _, _)) => Some(dt),
        Err(_) => None,
    };
}
