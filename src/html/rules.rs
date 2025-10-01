//! # rules.rs - 타이포그래피 정규화
//!
//! ## 목적
//! 사용자 입력을 표준화하여 일관된 문서 품질을 보장합니다.
//! 모호한 유니코드 문자, 보이지 않는 문자를 처리하고 스마트 쿼트를 적용합니다.
//!
//! ## 핵심 기능
//! - **모호한 문자 치환**: 유사하게 보이는 유니코드를 표준 문자로 변환
//!   - 예: 전각 숫자 "２" → ASCII "2"
//! - **보이지 않는 문자 제거**: Zero-width space 등 제거
//! - **스마트 쿼트**: 직선 따옴표를 구부러진 따옴표로 변환
//!   - `"text"` → `"text"`
//!   - `'text'` → `'text'`
//!   - `it's` → `it's` (아포스트로피는 유지)
//!
//! ## 데이터 소스
//! - `ambiguous.json`: 로케일별 모호한 문자 매핑 (예: ja, ko, zh-hans)
//! - `invisibleCharacters.json`: 로케일별 제거할 보이지 않는 문자
//! - 빌드 시점에 lazy_static으로 로드
//!
//! ## 사용 예시
//! ```rust
//! let rule = Default { rules: vec![RuleList::All] };
//! 
//! // 모호한 문자 치환
//! let normalized = rule.replace_ambiguous_chars("２");  // → "2"
//! 
//! // 보이지 않는 문자 제거
//! let cleaned = rule.remove_invisible_chars("hello\u{200B}world");  // → "helloworld"
//! 
//! // 스마트 쿼트
//! let pretty = rule.punctuation_rule(r#""Hello" and 'world'"#);
//! // → ""Hello" and 'world'"
//! ```
//!
//! ## 구현 상태
//! - [x] ambiguous.json, invisibleCharacters.json 로드
//! - [x] 로케일별 문자 매핑 (_common, _default 폴백)
//! - [x] `replace_ambiguous_chars` 구현
//! - [x] `remove_invisible_chars` 구현
//! - [x] 스마트 쿼트 변환 (아포스트로피 감지)
//! - [ ] TODO: Punctuation 트레이트 완성 (ellipsis, em-dash)
//! - [ ] TODO: build.rs로 JSON → Rust 코드 생성 (컴파일 타임 검증)
//!
//! ## 설계 결정
//! - **런타임 JSON 로드**: 현재는 lazy_static으로 파일 읽기. 향후 build.rs로
//!   컴파일 타임에 Rust 코드로 변환하면 "컴파일 성공 = 안전" 철학에 더 부합.
//! - **로케일 우선순위**: locale → lang-code → _default → _common 순서로 폴백.
//! - **아포스트로피 감지**: 전후 문자가 알파벳이면 따옴표가 아닌 아포스트로피로 처리.
//!
//! ## 로케일 처리
//! ```text
//! 입력: "zh-hans"
//! 1. zh-hans 전용 규칙 확인
//! 2. 없으면 zh 규칙 확인
//! 3. 없으면 _default 규칙
//! 4. 없으면 _common 규칙
//! ```
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

/// 정규화 규칙을 관리하는 핵심 구조체.
/// lazy_static으로 전역 싱글톤 인스턴스 생성.
impl SanitizationRules {
    fn from_files<P: AsRef<Path>>(
        invisible_path: P,
        ambiguous_path: P,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let ambiguous_json = fs::read_to_string(ambiguous_path)?;
        let ambiguous_data: CodepointData = serde_json::from_str(&ambiguous_json)?;

        let invisible_json = fs::read_to_string(invisible_path)?;
        let invisible_data: CodepointData = serde_json::from_str(&invisible_json)?;

        let mut ambiguous_map: HashMap<String, HashMap<char, char>> = HashMap::new();

        // ambiguous.json: [원본1, 대체1, 원본2, 대체2, ...] 형식
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

        // ambiguous 문자는 invisible 목록에서 제외 (중복 방지)
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

    /// 로케일에 해당하는 보이지 않는 문자 집합 반환.
    /// _common + locale 규칙 병합.
    fn get_invisible_chars(&self, locale: &str) -> HashSet<char> {
        let mut result = HashSet::new();

        if let Some(common_chars) = self.invisible_chars.get("_common") {
            result.extend(common_chars);
        }

        if let Some(locale_chars) = self.invisible_chars.get(locale) {
            result.extend(locale_chars);
        } else if locale.contains('-') {
            // "zh-hans" → "zh"로 폴백
            if let Some(lang_code) = locale.split('-').next() {
                if let Some(lang_chars) = self.invisible_chars.get(lang_code) {
                    result.extend(lang_chars);
                }
            }
        }

        result
    }

    /// 로케일에 해당하는 모호한 문자 매핑 반환.
    /// locale → lang-code → _default → _common 순서로 폴백.
    fn get_ambiguous_pairs(&self, locale: &str) -> &HashMap<char, char> {
        if let Some(locale_map) = self.ambiguous_map.get(locale) {
            return locale_map;
        }

        if locale.contains('-') {
            if let Some(lang_code) = locale.split('-').next() {
                if let Some(lang_map) = self.ambiguous_map.get(lang_code) {
                    return &lang_map;
                }
            }
        }

        if let Some(default_map) = self.ambiguous_map.get("_default") {
            return &default_map;
        }

        if let Some(common_map) = self.ambiguous_map.get("_common") {
            return &common_map;
        }

        &EMPTY_AMBIGUOUS_MAP
    }
}

lazy_static! {
    static ref RULES: SanitizationRules = {
        SanitizationRules::from_files(
            "src/html/invisibleCharacters.json",
            "src/html/ambiguous.json",
        )
        .expect("Failed to load sanitization rule files")
    };
    static ref EMPTY_AMBIGUOUS_MAP: HashMap<char, char> = HashMap::new();
}

/// 적용할 규칙 목록
pub enum RuleList {
    All,
    AmbiguousChar,
    InvisibleCharacters,
    Punctuation,
    //TODO add more rules pair with Rules
}
/// 구두점 변환 규칙. TODO: 완전히 구현 필요.
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

/// 정규화 규칙을 적용하는 트레이트.
/// 사용자는 이 트레이트를 구현하여 커스텀 규칙 제공 가능.
pub trait Rules: Sized {
    type Punctuations: Punctuation;
    fn apply(&self, input: &str) -> String;
    fn replace_ambiguous_chars(&self, input: &str) -> String;
    fn remove_invisible_chars(&self, input: &str) -> String;
    fn punctuation_rule(&self, input: &str) -> String;
    //TODO add more rules
}

/// 기본 규칙 구현체. "_default" 로케일 사용.
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

    /// 등록된 규칙을 순서대로 적용
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

    /// 스마트 쿼트 변환. 여는/닫는 따옴표 구분, 아포스트로피 감지.
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
                    // 아포스트로피 감지: 전후가 알파벳이면 아포스트로피
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
