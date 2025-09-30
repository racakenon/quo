use std::{
    collections::{HashMap, HashSet},
    fmt::Write,
    marker::PhantomData,
};

use crate::html::{
    rules,
    trust::{self, AttrValue, SafeString},
};
#[derive(Clone)]
pub enum AttrValues {
    Token(trust::AttrValue),
    Bool(bool),
    Set(HashSet<trust::AttrValue>),
    List(Vec<trust::AttrValue>),
}

impl AttrValues {
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

#[derive(Clone, Copy)]
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
    pub fn id(self, id: trust::AttrValue) -> Self {
        let table = self
            .table
            .add(trust::AttrKey::from_str("id"), AttrValues::Token(id));
        Attributes {
            table,
            _marker: self._marker,
        }
    }

    pub fn class(self, classes: HashSet<trust::AttrValue>) -> Self {
        let class_key = trust::AttrKey::from_str("class");
        let mut classes = classes;

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

impl<T: attr_types::ForImage> Attributes<T> {
    pub fn src(self, src: trust::AttrValue) -> Self {
        let table = self
            .table
            .add(trust::AttrKey::from_str("src"), AttrValues::Token(src));
        Attributes {
            table,
            _marker: self._marker,
        }
    }
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
