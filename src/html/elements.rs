//! # elements.rs - 타입 안전 HTML 요소
//!
//! ## 목적
//! HTML5 요소를 개별 타입으로 구현하여 구조적 정확성을 보장합니다.
//!
//! ## 핵심 원칙
//!
//! ### 1. 한 요소 = 한 타입
//! 각 HTML 요소를 별도 구조체로 구현합니다.
//! ```rust
//! pub struct H1 { /* ... */ }
//! pub struct H2 { /* ... */ }
//! pub struct Div { /* ... */ }
//! ```
//!
//! ### 2. Content Category로 타입 제약
//! HTML5 명세의 콘텐츠 카테고리를 트레이트로 표현하여
//! 잘못된 HTML 구조를 컴파일 타임에 방지합니다.
//! ```rust
//! impl FlowContent for H1 {}    // H1은 Flow content
//! impl Heading for H1 {}        // H1은 Heading
//! ```
//!
//! ### 3. Node 트레이트로 IRNode 변환
//! 모든 요소는 `to_irnode()`를 통해 중간 표현으로 변환됩니다.
//!
//! ## 요소 구현 패턴
//!
//! ### 기본 구조
//! ```rust
//! pub struct ElementName {
//!     attrs: SharedAttrs,      // HTML 속성
//!     content_or_childs: T,    // 자식 요소 또는 콘텐츠
//! }
//!
//! impl ElementName {
//!     pub fn new(attrs: Attributes<SomeType>, ...) -> Self {
//!         // 생성자
//!     }
//! }
//!
//! impl Node for ElementName {
//!     fn to_irnode(&self) -> IRNode {
//!         // IRNode로 변환
//!     }
//! }
//!
//! impl ContentCategory for ElementName {}  // 카테고리 구현
//! ```
//!
//! ## 구현 상태
//! - [x] H1, H2 (제목 요소)
//! - [x] Div (컨테이너)
//! - [x] Img (이미지)
//! - [ ] TODO: 텍스트 요소 (p, span, a, strong, em, code)
//! - [ ] TODO: 리스트 (ul, ol, li)
//! - [ ] TODO: 의미론적 요소 (article, section, nav, header, footer, aside)
//! - [ ] TODO: 테이블 (table, thead, tbody, tr, th, td)
//! - [ ] TODO: 폼 (form, input, button, label, textarea, select)
//! - [ ] TODO: 미디어 (video, audio, picture, source)
//!
//! ## 구현된 요소
//!
//! ### H1, H2 (제목 요소)
//! **특징:**
//! - 텍스트 콘텐츠만 가능 (`Content`)
//! - FlowContent, Heading 구현
//!
//! **제약:**
//! - 중첩된 제목 불가 (H1 안에 H2 불가)
//! - 텍스트만 가능 (Div 같은 블록 요소 불가)
//!
//! ### Div (범용 컨테이너)
//! **특징:**
//! - FlowContent 자식들을 가질 수 있음
//! - 가장 유연한 컨테이너
//!
//! **사용:**
//! ```rust
//! Div::new(
//!     AttrBuilder::global(),
//!     vec![
//!         Box::new(H1::new(/* ... */)),
//!         Box::new(H2::new(/* ... */)),
//!         Box::new(Img::new(/* ... */)),
//!     ]
//! )
//! ```
//!
//! ### Img (이미지)
//! **특징:**
//! - Void 요소 (자식 없음)
//! - src, alt 속성 필수
//! - FlowContent 구현
//!
//! ## 새 요소 추가 가이드
//!
//! ### 1단계: 요소 정의
//! ```rust
//! pub struct P {
//!     attrs: SharedAttrs,
//!     content: Vec<Box<dyn PhrasingContent>>,  // P는 Phrasing만
//! }
//! ```
//!
//! ### 2단계: 생성자
//! ```rust
//! impl P {
//!     pub fn new(
//!         attrs: Attributes<Global>,
//!         content: Vec<Box<dyn PhrasingContent>>
//!     ) -> Self {
//!         P {
//!             attrs: SharedAttrs::from_map(attrs.table),
//!             content,
//!         }
//!     }
//! }
//! ```
//!
//! ### 3단계: Node 구현
//! ```rust
//! impl Node for P {
//!     fn to_irnode(&self) -> IRNode {
//!         IRNode::new(
//!             TagName::from_str("p"),
//!             self.attrs.clone(),
//!             ElementType::Normal,
//!             self.content.iter()
//!                 .map(|c| Element::Node(c.to_irnode()))
//!                 .collect()
//!         )
//!     }
//! }
//! ```
//!
//! ### 4단계: Content Category 구현
//! ```rust
//! impl FlowContent for P {}
//! impl Palpable for P {}  // 비어있지 않은 콘텐츠
//! ```
//!
//! ## 설계 결정
//!
//! ### 왜 Box<dyn FlowContent>인가?
//! ```rust
//! pub struct Div {
//!     childs: Vec<Box<dyn FlowContent>>,  // 다형성 필요
//! }
//! ```
//!
//! **이유:**
//! - 다양한 타입의 자식 허용 (H1, P, Img 등)
//! - 런타임 다형성 필요
//!
//! **트레이드오프:**
//! - 힙 할당 발생
//! - vtable 간접 참조
//!
//! **완화:**
//! - `to_irnode()`로 변환 후에는 `Vec<IRNode>` (성능 좋음)
//! - 생성 단계에서만 비용 발생 (빌드 시 한 번)
//!
//! ### 왜 Content는 직접 저장하는가?
//! ```rust
//! pub struct H1 {
//!     content: Content,  // Box 없음
//! }
//! ```
//!
//! **이유:**
//! - H1은 텍스트만 가능 (다형성 불필요)
//! - 불필요한 힙 할당 방지
//! - 단순한 타입은 직접 저장이 효율적
//!
//! ### 왜 Element::Node()로 감싸는가?
//! ```rust
//! childs: childs.iter()
//!     .map(|c| Element::Node(c.to_irnode()))  // Element로 감싸기
//!     .collect()
//! ```
//!
//! **이유:**
//! - IRNode는 `Vec<Element>`를 자식으로 가짐
//! - Element는 Text, Node, Raw를 포함하는 enum
//! - 통일된 인터페이스 제공
//!
//! ## HTML5 명세 준수
//!
//! ### Content Model
//! 각 요소가 가질 수 있는 자식을 HTML5 명세에 따라 제한:
//!
//! | 요소 | 허용 자식 | 구현 |
//! |------|----------|------|
//! | div | FlowContent | `Vec<Box<dyn FlowContent>>` |
//! | p | PhrasingContent | `Vec<Box<dyn PhrasingContent>>` |
//! | h1~h6 | PhrasingContent | `Content` (단순화) |
//! | ul | li | `Vec<Li>` |
//! | img | (없음) | Void 요소 |
//!
//! ### Void Elements
//! 자식을 가질 수 없는 요소들:
//! ```rust
//! // Void 요소 목록
//! // area, base, br, col, embed, hr, img, input,
//! // link, meta, param, source, track, wbr
//!
//! impl Node for Img {
//!     fn to_irnode(&self) -> IRNode {
//!         IRNode::new(
//!             TagName::from_str("img"),
//!             self.attrs.clone(),
//!             ElementType::Void,  // Void 명시
//!             vec![],             // 자식 없음
//!         )
//!     }
//! }
//! ```
//!

use crate::html::attributes::{Attributes, Global, Image, SharedAttrs};
use crate::html::node::{Element, ElementType, FlowContent, Heading, IRNode, Node};
use crate::html::trust::{self, Content, TagName};

// ============================================================================
// 제목 요소 (Heading Elements)
// ============================================================================

/// H1 제목 요소. 가장 높은 수준의 제목.
///
/// # HTML5 명세
/// - Content model: Phrasing content
/// - Categories: Flow content, Heading content, Palpable content
#[derive(Clone)]
pub struct H1 {
    attrs: SharedAttrs,
    content: trust::Content,
}

impl H1 {
    pub fn new(attrs: Attributes<Global>, content: Content) -> Self {
        H1 {
            attrs: SharedAttrs::from_map(attrs.table),
            content: content,
        }
    }
}

impl Node for H1 {
    fn to_irnode(&self) -> IRNode {
        IRNode::new(
            TagName::from_str("h1"),
            self.attrs.clone(),
            ElementType::Normal,
            vec![Element::Text(self.content.clone())],
        )
    }
}

impl FlowContent for H1 {}
impl Heading for H1 {}

/// H2 제목 요소. 두 번째 수준의 제목.
///
/// # HTML5 명세
/// - Content model: Phrasing content
/// - Categories: Flow content, Heading content, Palpable content
#[derive(Clone)]
pub struct H2 {
    attrs: SharedAttrs,
    content: trust::Content,
}

impl H2 {
    pub fn new(attrs: Attributes<Global>, content: Content) -> Self {
        H2 {
            attrs: SharedAttrs::from_map(attrs.table),
            content,
        }
    }
}

impl Node for H2 {
    fn to_irnode(&self) -> IRNode {
        IRNode::new(
            TagName::from_str("h2"),
            self.attrs.clone(),
            ElementType::Normal,
            vec![Element::Text(self.content.clone())],
        )
    }
}

impl FlowContent for H2 {}

// ============================================================================
// 컨테이너 요소 (Container Elements)
// ============================================================================

/// Div 요소. 범용 블록 레벨 컨테이너.
///
/// # HTML5 명세
/// - Content model: Flow content
/// - Categories: Flow content, Palpable content (자식이 있을 때)
///
/// # 특징
/// - 가장 유연한 컨테이너
/// - 의미 없는 그룹핑에 사용
/// - 레이아웃 목적으로 주로 사용
#[derive(Clone)]
pub struct Div {
    attrs: SharedAttrs,
    childs: Vec<Element>,
}

impl Div {
    /// 새 Div 생성
    ///
    /// # Arguments
    /// - `attrs`: HTML 속성
    /// - `childs`: FlowContent를 구현하는 자식 요소들
    pub fn new(attrs: Attributes<Global>, childs: Vec<Box<dyn FlowContent>>) -> Self {
        Div {
            attrs: SharedAttrs::from_map(attrs.table),
            childs: childs
                .iter()
                .map(|c| Element::Node(c.to_irnode()))
                .collect(),
        }
    }
}

impl Node for Div {
    fn to_irnode(&self) -> IRNode {
        IRNode::new(
            TagName::from_str("div"),
            self.attrs.clone(),
            ElementType::Normal,
            self.childs.clone(),
        )
    }
}

impl FlowContent for Div {}

// ============================================================================
// 임베디드 콘텐츠 (Embedded Content)
// ============================================================================

/// Img 요소. 이미지 삽입.
///
/// # HTML5 명세
/// - Content model: (없음 - Void 요소)
/// - Categories: Flow content, Phrasing content, Embedded content,
///               Interactive content (usemap 속성 있을 때),
///               Palpable content
///
/// # 필수 속성
/// - `src`: 이미지 URL
/// - `alt`: 대체 텍스트
pub struct Img {
    attrs: SharedAttrs,
}

impl Img {
    /// 새 Img 생성
    ///
    /// # Arguments
    /// - `attrs`: Image 타입 속성 (src, alt 포함)
    ///
    /// # Example
    /// ```rust
    /// let img = Img::new(
    ///     AttrBuilder::image()
    ///         .src(AttrValue::from_str("/logo.png", &rule))
    ///         .alt(AttrValue::from_str("Company Logo", &rule))
    /// );
    /// ```
    pub fn new(attrs: Attributes<Image>) -> Self {
        Img {
            attrs: SharedAttrs::from_map(attrs.table),
        }
    }
}

impl Node for Img {
    fn to_irnode(&self) -> IRNode {
        IRNode::new(
            TagName::from_str("img"),
            self.attrs.clone(),
            ElementType::Void,  // Void: 자식 없음
            vec![],
        )
    }
}

impl FlowContent for Img {}

// TODO: 다음 요소들 구현
// - P: 문단
// - Span: 인라인 컨테이너
// - A: 링크
// - Strong, Em: 강조
// - Code, Pre: 코드
// - Ul, Ol, Li: 리스트
// - Article, Section, Nav, Header, Footer, Aside: 의미론적 요소
// - Table, Thead, Tbody, Tr, Th, Td: 테이블
