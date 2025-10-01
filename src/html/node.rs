//! # node.rs - 공통 중간 표현 (IRNode)
//!
//! ## 목적
//! 모든 HTML 구조를 통일된 형식으로 표현하여 성능과 타입 안전성을 동시에 확보합니다.
//!
//! ## 핵심 문제와 해결
//! **문제:** Rust에서 `Vec<Box<dyn Block>>`은 성능 비용이 큼
//! - 각 요소마다 힙 할당
//! - vtable 간접 참조
//! - 메모리 단편화
//! - 캐시 미스
//!
//! **해결:** 공통 중간 표현 (IRNode)
//! - 컴파일 타임에 크기 결정 가능
//! - 연속된 메모리 레이아웃 (`Vec<IRNode>`)
//! - vtable 조회 없이 직접 접근
//! - Zero-cost abstraction 달성
//!
//! ## 변환 흐름
//! ```text
//! Block (의미론적)
//!   ↓ render_to_ir()
//! IRNode (구조적)
//!   ↓ accept(renderer)
//! HTML 문자열
//! ```
//!
//! ## 핵심 타입
//!
//! ### IRNode
//! HTML 요소의 중간 표현. 태그, 속성, 자식 요소를 포함합니다.
//!
//! ### Element
//! IRNode의 자식이 될 수 있는 타입:
//! - `Text(Content)`: 텍스트 노드
//! - `Node(IRNode)`: 중첩된 요소
//! - `Raw(HtmlBlock)`: 신뢰된 HTML (외부 도구 출력)
//!
//! ### ElementType
//! HTML5 요소 분류:
//! - `Void`: 자식을 가질 수 없는 요소 (`<img>`, `<br>`)
//! - `Normal`: 자식을 가질 수 있는 요소 (`<div>`, `<p>`)
//!
//! ## 사용 예시
//! ```rust
//! // Block이 IRNode로 변환
//! impl Block for H1 {
//!     fn render_to_ir(&self, ctx: &RenderContext) -> IRNode {
//!         IRNode::new(
//!             TagName::from_str("h1"),
//!             self.attrs.clone(),
//!             ElementType::Normal,
//!             vec![Element::Text(self.content.clone())]
//!         )
//!     }
//! }
//!
//! // IRNode를 렌더러로 처리
//! let renderer = HtmlRenderer::new();
//! let final_renderer = irnode.accept(renderer);
//! let html = final_renderer.finalize();
//! ```
//!
//! ## 구현 상태
//! - [x] IRNode 코어 구조
//! - [x] Element enum (Text, Node, Raw)
//! - [x] ElementType enum (Void, Normal)
//! - [x] Visitor 패턴 (`accept` 메서드)
//! - [x] Content category 트레이트 정의
//! - [ ] TODO: 모든 Content category 트레이트 구현체 추가
//! - [ ] TODO: IRNode 빌더 패턴 (편의성 향상)
//!
//! ## Content Category 트레이트
//! HTML5 명세의 콘텐츠 카테고리를 트레이트로 표현:
//! - `FlowContent`: 대부분의 요소 (div, p, h1, ...)
//! - `PhrasingContent`: 텍스트 레벨 요소 (span, strong, em, ...)
//! - `Heading`: 제목 요소 (h1~h6)
//! - `Sectioning`: 섹션 요소 (article, section, nav, ...)
//! - 기타: Embedded, Interactive, MetadataContent 등
//!
//! 목적: 타입 시스템으로 HTML 구조 규칙 강제
//! ```rust
//! // ✅ p 요소는 FlowContent를 자식으로 가질 수 있음
//! fn P::new(children: Vec<Box<dyn PhrasingContent>>)
//!
//! // ❌ p 요소 안에 div 불가 (컴파일 에러)
//! // div는 FlowContent지만 PhrasingContent 아님
//! ```
//!
//! ## Visitor 패턴
//! `accept()` 메서드는 렌더러가 트리를 순회하며 처리할 수 있도록 합니다.
//!
//! ### 순회 순서
//! 1. `visit_node_begin(self)` - 여는 태그 처리
//! 2. 자식 요소들 재귀 순회
//!    - Text → `visit_text(content)`
//!    - Node → 재귀 `accept()`
//!    - Raw → `visit_raw(html_block)`
//! 3. `visit_node_end(self)` - 닫는 태그 처리
//!
//! ### fold 패턴
//! 불변 렌더러 패턴을 위해 fold를 사용:
//! ```rust
//! let renderer_after_children = self.childs.iter().fold(
//!     renderer_after_begin,
//!     |current_renderer, child| {
//!         // 각 자식 처리 후 새 렌더러 반환
//!         match child {
//!             Element::Text(c) => current_renderer.visit_text(c),
//!             Element::Node(n) => n.accept(current_renderer),
//!             Element::Raw(h) => current_renderer.visit_raw(h),
//!         }
//!     }
//! );
//! ```
//!
//! ## 설계 결정
//!
//! ### 왜 Box<IRNode>가 아닌 IRNode인가?
//! ```rust
//! // ❌ Box 사용
//! pub enum Element {
//!     Node(Box<IRNode>),  // 불필요한 간접 참조
//! }
//!
//! // ✅ 직접 사용
//! pub enum Element {
//!     Node(IRNode),  // Element가 이미 enum이므로 크기 고정
//! }
//! ```
//! Element는 enum이고, IRNode의 크기는 컴파일 타임에 결정 가능합니다.
//! 재귀적 구조지만 enum 자체가 최대 variant 크기로 고정되므로 안전합니다.
//!
//! ### 왜 clone()을 사용하는가?
//! ```rust
//! pub struct IRNode {
//!     attrs: SharedAttrs,  // Arc<AttrHashMap>
//!     childs: Vec<Element>,
//! }
//! ```
//! - `SharedAttrs`는 내부적으로 `Arc` 사용 → clone()은 참조 카운트만 증가
//! - 실제 데이터 복사 없음 (cheap clone)
//! - 불변 데이터 구조 유지
//!
//! ## 향후 개선
//! - [ ] IRNode 빌더: `IRNode::builder().tag("div").attr(...).child(...).build()`
//! - [ ] 타입 안전 자식 검증: Content category 기반 컴파일 타임 검증
//! - [ ] 성능 프로파일링: 실제 병목 지점 확인

use crate::html::attributes::SharedAttrs;
use crate::html::renderer::Renderer;
use crate::html::trust::Content;
use crate::html::trust::HtmlBlock;
use crate::html::trust::TagName;

/// Block을 IRNode로 변환하는 트레이트.
/// 모든 HTML 요소와 사용자 정의 Block이 구현해야 합니다.
pub trait Node {
    fn to_irnode(&self) -> IRNode;
}

/// IRNode의 자식이 될 수 있는 타입.
#[derive(Clone)]
pub enum Element {
    Text(Content),      // 텍스트 노드 (이스케이프됨)
    Node(IRNode),       // 중첩된 HTML 요소
    Raw(HtmlBlock),     // 신뢰된 HTML (이스케이프 없음)
}

/// HTML 요소 타입. HTML5 명세에 따른 분류.
#[derive(Debug, Clone)]
pub enum ElementType {
    Void,    // 자식 불가능: <img>, <br>, <hr>, <input> 등
    Normal,  // 자식 가능: <div>, <p>, <span> 등
}

/// HTML 요소의 중간 표현. 모든 Block은 최종적으로 IRNode로 변환됩니다.
#[derive(Clone)]
pub struct IRNode {
    tag: TagName,
    attrs: SharedAttrs,
    tagtype: ElementType,
    childs: Vec<Element>,
}

impl IRNode {
    pub fn new(
        tag: TagName,
        attrs: SharedAttrs,
        tagtype: ElementType,
        childs: Vec<Element>,
    ) -> Self {
        IRNode {
            tag,
            attrs,
            tagtype,
            childs,
        }
    }

    pub fn get_tag(&self) -> &TagName {
        &self.tag
    }

    pub fn get_attrs(&self) -> &SharedAttrs {
        &self.attrs
    }

    pub fn get_type(&self) -> &ElementType {
        &self.tagtype
    }

    /// Visitor 패턴: 렌더러가 이 노드와 자식들을 순회하도록 합니다.
    ///
    /// 순회 순서:
    /// 1. visit_node_begin (여는 태그)
    /// 2. 자식들 재귀 순회
    /// 3. visit_node_end (닫는 태그)
    pub fn accept<R: Renderer>(&self, renderer: R) -> R {
        let renderer_after_begin = renderer.visit_node_begin(self);
        let renderer_after_children = self.childs.iter().fold(
            renderer_after_begin,
            |current_renderer, child| match child {
                Element::Text(content) => current_renderer.visit_text(&content),
                Element::Node(irnode) => irnode.accept(current_renderer),
                Element::Raw(html_block) => current_renderer.visit_raw(&html_block),
            },
        );
        let final_renderer = renderer_after_children.visit_node_end(self);
        final_renderer
    }
}

// ============================================================================
// Content Category 트레이트
// HTML5 명세의 콘텐츠 카테고리를 타입으로 표현
// 참고: https://html.spec.whatwg.org/multipage/dom.html#kinds-of-content
// ============================================================================

/// Metadata content: <title>, <link>, <meta>, <style>, <script>
pub trait MetadataContent: Node {}

/// Flow content: 대부분의 body 내부 요소
/// div, p, h1~h6, ul, ol, table, form, section 등
pub trait FlowContent: Node {}

/// Sectioning content: 문서 구조를 나타내는 요소
/// article, aside, nav, section
pub trait Sectioning: Node {}

/// Heading content: 제목 요소
/// h1, h2, h3, h4, h5, h6
pub trait Heading: Node {}

/// Phrasing content: 텍스트와 텍스트 레벨 마크업
/// span, a, strong, em, code, img, br 등
pub trait Phrasing: Node {}

/// Embedded content: 외부 리소스를 포함하는 요소
/// img, video, audio, iframe, canvas, svg 등
pub trait Embedded: Node {}

/// Interactive content: 사용자 상호작용 요소
/// a, button, input, select, textarea 등
pub trait Interactive: Node {}

/// Palpable content: 비어있지 않은 콘텐츠
/// 텍스트나 embedded content를 포함하는 요소
pub trait Palpable: Node {}

/// Script-supporting elements: 스크립트 관련 요소
/// script, template
pub trait Script: Node {}

/// Form-associated elements: 폼 관련 요소
/// input, button, select, textarea, label 등
pub trait Formassociated: Node {}

/// Transparent content: 부모의 콘텐츠 모델을 따르는 요소
/// a, ins, del, object 등
pub trait Transparentcontent: Node {}

/// Ordered list content: ol의 자식으로만 사용 가능
/// li (ordered list 컨텍스트)
pub trait OlContent: Node {}

// TODO: 추가 카테고리
// - TableContent (thead, tbody, tfoot, tr)
// - TrContent (th, td)
// - UlContent (li in unordered list)
// - DlContent (dt, dd)
// - SelectContent (option, optgroup)
