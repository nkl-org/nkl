//! Evaluated Nuclear Data File (ENDF) module.
//!
//! # References
//!
//! Trkov, A., Herman, M., & Brown, D. A. (2012). *ENDF-6 formats manual*. Brookhaven National Laboratory, 80.

// Error
mod error;
pub use error::EndfError;

// Primitives
mod integer;
pub use integer::{parse_endf_integer, ParseEndfIntegerError};

mod float;
pub use float::{parse_endf_float, ParseEndfFloatError};

// Records
mod records;
pub use records::{Cont, Intg, List, Tab1, Tab2, Text};

// Reader
mod read;
pub use read::EndfReader;

/// Parse ENDF integer at specified column in `record`.
///
/// # Format
///
/// Refer to [`parse_endf_integer`] documentation for ENDF integer format.
///
/// # Panics
///
/// Panics if invalid `column` index: `column` ∉ `[1, 6]`
///
/// # Errors
///
/// [`EndfError`] is returned if:
/// - invalid format of the record
/// - invalid data of the record
/// - invalid integer format
///
/// # Examples
///
/// ```
/// use nkl::data::endf::parse_integer;
/// let record = " 1.23456789-1.23456789          1          2          3          412341212312345";
/// assert_eq!(parse_integer(record, 3).unwrap(), 1);
/// ```
pub fn parse_integer<R: AsRef<[u8]>>(record: R, column: usize) -> Result<i64, EndfError> {
    assert!(column > 0);
    assert!(column <= 6);
    let start = (column - 1) * 11;
    let stop = column * 11;
    match record.as_ref().get(start..stop) {
        Some(slice) => match parse_endf_integer(slice) {
            Ok(integer) => Ok(integer),
            Err(_) => Err(EndfError::Data),
        },
        None => Err(EndfError::Format),
    }
}

/// Parse ENDF float at specified column in `record`.
///
/// # Format
///
/// Refer to [`parse_endf_float`] documentation for ENDF float format.
///
/// # Panics
///
/// Panics if invalid `column` index: `column` ∉ `[1, 6]`
///
/// # Errors
///
/// [`EndfError`] is returned if:
/// - invalid format of the record
/// - invalid data of the record
/// - invalid float format
///
/// # Examples
///
/// ```
/// use nkl::data::endf::parse_float;
/// let record = " 1.23456789-1.23456789          1          2          3          412341212312345";
/// assert_eq!(parse_float(record, 1).unwrap(), 1.23456789);
/// ```
pub fn parse_float<R: AsRef<[u8]>>(record: R, column: usize) -> Result<f64, EndfError> {
    assert!(column > 0);
    assert!(column <= 6);
    let start = (column - 1) * 11;
    let stop = column * 11;
    match record.as_ref().get(start..stop) {
        Some(slice) => match parse_endf_float(slice) {
            Ok(float) => Ok(float),
            Err(_) => Err(EndfError::Data),
        },
        None => Err(EndfError::Format),
    }
}

type Record = (f64, f64, i64, i64, i64, i64, i32, u32, u32, Option<u32>);

/// Parse ENDF record.
///
/// # Format
///
/// This function assume following format:
///
/// ```text
/// record = float[11] float[11] integer[11] integer[11] integer[11] integer[11] mat[4] mf[2] mt[3] ns[5]?
/// ```
///
/// Integers and floating point numbers format are described in relevant
/// sections available in
/// [`parse_endf_integer`](crate::data::endf::parse_endf_integer) and
/// [`parse_endf_float`](crate::data::endf::parse_endf_float).
///
/// # Errors
///
/// [`EndfError`] is returned if:
/// - invalid format of the record
/// - invalid data of the record
/// - parsing float from column 1 or 2 failed
/// - parsing integer from column 3-6 failed
/// - parsing MAT/MF/MT/NS control number failed
///
/// # Examples
///
/// ```
/// use nkl::data::endf::parse_record;
/// let record = " 1.23456789-1.23456789          1          2          3          412341212312345";
/// let (c1, c2, l1, l2, n1, n2, mat, mf, mt, ns) = parse_record(record.as_bytes()).unwrap();
/// assert_eq!(c1, 1.23456789);
/// assert_eq!(c2, -1.23456789);
/// assert_eq!(l1, 1);
/// assert_eq!(l2, 2);
/// assert_eq!(n1, 3);
/// assert_eq!(n2, 4);
/// assert_eq!(mat, 1234);
/// assert_eq!(mf, 12);
/// assert_eq!(mt, 123);
/// assert_eq!(ns, Some(12345))
/// ```
pub fn parse_record<R: AsRef<[u8]>>(record: R) -> Result<Record, EndfError> {
    let record = record.as_ref();
    let c1 = parse_float(record, 1)?;
    let c2 = parse_float(record, 2)?;
    let l1 = parse_integer(record, 3)?;
    let l2 = parse_integer(record, 4)?;
    let n1 = parse_integer(record, 5)?;
    let n2 = parse_integer(record, 6)?;
    let mat = parse_material(record)?;
    let mf = parse_file(record)?;
    let mt = parse_section(record)?;
    let ns = parse_sequence(record)?;
    Ok((c1, c2, l1, l2, n1, n2, mat, mf, mt, ns))
}

type ControlNumbers = (i32, u32, u32, Option<u32>);

/// Parse ENDF record control numbers.
///
/// # Format
///
/// This function assume following format:
///
/// ```text
/// record = any[66] mat[4] mf[2] mt[3] ns[5]?
/// ```
///
/// Integers format are described in relevant section available in
/// [`parse_endf_integer`](crate::data::endf::parse_endf_integer).
///
/// # Errors
///
/// [`EndfError`] is returned if:
/// - invalid format of the record
/// - invalid data of the record
/// - parsing MAT/MF/MT/NS control number failed
///
/// # Examples
///
/// ```
/// use nkl::data::endf::parse_control_numbers;
/// let record = " 1.23456789-1.23456789          1          2          3          412341212312345";
/// let (mat, mf, mt, ns) = parse_control_numbers(record).unwrap();
/// assert_eq!(mat, 1234);
/// assert_eq!(mf, 12);
/// assert_eq!(mt, 123);
/// assert_eq!(ns, Some(12345))
/// ```
pub fn parse_control_numbers<R: AsRef<[u8]>>(record: R) -> Result<ControlNumbers, EndfError> {
    let record = record.as_ref();
    let mat = parse_material(record)?;
    let mf = parse_file(record)?;
    let mt = parse_section(record)?;
    let ns = parse_sequence(record)?;
    Ok((mat, mf, mt, ns))
}

/// Parse ENDF *MAT* material control number in `record`.
///
/// # Format
///
/// The *MAT* control number is given in columns 67-70 of the record.
///
/// ENDF integer format is specified in
/// [`parse_endf_integer`](crate::data::endf::parse_endf_integer).
///
/// # Errors
///
/// [`EndfError`] is returned if:
/// - invalid record format
/// - parsing MAT control number failed
///
/// # Examples
///
/// ```
/// use nkl::data::endf::parse_material;
/// let record = " 1.23456789-1.23456789          1          2          3          412341212312345";
/// let mat = parse_material(record.as_bytes()).unwrap();
/// assert_eq!(mat, 1234);
/// ```
pub fn parse_material<R: AsRef<[u8]>>(record: R) -> Result<i32, EndfError> {
    let record = record.as_ref();
    match record.get(66..70) {
        Some(slice) => match parse_endf_integer(slice) {
            Ok(integer) => Ok(integer as i32),
            Err(_) => Err(EndfError::Data),
        },
        None => Err(EndfError::Format),
    }
}

/// Parse ENDF *MF* file control number in `record`.
///
/// # Format
///
/// The *MF* control number is given in columns 71-72 of the record.
///
/// ENDF integer format is specified in
/// [`parse_endf_integer`](crate::data::endf::parse_endf_integer).
///
/// # Errors
///
/// [`EndfError`] is returned if:
/// - invalid record format
/// - parsing MF control number failed
///
/// # Examples
///
/// ```
/// use nkl::data::endf::parse_file;
/// let record = " 1.23456789-1.23456789          1          2          3          412341212312345";
/// let mf = parse_file(record.as_bytes()).unwrap();
/// assert_eq!(mf, 12);
/// ```
pub fn parse_file<R: AsRef<[u8]>>(record: R) -> Result<u32, EndfError> {
    let record = record.as_ref();
    match record.get(70..72) {
        Some(slice) => match parse_endf_integer(slice) {
            Ok(integer) => Ok(integer as u32),
            Err(_) => Err(EndfError::Data),
        },
        None => Err(EndfError::Format),
    }
}

/// Parse ENDF *MT* section control number in `record`.
///
/// # Format
///
/// The *MT* control number is given in columns 73-75 of the record.
///
/// ENDF integer format is specified in
/// [`parse_endf_integer`](crate::data::endf::parse_endf_integer).
///
/// # Errors
///
/// [`EndfError`] is returned if:
/// - invalid record format
/// - parsing MT control number failed
///
/// # Examples
///
/// ```
/// use nkl::data::endf::parse_section;
/// let record = " 1.23456789-1.23456789          1          2          3          412341212312345";
/// let mt = parse_section(record.as_bytes()).unwrap();
/// assert_eq!(mt, 123);
/// ```
pub fn parse_section<R: AsRef<[u8]>>(record: R) -> Result<u32, EndfError> {
    let record = record.as_ref();
    match record.get(72..75) {
        Some(slice) => match parse_endf_integer(slice) {
            Ok(integer) => Ok(integer as u32),
            Err(_) => Err(EndfError::Data),
        },
        None => Err(EndfError::Format),
    }
}

/// Parse ENDF optional *NS* sequence control number in `record`.
///
/// # Format
///
/// The *NS* control number is given in columns 76-80 of the record and is
/// optional.
///
/// ENDF integer format is specified in
/// [`parse_endf_integer`](crate::data::endf::parse_endf_integer).
///
/// # Errors
///
/// [`EndfError`] is returned if:
/// - invalid record format
/// - parsing NS control number failed
///
/// # Examples
///
/// ```
/// use nkl::data::endf::parse_sequence;
/// let record = " 1.23456789-1.23456789          1          2          3          412341212312345";
/// let ns = parse_sequence(record.as_bytes()).unwrap();
/// assert_eq!(ns, Some(12345));
/// ```
pub fn parse_sequence<R: AsRef<[u8]>>(record: R) -> Result<Option<u32>, EndfError> {
    let record = record.as_ref();
    match record.get(75..80) {
        Some(slice) => match parse_endf_integer(slice) {
            Ok(integer) => Ok(Some(integer as u32)),
            Err(_) => Err(EndfError::Data),
        },
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer() {
        let record =
            "          1         -1 1234567890-1234567890 1         -1         12341212312345";
        assert_eq!(parse_integer(record, 1).unwrap(), 1);
        assert_eq!(parse_integer(record, 2).unwrap(), -1);
        assert_eq!(parse_integer(record, 3).unwrap(), 1234567890);
        assert_eq!(parse_integer(record, 4).unwrap(), -1234567890);
        assert_eq!(parse_integer(record, 5).unwrap(), 1);
        assert_eq!(parse_integer(record, 6).unwrap(), -1);
    }

    #[test]
    fn float() {
        let record =
            " 1.23456789-1.23456789 1.35790+12-1.35790-12 2.4680E+13-2.4680E-1312341212312345";
        assert_eq!(parse_float(record, 1).unwrap(), 1.23456789);
        assert_eq!(parse_float(record, 2).unwrap(), -1.23456789);
        assert_eq!(parse_float(record, 3).unwrap(), 1.35790E12);
        assert_eq!(parse_float(record, 4).unwrap(), -1.35790E-12);
        assert_eq!(parse_float(record, 5).unwrap(), 2.4680E+13);
        assert_eq!(parse_float(record, 6).unwrap(), -2.4680E-13);
    }

    #[test]
    fn controls() {
        let record =
            " 1.23456789-1.23456789          1          2          3          412341212312345";
        let (mat, mf, mt, ns) = parse_control_numbers(record).unwrap();
        assert_eq!(mat, 1234);
        assert_eq!(mf, 12);
        assert_eq!(mt, 123);
        assert_eq!(ns, Some(12345));
    }

    #[test]
    fn record() {
        let record =
            " 1.23456789-1.23456789          1          2          3          412341212312345";
        let (c1, c2, l1, l2, n1, n2, mat, mf, mt, ns) = parse_record(record.as_bytes()).unwrap();
        assert_eq!(c1, 1.23456789);
        assert_eq!(c2, -1.23456789);
        assert_eq!(l1, 1);
        assert_eq!(l2, 2);
        assert_eq!(n1, 3);
        assert_eq!(n2, 4);
        assert_eq!(mat, 1234);
        assert_eq!(mf, 12);
        assert_eq!(mt, 123);
        assert_eq!(ns, Some(12345))
    }

    #[test]
    fn material() {
        let record =
            " 1.23456789-1.23456789          1          2          3          412341212312345";
        let mat = parse_material(record.as_bytes()).unwrap();
        assert_eq!(mat, 1234);
    }

    #[test]
    fn file() {
        let record =
            " 1.23456789-1.23456789          1          2          3          412341212312345";
        let mf = parse_file(record.as_bytes()).unwrap();
        assert_eq!(mf, 12);
    }

    #[test]
    fn section() {
        let record =
            " 1.23456789-1.23456789          1          2          3          412341212312345";
        let mt = parse_section(record.as_bytes()).unwrap();
        assert_eq!(mt, 123);
    }

    #[test]
    fn sequence_some() {
        let record =
            " 1.23456789-1.23456789          1          2          3          412341212312345";
        let ns = parse_sequence(record.as_bytes()).unwrap();
        assert_eq!(ns, Some(12345));
    }

    #[test]
    fn sequence_none() {
        let record = " 1.23456789-1.23456789          1          2          3          4123412123";
        let ns = parse_sequence(record.as_bytes()).unwrap();
        assert_eq!(ns, None);
    }
}
