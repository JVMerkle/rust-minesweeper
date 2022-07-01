use std::fmt;
use std::fmt::Formatter;
use std::string::String;
use std::vec::Vec;

use regex::Regex;

#[derive(Debug, Clone)]
struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid identifiers (allowed are [0-9A-Za-z-])")
    }
}

struct SemVer {
    major: u32,
    minor: u32,
    patch: u32,
    pre_release: Vec<String>,
    build_metadata: Vec<String>,

    regex_matcher: Regex,
}

impl SemVer {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        SemVer {
            major,
            minor,
            patch,
            pre_release: Vec::new(),
            build_metadata: Vec::new(),
            regex_matcher: Regex::new(r"^[0-9A-Za-z-]+$").unwrap(),
        }
    }

    pub fn major(&self) -> u32 { self.major }
    pub fn minor(&self) -> u32 { self.minor }
    pub fn patch(&self) -> u32 { self.patch }
    pub fn pre_release(&self) -> &[String] { &self.pre_release }
    pub fn build_metadata(&self) -> &[String] { &self.build_metadata }

    pub fn add_pre_release(&mut self, pre_release: String) -> Result<(), ParseError> {
        if !self.regex_matcher.is_match(pre_release.as_str()) {
            Err(ParseError)
        } else {
            self.pre_release.push(pre_release);
            Ok(())
        }
    }

    pub fn add_build_metadata(&mut self, build_metadata: String) -> Result<(), ParseError> {
        if !self.regex_matcher.is_match(build_metadata.as_str()) {
            Err(ParseError)
        } else {
            self.build_metadata.push(build_metadata);
            Ok(())
        }
    }

    fn fmt_list(f: &mut Formatter<'_>, list: &[String]) -> fmt::Result {
        for (i, e) in list.iter().enumerate() {
            write!(f, "{}", e)?;
            if i != list.len() - 1 {
                write!(f, ".")?;
            }
        }
        Ok(())
    }

    fn fmt_pre_release(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if !self.pre_release.is_empty() {
            write!(f, "-")?;
        }
        Self::fmt_list(f, &self.pre_release)?;
        Ok(())
    }

    fn fmt_build_metadata(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if !self.build_metadata.is_empty() {
            write!(f, "+")?;
        }
        Self::fmt_list(f, &self.build_metadata)?;
        Ok(())
    }
}

impl fmt::Display for SemVer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        self.fmt_pre_release(f)?;
        self.fmt_build_metadata(f)?;
        Ok(())
    }
}

fn main() {
    let mut v = SemVer::new(1, 2, 3);
    v.add_pre_release(String::from("alpha")).unwrap();
    v.add_build_metadata(String::from("git0xdeadbeef")).unwrap();
    v.add_build_metadata(String::from("build-id-15")).unwrap();

    println!("Hello, world! This is version {}", v);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pre_release_parse_success() {
        let mut v = SemVer::new(1, 2, 3);
        let is_err = v.add_pre_release(String::from("1AlpHa99")).is_err();
        assert_eq!(is_err, false);
    }

    #[test]
    fn pre_release_parse_failure() {
        let mut v = SemVer::new(1, 2, 3);
        let is_err = v.add_pre_release(String::from("5/&§/$923478")).is_err();
        assert_eq!(is_err, true);
    }
}
