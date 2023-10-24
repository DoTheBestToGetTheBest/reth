use crate::NippyJarError;
use serde::{Deserialize, Serialize};

mod cuckoo;
pub use cuckoo::Cuckoo;

/// Membership filter set trait.
pub trait InclusionFilter {
    /// Add element to the inclusion list.
    fn add(&mut self, element: &[u8]) -> Result<(), NippyJarError>;

    /// Checks if the element belongs to the inclusion list. **There might be false positives.**
    fn contains(&self, element: &[u8]) -> Result<bool, NippyJarError>;
}

/// Enum with different [`InclusionFilter`] types.
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum InclusionFilters {
    Cuckoo(Cuckoo),
    // Avoids irrefutable let errors. Remove this after adding another one.
    Unused,
}

impl InclusionFilter for InclusionFilters {
    fn add(&mut self, element: &[u8]) -> Result<(), NippyJarError> {
        match self {
            InclusionFilters::Cuckoo(c) => c.add(element),
            InclusionFilters::Unused => Err(NippyJarError::UnsupportedFilterQuery),
        }
    }

    fn contains(&self, element: &[u8]) -> Result<bool, NippyJarError> {
        match self {
            InclusionFilters::Cuckoo(c) => c.contains(element),
            InclusionFilters::Unused => Err(NippyJarError::UnsupportedFilterQuery),
        }
    }
}
