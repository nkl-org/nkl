/// ENDF **CONT** record.
#[derive(Clone, Debug, PartialEq)]
pub struct Cont(pub f64, pub f64, pub i64, pub i64, pub i64, pub i64);

/// ENDF **INTG** record.
#[derive(Clone, Debug, PartialEq)]
pub struct Intg(pub i64, pub i64, pub Vec<i64>);

/// ENDF **LIST** record.
#[derive(Clone, Debug, PartialEq)]
pub struct List(
    pub f64,
    pub f64,
    pub i64,
    pub i64,
    pub usize,
    pub i64,
    pub Vec<f64>,
);

/// ENDF **TAB1** record.
#[derive(Clone, Debug, PartialEq)]
pub struct Tab1(
    pub f64,
    pub f64,
    pub i64,
    pub i64,
    pub usize,
    pub usize,
    pub Vec<(u32, usize)>,
    pub Vec<(f64, f64)>,
);

/// ENDF **TAB2** record.
#[derive(Clone, Debug, PartialEq)]
pub struct Tab2(
    pub f64,
    pub f64,
    pub i64,
    pub i64,
    pub usize,
    pub usize,
    pub Vec<(u32, usize)>,
);

/// ENDF **TEXT** record.
#[derive(Clone, Debug, PartialEq)]
pub struct Text(pub String);
