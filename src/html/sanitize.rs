pub struct Content(String);
pub struct AttrValue(String);

#[derive(Debug, Clone, PartialEq, Eq)]
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

pub fn remove_extra_whitespace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<&str>>().join(" ")
}

pub fn convert_selective_ascii(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            'À'..='Å' | 'Ā' | 'à'..='å' | 'ā' => {
                result.push(if c.is_uppercase() { 'A' } else { 'a' })
            }
            'Ç' | 'Č' | 'ç' | 'č' => result.push(if c.is_uppercase() { 'C' } else { 'c' }),
            'È'..='Ë' | 'Ē' | 'Ę' | 'è'..='ë' | 'ē' | 'ę' => {
                result.push(if c.is_uppercase() { 'E' } else { 'e' })
            }
            'Ì'..='Ï' | 'Ī' | 'ì'..='ï' | 'ī' => {
                result.push(if c.is_uppercase() { 'I' } else { 'i' })
            }
            'Ñ' | 'ñ' => result.push(if c.is_uppercase() { 'N' } else { 'n' }),
            'Ò'..='Ö' | 'Ø' | 'Ō' | 'ò'..='ö' | 'ø' | 'ō' => {
                result.push(if c.is_uppercase() { 'O' } else { 'o' })
            }
            'Ù'..='Ü' | 'Ū' | 'ù'..='ü' | 'ū' => {
                result.push(if c.is_uppercase() { 'U' } else { 'u' })
            }
            'Š' | 'š' => result.push(if c.is_uppercase() { 'S' } else { 's' }),
            'Ý' | 'Ÿ' | 'ý' | 'ÿ' => result.push(if c.is_uppercase() { 'Y' } else { 'y' }),
            'Ž' | 'ž' => result.push(if c.is_uppercase() { 'Z' } else { 'z' }),
            'Æ' => result.push_str("AE"),
            'æ' => result.push_str("ae"),
            'Œ' => result.push_str("OE"),
            'œ' => result.push_str("oe"),
            'ß' => result.push_str("ss"),
            _ => result.push(c),
        }
    }
    result
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
    fn test_whitespace_cleanup() {
        let input = "  \t  leading and   trailing \n spaces  ";
        let expected = "leading and trailing spaces";
        assert_eq!(remove_extra_whitespace(input), expected);

        let only_whitespace = " \n \t ";
        assert_eq!(remove_extra_whitespace(only_whitespace), "");
    }

    #[test]
    fn test_selective_ascii_conversion() {
        let input = "résumé à la carte, 안녕하세요, über-naïve";
        let expected = "resume a la carte, 안녕하세요, uber-naive";
        assert_eq!(convert_selective_ascii(input), expected);
    }

    #[test]
    fn test_full_cleanup_and_sanitization_pipeline() {
        let messy_input = r#"
            "   Voilà, un résumé   über-cool! & check this <tag> out   "
        "#;

        let cleaned_whitespace = remove_extra_whitespace(messy_input);
        assert_eq!(
            cleaned_whitespace,
            r#"" Voilà, un résumé über-cool! & check this <tag> out ""#
        );

        let ascii_text = convert_selective_ascii(&cleaned_whitespace);
        assert_eq!(
            ascii_text,
            r#"" Voila, un resume uber-cool! & check this <tag> out ""#
        );

        let smart_quoted_text = convert_to_smart_quotes(&ascii_text);
        assert_eq!(
            smart_quoted_text,
            "“ Voila, un resume uber-cool! & check this <tag> out ”"
        );

        let final_content = Content::from_str(&smart_quoted_text);
        let expected = "“ Voila, un resume uber-cool! &amp; check this &lt;tag&gt; out ”";
        assert_eq!(final_content.into_inner(), expected);
    }
}
