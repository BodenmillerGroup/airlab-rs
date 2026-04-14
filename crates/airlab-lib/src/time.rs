#![allow(clippy::module_name_repetitions)]
use time::{Duration, OffsetDateTime};

pub use time::format_description::well_known::Rfc3339;

#[must_use]
pub fn now_utc() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

pub fn format_time(time: OffsetDateTime) -> Result<String> {
    time.format(&Rfc3339)
        .map_err(|_| Error::FailToTimeFormat(time.unix_timestamp()))
}

pub fn now_utc_plus_sec_str(sec: f64) -> Result<String> {
    let new_time = now_utc() + Duration::seconds_f64(sec);
    format_time(new_time)
}

pub fn parse_utc(moment: &str) -> Result<OffsetDateTime> {
    OffsetDateTime::parse(moment, &Rfc3339).map_err(|_| Error::FailToDateParse(moment.to_string()))
}

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    FailToDateParse(String),
    FailToTimeFormat(i64),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
