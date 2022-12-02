use std::io::Read;
use std::str::Lines;

use super::{AceError, Table};

/// Parse ACE table.
pub fn parse_ace_table<R: Read>(mut table: R) -> Result<Table, AceError> {
    let mut ace = String::new();
    table.read_to_string(&mut ace)?;
    let Some(line) = ace.lines().next() else {
        return Err(AceError::EndOfFile)
    };
    if line.starts_with("2.") {
        parse_table_version2(ace)
    } else {
        parse_table_version1(ace)
    }
}

fn parse_table_version1(ace: String) -> Result<Table, AceError> {
    let mut iter = ace.lines();
    let Some(line) = iter.next() else {
        return Err(AceError::EndOfFile)
    };
    let id = line[..10].trim().to_owned();
    let Ok(atomic_weight_ratio) = line[10..22].trim().parse() else {
        return Err(AceError::Format)
    };
    let Ok(temperature) = line[22..34].trim().parse() else {
        return Err(AceError::Format)
    };
    iter.next();
    let izaw = parse_izaw_array(&mut iter)?;
    let nxs = parse_nxs_array(&mut iter)?;
    let jxs = parse_jxs_array(&mut iter)?;
    let xss = parse_xss_array(&mut iter, nxs[0])?;
    Ok(Table {
        id,
        atomic_weight_ratio,
        temperature,
        izaw,
        nxs,
        jxs,
        xss,
    })
}

fn parse_table_version2(ace: String) -> Result<Table, AceError> {
    let mut iter = ace.lines();
    let Some(line) = iter.next() else {
        return Err(AceError::EndOfFile)
    };
    let id = line[11..35].trim().to_owned();
    let Some(line) = iter.next() else {
        return Err(AceError::EndOfFile)
    };
    let Ok(atomic_weight_ratio) = line[..12].trim().parse() else {
        return Err(AceError::Format)
    };
    let Ok(temperature) = line[13..25].trim().parse() else {
        return Err(AceError::Format)
    };
    let Ok(comment) = line[37..].trim().parse() else {
        return Err(AceError::Format)
    };
    for _ in 0..comment {
        iter.next();
    }
    let izaw = parse_izaw_array(&mut iter)?;
    let nxs = parse_nxs_array(&mut iter)?;
    let jxs = parse_jxs_array(&mut iter)?;
    let xss = parse_xss_array(&mut iter, nxs[0])?;
    Ok(Table {
        id,
        atomic_weight_ratio,
        temperature,
        izaw,
        nxs,
        jxs,
        xss,
    })
}

fn parse_izaw_array(lines: &mut Lines) -> Result<Vec<(u32, f64)>, AceError> {
    let mut izaw = Vec::with_capacity(16);
    for _ in 0..4 {
        let Some(line) = lines.next() else {
            return Err(AceError::EndOfFile)
        };
        for i in 0..4 {
            let mut start = i * 18;
            let mut stop = start + 7;
            let Ok(iz) = line[start..stop].trim().parse() else {
                return Err(AceError::Format)
            };
            start = stop;
            stop = start + 11;
            let Ok(aw) = line[start..stop].trim().parse() else {
                return Err(AceError::Format)
            };
            izaw.push((iz, aw));
        }
    }
    Ok(izaw)
}

fn parse_nxs_array(lines: &mut Lines) -> Result<Vec<usize>, AceError> {
    let mut nxs = Vec::with_capacity(16);
    for _ in 0..2 {
        let Some(line) = lines.next() else {
            return Err(AceError::EndOfFile)
        };
        for i in 0..8 {
            let start = i * 9;
            let stop = i * 9 + 9;
            let Ok(integer) = line[start..stop].trim().parse() else {
                return Err(AceError::Format)
            };
            nxs.push(integer);
        }
    }
    Ok(nxs)
}

fn parse_jxs_array(lines: &mut Lines) -> Result<Vec<usize>, AceError> {
    let mut nxs = Vec::with_capacity(16);
    for _ in 0..4 {
        let Some(line) = lines.next() else {
            return Err(AceError::EndOfFile)
        };
        for i in 0..8 {
            let start = i * 9;
            let stop = i * 9 + 9;
            let Ok(integer) = line[start..stop].trim().parse() else {
                return Err(AceError::Format)
            };
            nxs.push(integer);
        }
    }
    Ok(nxs)
}

fn parse_xss_array(lines: &mut Lines, size: usize) -> Result<Vec<f64>, AceError> {
    let mut xss = Vec::with_capacity(size);
    for line in lines {
        for i in 0..4 {
            let start = i * 20;
            let stop = i * 20 + 20;
            let Ok(float) = line[start..stop].trim().parse() else {
                return Err(AceError::Format)
            };
            xss.push(float);
        }
    }
    Ok(xss)
}
