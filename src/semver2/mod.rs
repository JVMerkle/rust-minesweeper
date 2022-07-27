use std::fmt;
use std::fmt::Formatter;
use std::string::String;
use std::vec::Vec;

use regex::Regex;

#[derive(Debug, Clone)]
pub struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid identifiers (allowed are [0-9A-Za-z-])")
    }
}

pub(crate) struct SemVer {
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

    #[allow(unused)]
    pub fn from_str(str: &str) -> Result<SemVer, ParseError> {
        let triple = Regex::new(r"^[0-9]+\.[0-9]+\.[0-9]").unwrap();
        let pre_release = Regex::new(r"^-[0-9A-Za-z-]+").unwrap();
        let build_metadata = Regex::new(r"^+[0-9A-Za-z-]+$").unwrap();

        let mut matched = 0;
        match triple.find(str) {
            Some(t) => {
                matched += t.as_str().len();
                println!("Found {}", t.as_str())
            },
            None => panic!("aa")
        }

        match pre_release.find(&str[matched..]) {
            Some(t) => {
                matched += t.as_str().len();
                println!("Found {}", t.as_str());
            },
            None => panic!("no pre_release")
        }

        match build_metadata.find(&str[matched..]) {
            Some(t) => {
                matched += t.as_str().len();
                println!("Found {}", t.as_str());
            },
            None => panic!("no build_metadata")
        }

        Err(ParseError)
    }

    #[allow(unused)]
    pub fn major(&self) -> u32 { self.major }

    #[allow(unused)]
    pub fn minor(&self) -> u32 { self.minor }

    #[allow(unused)]
    pub fn patch(&self) -> u32 { self.patch }

    #[allow(unused)]
    pub fn pre_release(&self) -> &[String] { &self.pre_release }

    #[allow(unused)]
    pub fn build_metadata(&self) -> &[String] { &self.build_metadata }

    pub fn add_pre_release(&mut self, pre_release: String) -> Result<(), ParseError> {
        if !self.regex_matcher.is_match(pre_release.as_str()) {
            Err(ParseError)
        } else {
            self.pre_release.push(pre_release);
            Ok(())
        }
    }

    #[allow(unused)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pre_release_parse_success() {
        let mut v = SemVer::new(1, 2, 3);
        v.add_pre_release(String::from("1AlpHa99")).unwrap();
    }

    #[test]
    fn pre_release_parse_failure1() {
        let mut v = SemVer::new(1, 2, 3);
        let is_err = v.add_pre_release(String::from("...1AlpHa99...")).is_err();
        assert_eq!(is_err, true);
    }

    #[test]
    fn pre_release_parse_failure2() {
        let mut v = SemVer::new(1, 2, 3);
        let is_err = v.add_pre_release(String::from("5/&§/$923478")).is_err();
        assert_eq!(is_err, true);
    }

    // #[test]
    // fn from_string_success() {
    //     let v = SemVer::from_str("1.2.3-alpha.bar+foo1.334.a556").unwrap();
    //
    //     assert_eq!(v.major(), 1);
    //     assert_eq!(v.minor(), 2);
    //     assert_eq!(v.patch(), 3);
    //     assert_eq!(v.pre_release().len(), 2);
    //     assert_eq!(v.pre_release()[0], "alpha");
    //     assert_eq!(v.pre_release()[1], "bar");
    //     assert_eq!(v.build_metadata().len(), 3);
    //     assert_eq!(v.build_metadata()[0], "foo1");
    //     assert_eq!(v.build_metadata()[1], "334");
    //     assert_eq!(v.build_metadata()[2], "a556");
    // }
}
