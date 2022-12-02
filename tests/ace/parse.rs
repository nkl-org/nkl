use std::error::Error;
use std::io::Cursor;

use nkl::data::ace::parse_table;

#[test]
fn version1() -> Result<(), Box<dyn Error>> {
    let ace = include_bytes!("data/version1.ace");
    let cursor = Cursor::new(ace);
    let table = parse_table(cursor)?;
    assert_eq!(table.id(), "12345.12c");
    assert_eq!(table.atomic_weight_ratio(), 123.1234567);
    assert_eq!(table.temperature(), 1.23456E-12);
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
    Ok(())
}
