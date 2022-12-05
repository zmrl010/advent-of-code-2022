use chrono::{Datelike, Local};

pub type Year = u16;
pub type Day = u8;

pub fn this_year() -> Year {
    let local = Local::now();
    local.year() as Year
}

pub fn day_of_month() -> Day {
    let local = Local::now();
    local.day() as Day
}
