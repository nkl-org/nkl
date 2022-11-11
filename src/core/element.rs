/// Periodic table chemical element.
///
/// # Examples
///
/// ```
/// # use nkl::core::Element;
/// let hydrogen = Element::Hydrogen;
/// let helium = Element::from_name("Helium");
/// let lithium = Element::from_symbol("Li");
/// let beryllium = Element::from_atomic_number(4);
///
/// let name = hydrogen.name();
/// let symbol = hydrogen.symbol();
/// let atomic_number = hydrogen.atomic_number();
/// ```
///
/// # Notes
///
/// - Chemical element from *Hydrogen* (Z = 1) to *Oganesson* (Z = 118) are included.
/// - `Element` enum is marked as non exhaustive for future-proofing.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum Element {
    /// Hydrogen, symbol='H', atomic number Z=1
    Hydrogen,
    /// Helium, symbol='He', atomic number Z=2
    Helium,
    /// Lithium, symbol='Li', atomic number Z=3
    Lithium,
    /// Beryllium, symbol='Be', atomic number Z=4
    Beryllium,
    /// Boron, symbol='B', atomic number Z=5
    Boron,
    /// Carbon, symbol='C', atomic number Z=6
    Carbon,
    /// Nitrogen, symbol='N', atomic number Z=7
    Nitrogen,
    /// Oxygen, symbol='O', atomic number Z=8
    Oxygen,
    /// Fluorine, symbol='F', atomic number Z=9
    Fluorine,
    /// Neon, symbol='Ne', atomic number Z=10
    Neon,
    /// Sodium, symbol='Na', atomic number Z=11
    Sodium,
    /// Magnesium, symbol='Mg', atomic number Z=12
    Magnesium,
    /// Aluminium, symbol='Al', atomic number Z=13
    Aluminium,
    /// Silicon, symbol='Si', atomic number Z=14
    Silicon,
    /// Phosphorus, symbol='P', atomic number Z=15
    Phosphorus,
    /// Sulfur, symbol='S', atomic number Z=16
    Sulfur,
    /// Chlorine, symbol='Cl', atomic number Z=17
    Chlorine,
    /// Argon, symbol='Ar', atomic number Z=18
    Argon,
    /// Potassium, symbol='K', atomic number Z=19
    Potassium,
    /// Calcium, symbol='Ca', atomic number Z=20
    Calcium,
    /// Scandium, symbol='Sc', atomic number Z=21
    Scandium,
    /// Titanium, symbol='Ti', atomic number Z=22
    Titanium,
    /// Vanadium, symbol='V', atomic number Z=23
    Vanadium,
    /// Chromium, symbol='Cr', atomic number Z=24
    Chromium,
    /// Manganese, symbol='Mn', atomic number Z=25
    Manganese,
    /// Iron, symbol='Fe', atomic number Z=26
    Iron,
    /// Cobalt, symbol='Co', atomic number Z=27
    Cobalt,
    /// Nickel, symbol='Ni', atomic number Z=28
    Nickel,
    /// Copper, symbol='Cu', atomic number Z=29
    Copper,
    /// Zinc, symbol='Zn', atomic number Z=30
    Zinc,
    /// Gallium, symbol='Ga', atomic number Z=31
    Gallium,
    /// Germanium, symbol='Ge', atomic number Z=32
    Germanium,
    /// Arsenic, symbol='As', atomic number Z=33
    Arsenic,
    /// Selenium, symbol='Se', atomic number Z=34
    Selenium,
    /// Bromine, symbol='Br', atomic number Z=35
    Bromine,
    /// Krypton, symbol='Kr', atomic number Z=36
    Krypton,
    /// Rubidium, symbol='Rb', atomic number Z=37
    Rubidium,
    /// Strontium, symbol='Sr', atomic number Z=38
    Strontium,
    /// Yttrium, symbol='Y', atomic number Z=39
    Yttrium,
    /// Zirconium, symbol='Zr', atomic number Z=40
    Zirconium,
    /// Niobium, symbol='Nb', atomic number Z=41
    Niobium,
    /// Molybdenum, symbol='Mo', atomic number Z=42
    Molybdenum,
    /// Technetium, symbol='Tc', atomic number Z=43
    Technetium,
    /// Ruthenium, symbol='Ru', atomic number Z=44
    Ruthenium,
    /// Rhodium, symbol='Rh', atomic number Z=45
    Rhodium,
    /// Palladium, symbol='Pd', atomic number Z=46
    Palladium,
    /// Silver, symbol='Ag', atomic number Z=47
    Silver,
    /// Cadmium, symbol='Cd', atomic number Z=48
    Cadmium,
    /// Indium, symbol='In', atomic number Z=49
    Indium,
    /// Tin, symbol='Sn', atomic number Z=50
    Tin,
    /// Antimony, symbol='Sb', atomic number Z=51
    Antimony,
    /// Tellurium, symbol='Te', atomic number Z=52
    Tellurium,
    /// Iodine, symbol='I', atomic number Z=53
    Iodine,
    /// Xenon, symbol='Xe', atomic number Z=54
    Xenon,
    /// Caesium, symbol='Cs', atomic number Z=55
    Caesium,
    /// Barium, symbol='Ba', atomic number Z=56
    Barium,
    /// Lanthanum, symbol='La', atomic number Z=57
    Lanthanum,
    /// Cerium, symbol='Ce', atomic number Z=58
    Cerium,
    /// Praseodymium, symbol='Pr', atomic number Z=59
    Praseodymium,
    /// Neodymium, symbol='Nd', atomic number Z=60
    Neodymium,
    /// Promethium, symbol='Pm', atomic number Z=61
    Promethium,
    /// Samarium, symbol='Sm', atomic number Z=62
    Samarium,
    /// Europium, symbol='Eu', atomic number Z=63
    Europium,
    /// Gadolinium, symbol='Gd', atomic number Z=64
    Gadolinium,
    /// Terbium, symbol='Tb', atomic number Z=65
    Terbium,
    /// Dysprosium, symbol='Dy', atomic number Z=66
    Dysprosium,
    /// Holmium, symbol='Ho', atomic number Z=67
    Holmium,
    /// Erbium, symbol='Er', atomic number Z=68
    Erbium,
    /// Thulium, symbol='Tm', atomic number Z=69
    Thulium,
    /// Ytterbium, symbol='Yb', atomic number Z=70
    Ytterbium,
    /// Lutetium, symbol='Lu', atomic number Z=71
    Lutetium,
    /// Hafnium, symbol='Hf', atomic number Z=72
    Hafnium,
    /// Tantalum, symbol='Ta', atomic number Z=73
    Tantalum,
    /// Tungsten, symbol='W', atomic number Z=74
    Tungsten,
    /// Rhenium, symbol='Re', atomic number Z=75
    Rhenium,
    /// Osmium, symbol='Os', atomic number Z=76
    Osmium,
    /// Iridium, symbol='Ir', atomic number Z=77
    Iridium,
    /// Platinum, symbol='Pt', atomic number Z=78
    Platinum,
    /// Gold, symbol='Au', atomic number Z=79
    Gold,
    /// Mercury, symbol='Hg', atomic number Z=80
    Mercury,
    /// Thallium, symbol='Tl', atomic number Z=81
    Thallium,
    /// Lead, symbol='Pb', atomic number Z=82
    Lead,
    /// Bismuth, symbol='Bi', atomic number Z=83
    Bismuth,
    /// Polonium, symbol='Po', atomic number Z=84
    Polonium,
    /// Astatine, symbol='At', atomic number Z=85
    Astatine,
    /// Radon, symbol='Rn', atomic number Z=86
    Radon,
    /// Francium, symbol='Fr', atomic number Z=87
    Francium,
    /// Radium, symbol='Ra', atomic number Z=88
    Radium,
    /// Actinium, symbol='Ac', atomic number Z=89
    Actinium,
    /// Thorium, symbol='Th', atomic number Z=90
    Thorium,
    /// Protactinium, symbol='Pa', atomic number Z=91
    Protactinium,
    /// Uranium, symbol='U', atomic number Z=92
    Uranium,
    /// Neptunium, symbol='Np', atomic number Z=93
    Neptunium,
    /// Plutonium, symbol='Pu', atomic number Z=94
    Plutonium,
    /// Americium, symbol='Am', atomic number Z=95
    Americium,
    /// Curium, symbol='Cm', atomic number Z=96
    Curium,
    /// Berkelium, symbol='Bk', atomic number Z=97
    Berkelium,
    /// Californium, symbol='Cf', atomic number Z=98
    Californium,
    /// Einsteinium, symbol='Es', atomic number Z=99
    Einsteinium,
    /// Fermium, symbol='Fm', atomic number Z=100
    Fermium,
    /// Mendelevium, symbol='Md', atomic number Z=101
    Mendelevium,
    /// Nobelium, symbol='No', atomic number Z=102
    Nobelium,
    /// Lawrencium, symbol='Lr', atomic number Z=103
    Lawrencium,
    /// Rutherfordium, symbol='Rf', atomic number Z=104
    Rutherfordium,
    /// Dubnium, symbol='Db', atomic number Z=105
    Dubnium,
    /// Seaborgium, symbol='Sg', atomic number Z=106
    Seaborgium,
    /// Bohrium, symbol='Bh', atomic number Z=107
    Bohrium,
    /// Hassium, symbol='Hs', atomic number Z=108
    Hassium,
    /// Meitnerium, symbol='Mt', atomic number Z=109
    Meitnerium,
    /// Darmstadtium, symbol='Ds', atomic number Z=110
    Darmstadtium,
    /// Roentgenium, symbol='Rg', atomic number Z=111
    Roentgenium,
    /// Copernicium, symbol='Cn', atomic number Z=112
    Copernicium,
    /// Nihonium, symbol='Nh', atomic number Z=113
    Nihonium,
    /// Flerovium, symbol='Fl', atomic number Z=114
    Flerovium,
    /// Moscovium, symbol='Mc', atomic number Z=115
    Moscovium,
    /// Livermorium, symbol='Lv', atomic number Z=116
    Livermorium,
    /// Tennessine, symbol='Ts', atomic number Z=117
    Tennessine,
    /// Oganesson, symbol='Og', atomic number Z=118
    Oganesson,
}

impl Element {
    const ELEMENTS: [Self; 118] = [
        Self::Hydrogen,
        Self::Helium,
        Self::Lithium,
        Self::Beryllium,
        Self::Boron,
        Self::Carbon,
        Self::Nitrogen,
        Self::Oxygen,
        Self::Fluorine,
        Self::Neon,
        Self::Sodium,
        Self::Magnesium,
        Self::Aluminium,
        Self::Silicon,
        Self::Phosphorus,
        Self::Sulfur,
        Self::Chlorine,
        Self::Argon,
        Self::Potassium,
        Self::Calcium,
        Self::Scandium,
        Self::Titanium,
        Self::Vanadium,
        Self::Chromium,
        Self::Manganese,
        Self::Iron,
        Self::Cobalt,
        Self::Nickel,
        Self::Copper,
        Self::Zinc,
        Self::Gallium,
        Self::Germanium,
        Self::Arsenic,
        Self::Selenium,
        Self::Bromine,
        Self::Krypton,
        Self::Rubidium,
        Self::Strontium,
        Self::Yttrium,
        Self::Zirconium,
        Self::Niobium,
        Self::Molybdenum,
        Self::Technetium,
        Self::Ruthenium,
        Self::Rhodium,
        Self::Palladium,
        Self::Silver,
        Self::Cadmium,
        Self::Indium,
        Self::Tin,
        Self::Antimony,
        Self::Tellurium,
        Self::Iodine,
        Self::Xenon,
        Self::Caesium,
        Self::Barium,
        Self::Lanthanum,
        Self::Cerium,
        Self::Praseodymium,
        Self::Neodymium,
        Self::Promethium,
        Self::Samarium,
        Self::Europium,
        Self::Gadolinium,
        Self::Terbium,
        Self::Dysprosium,
        Self::Holmium,
        Self::Erbium,
        Self::Thulium,
        Self::Ytterbium,
        Self::Lutetium,
        Self::Hafnium,
        Self::Tantalum,
        Self::Tungsten,
        Self::Rhenium,
        Self::Osmium,
        Self::Iridium,
        Self::Platinum,
        Self::Gold,
        Self::Mercury,
        Self::Thallium,
        Self::Lead,
        Self::Bismuth,
        Self::Polonium,
        Self::Astatine,
        Self::Radon,
        Self::Francium,
        Self::Radium,
        Self::Actinium,
        Self::Thorium,
        Self::Protactinium,
        Self::Uranium,
        Self::Neptunium,
        Self::Plutonium,
        Self::Americium,
        Self::Curium,
        Self::Berkelium,
        Self::Californium,
        Self::Einsteinium,
        Self::Fermium,
        Self::Mendelevium,
        Self::Nobelium,
        Self::Lawrencium,
        Self::Rutherfordium,
        Self::Dubnium,
        Self::Seaborgium,
        Self::Bohrium,
        Self::Hassium,
        Self::Meitnerium,
        Self::Darmstadtium,
        Self::Roentgenium,
        Self::Copernicium,
        Self::Nihonium,
        Self::Flerovium,
        Self::Moscovium,
        Self::Livermorium,
        Self::Tennessine,
        Self::Oganesson,
    ];
    /// Returns `Element` corresponding to specified `name` (case insensitive).
    ///
    /// # Returns
    ///
    /// - `Some(element)` if `name` is a standard element name (case insensitive)
    /// - `None` if `name` is **not** a standard element name
    ///
    /// # Examples
    ///
    /// ```
    /// # use nkl::core::Element;
    /// assert_eq!(Element::from_name("Hydrogen"), Some(Element::Hydrogen));
    /// ```
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_ascii_lowercase().as_str() {
            "hydrogen" => Some(Self::Hydrogen),
            "helium" => Some(Self::Helium),
            "lithium" => Some(Self::Lithium),
            "beryllium" => Some(Self::Beryllium),
            "boron" => Some(Self::Boron),
            "carbon" => Some(Self::Carbon),
            "nitrogen" => Some(Self::Nitrogen),
            "oxygen" => Some(Self::Oxygen),
            "fluorine" => Some(Self::Fluorine),
            "neon" => Some(Self::Neon),
            "sodium" => Some(Self::Sodium),
            "magnesium" => Some(Self::Magnesium),
            "aluminium" => Some(Self::Aluminium),
            "silicon" => Some(Self::Silicon),
            "phosphorus" => Some(Self::Phosphorus),
            "sulfur" => Some(Self::Sulfur),
            "chlorine" => Some(Self::Chlorine),
            "argon" => Some(Self::Argon),
            "potassium" => Some(Self::Potassium),
            "calcium" => Some(Self::Calcium),
            "scandium" => Some(Self::Scandium),
            "titanium" => Some(Self::Titanium),
            "vanadium" => Some(Self::Vanadium),
            "chromium" => Some(Self::Chromium),
            "manganese" => Some(Self::Manganese),
            "iron" => Some(Self::Iron),
            "cobalt" => Some(Self::Cobalt),
            "nickel" => Some(Self::Nickel),
            "copper" => Some(Self::Copper),
            "zinc" => Some(Self::Zinc),
            "gallium" => Some(Self::Gallium),
            "germanium" => Some(Self::Germanium),
            "arsenic" => Some(Self::Arsenic),
            "selenium" => Some(Self::Selenium),
            "bromine" => Some(Self::Bromine),
            "krypton" => Some(Self::Krypton),
            "rubidium" => Some(Self::Rubidium),
            "strontium" => Some(Self::Strontium),
            "yttrium" => Some(Self::Yttrium),
            "zirconium" => Some(Self::Zirconium),
            "niobium" => Some(Self::Niobium),
            "molybdenum" => Some(Self::Molybdenum),
            "technetium" => Some(Self::Technetium),
            "ruthenium" => Some(Self::Ruthenium),
            "rhodium" => Some(Self::Rhodium),
            "palladium" => Some(Self::Palladium),
            "silver" => Some(Self::Silver),
            "cadmium" => Some(Self::Cadmium),
            "indium" => Some(Self::Indium),
            "tin" => Some(Self::Tin),
            "antimony" => Some(Self::Antimony),
            "tellurium" => Some(Self::Tellurium),
            "iodine" => Some(Self::Iodine),
            "xenon" => Some(Self::Xenon),
            "caesium" => Some(Self::Caesium),
            "barium" => Some(Self::Barium),
            "lanthanum" => Some(Self::Lanthanum),
            "cerium" => Some(Self::Cerium),
            "praseodymium" => Some(Self::Praseodymium),
            "neodymium" => Some(Self::Neodymium),
            "promethium" => Some(Self::Promethium),
            "samarium" => Some(Self::Samarium),
            "europium" => Some(Self::Europium),
            "gadolinium" => Some(Self::Gadolinium),
            "terbium" => Some(Self::Terbium),
            "dysprosium" => Some(Self::Dysprosium),
            "holmium" => Some(Self::Holmium),
            "erbium" => Some(Self::Erbium),
            "thulium" => Some(Self::Thulium),
            "ytterbium" => Some(Self::Ytterbium),
            "lutetium" => Some(Self::Lutetium),
            "hafnium" => Some(Self::Hafnium),
            "tantalum" => Some(Self::Tantalum),
            "tungsten" => Some(Self::Tungsten),
            "rhenium" => Some(Self::Rhenium),
            "osmium" => Some(Self::Osmium),
            "iridium" => Some(Self::Iridium),
            "platinum" => Some(Self::Platinum),
            "gold" => Some(Self::Gold),
            "mercury" => Some(Self::Mercury),
            "thallium" => Some(Self::Thallium),
            "lead" => Some(Self::Lead),
            "bismuth" => Some(Self::Bismuth),
            "polonium" => Some(Self::Polonium),
            "astatine" => Some(Self::Astatine),
            "radon" => Some(Self::Radon),
            "francium" => Some(Self::Francium),
            "radium" => Some(Self::Radium),
            "actinium" => Some(Self::Actinium),
            "thorium" => Some(Self::Thorium),
            "protactinium" => Some(Self::Protactinium),
            "uranium" => Some(Self::Uranium),
            "neptunium" => Some(Self::Neptunium),
            "plutonium" => Some(Self::Plutonium),
            "americium" => Some(Self::Americium),
            "curium" => Some(Self::Curium),
            "berkelium" => Some(Self::Berkelium),
            "californium" => Some(Self::Californium),
            "einsteinium" => Some(Self::Einsteinium),
            "fermium" => Some(Self::Fermium),
            "mendelevium" => Some(Self::Mendelevium),
            "nobelium" => Some(Self::Nobelium),
            "lawrencium" => Some(Self::Lawrencium),
            "rutherfordium" => Some(Self::Rutherfordium),
            "dubnium" => Some(Self::Dubnium),
            "seaborgium" => Some(Self::Seaborgium),
            "bohrium" => Some(Self::Bohrium),
            "hassium" => Some(Self::Hassium),
            "meitnerium" => Some(Self::Meitnerium),
            "darmstadtium" => Some(Self::Darmstadtium),
            "roentgenium" => Some(Self::Roentgenium),
            "copernicium" => Some(Self::Copernicium),
            "nihonium" => Some(Self::Nihonium),
            "flerovium" => Some(Self::Flerovium),
            "moscovium" => Some(Self::Moscovium),
            "livermorium" => Some(Self::Livermorium),
            "tennessine" => Some(Self::Tennessine),
            "oganesson" => Some(Self::Oganesson),
            _ => None,
        }
    }

    /// Returns `Element` corresponding to specified symbol.
    ///
    /// # Returns
    ///
    /// - `Some(element)` if `symbol` is a standard element symbol (case insensitive)
    /// - `None` if `symbol` is **not** a standard element symbol
    ///
    /// # Examples
    ///
    /// ```
    /// # use nkl::core::Element;
    /// assert_eq!(Element::from_symbol("H"), Some(Element::Hydrogen));
    /// ```
    pub fn from_symbol(symbol: &str) -> Option<Self> {
        match symbol.to_ascii_lowercase().as_str() {
            "h" => Some(Self::Hydrogen),
            "he" => Some(Self::Helium),
            "li" => Some(Self::Lithium),
            "be" => Some(Self::Beryllium),
            "b" => Some(Self::Boron),
            "c" => Some(Self::Carbon),
            "n" => Some(Self::Nitrogen),
            "o" => Some(Self::Oxygen),
            "f" => Some(Self::Fluorine),
            "ne" => Some(Self::Neon),
            "na" => Some(Self::Sodium),
            "mg" => Some(Self::Magnesium),
            "al" => Some(Self::Aluminium),
            "si" => Some(Self::Silicon),
            "p" => Some(Self::Phosphorus),
            "s" => Some(Self::Sulfur),
            "cl" => Some(Self::Chlorine),
            "ar" => Some(Self::Argon),
            "k" => Some(Self::Potassium),
            "ca" => Some(Self::Calcium),
            "sc" => Some(Self::Scandium),
            "ti" => Some(Self::Titanium),
            "v" => Some(Self::Vanadium),
            "cr" => Some(Self::Chromium),
            "mn" => Some(Self::Manganese),
            "fe" => Some(Self::Iron),
            "co" => Some(Self::Cobalt),
            "ni" => Some(Self::Nickel),
            "cu" => Some(Self::Copper),
            "zn" => Some(Self::Zinc),
            "ga" => Some(Self::Gallium),
            "ge" => Some(Self::Germanium),
            "as" => Some(Self::Arsenic),
            "se" => Some(Self::Selenium),
            "br" => Some(Self::Bromine),
            "kr" => Some(Self::Krypton),
            "rb" => Some(Self::Rubidium),
            "sr" => Some(Self::Strontium),
            "y" => Some(Self::Yttrium),
            "zr" => Some(Self::Zirconium),
            "nb" => Some(Self::Niobium),
            "mo" => Some(Self::Molybdenum),
            "tc" => Some(Self::Technetium),
            "ru" => Some(Self::Ruthenium),
            "rh" => Some(Self::Rhodium),
            "pd" => Some(Self::Palladium),
            "ag" => Some(Self::Silver),
            "cd" => Some(Self::Cadmium),
            "in" => Some(Self::Indium),
            "sn" => Some(Self::Tin),
            "sb" => Some(Self::Antimony),
            "te" => Some(Self::Tellurium),
            "i" => Some(Self::Iodine),
            "xe" => Some(Self::Xenon),
            "cs" => Some(Self::Caesium),
            "ba" => Some(Self::Barium),
            "la" => Some(Self::Lanthanum),
            "ce" => Some(Self::Cerium),
            "pr" => Some(Self::Praseodymium),
            "nd" => Some(Self::Neodymium),
            "pm" => Some(Self::Promethium),
            "sm" => Some(Self::Samarium),
            "eu" => Some(Self::Europium),
            "gd" => Some(Self::Gadolinium),
            "tb" => Some(Self::Terbium),
            "dy" => Some(Self::Dysprosium),
            "ho" => Some(Self::Holmium),
            "er" => Some(Self::Erbium),
            "tm" => Some(Self::Thulium),
            "yb" => Some(Self::Ytterbium),
            "lu" => Some(Self::Lutetium),
            "hf" => Some(Self::Hafnium),
            "ta" => Some(Self::Tantalum),
            "w" => Some(Self::Tungsten),
            "re" => Some(Self::Rhenium),
            "os" => Some(Self::Osmium),
            "ir" => Some(Self::Iridium),
            "pt" => Some(Self::Platinum),
            "au" => Some(Self::Gold),
            "hg" => Some(Self::Mercury),
            "tl" => Some(Self::Thallium),
            "pb" => Some(Self::Lead),
            "bi" => Some(Self::Bismuth),
            "po" => Some(Self::Polonium),
            "at" => Some(Self::Astatine),
            "rn" => Some(Self::Radon),
            "fr" => Some(Self::Francium),
            "ra" => Some(Self::Radium),
            "ac" => Some(Self::Actinium),
            "th" => Some(Self::Thorium),
            "pa" => Some(Self::Protactinium),
            "u" => Some(Self::Uranium),
            "np" => Some(Self::Neptunium),
            "pu" => Some(Self::Plutonium),
            "am" => Some(Self::Americium),
            "cm" => Some(Self::Curium),
            "bk" => Some(Self::Berkelium),
            "cf" => Some(Self::Californium),
            "es" => Some(Self::Einsteinium),
            "fm" => Some(Self::Fermium),
            "md" => Some(Self::Mendelevium),
            "no" => Some(Self::Nobelium),
            "lr" => Some(Self::Lawrencium),
            "rf" => Some(Self::Rutherfordium),
            "db" => Some(Self::Dubnium),
            "sg" => Some(Self::Seaborgium),
            "bh" => Some(Self::Bohrium),
            "hs" => Some(Self::Hassium),
            "mt" => Some(Self::Meitnerium),
            "ds" => Some(Self::Darmstadtium),
            "rg" => Some(Self::Roentgenium),
            "cn" => Some(Self::Copernicium),
            "nh" => Some(Self::Nihonium),
            "fl" => Some(Self::Flerovium),
            "mc" => Some(Self::Moscovium),
            "lv" => Some(Self::Livermorium),
            "ts" => Some(Self::Tennessine),
            "og" => Some(Self::Oganesson),
            _ => None,
        }
    }

    /// Returns `Element` corresponding to specified atomic number.
    ///
    /// # Returns
    ///
    /// - `Some(element)` if `atomic_number` ∈ `[1, 118]`
    /// - `None` if `atomic_number` ∉ `[0, 118]`
    ///
    /// # Examples
    ///
    /// ```
    /// # use nkl::core::Element;
    /// assert_eq!(Element::from_atomic_number(1), Some(Element::Hydrogen));
    /// ```
    pub fn from_atomic_number(atomic_number: u32) -> Option<Self> {
        match atomic_number {
            1 => Some(Self::Hydrogen),
            2 => Some(Self::Helium),
            3 => Some(Self::Lithium),
            4 => Some(Self::Beryllium),
            5 => Some(Self::Boron),
            6 => Some(Self::Carbon),
            7 => Some(Self::Nitrogen),
            8 => Some(Self::Oxygen),
            9 => Some(Self::Fluorine),
            10 => Some(Self::Neon),
            11 => Some(Self::Sodium),
            12 => Some(Self::Magnesium),
            13 => Some(Self::Aluminium),
            14 => Some(Self::Silicon),
            15 => Some(Self::Phosphorus),
            16 => Some(Self::Sulfur),
            17 => Some(Self::Chlorine),
            18 => Some(Self::Argon),
            19 => Some(Self::Potassium),
            20 => Some(Self::Calcium),
            21 => Some(Self::Scandium),
            22 => Some(Self::Titanium),
            23 => Some(Self::Vanadium),
            24 => Some(Self::Chromium),
            25 => Some(Self::Manganese),
            26 => Some(Self::Iron),
            27 => Some(Self::Cobalt),
            28 => Some(Self::Nickel),
            29 => Some(Self::Copper),
            30 => Some(Self::Zinc),
            31 => Some(Self::Gallium),
            32 => Some(Self::Germanium),
            33 => Some(Self::Arsenic),
            34 => Some(Self::Selenium),
            35 => Some(Self::Bromine),
            36 => Some(Self::Krypton),
            37 => Some(Self::Rubidium),
            38 => Some(Self::Strontium),
            39 => Some(Self::Yttrium),
            40 => Some(Self::Zirconium),
            41 => Some(Self::Niobium),
            42 => Some(Self::Molybdenum),
            43 => Some(Self::Technetium),
            44 => Some(Self::Ruthenium),
            45 => Some(Self::Rhodium),
            46 => Some(Self::Palladium),
            47 => Some(Self::Silver),
            48 => Some(Self::Cadmium),
            49 => Some(Self::Indium),
            50 => Some(Self::Tin),
            51 => Some(Self::Antimony),
            52 => Some(Self::Tellurium),
            53 => Some(Self::Iodine),
            54 => Some(Self::Xenon),
            55 => Some(Self::Caesium),
            56 => Some(Self::Barium),
            57 => Some(Self::Lanthanum),
            58 => Some(Self::Cerium),
            59 => Some(Self::Praseodymium),
            60 => Some(Self::Neodymium),
            61 => Some(Self::Promethium),
            62 => Some(Self::Samarium),
            63 => Some(Self::Europium),
            64 => Some(Self::Gadolinium),
            65 => Some(Self::Terbium),
            66 => Some(Self::Dysprosium),
            67 => Some(Self::Holmium),
            68 => Some(Self::Erbium),
            69 => Some(Self::Thulium),
            70 => Some(Self::Ytterbium),
            71 => Some(Self::Lutetium),
            72 => Some(Self::Hafnium),
            73 => Some(Self::Tantalum),
            74 => Some(Self::Tungsten),
            75 => Some(Self::Rhenium),
            76 => Some(Self::Osmium),
            77 => Some(Self::Iridium),
            78 => Some(Self::Platinum),
            79 => Some(Self::Gold),
            80 => Some(Self::Mercury),
            81 => Some(Self::Thallium),
            82 => Some(Self::Lead),
            83 => Some(Self::Bismuth),
            84 => Some(Self::Polonium),
            85 => Some(Self::Astatine),
            86 => Some(Self::Radon),
            87 => Some(Self::Francium),
            88 => Some(Self::Radium),
            89 => Some(Self::Actinium),
            90 => Some(Self::Thorium),
            91 => Some(Self::Protactinium),
            92 => Some(Self::Uranium),
            93 => Some(Self::Neptunium),
            94 => Some(Self::Plutonium),
            95 => Some(Self::Americium),
            96 => Some(Self::Curium),
            97 => Some(Self::Berkelium),
            98 => Some(Self::Californium),
            99 => Some(Self::Einsteinium),
            100 => Some(Self::Fermium),
            101 => Some(Self::Mendelevium),
            102 => Some(Self::Nobelium),
            103 => Some(Self::Lawrencium),
            104 => Some(Self::Rutherfordium),
            105 => Some(Self::Dubnium),
            106 => Some(Self::Seaborgium),
            107 => Some(Self::Bohrium),
            108 => Some(Self::Hassium),
            109 => Some(Self::Meitnerium),
            110 => Some(Self::Darmstadtium),
            111 => Some(Self::Roentgenium),
            112 => Some(Self::Copernicium),
            113 => Some(Self::Nihonium),
            114 => Some(Self::Flerovium),
            115 => Some(Self::Moscovium),
            116 => Some(Self::Livermorium),
            117 => Some(Self::Tennessine),
            118 => Some(Self::Oganesson),
            _ => None,
        }
    }

    /// Returns `Element`'s name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use nkl::core::Element;
    /// assert_eq!(Element::Hydrogen.name(), "Hydrogen");
    /// ```
    pub fn name(&self) -> &str {
        match self {
            Self::Hydrogen => "Hydrogen",
            Self::Helium => "Helium",
            Self::Lithium => "Lithium",
            Self::Beryllium => "Beryllium",
            Self::Boron => "Boron",
            Self::Carbon => "Carbon",
            Self::Nitrogen => "Nitrogen",
            Self::Oxygen => "Oxygen",
            Self::Fluorine => "Fluorine",
            Self::Neon => "Neon",
            Self::Sodium => "Sodium",
            Self::Magnesium => "Magnesium",
            Self::Aluminium => "Aluminium",
            Self::Silicon => "Silicon",
            Self::Phosphorus => "Phosphorus",
            Self::Sulfur => "Sulfur",
            Self::Chlorine => "Chlorine",
            Self::Argon => "Argon",
            Self::Potassium => "Potassium",
            Self::Calcium => "Calcium",
            Self::Scandium => "Scandium",
            Self::Titanium => "Titanium",
            Self::Vanadium => "Vanadium",
            Self::Chromium => "Chromium",
            Self::Manganese => "Manganese",
            Self::Iron => "Iron",
            Self::Cobalt => "Cobalt",
            Self::Nickel => "Nickel",
            Self::Copper => "Copper",
            Self::Zinc => "Zinc",
            Self::Gallium => "Gallium",
            Self::Germanium => "Germanium",
            Self::Arsenic => "Arsenic",
            Self::Selenium => "Selenium",
            Self::Bromine => "Bromine",
            Self::Krypton => "Krypton",
            Self::Rubidium => "Rubidium",
            Self::Strontium => "Strontium",
            Self::Yttrium => "Yttrium",
            Self::Zirconium => "Zirconium",
            Self::Niobium => "Niobium",
            Self::Molybdenum => "Molybdenum",
            Self::Technetium => "Technetium",
            Self::Ruthenium => "Ruthenium",
            Self::Rhodium => "Rhodium",
            Self::Palladium => "Palladium",
            Self::Silver => "Silver",
            Self::Cadmium => "Cadmium",
            Self::Indium => "Indium",
            Self::Tin => "Tin",
            Self::Antimony => "Antimony",
            Self::Tellurium => "Tellurium",
            Self::Iodine => "Iodine",
            Self::Xenon => "Xenon",
            Self::Caesium => "Caesium",
            Self::Barium => "Barium",
            Self::Lanthanum => "Lanthanum",
            Self::Cerium => "Cerium",
            Self::Praseodymium => "Praseodymium",
            Self::Neodymium => "Neodymium",
            Self::Promethium => "Promethium",
            Self::Samarium => "Samarium",
            Self::Europium => "Europium",
            Self::Gadolinium => "Gadolinium",
            Self::Terbium => "Terbium",
            Self::Dysprosium => "Dysprosium",
            Self::Holmium => "Holmium",
            Self::Erbium => "Erbium",
            Self::Thulium => "Thulium",
            Self::Ytterbium => "Ytterbium",
            Self::Lutetium => "Lutetium",
            Self::Hafnium => "Hafnium",
            Self::Tantalum => "Tantalum",
            Self::Tungsten => "Tungsten",
            Self::Rhenium => "Rhenium",
            Self::Osmium => "Osmium",
            Self::Iridium => "Iridium",
            Self::Platinum => "Platinum",
            Self::Gold => "Gold",
            Self::Mercury => "Mercury",
            Self::Thallium => "Thallium",
            Self::Lead => "Lead",
            Self::Bismuth => "Bismuth",
            Self::Polonium => "Polonium",
            Self::Astatine => "Astatine",
            Self::Radon => "Radon",
            Self::Francium => "Francium",
            Self::Radium => "Radium",
            Self::Actinium => "Actinium",
            Self::Thorium => "Thorium",
            Self::Protactinium => "Protactinium",
            Self::Uranium => "Uranium",
            Self::Neptunium => "Neptunium",
            Self::Plutonium => "Plutonium",
            Self::Americium => "Americium",
            Self::Curium => "Curium",
            Self::Berkelium => "Berkelium",
            Self::Californium => "Californium",
            Self::Einsteinium => "Einsteinium",
            Self::Fermium => "Fermium",
            Self::Mendelevium => "Mendelevium",
            Self::Nobelium => "Nobelium",
            Self::Lawrencium => "Lawrencium",
            Self::Rutherfordium => "Rutherfordium",
            Self::Dubnium => "Dubnium",
            Self::Seaborgium => "Seaborgium",
            Self::Bohrium => "Bohrium",
            Self::Hassium => "Hassium",
            Self::Meitnerium => "Meitnerium",
            Self::Darmstadtium => "Darmstadtium",
            Self::Roentgenium => "Roentgenium",
            Self::Copernicium => "Copernicium",
            Self::Nihonium => "Nihonium",
            Self::Flerovium => "Flerovium",
            Self::Moscovium => "Moscovium",
            Self::Livermorium => "Livermorium",
            Self::Tennessine => "Tennessine",
            Self::Oganesson => "Oganesson",
        }
    }

    /// Returns `Element`'s symbol.
    ///
    /// # Examples
    ///
    /// ```
    /// # use nkl::core::Element;
    /// assert_eq!(Element::Hydrogen.symbol(), "H");
    /// ```
    pub fn symbol(&self) -> &str {
        match self {
            Self::Hydrogen => "H",
            Self::Helium => "He",
            Self::Lithium => "Li",
            Self::Beryllium => "Be",
            Self::Boron => "B",
            Self::Carbon => "C",
            Self::Nitrogen => "N",
            Self::Oxygen => "O",
            Self::Fluorine => "F",
            Self::Neon => "Ne",
            Self::Sodium => "Na",
            Self::Magnesium => "Mg",
            Self::Aluminium => "Al",
            Self::Silicon => "Si",
            Self::Phosphorus => "P",
            Self::Sulfur => "S",
            Self::Chlorine => "Cl",
            Self::Argon => "Ar",
            Self::Potassium => "K",
            Self::Calcium => "Ca",
            Self::Scandium => "Sc",
            Self::Titanium => "Ti",
            Self::Vanadium => "V",
            Self::Chromium => "Cr",
            Self::Manganese => "Mn",
            Self::Iron => "Fe",
            Self::Cobalt => "Co",
            Self::Nickel => "Ni",
            Self::Copper => "Cu",
            Self::Zinc => "Zn",
            Self::Gallium => "Ga",
            Self::Germanium => "Ge",
            Self::Arsenic => "As",
            Self::Selenium => "Se",
            Self::Bromine => "Br",
            Self::Krypton => "Kr",
            Self::Rubidium => "Rb",
            Self::Strontium => "Sr",
            Self::Yttrium => "Y",
            Self::Zirconium => "Zr",
            Self::Niobium => "Nb",
            Self::Molybdenum => "Mo",
            Self::Technetium => "Tc",
            Self::Ruthenium => "Ru",
            Self::Rhodium => "Rh",
            Self::Palladium => "Pd",
            Self::Silver => "Ag",
            Self::Cadmium => "Cd",
            Self::Indium => "In",
            Self::Tin => "Sn",
            Self::Antimony => "Sb",
            Self::Tellurium => "Te",
            Self::Iodine => "I",
            Self::Xenon => "Xe",
            Self::Caesium => "Cs",
            Self::Barium => "Ba",
            Self::Lanthanum => "La",
            Self::Cerium => "Ce",
            Self::Praseodymium => "Pr",
            Self::Neodymium => "Nd",
            Self::Promethium => "Pm",
            Self::Samarium => "Sm",
            Self::Europium => "Eu",
            Self::Gadolinium => "Gd",
            Self::Terbium => "Tb",
            Self::Dysprosium => "Dy",
            Self::Holmium => "Ho",
            Self::Erbium => "Er",
            Self::Thulium => "Tm",
            Self::Ytterbium => "Yb",
            Self::Lutetium => "Lu",
            Self::Hafnium => "Hf",
            Self::Tantalum => "Ta",
            Self::Tungsten => "W",
            Self::Rhenium => "Re",
            Self::Osmium => "Os",
            Self::Iridium => "Ir",
            Self::Platinum => "Pt",
            Self::Gold => "Au",
            Self::Mercury => "Hg",
            Self::Thallium => "Tl",
            Self::Lead => "Pb",
            Self::Bismuth => "Bi",
            Self::Polonium => "Po",
            Self::Astatine => "At",
            Self::Radon => "Rn",
            Self::Francium => "Fr",
            Self::Radium => "Ra",
            Self::Actinium => "Ac",
            Self::Thorium => "Th",
            Self::Protactinium => "Pa",
            Self::Uranium => "U",
            Self::Neptunium => "Np",
            Self::Plutonium => "Pu",
            Self::Americium => "Am",
            Self::Curium => "Cm",
            Self::Berkelium => "Bk",
            Self::Californium => "Cf",
            Self::Einsteinium => "Es",
            Self::Fermium => "Fm",
            Self::Mendelevium => "Md",
            Self::Nobelium => "No",
            Self::Lawrencium => "Lr",
            Self::Rutherfordium => "Rf",
            Self::Dubnium => "Db",
            Self::Seaborgium => "Sg",
            Self::Bohrium => "Bh",
            Self::Hassium => "Hs",
            Self::Meitnerium => "Mt",
            Self::Darmstadtium => "Ds",
            Self::Roentgenium => "Rg",
            Self::Copernicium => "Cn",
            Self::Nihonium => "Nh",
            Self::Flerovium => "Fl",
            Self::Moscovium => "Mc",
            Self::Livermorium => "Lv",
            Self::Tennessine => "Ts",
            Self::Oganesson => "Og",
        }
    }

    /// Returns `Element`'s atomic number `A`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use nkl::core::Element;
    /// assert_eq!(Element::Hydrogen.atomic_number(), 1);
    /// ```
    pub fn atomic_number(&self) -> u32 {
        match self {
            Self::Hydrogen => 1,
            Self::Helium => 2,
            Self::Lithium => 3,
            Self::Beryllium => 4,
            Self::Boron => 5,
            Self::Carbon => 6,
            Self::Nitrogen => 7,
            Self::Oxygen => 8,
            Self::Fluorine => 9,
            Self::Neon => 10,
            Self::Sodium => 11,
            Self::Magnesium => 12,
            Self::Aluminium => 13,
            Self::Silicon => 14,
            Self::Phosphorus => 15,
            Self::Sulfur => 16,
            Self::Chlorine => 17,
            Self::Argon => 18,
            Self::Potassium => 19,
            Self::Calcium => 20,
            Self::Scandium => 21,
            Self::Titanium => 22,
            Self::Vanadium => 23,
            Self::Chromium => 24,
            Self::Manganese => 25,
            Self::Iron => 26,
            Self::Cobalt => 27,
            Self::Nickel => 28,
            Self::Copper => 29,
            Self::Zinc => 30,
            Self::Gallium => 31,
            Self::Germanium => 32,
            Self::Arsenic => 33,
            Self::Selenium => 34,
            Self::Bromine => 35,
            Self::Krypton => 36,
            Self::Rubidium => 37,
            Self::Strontium => 38,
            Self::Yttrium => 39,
            Self::Zirconium => 40,
            Self::Niobium => 41,
            Self::Molybdenum => 42,
            Self::Technetium => 43,
            Self::Ruthenium => 44,
            Self::Rhodium => 45,
            Self::Palladium => 46,
            Self::Silver => 47,
            Self::Cadmium => 48,
            Self::Indium => 49,
            Self::Tin => 50,
            Self::Antimony => 51,
            Self::Tellurium => 52,
            Self::Iodine => 53,
            Self::Xenon => 54,
            Self::Caesium => 55,
            Self::Barium => 56,
            Self::Lanthanum => 57,
            Self::Cerium => 58,
            Self::Praseodymium => 59,
            Self::Neodymium => 60,
            Self::Promethium => 61,
            Self::Samarium => 62,
            Self::Europium => 63,
            Self::Gadolinium => 64,
            Self::Terbium => 65,
            Self::Dysprosium => 66,
            Self::Holmium => 67,
            Self::Erbium => 68,
            Self::Thulium => 69,
            Self::Ytterbium => 70,
            Self::Lutetium => 71,
            Self::Hafnium => 72,
            Self::Tantalum => 73,
            Self::Tungsten => 74,
            Self::Rhenium => 75,
            Self::Osmium => 76,
            Self::Iridium => 77,
            Self::Platinum => 78,
            Self::Gold => 79,
            Self::Mercury => 80,
            Self::Thallium => 81,
            Self::Lead => 82,
            Self::Bismuth => 83,
            Self::Polonium => 84,
            Self::Astatine => 85,
            Self::Radon => 86,
            Self::Francium => 87,
            Self::Radium => 88,
            Self::Actinium => 89,
            Self::Thorium => 90,
            Self::Protactinium => 91,
            Self::Uranium => 92,
            Self::Neptunium => 93,
            Self::Plutonium => 94,
            Self::Americium => 95,
            Self::Curium => 96,
            Self::Berkelium => 97,
            Self::Californium => 98,
            Self::Einsteinium => 99,
            Self::Fermium => 100,
            Self::Mendelevium => 101,
            Self::Nobelium => 102,
            Self::Lawrencium => 103,
            Self::Rutherfordium => 104,
            Self::Dubnium => 105,
            Self::Seaborgium => 106,
            Self::Bohrium => 107,
            Self::Hassium => 108,
            Self::Meitnerium => 109,
            Self::Darmstadtium => 110,
            Self::Roentgenium => 111,
            Self::Copernicium => 112,
            Self::Nihonium => 113,
            Self::Flerovium => 114,
            Self::Moscovium => 115,
            Self::Livermorium => 116,
            Self::Tennessine => 117,
            Self::Oganesson => 118,
        }
    }

    /// Returns `Element`'s group (periodic table column number).
    ///
    /// # Examples
    ///
    /// ```
    /// # use nkl::core::Element;
    /// assert_eq!(Element::Hydrogen.group(), Some(1));
    /// ```
    ///
    /// # Notes
    ///
    /// Lanthanides and actinides (f-block) do not have a group.
    pub fn group(&self) -> Option<u32> {
        match self {
            Element::Hydrogen => Some(1),
            Element::Helium => Some(18),
            Element::Lithium => Some(1),
            Element::Beryllium => Some(2),
            Element::Boron => Some(13),
            Element::Carbon => Some(14),
            Element::Nitrogen => Some(15),
            Element::Oxygen => Some(16),
            Element::Fluorine => Some(17),
            Element::Neon => Some(18),
            Element::Sodium => Some(1),
            Element::Magnesium => Some(2),
            Element::Aluminium => Some(13),
            Element::Silicon => Some(14),
            Element::Phosphorus => Some(15),
            Element::Sulfur => Some(16),
            Element::Chlorine => Some(17),
            Element::Argon => Some(18),
            Element::Potassium => Some(1),
            Element::Calcium => Some(2),
            Element::Scandium => Some(3),
            Element::Titanium => Some(4),
            Element::Vanadium => Some(5),
            Element::Chromium => Some(6),
            Element::Manganese => Some(7),
            Element::Iron => Some(8),
            Element::Cobalt => Some(9),
            Element::Nickel => Some(10),
            Element::Copper => Some(11),
            Element::Zinc => Some(12),
            Element::Gallium => Some(13),
            Element::Germanium => Some(14),
            Element::Arsenic => Some(15),
            Element::Selenium => Some(61),
            Element::Bromine => Some(17),
            Element::Krypton => Some(18),
            Element::Rubidium => Some(1),
            Element::Strontium => Some(2),
            Element::Yttrium => Some(3),
            Element::Zirconium => Some(4),
            Element::Niobium => Some(5),
            Element::Molybdenum => Some(6),
            Element::Technetium => Some(7),
            Element::Ruthenium => Some(8),
            Element::Rhodium => Some(9),
            Element::Palladium => Some(10),
            Element::Silver => Some(11),
            Element::Cadmium => Some(12),
            Element::Indium => Some(13),
            Element::Tin => Some(14),
            Element::Antimony => Some(15),
            Element::Tellurium => Some(16),
            Element::Iodine => Some(17),
            Element::Xenon => Some(18),
            Element::Caesium => Some(1),
            Element::Barium => Some(2),
            Element::Lanthanum => None,
            Element::Cerium => None,
            Element::Praseodymium => None,
            Element::Neodymium => None,
            Element::Promethium => None,
            Element::Samarium => None,
            Element::Europium => None,
            Element::Gadolinium => None,
            Element::Terbium => None,
            Element::Dysprosium => None,
            Element::Holmium => None,
            Element::Erbium => None,
            Element::Thulium => None,
            Element::Ytterbium => None,
            Element::Lutetium => Some(3),
            Element::Hafnium => Some(4),
            Element::Tantalum => Some(5),
            Element::Tungsten => Some(6),
            Element::Rhenium => Some(7),
            Element::Osmium => Some(8),
            Element::Iridium => Some(9),
            Element::Platinum => Some(10),
            Element::Gold => Some(11),
            Element::Mercury => Some(12),
            Element::Thallium => Some(13),
            Element::Lead => Some(14),
            Element::Bismuth => Some(15),
            Element::Polonium => Some(16),
            Element::Astatine => Some(17),
            Element::Radon => Some(18),
            Element::Francium => Some(1),
            Element::Radium => Some(2),
            Element::Actinium => None,
            Element::Thorium => None,
            Element::Protactinium => None,
            Element::Uranium => None,
            Element::Neptunium => None,
            Element::Plutonium => None,
            Element::Americium => None,
            Element::Curium => None,
            Element::Berkelium => None,
            Element::Californium => None,
            Element::Einsteinium => None,
            Element::Fermium => None,
            Element::Mendelevium => None,
            Element::Nobelium => None,
            Element::Lawrencium => Some(3),
            Element::Rutherfordium => Some(4),
            Element::Dubnium => Some(5),
            Element::Seaborgium => Some(6),
            Element::Bohrium => Some(7),
            Element::Hassium => Some(8),
            Element::Meitnerium => Some(9),
            Element::Darmstadtium => Some(10),
            Element::Roentgenium => Some(11),
            Element::Copernicium => Some(12),
            Element::Nihonium => Some(13),
            Element::Flerovium => Some(14),
            Element::Moscovium => Some(15),
            Element::Livermorium => Some(16),
            Element::Tennessine => Some(17),
            Element::Oganesson => Some(18),
        }
    }

    /// Returns an iterator over all elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use nkl::core::Element;
    /// for element in Element::iter() {
    ///     println!("{}", element.name());
    /// }
    /// ```
    pub fn iter() -> impl Iterator<Item = Element> {
        Self::ELEMENTS.iter().copied()
    }
}
