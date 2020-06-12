use chrono::NaiveDateTime;
use dtparse::Parser;
use std::collections::HashMap;

pub fn parse_datetime(date_time: String) -> Option<NaiveDateTime> {
    // HACK: Date parsing is hard!
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
        .ok();
}
