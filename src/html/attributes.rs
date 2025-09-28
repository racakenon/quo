use std::collections::HashMap;

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
