use chrono::NaiveDateTime;
use dtparse::Parser;
use std::collections::HashMap;
use std::panic;

pub fn parse_datetime(date_time: String) -> Option<NaiveDateTime> {
    // HACK: Date parsing is hard!
    let parse_match = panic::catch_unwind(|| {
        return Parser::default().parse(
            &date_time,
            None,
            None,
            true,
            false,
            None,
            true,
            &HashMap::new(),
        );
    });
    if parse_match.is_err() {
        eprintln!(
            "This panic was caused by attempting to parse '{}'.\n\n\n",
            date_time
        );
        return None;
    }
    return match parse_match.ok()? {
        Ok((dt, _, _)) => Some(dt),
        Err(_) => None,
    };
}
