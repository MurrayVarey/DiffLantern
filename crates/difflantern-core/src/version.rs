//! Checks that git is new enough (SPEC.md section 4: git 2.30 or later).

use std::fmt;
use std::path::Path;

use crate::git::GitRunner;

/// A git release number, e.g. 2.43.0. Fields compare in order, so the
/// derived ordering is release order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct GitVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

/// The oldest supported git: 2.30 added `git diff --merge-base`.
pub const MINIMUM_GIT: GitVersion = GitVersion {
    major: 2,
    minor: 30,
    patch: 0,
};

impl fmt::Display for GitVersion {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

/// Why git can't be used.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GitVersionError {
    /// git could not be started.
    Missing,
    /// `git --version` exited unsuccessfully. `None` if ended by a signal.
    Failed { code: Option<i32> },
    /// `git --version` printed something that isn't a version line.
    Unreadable,
    /// git is older than [`MINIMUM_GIT`].
    TooOld { found: GitVersion },
}

impl fmt::Display for GitVersionError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

/// Reads the output of `git --version` and accepts it if git is new enough.
/// Pure: takes git's stdout, runs nothing.
pub fn version_from_output(_stdout: &[u8]) -> Result<GitVersion, GitVersionError> {
    todo!()
}

/// Runs `git --version` in `dir` and checks that git is new enough.
pub fn check_git_version(
    _runner: &impl GitRunner,
    _dir: &Path,
) -> Result<GitVersion, GitVersionError> {
    todo!()
}

#[cfg(test)]
mod git_version {
    use super::*;
    use crate::git::GitOutput;
    use std::io;

    fn version(major: u32, minor: u32, patch: u32) -> GitVersion {
        GitVersion {
            major,
            minor,
            patch,
        }
    }

    // `git version 2.30.0` → accepted (exactly the minimum).
    #[test]
    fn exactly_the_minimum_version_is_accepted() {
        assert_eq!(
            version_from_output(b"git version 2.30.0\n"),
            Ok(version(2, 30, 0))
        );
    }

    // `git version 2.29.9` → too old; the error reports the version found.
    #[test]
    fn version_below_the_minimum_is_too_old_and_reports_it() {
        let err = version_from_output(b"git version 2.29.9\n").unwrap_err();
        assert_eq!(
            err,
            GitVersionError::TooOld {
                found: version(2, 29, 9)
            }
        );
        assert!(err.to_string().contains("2.29.9"), "message was {err}");
    }

    // `git version 2.43.0.windows.1` → accepted (Windows build suffix).
    #[test]
    fn windows_build_suffix_is_accepted() {
        assert_eq!(
            version_from_output(b"git version 2.43.0.windows.1\n"),
            Ok(version(2, 43, 0))
        );
    }

    // `git version 2.39.3 (Apple Git-146)` → accepted (macOS suffix).
    #[test]
    fn macos_suffix_is_accepted() {
        assert_eq!(
            version_from_output(b"git version 2.39.3 (Apple Git-146)\n"),
            Ok(version(2, 39, 3))
        );
    }

    // `git version 3.0.0` → accepted (a higher major version beats a lower
    // minor).
    #[test]
    fn higher_major_version_with_lower_minor_is_accepted() {
        assert_eq!(
            version_from_output(b"git version 3.0.0\n"),
            Ok(version(3, 0, 0))
        );
    }

    // Output that isn't a git version line (empty or garbage) → error saying
    // the git version couldn't be read.
    #[test]
    fn output_that_is_not_a_version_line_is_unreadable() {
        let cases: [&[u8]; 6] = [
            b"",
            b"hello\n",
            b"git version \n",
            b"git version 2.x.0\n",
            b"git version 2.30\n",
            b"version 2.30.0\n",
        ];
        for stdout in cases {
            assert_eq!(
                version_from_output(stdout),
                Err(GitVersionError::Unreadable),
                "stdout was {:?}",
                String::from_utf8_lossy(stdout)
            );
        }
        let message = GitVersionError::Unreadable.to_string();
        assert!(message.contains("version"), "message was {message}");
    }

    /// Answers every git command with a fixed result, and checks that the
    /// only command it is asked to run is `git --version`.
    struct FakeGit(Result<GitOutput, io::ErrorKind>);

    impl FakeGit {
        fn prints(stdout: &str) -> Self {
            FakeGit(Ok(GitOutput {
                stdout: stdout.as_bytes().to_vec(),
                stderr: Vec::new(),
                code: Some(0),
            }))
        }
    }

    impl GitRunner for FakeGit {
        fn run(&self, _dir: &Path, args: &[&str]) -> io::Result<GitOutput> {
            assert_eq!(args, ["--version"]);
            self.0.clone().map_err(io::Error::from)
        }
    }

    fn check(fake: FakeGit) -> Result<GitVersion, GitVersionError> {
        check_git_version(&fake, Path::new("."))
    }

    // git can't be started → error saying git wasn't found (tested with a
    // fake).
    #[test]
    fn git_that_cannot_be_started_is_reported_missing() {
        let err = check(FakeGit(Err(io::ErrorKind::NotFound))).unwrap_err();
        assert_eq!(err, GitVersionError::Missing);
        assert!(err.to_string().contains("not found"), "message was {err}");
    }

    // git reports 2.29.0 → error naming 2.29.0 and the 2.30 minimum (tested
    // with a fake).
    #[test]
    fn old_git_is_rejected_naming_found_and_minimum_versions() {
        let err = check(FakeGit::prints("git version 2.29.0\n")).unwrap_err();
        assert_eq!(
            err,
            GitVersionError::TooOld {
                found: version(2, 29, 0)
            }
        );
        let message = err.to_string();
        assert!(message.contains("2.29.0"), "message was {message}");
        assert!(message.contains("2.30"), "message was {message}");
    }

    // git reports 2.30.0 → accepted (tested with a fake).
    #[test]
    fn git_at_the_minimum_version_is_accepted() {
        assert_eq!(
            check(FakeGit::prints("git version 2.30.0\n")),
            Ok(version(2, 30, 0))
        );
    }

    // `git --version` exits with a non-zero code → error saying git failed
    // (tested with a fake).
    #[test]
    fn git_version_exiting_non_zero_is_reported_as_failed() {
        let err = check(FakeGit(Ok(GitOutput {
            stdout: b"git version 2.30.0\n".to_vec(),
            stderr: b"something broke\n".to_vec(),
            code: Some(1),
        })))
        .unwrap_err();
        assert_eq!(err, GitVersionError::Failed { code: Some(1) });
        assert!(err.to_string().contains("failed"), "message was {err}");
    }
}
