use anyhow::Ok;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

#[test]
fn spaces() {
    let result = Grammar::parse(Rule::spaces, "  ");
    assert!(result.is_ok() == true)
}

#[test]
fn digit() {
    let result = Grammar::parse(Rule::digit, "1");
    assert!(result.is_ok() == true)
}

#[test]
fn second() {
    let result = Grammar::parse(Rule::second, "12");
    assert!(result.is_ok() == true)
}

#[test]
fn minute() {
    let result = Grammar::parse(Rule::minute, "35");
    assert!(result.is_ok() == true)
}

#[test]
fn hour() {
    let result = Grammar::parse(Rule::hour, "17");
    assert!(result.is_ok() == true)
}

#[test]
fn time() {
    let result = Grammar::parse(Rule::time, "17:45:17");
    assert!(result.is_ok() == true)
}

#[test]
fn day() {
    let result = Grammar::parse(Rule::day, "17");
    assert!(result.is_ok() == true)
}

#[test]
fn month() {
    let result = Grammar::parse(Rule::month, "11");
    assert!(result.is_ok() == true)
}

#[test]
fn year() {
    let result = Grammar::parse(Rule::year, "2019");
    assert!(result.is_ok() == true)
}

#[test]
fn date() {
    let result = Grammar::parse(Rule::date, "28.12.2029");
    assert!(result.is_ok() == true)
}

#[test]
fn date_time() {
    let result = Grammar::parse(Rule::date_time, "06.11.2024 19:34:00");
    assert!(result.is_ok() == true)
}
