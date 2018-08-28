use failure::Error;
use std::{ops::Deref, path::Path, str::FromStr};

#[derive(Debug)]
pub struct GlobMatcher {
    inner: ::globset::GlobMatcher,
}

#[derive(Debug)]
pub struct GlobMatchers {
    inner: Vec<GlobMatcher>,
}

impl GlobMatcher {
    pub fn is_match<P: AsRef<Path>>(&self, path: P) -> bool {
        self.inner.is_match(path)
    }
}

impl FromStr for GlobMatcher {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(GlobMatcher {
            inner: ::globset::Glob::new(s)?.compile_matcher(),
        })
    }
}

impl Deref for GlobMatcher {
    type Target = ::globset::GlobMatcher;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl FromStr for GlobMatchers {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits = s.trim_matches(',').split(',');

        Ok(GlobMatchers {
            inner: splits
                .map(|s| GlobMatcher::from_str(s))
                .collect::<Result<Vec<GlobMatcher>, _>>()?,
        })
    }
}

impl Deref for GlobMatchers {
    type Target = [GlobMatcher];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
