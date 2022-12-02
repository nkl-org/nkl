use std::error::Error;
use std::io::Cursor;

use nkl::data::ace::parse_table;

const IZAW: [(u32, f64); 16] = [
    (1, 1.0),
    (2, 2.0),
    (3, 3.0),
    (4, 4.0),
    (5, 5.0),
    (6, 6.0),
    (7, 7.0),
    (8, 8.0),
    (9, 9.0),
    (10, 10.0),
    (11, 11.0),
    (12, 12.0),
    (13, 13.0),
    (14, 14.0),
    (15, 15.0),
    (16, 16.0),
];

const NXS: [usize; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
const JXS: [usize; 32] = [
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31, 32,
];
const XSS: [f64; 4] = [1.0, 2.0, 3.0, 4.0];

#[test]
fn version1() -> Result<(), Box<dyn Error>> {
    let ace = include_bytes!("data/version1.ace");
    let cursor = Cursor::new(ace);
    let table = parse_table(cursor)?;
    assert_eq!(table.id(), "12345.12c");
    assert_eq!(table.atomic_weight_ratio(), 123.1234567);
    assert_eq!(table.temperature(), 1.23456E-12);
    assert_eq!(table.izaw(), IZAW);
    assert_eq!(table.nxs(), NXS);
    assert_eq!(table.jxs(), JXS);
    assert_eq!(table.xss(), XSS);
    Ok(())
}

#[test]
fn version2() -> Result<(), Box<dyn Error>> {
    let ace = include_bytes!("data/version2.ace");
    let cursor = Cursor::new(ace);
    let table = parse_table(cursor)?;
    assert_eq!(table.id(), "1123123.123c");
    assert_eq!(table.atomic_weight_ratio(), 123.1234567);
    assert_eq!(table.temperature(), 1.23456E-12);
    assert_eq!(table.izaw(), IZAW);
    assert_eq!(table.nxs(), NXS);
    assert_eq!(table.jxs(), JXS);
    assert_eq!(table.xss(), XSS);
    Ok(())
}
