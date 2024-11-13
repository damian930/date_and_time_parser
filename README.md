# [damians_custom_datetime_parser](https://crates.io/crates/damians_custom_datetime_parser)

## Project Overview

`damians_custom_datetime_parser` is a Rust library designed for parsing custom date and time strings using the `pest` parser. It validates and extracts day, month, year, hour, minute, and second components from date-time strings, providing users with detailed feedback on the success or failure of the parsing process.

The library utilizes a custom grammar defined in the `grammar.pest` file to facilitate parsing, and the parser is robust enough to handle errors, providing detailed error messages in case of invalid input.

## Features

- **Custom Date and Time Parsing**: Easily parse date-time strings such as `06.11.2024 19:34:00`.
- **Error Reporting**: Get detailed error information when parsing fails, indicating which part of the string couldn't be parsed.
- **CLI Support**: A simple command-line interface (CLI) that supports parsing date-time strings and displaying help and credits information.

## Grammar Rules

The parser for this project is based on a custom grammar defined using `pest`, which is a PEG (Parsing Expression Grammar) parser for Rust. The following are the key grammar rules for parsing:

1. **Date Format**: The date part of the string is expected to be in the format `DD.MM.YYYY`.

   - `day` is a 2-digit number (`DD`).
   - `month` is a 2-digit number (`MM`).
   - `year` is a 4-digit number (`YYYY`).

2. **Time Format**: The time part of the string is expected to be in the format `HH:MM:SS`.

   - `hours`, `minutes`, and `seconds` are all 2-digit numbers (`HH`, `MM`, `SS`).

3. **Whitespace Handling**: The parser skips any unnecessary whitespace between the date and time components.

4. **Error Handling**: If the date-time string deviates from the expected format, the parser raises an error specifying which part of the string caused the issue.

## Parsing Process

The `DateTimeParser` parses a date-time string following these steps:

1. **Input**: A string like `06.11.2024 19:34:00` is provided to the parser.
2. **Parsing**: The string is parsed according to the grammar, extracting the day, month, year, hours, minutes, and seconds.
3. **Success**: If the parsing is successful, a `DateTime` struct is returned with the parsed components (day, month, year, hours, minutes, seconds).
4. **Failure**: If the parsing fails, an error is returned indicating the specific issue with the input string.

The parsing operation uses `pest`'s grammar pattern matching to break the string into parts, and it can handle a variety of invalid input formats, making it robust for use in real-world scenarios.

## Usage

### Command-Line Interface (CLI)

The CLI provides the following commands:

1. **Help**: Prints usage instructions.

   ```bash
   cargo run help
   ```

2. **Credits**: Prints credits.

   ```bash
   cargo run credits
   ```

3. **Parse**: Parse DateTime.
   ```bash
   cargo run parse "06.11.2024 19:34:00"
   ```

### Example

Given the input string `"06.11.2024 19:34:00"`, the parser will output the following:

```plaintext
DateTime {
    day: "06",
    month: "11",
    year: "2024",
    hours: "19",
    minutes: "34",
    seconds: "00",
}
```

### Parsing Example Code

Here's an example of how to use the `damians_custom_datetime_parser` library in your own Rust program:

```rust
use damians_custom_datetime_parser::*;

fn main() {
    let date_time_str = "06.11.2024 19:34:00";
    match DateTime::from_data_time(date_time_str) {
        Ok(date_time) => println!("{:?}", date_time),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Error Handling

If an invalid date-time string is provided, for example, `"06/11/2024 19:34:00"`, the parser will return an error such as:

```plaintext
ParseError(
        " --> 1:1\n  |\n1 | 06/11/2024 19:34:00\n  | ^---\n  |\n  = expected date",
    )
```

## Unit Tests

Unit tests are provided to ensure that the parsing logic works correctly. The tests cover the following scenarios:

- Valid date-time strings in the correct format.
- Invalid strings that don't match the expected format.
- Error handling when unexpected input is encountered.

### Test Coverage

Each of the grammar rules is covered by at least one unit test, ensuring that the parser behaves correctly in different scenarios.

## Dependencies

- `pest`: A PEG parser for Rust.
- `thiserror`: A library for defining custom error types.
- `anyhow`: A library for easy error handling.
- `pest_derive`: A procedural macro for generating parsers from `.pest` grammar files.

## Project Structure

The project consists of the following main files:

- `lib.rs`: Contains the `DateTimeParser` and `DateTime` struct, along with the parsing logic.
- `main.rs`: Implements the command-line interface (CLI) and invokes the parser.
- `grammar.pest`: The grammar definition for the date-time string.
- `tests/`: Contains unit tests to ensure the correctness of the parser.

### Project Class Diagram

```
+----------------------------------------------------+
|                    DateTimeParser                  |
|----------------------------------------------------|
| - Grammar rules for date-time parsing              |
| - Methods: parse()                                 |
+----------------------------------------------------+
                        |
                        v
+----------------------------------------------------+
|                       DateTime                    |
|----------------------------------------------------|
| - day: String                                      |
| - month: String                                    |
| - year: String                                     |
| - hours: String                                    |
| - minutes: String                                  |
| - seconds: String                                  |
|----------------------------------------------------|
| + from_data_time(date_time: &str) -> Result<Self,  |
|     DateTimeError>                                 |
+----------------------------------------------------+
                        |
                        v
+----------------------------------------------------+
|                    DateTimeError                   |
|----------------------------------------------------|
| - error_message: String                            |
|----------------------------------------------------|
| + ParseError(String)                               |
+----------------------------------------------------+
                        |
                        v
+----------------------------------------------------+
|               Command-Line Interface               |
|----------------------------------------------------|
| + parse(date_time: &str) -> Result<DateTime, DateTimeError> |
| + get_help()                                       |
| + print_credits()                                  |
+----------------------------------------------------+
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
