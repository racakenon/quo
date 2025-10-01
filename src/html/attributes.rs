//! # attributes.rs - 타입 안전 HTML 속성 관리
//!
//! ## 목적
//! HTML 속성을 타입 안전하게 관리하고, 요소별로 허용된 속성만 설정할 수 있도록 합니다.
//!
//! ## 핵심 개념
//!
//! ### 1. PhantomData로 타입 제약
//! ```rust
//! pub struct Attributes<T> {
//!     pub table: AttrHashMap,
//!     _marker: PhantomData<T>,  // 타입 제약용
//! }
//! ```
//! - `T`는 실제 데이터를 저장하지 않음
//! - 컴파일 타임 타입 검증에만 사용
//! - 메모리 오버헤드 없음 (zero-sized type)
//!
//! ### 2. 트레이트로 속성 그룹 정의
//! ```rust
//! // Global 속성: 모든 요소에 사용 가능
//! impl<T: ForGlobal> Attributes<T> {
//!     pub fn id(self, ...) -> Self { /* ... */ }
//!     pub fn class(self, ...) -> Self { /* ... */ }
//! }
//!
//! // Image 전용 속성
//! impl<T: ForImage> Attributes<T> {
//!     pub fn src(self, ...) -> Self { /* ... */ }
//!     pub fn alt(self, ...) -> Self { /* ... */ }
//! }
//! ```
//!
//! ### 3. SharedAttrs로 불변 공유
//! ```rust
//! pub struct SharedAttrs(Arc<AttrHashMap>);
//! ```
//! - `Arc`로 속성 맵을 참조 카운트 공유
//! - `clone()`이 cheap (참조 카운트만 증가)
//! - IRNode에서 사용 (불변 구조)
//!
//! ## 데이터 구조
//!
//! ### AttrHashMap
//! 실제 속성 데이터를 저장하는 HashMap 래퍼.
//! ```rust
//! pub struct AttrHashMap {
//!     table: HashMap<AttrKey, AttrValues>,
//! }
//! ```
//!
//! ### AttrValues
//! 속성값의 다양한 형태를 표현하는 enum:
//! ```rust
//! pub enum AttrValues {
//!     Token(AttrValue),           // 단일 값: id="main"
//!     Bool(bool),                 // 불린 속성: disabled
//!     Set(HashSet<AttrValue>),    // 집합: class="btn primary"
//!     List(Vec<AttrValue>),       // 순서 있는 목록 (향후 사용)
//! }
//! ```
//!
//! ## 사용 예시
//!
//! ### 기본 사용
//! ```rust
//! // Global 속성만 사용 가능
//! let div_attrs = AttrBuilder::global()
//!     .id(AttrValue::from_str("container", &rule))
//!     .class(AttrValues::build_set(vec!["box".into()], &rule));
//!
//! // Image 속성 사용 가능 (Global 포함)
//! let img_attrs = AttrBuilder::image()
//!     .src(AttrValue::from_str("/logo.png", &rule))
//!     .alt(AttrValue::from_str("Logo", &rule))
//!     .id(AttrValue::from_str("logo", &rule));  // Global도 가능
//! ```
//!
//! ### 컴파일 타임 검증
//! ```rust
//! // ✅ 허용
//! AttrBuilder::image().src(url)  // Image는 ForImage 구현
//!
//! // ❌ 컴파일 에러
//! AttrBuilder::global().src(url)  // Global은 ForImage 미구현
//! ```
//!
//! ## 구현 상태
//! - [x] AttrHashMap 기본 구조
//! - [x] AttrValues enum (Token, Bool, Set, List)
//! - [x] SharedAttrs (Arc 기반 공유)
//! - [x] PhantomData 타입 제약
//! - [x] Global 속성 (id, class, title)
//! - [x] Image 속성 (src, alt)
//! - [x] MergeMode (Keep, Force)
//! - [x] class 속성 병합 로직
//! - [ ] TODO: 더 많은 Global 속성 (data-*, aria-*, style 등)
//! - [ ] TODO: 다른 요소별 속성 그룹 (Form, Table, Media 등)
//! - [ ] TODO: 속성값 검증 (URL 형식, 숫자 범위 등)
//!
//! ## 핵심 타입
//!
//! ### AttrValues::Token
//! 단일 문자열 값을 가지는 속성.
//! ```rust
//! // id="main"
//! AttrValues::Token(AttrValue::from_str("main", &rule))
//! ```
//!
//! ### AttrValues::Bool
//! 불린 속성 (값 없음).
//! ```rust
//! // <input disabled>
//! AttrValues::Bool(true)
//!
//! // 속성 없음
//! AttrValues::Bool(false)
//! ```
//!
//! ### AttrValues::Set
//! 중복 없는 값 집합 (주로 class 속성).
//! ```rust
//! // class="btn primary large"
//! AttrValues::Set(HashSet::from([
//!     AttrValue::from_str("btn", &rule),
//!     AttrValue::from_str("primary", &rule),
//!     AttrValue::from_str("large", &rule),
//! ]))
//! ```
//! - 자동으로 중복 제거
//! - 알파벳 순서로 정렬되어 출력
//!
//! ### AttrValues::List
//! 순서가 있는 값 목록 (향후 확장용).
//! ```rust
//! // 예: srcset="small.jpg 480w, large.jpg 1024w"
//! AttrValues::List(vec![...])
//! ```
//!
//! ## 속성 병합 (Merge)
//!
//! ### MergeMode
//! ```rust
//! pub enum MergeMode {
//!     Keep,   // 기존 값 유지
//!     Force,  // 새 값으로 덮어쓰기
//! }
//! ```
//!
//! ### 병합 동작
//! ```rust
//! let base = AttrHashMap::new()
//!     .add(AttrKey::from_str("id"), AttrValues::Token(id1));
//!
//! let override = AttrHashMap::new()
//!     .add(AttrKey::from_str("id"), AttrValues::Token(id2));
//!
//! // Keep: id1 유지
//! let merged = base.merge(&override, MergeMode::Keep);
//!
//! // Force: id2 사용
//! let merged = base.merge(&override, MergeMode::Force);
//! ```
//!
//! ## class 속성 특수 처리
//!
//! ### 누적 병합
//! class 속성은 덮어쓰기 대신 누적합니다.
//! ```rust
//! let attrs1 = Attributes::global()
//!     .class(classes!["btn"]);
//!
//! let attrs2 = attrs1.class(classes!["primary"]);
//! // 결과: class="btn primary"
//! ```
//!
//! ### 중복 제거
//! ```rust
//! let attrs = Attributes::global()
//!     .class(classes!["btn", "primary"])
//!     .class(classes!["btn", "large"]);
//! // 결과: class="btn large primary" (btn 중복 제거, 정렬됨)
//! ```
//!
//! ## HTML 문자열 변환
//!
//! ### into_string() 동작
//! ```rust
//! let attrs = AttrHashMap::new()
//!     .add(AttrKey::from_str("id"), AttrValues::Token(id))
//!     .add(AttrKey::from_str("class"), AttrValues::Set(classes))
//!     .add(AttrKey::from_str("disabled"), AttrValues::Bool(true));
//!
//! attrs.into_string();
//! // → ' class="btn primary" disabled id="main"'
//! //   (알파벳 순서, 앞에 공백)
//! ```
//!
//! ### 변환 규칙
//! - **정렬**: 속성 키를 알파벳 순으로 정렬
//! - **Token**: ` key="value"`
//! - **Bool(true)**: ` key`
//! - **Bool(false)**: (출력 안 함)
//! - **Set**: ` key="val1 val2 val3"` (정렬됨)
//! - **List**: (현재 미구현)
//!
//! ## 설계 결정
//!
//! ### 왜 PhantomData인가?
//! **대안 1: 런타임 검증**
//! ```rust
//! impl Attributes {
//!     pub fn src(&mut self, url: String) {
//!         if self.element_type != "img" {
//!             panic!("src는 img에만 사용 가능");
//!         }
//!     }
//! }
//! ```
//! 문제: 런타임 에러, 타입 안전성 없음
//!
//! **대안 2: 매크로**
//! ```rust
//! html! { <img src={url} /> }
//! ```
//! 문제: 복잡한 매크로, IDE 지원 약함
//!
//! **PhantomData 방식:**
//! ```rust
//! impl<T: ForImage> Attributes<T> {
//!     pub fn src(self, url: AttrValue) -> Self { /* ... */ }
//! }
//! ```
//! 장점:
//! - 컴파일 타임 검증
//! - 메모리 오버헤드 없음
//! - IDE 자동완성 지원
//!
//! ### 왜 Arc<AttrHashMap>인가?
//! **이유:**
//! - IRNode는 불변 구조
//! - 여러 IRNode가 동일한 속성 공유 가능
//! - `clone()`이 cheap (데이터 복사 없음)
//!
//! **예시:**
//! ```rust
//! let shared = SharedAttrs::from_map(hashmap);
//! let irnode1 = IRNode::new(tag1, shared.clone(), ...);
//! let irnode2 = IRNode::new(tag2, shared.clone(), ...);
//! // shared 데이터는 한 번만 메모리에 존재
//! ```
//!
//! ### 왜 Set을 사용하는가? (class 속성)
//! **이유:**
//! - 중복 자동 제거
//! - 순서 무관 (CSS에서 class 순서는 의미 없음)
//! - 병합 시 자동으로 합쳐짐
//!
//! **출력 시 정렬:**
//! - 일관된 HTML 생성
//! - 테스트/비교 용이
//!
//! ## SharedAttrs 메서드
//!
//! ### with_added
//! 새 속성을 추가한 새 SharedAttrs 반환 (불변 패턴).
//! ```rust
//! let base = SharedAttrs::new();
//! let with_id = base.with_added(
//!     AttrKey::from_str("id"),
//!     AttrValues::Token(id)
//! );
//! // base는 변경되지 않음
//! ```
//!
//! ### into_string
//! HTML 속성 문자열 생성.
//! ```rust
//! let attrs = SharedAttrs::from_map(map);
//! let html_str = attrs.into_string();
//! // → ' id="main" class="btn"'
//! ```
//!
//! ## 향후 개선
//!
//! ### 우선순위: 높음
//! - [ ] 더 많은 Global 속성
//!   - [ ] data-* 속성 지원
//!   - [ ] aria-* 속성 지원
//!   - [ ] style 속성 (인라인 CSS)
//!   - [ ] role 속성
//! - [ ] Form 속성 그룹 (name, value, type, required 등)
//! - [ ] 속성값 검증 (URL, 숫자, 열거형)
//!
//! ### 우선순위: 중간
//! - [ ] Table 속성 (colspan, rowspan)
//! - [ ] Media 속성 (controls, autoplay, loop)
//! - [ ] 이벤트 핸들러 속성 (onclick 등) - 사용 여부 검토
//!
//! ### 우선순위: 낮음
//! - [ ] 커스텀 속성 검증기
//! - [ ] 속성 그룹 매크로
//!
//! ## 예제: 속성 그룹 추가하기
//!
//! ```rust
//! // 1. 트레이트 정의
//! pub trait ForForm: ForGlobal {}
//!
//! // 2. 타입 정의
//! pub struct Form;
//! impl ForGlobal for Form {}
//! impl ForForm for Form {}
//!
//! // 3. 속성 메서드 추가
//! impl<T: ForForm> Attributes<T> {
//!     pub fn name(self, name: AttrValue) -> Self {
//!         let table = self.table.add(
//!             AttrKey::from_str("name"),
//!             AttrValues::Token(name)
//!         );
//!         Attributes { table, _marker: self._marker }
//!     }
//!
//!     pub fn required(self, required: bool) -> Self {
//!         let table = self.table.add(
//!             AttrKey::from_str("required"),
//!             AttrValues::Bool(required)
//!         );
//!         Attributes { table, _marker: self._marker }
//!     }
//! }
//!
//! // 4. 빌더 추가
//! impl AttrBuilder {
//!     pub fn form() -> Attributes<Form> {
//!         Attributes {
//!             table: AttrHashMap::new(),
//!             _marker: PhantomData,
//!         }
//!     }
//! }
//!
//! // 5. 사용
//! let form_attrs = AttrBuilder::form()
//!     .name(AttrValue::from_str("login", &rule))
//!     .required(true)
//!     .id(AttrValue::from_str("login-form", &rule));  // Global도 가능
//! ```

use std::{
    collections::{HashMap, HashSet},
    fmt::Write,
    marker::PhantomData, 
    sync::Arc,
};

use crate::html::{
    rules,
    trust::{self, AttrValue, SafeString},
};

/// 속성값의 다양한 형태를 표현하는 enum.
#[derive(Clone)]
pub enum AttrValues {
    Token(trust::AttrValue),           // 단일 값: id="main"
    Bool(bool),                        // 불린 속성: disabled
    Set(HashSet<trust::AttrValue>),    // 집합: class="btn primary"
    List(Vec<trust::AttrValue>),       // 순서 있는 목록 (향후 사용)
}

impl AttrValues {
    /// Vec<String>을 HashSet<AttrValue>로 변환.
    /// class 속성 등에서 사용.
    pub fn build_set<T>(list: Vec<String>, rule: &T) -> HashSet<AttrValue>
    where
        T: rules::Rules,
    {
        let set = list
            .into_iter()
            .map(|s| AttrValue::from_str(&s, rule))
            .collect();
        set
    }
}

/// 속성 병합 모드.
#[derive(Clone, Copy)]
pub enum MergeMode {
    Keep,   // 기존 값 유지 (새 값 무시)
    Force,  // 새 값으로 덮어쓰기
}

/// Arc로 감싼 불변 속성 맵. IRNode에서 사용.
/// clone()은 참조 카운트만 증가 (cheap).
#[derive(Clone)]
pub struct SharedAttrs(Arc<AttrHashMap>);

impl SharedAttrs {
    pub fn new() -> Self {
        SharedAttrs(Arc::new(AttrHashMap::new()))
    }
    
    pub fn from_map(map: AttrHashMap) -> Self {
        SharedAttrs(Arc::new(map))
    }
    
    pub fn get(&self) -> &AttrHashMap {
        &self.0
    }
    
    /// 새 속성을 추가한 새 SharedAttrs 반환 (불변 패턴).
    pub fn with_added(&self, k: trust::AttrKey, v: AttrValues) -> Self {
        let mut new_map = (*self.0).clone();
        new_map = new_map.add(k, v);
        SharedAttrs(Arc::new(new_map))
    }
    
    /// HTML 속성 문자열로 변환.
    pub fn into_string(&self) -> String {
        self.0.into_string()
    }
}

/// 실제 속성 데이터를 저장하는 HashMap 래퍼.
#[derive(Clone)]
pub struct AttrHashMap {
    table: HashMap<trust::AttrKey, AttrValues>,
}

impl AttrHashMap {
    pub fn new() -> Self {
        AttrHashMap {
            table: HashMap::new(),
        }
    }

    /// 새 속성을 추가한 새 AttrHashMap 반환 (불변 패턴).
    pub fn add(self, k: trust::AttrKey, v: AttrValues) -> Self {
        let mut tb = self.table;
        tb.insert(k, v);
        AttrHashMap { table: tb }
    }

    pub fn get(&self, k: &trust::AttrKey) -> Option<&AttrValues> {
        self.table.get(k)
    }

    pub fn all(&self) -> Vec<(trust::AttrKey, AttrValues)> {
        self.table
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    /// 다른 AttrHashMap과 병합.
    ///
    /// # 병합 규칙
    /// - Keep: 이미 존재하는 키는 유지, 새 키만 추가
    /// - Force: 모든 키를 새 값으로 덮어쓰기
    pub fn merge(self, map: &AttrHashMap, mode: MergeMode) -> Self {
        let map = map.clone();
        let mut table = self.table;

        match mode {
            MergeMode::Keep => {
                for (k, v) in map.table.into_iter() {
                    table.entry(k).or_insert(v);
                }
            }
            MergeMode::Force => {
                table.extend(map.table.into_iter());
            }
        }

        AttrHashMap { table }
    }

    /// HTML 속성 문자열로 변환.
    ///
    /// # 출력 형식
    /// - 속성 키를 알파벳 순으로 정렬
    /// - 각 속성 앞에 공백 추가
    /// - Token: ` key="value"`
    /// - Bool(true): ` key`
    /// - Bool(false): (출력 안 함)
    /// - Set: ` key="val1 val2"` (정렬됨)
    pub fn into_string(&self) -> String {
        let mut result = String::new();
        let mut sorted_attrs: Vec<_> = self.table.iter().collect();
        sorted_attrs.sort_by_key(|(k, _)| *k);
        
        for (k, v) in sorted_attrs {
            match v {
                AttrValues::Token(val) => {
                    let _ = write!(result, r#" {}="{}""#, k.as_str(), val.as_str());
                }
                AttrValues::Bool(true) => {
                    let _ = write!(result, " {}", k.as_str());
                }
                AttrValues::Set(classes) => {
                    if classes.is_empty() {
                        continue;
                    }
                    let mut sorted_classes: Vec<_> = classes.iter().collect();
                    sorted_classes.sort();
                    let class_string = sorted_classes
                        .iter()
                        .map(|c| c.as_str())
                        .collect::<Vec<_>>()
                        .join(" ");
                    let _ = write!(result, r#" {}="{}""#, k.as_str(), &class_string);
                }
                _ => (),
            }
        }
        result
    }
}

// ============================================================================
// 속성 그룹 트레이트
// ============================================================================

/// 속성 그룹을 정의하는 트레이트들.
/// PhantomData와 함께 컴파일 타임 타입 검증에 사용.
pub mod attr_types {
    /// Global 속성: 모든 HTML 요소에 사용 가능.
    pub trait ForGlobal {}
    
    /// Image 속성: img 요소 전용 + Global 속성.
    pub trait ForImage: ForGlobal {}
}

/// Global 속성 타입.
#[derive(Clone)]
pub struct Global;
impl attr_types::ForGlobal for Global {}

/// Image 속성 타입.
#[derive(Clone)]
pub struct Image;
impl attr_types::ForGlobal for Image {}
impl attr_types::ForImage for Image {}

// ============================================================================
// Attributes 구조체 (PhantomData 타입 제약)
// ============================================================================

/// 타입 안전 속성 빌더. PhantomData로 타입 제약.
#[derive(Clone)]
pub struct Attributes<T> {
    pub table: AttrHashMap,
    _marker: PhantomData<T>,
}

/// 속성 빌더 진입점.
pub struct AttrBuilder;

impl AttrBuilder {
    /// Global 속성만 사용 가능한 빌더 생성.
    pub fn global() -> Attributes<Global> {
        Attributes {
            table: AttrHashMap::new(),
            _marker: PhantomData,
        }
    }
    
    /// Image 속성 사용 가능한 빌더 생성 (Global 포함).
    pub fn image() -> Attributes<Image> {
        Attributes {
            table: AttrHashMap::new(),
            _marker: PhantomData,
        }
    }
}

// ============================================================================
// Global 속성 구현 (모든 요소)
// ============================================================================

impl<T: attr_types::ForGlobal> Attributes<T> {
    /// id 속성 설정. 문서 내 고유 식별자.
    pub fn id(self, id: trust::AttrValue) -> Self {
        let table = self
            .table
            .add(trust::AttrKey::from_str("id"), AttrValues::Token(id));
        Attributes {
            table,
            _marker: self._marker,
        }
    }

    /// class 속성 설정 (누적 병합).
    ///
    /// # 특수 동작
    /// - 기존 class와 병합 (덮어쓰기 아님)
    /// - 중복 자동 제거
    /// - 출력 시 알파벳 순 정렬
    pub fn class(self, classes: HashSet<trust::AttrValue>) -> Self {
        let class_key = trust::AttrKey::from_str("class");
        let mut classes = classes;

        // 기존 class 속성과 병합
        if let Some(existing_attr) = self.table.get(&class_key) {
            match existing_attr {
                AttrValues::Token(token) => {
                    classes.insert(token.clone());
                }
                AttrValues::Set(existing_set) => {
                    classes.extend(existing_set.iter().cloned());
                }
                AttrValues::List(attr_values) => {
                    let existing_list = attr_values.clone();
                    classes.extend(existing_list);
                }
                AttrValues::Bool(_) => unreachable!("Class attribute cannot be a boolean."),
            }
        }

        let new_table = self.table.add(class_key, AttrValues::Set(classes));

        Attributes {
            table: new_table,
            _marker: self._marker,
        }
    }

    /// title 속성 설정. 요소에 대한 추가 정보 (툴팁).
    pub fn title(self, title: trust::AttrValue) -> Self {
        let table = self
            .table
            .add(trust::AttrKey::from_str("title"), AttrValues::Token(title));
        Attributes {
            table,
            _marker: self._marker,
        }
    }
}

// ============================================================================
// Image 속성 구현 (img 요소)
// ============================================================================

impl<T: attr_types::ForImage> Attributes<T> {
    /// src 속성 설정. 이미지 URL (필수).
    pub fn src(self, src: trust::AttrValue) -> Self {
        let table = self
            .table
            .add(trust::AttrKey::from_str("src"), AttrValues::Token(src));
        Attributes {
            table,
            _marker: self._marker,
        }
    }
    
    /// alt 속성 설정. 대체 텍스트 (필수).
    pub fn alt(self, alt: trust::AttrValue) -> Self {
        let table = self
            .table
            .add(trust::AttrKey::from_str("alt"), AttrValues::Token(alt));
        Attributes {
            table,
            _marker: self._marker,
        }
    }
}

// TODO: 추가 속성 그룹
// - ForForm: name, value, type, required, disabled 등
// - ForTable: colspan, rowspan 등
// - ForMedia: controls, autoplay, loop 등
// - data-* 속성 지원
// - aria-* 속성 지원
