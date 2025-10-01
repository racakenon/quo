//! # cite - 사이트 전체 빌드 오케스트레이션 계층
//!
//! ## 계층의 목적
//! 프로그램 전체에 유일하게 존재하는 전역 객체로서, 모든 페이지를 관리하고
//! 빌드 프로세스를 오케스트레이션합니다.
//!
//! ## 설계 철학
//!
//! ### 1. 중앙 집중 시스템 (Centralized System)
//! **왜 전역 객체가 필요한가?**
//! - 페이지 간 상호 참조: 한 페이지가 다른 페이지를 링크하려면 모든 페이지 정보 필요
//! - 전역 기능: 사이트맵, RSS 피드, 검색 인덱스는 전체 사이트 필요
//! - 일관성 보장: 전역 설정을 한 곳에서 관리
//!
//! ### 2. 방문자 파이프라인 (Visitor Pipeline)
//! 복잡한 빌드 과정을 독립적인 단계로 분리하여 순차 실행합니다.
//!
//! **분석 단계 (Analysis Pass):**
//! 1. `MetadataCollector`: 전체 메타데이터 수집 및 병합
//! 2. `IdGenerator`: 고유 ID 부여 (페이지, 블록)
//! 3. `Counter`: 자동 번호 매기기 (figure, footnote)
//! 4. `LinkResolver`: 링크 검증 및 해결
//! 5. `BacklinkGenerator`: 백링크 생성
//! 6. `CollectionBuilder`: 태그/카테고리 컬렉션
//!
//! **렌더링 단계 (Rendering Pass):**
//! 7. `HtmlRenderer`: 최종 HTML 파일 생성
//! 8. `SitemapGenerator`: sitemap.xml 생성
//! 9. `RssGenerator`: RSS 피드 생성
//! 10. `SearchIndexGenerator`: 검색 인덱스 생성
//!
//! ### 3. 사이트 인덱스 (Site Index)
//! 모든 분석 정보를 하나의 데이터 구조로 통합합니다.
//!
//! **포함 정보:**
//! - 메타데이터: 계층적 병합 결과 (`ResolvedMetadata`)
//! - 식별자: 페이지/블록 ID 맵
//! - 링크: 페이지 간 링크 및 백링크
//! - 컬렉션: 태그별/카테고리별 페이지 목록
//! - 카운터: figure, footnote 번호
//!
//! ## 계층 관계
//!
//! ### Site의 고유 책임
//! - 모든 Page 등록 및 관리
//! - 방문자 파이프라인 실행
//! - 사이트 전역 메타데이터 제공
//! - 빌드 결과물 생성 (HTML, sitemap.xml, feed.xml 등)
//!
//! ### 하위 계층과의 상호작용
//! ```text
//! Site (Cite)
//!   ↓ 등록
//! Page 인스턴스들
//!   ↓ layout() 호출 → Block 트리 획득
//! Block 인스턴스들
//!   ↓ render_to_ir() 호출 → IRNode 생성
//! HTML 파일 출력
//! ```
//!
//! ## 핵심 구조체
//!
//! ### Site
//! - `pages`: 등록된 모든 페이지
//! - `visitors`: 등록된 방문자들 (실행 순서 유지)
//! - `global_metadata`: 사이트 전역 메타데이터
//! - `config`: 빌드 설정 (출력 경로, 기본 URL 등)
//!
//! ### SiteConfig
//! - `name`: 사이트 이름
//! - `base_url`: 기본 URL (링크 생성용)
//! - `output_dir`: 출력 디렉토리
//! - `language`: 기본 언어
//!
//! ### SiteIndex
//! - `resolved_metadata`: 병합된 메타데이터 맵
//! - `block_ids`, `page_ids`: ID 맵
//! - `links`, `backlinks`: 링크 관계
//! - `tags`, `categories`: 컬렉션
//! - `counters`: 자동 번호
//!
//! ## 빌드 프로세스
//!
//! ### 전체 흐름
//! ```text
//! 1. 초기화
//!    Site::new() → 전역 설정
//!
//! 2. 등록 단계
//!    site.register_visitor(visitor)
//!    site.register_page(page)
//!
//! 3. 빌드 실행
//!    site.build()
//!      ↓
//!    3.1. 페이지 트리 수집
//!         page.layout() → Block 트리
//!      ↓
//!    3.2. 방문자 파이프라인 실행
//!         각 방문자가 순차적으로 사이트 순회
//!      ↓
//!    3.3. 사이트 인덱스 생성
//!         방문자 결과를 SiteIndex로 통합
//!      ↓
//!    3.4. 렌더링
//!         RenderContext 생성 및 HTML 파일 생성
//!      ↓
//!    3.5. 전역 파일 생성
//!         sitemap.xml, feed.xml, search.json 등
//! ```
//!
//! ## Visitor 트레이트
//!
//! ### 메서드
//! - `visit_site(&mut self, site: &Site)`: 사이트 방문 시작
//! - `visit_page(&mut self, page: &dyn Page, ctx: &SiteContext)`: 각 페이지 방문
//! - `visit_block(&mut self, block: &dyn Block, ctx: &PageContext)`: 각 블록 방문
//! - `finalize(&self) -> VisitorResult`: 방문 완료 후 결과 반환
//!
//! ### 방문자 분류
//!
//! **분석 방문자:**
//! - `MetadataCollector`: Site → Page → Block 메타데이터 병합
//! - `IdGenerator`: 경로/사용자 지정 기반 고유 ID 생성
//! - `Counter`: 페이지별/사이트별 자동 번호 부여
//! - `LinkResolver`: 링크 대상 검증 및 해결
//! - `BacklinkGenerator`: 역방향 링크 맵 생성
//! - `CollectionBuilder`: 태그/카테고리별 페이지 그룹화
//!
//! **렌더링 방문자:**
//! - `HtmlRenderer`: IRNode → HTML 파일
//!
//! **전역 파일 방문자:**
//! - `SitemapGenerator`: sitemap.xml (SEO)
//! - `RssGenerator`: feed.xml (구독)
//! - `SearchIndexGenerator`: search.json (검색)
//!
//! ## 전역 기능
//!
//! ### 상호 참조 (Cross-Reference)
//! - 링크 검증: 존재하지 않는 페이지 링크 감지 (컴파일 타임)
//! - 백링크 생성: 특정 페이지를 참조하는 모든 페이지 목록
//! - ID 기반 참조: 안정적인 ID로 블록 간 참조
//!
//! ### 컬렉션 (Collections)
//! - 태그별 페이지 모음: `get_pages_by_tag("rust")`
//! - 카테고리별 페이지 모음: `get_pages_by_category("tutorial")`
//! - 최근 수정 페이지: `get_recent_pages(10)`
//! - 날짜 기반 정렬: `get_pages_by_date()`
//!
//! ### 전역 문서
//! - `sitemap.xml`: 검색 엔진용 사이트 구조
//! - `feed.xml`: RSS 구독 피드
//! - `search.json`: 클라이언트 사이드 검색 인덱스
//! - `404.html`: 에러 페이지
//!


pub mod cite;
