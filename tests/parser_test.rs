use damians_custom_datetime_parser::*;
use pest::Parser;

#[test]
fn parse_spaces_valid() -> anyhow::Result<()> {
    let inputs = vec![" ", "     "];
    for input in inputs {
        let pairs = DateTimeParser::parse(Rule::spaces, input)?;
        assert_eq!(input, pairs.as_str());
    }

    Ok(())
}

#[test]
fn parse_spaces_invalid() -> anyhow::Result<()> {
    let inputs = vec!["a ", "a     "];
    for input in inputs {
        let pairs = DateTimeParser::parse(Rule::spaces, input);
        assert!(pairs.is_err());
    }

    Ok(())
}


#[test]
fn parse_digit_valid() -> anyhow::Result<()> {
    let inputs = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"];
    for input in inputs {
        let pairs = DateTimeParser::parse(Rule::digit, input)?;
        assert_eq!(input, pairs.as_str());
    }

    Ok(())
}

#[test]
fn parse_digit_invalid() -> anyhow::Result<()> {
    let inputs = vec!["a", "/", " "];
    for input in inputs {
        let pairs = DateTimeParser::parse(Rule::digit, input);
        assert!(pairs.is_err());
    }

    Ok(())
}

#[test]
fn parse_second_valid() -> anyhow::Result<()> {
    let inputs = vec!["12", "89"];
    for input in inputs {
        let pairs = DateTimeParser::parse(Rule::second, input)?;
        assert_eq!(input, pairs.as_str());
    }

    Ok(())
}

#[test]
fn parse_second_invalid() -> anyhow::Result<()> {
    let inputs = vec!["1", "9", "a", " "];
    for input in inputs {
        let pairs = DateTimeParser::parse(Rule::second, input);
        assert!(pairs.is_err());
    }

    Ok(())
}


#[test]
fn parse_minute_valid() -> anyhow::Result<()> {
    let inputs = vec!["12", "89"];
    for input in inputs {
        let pairs = DateTimeParser::parse(Rule::minute, input)?;
        assert_eq!(input, pairs.as_str());
    }

    Ok(())
}

#[test]
fn parse_minute_invalid() -> anyhow::Result<()> {
    let inputs = vec!["1", "9", "a", " "];
    for input in inputs {
        let pairs = DateTimeParser::parse(Rule::minute, input);
        assert!(pairs.is_err());
    }

    Ok(())
}

#[test]
fn parse_hour_valid() -> anyhow::Result<()> {
    let inputs = vec!["12", "89"];
    for input in inputs {
        let pairs = DateTimeParser::parse(Rule::hour, input)?;
        assert_eq!(input, pairs.as_str());
    }

    Ok(())
}

#[test]
fn parse_hour_invalid() -> anyhow::Result<()> {
    let inputs = vec!["1", "9", "a", " "];
    for input in inputs {
        let pairs = DateTimeParser::parse(Rule::hour, input);
        assert!(pairs.is_err());
    }

    Ok(())
}

#[test]
fn parse_time_valid() -> anyhow::Result<()> {
    let inputs = vec!["12:49:00", "07:14:24"];
    for input in inputs {
        let pairs = DateTimeParser::parse(Rule::time, input)?;
        assert_eq!(input, pairs.as_str());
    }

    Ok(())
}

#[test]
fn parse_time_invalid() -> anyhow::Result<()> {
    let inputs = vec!["12:11 00", "12-11:00", "2:11 00"];
    for input in inputs {
        let pairs = DateTimeParser::parse(Rule::time, input);
        assert!(pairs.is_err());
    }

    Ok(())
}

#[test]
fn parse_day_valid() -> anyhow::Result<()> {
    let inputs = vec!["12", "08"];
    for input in inputs {
        let pairs = DateTimeParser::parse(Rule::day, input)?;
        assert_eq!(input, pairs.as_str());
    }

    Ok(())
}

#[test]
fn parse_day_invalid() -> anyhow::Result<()> {
    let inputs = vec!["1", "9", "a", " "];
    for input in inputs {
        let pairs = DateTimeParser::parse(Rule::day, input);
        assert!(pairs.is_err());
    }

    Ok(())
}

#[test]
fn parse_month_valid() -> anyhow::Result<()> {
    let inputs = vec!["12", "08"];
    for input in inputs {
        let pairs = DateTimeParser::parse(Rule::month, input)?;
        assert_eq!(input, pairs.as_str());
    }

    Ok(())
}

#[test]
fn parse_month_invalid() -> anyhow::Result<()> {
    let inputs = vec!["1", "9", "a", " "];
    for input in inputs {
        let pairs = DateTimeParser::parse(Rule::month, input);
        assert!(pairs.is_err());
    }

    Ok(())
}

#[test]
fn parse_year_valid() -> anyhow::Result<()> {
    let inputs = vec!["2004", "1998", "0734"];
    for input in inputs {
        let pairs = DateTimeParser::parse(Rule::year, input)?;
        assert_eq!(input, pairs.as_str());
    }

    Ok(())
}

#[test]
fn parse_year_invalid() -> anyhow::Result<()> {
    let inputs = vec!["204", "13", "7"];
    for input in inputs {
        let pairs = DateTimeParser::parse(Rule::year, input);
        assert!(pairs.is_err());
    }

    Ok(())
}

#[test]
fn parse_date_valid() -> anyhow::Result<()> {
    let inputs = vec!["19.11.2024", "19/11/2024", "19-11-2024", "19.11-2024", "19.11/2024", "19/11-2024"];
    for input in inputs {
        let pairs = DateTimeParser::parse(Rule::date, input)?;
        assert_eq!(input, pairs.as_str());
    }

    Ok(())
}

#[test]
fn parse_date_invalid() -> anyhow::Result<()> {
    let inputs = vec!["19_11_2024", "19 11 2024"];
    for input in inputs {
        let pairs = DateTimeParser::parse(Rule::date, input);
        assert!(pairs.is_err());
    }

    Ok(())
}

#[test]
fn parse_timezone_valid() -> anyhow::Result<()> {
    let inputs = vec!["+02:00", "-11:15"];
    for input in inputs {
        let pairs = DateTimeParser::parse(Rule::timezone, input)?;
        assert_eq!(input, pairs.as_str());
    }

    Ok(())
}

#[test]
fn parse_timezone_invalid() -> anyhow::Result<()> {
    let inputs = vec!["+2:00", "10:00"];
    for input in inputs {
        let pairs = DateTimeParser::parse(Rule::timezone, input);
        assert!(pairs.is_err());
    }

    Ok(())
}

#[test]
fn parse_date_time_valid() -> anyhow::Result<()> {
    let inputs = vec![
        "19.11.2024  12:56:00",
        "19/11/2024  12:56:00", 
        "19-11-2024  12:56:00", 
        "19/11/2024  12:56:00 +02:00", 
        "19/11/2024  12:56:00 -02:00"
        ];
    for input in inputs {
        let pairs = DateTimeParser::parse(Rule::date_time, input)?;
        assert_eq!(input, pairs.as_str());
    }

    Ok(())
}

#[test]
fn parse_date_time_invalid() -> anyhow::Result<()> {
    let inputs = vec![
        "19_11/2024  12:56:00", 
        " 19-11-2024  12:56:00", 
        "19/11/2024_12:56:00 +02:00", 
        "19/11/2024  12:56:00 02:00"
        ];
    for input in inputs {
        let pairs = DateTimeParser::parse(Rule::date_time, input);
        assert!(pairs.is_err());
    }

    Ok(())
}

#[test]
fn extract_data_from_parsing_datetime_valid() -> anyhow::Result<()> {
    let inputs = vec![
        ("19.11.2024  12:56:00", DateTime{day: "19".to_string(), month: "11".to_string(), year:"2024".to_string(), hours: "12".to_string(), minutes: "56".to_string(), seconds: "00".to_string(), time_zone_offset: None::<String>}), 
        ("19/11/2024  12:56:00", DateTime{day: "19".to_string(), month: "11".to_string(), year:"2024".to_string(), hours: "12".to_string(), minutes: "56".to_string(), seconds: "00".to_string(), time_zone_offset: None::<String>}), 
        ("19-11-2024  12:56:00", DateTime{day: "19".to_string(), month: "11".to_string(), year:"2024".to_string(), hours: "12".to_string(), minutes: "56".to_string(), seconds: "00".to_string(), time_zone_offset: None::<String>}),
        ("19/11/2024  12:56:00 +02:00",  DateTime{day: "19".to_string(), month: "11".to_string(), year:"2024".to_string(), hours: "12".to_string(), minutes: "56".to_string(), seconds: "00".to_string(), time_zone_offset: Some::<String>("+02:00".to_string())}),
        ("19/11/2024  12:56:00 -02:00",  DateTime{day: "19".to_string(), month: "11".to_string(), year:"2024".to_string(), hours: "12".to_string(), minutes: "56".to_string(), seconds: "00".to_string(), time_zone_offset: Some::<String>("-02:00".to_string())}),
        ];
    for (input, expected_result) in inputs {
        let result = DateTime::from_data_time(input)?;
        assert_eq!(expected_result, result);
    }

    Ok(())
}
