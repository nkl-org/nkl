use std::io::BufRead;

use super::{
    parse_endf_integer, parse_float, parse_integer, Cont, EndfError, Intg, List, Tab1, Tab2, Text,
};

// Maximum endf line length: 80 chars + optional `\r` + `\n`.
const ENDF_MAX_LINE_LENGTH: usize = 82;

/// Reader specialized for ENDF format files.
#[derive(Debug)]
pub struct EndfReader<B: BufRead> {
    buf: B,
}

impl<B: BufRead> EndfReader<B> {
    /// Creates an `EndfReader` from specified source.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::fs::File;
    /// use std::io::BufReader;
    /// use nkl::data::endf::EndfReader;
    ///
    /// let path = "path/to/file.endf";
    /// let file = File::open(path).expect("could not open endf file");
    /// let buf_reader = BufReader::new(file);
    /// let endf_reader = EndfReader::new(buf_reader);
    /// ```
    pub fn new(buf: B) -> Self {
        Self { buf }
    }

    /// Reads a line from the `EndfReader`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::fs::File;
    /// use std::io::BufReader;
    /// use nkl::data::endf::EndfReader;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut reader = EndfReader::new(BufReader::new(File::open("file.endf")?));
    /// let line = reader.read_line()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn read_line(&mut self) -> Result<Vec<u8>, EndfError> {
        let mut buf = Vec::with_capacity(ENDF_MAX_LINE_LENGTH);
        match self.buf.read_until(b'\n', &mut buf) {
            Ok(0) => Err(EndfError::EndOfFile),
            Err(error) => Err(error.into()),
            Ok(_) => Ok(buf),
        }
    }

    /// Reads a **CONT** record from the `EndfReader`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::fs::File;
    /// use std::io::BufReader;
    /// use nkl::data::endf::EndfReader;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut reader = EndfReader::new(BufReader::new(File::open("file.endf")?));
    /// let cont = reader.read_cont()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Errors if:
    /// - I/O error occurs
    /// - malformed/invalid data
    pub fn read_cont(&mut self) -> Result<Cont, EndfError> {
        let mut buf = Vec::with_capacity(ENDF_MAX_LINE_LENGTH);
        match self.buf.read_until(b'\n', &mut buf) {
            Ok(0) => Err(EndfError::EndOfFile),
            Err(error) => Err(error.into()),
            Ok(_) => {
                let c1 = parse_float(&buf, 1)?;
                let c2 = parse_float(&buf, 2)?;
                let l1 = parse_integer(&buf, 3)?;
                let l2 = parse_integer(&buf, 4)?;
                let n1 = parse_integer(&buf, 5)?;
                let n2 = parse_integer(&buf, 6)?;
                Ok(Cont(c1, c2, l1, l2, n1, n2))
            }
        }
    }

    /// Reads a **INTG** record from the `EndfReader`.
    ///
    /// `ndigit` denotes the number of digits for values.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::fs::File;
    /// use std::io::BufReader;
    /// use nkl::data::endf::EndfReader;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut reader = EndfReader::new(BufReader::new(File::open("file.endf")?));
    /// let intg = reader.read_intg(2)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Errors if:
    /// - I/O error occurs
    /// - malformed/invalid data
    pub fn read_intg(&mut self, ndigit: usize) -> Result<Intg, EndfError> {
        assert!(ndigit >= 2);
        assert!(ndigit <= 6);
        let mut buf = Vec::with_capacity(ENDF_MAX_LINE_LENGTH);
        match self.buf.read_until(b'\n', &mut buf) {
            Ok(0) => Err(EndfError::EndOfFile),
            Err(error) => Err(error.into()),
            Ok(_) => {
                let ii = match buf.get(0..5) {
                    Some(slice) => match parse_endf_integer(slice) {
                        Ok(integer) => integer,
                        Err(_) => return Err(EndfError::Data),
                    },
                    None => return Err(EndfError::Format),
                };
                let jj = match buf.get(5..10) {
                    Some(slice) => match parse_endf_integer(slice) {
                        Ok(integer) => integer,
                        Err(_) => return Err(EndfError::Data),
                    },
                    None => return Err(EndfError::Format),
                };
                let mut kij = Vec::new();
                let mut ptr = if ndigit <= 5 { 11 } else { 10 };
                loop {
                    if ptr + ndigit + 1 > 66 {
                        break;
                    }
                    let slice = &buf[ptr..ptr + ndigit + 1];
                    let value = match parse_endf_integer(slice) {
                        Ok(value) => value,
                        Err(_) => return Err(EndfError::Data),
                    };
                    kij.push(value);
                    ptr += ndigit + 1;
                }
                Ok(Intg(ii, jj, kij))
            }
        }
    }

    /// Reads a **LIST** record from the `EndfReader`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::fs::File;
    /// use std::io::BufReader;
    /// use nkl::data::endf::EndfReader;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut reader = EndfReader::new(BufReader::new(File::open("file.endf")?));
    /// let list = reader.read_list()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Errors if:
    /// - I/O error occurs
    /// - malformed/invalid data
    pub fn read_list(&mut self) -> Result<List, EndfError> {
        let mut buf = Vec::with_capacity(ENDF_MAX_LINE_LENGTH);
        match self.buf.read_until(b'\n', &mut buf) {
            Ok(0) => Err(EndfError::EndOfFile),
            Err(error) => Err(error.into()),
            Ok(_) => {
                let c1 = parse_float(&buf, 1)?;
                let c2 = parse_float(&buf, 2)?;
                let l1 = parse_integer(&buf, 3)?;
                let l2 = parse_integer(&buf, 4)?;
                let npl = parse_integer(&buf, 5)?;
                let n2 = parse_integer(&buf, 6)?;
                let npl: usize = match npl.try_into() {
                    Ok(npl) => npl,
                    Err(_) => return Err(EndfError::Data),
                };
                let mut b = Vec::with_capacity(npl);
                while b.len() < npl {
                    buf.clear();
                    match self.buf.read_until(b'\n', &mut buf) {
                        Ok(0) => return Err(EndfError::EndOfFile),
                        Err(error) => return Err(error.into()),
                        Ok(_) => {
                            for col in 0..6 {
                                if b.len() == npl {
                                    break;
                                }
                                let float = parse_float(&buf, col + 1)?;
                                b.push(float);
                            }
                        }
                    }
                }
                Ok(List(c1, c2, l1, l2, npl, n2, b))
            }
        }
    }

    /// Reads a **TAB1** record from the `EndfReader`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::fs::File;
    /// use std::io::BufReader;
    /// use nkl::data::endf::EndfReader;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut reader = EndfReader::new(BufReader::new(File::open("file.endf")?));
    /// let tab1 = reader.read_tab1()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Errors if:
    /// - I/O error occurs
    /// - malformed/invalid data
    pub fn read_tab1(&mut self) -> Result<Tab1, EndfError> {
        let mut buf = Vec::with_capacity(ENDF_MAX_LINE_LENGTH);
        match self.buf.read_until(b'\n', &mut buf) {
            Ok(0) => Err(EndfError::EndOfFile),
            Err(error) => Err(error.into()),
            Ok(_) => {
                let c1 = parse_float(&buf, 1)?;
                let c2 = parse_float(&buf, 2)?;
                let l1 = parse_integer(&buf, 3)?;
                let l2 = parse_integer(&buf, 4)?;
                let nr = parse_integer(&buf, 5)?;
                let np = parse_integer(&buf, 6)?;
                let nr: usize = match nr.try_into() {
                    Ok(nr) => nr,
                    Err(_) => return Err(EndfError::Data),
                };
                let np: usize = match np.try_into() {
                    Ok(np) => np,
                    Err(_) => return Err(EndfError::Data),
                };
                let mut int = Vec::with_capacity(nr);
                while int.len() < nr {
                    buf.clear();
                    match self.buf.read_until(b'\n', &mut buf) {
                        Ok(0) => return Err(EndfError::EndOfFile),
                        Err(error) => return Err(error.into()),
                        Ok(_) => {
                            for col in 0..3 {
                                if int.len() == nr {
                                    break;
                                }
                                let nbt = parse_integer(&buf, 2 * col + 1)?;
                                let nbt: u32 = match nbt.try_into() {
                                    Ok(nbt) => nbt,
                                    Err(_) => return Err(EndfError::Data),
                                };
                                let scheme = parse_integer(&buf, 2 * col + 2)?;
                                let scheme: usize = match scheme.try_into() {
                                    Ok(scheme) => scheme,
                                    Err(_) => return Err(EndfError::Data),
                                };
                                int.push((nbt, scheme));
                            }
                        }
                    }
                }
                let mut tab = Vec::with_capacity(np);
                while tab.len() < np {
                    buf.clear();
                    match self.buf.read_until(b'\n', &mut buf) {
                        Ok(0) => return Err(EndfError::EndOfFile),
                        Err(error) => return Err(error.into()),
                        Ok(_) => {
                            for col in 0..3 {
                                if tab.len() == np {
                                    break;
                                }
                                let x = parse_float(&buf, 2 * col + 1)?;
                                let y = parse_float(&buf, 2 * col + 2)?;
                                tab.push((x, y));
                            }
                        }
                    }
                }
                Ok(Tab1(c1, c2, l1, l2, nr, np, int, tab))
            }
        }
    }

    /// Reads a **TAB2** record from the `EndfReader`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::fs::File;
    /// use std::io::BufReader;
    /// use nkl::data::endf::EndfReader;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut reader = EndfReader::new(BufReader::new(File::open("file.endf")?));
    /// let tab2 = reader.read_tab2()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Errors if:
    /// - I/O error occurs
    /// - malformed/invalid data
    pub fn read_tab2(&mut self) -> Result<Tab2, EndfError> {
        let mut buf = Vec::with_capacity(ENDF_MAX_LINE_LENGTH);
        match self.buf.read_until(b'\n', &mut buf) {
            Ok(0) => Err(EndfError::EndOfFile),
            Err(error) => Err(error.into()),
            Ok(_) => {
                let c1 = parse_float(&buf, 1)?;
                let c2 = parse_float(&buf, 2)?;
                let l1 = parse_integer(&buf, 3)?;
                let l2 = parse_integer(&buf, 4)?;
                let nr = parse_integer(&buf, 5)?;
                let nz = parse_integer(&buf, 6)?;
                let nr: usize = match nr.try_into() {
                    Ok(nr) => nr,
                    Err(_) => return Err(EndfError::Data),
                };
                let nz: usize = match nz.try_into() {
                    Ok(nz) => nz,
                    Err(_) => return Err(EndfError::Data),
                };
                let mut int = Vec::with_capacity(nr);
                while int.len() < nr {
                    buf.clear();
                    match self.buf.read_until(b'\n', &mut buf) {
                        Ok(0) => return Err(EndfError::EndOfFile),
                        Err(error) => return Err(error.into()),
                        Ok(_) => {
                            for col in 0..3 {
                                if int.len() == nr {
                                    break;
                                }
                                let nbt = parse_integer(&buf, 2 * col + 1)?;
                                let nbt: u32 = match nbt.try_into() {
                                    Ok(nbt) => nbt,
                                    Err(_) => return Err(EndfError::Data),
                                };
                                let scheme = parse_integer(&buf, 2 * col + 2)?;
                                let scheme: usize = match scheme.try_into() {
                                    Ok(scheme) => scheme,
                                    Err(_) => return Err(EndfError::Data),
                                };
                                int.push((nbt, scheme));
                            }
                        }
                    }
                }
                Ok(Tab2(c1, c2, l1, l2, nr, nz, int))
            }
        }
    }

    /// Reads a **TEXT** record from the `EndfReader`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::fs::File;
    /// use std::io::BufReader;
    /// use nkl::data::endf::EndfReader;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut reader = EndfReader::new(BufReader::new(File::open("file.endf")?));
    /// let text = reader.read_text()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Errors if:
    /// - I/O error occurs
    /// - malformed/invalid data
    pub fn read_text(&mut self) -> Result<Text, EndfError> {
        let mut buf = Vec::with_capacity(ENDF_MAX_LINE_LENGTH);
        match self.buf.read_until(b'\n', &mut buf) {
            Ok(0) => Err(EndfError::EndOfFile),
            Err(error) => Err(error.into()),
            Ok(_) => {
                let hl = match String::from_utf8(buf[..66].to_vec()) {
                    Ok(string) => string,
                    Err(_) => return Err(EndfError::Data),
                };
                Ok(Text(hl))
            }
        }
    }
}
