//! Runs the `git` executable. This is the only module that starts processes;
//! everything else receives a [`GitRunner`] so tests can substitute a fake.

use std::io;
use std::path::Path;

/// What one git command produced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitOutput {
    /// Raw bytes: git output (e.g. with `-z`, or file names) need not be UTF-8.
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    /// `None` if git was ended by a signal rather than exiting.
    pub code: Option<i32>,
}

/// Runs git with a list of arguments in a given folder.
pub trait GitRunner {
    /// Fails only if git could not be started (e.g. it is not installed).
    fn run(&self, dir: &Path, args: &[&str]) -> io::Result<GitOutput>;
}

/// The real `git` found on `PATH`.
pub struct SystemGit;

impl GitRunner for SystemGit {
    fn run(&self, _dir: &Path, _args: &[&str]) -> io::Result<GitOutput> {
        todo!()
    }
}

#[cfg(test)]
mod git_runner {
    use super::*;

    // Running `git --version` succeeds and prints git's version line.
    #[test]
    fn git_version_succeeds_and_prints_version_line() {
        let out = SystemGit
            .run(&std::env::temp_dir(), &["--version"])
            .expect("git should start");
        assert_eq!(out.code, Some(0));
        assert!(
            out.stdout.starts_with(b"git version "),
            "stdout was {:?}",
            String::from_utf8_lossy(&out.stdout)
        );
    }

    // Running git with an unknown option fails with a non-zero exit code and
    // an error message on stderr that names the option.
    #[test]
    fn unknown_option_fails_with_error_on_stderr() {
        let out = SystemGit
            .run(&std::env::temp_dir(), &["--no-such-option"])
            .expect("git should start");
        assert!(
            matches!(out.code, Some(code) if code != 0),
            "exit code was {:?}",
            out.code
        );
        assert!(
            String::from_utf8_lossy(&out.stderr).contains("--no-such-option"),
            "stderr was {:?}",
            String::from_utf8_lossy(&out.stderr)
        );
    }
}
