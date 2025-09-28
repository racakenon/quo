use crate::html::rules;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Content(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AttrValue(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AttrKey(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtmlBlock(String);

pub trait SafeString: Sized {
    fn from_str<T>(s: &str, rule: &T) -> Self
    where
        T: rules::Rules;
    fn to_str(self) -> String;
}

impl SafeString for Content {
    fn to_str(self) -> String {
        self.0
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
    fn to_str(self) -> String {
        self.0
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
    pub(crate) fn new_trusted(key: &str) -> Self {
        AttrKey(key.to_string())
    }
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl HtmlBlock {
    pub(crate) fn new_trusted(block: &str) -> Self {
        HtmlBlock(block.to_string())
    }
    pub fn into_inner(self) -> String {
        self.0
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
