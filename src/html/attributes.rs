use std::collections::HashMap;

use crate::html::trust;
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
pub trait AttrMap {
    fn new() -> Self;
    fn add(self, k: trust::AttrKey, v: AttrValues) -> Self;
    fn get(&self, k: &trust::AttrKey) -> Option<&AttrValues>;
    fn all(&self) -> Vec<(trust::AttrKey, AttrValues)>;
    fn merge<T>(self, map: T, mode: MergeMode) -> Self
    where
        T: AttrMap;
}

#[derive(Clone)]
pub struct AttrHashMap {
    table: HashMap<trust::AttrKey, AttrValues>,
}

impl AttrMap for AttrHashMap {
    fn new() -> Self {
        AttrHashMap {
            table: HashMap::new(),
        }
    }

    fn add(self, k: trust::AttrKey, v: AttrValues) -> Self {
        let mut tb = self.table;
        tb.insert(k, v);
        AttrHashMap { table: tb }
    }

    fn get(&self, k: &trust::AttrKey) -> Option<&AttrValues> {
        self.table.get(k)
    }

    fn all(&self) -> Vec<(trust::AttrKey, AttrValues)> {
        self.table
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    fn merge<T>(self, map: T, mode: MergeMode) -> Self
    where
        T: AttrMap,
    {
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
}
