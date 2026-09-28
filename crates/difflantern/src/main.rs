fn main() {}

#[cfg(test)]
mod tests {
    // The CLI crate can call into the core library, which reports the
    // DiffLantern name.
    #[test]
    fn core_library_is_linked() {
        assert_eq!(difflantern_core::name(), "DiffLantern");
    }
}
