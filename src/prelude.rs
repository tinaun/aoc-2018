
pub use itertools::Itertools;
pub use permutohedron::LexicalPermutation;

pub use std::collections::BTreeSet as Set;
pub use std::collections::BTreeMap as Map;

pub use std::io;
pub use std::fmt::{self, Display};

pub use serde_derive::{Serialize, Deserialize};

pub use serde_scan::from_str as scan;
pub use serde_scan::scan as s;

pub fn count<I, T>(iter: I) -> Map<T, usize> 
    where I: IntoIterator<Item = T>,
          T: Ord + Eq,
{
    let mut map = Map::new();

    for item in iter {
        *map.entry(item).or_insert(0) += 1;
    }

    map
}