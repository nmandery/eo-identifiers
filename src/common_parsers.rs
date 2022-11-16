use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case, take_while, take_while_m_n};
use nom::character::{is_alphanumeric, is_digit};
use nom::combinator::{map, opt};
use nom::error::Error;
use nom::sequence::tuple;
use nom::{Err, IResult};
use num_traits::PrimInt;
use std::fmt::Debug;
use std::str::FromStr;

pub(crate) fn is_char_alphanumeric(chr: char) -> bool {
    chr.is_ascii() && is_alphanumeric(chr as u8)
}

pub(crate) fn take_alphanumeric(i: &str) -> IResult<&str, &str> {
    take_while(is_char_alphanumeric)(i)
}

pub fn take_alphanumeric_n(n: usize) -> impl Fn(&str) -> IResult<&str, &str> {
    move |i: &str| take_while_m_n(n, n, is_char_alphanumeric)(i)
}

fn is_char_digit(chr: char) -> bool {
    chr.is_ascii() && is_digit(chr as u8)
}

/// taken and modified from https://github.com/badboy/iso8601/blob/main/src/parsers.rs
pub(crate) fn take_n_digits<T>(n: usize) -> impl Fn(&str) -> IResult<&str, T>
where
    T: FromStr + PrimInt,
    <T as FromStr>::Err: Debug,
{
    move |i: &str| {
        let (i, digits) = take_while_m_n(n, n, is_char_digit)(i)?;

        let res = digits
            .parse()
            .expect("Invalid string, expected ASCII representation of a number");

        Ok((i, res))
    }
}

/// taken and modified from https://github.com/badboy/iso8601/blob/main/src/parsers.rs
pub fn take_n_digits_in_range<T>(
    n: usize,
    range: impl core::ops::RangeBounds<T>,
) -> impl Fn(&str) -> IResult<&str, T>
where
    T: FromStr + PrimInt,
    <T as FromStr>::Err: Debug,
{
    move |i: &str| {
        let (new_i, number) = take_n_digits(n)(i)?;
        if range.contains(&number) {
            Ok((new_i, number))
        } else {
            Err(Err::Error(Error::new(i, nom::error::ErrorKind::Eof)))
        }
    }
}

/// taken and modified from https://github.com/badboy/iso8601/blob/main/src/parsers.rs
fn sign(i: &str) -> IResult<&str, i32> {
    map(alt((tag("-"), tag("+"))), |s: &str| match s {
        "-" => -1,
        _ => 1,
    })(i)
}

/// taken and modified from https://github.com/badboy/iso8601/blob/main/src/parsers.rs
// [+/-]YYYY
pub(crate) fn date_year(i: &str) -> IResult<&str, i32> {
    // The sign is optional, but defaults to `+`
    map(
        tuple((
            opt(sign),               // [+/-]
            take_n_digits::<u32>(4), // year
        )),
        |(s, year)| s.unwrap_or(1) * year as i32,
    )(i)
}

/// taken and modified from https://github.com/badboy/iso8601/blob/main/src/parsers.rs
// MM
fn date_month(i: &str) -> IResult<&str, u32> {
    take_n_digits_in_range(2, 1..=12)(i)
}

/// taken and modified from https://github.com/badboy/iso8601/blob/main/src/parsers.rs
// DD
fn date_day(i: &str) -> IResult<&str, u32> {
    take_n_digits_in_range(2, 1..=31)(i)
}

/// taken and modified from https://github.com/badboy/iso8601/blob/main/src/parsers.rs
// HH
fn time_hour(i: &str) -> IResult<&str, u32> {
    take_n_digits_in_range(2, 0..=24)(i)
}

/// taken and modified from https://github.com/badboy/iso8601/blob/main/src/parsers.rs
// MM
fn time_minute(i: &str) -> IResult<&str, u32> {
    take_n_digits_in_range(2, 0..=59)(i)
}

/// taken and modified from https://github.com/badboy/iso8601/blob/main/src/parsers.rs
// SS
fn time_second(i: &str) -> IResult<&str, u32> {
    take_n_digits_in_range(2, 0..=60)(i)
}

fn t_separator(i: &str) -> IResult<&str, ()> {
    map(tag_no_case("t"), |_| ())(i)
}

pub(crate) fn parse_simple_date(s: &str) -> IResult<&str, NaiveDate> {
    map(tuple((date_year, date_month, date_day)), |(y, m, d)| {
        NaiveDate::from_ymd(y, m, d)
    })(s)
}

pub(crate) fn parse_simple_time(s: &str) -> IResult<&str, NaiveTime> {
    map(
        tuple((time_hour, time_minute, time_second)),
        |(h, mn, s)| NaiveTime::from_hms(h, mn, s),
    )(s)
}

pub(crate) fn parse_esa_timestamp(s: &str) -> IResult<&str, NaiveDateTime> {
    map(
        tuple((parse_simple_date, opt(t_separator), parse_simple_time)),
        |(date, _, time)| NaiveDateTime::new(date, time),
    )(s)
}

#[cfg(test)]
mod tests {
    use crate::common_parsers::parse_esa_timestamp;
    use chrono::{Datelike, Timelike};

    #[test]
    fn parse_esa_timestamp_with_t() {
        let (_, ts) = parse_esa_timestamp("20200207T051836").unwrap();
        assert_eq!(ts.year(), 2020);
        assert_eq!(ts.month(), 2);
        assert_eq!(ts.day(), 7);
        assert_eq!(ts.hour(), 5);
        assert_eq!(ts.minute(), 18);
        assert_eq!(ts.second(), 36);
    }

    #[test]
    fn parse_esa_timestamp_without_t() {
        let (_, ts) = parse_esa_timestamp("20200207051836").unwrap();
        assert_eq!(ts.year(), 2020);
        assert_eq!(ts.month(), 2);
        assert_eq!(ts.day(), 7);
        assert_eq!(ts.hour(), 5);
        assert_eq!(ts.minute(), 18);
        assert_eq!(ts.second(), 36);
    }
}
