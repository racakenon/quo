//! # metadata - 타입 안전 메타데이터 시스템 (택배 메시지 모델)
//!
//! ## 모듈의 목적
//! 계층적 설정 관리를 위한 타입 안전 메타데이터 시스템을 제공합니다.
//! TypeId 기반 저장소로 사용자가 무한히 확장 가능하며, 컴파일 타임 타입 검증을 보장합니다.
//!
//! ## 설계 철학
//!
//! ### 1. 택배 메시지 모델 (Courier Message Model)
//! 계층적 설정을 직관적인 은유로 관리합니다.
//!
//! **화물 (Cargo):**
//! - 개별 설정값: `theme: "dark"`, `color: "#3b82f6"`
//! - 사용자 정의 타입으로 표현: `struct ThemeSettings { mode: DarkMode }`
//!
//! **유형 (Type):**
//! - TypeId로 화물 분류
//! - 타입 자체가 고유한 키
//! - 네임스페이스 자동 분리
//!
//! **수거차 (Courier):**
//! - 빌드 시점에 계층 순회: Site → Page → Block
//! - 우선순위 규칙: "가까운 곳이 우선"
//! - `metadata.merge()` 로직으로 구현
//!
//! **중앙 처리소 (Hub):**
//! - 최종 병합 결과: `ResolvedMetadata`
//! - 렌더링 시 `RenderContext`에 포함
//! - 타입 안전 접근: `get::<ThemeSettings>()`
//!
//! ### 2. 무한 확장성 (Unlimited Extensibility)
//! 라이브러리는 기본 세트만 제공, 사용자가 필요한 만큼 확장합니다.
//!
//! **라이브러리 제공:**
//! - `MetadataValue` 트레이트
//! - `MetadataMap` 저장소
//! - `Metadata` 빌더
//!
//! **사용자 확장:**
//! - 자신의 구조체 정의
//! - `MetadataValue` 구현
//! - `to_css()` 등 변환 메서드 추가
//!
//! ### 3. 타입 안전성 (Type Safety)
//! 모든 메타데이터 접근을 컴파일 타임에 검증합니다.
//!
//! **컴파일 타임 검증:**
//! - 타입 불일치 방지
//! - 존재하지 않는 키 접근 시 `None` 반환 (런타임 에러 없음)
//! - IDE 자동완성 지원
//!
//! ## 핵심 구조
//!
//! ### MetadataMap
//! TypeId를 키로 사용하는 타입 안전 저장소입니다.
//!
//! **구조:**
//! - `data: HashMap<TypeId, Box<dyn Any>>`
//! - 디버그용: `type_names: Vec<&'static str>` (debug_assertions)
//!
//! **메서드:**
//! - `insert<T: MetadataValue>(value: T)`: 타입 안전 삽입
//! - `get<T: MetadataValue>() -> Option<&T>`: 타입 안전 접근
//! - `merge(other: &MetadataMap) -> MetadataMap`: 병합 (덮어쓰기)
//!
//! ### MetadataValue 트레이트
//! 메타데이터로 사용 가능한 타입의 마커 트레이트입니다.
//!
//! **요구사항:**
//! - `Clone`: 병합 시 복사 필요
//! - `'static`: TypeId 사용을 위해
//!
//! **구현 예시:**
//! - 라이브러리: `ThemeSettings`, `LayoutConfig`
//! - 사용자: `FgOklab`, `CustomColors`, `MyAnalytics`
//!
//! ### Metadata (빌더)
//! 사용자 친화적인 메타데이터 생성 인터페이스입니다.
//!
//! **메서드:**
//! - `new()`: 빈 메타데이터 생성
//! - 라이브러리 제공: `theme()`, `layout()` 등
//! - 사용자 확장: `custom<T>(value: T)`
//!
//! ## 계층 관계
//!
//! ### HTML 계층과의 독립성
//! ```text
//! metadata 모듈 ←──── Block 계층
//!                 ↖─── Page 계층
//!                  ↖── Cite 계층
//!
//! (HTML 계층은 metadata를 모름)
//! ```
//!
//! **이유:**
//! - HTML은 메타데이터 불필요 (순수 표현 계층)
//! - 메타데이터는 Block 이상에서만 사용
//! - 명확한 계층 분리
//!
//! ### 상위 계층에서의 사용
//!
//! **Block 계층:**
//! - `Block::metadata()`: 블록 수준 메타데이터 제공
//! - `Block::render_to_ir(ctx)`: 메타데이터 소비
//!
//! **Page 계층:**
//! - `Page::metadata()`: 페이지 수준 메타데이터 제공
//!
//! **Cite 계층:**
//! - `Site::metadata()`: 사이트 전역 메타데이터 제공
//! - `MetadataCollector`: 모든 계층 수집 및 병합
//!
//! ## 데이터 흐름
//!
//! ### 1. 수집 단계 (Collection)
//! ```text
//! Site
//!   ├─ metadata() → SiteMetadata
//!   │
//!   ├─ Page1
//!   │  ├─ metadata() → Page1Metadata
//!   │  └─ Block1
//!   │     └─ metadata() → Block1Metadata
//!   │
//!   └─ Page2
//!      └─ metadata() → Page2Metadata
//! ```
//!
//! ### 2. 병합 단계 (Merge)
//! ```text
//! Cite 계층의 MetadataCollector 방문자가 수행:
//!
//! Block1의 최종 메타데이터 =
//!   SiteMetadata
//!     .merge(Page1Metadata)    // Page 설정이 Site 덮어쓰기
//!     .merge(Block1Metadata)   // Block 설정이 Page 덮어쓰기
//! ```
//!
//! ### 3. 전달 단계 (Delivery)
//! ```text
//! RenderContext {
//!   metadata: ResolvedMetadata,  // 병합 완료된 메타데이터
//!   ...
//! }
//!   ↓
//! Block::render_to_ir(ctx)
//!   ↓
//! let theme = ctx.metadata.get::<ThemeSettings>();
//! ```
//!
//! ## 병합 규칙
//!
//! ### 기본 병합: 완전 덮어쓰기
//! ```text
//! Site:  { theme: Light, color: Blue }
//! Page:  { theme: Dark }
//! 결과:  { theme: Dark, color: Blue }
//!        ↑ Page의 theme이 Site 덮어씀
//!        ↑ color는 Site에서 상속
//! ```
//!
//! ### 특수 병합: MergeableMetadata (선택적)
//! 사용자가 부분 병합이 필요한 경우 구현할 수 있는 트레이트입니다.
//!
//! **트레이트:**
//! - `merge_with(&self, base: &Self) -> Self`
//!
//! **사용 예시:**
//! - CSS 변수: 일부만 오버라이드
//! - 컬렉션: 추가 병합 (덮어쓰기 아님)
//!
//! **기본 동작 유지:**
//! - 구현하지 않으면 완전 덮어쓰기
//! - 필요한 타입만 선택적 구현
//!
//! ## 사용자 확장 패턴
//!
//! ### 1단계: 타입 정의
//! 사용자 정의 구조체를 만들고 필요한 필드를 추가합니다.
//!
//! **예시 목적:**
//! - CSS 색상: Oklab 색공간 사용
//! - 검증: 생성자에서 값 범위 체크
//! - 변환: CSS 문자열로 변환
//!
//! ### 2단계: MetadataValue 구현
//! 마커 트레이트를 구현하여 메타데이터로 사용 가능하게 만듭니다.
//!
//! ### 3단계: 변환 메서드
//! 메타데이터를 실제 사용 형식으로 변환하는 메서드를 추가합니다.
//!
//! **일반적 패턴:**
//! - `to_css()`: CSS 문자열 생성
//! - `to_json()`: JSON 직렬화
//! - `to_html_attr()`: HTML 속성값 생성
//!
//! ### 4단계: 사용
//! 빌더 패턴으로 메타데이터 설정 및 렌더링 시 소비합니다.
//!
//! ## 모듈 구조
//!
//! ```text
//! metadata/
//! ├─ mod.rs           - 모듈 진입점 (이 파일)
//! ├─ map.rs           - MetadataMap 구현
//! ├─ value.rs         - MetadataValue 트레이트
//! ├─ builder.rs       - Metadata 빌더 패턴
//! ├─ merge.rs         - 병합 로직, MergeableMetadata
//! └─ prelude.rs       - 자주 쓰는 타입 재수출
//! ```
//!
//! ## 구현 상태
//!
//! ### 우선순위: 높음 (필수)
//! - [ ] MetadataMap 구조체 및 기본 메서드
//! - [ ] MetadataValue 트레이트
//! - [ ] Metadata 빌더
//! - [ ] 기본 병합 로직
//!
//! ### 우선순위: 중간 (중요)
//! - [ ] MergeableMetadata 트레이트 (선택적 부분 병합)
//! - [ ] 디버그 헬퍼 (debug_dump, type_names)
//! - [ ] 라이브러리 기본 메타데이터 타입 (Theme, Layout 등)
//!
//! ### 우선순위: 낮음 (향상)
//! - [ ] 메타데이터 검증
//! - [ ] 직렬화/역직렬화 (파일에서 로드)
//! - [ ] 메타데이터 프리셋
//!
//! ## 설계 결정
//!
//! ### 왜 TypeId를 키로 사용하는가?
//!
//! **대안 1: 문자열 키**
//! - 오타 가능: `"tehme"` vs `"theme"`
//! - 런타임 오류
//! - 네임스페이스 충돌 가능
//!
//! **대안 2: Enum**
//! - 사용자 확장 불가능
//! - 라이브러리 수정 필요
//!
//! **TypeId 방식:**
//! - 컴파일 타임 검증
//! - 오타 불가능
//! - 자동 네임스페이스 분리
//! - 무한 확장 가능
//!
//! ### 왜 완전 덮어쓰기가 기본인가?
//!
//! **이유:**
//! - 단순성: 병합 로직이 명확
//! - 예측 가능: 가까운 설정이 항상 우선
//! - 대부분 충분: 90% 사용 사례 커버
//!
//! **부분 병합 필요 시:**
//! - `MergeableMetadata` 트레이트 구현
//! - 필요한 타입만 선택적 적용
//!
//! ### 왜 Box<dyn Any>를 사용하는가?
//!
//! **문제:** 다양한 타입을 하나의 HashMap에 저장
//!
//! **대안 1: Enum으로 모든 타입 나열**
//! - 사용자 확장 불가능
//!
//! **대안 2: 제네릭**
//! - HashMap이 특정 타입으로 고정됨
//!
//! **Box<dyn Any> 방식:**
//! - 모든 타입 저장 가능
//! - TypeId로 안전한 다운캐스팅
//! - 런타임 오버헤드 최소 (빌드 시 한 번만)
//!
//! ## HTML 속성 vs 메타데이터
//!
//! ### 차이점
//!
//! **HTML 속성 (html/attributes.rs):**
//! - 목적: HTML 요소의 속성 (`id`, `class`, `src`)
//! - 범위: HTML 계층 전용
//! - 저장: `HashMap<AttrKey, AttrValues>`
//! - 사용: 렌더링 시점에 HTML 문자열 생성
//!
//! **메타데이터 (metadata/):**
//! - 목적: 계층 간 공유 설정 (테마, CSS 변수, 사용자 정의)
//! - 범위: 전체 프로젝트 (Block, Page, Cite)
//! - 저장: `HashMap<TypeId, Box<dyn Any>>`
//! - 사용: 렌더링 방식 결정, 전역 기능
//!
