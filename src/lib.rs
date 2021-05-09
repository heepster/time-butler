use chrono::prelude::*;
use regex::Regex;

pub fn date_parse(time_str: &str) -> Result<DateTime<Local>, &'static str> {
    let unix_secs_format = Regex::new(r"^\d{10}$").unwrap();
    let unix_secs_with_nano_format = Regex::new(r"^\d{13}$").unwrap();

    if unix_secs_format.is_match(time_str) {
        let integer = time_str.parse::<i64>().unwrap();
        let dt = Utc.timestamp(integer, 0).with_timezone(&Local);
        return Ok(dt);
    } else if unix_secs_with_nano_format.is_match(time_str) {
        let integer = time_str.parse::<i64>().unwrap();
        let seconds = integer / 1000;
        let nano = (integer % 1000) as u32;
        let dt = Utc.timestamp(seconds, nano).with_timezone(&Local);
        return Ok(dt);
    } else {
        let mut date_formats = vec![
            "%Y-%m-%d %H:%M:%S %z",
            "%Y-%m-%dT%H:%M:%S %z",
            "%Y-%m-%dT%H:%M:%S%z",
        ];

        for date_format in date_formats.iter_mut() {
            let parsed = DateTime::parse_from_str(time_str, date_format);
            if parsed.is_ok() {
                return Ok(parsed.unwrap().with_timezone(&Local));
            }
        }

        return Err("Datetime not recognized");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_10_digit_unix_timestamp() {
        let test_dt = "1620587968";
        let result = date_parse(test_dt);
        let expected = Ok(Utc.timestamp(1620587968, 0).with_timezone(&Local));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_13_digit_unix_timestamp() {
        let test_dt = "1620587968333";
        let result = date_parse(test_dt);
        let expected = Ok(Utc.timestamp(1620587968, 333).with_timezone(&Local));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_timestamp_1() {
        let test_dt = "2021-05-09 20:37:39 +00:00";
        let result = date_parse(test_dt);
        let expected = Ok(Utc
            .ymd(2021, 5, 09)
            .and_hms_milli(20, 37, 39, 0)
            .with_timezone(&Local));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_timestamp_2() {
        let test_dt = "2021-05-09T20:37:39 +00:00";
        let result = date_parse(test_dt);
        let expected = Ok(Utc
            .ymd(2021, 5, 09)
            .and_hms_milli(20, 37, 39, 0)
            .with_timezone(&Local));
        assert_eq!(result, expected);
    }
    #[test]
    fn test_timestamp_3() {
        let test_dt = "2021-05-09T20:37:39+00:00";
        let result = date_parse(test_dt);
        let expected = Ok(Utc
            .ymd(2021, 5, 09)
            .and_hms_milli(20, 37, 39, 0)
            .with_timezone(&Local));
        assert_eq!(result, expected);
    }
}
