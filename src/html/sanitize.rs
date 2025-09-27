use lazy_static::lazy_static;
use serde::Deserialize;
use std::char;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Content(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AttrValue(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AttrKey(String);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtmlBlock(String);

pub trait SafeString: Sized {
    fn from_str(s: &str) -> Self;
    fn into_inner(self) -> String;
}

impl SafeString for Content {
    fn from_str(s: &str) -> Self {
        Content(escape_html_chars(s))
    }
    fn into_inner(self) -> String {
        self.0
    }
}

impl SafeString for AttrValue {
    fn from_str(s: &str) -> Self {
        AttrValue(escape_html_chars(s))
    }
    fn into_inner(self) -> String {
        self.0
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
pub fn convert_to_smart_quotes(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut is_in_double_quote = false;
    let mut is_in_single_quote = false;

    let chars: Vec<char> = s.chars().collect();

    for (i, &current_char) in chars.iter().enumerate() {
        match current_char {
            '"' => {
                if is_in_double_quote {
                    result.push('”');
                } else {
                    result.push('“');
                }
                is_in_double_quote = !is_in_double_quote;
            }
            '\'' => {
                let is_apostrophe = if i > 0 && i < chars.len() - 1 {
                    chars[i - 1].is_alphabetic() && chars[i + 1].is_alphabetic()
                } else {
                    false
                };

                if is_apostrophe {
                    result.push('’');
                } else {
                    if is_in_single_quote {
                        result.push('’');
                    } else {
                        result.push('‘');
                    }
                    is_in_single_quote = !is_in_single_quote;
                }
            }
            _ => {
                result.push(current_char);
            }
        }
    }

    result
}

mod rules {
    use super::*;

    #[derive(Deserialize, Debug)]
    struct CodepointData(HashMap<String, Vec<u32>>);

    pub struct SanitizationRules {
        pub invisible_chars: HashSet<char>,
        pub ambiguous_map: HashMap<char, char>,
    }

    impl SanitizationRules {
        pub fn from_files<P: AsRef<Path>>(
            invisible_path: P,
            ambiguous_path: P,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            let ambiguous_json = fs::read_to_string(ambiguous_path)?;
            let ambiguous_data: CodepointData = serde_json::from_str(&ambiguous_json)?;
            let mut ambiguous_map: HashMap<char, char> = HashMap::new();

            let keys_to_process = ["_common", "_default"];
            for key in keys_to_process.iter() {
                if let Some(codepoints) = ambiguous_data.0.get(*key) {
                    for pair in codepoints.chunks_exact(2) {
                        if let (Some(original), Some(replacement)) =
                            (char::from_u32(pair[0]), char::from_u32(pair[1]))
                        {
                            ambiguous_map.insert(original, replacement);
                        }
                    }
                }
            }

            let invisible_json = fs::read_to_string(invisible_path)?;
            let invisible_data: CodepointData = serde_json::from_str(&invisible_json)?;
            let mut invisible_chars: HashSet<char> = invisible_data
                .0
                .values()
                .flatten()
                .filter_map(|&codepoint| char::from_u32(codepoint))
                .collect();

            for ambiguous_char in ambiguous_map.keys() {
                invisible_chars.remove(ambiguous_char);
            }

            Ok(SanitizationRules {
                invisible_chars,
                ambiguous_map,
            })
        }
    }

    lazy_static! {
        pub static ref RULES: SanitizationRules = {
            SanitizationRules::from_files(
                "src/html/invisibleCharacters.json",
                "src/html/ambiguous.json",
            )
            .expect("Failed to load sanitization rule files")
        };
    }
}
pub fn remove_invisible_chars(input: &str) -> String {
    input
        .chars()
        .filter(|c| !rules::RULES.invisible_chars.contains(c))
        .collect()
}

pub fn replace_ambiguous_chars(input: &str) -> String {
    input
        .chars()
        .map(|c| rules::RULES.ambiguous_map.get(&c).copied().unwrap_or(c))
        .collect()
}

pub fn sanitize_string(input: &str) -> String {
    let without_invisible = remove_invisible_chars(input);
    replace_ambiguous_chars(&without_invisible)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_sanitization() {
        let malicious_input = "<script>alert('XSS & Fun')</script>";
        let expected = "&lt;script&gt;alert(&#39;XSS &amp; Fun&#39;)&lt;/script&gt;";
        let sanitized_content = Content::from_str(malicious_input);
        assert_eq!(sanitized_content.into_inner(), expected);
    }

    #[test]
    fn test_attr_value_sanitization() {
        let malicious_input = "\"><script>alert(\"danger\")</script>";
        let expected = "&quot;&gt;&lt;script&gt;alert(&quot;danger&quot;)&lt;/script&gt;";
        let sanitized_value = AttrValue::from_str(malicious_input);
        assert_eq!(sanitized_value.into_inner(), expected);
    }

    #[test]
    fn test_trusted_types_do_not_sanitize() {
        let raw_html = "<div class='trusted'><span>Hello</span></div>";
        let raw_key = "data-trusted-id";

        let html_block = HtmlBlock::new_trusted(raw_html);
        let attr_key = AttrKey::new_trusted(raw_key);

        assert_eq!(html_block.into_inner(), raw_html);
        assert_eq!(attr_key.into_inner(), raw_key);
    }

    #[test]
    fn test_smart_quotes_improved_logic() {
        let input_1 = "It's a beautiful day.";
        let expected_1 = "It’s a beautiful day.";
        assert_eq!(convert_to_smart_quotes(input_1), expected_1);

        let input_2 = "'Hello world,' she said.";
        let expected_2 = "‘Hello world,’ she said.";
        assert_eq!(convert_to_smart_quotes(input_2), expected_2);

        let input_3 = "She replied, 'It's my cat.'";
        let expected_3 = "She replied, ‘It’s my cat.’";
        assert_eq!(convert_to_smart_quotes(input_3), expected_3);

        let input_4 = r#""'Tis the season,' he sang.""#;
        let expected_4 = "“‘Tis the season,’ he sang.”";
        assert_eq!(convert_to_smart_quotes(input_4), expected_4);
    }

    #[test]
    fn test_remove_invisible() {
        let input = "Hello\u{200B}World";
        assert_eq!(remove_invisible_chars(input), "HelloWorld");
    }

    #[test]
    fn test_replace_ambiguous() {
        let input = "Русский Алфавит";
        let expected = "Pyccкий Aлфaвит";
        assert_eq!(replace_ambiguous_chars(input), expected);
    }

    #[test]
    fn test_full_sanitization() {
        let input = "С\u{200b}АША\u{00a0}the\u{00a0}spy";
        let expected = "CAШA the spy";

        assert_eq!(sanitize_string(input), expected);
    }
}
