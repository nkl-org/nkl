//! Mass data module.
//!
//! The [`mass`](`crate::data::mass`) module API consist of a single trait:
//! [`AtomicMassLibrary`] which provides access to atomic mass data through the
//! [`AtomicMassLibrary::get`] method.
//!
//! The module provides three main resources implementation for atomic mass data:
//! - [ENDF/B VIII.0](`EndfbAtomicMassLibrary`)
//! - [JEFF 3.3](`JeffAtomicMassLibrary`)
//! - [JENDL 5](`JendlAtomicMassLibrary`)

use std::{
    cell::{Cell, UnsafeCell},
    collections::HashMap,
    mem::MaybeUninit,
    sync::Once,
};

use crate::core::Zai;

// Lazy initialization.
// Replace with std implementation after stabilization.
struct Lazy<T, F = fn() -> T> {
    once: Once,
    init: Cell<Option<F>>,
    value: UnsafeCell<MaybeUninit<T>>,
}

impl<T, F: FnOnce() -> T> Lazy<T, F> {
    const fn new(init: F) -> Self {
        Lazy {
            once: Once::new(),
            init: Cell::new(Some(init)),
            value: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }

    fn get(&self) -> &T {
        self.once.call_once(|| {
            let value = (self.init.take().unwrap())();
            unsafe {
                (*self.value.get()).write(value);
            }
        });
        unsafe { (*self.value.get()).assume_init_ref() }
    }
}

unsafe impl<T: Sync + Send, F: Sync + Send> Sync for Lazy<T, F> {}
unsafe impl<T: Send, F: Send> Send for Lazy<T, F> {}

/// Atomic mass library trait.
pub trait AtomicMassLibrary {
    /// Returns atomic mass of `zai`.
    fn get(&self, zai: Zai) -> Option<f64>;
}

static ENDFB_ATOMIC_MASSES: Lazy<HashMap<Zai, f64>> = Lazy::new(|| {
    let source = include_str!("../../data/atomic_masses/endfb");
    init_atomic_masses(source)
});

static JEFF_ATOMIC_MASSES: Lazy<HashMap<Zai, f64>> = Lazy::new(|| {
    let source = include_str!("../../data/atomic_masses/jeff");
    init_atomic_masses(source)
});

static JENDL_ATOMIC_MASSES: Lazy<HashMap<Zai, f64>> = Lazy::new(|| {
    let source = include_str!("../../data/atomic_masses/jendl");
    init_atomic_masses(source)
});

/// ENDF/B atomic mass library.
///
/// # Reference
///
/// Brown, D. A., et al.  
/// *ENDF/B-VIII. 0: The 8th major release of the nuclear reaction data library with CIELO-project cross sections, new standards and thermal scattering data.*  
/// Nuclear Data Sheets (2018).  
/// Volume: 148  
/// Pages: 1-142  
/// DOI: <https://doi.org/10.1016/j.nds.2018.02.001>
#[derive(Debug)]
pub struct EndfbAtomicMassLibrary;

impl AtomicMassLibrary for EndfbAtomicMassLibrary {
    fn get(&self, zai: Zai) -> Option<f64> {
        ENDFB_ATOMIC_MASSES.get().get(&zai).copied()
    }
}

/// JEFF atomic mass library.
///
/// # Reference
///
/// Plompen, A.J.M., Cabellos, O., De Saint Jean, C. et al.  
/// The joint evaluated fission and fusion nuclear data library, JEFF-3.3.   
/// The European Physical Journal (2020).  
/// Volume: 56  
/// Issue: 7  
/// Pages: 1-108  
/// DOI: <https://doi.org/10.1140/epja/s10050-020-00141-9>
#[derive(Debug)]
pub struct JeffAtomicMassLibrary;

impl AtomicMassLibrary for JeffAtomicMassLibrary {
    fn get(&self, zai: Zai) -> Option<f64> {
        JEFF_ATOMIC_MASSES.get().get(&zai).copied()
    }
}

/// JENDL atomic mass library.
///
/// # Reference
///
/// Iwamoto, O., et al.  
/// Status of JENDL.  
/// In EPJ Web of Conferences.  
/// EDP Sciences (2020).  
/// Volume: 239  
/// Pages: 09002  
/// DOI: <https://doi.org/10.1051/epjconf/202023909002>
#[derive(Debug)]
pub struct JendlAtomicMassLibrary;

impl AtomicMassLibrary for JendlAtomicMassLibrary {
    fn get(&self, zai: Zai) -> Option<f64> {
        JENDL_ATOMIC_MASSES.get().get(&zai).copied()
    }
}

fn init_atomic_masses(source: &str) -> HashMap<Zai, f64> {
    let mut table = HashMap::new();
    for line in source.lines() {
        let z: u32 = line[..3].trim().parse().unwrap();
        let a: u32 = line[4..7].trim().parse().unwrap();
        let i: u32 = line[8..9].trim().parse().unwrap();
        let zai = Zai::new(z, a, i);
        let mass: f64 = line[35..].trim().parse().unwrap();
        table.insert(zai, mass);
    }
    table
}
