use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

 // strcture for parsing using pest and grammar inside #[grammar...]
#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct DateTimeParser;

// Parsing Error type
#[derive(Debug, Error)]
pub enum DateTimeError {
    #[error("DateTime parsing error: {0}")]
    ParseError(String),
}

// Result of parsing
#[derive(Debug)]
pub struct DateTime {
    pub day: String,
    pub month: String,
    pub year: String,

    pub hours: String,
    pub minutes: String,
    pub seconds: String,
}

impl DateTime {
    pub fn from_data_time(date_time: &str) -> Result<Self, DateTimeError> {
        // parse string into pest's pairs, future unwraps are not dangerous
        let pairs = DateTimeParser::parse(Rule::date_time, date_time)
            .map_err(|_err| DateTimeError::ParseError(_err.to_string()))?;

        // { SOI ~ date ~ spaces ~ time ~ EOI }

        // access the global pair that holds date, spaces, time as it`s inside pairs
        let mut global_iter = pairs.into_iter().next().unwrap().into_inner();

        // get parsed elements: date, spaces, time
        let date_pair = global_iter.next().unwrap();
        let _ = global_iter.next(); // skipping spaces
        let time_pair = global_iter.next().unwrap();

        // get day, month, year from date_pair
        let mut date_parts_iter = date_pair.into_inner();

        let day = date_parts_iter.next().unwrap().as_str().to_string();
        let month = date_parts_iter.next().unwrap().as_str().to_string();
        let year = date_parts_iter.next().unwrap().as_str().to_string();

        // get hours, minutes, seconds from time_pair
        let mut time_parts_iter = time_pair.into_inner();

        let hours = time_parts_iter.next().unwrap().as_str().to_string();
        let minutes = time_parts_iter.next().unwrap().as_str().to_string();
        let seconds = time_parts_iter.next().unwrap().as_str().to_string();

        Ok(DateTime {
            day,
            month,
            year,
            hours,
            minutes,
            seconds,
        })
    }
}
