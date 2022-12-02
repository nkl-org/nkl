use std::{error::Error, fmt::Display};

/// Parse ENDF integer.
///
/// # Format
///
/// ENDF format integers have the following format:
///
/// ```text
/// endf_integer = sign? digits[1:10]
/// sign = '+' | '-'
/// digits = '0' - '9'
/// ```
///
/// ENDF integers can be read with fortran `I11` format.
///
/// # Examples
///
/// ```
/// use nkl::data::endf::parse_endf_integer;
/// let integer = parse_endf_integer(" 1234567890").unwrap();
/// assert_eq!(integer, 1234567890);
/// ```
///
/// # Details
///
/// This function respects following rules (as stated by ENDF format specification):
/// - Leading/Trailing space are ignored
/// - Blank slice are considered to be `0` (fortran `I11` input processing rule)
/// - Space character within numbers are ignored (fortran `I11` blank interpretation mode)
/// - Plus sign is optional
///
/// # Errors
///
/// [`ParseEndfIntegerError`] is returned if:
/// - `integer.is_empty()`: empty slice
/// - `integer.len() > 11`: too long slice
/// - `integer` contains only sign `-` or `+`
/// - `integer` contains invalid sign/digit
/// - `integer` is only partially parsable
pub fn parse_endf_integer<I: AsRef<[u8]>>(integer: I) -> Result<i64, ParseEndfIntegerError> {
    // The implementation here is based on following objectives:
    // - Support fortran E-less format
    // - Support fortran blank interpretation mode
    // - Do not incur UTF-8 validation => no conversion to string
    // - Rely on limited integer numbers length in ENDF format (<= 11)
    //   => prevent overflow
    let integer = integer.as_ref();
    // -> empty slice
    if integer.is_empty() {
        return Err(ParseEndfIntegerError);
    }
    // ENDF integers are limited to 11 characters (sign + 10 digits)
    // -> too long slice
    if integer.len() > 11 {
        return Err(ParseEndfIntegerError);
    }
    // - integer.len() <= 11 => no i64 overflow (i64 max digits = 19 > 11)
    let mut iter = integer.iter().filter(|&b| *b != b' ').peekable();
    // extract sign
    let negative = match iter.peek() {
        // -> blank slice
        None => return Ok(0),
        Some(b'-') => {
            iter.next();
            true
        }
        Some(b'+') => {
            iter.next();
            false
        }
        Some(_) => false,
    };
    // -> sign only
    if iter.peek().is_none() {
        return Err(ParseEndfIntegerError);
    }
    // parse digits
    let mut value = 0;
    for byte in iter {
        if byte.is_ascii_digit() {
            value = value * 10 + (byte - b'0') as i64; // no overflow
        } else {
            return Err(ParseEndfIntegerError);
        }
    }
    // apply sign
    if negative {
        value = -value;
    }
    Ok(value)
}

/// Error returned when parsing an ENDF integer with [`parse_endf_integer`] fails.
#[derive(Debug)]
pub struct ParseEndfIntegerError;

impl Display for ParseEndfIntegerError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "parse ENDF integer error")
    }
}

impl Error for ParseEndfIntegerError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_endf_integer_eq(str: &str, value: i64) {
        let int = parse_endf_integer(str).unwrap_or_else(|_| panic!("error parsing \"{str}\""));
        assert_eq!(int, value);
    }

    #[test]
    fn empty_slice() {
        assert!(parse_endf_integer("").is_err());
    }

    #[test]
    fn too_long_slice() {
        assert!(parse_endf_integer(" -1234567890").is_err());
        assert!(parse_endf_integer("            ").is_err());
    }

    #[test]
    fn sign_only() {
        assert!(parse_endf_integer("-").is_err());
        assert!(parse_endf_integer("+").is_err());
    }

    #[test]
    fn invalid_sign() {
        assert!(parse_endf_integer("*123").is_err());
        assert!(parse_endf_integer(" *123 ").is_err());
        assert!(parse_endf_integer("* 123 ").is_err());
    }

    #[test]
    fn invalid_digit() {
        assert!(parse_endf_integer("a").is_err());
        assert!(parse_endf_integer(" a").is_err());
        assert!(parse_endf_integer("+a").is_err());
        assert!(parse_endf_integer("-a").is_err());
        assert!(parse_endf_integer(" a ").is_err());
    }

    #[test]
    fn partial() {
        assert!(parse_endf_integer(" -12345 X").is_err());
    }

    #[test]
    fn double_sign() {
        assert!(parse_endf_integer("-+1").is_err());
        assert!(parse_endf_integer("-+1").is_err());
        assert!(parse_endf_integer("--1").is_err());
        assert!(parse_endf_integer("++1").is_err());
    }

    #[test]
    fn floats() {
        assert!(parse_endf_integer(" 12345678.9").is_err());
        assert!(parse_endf_integer(" 1.2345E+12").is_err());
    }

    #[test]
    fn standard() {
        assert_endf_integer_eq("0", 0);
        assert_endf_integer_eq(" 0", 0);
        assert_endf_integer_eq("-0", 0);
        assert_endf_integer_eq("+0", 0);
        assert_endf_integer_eq("1", 1);
        assert_endf_integer_eq(" 1", 1);
        assert_endf_integer_eq("-1", -1);
        assert_endf_integer_eq("+1", 1);
        assert_endf_integer_eq(" 1234567890", 1234567890);
        assert_endf_integer_eq("-1234567890", -1234567890);
        assert_endf_integer_eq("+1234567890", 1234567890);
    }

    #[test]
    fn n_digit() {
        assert_endf_integer_eq(" 1", 1);
        assert_endf_integer_eq(" 12", 12);
        assert_endf_integer_eq(" 123", 123);
        assert_endf_integer_eq(" 1234", 1234);
        assert_endf_integer_eq(" 12345", 12345);
        assert_endf_integer_eq(" 123456", 123456);
        assert_endf_integer_eq(" 1234567", 1234567);
        assert_endf_integer_eq(" 12345678", 12345678);
        assert_endf_integer_eq(" 123456789", 123456789);
        assert_endf_integer_eq(" 1234567890", 1234567890);
        assert_endf_integer_eq(" 1         ", 1);
        assert_endf_integer_eq(" 12        ", 12);
        assert_endf_integer_eq(" 123       ", 123);
        assert_endf_integer_eq(" 1234      ", 1234);
        assert_endf_integer_eq(" 12345     ", 12345);
        assert_endf_integer_eq(" 123456    ", 123456);
        assert_endf_integer_eq(" 1234567   ", 1234567);
        assert_endf_integer_eq(" 12345678  ", 12345678);
        assert_endf_integer_eq(" 123456789 ", 123456789);
        assert_endf_integer_eq(" 1234567890", 1234567890);
        assert_endf_integer_eq("          1", 1);
        assert_endf_integer_eq("         12", 12);
        assert_endf_integer_eq("        123", 123);
        assert_endf_integer_eq("       1234", 1234);
        assert_endf_integer_eq("      12345", 12345);
        assert_endf_integer_eq("     123456", 123456);
        assert_endf_integer_eq("    1234567", 1234567);
        assert_endf_integer_eq("   12345678", 12345678);
        assert_endf_integer_eq("  123456789", 123456789);
        assert_endf_integer_eq(" 1234567890", 1234567890);
    }

    #[test]
    fn blank() {
        assert_endf_integer_eq(" ", 0);
        assert_endf_integer_eq("  ", 0);
        assert_endf_integer_eq("   ", 0);
        assert_endf_integer_eq("    ", 0);
        assert_endf_integer_eq("     ", 0);
        assert_endf_integer_eq("      ", 0);
        assert_endf_integer_eq("       ", 0);
        assert_endf_integer_eq("        ", 0);
        assert_endf_integer_eq("         ", 0);
        assert_endf_integer_eq("          ", 0);
        assert_endf_integer_eq("           ", 0);
    }

    #[test]
    fn space() {
        assert_endf_integer_eq(" 12 ", 12);
        assert_endf_integer_eq(" 1 2 ", 12);
        assert_endf_integer_eq(" 1  2 ", 12);
        assert_endf_integer_eq(" 1   2 ", 12);
        assert_endf_integer_eq(" 1    2 ", 12);
        assert_endf_integer_eq(" 1     2 ", 12);
        assert_endf_integer_eq(" 1      2 ", 12);
        assert_endf_integer_eq(" 1       2 ", 12);
        assert_endf_integer_eq("1         2", 12);
        assert_endf_integer_eq("-    1    2", -12);
        assert_endf_integer_eq("+    1    2", 12);
        assert_endf_integer_eq("1 2 3 4 5 6", 123456)
    }
}
