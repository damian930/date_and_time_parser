use damians_custom_datetime_parser::*;
use pest::Parser;

#[test]
fn parse_spaces_valid() {
    let result = DateTimeParser::parse(Rule::spaces, "  ");
    assert!(
        result.is_ok(),
        "Something went wrong while parsing empty symbols using Rule::spaces"
    );
}

#[test]
fn parse_spaces_invalid() {
    let result = DateTimeParser::parse(Rule::spaces, "g  ");
    assert!(result.is_err(), "Something went wrong while parsing letters using Rule::spaces, was supposed to return DateTimeError");
}

#[test]
fn parse_digit_valid() {
    let result = DateTimeParser::parse(Rule::digit, "1");
    assert!(
        result.is_ok(),
        "Something went wrong while parsing a digit using Rule::digit"
    );
}

#[test]
fn parse_digit_invalid() {
    let result = DateTimeParser::parse(Rule::digit, "g");
    assert!(result.is_err(), "Something went wrong while parsing a letters using Rule::digit, was supposed to return DateTimeError");
}

#[test]
fn parse_second_valid() {
    let result = DateTimeParser::parse(Rule::second, "12");
    assert!(
        result.is_ok(),
        "Something went wrong while parsing a second using Rule::second"
    );
}

#[test]
fn parse_second_invalid() {
    let result = DateTimeParser::parse(Rule::second, "1");
    assert!(result.is_err(), "Something went wrong while parsing a second using Rule::second, was supposed to return DateTimeError");
}

#[test]
fn parse_minute_valid() {
    let result = DateTimeParser::parse(Rule::minute, "12");
    assert!(
        result.is_ok(),
        "Something went wrong while parsing a minute using Rule::minute"
    );
}

#[test]
fn parse_minute_invalid() {
    let result = DateTimeParser::parse(Rule::minute, "1");
    assert!(result.is_err(), "Something went wrong while parsing a minute using Rule::minute, was supposed to return DateTimeError");
}

#[test]
fn parse_hour_valid() {
    let result = DateTimeParser::parse(Rule::hour, "12");
    assert!(
        result.is_ok(),
        "Something went wrong while parsing an hour using Rule::hour"
    );
}

#[test]
fn parse_hour_invalid() {
    let result = DateTimeParser::parse(Rule::hour, "1");
    assert!(result.is_err(), "Something went wrong while parsing an hour using Rule::hour, was supposed to return DateTimeError");
}

#[test]
fn parse_time_valid() {
    let result = DateTimeParser::parse(Rule::time, "12:49:00");
    assert!(
        result.is_ok(),
        "Something went wrong while parsing time using Rule::time"
    );
}

#[test]
fn parse_time_invalid() {
    let result = DateTimeParser::parse(Rule::time, "12:11 00");
    assert!(result.is_err(), "Something went wrong while parsing time using Rule::time, was supposed to return DateTimeError");
}

#[test]
fn parse_day_valid() {
    let result = DateTimeParser::parse(Rule::day, "17");
    assert!(
        result.is_ok(),
        "Something went wrong while parsing a day using Rule::day"
    );
}

#[test]
fn parse_day_invalid() {
    let result = DateTimeParser::parse(Rule::day, "1");
    assert!(result.is_err(), "Something went wrong while parsing a day using Rule::day, was supposed to return DateTimeError");
}

#[test]
fn parse_month_valid() {
    let result = DateTimeParser::parse(Rule::month, "01");
    assert!(
        result.is_ok(),
        "Something went wrong while parsing a month using Rule::month"
    );
}

#[test]
fn parse_month_invalid() {
    let result = DateTimeParser::parse(Rule::month, "1");
    assert!(result.is_err(), "Something went wrong while parsing a month using Rule::month, was supposed to return DateTimeError");
}

#[test]
fn parse_year_valid() {
    let result = DateTimeParser::parse(Rule::year, "2019");
    assert!(
        result.is_ok(),
        "Something went wrong while parsing a year using Rule::year"
    );
}

#[test]
fn parse_year_invalid() {
    let result = DateTimeParser::parse(Rule::year, "999");
    assert!(result.is_err(), "Something went wrong while parsing a year using Rule::year, was supposed to return DateTimeError");
}

#[test]
fn parse_date_valid() {
    let result = DateTimeParser::parse(Rule::date, "19.11.2024");
    assert!(
        result.is_ok(),
        "Something went wrong while parsing a date using Rule::date"
    );
}

#[test]
fn parse_date_invalid() {
    let result = DateTimeParser::parse(Rule::date, "999");
    assert!(result.is_err(), "Something went wrong while parsing a date using Rule::date, was supposed to return DateTimeError")
}

#[test]
fn parse_date_time_valid() {
    let result = DateTimeParser::parse(Rule::date_time, "19.11.2024  12:56:00");
    assert!(
        result.is_ok(),
        "Something went wrong_1 while parsing date_time using Rule::date_time"
    );
}

#[test]
fn parse_date_time_invalid() {
    let result = DateTimeParser::parse(Rule::date_time, "19.11.2024_12:56:00");
    assert!(result.is_err(), "Something went wrong while parsing date_time using Rule::date_time, was supposed to return DateTimeError")
}
