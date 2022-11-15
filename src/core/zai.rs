use crate::core::Element;
/// Nuclide identifier `ZAI`.
///
/// - `Z`: *atomic number* / proton number / nuclear charge number
/// - `A`: *mass number* / nucleon number
/// - `I`: *isomeric state number* / nuclear energy state / metastable state level
///
/// # Examples
///
/// ```
/// use nkl::core::{Element, Zai};
///
/// // From atomic/mass/isomeric-state numbers
/// let h1 = Zai::new(1, 1, 0);
/// // From standard name
/// let h1 = Zai::from_name("H1").unwrap();
/// // From id
/// let h1 = Zai::from_id(10010).unwrap();
///
/// assert_eq!(h1.atomic_number(), 1);
/// assert_eq!(h1.mass_number(), 1);
/// assert_eq!(h1.isomeric_state_number(), 0);
/// assert_eq!(h1.element(), Element::Hydrogen)
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Zai {
    atomic_number: u32,
    mass_number: u32,
    isomeric_state_number: u32,
}

impl Zai {
    /// Creates a new nuclide identifier (ZAI) from specified numbers.
    ///
    /// # Parameters
    ///
    /// - `atomic_number`: atomic number `Z`
    /// - `mass_number`: mass number `A`
    /// - `isomeric_state_number`: isomeric state number `I`
    ///
    /// # Examples
    ///
    /// ```
    /// use nkl::core::Zai;
    ///
    /// // H1 -> Z = 1, A = 1, I = 0
    /// let protium = Zai::new(1, 1, 0);
    /// // H2 -> Z = 1, A = 2, I = 0
    /// let deuterium = Zai::new(1, 2, 0);
    /// // H3 -> Z = 1, A = 3, I = 0
    /// let tritium = Zai::new(1, 3, 0);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if
    /// - `atomic_number` ∉ `[1, 118]`
    /// - number of nucleons is less than number of protons (`mass_number < atomic_number`)
    /// - `mass_number >= 1000`
    /// - `isomeric_state_number >= 10`
    pub fn new(atomic_number: u32, mass_number: u32, isomeric_state_number: u32) -> Self {
        assert!(atomic_number > 0);
        assert!(atomic_number <= Element::MAX_ATOMIC_NUMBER);
        assert!(mass_number >= atomic_number);
        assert!(mass_number < 1000);
        assert!(isomeric_state_number < 10);
        Self {
            atomic_number,
            mass_number,
            isomeric_state_number,
        }
    }

    /// Creates a new nuclide identifier from nuclide's name.
    ///
    /// # Format
    ///
    /// - Ground state nuclide: `XxAAA`
    /// - Metastable nuclide: `XxAAAmI`
    ///
    /// with:
    /// - `Xx`: one or two letter element's symbol (see [`Element`])
    /// - `AAA`: one to three (inclusive) digit(s) mass number
    /// - `I`: one digit isomeric state number
    ///
    /// # Returns
    ///
    /// - `Some(zai)` if `name` is a conformant nuclide's name
    /// - `None` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use nkl::core::Zai;
    ///
    /// // H1 -> Z = 1, A = 1, I = 0
    /// assert_eq!(Zai::from_name("H1"), Some(Zai::new(1, 1, 0)));
    /// // U235 -> Z = 92, A = 235, I = 0
    /// assert_eq!(Zai::from_name("U235"), Some(Zai::new(92, 235, 0)));
    /// // Pu239 -> Z = 94, A = 239, I = 0
    /// assert_eq!(Zai::from_name("Pu239"), Some(Zai::new(94, 239, 0)));
    /// // Am242 -> Z = 95, A = 242, I = 0
    /// assert_eq!(Zai::from_name("Am242"), Some(Zai::new(95, 242, 0)));
    /// // Am242m1 -> Z = 95, A = 242, I = 1
    /// assert_eq!(Zai::from_name("Am242m1"), Some(Zai::new(95, 242, 1)));
    /// // Am242m1 -> Z = 95, A = 242, I = 2
    /// assert_eq!(Zai::from_name("Am242m2"), Some(Zai::new(95, 242, 2)));
    /// ```
    pub fn from_name(name: &str) -> Option<Self> {
        // Check for ASCII.
        if !name.is_ascii() {
            return None;
        }
        // Initialize variables.
        let mut ptr = 0;
        let mut bytes = name.bytes().peekable();
        // Parse symbol.
        match bytes.next() {
            Some(byte) if (b'A'..=b'Z').contains(&byte) => {
                ptr += 1;
            }
            _ => return None,
        }
        match bytes.peek() {
            Some(byte) if (b'a'..=b'z').contains(byte) => {
                ptr += 1;
                bytes.next();
            }
            _ => (),
        }
        // Convert symbol to atomic number.
        let element = match Element::from_symbol(&name[..ptr]) {
            Some(element) => element,
            None => return None,
        };
        // Check atomic number.
        let atomic_number = element.atomic_number();
        if atomic_number == 0 || atomic_number > Element::MAX_ATOMIC_NUMBER {
            return None;
        }
        // Parse mass number.
        let start = ptr;
        match bytes.next() {
            Some(byte) if (b'1'..=b'9').contains(&byte) => {
                ptr += 1;
            }
            _ => return None,
        }
        for _ in 0..2 {
            match bytes.peek() {
                Some(byte) if (b'0'..=b'9').contains(byte) => {
                    ptr += 1;
                    bytes.next();
                }
                _ => break,
            }
        }
        let mass_number = match name[start..ptr].parse() {
            Ok(mass_number) => mass_number,
            Err(_) => return None,
        };
        // Check mass number.
        if mass_number < atomic_number {
            return None;
        }
        // Parse isomeric state number.
        let isomeric_state_number = match bytes.next() {
            None => 0,
            Some(b'm') => match bytes.next() {
                Some(byte) if (b'1'..=b'9').contains(&byte) => (byte - b'0') as u32,
                _ => return None,
            },
            _ => return None,
        };
        Some(Self {
            atomic_number,
            mass_number,
            isomeric_state_number,
        })
    }

    /// Creates a new nuclide identifier from nuclide's id.
    ///
    /// # Format
    ///
    /// ```text
    /// ID = Z × 10000 + A × 10 + I
    /// ```
    ///
    /// with:
    /// - `Z`: atomic number
    /// - `A`: mass number
    /// - `I`: isomeric state number
    ///
    /// # Returns
    ///
    /// - `Some(zai)` if `id` is a conformant nuclide's id
    /// - `None` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use nkl::core::Zai;
    ///
    /// // H1 -> Z = 1, A = 1, I = 0
    /// assert_eq!(Zai::from_id(10010), Some(Zai::new(1, 1, 0)));
    /// // U235 -> Z = 92, A = 235, I = 0
    /// assert_eq!(Zai::from_id(922350), Some(Zai::new(92, 235, 0)));
    /// // Pu239 -> Z = 94, A = 239, I = 0
    /// assert_eq!(Zai::from_id(942390), Some(Zai::new(94, 239, 0)));
    /// // Am242 -> Z = 95, A = 242, I = 0
    /// assert_eq!(Zai::from_id(952420), Some(Zai::new(95, 242, 0)));
    /// // Am242m1 -> Z = 95, A = 242, I = 1
    /// assert_eq!(Zai::from_id(952421), Some(Zai::new(95, 242, 1)));
    /// // Am242m2 -> Z = 95, A = 242, I = 2
    /// assert_eq!(Zai::from_id(952422), Some(Zai::new(95, 242, 2)));
    /// ```
    pub fn from_id(id: u32) -> Option<Self> {
        let atomic_number = id / 10000;
        if atomic_number == 0 || atomic_number > Element::MAX_ATOMIC_NUMBER {
            return None;
        }
        let mass_number = id % 10000 / 10;
        if mass_number >= 1000 || mass_number < atomic_number {
            return None;
        }
        let isomeric_state_number = id % 10;
        Some(Self {
            atomic_number,
            mass_number,
            isomeric_state_number,
        })
    }

    /// Returns atomic number `Z`.
    ///
    /// # Examples
    ///
    /// ```
    /// use nkl::core::Zai;
    ///
    /// let zai = Zai::new(1, 2, 0);
    /// assert_eq!(zai.atomic_number(), 1);
    /// ```
    pub fn atomic_number(&self) -> u32 {
        self.atomic_number
    }

    /// Returns mass number `A`.
    ///
    /// # Examples
    ///
    /// ```
    /// use nkl::core::Zai;
    ///
    /// let zai = Zai::new(1, 2, 0);
    /// assert_eq!(zai.mass_number(), 2);
    /// ```
    pub fn mass_number(&self) -> u32 {
        self.mass_number
    }

    /// Returns isomeric state number `I`.
    ///
    /// # Examples
    ///
    /// ```
    /// use nkl::core::Zai;
    ///
    /// let zai = Zai::new(1, 2, 0);
    /// assert_eq!(zai.isomeric_state_number(), 0);
    /// ```
    pub fn isomeric_state_number(&self) -> u32 {
        self.isomeric_state_number
    }

    /// Returns nuclide `ID`.
    ///
    /// # Format
    ///
    /// Nuclide ID is given by:
    ///
    /// ```text
    /// ID = Z × 10000 + A × 10 + I
    /// ```
    ///
    /// with:
    /// - `Z`: atomic number
    /// - `A`: mass number
    /// - `I`: isomeric state number
    ///
    /// # Examples
    ///
    /// ```
    /// use nkl::core::Zai;
    ///
    /// let h1 = Zai::new(1, 1, 0);
    /// assert_eq!(h1.id(), 10010);
    ///
    /// let u235 = Zai::new(92, 235, 0);
    /// assert_eq!(u235.id(), 922350);
    ///
    /// let am242m1 = Zai::new(95, 242, 1);
    /// assert_eq!(am242m1.id(), 952421);
    ///
    /// let am242m2 = Zai::new(95, 242, 2);
    /// assert_eq!(am242m2.id(), 952422);
    pub fn id(&self) -> u32 {
        self.atomic_number * 10000 + self.mass_number * 10 + self.isomeric_state_number
    }

    /// Returns number of protons `Z` (identical to *atomic number*).
    ///
    /// # Examples
    ///
    /// ```
    /// use nkl::core::Zai;
    ///
    /// let tritium = Zai::new(1, 3, 0);
    /// assert_eq!(tritium.protons(), 1);
    /// ```
    ///
    /// # See also
    ///
    /// [`atomic_number`](Self::atomic_number)
    pub fn protons(&self) -> u32 {
        self.atomic_number()
    }

    /// Returns number of neutrons `N = A - Z`.
    ///
    /// # Examples
    ///
    /// ```
    /// use nkl::core::Zai;
    ///
    /// let tritium = Zai::new(1, 3, 0);
    /// assert_eq!(tritium.neutrons(), 2);
    /// ```
    pub fn neutrons(&self) -> u32 {
        assert!(self.mass_number >= self.atomic_number);
        self.mass_number() - self.atomic_number()
    }

    /// Returns number of nucleons `A` (identical to *mass number*).
    ///
    /// # Examples
    ///
    /// ```
    /// use nkl::core::Zai;
    ///
    /// let tritium = Zai::new(1, 3, 0);
    /// assert_eq!(tritium.nucleons(), 3);
    /// ```
    ///
    /// # See also
    ///
    /// [`mass_number`](Self::mass_number)
    pub fn nucleons(&self) -> u32 {
        self.mass_number()
    }

    /// Returns nuclide identifier's chemical element.
    ///
    /// # Examples
    ///
    /// ```
    /// use nkl::core::{Element, Zai};
    ///
    /// let protium = Zai::new(1, 1, 0);
    /// assert_eq!(protium.element(), Element::Hydrogen);
    /// ```
    ///
    /// # See Also
    ///
    /// - [`Element`](crate::core::Element)
    pub fn element(&self) -> Element {
        assert!(self.atomic_number > 0);
        assert!(self.atomic_number <= Element::MAX_ATOMIC_NUMBER);
        // soundness: self.atomic_number is in periodic table range [1, MAX_ATOMIC_NUMBER]
        Element::from_atomic_number(self.atomic_number).unwrap()
    }

    /// Converts `ZAI` **to** `(Z, A, I)` tuple.
    ///
    /// # Examples
    ///
    /// ```
    /// use nkl::core::Zai;
    ///
    /// let zai = Zai::new(1, 2, 0);
    /// assert_eq!(zai.as_tuple(), (1, 2, 0));
    /// ```
    pub fn as_tuple(&self) -> (u32, u32, u32) {
        (
            self.atomic_number,
            self.mass_number,
            self.isomeric_state_number,
        )
    }

    /// Converts `ZAI` **into** `(Z, A, I)` tuple.
    ///
    /// # Examples
    ///
    /// ```
    /// use nkl::core::Zai;
    ///
    /// let zai = Zai::new(1, 2, 0);
    /// assert_eq!(zai.into_tuple(), (1, 2, 0));
    /// ```
    pub fn into_tuple(self) -> (u32, u32, u32) {
        (
            self.atomic_number,
            self.mass_number,
            self.isomeric_state_number,
        )
    }

    /// Returns `true` if the nuclide identifier isomeric state `I` is `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use nkl::core::Zai;
    ///
    /// let tc99 = Zai::new(43, 99, 0);
    /// assert!(tc99.is_ground_state());
    /// ```
    pub fn is_ground_state(&self) -> bool {
        self.isomeric_state_number == 0
    }

    /// Returns `true` if the nuclide identifier isomeric state `I` is **not** `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use nkl::core::Zai;
    ///
    /// let tc99m1 = Zai::new(43, 99, 1);
    /// assert!(tc99m1.is_metastable_state());
    /// ```
    pub fn is_metastable_state(&self) -> bool {
        self.isomeric_state_number != 0
    }

    /// Returns nuclide's name identified by this `ZAI` identifier.
    ///
    /// # Examples
    ///
    /// ```
    /// use nkl::core::Zai;
    ///
    /// let h1 = Zai::new(1, 1, 0);
    /// assert_eq!(h1.name(), "H1");
    ///
    /// let tc99m1 = Zai::new(43, 99, 1);
    /// assert_eq!(tc99m1.name(), "Tc99m1");
    /// ```
    pub fn name(&self) -> String {
        let element = self.element();
        let symbol = element.symbol();
        let mass = self.mass_number;
        if self.is_ground_state() {
            format!("{}{}", symbol, mass)
        } else {
            let isomer = self.isomeric_state_number;
            format!("{}{}m{}", symbol, mass, isomer)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn new_invalid_atomic_number_min() {
        Zai::new(0, 1, 0);
    }

    #[test]
    #[should_panic]
    fn new_invalid_atomic_number_max() {
        Zai::new(119, 119, 0);
    }

    #[test]
    #[should_panic]
    fn new_invalid_mass_number() {
        Zai::new(1, 0, 0);
    }

    #[test]
    #[should_panic]
    fn new_inconsistent_atomic_mass_numbers() {
        Zai::new(2, 1, 0);
    }

    #[test]
    fn from_name_invalid() {
        // invalid symbol
        assert!(Zai::from_name("X1").is_none());
        assert!(Zai::from_name("Xx1").is_none());
        assert!(Zai::from_name("Abc123").is_none());

        // invalid mass number
        assert!(Zai::from_name("H0").is_none());
        assert!(Zai::from_name("He0").is_none());
        assert!(Zai::from_name("He04").is_none());
        assert!(Zai::from_name("He004").is_none());
        assert!(Zai::from_name("He1234").is_none());

        // incoherent atomic/mass numbers
        assert!(Zai::from_name("He1").is_none());

        // invalid metastable separator
        assert!(Zai::from_name("H1g").is_none());
        assert!(Zai::from_name("H1n1").is_none());

        // invalid isomeric state number
        assert!(Zai::from_name("H1mx").is_none());
        assert!(Zai::from_name("H1m0").is_none());
    }

    #[test]
    fn from_id_invalid() {
        // invalid atomic number
        assert!(Zai::from_id(1234).is_none()); // Z = 0
        assert!(Zai::from_id(12341231).is_none()); // Z > 118
        assert!(Zai::from_id(11941231).is_none()); // Z > 118

        // invalid mass number
        assert!(Zai::from_id(10000).is_none()); // A = 0
        assert!(Zai::from_id(12312341).is_none()); // A >= 1000
        assert!(Zai::from_id(12310001).is_none()); // A >= 1000
    }

    #[test]
    fn name() {
        assert_eq!(Zai::new(1, 1, 0).name(), "H1");
        assert_eq!(Zai::new(1, 2, 0).name(), "H2");
        assert_eq!(Zai::new(1, 3, 0).name(), "H3");
        assert_eq!(Zai::new(27, 58, 1).name(), "Co58m1");
        assert_eq!(Zai::new(72, 178, 2).name(), "Hf178m2");
    }
}
