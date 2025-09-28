use lazy_static::lazy_static;
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
    str::FromStr,
};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct CodepointData(HashMap<String, Vec<u32>>);

pub struct SanitizationRules {
    invisible_chars: HashMap<String, HashSet<char>>,
    ambiguous_map: HashMap<String, HashMap<char, char>>,
}

impl SanitizationRules {
    pub fn from_files<P: AsRef<Path>>(
        invisible_path: P,
        ambiguous_path: P,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let ambiguous_json = fs::read_to_string(ambiguous_path)?;
        let ambiguous_data: CodepointData = serde_json::from_str(&ambiguous_json)?;

        let invisible_json = fs::read_to_string(invisible_path)?;
        let invisible_data: CodepointData = serde_json::from_str(&invisible_json)?;

        let mut ambiguous_map: HashMap<String, HashMap<char, char>> = HashMap::new();

        for (k, v) in ambiguous_data.0.iter() {
            let mut replace_map = HashMap::new();
            for pair in v.chunks_exact(2) {
                if let (Some(original), Some(replacement)) =
                    (char::from_u32(pair[0]), char::from_u32(pair[1]))
                {
                    replace_map.insert(original, replacement);
                }
            }
            ambiguous_map.insert(k.to_string(), replace_map);
        }

        let mut invisible_chars: HashMap<String, HashSet<char>> = HashMap::new();

        for (k, v) in invisible_data.0.iter() {
            let invisible_list: HashSet<char> = v
                .iter()
                .filter_map(|&codepoint| char::from_u32(codepoint))
                .collect();
            invisible_chars.insert(k.to_string(), invisible_list);
        }

        for (locale, ambiguous_chars_map) in ambiguous_map.iter() {
            if let Some(invisible_set) = invisible_chars.get_mut(locale) {
                for ambiguous_char in ambiguous_chars_map.keys() {
                    invisible_set.remove(ambiguous_char);
                }
            }
        }

        Ok(SanitizationRules {
            invisible_chars,
            ambiguous_map,
        })
    }

    pub fn get_invisible_chars(&self, locale: &str) -> HashSet<char> {
        let mut result = HashSet::new();

        if let Some(common_chars) = self.invisible_chars.get("_common") {
            result.extend(common_chars);
        }

        if let Some(locale_chars) = self.invisible_chars.get(locale) {
            result.extend(locale_chars);
        } else if locale.contains('-') {
            if let Some(lang_code) = locale.split('-').next() {
                if let Some(lang_chars) = self.invisible_chars.get(lang_code) {
                    result.extend(lang_chars);
                }
            }
        }

        result
    }

    pub fn get_ambiguous_pairs(&self, locale: &str) -> HashMap<char, char> {
        if let Some(locale_map) = self.ambiguous_map.get(locale) {
            return locale_map.clone();
        }

        if locale.contains('-') {
            if let Some(lang_code) = locale.split('-').next() {
                if let Some(lang_map) = self.ambiguous_map.get(lang_code) {
                    return lang_map.clone();
                }
            }
        }

        if let Some(default_map) = self.ambiguous_map.get("_default") {
            return default_map.clone();
        }

        if let Some(common_map) = self.ambiguous_map.get("_common") {
            return common_map.clone();
        }

        HashMap::new()
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

pub enum RuleList {
    All,
    AmbiguousChar,
    InvisibleCharacters,
    Punctuation,
    //TODO add more rules pair with Rules
}
pub trait Punctuation {
    fn ellipsis() -> char {
        '…'
    }
    fn en_dash() -> char {
        '–'
    }
    fn em_dash() -> char {
        '–'
    }
    //TODO diagraph symbols offer
}

pub trait Rules: Sized {
    type Punctuations: Punctuation;
    fn apply(&self, input: &str) -> String;
    fn replace_ambiguous_chars(&self, input: &str) -> String;
    fn remove_invisible_chars(&self, input: &str) -> String;
    fn punctuation_rule(&self, input: &str) -> String;
    //TODO add more rules
}

pub struct Default {
    pub rules: Vec<RuleList>,
}

pub enum DefaultPunc {
    Ellipsis,
    En,
    Em,
}

impl Punctuation for DefaultPunc {}

impl Rules for Default {
    type Punctuations = DefaultPunc;

    fn apply(&self, input: &str) -> String {
        let mut result = String::from_str(input).unwrap();
        for rule in self.rules.iter() {
            match rule {
                RuleList::All => {
                    result = self.replace_ambiguous_chars(&result);
                    result = self.remove_invisible_chars(&result);
                    result = self.punctuation_rule(&result);
                }
                RuleList::AmbiguousChar => {
                    result = self.replace_ambiguous_chars(&result);
                }
                RuleList::InvisibleCharacters => {
                    result = self.remove_invisible_chars(&result);
                }
                RuleList::Punctuation => todo!(),
            }
        }
        result
    }

    fn replace_ambiguous_chars(&self, input: &str) -> String {
        let ambiguous_pair = RULES.get_ambiguous_pairs("_default");
        input
            .chars()
            .map(|c| ambiguous_pair.get(&c).copied().unwrap_or(c))
            .collect()
    }

    fn remove_invisible_chars(&self, input: &str) -> String {
        let invisible_set = RULES.get_invisible_chars("_default");
        input
            .chars()
            .filter(|c| !invisible_set.contains(c))
            .collect()
    }

    fn punctuation_rule(&self, input: &str) -> String {
        let mut result = String::with_capacity(input.len());
        let mut is_in_double_quote = false;
        let mut is_in_single_quote = false;

        let chars: Vec<char> = input.chars().collect();

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
}
