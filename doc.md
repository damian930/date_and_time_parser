# Grammar Rules for DateTime Parser

This file describes the grammar rules used in the Rust parser for parsing `date and time` strings. The grammar is designed to handle typical date-time formats, including support for time zones and common delimiters like `".", "/", "-"`

## **spaces**

```rust
spaces = { " "+ }
```

- **Description**: This rule matches one or more space characters.
- **Usage**: It is used to skip spaces between components of the date-time string.

## **digit**

```rust
digit = { '0'..'9' }
```

- **Description**: This rule matches any single digit character from '0' to '9'.
- **Usage**: It is used throughout the grammar to identify individual digits inside the date and time components.

## **second, minute, hour, day, month, year**

```rust
second = { digit ~ digit }
minute = { digit ~ digit }
hour = { digit ~ digit }
day = { digit ~ digit }
month = { digit ~ digit }
year = { digit ~ digit ~ digit ~ digit }
```

- **Description**: These rules match two-digit values for second, minute, hour, day, month and a four-digit value for year.
- **Usage**: These rules are used to parse individual components of the date and time (e.g., hours, minutes, seconds, days, months, and years) from the input string.

## **time**

```rust
time = { hour ~ ":" ~ minute ~ ":" ~ second}
```

- **Description**: This rule matches a time string in the format HH:MM:SS (e.g., "12:30:45").
- **Usage**: It is used to parse the complete time component, combining hours, minutes, and seconds, separated by colons.

## **date**

```rust
date = { day ~ ("." | "/" | "-") ~ month ~ ("." | "/" | "-") ~ year }
```

- **Description**: This rule matches a date string in the format DD.MM.YYYY, DD/MM/YYYY, or DD-MM-YYYY (e.g., "25/12/2024").
- **Usage**: It is used to parse the full date string, combining day, month, and year with a separator (either a dot, slash, or dash).

## **timezone**

```rust
timezone = { ("+" | "-") ~ digit ~ digit ~ ":" ~ digit ~ digit }
```

- **Description**: This rule matches a timezone offset string in the format +HH:MM or -HH:MM (e.g., "+02:00" or "-05:30").
- **Usage**: It is used to optionally parse the time zone information, which appears after the date and time components.

## **date_time**

```rust
date_time = { SOI ~ date ~ spaces ~ time ~ (spaces ~ timezone)? ~ EOI }
```

- **Description**: This is the top-level rule that matches a complete date-time string. It matches the following structure:
  - A date (e.g., "25/12/2024")
  - Followed by a space(s)
- A time (e.g., "12:30:45")
  - Optionally, space(s) followed by a time zone (e.g., "+02:00")
- **Usage**: This rule is used to parse a complete date-time string, optionally with a time zone.
