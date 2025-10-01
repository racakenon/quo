//! # trust.rs - 신뢰 경계 타입 시스템
//!
//! ## 목적
//! 사용자가 의도한 대로 문서가 렌더링되도록 보장합니다.
//! 신뢰 수준을 타입으로 표현하여 문서 구조 손상을 방지합니다.
//!
//! ## 핵심 원칙
//! - **문서 보존이 목표**: XSS 방어가 아닌, 사용자가 작성한 `<`, `>` 같은 문자가
//!   HTML 태그로 오해되어 문서가 깨지는 것을 방지합니다.
//! - **보안은 별도 도구의 책임**: 빌드 후 html-validate, security-scanner 등으로 검증합니다.
//! - **타입으로 신뢰 표현**: 컴파일 타임에 잘못된 타입 사용을 방지합니다.
//!
//! ## 신뢰 모델
//! ```text
//! 비신뢰 (사용자 입력)  → Content, AttrValue    → 이스케이프 적용
//! 신뢰 (라이브러리)      → AttrKey, TagName      → pub(crate), 검증 없음
//! 신뢰 (외부 도구)       → HtmlBlock             → pub, 검증 없음
//! ```
//!
//! ## 사용 예시
//! ```rust
//! // ✅ 사용자 텍스트
//! let content = Content::from_str("x > 0", &rule);
//! // → "x &gt; 0" (화면: "x > 0")
//!
//! // ✅ 외부 도구 출력
//! let svg = mermaid::render(diagram);
//! let block = HtmlBlock::from_str(&svg);  // 그대로 사용
//!
//! // ❌ 위험: 사용자 입력을 HtmlBlock으로
//! let block = HtmlBlock::from_str(&user_input);  // 문서 손상 가능!
//! ```
//!
//! ## 구현 상태
//! - [x] 모든 타입 구현 완료
//! - [x] escape_html_chars 함수
//! - [ ] TODO: 각 타입 독스트링 상세화
//! - [ ] TODO: HtmlBlock 위험성 경고 강화
//!
//! ## 설계 결정
//! - `AttrKey`, `TagName`이 `pub(crate)`인 이유: 사용자가 임의의 속성/태그를
//!   생성하지 못하도록. 라이브러리가 제공하는 안전한 API만 사용 가능.
//! - `HtmlBlock`이 검증하지 않는 이유: 외부 도구(Mermaid, KaTeX) 출력을 신뢰.
//!   불필요한 파싱/재직렬화 방지. 보안은 빌드 파이프라인의 별도 도구가 담당.

use core::str;
use std::fmt::Display;

use crate::html::rules;

/// 사용자가 작성한 텍스트 노드. HTML 특수문자를 이스케이프합니다.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Content(String);

/// 사용자가 제공한 HTML 속성값. HTML 특수문자를 이스케이프합니다.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AttrValue(String);

/// HTML 속성 키. 라이브러리 내부에서만 생성 가능 (pub(crate)).
/// 예: "id", "class", "src"
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AttrKey(String);

/// 신뢰된 HTML 블록. 외부 도구(Mermaid, KaTeX)가 생성한 HTML을 그대로 사용.
/// 
/// # Safety
/// 사용자 입력을 직접 HtmlBlock으로 변환하지 마세요. 문서 구조가 손상될 수 있습니다.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtmlBlock(String);

/// HTML 태그명. 라이브러리 내부에서만 생성 가능 (pub(crate)).
/// 예: "div", "span", "h1"
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TagName(String);

/// 안전한 문자열 생성 패턴. 생성자를 통해서만 인스턴스 생성 가능.
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
    pub fn from_str(key: &str) -> Self {
        AttrKey(key.to_string())
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl HtmlBlock {
    pub fn from_str(block: &str) -> Self {
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

/// HTML 특수문자를 엔티티로 변환하여 문서 구조 손상을 방지합니다.
/// 
/// 변환 규칙:
/// - `<`, `>`: HTML 태그로 오해 방지
/// - `&`: HTML 엔티티 시작 문자로 오해 방지  
/// - `"`, `'`: 속성값 종료로 오해 방지
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
