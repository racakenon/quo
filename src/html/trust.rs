use core::str;
use std::fmt::Display;

use crate::html::rules;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Content(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AttrValue(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash,PartialOrd, Ord)]
pub struct AttrKey(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtmlBlock(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TagName(String);

pub trait SafeString: Sized {
    fn from_str<T>(s: &str, rule: &T) -> Self
    where
        T: rules::Rules;
    fn as_str(&self) -> &str;
}
impl Display for Content {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Display for AttrKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl Display for AttrValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl Display for HtmlBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl SafeString for Content {
    fn as_str(&self) -> &str {
        &self.0
    }

    fn from_str<T>(s: &str, rule: &T) -> Self
    where
        T: rules::Rules,
    {
        let typo = rule.apply(s);
        Content(escape_html_chars(&typo))
    }
}

impl SafeString for AttrValue {
    fn as_str(&self) -> &str {
        &self.0
    }

    fn from_str<T>(s: &str, rule: &T) -> Self
    where
        T: rules::Rules,
    {
        let typo = rule.apply(s);
        AttrValue(escape_html_chars(&typo))
    }
}

impl AttrKey {
    pub(crate) fn from_str(key: &str) -> Self {
        AttrKey(key.to_string())
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl HtmlBlock {
    pub(crate) fn from_str(block: &str) -> Self {
        HtmlBlock(block.to_string())
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TagName {
    pub(crate) fn from_str(block: &str) -> Self {
        TagName(block.to_string())
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn escape_html_chars(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    for c in input.chars() {
        match c {
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '&' => output.push_str("&amp;"),
            '"' => output.push_str("&quot;"),
            '\'' => output.push_str("&#39;"),
            _ => output.push(c),
        }
    }
    output
}
