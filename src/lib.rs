/*! A validator for greek social security number (AMKA)
More information is available on
[AMKA.gr](https://www.amka.gr/tieinai.html).
*/

use chrono::format::ParseError;
use chrono::NaiveDate;
use luhn;

fn is_string_numeric(str: String) -> bool {
    for c in str.chars() {
        if !c.is_numeric() {
            return false;
        }
    }

    true
}

/// Validates the given string for AMKA
pub fn validate(amka: &str) -> (bool, &str) {
    if !is_string_numeric(amka.to_string()) {
        return (false, "Provided AMKA is not a numeric value");
    }

    if amka.chars().count() != 11 {
        return (false, "Provided number is not 11 digits long");
    }

    let check_date = || -> Result<(), ParseError> {
        let date_part = &amka[..6];
        NaiveDate::parse_from_str(date_part, "%d%m%y")?;
        Ok(())
    };

    if let Err(_err) = check_date() {
        return (
            false,
            "First 6 digits of the provided AMKA do not represent a valid date",
        );
    }

    if !luhn::valid(amka) {
        return (false, "Provided AMKA does not have a valid checkdigit");
    }

    (true, "")
}

#[cfg(test)]
mod tests {
    use crate::validate;

    #[test]
    fn it_validates_valid_amka() {
        let (is_valid, _err) = validate("09095986684");
        assert_eq!(true, is_valid);
    }

    #[test]
    fn it_fails_when_not_a_number() {
        let (is_valid, _err) = validate("asvs");
        assert_eq!(false, is_valid);
    }

    #[test]
    fn it_fails_when_short_number() {
        let (is_valid, _err) = validate("09095986");
        assert_eq!(false, is_valid);
    }

    #[test]
    fn it_fails_when_long_number() {
        let (is_valid, _err) = validate("090959866845");
        assert_eq!(false, is_valid);
    }

    #[test]
    fn it_fails_when_not_a_valid_date() {
        let (is_valid, _err) = validate("39095986681");
        assert_eq!(false, is_valid);
    }

    #[test]
    fn it_fails_with_bad_checkdigit() {
        let (is_valid, _err) = validate("09095986680");
        assert_eq!(false, is_valid);
    }

    #[test]
    fn readme() {
        // An invalid AMKA
        let (is_valid, err) = validate("09095986680");
        assert!(!is_valid);
        println!("{}", err);

        // An valid AMKA
        let (is_valid, err) = validate("09095986684");
        assert!(is_valid);
        assert_eq!("", err)
    }
}
