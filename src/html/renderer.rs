//! # renderer.rs - HTML 렌더러
//!
//! ## 목적
//! IRNode 트리를 순회하며 최종 HTML 문자열을 생성합니다.
//!
//! ## 핵심 개념
//!
//! ### Visitor 패턴
//! 렌더러는 Visitor 패턴을 구현하여 IRNode 트리를 순회합니다.
//! 각 노드/요소 방문 시 적절한 메서드가 호출됩니다.
//!
//! ### 불변 렌더러 (Immutable Renderer)
//! 함수형 프로그래밍 스타일을 채택하여 각 방문마다 새 렌더러를 반환합니다.
//!
//! **장점:**
//! - 예측 가능: 이전 상태에 영향받지 않음
//! - 디버깅 용이: 각 단계의 중간 결과 확인 가능
//! - 테스트 쉬움: 부작용 없음
//! - 병렬화 가능성: 향후 병렬 렌더링 고려
//!
//! **단점:**
//! - 성능: 매 단계마다 문자열 복사 발생
//! - 메모리: 중간 렌더러 인스턴스 생성
//!
//! **트레이드오프:**
//! 현재는 단순성과 안전성을 우선. 실제 병목 확인 후 최적화 계획.
//!
//! ## 렌더링 흐름
//! ```text
//! IRNode 트리
//!   ↓
//! visit_node_begin()      → "<div class='container'>"
//!   ↓
//! visit_text()            → "Hello "
//!   ↓
//! visit_node_begin()      → "<strong>"
//!   ↓
//! visit_text()            → "World"
//!   ↓
//! visit_node_end()        → "</strong>"
//!   ↓
//! visit_node_end()        → "</div>"
//!   ↓
//! finalize()              → 최종 HTML 문자열
//! ```
//!
//! ## 사용 예시
//! ```rust
//! let renderer = HtmlRenderer::new();
//! let irnode = div.to_irnode();
//!
//! // IRNode가 렌더러를 받아 순회
//! let final_renderer = irnode.accept(renderer);
//!
//! // 최종 HTML 문자열 획득
//! let html = final_renderer.finalize();
//! println!("{}", html);
//! // → "<div class='container'>Hello <strong>World</strong></div>"
//! ```
//!
//!
//! ## Renderer 트레이트
//! 모든 렌더러가 구현해야 하는 인터페이스.
//! 새로운 출력 형식을 지원하려면 이 트레이트를 구현하면 됩니다.
//!
//! ### 메서드
//! - `visit_node_begin`: 노드 시작 (여는 태그)
//! - `visit_node_end`: 노드 종료 (닫는 태그)
//! - `visit_text`: 텍스트 노드
//! - `visit_raw`: 신뢰된 HTML 블록
//! - `finalize`: 최종 결과 반환
//!
//! ## HtmlRenderer 구현
//!
//! ### 내부 버퍼
//! ```rust
//! pub struct HtmlRenderer {
//!     buffer: HtmlBlock,  // 누적된 HTML 문자열
//! }
//! ```
//!
//! ### 각 visit 메서드 동작
//!
//! #### visit_node_begin
//! ```rust
//! // 여는 태그 생성
//! "<tagname attr1='val1' attr2='val2'>"  // Normal
//! "<tagname attr1='val1' attr2='val2' >" // Void (공백 추가)
//! ```
//!
//! #### visit_node_end
//! ```rust
//! // 닫는 태그 생성 (Normal만)
//! "</tagname>"
//!
//! // Void 요소는 닫는 태그 없음
//! ```
//!
//! #### visit_text
//! ```rust
//! // Content는 이미 이스케이프됨
//! buffer += content.as_str();
//! ```
//!
//! #### visit_raw
//! ```rust
//! // HtmlBlock은 신뢰된 HTML
//! buffer += html_block.as_str();
//! ```
//!
//! ## 설계 결정
//!
//! ### 왜 불변 패턴인가?
//! **대안: 가변 렌더러**
//! ```rust
//! let mut renderer = HtmlRenderer::new();
//! renderer.visit_node(&node);  // 내부 상태 변경
//! let html = renderer.finalize();
//! ```
//!
//! **문제점:**
//! - 방문 순서에 따라 결과가 달라질 수 있음
//! - 중간 상태 확인 어려움
//! - 테스트 시 상태 초기화 필요
//!
//! **불변 패턴:**
//! ```rust
//! let r0 = HtmlRenderer::new();
//! let r1 = r0.visit_node(&node1);  // 새 인스턴스
//! let r2 = r1.visit_node(&node2);  // 새 인스턴스
//! // r0, r1은 변경되지 않음
//! ```
//!
//! **이점:**
//! - 각 단계 독립적
//! - 중간 결과 디버깅 가능
//! - 부작용 없음
//!
//! ### 왜 Void 요소에 공백을 추가하는가?
//! ```rust
//! "<img src='...' >"  // 공백 있음
//! "<div>"             // 공백 없음
//! ```
//!
//! HTML5에서는 self-closing 문법(`/>`)이 선택사항이지만,
//! XML/XHTML 호환성을 위해 공백을 추가합니다.
//! (향후 옵션으로 제어 가능하도록 개선 예정)
//!
//! ### 왜 매번 새 HtmlBlock을 생성하는가?
//! ```rust
//! HtmlRenderer {
//!     buffer: HtmlBlock::from_str(&buffer),  // 새 인스턴스
//! }
//! ```
//!
//! **현재:** 단순성 우선
//! - 코드 이해 쉬움
//! - 불변 패턴 명확히 표현
//!
//! **향후 최적화 옵션:**
//! - `Cow<str>` 사용
//! - 버퍼 재사용 (가변 렌더러 도입)
//! - 청크 기반 렌더링
//!
//! 단, **조기 최적화 방지**: 실제 병목이 확인되면 적용
//!
//! ## 확장 가능성
//!
//! ### 다른 렌더러 구현 예시
//! ```rust
//! // JSON 렌더러 (디버깅용)
//! pub struct JsonRenderer {
//!     json: serde_json::Value,
//! }
//!
//! impl Renderer for JsonRenderer {
//!     type Output = String;
//!     // IRNode 트리를 JSON으로 변환
//! }
//!
//! // Markdown 렌더러 (HTML → Markdown 역변환)
//! pub struct MarkdownRenderer {
//!     markdown: String,
//! }
//! ```
//!
//! ## 성능 고려사항
//!
//! ### 현재 성능 특성
//! - 시간 복잡도: O(n) - 각 노드를 한 번씩 방문
//! - 공간 복잡도: O(n) - 중간 렌더러 인스턴스 생성
//! - 문자열 복사: 매 visit마다 발생
//!
//! ### 최적화 시나리오
//! **벤치마크 후 결정:**
//! 1. 문자열 복사가 병목인가?
//!    → `Cow<str>` 도입
//! 2. 메모리 할당이 문제인가?
//!    → 버퍼 재사용
//! 3. 전체 빌드 시간이 느린가?
//!    → 병렬 렌더링
//!
//! **목표:** 1000 페이지 사이트를 10초 이내 빌드
//!

use crate::html::node::{ElementType, IRNode};
use crate::html::trust::{Content, HtmlBlock, SafeString};

/// 렌더러 인터페이스. 모든 렌더러가 구현해야 합니다.
///
/// 새로운 출력 형식(JSON, Markdown 등)을 지원하려면
/// 이 트레이트를 구현하면 됩니다.
pub trait Renderer: Clone {
    type Output;

    /// 노드 시작 시 호출 (여는 태그 처리)
    fn visit_node_begin(&self, node: &IRNode) -> Self;

    /// 노드 종료 시 호출 (닫는 태그 처리)
    fn visit_node_end(&self, node: &IRNode) -> Self;

    /// 텍스트 노드 방문 시 호출
    fn visit_text(&self, content: &Content) -> Self;

    /// 신뢰된 HTML 블록 방문 시 호출
    fn visit_raw(&self, html: &HtmlBlock) -> Self;

    /// 최종 결과 반환
    fn finalize(&self) -> &Self::Output;
}

/// HTML 문자열 렌더러. IRNode → HTML 변환.
#[derive(Clone)]
pub struct HtmlRenderer {
    buffer: HtmlBlock,
}

impl HtmlRenderer {
    pub fn new() -> Self {
        HtmlRenderer {
            buffer: HtmlBlock::from_str(&String::new()),
        }
    }
}

impl Renderer for HtmlRenderer {
    type Output = HtmlBlock;

    /// 여는 태그 생성
    /// Normal: `<tag attr="val">`
    /// Void: `<tag attr="val" >` (공백 추가)
    fn visit_node_begin(&self, node: &IRNode) -> Self {
        let mut buffer = self.buffer.as_str().to_string();

        buffer.push('<');
        buffer.push_str(&node.get_tag().as_str());
        buffer.push_str(&node.get_attrs().into_string());

        match node.get_type() {
            ElementType::Void => {
                buffer.push_str(" >");  // Void: 공백 추가
            }
            ElementType::Normal => {
                buffer.push('>');
            }
        }

        HtmlRenderer {
            buffer: HtmlBlock::from_str(&buffer),
        }
    }

    /// 닫는 태그 생성 (Normal만)
    /// Normal: `</tag>`
    /// Void: (아무것도 하지 않음)
    fn visit_node_end(&self, node: &IRNode) -> Self {
        let mut buffer = self.buffer.as_str().to_string();

        match node.get_type() {
            ElementType::Normal => {
                buffer.push_str("</");
                buffer.push_str(&node.get_tag().as_str());
                buffer.push('>');
            }
            ElementType::Void => {
                // Void 요소는 닫는 태그 없음
            }
        }

        HtmlRenderer {
            buffer: HtmlBlock::from_str(&buffer),
        }
    }

    /// 텍스트 노드 추가
    /// Content는 이미 이스케이프되어 있음
    fn visit_text(&self, content: &Content) -> Self {
        let mut buffer = self.buffer.as_str().to_string();
        buffer.push_str(&content.as_str());

        HtmlRenderer {
            buffer: HtmlBlock::from_str(&buffer),
        }
    }

    /// 신뢰된 HTML 블록 추가
    /// HtmlBlock은 이스케이프하지 않고 그대로 사용
    fn visit_raw(&self, html: &HtmlBlock) -> Self {
        let mut buffer = self.buffer.as_str().to_string();
        buffer.push_str(&html.as_str());

        HtmlRenderer {
            buffer: HtmlBlock::from_str(&buffer),
        }
    }

    /// 최종 HTML 문자열 반환
    fn finalize(&self) -> &Self::Output {
        &self.buffer
    }
}
