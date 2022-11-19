use std::{error::Error, io::Cursor};

use nkl::data::endf::{Cont, EndfReader, Intg, List, Tab1, Tab2, Text};

#[test]
fn line() -> Result<(), Box<dyn Error>> {
    let endf = include_bytes!("data/default.endf");
    let cursor = Cursor::new(endf);
    let mut reader = EndfReader::new(cursor);
    assert_eq!(
        reader.read_line()?,
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZ abcdefghijklmnopqrstuvwxyz 0123456789     1 0  0    0\n"
    );
    assert_eq!(
        reader.read_line()?,
        b" 1.00000000 2.00000000          1          2          3          4   1 1451    1\n"
    );
    assert_eq!(
        reader.read_line()?,
        b" 3.00000000 4.00000000          5          6          7          8   1 1451    2\n"
    );
    assert_eq!(
        reader.read_line()?,
        b" 5.00000000 6.00000000          9         10         11         12   1 1451    3\n"
    );
    assert_eq!(
        reader.read_line()?,
        b" 7.00000000 8.00000000         13         14         15         16   1 1451    4\n"
    );
    Ok(())
}

#[test]
fn cont() -> Result<(), Box<dyn Error>> {
    let endf = include_bytes!("data/cont.endf");
    let cursor = Cursor::new(endf);
    let mut reader = EndfReader::new(cursor);
    let Cont(c1, c2, l1, l2, n1, n2) = reader.read_cont()?;
    assert_eq!(c1, 1.);
    assert_eq!(c2, 2.);
    assert_eq!(l1, 1);
    assert_eq!(l2, 2);
    assert_eq!(n1, 3);
    assert_eq!(n2, 4);
    Ok(())
}

#[test]
fn dir() -> Result<(), Box<dyn Error>> {
    let endf = include_bytes!("data/dir.endf");
    let cursor = Cursor::new(endf);
    let mut reader = EndfReader::new(cursor);
    let Cont(c1, c2, l1, l2, n1, n2) = reader.read_cont()?;
    assert_eq!(c1, 0.);
    assert_eq!(c2, 0.);
    assert_eq!(l1, 1);
    assert_eq!(l2, 2);
    assert_eq!(n1, 3);
    assert_eq!(n2, 4);
    Ok(())
}

#[test]
fn end() -> Result<(), Box<dyn Error>> {
    let endf = include_bytes!("data/end.endf");
    let cursor = Cursor::new(endf);
    let mut reader = EndfReader::new(cursor);
    for _ in 0..4 {
        let Cont(c1, c2, l1, l2, n1, n2) = reader.read_cont()?;
        assert_eq!(c1, 0.);
        assert_eq!(c2, 0.);
        assert_eq!(l1, 0);
        assert_eq!(l2, 0);
        assert_eq!(n1, 0);
        assert_eq!(n2, 0);
    }
    Ok(())
}

#[test]
fn head() -> Result<(), Box<dyn Error>> {
    let endf = include_bytes!("data/dir.endf");
    let cursor = Cursor::new(endf);
    let mut reader = EndfReader::new(cursor);
    let Cont(c1, c2, l1, l2, n1, n2) = reader.read_cont()?;
    assert_eq!(c1, 0.);
    assert_eq!(c2, 0.);
    assert_eq!(l1, 1);
    assert_eq!(l2, 2);
    assert_eq!(n1, 3);
    assert_eq!(n2, 4);
    Ok(())
}

#[test]
fn intg() -> Result<(), Box<dyn Error>> {
    let endf = include_bytes!("data/intg.endf");
    let cursor = Cursor::new(endf);
    let mut reader = EndfReader::new(cursor);
    for ndigit in 2..=6 {
        let Intg(ii, jj, kij) = reader.read_intg(ndigit)?;
        assert_eq!(ii, 12345);
        assert_eq!(jj, 12345);
        let (value, len) = match ndigit {
            2 => (123, 18),
            3 => (1234, 13),
            4 => (12345, 11),
            5 => (123456, 9),
            6 => (1234567, 8),
            _ => unreachable!(),
        };
        assert_eq!(kij, vec![value; len]);
    }
    Ok(())
}

#[test]
fn list() -> Result<(), Box<dyn Error>> {
    let endf = include_bytes!("data/list.endf");
    let cursor = Cursor::new(endf);
    let mut reader = EndfReader::new(cursor);
    let List(c1, c2, l1, l2, npl, n2, b) = reader.read_list()?;
    assert_eq!(c1, 1.);
    assert_eq!(c2, 2.);
    assert_eq!(l1, 1);
    assert_eq!(l2, 2);
    assert_eq!(npl, 3);
    assert_eq!(n2, 4);
    assert_eq!(b.len(), npl);
    assert_eq!(b, vec![1., 2., 3.]);
    Ok(())
}

#[test]
fn tab1() -> Result<(), Box<dyn Error>> {
    let endf = include_bytes!("data/tab1.endf");
    let cursor = Cursor::new(endf);
    let mut reader = EndfReader::new(cursor);
    let Tab1(c1, c2, l1, l2, nr, np, int, tab) = reader.read_tab1()?;
    assert_eq!(c1, 1.);
    assert_eq!(c2, 2.);
    assert_eq!(l1, 1);
    assert_eq!(l2, 2);
    assert_eq!(nr, 2);
    assert_eq!(np, 4);
    assert_eq!(int.len(), nr);
    assert_eq!(tab.len(), np);
    assert_eq!(int, vec![(1, 2), (3, 4)]);
    assert_eq!(tab, vec![(1., 2.), (3., 4.), (5., 6.), (7., 8.)]);
    Ok(())
}

#[test]
fn tab2() -> Result<(), Box<dyn Error>> {
    let endf = include_bytes!("data/tab2.endf");
    let cursor = Cursor::new(endf);
    let mut reader = EndfReader::new(cursor);
    let Tab2(c1, c2, l1, l2, nr, nz, int) = reader.read_tab2()?;
    assert_eq!(c1, 1.);
    assert_eq!(c2, 2.);
    assert_eq!(l1, 1);
    assert_eq!(l2, 2);
    assert_eq!(nr, 3);
    assert_eq!(nz, 4);
    assert_eq!(int.len(), nr);
    assert_eq!(int, vec![(1, 2), (3, 4), (5, 6)]);
    Ok(())
}

#[test]
fn text() -> Result<(), Box<dyn Error>> {
    let endf = include_bytes!("data/text.endf");
    let cursor = Cursor::new(endf);
    let mut reader = EndfReader::new(cursor);
    let Text(hl) = reader.read_text()?;
    assert_eq!(
        hl,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ abcdefghijklmnopqrstuvwxyz 0123456789  "
    );
    Ok(())
}

#[test]
fn tpid() -> Result<(), Box<dyn Error>> {
    let endf = include_bytes!("data/tpid.endf");
    let cursor = Cursor::new(endf);
    let mut reader = EndfReader::new(cursor);
    let Text(hl) = reader.read_text()?;
    assert_eq!(
        hl,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ abcdefghijklmnopqrstuvwxyz 0123456789  "
    );
    Ok(())
}
