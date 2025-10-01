//! # page - 완전한 HTML 페이지 생성 계층
//!
//! ## 계층의 목적
//! Block들을 조합하여 완전한 HTML 문서를 생성합니다.
//! 레이아웃, 메타태그, 스타일시트/스크립트 주입 등 페이지 수준의 책임을 담당합니다.
//!
//! ## 설계 철학
//!
//! ### 1. 템플릿으로서의 Page
//! Page는 **데이터가 아닌 구조**를 정의합니다.
//! ```text
//! 잘못된 이해:
//!   Page = 콘텐츠 데이터를 담는 컨테이너
//!
//! 올바른 이해:
//!   Page = 콘텐츠를 배치하는 방법을 정의하는 템플릿
//! ```
//!
//! **템플릿 공유:**
//! ```rust
//! // 모든 BlogPostPage 인스턴스는 동일한 레이아웃 사용
//! pub struct BlogPostPage {
//!     title: String,        // 데이터
//!     content: Vec<Block>,  // 데이터
//! }
//!
//! impl Page for BlogPostPage {
//!     fn layout(&self) -> IRNode {
//!         // 모든 인스턴스가 공유하는 레이아웃 구조
//!         VBox::new(vec![
//!             Box::new(Header { /* ... */ }),
//!             Box::new(Main { /* ... */ }),
//!             Box::new(Footer { /* ... */ }),
//!         ]).to_irnode()
//!     }
//! }
//!
//! let post1 = BlogPostPage { title: "Post 1", /* ... */ };
//! let post2 = BlogPostPage { title: "Post 2", /* ... */ };
//! // → 둘 다 동일한 Header-Main-Footer 구조
//! ```
//!
//! ### 2. 레이아웃 트리 (Layout Tree)
//! Page는 특수 레이아웃 Block들과 사용자 Block들을 조합한 트리 구조입니다.
//! ```text
//! VBox (수직 배치)
//! ├─ Header
//! │  └─ H1 (제목)
//! ├─ HBox (수평 배치)
//! │  ├─ Main (70% 너비)
//! │  │  ├─ Paragraph
//! │  │  ├─ CodeBlock
//! │  │  └─ ImageBlock
//! │  └─ Aside (30% 너비)
//! │     ├─ TableOfContents
//! │     └─ RelatedPosts
//! └─ Footer
//!    └─ Copyright
//! ```
//!
//! ### 3. 완전한 HTML 문서 생성
//! Page는 `<html>`, `<head>`, `<body>` 전체를 책임집니다.
//! ```html
//! <!DOCTYPE html>
//! <html lang="ko">
//! <head>
//!   <meta charset="UTF-8">
//!   <title>페이지 제목</title>
//!   <link rel="stylesheet" href="/styles.css">
//! </head>
//! <body>
//!   <!-- layout()이 생성한 내용 -->
//! </body>
//! </html>
//! ```
//!
//! ## 계층 관계
//!
//! ### 상위 계층 (Cite)과의 관계
//! ```text
//! Cite 계층
//!   ↓ 등록
//! Page 인스턴스들
//!   ↓ 방문자 파이프라인
//! 1. 메타데이터 수집
//! 2. ID 생성
//! 3. 렌더링
//!   ↓
//! HTML 파일들
//! ```
//!
//! **Cite의 책임:**
//! - 모든 Page 등록 및 관리
//! - 방문자 파이프라인 실행
//! - 사이트 전역 메타데이터 제공
//! - 페이지 간 링크 해결
//!
//! **Page의 책임:**
//! - 레이아웃 정의 (`layout()`)
//! - 페이지 메타데이터 제공 (`metadata()`)
//! - 출력 경로 지정 (`path()`)
//! - HTML head 구성 (`head()`)
//!
//! ### 하위 계층 (Block)과의 관계
//! ```text
//! Page 계층 (구조)
//!   ↓ 소유
//! Block 인스턴스들 (콘텐츠)
//!   ↓ 배치
//! 레이아웃 Block (HBox, VBox, Grid)
//! ```
//!
//! **관계 특성:**
//! - **소유**: Page가 Block들을 소유
//! - **조합**: 레이아웃 Block으로 배치
//! - **렌더링 위임**: 각 Block의 `render_to_ir()` 호출
//!
//! **데이터 흐름:**
//! ```rust
//! impl Page for MyPage {
//!     fn layout(&self) -> IRNode {
//!         // 1. 레이아웃 Block 생성
//!         VBox::new(vec![
//!             // 2. 사용자 Block들 배치
//!             Box::new(self.header_block),
//!             Box::new(self.content_block),
//!             Box::new(self.footer_block),
//!         ]).to_irnode()
//!     }
//! }
//! ```
//!
//! ## 핵심 트레이트
//!
//! ### Page 트레이트
//! ```rust
//! pub trait Page {
//!     /// 페이지의 레이아웃 트리 반환.
//!     /// Block들을 레이아웃 Block으로 조합한 구조.
//!     fn layout(&self) -> IRNode;
//!     
//!     /// 페이지 수준 메타데이터 제공.
//!     /// Cite 계층에서 수집하여 Site 메타데이터와 병합.
//!     fn metadata(&self) -> Metadata {
//!         Metadata::new()  // 기본: 빈 메타데이터
//!     }
//!     
//!     /// 출력 파일 경로.
//!     /// 예: "blog/my-post.html", "index.html"
//!     fn path(&self) -> &str;
//!     
//!     /// HTML <head> 내용 생성.
//!     /// 메타태그, 스타일시트, 스크립트 등.
//!     fn head(&self, ctx: &RenderContext) -> HeadElements {
//!         HeadElements::default()  // 기본: 기본 메타태그
//!     }
//! }
//! ```
//!
//! ## 레이아웃 시스템
//!
//! ### 레이아웃 Block
//! 자식 Block들을 특정 방식으로 배치하는 특수 Block입니다.
//!
//! #### VBox (수직 배치)
//! ```rust
//! pub struct VBox {
//!     children: Vec<Box<dyn Block>>,
//!     spacing: Option<Spacing>,
//!     alignment: VerticalAlignment,
//! }
//!
//! // 렌더링 결과
//! <div class="vbox">
//!   <div class="vbox-item"><!-- child 1 --></div>
//!   <div class="vbox-item"><!-- child 2 --></div>
//!   <div class="vbox-item"><!-- child 3 --></div>
//! </div>
//! ```
//!
//! #### HBox (수평 배치)
//! ```rust
//! pub struct HBox {
//!     children: Vec<Box<dyn Block>>,
//!     spacing: Option<Spacing>,
//!     alignment: HorizontalAlignment,
//!     widths: Vec<Width>,  // [70%, 30%] 등
//! }
//!
//! // 렌더링 결과
//! <div class="hbox">
//!   <div class="hbox-item" style="flex: 0 0 70%"><!-- child 1 --></div>
//!   <div class="hbox-item" style="flex: 0 0 30%"><!-- child 2 --></div>
//! </div>
//! ```
//!
//! #### Grid (그리드 배치)
//! ```rust
//! pub struct Grid {
//!     children: Vec<Box<dyn Block>>,
//!     columns: usize,
//!     gap: Option<Spacing>,
//! }
//!
//! // 렌더링 결과
//! <div class="grid" style="grid-template-columns: repeat(3, 1fr)">
//!   <div class="grid-item"><!-- child 1 --></div>
//!   <div class="grid-item"><!-- child 2 --></div>
//!   <div class="grid-item"><!-- child 3 --></div>
//! </div>
//! ```
//!
//! ### 레이아웃 조합 예시
//! ```rust
//! fn layout(&self) -> IRNode {
//!     VBox::new(vec![
//!         // 헤더 (전체 너비)
//!         Box::new(Header::new(/* ... */)),
//!         
//!         // 메인 콘텐츠 (2단 레이아웃)
//!         Box::new(HBox::new(vec![
//!             // 왼쪽: 본문 (70%)
//!             Box::new(Main::new(vec![
//!                 Box::new(self.title_block),
//!                 Box::new(self.content_block),
//!             ])),
//!             
//!             // 오른쪽: 사이드바 (30%)
//!             Box::new(Aside::new(vec![
//!                 Box::new(TableOfContents::new()),
//!                 Box::new(RelatedPosts::new()),
//!             ])),
//!         ])
//!         .widths(vec![Width::Percent(70), Width::Percent(30)])),
//!         
//!         // 푸터 (전체 너비)
//!         Box::new(Footer::new(/* ... */)),
//!     ])
//!     .spacing(Spacing::Large)
//!     .to_irnode()
//! }
//! ```
//!
//! ## HeadElements (HTML head 관리)
//!
//! ### 역할
//! `<head>` 태그 내부의 메타태그, 링크, 스크립트 등을 관리합니다.
//!
//! ### 구조
//! ```rust
//! pub struct HeadElements {
//!     pub title: String,
//!     pub charset: String,
//!     pub viewport: String,
//!     pub description: Option<String>,
//!     pub keywords: Vec<String>,
//!     pub canonical_url: Option<String>,
//!     pub stylesheets: Vec<Stylesheet>,
//!     pub scripts: Vec<Script>,
//!     pub meta_tags: Vec<MetaTag>,
//! }
//! ```
//!
//! ### 사용 예시
//! ```rust
//! impl Page for BlogPostPage {
//!     fn head(&self, ctx: &RenderContext) -> HeadElements {
//!         HeadElements::new()
//!             .title(&format!("{} - My Blog", self.title))
//!             .description(&self.excerpt)
//!             .keywords(vec!["rust", "programming"])
//!             .stylesheet("/css/blog.css")
//!             .script("/js/highlight.js")
//!             .defer(true)
//!             .open_graph(OpenGraph {
//!                 title: self.title.clone(),
//!                 type_: "article",
//!                 image: self.cover_image.clone(),
//!             })
//!     }
//! }
//! ```
//!
//! ### 렌더링 결과
//! ```html
//! <head>
//!   <meta charset="UTF-8">
//!   <meta name="viewport" content="width=device-width, initial-scale=1.0">
//!   <title>My Post - My Blog</title>
//!   <meta name="description" content="Post excerpt...">
//!   <meta name="keywords" content="rust, programming">
//!   <link rel="stylesheet" href="/css/blog.css">
//!   <script src="/js/highlight.js" defer></script>
//!   <meta property="og:title" content="My Post">
//!   <meta property="og:type" content="article">
//! </head>
//! ```
//!
//! ## 메타데이터 책임
//!
//! ### 페이지 메타데이터 제공
//! ```rust
//! impl Page for BlogPostPage {
//!     fn metadata(&self) -> Metadata {
//!         Metadata::new()
//!             // 페이지 설정
//!             .custom(PageLayout {
//!                 sidebar: true,
//!                 toc: true,
//!                 max_width: "800px",
//!             })
//!             
//!             // 테마 오버라이드
//!             .custom(ColorTheme::Light)
//!             
//!             // 태그 (컬렉션용)
//!             .custom(Tags(vec!["rust", "tutorial"]))
//!             
//!             // 날짜 (정렬용)
//!             .custom(PublishDate(self.date))
//!     }
//! }
//! ```
//!
//! ### 메타데이터 병합 흐름
//! ```text
//! 1. Cite: 모든 Page의 metadata() 수집
//!    site_meta = Site.metadata()
//!    page1_meta = Page1.metadata()
//!    page2_meta = Page2.metadata()
//!
//! 2. 각 페이지별 병합
//!    resolved_page1 = site_meta.merge(page1_meta)
//!    resolved_page2 = site_meta.merge(page2_meta)
//!
//! 3. Block 렌더링 시 전달
//!    ctx = RenderContext { metadata: resolved_page1, ... }
//!    block.render_to_ir(&ctx)
//!
//! 4. Block이 추가 병합
//!    final = resolved_page1.merge(block.metadata())
//! ```
//!
//! ## Page 구현 패턴
//!
//! ### 기본 구조
//! ```rust
//! pub struct SimplePage {
//!     // 콘텐츠 데이터
//!     title: String,
//!     content: Vec<Box<dyn Block>>,
//!     
//!     // 메타데이터
//!     metadata: Metadata,
//!     
//!     // 출력 경로
//!     output_path: String,
//! }
//!
//! impl SimplePage {
//!     pub fn new(title: String, output_path: String) -> Self {
//!         SimplePage {
//!             title,
//!             content: vec![],
//!             metadata: Metadata::new(),
//!             output_path,
//!         }
//!     }
//!     
//!     pub fn add_block(mut self, block: Box<dyn Block>) -> Self {
//!         self.content.push(block);
//!         self
//!     }
//! }
//!
//! impl Page for SimplePage {
//!     fn layout(&self) -> IRNode {
//!         VBox::new(vec![
//!             Box::new(H1::new(
//!                 AttrBuilder::global(),
//!                 Content::from_str(&self.title, &rule)
//!             )),
//!             // content blocks를 래핑
//!         ] + self.content.clone())
//!         .to_irnode()
//!     }
//!     
//!     fn path(&self) -> &str {
//!         &self.output_path
//!     }
//!     
//!     fn metadata(&self) -> Metadata {
//!         self.metadata.clone()
//!     }
//! }
//! ```
//!
//! ### 복잡한 레이아웃
//! ```rust
//! pub struct BlogPostPage {
//!     title: String,
//!     author: String,
//!     date: Date,
//!     tags: Vec<String>,
//!     content: Vec<Box<dyn Block>>,
//! }
//!
//! impl Page for BlogPostPage {
//!     fn layout(&self) -> IRNode {
//!         VBox::new(vec![
//!             // 헤더
//!             Box::new(Header::new(vec![
//!                 Box::new(H1::new(
//!                     AttrBuilder::global().class(classes!["post-title"]),
//!                     Content::from_str(&self.title, &rule)
//!                 )),
//!                 Box::new(PostMeta::new(
//!                     &self.author,
//!                     &self.date,
//!                     &self.tags
//!                 )),
//!             ])),
//!             
//!             // 메인 콘텐츠 + 사이드바
//!             Box::new(HBox::new(vec![
//!                 // 본문
//!                 Box::new(Article::new(
//!                     AttrBuilder::global().class(classes!["post-content"]),
//!                     self.content.clone()
//!                 )),
//!                 
//!                 // 사이드바
//!                 Box::new(Aside::new(vec![
//!                     Box::new(TableOfContents::from_blocks(&self.content)),
//!                     Box::new(TagCloud::new(&self.tags)),
//!                     Box::new(ShareButtons::new()),
//!                 ])),
//!             ])
//!             .widths(vec![Width::Percent(70), Width::Percent(30)])
//!             .gap(Spacing::Large)),
//!             
//!             // 푸터
//!             Box::new(Footer::new(vec![
//!                 Box::new(RelatedPosts::new(&self.tags)),
//!                 Box::new(Comments::new()),
//!             ])),
//!         ])
//!         .to_irnode()
//!     }
//!     
//!     fn head(&self, ctx: &RenderContext) -> HeadElements {
//!         HeadElements::new()
//!             .title(&format!("{} - My Blog", self.title))
//!             .description(&self.excerpt())
//!             .canonical_url(&format!("/blog/{}", self.slug()))
//!             .stylesheet("/css/blog.css")
//!             .stylesheet("/css/syntax-highlight.css")
//!             .script("/js/toc.js")
//!             .defer(true)
//!     }
//!     
//!     fn path(&self) -> &str {
//!         &format!("blog/{}.html", self.slug())
//!     }
//! }
//! ```
//!
//! ## 구현해야 할 컴포넌트
//!
//! ### 우선순위: 높음 (레이아웃 Block)
//! - [ ] `VBox`: 수직 배치
//! - [ ] `HBox`: 수평 배치
//! - [ ] `Grid`: 그리드 배치
//! - [ ] `Spacer`: 공백
//! - [ ] `Divider`: 구분선
//!
//! ### 우선순위: 높음 (의미론적 컨테이너)
//! - [ ] `Header`: 페이지/섹션 헤더
//! - [ ] `Footer`: 페이지/섹션 푸터
//! - [ ] `Main`: 메인 콘텐츠
//! - [ ] `Aside`: 사이드바
//! - [ ] `Article`: 독립적인 콘텐츠
//! - [ ] `Section`: 주제별 섹션
//! - [ ] `Nav`: 네비게이션
//!
//! ### 우선순위: 중간 (페이지 컴포넌트)
//! - [ ] `TableOfContents`: 자동 목차
//! - [ ] `Breadcrumb`: 경로 네비게이션
//! - [ ] `Pagination`: 페이지네이션
//! - [ ] `RelatedPosts`: 관련 글 목록
//!
//! ### 우선순위: 낮음 (특수 기능)
//! - [ ] `Comments`: 댓글 시스템
//! - [ ] `ShareButtons`: 공유 버튼
//! - [ ] `SearchBox`: 검색창
//!
//! ## 설계 결정
//!
//! ### 왜 layout()은 IRNode를 반환하는가?
//! **대안 1: Vec<Block> 반환**
//! ```rust
//! fn layout(&self) -> Vec<Box<dyn Block>>
//! ```
//! 문제:
//! - 레이아웃 정보 손실 (VBox인지 HBox인지)
//! - 별도로 레이아웃 정보 전달 필요
//!
//! **대안 2: Block 반환**
//! ```rust
//! fn layout(&self) -> Box<dyn Block>
//! ```
//! 문제:
//! - 최상위 Block을 또 render_to_ir() 해야 함
//! - 불필요한 간접 참조
//!
//! **현재 방식: IRNode 반환**
//! ```rust
//! fn layout(&self) -> IRNode
//! ```
//! 장점:
//! - 레이아웃이 이미 IRNode로 구성됨
//! - 즉시 렌더링 가능
//! - 명확한 책임: Page는 완전한 구조를 제공
//!
//! ### 왜 head()는 별도 메서드인가?
//! **대안: layout()에 포함**
//! ```rust
//! fn layout(&self) -> FullHtml {
//!     FullHtml {
//!         head: /* ... */,
//!         body: /* ... */,
//!     }
//! }
//! ```
//!
//! **분리 이유:**
//! - `<head>`와 `<body>`는 독립적
//! - head는 메타데이터 기반 생성 가능
//! - layout은 콘텐츠 구조에만 집중
//! - 테스트 용이 (body만 검증 가능)
//!
//! ### 왜 레이아웃 Block이 필요한가?
//! **대안: CSS만 사용**
//! ```rust
//! Div::new(
//!     AttrBuilder::global().class(classes!["flex-container"]),
//!     children
//! )
//! ```
//!
//! **레이아웃 Block의 이점:**
//! - **명시적**: `HBox::new()` vs `Div + CSS class`
//! - **타입 안전**: 컴파일 타임에 레이아웃 검증
//! - **렌더링 유연성**: 다른 레이아웃 엔진 사용 가능
//! - **메타데이터 통합**: 레이아웃 설정도 메타데이터로 관리
//!
//! ## 사용 예시
//!
//! ### 간단한 페이지
//! ```rust
//! pub struct AboutPage {
//!     content: String,
//! }
//!
//! impl Page for AboutPage {
//!     fn layout(&self) -> IRNode {
//!         VBox::new(vec![
//!             Box::new(H1::new(
//!                 AttrBuilder::global(),
//!                 Content::from_str("About Us", &rule)
//!             )),
//!             Box::new(Paragraph::new(&self.content)),
//!         ]).to_irnode()
//!     }
//!     
//!     fn path(&self) -> &str {
//!         "about.html"
//!     }
//! }
//! ```
//!
//! ### 복잡한 페이지
//! ```rust
//! pub struct DocsPage {
//!     sections: Vec<DocSection>,
//!     sidebar_items: Vec<NavItem>,
//! }
//!
//! impl Page for DocsPage {
//!     fn layout(&self) -> IRNode {
//!         VBox::new(vec![
//!             // 상단 네비게이션
//!             Box::new(Nav::new(/* ... */)),
//!             
//!             // 메인 레이아웃 (3단 구성)
//!             Box::new(HBox::new(vec![
//!                 // 왼쪽 사이드바 (20%)
//!                 Box::new(Aside::new(
//!                     self.sidebar_items.iter()
//!                         .map(|item| Box::new(NavLink::new(item)))
//!                         .collect()
//!                 )),
//!                 
//!                 // 메인 콘텐츠 (60%)
//!                 Box::new(Main::new(
//!                     self.sections.iter()
//!                         .map(|sec| Box::new(sec.to_block()))
//!                         .collect()
//!                 )),
//!                 
//!                 // 오른쪽 목차 (20%)
//!                 Box::new(Aside::new(vec![
//!                     Box::new(TableOfContents::auto()),
//!                 ])),
//!             ])
//!             .widths(vec![
//!                 Width::Percent(20),
//!                 Width::Percent(60),
//!                 Width::Percent(20),
//!             ])),
//!         ]).to_irnode()
//!     }
//!     
//!     fn head(&self, ctx: &RenderContext) -> HeadElements {
//!         HeadElements::new()
//!             .title("Documentation")
//!             .stylesheet("/css/docs.css")
//!             .script("/js/search.js")
//!     }
//!     
//!     fn path(&self) -> &str {
//!         "docs/index.html"
//!     }
//! }
//! ```
//!
//! ## 향후 방향
//!
//! ### 단기 (Phase 1)
//! - 레이아웃 Block 구현 (VBox, HBox, Grid)
//! - 의미론적 컨테이너 (Header, Main, Aside 등)
//! - HeadElements 구조체 및 렌더링
//!
//! ### 중기 (Phase 2)
//! - TableOfContents 자동 생성
//! - Breadcrumb, Pagination
//! - 반응형 레이아웃 (모바일 대응)
//!
//! ### 장기 (Phase 3)
//! - 페이지 템플릿 매크로
//! - 컴포넌트 재사용 시스템
//! - 동적 레이아웃 (클라이언트 사이드)
//!
//! ## 참고 자료
//! - [HTML5 Semantic Elements](https://html.spec.whatwg.org/multipage/sections.html)
//! - [CSS Flexbox](https://css-tricks.com/snippets/css/a-guide-to-flexbox/)
//! - [CSS Grid](https://css-tricks.com/snippets/css/complete-guide-grid/)

pub mod page;
