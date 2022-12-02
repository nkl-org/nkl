//! A Compact ENDF (ACE) format module.

use std::error::Error as StdError;
use std::fmt::Display;
use std::io::{Error as IOError, Read};

/// ACE Table.
#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    id: String,
    atomic_weight_ratio: f64,
    temperature: f64,
    izaw: Vec<(u32, f64)>,
    nxs: Vec<usize>,
    jxs: Vec<usize>,
    xss: Vec<f64>,
}

impl Table {
    /// Returns table's id.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns table's atomic weight ratio.
    pub fn atomic_weight_ratio(&self) -> f64 {
        self.atomic_weight_ratio
    }

    /// Returns table's temperature.
    pub fn temperature(&self) -> f64 {
        self.temperature
    }

    /// Returns table's izaw array.
    pub fn izaw(&self) -> &[(u32, f64)] {
        &self.izaw
    }

    /// Returns table's nxs array.
    pub fn nxs(&self) -> &[usize] {
        &self.nxs
    }

    /// Returns table's jxs array.
    pub fn jxs(&self) -> &[usize] {
        &self.jxs
    }

    /// Returns table's xss array.
    pub fn xss(&self) -> &[f64] {
        &self.xss
    }
}

/// Parse ACE table.
pub fn parse_table<R: Read>(mut table: R) -> Result<Table, AceError> {
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
    Ok(Table {
        id,
        atomic_weight_ratio,
        temperature,
        izaw: Vec::new(),
        nxs: Vec::new(),
        jxs: Vec::new(),
        xss: Vec::new(),
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
    Ok(Table {
        id,
        atomic_weight_ratio,
        temperature,
        izaw: Vec::new(),
        nxs: Vec::new(),
        jxs: Vec::new(),
        xss: Vec::new(),
    })
}

/// The error type for [`ace`](crate::data::ace) module.
#[derive(Debug)]
pub enum AceError {
    /// Invalid data.
    Data,
    /// Reached end of file.
    EndOfFile,
    /// Invalid format.
    Format,
    /// I/O error.
    IO(IOError),
}

impl Display for AceError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AceError::Data => write!(fmt, "invalid ACE data"),
            AceError::EndOfFile => write!(fmt, "reached end of ACE file"),
            AceError::Format => write!(fmt, "invalid ACE format"),
            AceError::IO(_) => write!(fmt, "ACE I/O error"),
        }
    }
}

impl StdError for AceError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            AceError::IO(error) => Some(error),
            _ => None,
        }
    }
}

impl From<IOError> for AceError {
    fn from(error: IOError) -> Self {
        AceError::IO(error)
    }
}
