//! # html - 타입 안전 HTML 생성 계층
//!
//! ## 계층의 목적
//! 사용자가 의도한 대로 문서가 렌더링되도록 보장하는 저수준 HTML 생성 계층입니다.
//! XSS 같은 보안 공격이 아닌, **문서 구조 손상 방지**가 핵심 목표입니다.
//!
//! ## 설계 철학
//!
//! ### 1. 문서 보존, 보안 아님
//! ```text
//! Quo HTML 계층의 책임:
//!   ✅ 사용자가 작성한 '<', '>' 같은 문자가 HTML 태그로 오해되는 것 방지
//!   ✅ 타이포그래피 정규화로 일관된 문서 품질
//!   ✅ 구조적으로 올바른 HTML 생성
//!
//! 보안 도구의 책임:
//!   ✅ XSS 취약점 검사
//!   ✅ CSP 정책 검증
//!   ✅ HTML 구조 검증
//! ```
//!
//! ### 2. Block 계층을 위한 프리미티브 제공
//! HTML 계층은 사용자가 직접 사용하지 않습니다.
//! ```text
//! 사용자 (Block 계층)
//!   ↓ 사용
//! CodeBlock, MathBlock (의미론적 단위)
//!   ↓ 내부적으로 사용
//! HTML 계층 (div, span, pre 등)
//! ```
//!
//! ### 3. 컴파일 타임 안전성
//! - 신뢰 경계를 타입으로 표현
//! - 허용된 속성만 설정 가능
//! - IRNode로 성능과 타입 안전성 동시 확보
//!
//! ## 모듈 구조
//!
//! ```text
//! html/
//! ├─ trust.rs          - 신뢰 경계 타입 시스템 (Content, HtmlBlock 등)
//! ├─ rules.rs          - 타이포그래피 정규화 (모호한 문자, 스마트 쿼트)
//! ├─ attributes.rs     - 타입 안전 HTML 속성 관리
//! ├─ node.rs           - IRNode 중간 표현
//! ├─ renderer.rs       - IRNode → HTML 문자열 변환
//! ├─ elements.rs       - 타입 안전 HTML 요소 (H1, Div, Img 등)
//! └─ mod.rs            - 모듈 진입점 (이 파일)
//! ```
//!
//! ## 의존성 그래프
//!
//! ```text
//! renderer
//!   ↓ 사용
//! node ←─────────┐
//!   ↓ 사용        │
//! elements       │
//!   ↓ 사용   ↓ 사용│
//! attributes     │
//!   ↓ 사용        │
//! trust ←────────┤
//!   ↓ 사용        │
//! rules ─────────┘
//! ```
//!
//! ### 의존성 설명
//! - **trust**: 모든 문자열 타입의 기반 (Content, AttrValue 등)
//! - **rules**: trust가 사용하는 정규화 규칙
//! - **attributes**: trust 타입을 사용하여 속성 관리
//! - **node**: trust와 attributes를 사용하여 중간 표현 구축
//! - **elements**: attributes와 node를 사용하여 HTML 요소 구현
//! - **renderer**: node를 순회하며 최종 HTML 생성
//!
//! ## 데이터 흐름
//!
//! ### 생성 단계 (사용자 → IRNode)
//! ```text
//! 1. 사용자 입력
//!    "x > 0"
//!    ↓
//! 2. trust + rules (타이포그래피 정규화 + 이스케이프)
//!    Content("x &gt; 0")
//!    ↓
//! 3. attributes (속성 구성)
//!    AttrBuilder::global().id("example")
//!    ↓
//! 4. elements (HTML 요소 생성)
//!    H1::new(attrs, content)
//!    ↓
//! 5. node (IRNode 변환)
//!    IRNode { tag: "h1", attrs, childs: [Text("x &gt; 0")] }
//! ```
//!
//! ### 렌더링 단계 (IRNode → HTML)
//! ```text
//! IRNode 트리
//!   ↓
//! renderer (Visitor 패턴)
//!   ↓
//! HtmlRenderer::accept()
//!   ↓
//! "<h1 id='example'>x &gt; 0</h1>"
//! ```
//!
//! ## 핵심 개념
//!
//! ### 신뢰 경계 (Trust Boundary)
//! ```rust
//! // 비신뢰: 사용자 입력 → 이스케이프
//! Content::from_str("x > 0", &rule)  // → "x &gt; 0"
//!
//! // 신뢰: 라이브러리 내부 → pub(crate)
//! AttrKey::from_str("id")  // 사용자 접근 불가
//!
//! // 신뢰: 외부 도구 → 검증 없음
//! HtmlBlock::from_str(mermaid_svg)  // 그대로 사용
//! ```
//!
//! ### IRNode (중간 표현)
//! **문제:** `Vec<Box<dyn Block>>`은 성능 비용이 큼
//! - 힙 할당
//! - vtable 간접 참조
//! - 메모리 단편화
//!
//! **해결:** 공통 중간 표현
//! - `Vec<IRNode>`: 컴파일 타임 크기 결정
//! - 연속 메모리 레이아웃
//! - Zero-cost abstraction
//!
//! ### PhantomData 타입 제약
//! ```rust
//! // ✅ 허용
//! AttrBuilder::image().src(url)
//!
//! // ❌ 컴파일 에러
//! AttrBuilder::global().src(url)
//! ```
//! 컴파일 타임에 잘못된 속성 사용을 방지합니다.
//!
//! ## 상위 계층과의 관계
//!
//! ### Block 계층이 HTML 계층을 사용하는 방식
//! ```rust
//! // Block 계층 (사용자가 작성)
//! impl Block for CodeBlock {
//!     fn render_to_ir(&self, ctx: &RenderContext) -> IRNode {
//!         // HTML 계층 사용
//!         let pre = Pre::new(
//!             AttrBuilder::global()
//!                 .class(classes!["code-block"]),
//!             vec![
//!                 Code::new(
//!                     AttrBuilder::global()
//!                         .class(classes![format!("language-{}", self.lang)]),
//!                     HtmlBlock::from_str(&highlighted)  // 외부 도구 출력
//!                 )
//!             ]
//!         );
//!         
//!         pre.to_irnode()
//!     }
//! }
//! ```
//!
//! ### HTML 계층의 독립성
//! HTML 계층은 Block, Page, Cite 계층을 알지 못합니다.
//! ```rust
//! // ❌ HTML 계층은 이런 import 없음
//! use crate::block::*;
//! use crate::metadata::*;
//!
//! // ✅ 상위 계층만 HTML을 사용
//! // block/block.rs
//! use crate::html::*;
//! ```
//!
//! **이유:**
//! - 계층 구조 명확화
//! - 단방향 의존성 (상위 → 하위)
//! - HTML 계층 재사용 가능
//!
//! ## 구현 상태
//!
//! ### 완성된 모듈
//! - [x] trust: 신뢰 경계 타입 시스템
//! - [x] rules: 타이포그래피 정규화
//! - [x] attributes: PhantomData 타입 제약
//! - [x] node: IRNode 중간 표현, Visitor 패턴
//! - [x] renderer: 불변 렌더러
//! - [x] elements: 기본 요소 (H1, H2, Div, Img)
//!
//! ### 진행 중
//! - ⏳ elements: 나머지 HTML5 요소
//!   - [ ] 텍스트: p, span, a, strong, em, code
//!   - [ ] 리스트: ul, ol, li
//!   - [ ] 의미론적: article, section, nav, header, footer
//!   - [ ] 테이블: table, thead, tbody, tr, th, td
//!   - [ ] 폼: form, input, button, label
//!   - [ ] 미디어: video, audio, picture
//!
//! ### TODO
//! - [ ] attributes: 더 많은 속성 그룹 (Form, Table, Media)
//! - [ ] rules: Punctuation 트레이트 완성
//! - [ ] rules: build.rs로 JSON → Rust 코드 생성
//! - [ ] renderer: 포매팅 옵션 (들여쓰기, 압축)
//! - [ ] 성능 벤치마크 및 최적화
//!
//! ## 설계 트레이드오프
//!
//! ### 안전성 vs 편의성
//! **선택:** 안전성 우선
//! - 컴파일 타임 검증 강화
//! - 사용자 API는 Block 계층에서 제공
//! - HTML 계층은 저수준 프리미티브에 집중
//!
//! ### 성능 vs 단순성
//! **선택:** 단순성 우선 (현재)
//! - 불변 렌더러 (디버깅 용이)
//! - 문자열 복사 허용
//! - 실제 병목 확인 후 최적화
//!
//! ### 완전성 vs 실용성
//! **선택:** 실용성 우선
//! - 자주 쓰는 요소만 구현
//! - 희귀 요소는 HtmlBlock으로 우회 가능
//! - 점진적 확장
//!
//! ## 사용 예시
//!
//! ### 간단한 HTML 생성
//! ```rust
//! use quo::html::*;
//!
//! let rule = Default { rules: vec![RuleList::All] };
//!
//! let page = Div::new(
//!     AttrBuilder::global()
//!         .id(AttrValue::from_str("container", &rule))
//!         .class(AttrValues::build_set(vec!["wrapper".into()], &rule)),
//!     vec![
//!         Box::new(H1::new(
//!             AttrBuilder::global(),
//!             Content::from_str("Welcome", &rule)
//!         )),
//!         Box::new(Img::new(
//!             AttrBuilder::image()
//!                 .src(AttrValue::from_str("/logo.png", &rule))
//!                 .alt(AttrValue::from_str("Logo", &rule))
//!         )),
//!     ]
//! );
//!
//! let renderer = HtmlRenderer::new();
//! let irnode = page.to_irnode();
//! let html = irnode.accept(renderer).finalize();
//! println!("{}", html);
//! // → <div class="wrapper" id="container"><h1>Welcome</h1><img alt="Logo" src="/logo.png" ></div>
//! ```
//!
//! ### 외부 도구와 통합
//! ```rust
//! // Mermaid 다이어그램 렌더링
//! let diagram_code = r#"
//!     graph TD
//!     A[Start] --> B[Process]
//! "#;
//!
//! let mermaid_svg = external_mermaid::render(diagram_code);
//!
//! let container = Div::new(
//!     AttrBuilder::global().class(classes!["diagram"]),
//!     vec![
//!         Box::new(RawHtml::new(
//!             HtmlBlock::from_str(&mermaid_svg)
//!         ))
//!     ]
//! );
//! ```
//!
//! ## 향후 방향
//!
//! ### 단기 (Phase 1)
//! - HTML5 기본 요소 완성
//! - Block 계층 기본 구현과 연동 테스트
//!
//! ### 중기 (Phase 2)
//! - 속성 그룹 확장 (Form, Table, Media)
//! - Content Category 기반 타입 검증 강화
//! - 성능 측정 및 병목 해결
//!
//! ### 장기 (Phase 3)
//! - 다른 출력 포맷 지원 (JSON, Markdown)
//! - 스트리밍 렌더링 (메모리 효율)
//! - 병렬 렌더링 (대규모 사이트)
//!
//! ## 참고 자료
//! - [HTML5 명세](https://html.spec.whatwg.org/)
//! - [Content Categories](https://html.spec.whatwg.org/multipage/dom.html#kinds-of-content)
//! - [Character References](https://html.spec.whatwg.org/multipage/syntax.html#character-references)

pub mod trust;
pub mod rules;
pub mod attributes;
pub mod renderer;
pub mod node;
pub mod elements;
