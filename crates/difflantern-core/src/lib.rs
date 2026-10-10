//! Core of DiffLantern: parsing, counting, status and formatting.
//!
//! Everything here is pure except the [`git`] module, which runs git.

pub mod git;
pub mod version;

/// The product name.
pub fn name() -> &'static str {
    "DiffLantern"
}

#[cfg(test)]
mod tests {
    // The core library reports the DiffLantern name.
    #[test]
    fn core_library_reports_its_name() {
        assert_eq!(super::name(), "DiffLantern");
    }
}
