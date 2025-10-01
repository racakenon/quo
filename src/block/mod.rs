//! # block - 의미론적 콘텐츠 단위 계층
//!
//! ## 계층의 목적
//! HTML 요소를 조합하여 **의미 있는 콘텐츠 단위**를 제공합니다.
//! 사용자는 HTML의 세부사항이 아닌, 문서의 의미와 목적으로 콘텐츠를 작성합니다.
//!
//! ## 설계 철학
//!
//! ### 1. 의미론 우선, 구현은 숨김
//! ```text
//! 사용자가 생각하는 것:
//!   "여기에 Rust 코드를 구문 강조해서 보여주고 싶어"
//!   → CodeBlock::new().language("rust").content(code)
//!
//! 라이브러리가 처리하는 것:
//!   <pre class="code-block">
//!     <code class="language-rust hljs">
//!       <span class="keyword">fn</span> ...
//!     </code>
//!   </pre>
//! ```
//!
//! ### 2. 메타데이터 기반 렌더링
//! Block은 렌더링 방식을 메타데이터로부터 결정합니다.
//! ```rust
//! impl Block for CodeBlock {
//!     fn render_to_ir(&self, ctx: &RenderContext) -> IRNode {
//!         // 메타데이터에서 테마 가져오기
//!         let theme = ctx.metadata.get::<SyntaxTheme>()
//!             .unwrap_or(&SyntaxTheme::default());
//!         
//!         // 테마에 따라 다르게 렌더링
//!         let highlighted = syntax_highlight(&self.code, theme);
//!         // ...
//!     }
//! }
//! ```
//!
//! ### 3. 메타데이터 제공자
//! Block은 자신의 메타데이터를 상위 계층에 제공할 책임이 있습니다.
//! ```rust
//! impl Block for CodeBlock {
//!     fn metadata(&self) -> Metadata {
//!         Metadata::new()
//!             .custom(CodeBlockSettings {
//!                 show_line_numbers: self.show_line_numbers,
//!                 highlight_lines: self.highlight_lines.clone(),
//!             })
//!     }
//! }
//! ```
//!
//! ## 계층 관계
//!
//! ### 상위 계층 (Page)과의 관계
//! ```text
//! Page 계층
//!   ↓ 소유
//! Block 인스턴스들
//!   ↓ layout() 호출
//! Block 트리 구성
//!   ↓ render_to_ir() 호출
//! IRNode 트리 생성
//! ```
//!
//! **Page의 책임:**
//! - Block들을 레이아웃에 배치
//! - 페이지 전체 메타데이터 제공
//! - 렌더링 컨텍스트 전달
//!
//! **Block의 책임:**
//! - 자신을 IRNode로 변환
//! - 블록 수준 메타데이터 제공
//! - 컨텍스트 기반 렌더링
//!
//! ### 하위 계층 (HTML)과의 관계
//! ```text
//! Block 계층 (의미론)
//!   ↓ 사용
//! HTML 계층 (표현)
//! ```
//!
//! **관계 특성:**
//! - **단방향 의존**: Block → HTML (HTML은 Block을 모름)
//! - **추상화 경계**: Block은 "무엇"을, HTML은 "어떻게"를 담당
//! - **변경 격리**: HTML 구현 변경이 Block 사용자에게 영향 없음
//!
//! **예시:**
//! ```rust
//! // Block이 HTML을 사용
//! impl Block for MathBlock {
//!     fn render_to_ir(&self, ctx: &RenderContext) -> IRNode {
//!         // HTML 계층 사용
//!         Div::new(
//!             AttrBuilder::global().class(classes!["math-block"]),
//!             vec![
//!                 // 외부 렌더러(KaTeX) 출력을 HtmlBlock으로 주입
//!                 Box::new(RawHtml(
//!                     HtmlBlock::from_str(&katex_output)
//!                 ))
//!             ]
//!         ).to_irnode()
//!     }
//! }
//! ```
//!
//! ## 핵심 트레이트
//!
//! ### Block 트레이트
//! ```rust
//! pub trait Block {
//!     /// 블록을 IRNode로 변환. 렌더링의 핵심 메서드.
//!     fn render_to_ir(&self, ctx: &RenderContext) -> IRNode;
//!     
//!     /// 블록의 메타데이터 반환. Cite 계층에서 수집.
//!     fn metadata(&self) -> Metadata {
//!         Metadata::new()  // 기본: 빈 메타데이터
//!     }
//!     
//!     /// 블록의 고유 ID. 자동 생성 또는 사용자 지정.
//!     fn id(&self) -> Option<BlockId> {
//!         None  // 기본: 자동 생성
//!     }
//! }
//! ```
//!
//! ## RenderContext (렌더링 컨텍스트)
//!
//! ### 역할
//! Cite 계층에서 수집한 모든 정보를 Block에 전달합니다.
//!
//! ### 포함 정보
//! ```rust
//! pub struct RenderContext {
//!     /// 계층적으로 병합된 메타데이터 (Site → Page → Block)
//!     pub metadata: ResolvedMetadata,
//!     
//!     /// 모든 블록의 ID 맵 (상호 참조용)
//!     pub block_ids: HashMap<BlockPath, BlockId>,
//!     
//!     /// 페이지 간 링크 정보
//!     pub page_links: HashMap<PageId, Vec<Link>>,
//!     
//!     /// 자동 번호 매기기 정보
//!     pub counters: CounterMap,
//! }
//! ```
//!
//! ### 사용 패턴
//! ```rust
//! fn render_to_ir(&self, ctx: &RenderContext) -> IRNode {
//!     // 1. 메타데이터 접근
//!     let theme = ctx.metadata.get::<ColorTheme>();
//!     
//!     // 2. 다른 블록 참조
//!     if let Some(target_id) = ctx.block_ids.get(&self.ref_path) {
//!         // 링크 생성
//!     }
//!     
//!     // 3. 자동 번호 사용
//!     let fig_number = ctx.counters.get_number(self.id());
//!     
//!     // 4. IRNode 생성
//!     // ...
//! }
//! ```
//!
//! ## 메타데이터 책임
//!
//! ### 메타데이터 제공 (하향)
//! Block은 자신의 설정을 메타데이터로 제공합니다.
//!
//! ```rust
//! impl Block for ImageGallery {
//!     fn metadata(&self) -> Metadata {
//!         Metadata::new()
//!             .custom(GallerySettings {
//!                 columns: self.columns,
//!                 spacing: self.spacing,
//!                 lightbox_enabled: true,
//!             })
//!     }
//! }
//! ```
//!
//! **수집 과정:**
//! ```text
//! 1. Cite 계층이 모든 Block의 metadata() 호출
//! 2. 계층적 병합 (Site → Page → Block)
//! 3. ResolvedMetadata 생성
//! 4. RenderContext에 포함하여 다시 전달
//! ```
//!
//! ### 메타데이터 소비 (상향)
//! Block은 렌더링 시 상위에서 내려온 메타데이터를 사용합니다.
//!
//! ```rust
//! fn render_to_ir(&self, ctx: &RenderContext) -> IRNode {
//!     // Site 레벨 메타데이터
//!     let site_theme = ctx.metadata.get::<SiteTheme>();
//!     
//!     // Page 레벨 메타데이터
//!     let page_layout = ctx.metadata.get::<PageLayout>();
//!     
//!     // 자신의 메타데이터 (가장 우선)
//!     let block_settings = ctx.metadata.get::<GallerySettings>();
//!     
//!     // 병합된 메타데이터 기반 렌더링
//!     // ...
//! }
//! ```
//!
//! ### 메타데이터 우선순위
//! ```text
//! Block 메타데이터 (최우선)
//!   ↓ 없으면
//! Page 메타데이터
//!   ↓ 없으면
//! Site 메타데이터
//!   ↓ 없으면
//! 기본값
//! ```
//!
//! ## Block 구현 패턴
//!
//! ### 기본 구조
//! ```rust
//! pub struct MyBlock {
//!     // 사용자가 제공하는 데이터
//!     content: String,
//!     options: MyBlockOptions,
//!     
//!     // 블록 수준 메타데이터 (선택적)
//!     metadata: Metadata,
//! }
//!
//! impl MyBlock {
//!     pub fn new(content: String) -> Self {
//!         MyBlock {
//!             content,
//!             options: MyBlockOptions::default(),
//!             metadata: Metadata::new(),
//!         }
//!     }
//!     
//!     // 빌더 패턴
//!     pub fn with_option(mut self, opt: SomeOption) -> Self {
//!         self.options.set(opt);
//!         self
//!     }
//!     
//!     pub fn with_metadata(mut self, meta: Metadata) -> Self {
//!         self.metadata = meta;
//!         self
//!     }
//! }
//!
//! impl Block for MyBlock {
//!     fn render_to_ir(&self, ctx: &RenderContext) -> IRNode {
//!         // 구현
//!     }
//!     
//!     fn metadata(&self) -> Metadata {
//!         self.metadata.clone()
//!     }
//! }
//! ```
//!
//! ### 외부 도구 통합 패턴
//! ```rust
//! impl Block for DiagramBlock {
//!     fn render_to_ir(&self, ctx: &RenderContext) -> IRNode {
//!         // 1. 외부 렌더러 호출
//!         let svg = match self.engine {
//!             DiagramEngine::Mermaid => 
//!                 mermaid::render(&self.source),
//!             DiagramEngine::Graphviz => 
//!                 graphviz::render(&self.source),
//!         };
//!         
//!         // 2. 메타데이터 기반 래핑
//!         let settings = ctx.metadata.get::<DiagramSettings>();
//!         
//!         // 3. HTML로 조합
//!         Div::new(
//!             AttrBuilder::global()
//!                 .class(classes!["diagram", settings.theme.to_class()]),
//!             vec![
//!                 Box::new(RawHtml(HtmlBlock::from_str(&svg)))
//!             ]
//!         ).to_irnode()
//!     }
//! }
//! ```
//!
//! ### 복잡한 Block (중첩 구조)
//! ```rust
//! pub struct CalloutBlock {
//!     kind: CalloutKind,  // Note, Warning, Tip
//!     title: Option<String>,
//!     children: Vec<Box<dyn Block>>,  // 다른 Block 포함 가능
//! }
//!
//! impl Block for CalloutBlock {
//!     fn render_to_ir(&self, ctx: &RenderContext) -> IRNode {
//!         let mut elements = vec![];
//!         
//!         // 제목 추가
//!         if let Some(title) = &self.title {
//!             elements.push(
//!                 H3::new(
//!                     AttrBuilder::global().class(classes!["callout-title"]),
//!                     Content::from_str(title, &rule)
//!                 ).to_irnode()
//!             );
//!         }
//!         
//!         // 자식 Block들 렌더링
//!         for child in &self.children {
//!             elements.push(child.render_to_ir(ctx));
//!         }
//!         
//!         // 전체 래핑
//!         Div::new(
//!             AttrBuilder::global()
//!                 .class(classes!["callout", self.kind.to_class()]),
//!             elements.into_iter()
//!                 .map(|ir| Box::new(IrNodeWrapper(ir)) as Box<dyn FlowContent>)
//!                 .collect()
//!         ).to_irnode()
//!     }
//! }
//! ```
//!
//! ## 구현해야 할 Block 목록
//!
//! ### 우선순위: 높음 (기본 콘텐츠)
//! - [ ] `Paragraph`: 일반 문단
//! - [ ] `CodeBlock`: 코드 블록 (구문 강조)
//! - [ ] `MathBlock`: 수식 (KaTeX/MathJax)
//! - [ ] `ImageBlock`: 단일 이미지 (캡션 포함)
//! - [ ] `QuoteBlock`: 인용문
//!
//! ### 우선순위: 중간 (향상된 콘텐츠)
//! - [ ] `CalloutBlock`: Note, Warning, Tip, Info
//! - [ ] `DiagramBlock`: Mermaid, Graphviz
//! - [ ] `TableBlock`: 마크다운 스타일 테이블
//! - [ ] `ImageGallery`: 이미지 갤러리
//! - [ ] `VideoBlock`: 비디오 임베드
//!
//! ### 우선순위: 낮음 (특수 기능)
//! - [ ] `TableOfContents`: 자동 목차 생성
//! - [ ] `CodeComparison`: 코드 비교 (diff)
//! - [ ] `TabsBlock`: 탭 인터페이스
//! - [ ] `AccordionBlock`: 접을 수 있는 섹션
//! - [ ] `EmbedBlock`: 외부 콘텐츠 임베드 (YouTube, Twitter 등)
//!
//! ### 레이아웃 Block (Page 계층과 공유)
//! - [ ] `HBox`: 수평 배치
//! - [ ] `VBox`: 수직 배치
//! - [ ] `Grid`: 그리드 레이아웃
//! - [ ] `Spacer`: 공백
//! - [ ] `Divider`: 구분선
//!
//! ## 설계 결정
//!
//! ### 왜 Block은 메타데이터를 제공하는가?
//! **문제:** 렌더링에 필요한 정보를 어디서 가져올 것인가?
//!
//! **대안 1: Block이 모든 설정을 내부에 저장**
//! ```rust
//! pub struct CodeBlock {
//!     theme: SyntaxTheme,        // ❌ 중복
//!     line_numbers: bool,        // ❌ 페이지마다 다를 수 있음
//!     highlight_style: String,   // ❌ 사이트 전역 설정일 수 있음
//! }
//! ```
//! 문제:
//! - 설정 중복 (모든 CodeBlock이 동일한 테마 저장)
//! - 일관성 깨짐 (블록마다 다른 테마 사용 가능)
//! - 전역 설정 변경 어려움
//!
//! **대안 2: 렌더링 시 모든 설정을 인자로 전달**
//! ```rust
//! fn render_to_ir(
//!     &self,
//!     theme: SyntaxTheme,
//!     line_numbers: bool,
//!     // ... 수십 개 인자
//! ) -> IRNode
//! ```
//! 문제:
//! - 인자 폭발
//! - 새 설정 추가 시 모든 시그니처 변경
//!
//! **현재 방식: 메타데이터 시스템**
//! ```rust
//! fn render_to_ir(&self, ctx: &RenderContext) -> IRNode {
//!     let theme = ctx.metadata.get::<SyntaxTheme>();
//!     // ...
//! }
//! ```
//! 장점:
//! - 계층적 설정 (Site → Page → Block)
//! - 필요한 것만 가져옴
//! - 확장 용이 (새 메타데이터 타입 추가)
//!
//! ### 왜 render_to_ir()에 &self인가?
//! ```rust
//! fn render_to_ir(&self, ctx: &RenderContext) -> IRNode
//! //             ^^^^^ 불변 참조
//! ```
//!
//! **이유:**
//! - Block은 데이터만 저장, 렌더링은 읽기 전용
//! - 동일한 Block을 여러 번 렌더링 가능
//! - 멀티스레드 안전성 (향후)
//!
//! **예시:**
//! ```rust
//! let block = CodeBlock::new(code);
//!
//! // 동일한 블록을 다른 테마로 렌더링
//! let ctx_light = /* ... light theme ... */;
//! let html_light = block.render_to_ir(&ctx_light);
//!
//! let ctx_dark = /* ... dark theme ... */;
//! let html_dark = block.render_to_ir(&ctx_dark);
//! ```
//!
//! ### 왜 IRNode를 반환하는가? (소유권)
//! ```rust
//! fn render_to_ir(&self, ctx: &RenderContext) -> IRNode
//! //                                             ^^^^^^ 소유권 이동
//! ```
//!
//! **대안: 참조 반환**
//! ```rust
//! fn render_to_ir(&self, ctx: &RenderContext) -> &IRNode
//! ```
//! 문제:
//! - 생명주기 복잡도
//! - IRNode를 어디에 저장할 것인가?
//!
//! **현재 방식:**
//! - 렌더링 시점에 IRNode 생성
//! - 소유권 이동으로 명확한 책임
//! - Page가 모든 IRNode를 수집하여 트리 구성
//!
//! ## 사용 예시
//!
//! ### 간단한 Block 사용
//! ```rust
//! use quo::block::*;
//!
//! // Block 생성
//! let code = CodeBlock::new()
//!     .language("rust")
//!     .content("fn main() { println!(\"Hello\"); }")
//!     .show_line_numbers(true);
//!
//! // 메타데이터 설정
//! let code_with_meta = code.with_metadata(
//!     Metadata::new()
//!         .custom(SyntaxTheme::Dracula)
//! );
//!
//! // 렌더링 (일반적으로 Page 계층이 수행)
//! let ctx = RenderContext { /* ... */ };
//! let irnode = code_with_meta.render_to_ir(&ctx);
//! ```
//!
//! ### 중첩된 Block
//! ```rust
//! let callout = CalloutBlock::warning()
//!     .title("주의사항")
//!     .children(vec![
//!         Box::new(Paragraph::new("이 기능은 실험적입니다.")),
//!         Box::new(CodeBlock::new()
//!             .language("rust")
//!             .content("// 예제 코드")),
//!     ]);
//! ```
//!
//! ## 향후 방향
//!
//! ### 단기 (Phase 1)
//! - 기본 Block 구현 (Paragraph, CodeBlock, MathBlock)
//! - RenderContext 구조 확정
//! - HTML 계층과의 통합 테스트
//!
//! ### 중기 (Phase 2)
//! - 외부 도구 통합 (Mermaid, KaTeX)
//! - CalloutBlock, ImageGallery 등 향상된 Block
//! - 메타데이터 기반 테마 시스템
//!
//! ### 장기 (Phase 3)
//! - 사용자 정의 Block 지원 강화
//! - Block 매크로 (선언적 Block 생성)
//! - 인터랙티브 Block (클라이언트 사이드 기능)
//!
//! ## 참고 자료
//! - [Markdown 확장 문법](https://www.markdownguide.org/extended-syntax/)
//! - [MDX 컴포넌트](https://mdxjs.com/)
//! - [Notion 블록 시스템](https://developers.notion.com/reference/block)

pub mod block;
