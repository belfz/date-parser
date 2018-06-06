use std::str::FromStr;

#[cfg(test)]
mod tests {
    mod fn_parse_month {
        use super::super::{parse_month, ParseDateError};

        #[test]
        fn it_should_return_correct_month_number_from_uppercase_month() {
            assert_eq!(parse_month("May"), Ok(5));
        }

        #[test]
        fn it_should_return_correct_month_number_from_lowercase_month() {
            assert_eq!(parse_month("august"), Ok(8));
        }

        #[test]
        fn it_should_return_an_error_for_unknown_month() {
            let name = "noSuchMonth";

            assert_eq!(parse_month(name), Err(ParseDateError::UnknownMonth(name.to_string())));
        }
    }

    mod fn_tokenize {
        use super::super::{tokenize, ParseDateError};

        #[test]
        fn it_should_return_ok_four_element_tuple_of_tokens() {
            let input_str = "24th of May 1990";

            assert_eq!(tokenize(input_str), Ok(("24th".to_string(), "of".to_string(), "May".to_string(), "1990".to_string())));
        }

        #[test]
        fn it_should_return_err_malformed_date_string() {
            let input_str = "not enough tokens";

            assert_eq!(tokenize(input_str), Err(ParseDateError::MalformedDateString(input_str.to_string())));
        }
    }

    mod fn_parse_preposition {
        use super::super::{parse_preposition, ParseDateError};

        #[test]
        fn it_should_return_ok_if_preposition_is_of() {
            let input_str = "of";

            assert_eq!(parse_preposition(input_str), Ok(()));
        }

        #[test]
        fn it_should_return_err_unknown_preposition() {
            let input_str = "totallyNotAPreposition";

            assert_eq!(parse_preposition(input_str), Err(ParseDateError::UnknownPreposition(input_str.to_string())));
        }
    }

    mod fn_parse_day {
        use super::super::{parse_day, ParseDateError};

        #[test]
        fn it_should_parse_correct_day() {
            let input_str = "31st";
            let month = 1;
            let year = 2018;

            assert_eq!(parse_day(input_str, month, year), Ok(31));
        }

        #[test]
        fn it_should_return_err_when_day_does_not_fit_in_month() {
            let input_str = "31st";
            let month = 4;
            let year = 2018;

            assert_eq!(parse_day(input_str, month, year), Err(ParseDateError::DayDoesNotFitInMonth(31, month)));
        }

        #[test]
        fn it_should_return_err_on_negative_day() {
            let input_str = "-2nd";
            let month = 1;
            let year = 2018;

            assert_eq!(parse_day(input_str, month, year), Err(ParseDateError::MalformedDay(input_str.to_string())));
        }

        #[test]
        fn it_should_return_err_on_day_over_31() {
            let input_str = "32nd";
            let month = 1;
            let year = 2018;

            assert_eq!(parse_day(input_str, month, year), Err(ParseDateError::DayDoesNotFitInMonth(32, 1)));
        }

        #[test]
        fn it_should_return_err_when_day_cant_be_parsed() {
            let input_str = "some-trash";
            let month = 1;
            let year = 2018;

            assert_eq!(parse_day(input_str, month, year), Err(ParseDateError::MalformedDay(input_str.to_string())));
        }

        #[test]
        fn it_should_return_err_when_postfix_is_incorrect() {
            let input_str = "22rd";
            let month = 1;
            let year = 2018;

            assert_eq!(parse_day(input_str, month, year), Err(ParseDateError::InvalidDayPostfix(22, "rd".to_string())));
        }

        #[test]
        fn it_should_allow_february_29_on_leap_year() {
            let input_str = "29th";
            let month = 2;
            let leap_year = 2016;

            assert_eq!(parse_day(input_str, month, leap_year), Ok(29));
        }

        #[test]
        fn it_should_return_err_when_february_29_on_non_leap_year() {
            let input_str = "29th";
            let month = 2;
            let non_leap_year = 2017;

            assert_eq!(parse_day(input_str, month, non_leap_year), Err(ParseDateError::NonLeapYear(non_leap_year)));
        }
    }

    mod fn_parse_year {
        use super::super::{parse_year, ParseDateError};

        #[test]
        fn it_should_parse_correct_year() {
            let input_str = "1990";

            assert_eq!(parse_year(input_str), Ok(1990));
        }

        #[test]
        fn it_should_allow_negative_year() {
            let input_str = "-4000";

            assert_eq!(parse_year(input_str), Ok(-4000));
        }

        #[test]
        fn it_should_return_err_when_year_cant_be_parsed() {
            let input_str = "totallyNotAYear";

            assert_eq!(parse_year(input_str), Err(ParseDateError::MalformedYear(input_str.to_string())));
        }
    }

    mod end_to_end {
        use std::str::FromStr;
        use super::super::Date;

        #[test]
        fn it_should_parse_the_date() {
            let input_str = "24th of May 1990";
            let date = Date::from_str(input_str);
            
            assert!(date.is_ok());
            let unwrapped = date.unwrap();
            assert_eq!(unwrapped.get_day(), 24);
            assert_eq!(unwrapped.get_month(), 5);
            assert_eq!(unwrapped.get_year(), 1990);
        }
    }
}

#[derive(Debug, PartialEq)]
struct Date {
    day: u8,
    month: u8,
    year: i32,
}

#[derive(Debug, PartialEq)]
enum ParseDateError {
    MalformedDateString(String),
    UnknownMonth(String),
    UnknownPreposition(String),
    MalformedDay(String),
    DayDoesNotFitInMonth(u8, u8),
    InvalidDayPostfix(u8, String),
    NonLeapYear(i32),
    MalformedYear(String),
}

fn tokenize(input_str: &str) -> Result<(String, String, String, String), ParseDateError> {
    let tokens: Vec<String> = input_str.split_whitespace().map(|token| token.to_string()).collect();
    if tokens.len() == 4 {
        Ok((tokens[0].clone(), tokens[1].clone(), tokens[2].clone(), tokens[3].clone()))
    } else {
        Err(ParseDateError::MalformedDateString(input_str.to_string()))
    }
}

fn validate_day_postfix(day: u8, postfix: &str) -> Result<(), ParseDateError> {
    match (day, postfix) {
        (1, "st") | (21, "st") | (31, "st") |
        (2, "nd") | (22, "nd") |
        (3, "rd") | (23, "rd") |
        (4...20, "th") | (24...30, "th") => Ok(()),
        _ => Err(ParseDateError::InvalidDayPostfix(day, postfix.to_string()))
    }
}
fn is_leap_year(year: i32) -> bool {
    ((year % 4 == 0) && (year % 100 != 0)) || (year % 400 == 0)
}

fn validate_leap_year(day: u8, month: u8, year: i32) -> Result<(), ParseDateError> {
    if !is_leap_year(year) && month == 2 && day == 29 {
        Err(ParseDateError::NonLeapYear(year))
    } else {
        Ok(())
    }
}

static MONTHS_WITH_31: &'static [u8] = &[1, 3, 5, 7, 8, 10, 12];
static MONTHS_WITH_30: &'static [u8] = &[4, 6, 9, 11];

fn validate_day_month(day: u8, month: u8) -> Result<(), ParseDateError> {
    match (day, month) {
        (1...31, m) if MONTHS_WITH_31.contains(&m) => Ok(()),
        (1...30, m) if MONTHS_WITH_30.contains(&m) => Ok(()),
        (1...29, 2) => Ok(()),
        _ => Err(ParseDateError::DayDoesNotFitInMonth(day, month))
    }
}

fn parse_day(input_str: &str, month: u8, year: i32) -> Result<u8, ParseDateError> {
    let day = input_str
        .chars()
        .take_while(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<u8>()
        .map_err(|_| ParseDateError::MalformedDay(input_str.to_string()))?;

    let day_postfix = input_str.chars().skip_while(|c| c.is_digit(10)).collect::<String>();
    let _ = validate_day_month(day, month)?;
    let _ = validate_day_postfix(day, &day_postfix)?;
    let _ = validate_leap_year(day, month, year)?;
    
    Ok(day)
}

fn parse_preposition(input_str: &str) -> Result<(), ParseDateError> {
    if input_str == "of" { Ok(()) } else { Err(ParseDateError::UnknownPreposition(input_str.to_string())) }
}

fn parse_month(input_str: &str) -> Result<u8, ParseDateError> {
    match input_str.to_lowercase().as_ref() {
        "january" => Ok(1),
        "february" => Ok(2),
        "march" => Ok(3),
        "april" => Ok(4),
        "may" => Ok(5),
        "june" => Ok(6),
        "july" => Ok(7),
        "august" => Ok(8),
        "september" => Ok(9),
        "october" => Ok(10),
        "november" => Ok(11),
        "december" => Ok(12),
        _ => Err(ParseDateError::UnknownMonth(input_str.to_string()))
    }
}

fn parse_year(input_str: &str) -> Result<i32, ParseDateError> {
    input_str
        .parse::<i32>()
        .map_err(|_| ParseDateError::MalformedYear(input_str.to_string()))
}

impl FromStr for Date {
    type Err = ParseDateError;

    fn from_str(input_str: &str) -> Result<Self, Self::Err> {
        let (day_str, preposition, month_str, year_str) = tokenize(input_str)?;
        let year = parse_year(&year_str)?;
        let _ = parse_preposition(&preposition)?;
        let month = parse_month(&month_str)?;
        let day = parse_day(&day_str, month, year)?;
        Ok(Date { day, month, year})
    }
}

#[allow(dead_code)]
impl Date {
    pub fn get_day(&self) -> u8 {
        self.day
    }

    pub fn get_month(&self) -> u8 {
        self.month
    }

    pub fn get_year(&self) -> i32 {
        self.year
    }
}
