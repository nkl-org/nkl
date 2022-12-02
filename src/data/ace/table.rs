/// ACE Table.
#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub(crate) id: String,
    pub(crate) atomic_weight_ratio: f64,
    pub(crate) temperature: f64,
    pub(crate) izaw: Vec<(u32, f64)>,
    pub(crate) nxs: Vec<usize>,
    pub(crate) jxs: Vec<usize>,
    pub(crate) xss: Vec<f64>,
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
