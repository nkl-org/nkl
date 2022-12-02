use std::{error::Error, fmt::Display};

/// Power of ten table.
#[rustfmt::skip]
const POW_10_TABLE: [f64; 23] = [
    1e0 , 1e1 , 1e2 , 1e3 , 1e4 , 1e5 , 1e6 , 1e7 , 1e8 , 1e9 ,
    1e10, 1e11, 1e12, 1e13, 1e14, 1e15, 1e16, 1e17, 1e18, 1e19,
    1e20, 1e21, 1e22,
];

/// Parse ENDF float.
///
/// # Format
///
/// ENDF format floats have the following format:
///
/// ```text
/// endf_float = int_sign? (int dec_sep fraction ((exp_sep exp_sign? exp) | (exp_sign exp)))[1:10]
/// int_sign, exp_sign = sign
/// sign = '-' | '+'
/// int, fraction, exp = digits
/// digits = '0' - '9'
/// dec_sep = '.'
/// expsep = 'e' | 'E'
/// ```
///
/// ENDF floats can be read with fortran `F11.0` format.
///
/// # Examples
///
/// ```
/// use nkl::data::endf::parse_endf_float;
/// let float = parse_endf_float("1.2345E+01").unwrap();
/// assert!((float - 1.2345E+01).abs() < 1e-4);
/// ```
///
/// # Details
///
/// This function respects following rules (as stated by ENDF format specification):
/// - Leading/Trailing space are ignored
/// - Blank slice are considered to be `0` (fortran `F11.0` input processing rule)
/// - Space character within numbers are ignored (fortran `F11.0` blank interpretation mode)
/// - Plus sign is optional
/// - Integral part is optional
/// - Fractional part is optional
/// - Exponential part is optional
/// - Exponential part (if it exists) use one of the following forms:
///     - a sign followed by digits
///     - `e` or `E` followed by digits optionnaly preceded by a sign
///
/// # Errors
///
/// [`ParseEndfFloatError`] is returned if:
/// - `float.is_empty()`: empty slice
/// - `float.len() > 11`: too long slice
/// - `float` contains only sign `-` or `+`
/// - `float` contains only decimal separator `.`
/// - `float` contains only exponent separator `e` or `E`
/// - `float` contains invalid sign/digit
/// - `float` is only partially parsable
/// - `float`'s exponential part is empty after exponent separator
///
/// # Notes
///
/// `d` and `D` exponent separator are supported for legacy compatibility.
pub fn parse_endf_float<F: AsRef<[u8]>>(float: F) -> Result<f64, ParseEndfFloatError> {
    // Parsing floating point numbers correctly is extremely difficult due to
    // conversion between binary/decimal representation and roundings.
    //
    // The goal here is to provide a simple but effective method supporting
    // the fortran "E-less" format for most common floating point numbers
    // available in standard ENDF evaluations.
    // i.e: 1.2345+12 = 1.2345e+12
    //
    // The implementation here is based on following objectives:
    // - Support fortran E-less format
    // - Support fortran blank interpretation mode
    // - Do not incur UTF-8 validation => no conversion to string
    // - Rely on limited floating point numbers length in ENDF format (<= 11)
    //   => prevent overflow
    // - Rely on std library for non trivial case (described below)
    //
    // The IEEE-754 specification requires that the result of an elementary
    // arithmetic operation is correctly rounded. If for a given decimal number,
    // the decimal significand and the decimal power can be represented exactly
    // as floats then the decimal representation can be immediately converted to
    // a binary floating point number.
    //
    // Based on IEEE-754, a double-precision floating point number is a binary
    // floating point number where 53 bits are used for the significand and 11
    // bits for the exponent for a total of 64 bits.
    //
    // Denote:
    // - f the floating point number,
    // - m the binary significand,
    // - p the binary exponent,
    // - w the decimal significand and
    // - q the decimal exponent.
    //
    // f = m × 2^p = w × 10^q
    //
    // Knowing that:
    // - small power of 10 can be represented exactly as floats
    //      10^q = 5^q × 2^q
    //      5^q < 2^53 <=> -22 <= q <= 22
    // - ENDF decimal significand with at most 11 digits can be represented
    //   exactly as floats
    //      10^12 = 5^12 × 2^12
    //      5^12 < 2^53
    // all ENDF float f = w × 10^q such that -22 <= q <= 22 can be converted
    // directly to binary floating point numbers.
    //
    // For small exponent (|q| <= 22) this implementation does the following:
    // - parse sign, integral, fractional and signed exponent part according
    //   to fortran F11.0 specification
    // - convert integral, fraction and signed exponent part to
    //   significand/mantissa and decimal exponent
    // - convert significand/exponent to binary floating point numbers with
    //   precomputed power of 10
    // For greater exponent (|q| > 22) this implementation fall-back to following
    // method:
    // - parse sign, integral, fractional and signed exponent part according
    //   to fortran F11.0 specification
    // - convert integral, fraction and signed exponent part to
    //   significand/mantissa and decimal exponent
    // - reconstruct a string decimal representation of the number
    // - rely on standard library for correct floating point number parsing
    let float = float.as_ref();
    // -> empty slice
    if float.is_empty() {
        return Err(ParseEndfFloatError);
    }
    // -> too long slice
    if float.len() > 11 {
        return Err(ParseEndfFloatError);
    }
    // - float.len() <= 11 => no mantissa i64 overflow (i64 max digits = 19 > 11)
    let mut iter = float.iter().filter(|&b| *b != b' ').peekable();
    // extract sign
    let negative = match iter.peek() {
        // -> blank slice
        None => return Ok(0.),
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
        return Ok(0.);
    }
    // parse mantissa and exponent
    let mut mantissa = 0;
    let mut exponent = 0;
    // parse integral part
    loop {
        match iter.peek() {
            Some(&byte) if byte.is_ascii_digit() => {
                mantissa = mantissa * 10 + (byte - b'0') as i64; // no overflow
                iter.next();
            }
            _ => break,
        }
    }
    // parse fractional part
    if iter.peek() == Some(&&b'.') {
        iter.next();
        loop {
            match iter.peek() {
                Some(&byte) if byte.is_ascii_digit() => {
                    mantissa = mantissa * 10 + (byte - b'0') as i64; // no overflow
                    exponent -= 1; // reduce exponent
                    iter.next();
                }
                _ => break,
            }
        }
    }

    // parse exponential part
    // support 'd' and 'D' for legacy compatibility
    let mut exp_sep = false;
    match iter.peek() {
        Some(b'e') | Some(b'E') | Some(b'd') | Some(b'D') => {
            exp_sep = true;
            iter.next();
        }
        _ => {}
    }
    let negative_exponent = match iter.peek() {
        Some(b'-') => {
            exp_sep = true;
            iter.next();
            true
        }
        Some(b'+') => {
            exp_sep = true;
            iter.next();
            false
        }
        _ => false,
    };
    // -> empty exponential part
    if exp_sep && iter.peek().is_none() {
        return Err(ParseEndfFloatError);
    }
    let mut exp = 0;
    loop {
        match iter.peek() {
            Some(&byte) if byte.is_ascii_digit() => {
                exp = exp * 10 + (byte - b'0') as i32; // no overflow
                iter.next();
            }
            _ => break,
        }
    }
    // -> partial
    if iter.peek().is_some() {
        return Err(ParseEndfFloatError);
    }
    // fast return
    if mantissa == 0 {
        return Ok(0.);
    }
    // compute exponent
    if negative_exponent {
        exponent -= exp;
    } else {
        exponent += exp;
    }
    // fall back to std library for correct float parsing (IEEE 754) if |exponent| > 22
    let mut value = if exponent.abs() > 22 {
        let float = format!("{mantissa}e{exponent}");
        match float.parse() {
            Ok(value) => value,
            Err(_) => return Err(ParseEndfFloatError),
        }
    } else {
        let mut value = mantissa as f64;
        if exponent < 0 {
            value /= POW_10_TABLE[-exponent as usize]
        } else {
            value *= POW_10_TABLE[exponent as usize]
        }
        value
    };
    // apply sign
    if negative {
        value = -value;
    }
    Ok(value)
}

/// Error returned when parsing an ENDF float with [`parse_endf_float`] fails.
#[derive(Debug)]
pub struct ParseEndfFloatError;

impl Display for ParseEndfFloatError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "parse ENDF float error")
    }
}

impl Error for ParseEndfFloatError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_endf_float_eq(str: &str, value: f64) {
        let float = parse_endf_float(str).unwrap_or_else(|_| panic!("error parsing \"{str}\""));
        // float parsing should be exact
        assert_eq!(float, value);
    }

    #[test]
    fn empty_slice() {
        assert!(parse_endf_float("").is_err());
    }

    #[test]
    fn too_long() {
        assert!(parse_endf_float(" 1.234567890").is_err());
        assert!(parse_endf_float("-1.23456E-12").is_err());
    }

    #[test]
    fn exponential_separator_only() {
        assert!(parse_endf_float("e").is_err());
        assert!(parse_endf_float("E").is_err());
        assert!(parse_endf_float("d").is_err());
        assert!(parse_endf_float("D").is_err());
    }

    #[test]
    fn invalid_sign() {
        assert!(parse_endf_float("*1.23").is_err());
    }

    #[test]
    fn invalid_digit() {
        assert!(parse_endf_float("a.23").is_err());
        assert!(parse_endf_float("1.2a").is_err());
        assert!(parse_endf_float("1.23e-a").is_err());
    }

    #[test]
    fn invalid_integral_fractional_separator() {
        assert!(parse_endf_float("1,23").is_err());
    }

    #[test]
    fn invalid_exponential_separator() {
        assert!(parse_endf_float("1.23f4").is_err());
    }

    #[test]
    fn empty_exponential_part() {
        assert!(parse_endf_float("1.2E").is_err());
        assert!(parse_endf_float("1.2-").is_err());
        assert!(parse_endf_float("1.2+").is_err());
    }

    #[test]
    fn blank() {
        assert_endf_float_eq(" ", 0.);
        assert_endf_float_eq("  ", 0.);
        assert_endf_float_eq("   ", 0.);
        assert_endf_float_eq("    ", 0.);
        assert_endf_float_eq("     ", 0.);
        assert_endf_float_eq("      ", 0.);
        assert_endf_float_eq("       ", 0.);
        assert_endf_float_eq("        ", 0.);
        assert_endf_float_eq("         ", 0.);
        assert_endf_float_eq("          ", 0.);
        assert_endf_float_eq("           ", 0.);
    }

    #[test]
    fn standard() {
        assert_endf_float_eq("        0.0", 0.);
        assert_endf_float_eq("       +0.0", 0.);
        assert_endf_float_eq("       -0.0", -0.);
        assert_endf_float_eq("        1.0", 1.);
        assert_endf_float_eq("       +1.0", 1.);
        assert_endf_float_eq("       -1.0", -1.);
    }

    #[test]
    fn space_padding() {
        assert_endf_float_eq("1.0        ", 1.);
        assert_endf_float_eq("    1.0    ", 1.);
        assert_endf_float_eq("        1.0", 1.);
    }

    #[test]
    fn zero_padding() {
        assert_endf_float_eq(" 1.00000000", 1.);
        assert_endf_float_eq(" 0001.00000", 1.);
        assert_endf_float_eq(" 00000001.0", 1.);
    }

    #[test]
    fn decimal_separator_only() {
        assert_endf_float_eq("          .", 0.);
    }

    #[test]
    fn sign_only() {
        assert_endf_float_eq("          +", 0.);
        assert_endf_float_eq("          -", 0.);
    }

    #[test]
    fn sign_decimal_separator_only() {
        assert_endf_float_eq("         +.", 0.);
        assert_endf_float_eq("         -.", -0.);
    }

    #[test]
    fn exponent_padding() {
        assert_endf_float_eq("  1E0000000", 1.);
        assert_endf_float_eq(" 1E+0000000", 1.);
        assert_endf_float_eq(" 1E-0000000", 1.);
    }

    #[test]
    fn exponent_only() {
        assert_endf_float_eq("E1", 0.);
        assert_endf_float_eq("E12", 0.);
        assert_endf_float_eq("E123", 0.);
        assert_endf_float_eq("E+1", 0.);
        assert_endf_float_eq("E+12", 0.);
        assert_endf_float_eq("E+123", 0.);
        assert_endf_float_eq("E-1", 0.);
        assert_endf_float_eq("E-12", 0.);
        assert_endf_float_eq("E-123", 0.);
        assert_endf_float_eq("E1", 0.);
        assert_endf_float_eq("+.E1", 0.);
        assert_endf_float_eq("-.E1", 0.);
    }

    #[test]
    fn empty_integral_part() {
        assert_endf_float_eq(".1e-2", 0.1e-2);
    }

    #[test]
    fn empty_fractional_part() {
        assert_endf_float_eq("1.e-2", 1.0e-2);
    }

    #[test]
    fn decimal_separator_end() {
        assert_endf_float_eq(" 0.", 0.);
        assert_endf_float_eq("-1.", -1.);
        assert_endf_float_eq("+1.", 1.);
    }

    #[test]
    fn space_ignored() {
        assert_endf_float_eq("1    .    2", 1.2);
        assert_endf_float_eq("1 . 2 E - 3", 1.2e-3);
        assert_endf_float_eq("1    E    2", 1.0e2);
        assert_endf_float_eq("1.   E   -2", 1.0e-2);
        assert_endf_float_eq("- 1 . E + 2", -1.0e2);
        assert_endf_float_eq("+ 1 . E - 2", 1.0e-2);
        assert_endf_float_eq("  - 1 . 2  ", -1.2);
        assert_endf_float_eq("1 2 3 4 . 5", 1.2345e3);
        assert_endf_float_eq("1 2 . 3 4 5", 1.2345e1);
        assert_endf_float_eq("1 . 2 e 1 2", 1.2e12);
    }

    #[test]
    fn integers() {
        assert_endf_float_eq(" 1", 1.);
        assert_endf_float_eq(" 12", 12.);
        assert_endf_float_eq(" 123", 123.);
        assert_endf_float_eq(" 1234", 1234.);
        assert_endf_float_eq(" 12345", 12345.);
        assert_endf_float_eq(" 123456", 123456.);
        assert_endf_float_eq(" 1234567", 1234567.);
        assert_endf_float_eq(" 12345678", 12345678.);
        assert_endf_float_eq(" 123456789", 123456789.);
        assert_endf_float_eq(" 1234567890", 1234567890.);
        assert_endf_float_eq("-1", -1.);
        assert_endf_float_eq("-12", -12.);
        assert_endf_float_eq("-123", -123.);
        assert_endf_float_eq("-1234", -1234.);
        assert_endf_float_eq("-12345", -12345.);
        assert_endf_float_eq("-123456", -123456.);
        assert_endf_float_eq("-1234567", -1234567.);
        assert_endf_float_eq("-12345678", -12345678.);
        assert_endf_float_eq("-123456789", -123456789.);
        assert_endf_float_eq("-1234567890", -1234567890.);
        assert_endf_float_eq("+1", 1.);
        assert_endf_float_eq("+12", 12.);
        assert_endf_float_eq("+123", 123.);
        assert_endf_float_eq("+1234", 1234.);
        assert_endf_float_eq("+12345", 12345.);
        assert_endf_float_eq("+123456", 123456.);
        assert_endf_float_eq("+1234567", 1234567.);
        assert_endf_float_eq("+12345678", 12345678.);
        assert_endf_float_eq("+123456789", 123456789.);
        assert_endf_float_eq("+1234567890", 1234567890.);
    }

    #[test]
    fn floats() {
        assert_endf_float_eq(" 1.", 1.);
        assert_endf_float_eq(" 1.0", 1.);
        assert_endf_float_eq(" 1.2", 1.2);
        assert_endf_float_eq(" 1.23", 1.23);
        assert_endf_float_eq(" 1.234", 1.234);
        assert_endf_float_eq(" 1.2345", 1.2345);
        assert_endf_float_eq(" 1.23456", 1.23456);
        assert_endf_float_eq(" 1.234567", 1.234567);
        assert_endf_float_eq(" 1.2345678", 1.2345678);
        assert_endf_float_eq(" 1.23456789", 1.23456789);
        assert_endf_float_eq("-1.", -1.);
        assert_endf_float_eq("-1.0", -1.);
        assert_endf_float_eq("-1.2", -1.2);
        assert_endf_float_eq("-1.23", -1.23);
        assert_endf_float_eq("-1.234", -1.234);
        assert_endf_float_eq("-1.2345", -1.2345);
        assert_endf_float_eq("-1.23456", -1.23456);
        assert_endf_float_eq("-1.234567", -1.234567);
        assert_endf_float_eq("-1.2345678", -1.2345678);
        assert_endf_float_eq("-1.23456789", -1.23456789);
        assert_endf_float_eq("+1.", 1.);
        assert_endf_float_eq("+1.0", 1.);
        assert_endf_float_eq("+1.2", 1.2);
        assert_endf_float_eq("+1.23", 1.23);
        assert_endf_float_eq("+1.234", 1.234);
        assert_endf_float_eq("+1.2345", 1.2345);
        assert_endf_float_eq("+1.23456", 1.23456);
        assert_endf_float_eq("+1.234567", 1.234567);
        assert_endf_float_eq("+1.2345678", 1.2345678);
        assert_endf_float_eq("+1.23456789", 1.23456789);
    }

    #[test]
    fn scientific_one_digit() {
        assert_endf_float_eq(" 1.0E+1", 1.0e+1);
        assert_endf_float_eq(" 1.2E+1", 1.2e+1);
        assert_endf_float_eq(" 1.23E+1", 1.23e+1);
        assert_endf_float_eq(" 1.234E+1", 1.234e+1);
        assert_endf_float_eq(" 1.2345E+1", 1.2345e+1);
        assert_endf_float_eq(" 1.23456E+1", 1.23456e+1);
        assert_endf_float_eq(" 1.0+1", 1.0e+1);
        assert_endf_float_eq(" 1.2+1", 1.2e+1);
        assert_endf_float_eq(" 1.23+1", 1.23e+1);
        assert_endf_float_eq(" 1.234+1", 1.234e+1);
        assert_endf_float_eq(" 1.2345+1", 1.2345e+1);
        assert_endf_float_eq(" 1.23456+1", 1.23456e+1);
        assert_endf_float_eq(" 1.234567+1", 1.234567e+1);
        assert_endf_float_eq(" 1.0e+1", 1.0e+1);
        assert_endf_float_eq(" 1.0D+1", 1.0e+1);
        assert_endf_float_eq(" 1.0d+1", 1.0e+1);
    }

    #[test]
    fn scientific_two_digits() {
        assert_endf_float_eq(" 1.0E+01", 1.0e+1);
        assert_endf_float_eq(" 1.2E+01", 1.2e+1);
        assert_endf_float_eq(" 1.23E+01", 1.23e+1);
        assert_endf_float_eq(" 1.234E+01", 1.234e+1);
        assert_endf_float_eq(" 1.2345E+01", 1.2345e+1);
        assert_endf_float_eq(" 1.0+01", 1.0e+1);
        assert_endf_float_eq(" 1.2+01", 1.2e+1);
        assert_endf_float_eq(" 1.23+01", 1.23e+1);
        assert_endf_float_eq(" 1.234+01", 1.234e+1);
        assert_endf_float_eq(" 1.2345+01", 1.2345e+1);
        assert_endf_float_eq(" 1.23456+01", 1.23456e+1);
        assert_endf_float_eq(" 1.0e+01", 1.0e+1);
        assert_endf_float_eq(" 1.0D+01", 1.0e+1);
        assert_endf_float_eq(" 1.0d+01", 1.0e+1);
    }

    #[test]
    fn scientific_three_digits() {
        assert_endf_float_eq(" 1.0E+001", 1.0e+1);
        assert_endf_float_eq(" 1.2E+001", 1.2e+1);
        assert_endf_float_eq(" 1.23E+001", 1.23e+1);
        assert_endf_float_eq(" 1.234E+001", 1.234e+1);
        assert_endf_float_eq(" 1.0+001", 1.0e+1);
        assert_endf_float_eq(" 1.2+001", 1.2e+1);
        assert_endf_float_eq(" 1.23+001", 1.23e+1);
        assert_endf_float_eq(" 1.234+001", 1.234e+1);
        assert_endf_float_eq(" 1.2345+001", 1.2345e+1);
        assert_endf_float_eq(" 1.0e+001", 1.0e+1);
        assert_endf_float_eq(" 1.0D+001", 1.0e+1);
        assert_endf_float_eq(" 1.0d+001", 1.0e+1);
    }

    #[test]
    fn large_exponent() {
        assert_endf_float_eq("1.234567E23", 1.234567e23);
        assert_endf_float_eq("1.23456E123", 1.23456e123);
        assert_endf_float_eq("-1.23456E23", -1.23456e23);
        assert_endf_float_eq("-1.2345E123", -1.2345e123);
        assert_endf_float_eq("1.23456E-23", 1.23456e-23);
        assert_endf_float_eq("1.2345E-123", 1.2345e-123);
        assert_endf_float_eq("-1.2345E-23", -1.2345e-23);
        assert_endf_float_eq("-1.234E-123", -1.234e-123);
    }
}
