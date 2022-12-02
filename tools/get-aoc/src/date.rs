use chrono::{Datelike, Local};

pub fn this_year() -> u16 {
    let local = Local::now();
    local.year() as u16
}

pub fn day_of_month() -> u8 {
    let local = Local::now();
    local.day() as u8
}
