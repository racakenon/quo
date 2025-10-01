# Quo 아키텍처 문서

## 목차

1. [핵심 철학](#1-핵심-철학)
2. [계층 구조](#2-계층-구조)
3. [핵심 메커니즘](#3-핵심-메커니즘)
4. [설계 결정과 근거](#4-설계-결정과-근거)
5. [구현 상태](#5-구현-상태)
6. [다음 단계](#6-다음-단계)

---

## 1. 핵심 철학

### "컴파일에 성공하면, 사이트는 정상적으로 동작한다"

Quo는 Rust의 강력한 타입 시스템을 활용하여 웹 개발의 안정성을 근본적으로 개선합니다. 런타임에 발견되는 흔한 오류들을 컴파일 타임에 포착합니다:

- ❌ 깨진 링크 (404 에러)
- ❌ 잘못된 HTML 구조
- ❌ 문서 손상 (의도하지 않은 태그 생성)
- ❌ 타입 불일치 (메타데이터 접근 오류)

### 프로그래밍 우선 콘텐츠 (Programming-First Content)

**"코드가 곧 콘텐츠"**

Markdown이나 별도 파일 대신, Rust 코드로 직접 콘텐츠를 작성합니다. 이는 다음을 가능하게 합니다:

- **컴파일 타임 검증**: 모든 콘텐츠가 `struct`와 `enum`으로 표현되어 구조적 완결성 보장
- **프로그래밍 기능 활용**: `for` 루프, `if`문, 함수를 통한 동적 콘텐츠 생성
- **완벽한 개발 도구 지원**: LSP를 통한 자동완성, 타입 검증, 리팩토링


## 2. 계층 구조

Quo는 4개의 명확히 분리된 계층으로 구성됩니다. 각 계층은 단일 책임을 가지며, 상위 계층은 하위 계층에만 의존합니다.

```
┌─────────────────────────────────────────┐
│  Cite 계층 - 사이트 전체 빌드 오케스트레이션  │
│  (전역 객체, 유일한 인스턴스)              │
└────────────────┬────────────────────────┘
                 │ 관리
                 ↓
┌─────────────────────────────────────────┐
│  Page 계층 - 완전한 HTML 페이지 생성       │
│  (레이아웃 + Block 조합)                  │
└────────────────┬────────────────────────┘
                 │ 조합
                 ↓
┌─────────────────────────────────────────┐
│  Block 계층 - 의미론적 콘텐츠 단위         │
│  (CodeBlock, MathBlock, ...)           │
└────────────────┬────────────────────────┘
                 │ 변환
                 ↓
┌─────────────────────────────────────────┐
│  HTML 계층 - 타입 안전 HTML 프리미티브     │
│  (div, span, h1, ...)                  │
└─────────────────────────────────────────┘
```

### 2.1 HTML 계층 

**목적:** 사용자가 의도한 대로 문서가 렌더링되도록 보장

**핵심 원칙:** 문서 구조 손상 방지 (보안은 별도 도구의 책임)

#### 주요 모듈

**`trust.rs` - 신뢰 경계 (Trust Boundary)**

신뢰 수준을 타입으로 표현하여 컴파일 타임에 안전성을 보장합니다.

| 타입 | 신뢰 수준 | 가시성 | 처리 방식 |
|------|---------|--------|----------|
| `Content` | 비신뢰 (사용자 입력) | `pub` | HTML 이스케이프 |
| `AttrValue` | 비신뢰 (사용자 입력) | `pub` | HTML 이스케이프 |
| `AttrKey` | 신뢰 (라이브러리 내부) | `pub(crate)` | 검증 없음 |
| `TagName` | 신뢰 (라이브러리 내부) | `pub(crate)` | 검증 없음 |
| `HtmlBlock` | 신뢰 (외부 도구) | `pub` | 검증 없음 |

```rust
// 사용자 입력 → 이스케이프
let text = Content::from_str("x > 0", &rule);
// → "x &gt; 0" (화면에는 "x > 0"으로 표시)

// 외부 도구 출력 → 신뢰
let svg = mermaid::render(diagram);
let block = HtmlBlock::from_str(&svg);  // 그대로 사용
```

**`rules.rs` - 타이포그래피 정규화**

사용자 입력을 표준화하여 일관된 문서 품질을 보장합니다.

- 모호한 문자 치환 (유니코드 정규화)
- 보이지 않는 문자 제거
- 스마트 쿼트 변환 (`"` → `""`), (`'` → `''`)
- 로케일별 처리 (`ambiguous.json`, `invisibleCharacters.json`)

**`node.rs` - 공통 중간 표현 (IRNode)**

모든 HTML 구조를 통일된 형식으로 표현하여 성능과 타입 안전성을 동시에 확보합니다.

```rust
pub enum Element {
    Text(Content),      // 텍스트 노드
    Node(IRNode),       // 중첩된 요소
    Raw(HtmlBlock),     // 신뢰된 HTML
}

pub struct IRNode {
    tag: TagName,
    attrs: SharedAttrs,
    tagtype: ElementType,  // Void | Normal
    childs: Vec<Element>,
}
```

**설계 근거:**
- `Vec<Box<dyn Block>>` 대신 `Vec<IRNode>` 사용
- 트레이트 객체 회피 → 컴파일 타임 크기 결정
- 힙 할당 최소화 → 메모리 효율성
- vtable 간접 참조 제거 → 성능 향상

**`renderer.rs` - HTML 문자열 생성**

불변 Visitor 패턴으로 IRNode 트리를 순회하며 HTML을 생성합니다.

```rust
impl Renderer for HtmlRenderer {
    fn visit_node_begin(&self, node: &IRNode) -> Self;
    fn visit_node_end(&self, node: &IRNode) -> Self;
    fn visit_text(&self, content: &Content) -> Self;
    fn visit_raw(&self, html: &HtmlBlock) -> Self;
}
```

**`attributes.rs` - 타입 안전 속성 관리**

PhantomData를 활용하여 요소별로 허용된 속성만 설정 가능하도록 강제합니다.

```rust
// ✅ img 요소는 src 속성 가능
AttrBuilder::image().src(url).alt(text)

// ❌ div 요소는 src 속성 불가 (컴파일 에러)
AttrBuilder::global().src(url)  // 컴파일 실패
```

**`elements.rs` - 타입 안전 HTML 요소**

각 HTML 요소를 별도 타입으로 구현하여 구조적 정확성을 보장합니다.

- 구현 완료: `H1`, `H2`, `Div`, `Img`
- TODO: 나머지 HTML5 요소

### 2.2 Block 계층 (트레이트만 정의됨)

**목적:** HTML 요소를 조합한 의미론적 콘텐츠 단위

**설계 원칙:** 사용자는 HTML 세부사항이 아닌 **의미**로 사고합니다.

```rust
pub trait Block {
    fn render_to_ir(&self, ctx: &RenderContext) -> IRNode;
}
```

#### 구현 예정 블록

```rust
// 코드 블록 (구문 강조, treesitter, highlightjs)
pub struct CodeBlock {
    language: String,
    content: String,
    show_line_numbers: bool,
}

// 수식 블록 (KaTeX/MathJax/Typst)
pub struct MathBlock {
    content: String,
    display_mode: DisplayMode,  // Inline | Block
}

// 다이어그램 블록 (Mermaid/Graphviz)
pub struct DiagramBlock {
    engine: DiagramEngine,
    source: String,
}

// 콜아웃 블록 (Note, Warning, Tip)
pub struct CalloutBlock {
    kind: CalloutKind,
    title: Option<String>,
    content: Vec<Box<dyn Block>>,
}
```


### 2.3 Page 계층 (트레이트만 정의됨)

**목적:** Block들을 조합하여 완전한 HTML 페이지 생성

**핵심 개념:** 레이아웃 트리 (Layout Tree)

Page는 특수 레이아웃 Block들(`HBox`, `VBox`, `Grid`)과 사용자 정의 Block들을 조합한 트리 구조로 페이지 레이아웃을 정의합니다.

```rust
pub trait Page {
    fn layout(&self) -> IRNode;
    fn metadata(&self) -> Metadata;
}
```

#### 레이아웃 구조 예시

```rust
pub struct BlogPostPage {
    title: String,
    content: Vec<Box<dyn Block>>,
}

impl Page for BlogPostPage {
    fn layout(&self) -> IRNode {
        VBox::new(vec![
            // 헤더
            Box::new(Header::new(vec![
                Box::new(H1::new(
                    AttrBuilder::global(),
                    Content::from_str(&self.title, &rule)
                ))
            ])),
            
            // 메인 콘텐츠 (2컬럼)
            Box::new(HBox::new(vec![
                // 왼쪽: 본문
                Box::new(Main::new(
                    self.content.clone()
                )),
                
                // 오른쪽: 사이드바
                Box::new(Aside::new(vec![
                    Box::new(TableOfContents::new()),
                    Box::new(RelatedPosts::new()),
                ]))
            ])),
            
            // 푸터
            Box::new(Footer::new(/* ... */))
        ]).to_irnode()
    }
}
```

**템플릿 공유:**

동일한 구조체 타입의 인스턴스들은 `layout()` 메서드를 공유하므로, 자동으로 동일한 레이아웃을 사용합니다.

```rust
// 모든 BlogPostPage 인스턴스는 동일한 레이아웃 사용
let post1 = BlogPostPage { title: "Post 1", content: /* ... */ };
let post2 = BlogPostPage { title: "Post 2", content: /* ... */ };
// → 둘 다 동일한 2컬럼 레이아웃 적용
```

### 2.4 Cite 계층 (트레이트만 정의됨)

**목적:** 사이트 전체 빌드를 오케스트레이션하는 중앙 집중 시스템

**핵심:** 프로그램 전체에 유일하게 존재하는 전역 객체

```rust
pub struct Site {
    pages: Vec<Box<dyn Page>>,
    visitors: Vec<Box<dyn Visitor>>,
    global_metadata: Metadata,
}
```

#### 주요 책임

1. **페이지 관리**
   - 모든 Page 등록 및 관리
   - Block 트리 수집

2. **방문자 관리**
   - 분석/렌더링 방문자 등록
   - 방문자 파이프라인 실행

3. **전역 메타데이터**
   - 사이트 전역 설정 저장
   - 모든 페이지에서 접근 가능

4. **사이트 전역 기능**
   - **페이지 간 상호 참조**: 링크, 백링크
   - **컬렉션 생성**: 태그별 문서 모음, 최근 수정 문서 목록
   - **의존성 관리**: 페이지 간 링크로 인한 의존성 추적
   - **전역 문서 생성**: 사이트맵, RSS 피드, 검색 인덱스

#### 사용 예시

```rust
fn main() {
    let mut site = Site::new()
        .metadata(
            Metadata::new()
                .site_name("My Blog")
                .base_url("https://example.com")
        );
    
    // 방문자 등록
    site.register_visitor(Box::new(MetadataCollector::new()));
    site.register_visitor(Box::new(IdGenerator::new()));
    site.register_visitor(Box::new(Counter::new()));
    site.register_visitor(Box::new(HtmlRenderer::new()));
    
    // 페이지 등록 (수동, 향후 자동화 예정)
    site.register_page(Box::new(HomePage::new()));
    site.register_page(Box::new(AboutPage::new()));
    site.register_page(Box::new(BlogPostPage::new("Post 1", /* ... */)));
    site.register_page(Box::new(BlogPostPage::new("Post 2", /* ... */)));
    
    // 빌드 실행
    site.build("./dist")?;
}
```

---

## 3. 핵심 메커니즘

### 3.1 택배 메시지 모델 (메타데이터 시스템)

계층적 설정 관리를 위한 타입 안전 시스템입니다.

#### 문제 정의

설정이 여러 계층에 정의될 때 어떤 값을 사용할지 명확히 결정해야 합니다:

```rust
// Site 레벨
site.metadata().theme(ThemeSettings::light());

// Page 레벨 (특정 페이지만 다크 모드)
page.metadata().theme(ThemeSettings::dark());

// Block 레벨 (특정 블록만 고대비)
block.metadata().theme(ThemeSettings::high_contrast());

// 렌더링 시 어떤 값을 사용?
```

#### 해법: TypeId 기반 저장소

```rust
pub struct MetadataMap {
    data: HashMap<TypeId, Box<dyn Any>>,
    #[cfg(debug_assertions)]
    type_names: Vec<&'static str>,  // 디버깅용
}

impl MetadataMap {
    pub fn insert<T: MetadataValue>(&mut self, value: T) {
        self.data.insert(TypeId::of::<T>(), Box::new(value));
    }
    
    pub fn get<T: MetadataValue>(&self) -> Option<&T> {
        self.data
            .get(&TypeId::of::<T>())
            .and_then(|v| v.downcast_ref::<T>())
    }
}
```

#### 병합 규칙: "가까운 곳이 우선"

```rust
// 우선순위: Block > Page > Site
let resolved = site_metadata
    .merge(&page_metadata)
    .merge(&block_metadata);

// Block에서 정의하지 않은 설정은 상위에서 상속
resolved.get::<ThemeSettings>()  // Block 설정 사용
resolved.get::<FontFamily>()     // Page 설정 사용 (Block에 없음)
resolved.get::<SiteUrl>()        // Site 설정 사용 (Page에도 없음)
```

#### 사용자 확장 패턴

**1단계: 타입 정의 및 검증**

```rust
pub struct FgOklab {
    l: f32,  // lightness [0.0, 1.0]
    a: f32,  // green-red [-0.4, 0.4]
    b: f32,  // blue-yellow [-0.4, 0.4]
}

impl FgOklab {
    pub fn new(l: f32, a: f32, b: f32) -> Result<Self, ColorError> {
        // 생성 시점에 한 번만 검증
        if !(0.0..=1.0).contains(&l) {
            return Err(ColorError::InvalidLightness(l));
        }
        // ... 나머지 검증
        Ok(FgOklab { l, a, b })
    }
    
    pub fn to_css(&self) -> String {
        format!("oklab({} {} {})", self.l, self.a, self.b)
    }
}
```

**2단계: MetadataValue 구현**

```rust
impl MetadataValue for FgOklab {}
```

**3단계: 사용**

```rust
// 메타데이터 설정
let metadata = Metadata::new()
    .custom(FgOklab::new(0.7, 0.1, -0.05)?);

// 렌더링 시 활용
impl Block for MyBlock {
    fn render_to_ir(&self, ctx: &RenderContext) -> IRNode {
        let fg = ctx.metadata.get::<FgOklab>();
        let style = format!("color: {};", fg.to_css());
        // → "color: oklab(0.7 0.1 -0.05);"
    }
}
```

### 3.2 다단계 방문자 파이프라인

복잡한 기능(상호 참조, 자동 번호, 목차 생성)을 안전하게 구현하기 위한 다단계 빌드 프로세스입니다.

#### 실행 흐름

```
1. 사용자: 방문자 등록
   site.register_visitor(MetadataCollector);
   site.register_visitor(IdGenerator);
   site.register_visitor(HtmlRenderer);

2. 사용자: 페이지 등록
   site.register_page(page1);
   site.register_page(page2);

3. Cite: 빌드 시작
   site.build() 호출

4. Cite: 각 페이지에서 Block 트리 수집
   page1.layout() → IRNode (렌더링 전 구조)
   page2.layout() → IRNode

5. Cite: 등록된 방문자를 순서대로 실행
   for visitor in visitors:
       visitor.visit_site(&site)
       for page in pages:
           visitor.visit_page(&page)
           for block in blocks:
               visitor.visit_block(&block)

6. Cite: 각 방문자가 컨텍스트 수집/처리
   MetadataCollector → 전체 메타데이터 맵 생성
   IdGenerator → 모든 블록에 고유 ID 부여
   HtmlRenderer → 최종 HTML 파일 생성
```

#### Phase 1: 분석 단계 (Analysis Pass)

렌더링에 필요한 모든 정보를 미리 수집하여 **사이트 인덱스(Site Index)**를 구축합니다.

**MetadataCollector 방문자**

```rust
pub struct MetadataCollector {
    site_metadata: MetadataMap,
    page_metadata: HashMap<PageId, MetadataMap>,
    resolved_metadata: HashMap<BlockId, MetadataMap>,
}

impl Visitor for MetadataCollector {
    fn visit_block(&mut self, block: &dyn Block, page_id: PageId) {
        let block_meta = block.metadata();
        let page_meta = &self.page_metadata[&page_id];
        let site_meta = &self.site_metadata;
        
        // 병합: Block > Page > Site
        let resolved = site_meta
            .merge(page_meta)
            .merge(&block_meta);
        
        self.resolved_metadata.insert(block.id(), resolved);
    }
}
```

**IdGenerator 방문자**

모든 블록에 안정적인 고유 ID를 부여합니다.

```rust
pub struct IdGenerator {
    id_map: HashMap<BlockPath, BlockId>,
}

impl IdGenerator {
    fn generate_id(&mut self, block: &dyn Block, path: &BlockPath) -> BlockId {
        // 사용자 지정 ID가 있으면 사용
        if let Some(user_id) = block.user_id() {
            return BlockId::from(format!("{}/{}", path, user_id));
        }
        
        // 없으면 경로 기반 ID 생성
        BlockId::from(format!("{}/block-{}", path, self.counter))
    }
}
```

**Counter 방문자**

figure, footnote 등 자동 번호 매기기가 필요한 요소를 식별합니다.

```rust
pub struct Counter {
    figure_count: HashMap<PageId, usize>,
    footnote_count: HashMap<PageId, usize>,
}
```

#### Phase 2: 렌더링 단계 (Rendering Pass)

분석 단계에서 구축한 사이트 인덱스를 활용하여 최종 HTML을 생성합니다.

```rust
pub struct HtmlRenderer {
    site_index: SiteIndex,  // 분석 단계 결과
    output_dir: PathBuf,
}

impl Visitor for HtmlRenderer {
    fn visit_page(&mut self, page: &dyn Page) {
        // 1. RenderContext 생성 (사이트 인덱스 포함)
        let ctx = RenderContext {
            metadata: self.site_index.get_resolved_metadata(page.id()),
            block_ids: &self.site_index.block_ids,
            links: &self.site_index.links,
        };
        
        // 2. Page의 레이아웃 트리 획득
        let layout_tree = page.layout();
        
        // 3. 각 Block을 IRNode로 변환
        let ir_tree = self.render_tree(&layout_tree, &ctx);
        
        // 4. IRNode → HTML 문자열
        let html = self.ir_to_html(&ir_tree);
        
        // 5. 파일 저장
        self.write_file(page.path(), &html)?;
    }
}
```

#### 사이트 인덱스 (Site Index) 구조

```rust
pub struct SiteIndex {
    // 메타데이터
    resolved_metadata: HashMap<BlockId, MetadataMap>,
    
    // 식별자
    block_ids: HashMap<BlockPath, BlockId>,
    page_ids: HashMap<PagePath, PageId>,
    
    // 링크와 관계
    links: HashMap<PageId, Vec<Link>>,
    backlinks: HashMap<PageId, Vec<PageId>>,
    
    // 컬렉션
    tags: HashMap<Tag, Vec<PageId>>,
    recent_pages: Vec<PageId>,
    
    // 자동 번호
    figure_numbers: HashMap<BlockId, usize>,
    footnote_numbers: HashMap<BlockId, usize>,
}
```

## 4. 설계 결정과 근거

### 4.1 왜 IRNode인가?

**문제:** Rust에서 `Vec<Box<dyn Block>>`은 성능 비용이 큼

```rust
// ❌ 트레이트 객체 사용 시
pub struct Page {
    blocks: Vec<Box<dyn Block>>  // 각 요소마다 힙 할당
}
// → vtable 간접 참조
// → 메모리 단편화
// → 캐시 미스
```

**해법:** 공통 중간 표현 (IRNode)

```rust
// ✅ IRNode 사용 시
pub struct Page {
    blocks: Vec<IRNode>  // 고정 크기, 연속 메모리
}

pub enum Element {
    Text(Content),    // 고정 크기
    Node(IRNode),     // 재귀 (Box 필요하지만 최소화)
    Raw(HtmlBlock),   // 고정 크기
}
```

**효과:**
- Zero-cost abstraction 달성
- 메모리 레이아웃 최적화
- 컴파일 타임에 크기 결정 가능

### 4.2 왜 계층을 분리하는가?

**HTML vs Block 분리**

```rust
// 사용자 관점 (Block 계층)
CodeBlock::new()
    .language("rust")
    .content("fn main() {}")

// 라이브러리 관점 (HTML 계층)
Pre::new(
    Code::new(
        HtmlBlock::from_str(highlighted_html)
    )
)
```

**효과:**
- 사용자: 의미론적 단위로 사고
- 라이브러리: HTML 세부사항 변경 자유
- 관심사 명확히 분리

### 4.3 왜 TypeId 메타데이터인가?

**대안 1: 문자열 키**
```rust
metadata.insert("theme", theme);  // 오타 가능
metadata.get("tehme")  // 런타임 오류
```

**대안 2: Enum**
```rust
enum MetadataKey {
    Theme,
    Layout,
    // 사용자 확장 불가!
}
```

**TypeId 방식**
```rust
metadata.insert(ThemeSettings::dark());  // 오타 불가능
metadata.get::<ThemeSettings>()  // 컴파일 타임 검증

struct MyCustomData { /* ... */ }
impl MetadataValue for MyCustomData {}  // 사용자 확장 가능
```

### 4.4 왜 Cite는 전역 객체인가?

**필요성:**

페이지 간 상호 참조와 전역 기능을 구현하기 위해서는 **모든 페이지의 정보를 한 곳에서 관리**해야 합니다.

```rust
// ❌ 분산된 구조라면
let page1 = Page::new();
let page2 = Page::new();

// 페이지 간 링크를 어떻게 검증?
page1.link_to(page2)?  // page2가 존재하는지 어떻게 알 수 있나?

// 백링크를 어떻게 생성?
page2.get_backlinks()  // 어떤 페이지들이 page2를 링크하는지?

// 태그로 문서를 어떻게 모으나?
get_pages_with_tag("rust")  // 전역 레지스트리 없이 불가능
```

```rust
// ✅ 중앙 집중 구조
let mut site = Site::new();
site.register_page(page1);
site.register_page(page2);

// 사이트가 모든 페이지를 알고 있음
// → 링크 검증 가능
// → 백링크 생성 가능
// → 컬렉션 생성 가능
```

**구현하는 전역 기능:**

1. **상호 참조 검증**
   ```rust
   // 컴파일 타임에 깨진 링크 감지
   page1.link_to("page2")?  // page2가 존재하지 않으면 빌드 실패
   ```

2. **백링크 생성**
   ```rust
   // page2를 참조하는 모든 페이지 자동 수집
   site.get_backlinks("page2")  // [page1, page3, page5]
   ```

3. **컬렉션 (Collection)**
   ```rust
   // 태그별 문서 모음
   site.get_pages_by_tag("rust")
   
   // 최근 수정 문서
   site.get_recent_pages(10)
   
   // 카테고리별 문서
   site.get_pages_by_category("blog")
   ```

4. **사이트맵 생성**
   ```rust
   site.generate_sitemap("./dist/sitemap.xml")?
   ```

5. **RSS 피드 생성**
   ```rust
   site.generate_rss_feed("./dist/feed.xml")?
   ```

6. **검색 인덱스**
   ```rust
   site.generate_search_index("./dist/search.json")?
   ```

**설계 트레이드오프:**

| 장점 | 단점 |
|------|------|
| ✅ 전역 기능 구현 가능 | ❌ 테스트 시 전역 상태 관리 필요 |
| ✅ 페이지 간 일관성 보장 | ❌ 병렬 빌드 시 동기화 고려 필요 |
| ✅ 링크 검증 완벽 | ❌ 대규모 사이트(10,000+ 페이지)에서 메모리 사용량 증가 |

하지만 정적 사이트 생성기의 특성상 **빌드는 한 번만 실행**되므로, 전역 객체의 단점이 크게 문제되지 않습니다.

### 4.5 왜 불변 렌더러인가?

**문제:** 상태를 변경하는 렌더러는 예측하기 어렵고 디버깅이 어렵습니다.

```rust
// ❌ 가변 렌더러
let mut renderer = HtmlRenderer::new();
renderer.visit_node(node1);  // 내부 상태 변경
renderer.visit_node(node2);  // 이전 상태에 영향받음
// → 방문 순서에 따라 결과가 달라질 수 있음
```

**해법:** 불변 렌더러 (함수형 스타일)

```rust
// ✅ 불변 렌더러
let r0 = HtmlRenderer::new();
let r1 = r0.visit_node(node1);  // 새 렌더러 반환
let r2 = r1.visit_node(node2);  // 새 렌더러 반환
// → 각 단계가 독립적
// → 예측 가능
```

**효과:**
- 디버깅 용이 (각 단계의 중간 결과 확인 가능)
- 테스트 작성 쉬움 (부작용 없음)
- 병렬화 가능성 (향후)

**성능 고려사항:**
- 매 단계마다 문자열 복사 발생
- 현재는 단순성 우선 (조기 최적화 방지)
- 필요시 향후 버퍼 재사용으로 최적화 가능

### 4.6 왜 보안은 별도 도구의 책임인가?

**철학:** Unix 철학 - "한 가지 일을 잘하라"

```
Quo의 책임:
  ✅ 사용자 의도대로 문서 생성
  ✅ 구조적 완결성 보장
  ✅ 타입 안전성

보안 도구의 책임:
  ✅ XSS 취약점 검사
  ✅ CSP 검증
  ✅ HTML 구조 검증
```

**실용적 빌드 파이프라인:**

```bash
# 1. Quo로 사이트 생성
cargo run --release

# 2. HTML 구조 검증
html-validate dist/

# 3. 보안 스캔
npm run security-scan

# 4. 성능 검사
lighthouse dist/

# 5. 배포
./deploy.sh
```

**이점:**
- 각 도구가 전문 영역에 집중
- 도구 조합 자유 (사용자가 선택)
- Quo는 복잡도 증가 방지


## 7. 참고 자료

### 하위 문서

상세한 구현 내용은 각 모듈별 문서를 참조하세요:

- [HTML 계층 문서](src/html/README.md)
- [Block 계층 문서](src/block/README.md)
- [Page 계층 문서](src/page/README.md)
- [Cite 계층 문서](src/cite/README.md)

---

## 8. 용어 사전

| 용어 | 정의 |
|------|------|
| **IRNode** | Intermediate Representation Node - 모든 HTML 구조의 공통 중간 표현 |
| **Trust Boundary** | 신뢰 경계 - 검증 필요한 입력과 신뢰 가능한 입력을 구분하는 경계 |
| **Visitor** | 방문자 패턴 - 사이트 트리를 순회하며 특정 작업을 수행하는 객체 |
| **Site Index** | 사이트 인덱스 - 분석 단계에서 수집한 모든 페이지/블록 정보의 집합 |
| **RenderContext** | 렌더링 컨텍스트 - 렌더링 시 필요한 메타데이터와 사이트 인덱스를 포함한 객체 |
| **Metadata** | 메타데이터 - 설정, 테마, CSS 변수 등 렌더링에 영향을 주는 모든 정보 |
| **Layout Tree** | 레이아웃 트리 - Page가 정의하는 Block들의 배치 구조 |
| **Content Category** | 콘텐츠 카테고리 - HTML5 명세의 요소 분류 (Flow, Phrasing, Embedded 등) |


