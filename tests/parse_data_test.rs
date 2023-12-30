#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use todo_lib::parse_date;

    #[test]
    fn test_parse_valid_date() {
        let date_str = "2023-12-30";
        let result = parse_date(date_str);
        let expected_date = NaiveDate::from_ymd_opt(2023, 12, 30);

        assert_eq!(result, Ok(expected_date.unwrap()));
    }

    #[test]
    fn test_parse_invalid_date() {
        let date_str = "2023-02-30";
        let result = parse_date(date_str);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_short_date() {
        let date_str = "2023-12";
        let result = parse_date(date_str);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_format() {
        let date_str = "30-12-2023";
        let result = parse_date(date_str);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_empty_string() {
        let date_str = "";
        let result = parse_date(date_str);

        assert!(result.is_err());
    }
}
