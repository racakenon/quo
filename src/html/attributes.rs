use std::{collections::HashMap, marker::PhantomData};

use crate::html::trust::{self, SafeString};
#[derive(Clone)]
pub enum AttrValues {
    Token(trust::AttrValue),
    Bool(bool),
    List(Vec<trust::AttrValue>),
}
pub enum MergeMode {
    Keep,
    Force,
}

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

    pub fn merge(self, map: AttrHashMap, mode: MergeMode) -> Self {
        let mut table = self.table;

        match mode {
            MergeMode::Keep => {
                for (k, v) in map.all() {
                    table.entry(k).or_insert(v);
                }
            }
            MergeMode::Force => {
                table.extend(map.all());
            }
        }

        AttrHashMap { table }
    }

    pub fn to_str(&self) -> String {
        self.all()
            .iter()
            .map(|(k, v)| match v {
                AttrValues::Token(val) => {
                    let k = k.clone();
                    let val = val.clone();
                    format!(r#" {}="{}""#, k.to_str(), val.to_str())
                }
                AttrValues::Bool(true) => {
                    let k = k.clone();
                    format!(" {}", k.to_str())
                }
                _ => "".to_string(),
            })
            .collect()
    }
}

pub mod attr_types {
    pub trait ForGlobal {}
    pub trait ForImage: ForGlobal {}
}

pub struct Global;
impl attr_types::ForGlobal for Global {}

pub struct Image;
impl attr_types::ForGlobal for Image {}
impl attr_types::ForImage for Image {}

pub struct Attributes<T> {
    pub table: AttrHashMap,
    _marker: PhantomData<T>,
}

pub struct AttrBuilder;
impl AttrBuilder {
    pub fn global() -> Attributes<Global> {
        Attributes {
            table: AttrHashMap::new(),
            _marker: PhantomData,
        }
    }
    pub fn image() -> Attributes<Image> {
        Attributes {
            table: AttrHashMap::new(),
            _marker: PhantomData,
        }
    }
}

impl<T: attr_types::ForGlobal> Attributes<T> {
    pub fn id(mut self, id: trust::AttrValue) -> Self {
        self.table = self
            .table
            .add(trust::AttrKey::from_str("id"), AttrValues::Token(id));
        self
    }
    pub fn class(mut self, class: trust::AttrValue) -> Self {
        self.table = self
            .table
            .add(trust::AttrKey::from_str("class"), AttrValues::Token(class));
        self
    }
    pub fn title(mut self, title: trust::AttrValue) -> Self {
        self.table = self
            .table
            .add(trust::AttrKey::from_str("title"), AttrValues::Token(title));
        self
    }
}

impl<T: attr_types::ForImage> Attributes<T> {
    pub fn src(mut self, src: trust::AttrValue) -> Self {
        self.table = self
            .table
            .add(trust::AttrKey::from_str("src"), AttrValues::Token(src));
        self
    }
    pub fn alt(mut self, alt: trust::AttrValue) -> Self {
        self.table = self
            .table
            .add(trust::AttrKey::from_str("alt"), AttrValues::Token(alt));
        self
    }
}
