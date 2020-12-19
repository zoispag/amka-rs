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
pub fn valid(amka: &str) -> bool {
    if !is_string_numeric(amka.to_string()) {
        println!("Provided AMKA is not a numeric value");
        return false;
    }

    if amka.chars().count() != 11 {
        println!("Provided number is not 11 digits long");
        return false;
    }

    let check_date = || -> Result<(), ParseError> {
        let date_part = &amka[..6];
        NaiveDate::parse_from_str(date_part, "%d%m%y")?;
        Ok(())
    };

    if let Err(_err) = check_date() {
        println!("First 6 digits of the provided AMKA do not represent a valid date");
        return false;
    }

    if !luhn::valid(amka) {
        println!("Provided AMKA does not have a valid checkdigit");
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::valid;

    #[test]
    fn it_validates_valid_amka() {
        assert_eq!(true, valid("09095986684"));
    }

    #[test]
    fn it_fails_when_not_a_number() {
        assert_eq!(false, valid("asvs"));
    }

    #[test]
    fn it_fails_when_short_number() {
        assert_eq!(false, valid("09095986"));
    }

    #[test]
    fn it_fails_when_long_number() {
        assert_eq!(false, valid("090959866845"));
    }

    #[test]
    fn it_fails_when_not_a_valid_date() {
        assert_eq!(false, valid("39095986681"));
    }

    #[test]
    fn it_fails_with_bad_checkdigit() {
        assert_eq!(false, valid("09095986680"));
    }

    #[test]
    fn readme() {
        // An invalid AMKA
        assert!(!valid("09095986680"));

        // An valid AMKA
        assert!(valid("09095986684"));
    }
}
