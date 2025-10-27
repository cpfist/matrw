use std::fmt::Debug;

use indexmap::IndexMap;

use crate::interface::variable::MatVariable;
use crate::parser::v7::types::structure::Structure7;

#[derive(Debug, Clone)]
pub struct Structure {
    pub value: IndexMap<String, MatVariable>,
}

impl Structure {
    pub fn new(map: IndexMap<String, MatVariable>) -> Self {
        Self { value: map }
    }
    pub fn fieldnames(&self) -> Vec<String> {
        self.value.keys().cloned().collect()
    }
    pub fn get(&self, field: &str) -> Option<&MatVariable> {
        self.value.get(field)
    }
    pub fn take(&mut self, field: &str) -> Option<MatVariable> {
        self.value.shift_remove(field)
    }
}

impl From<Structure7> for Structure {
    fn from(value: Structure7) -> Self {
        let fieldnames = value.fieldnames();
        let mut map = IndexMap::new();
        for (val, key) in value.value().into_iter().zip(fieldnames.into_iter()) {
            map.insert(key, val.into());
        }

        Self::new(map)
    }
}

/// Check of every `Structure` has the same field names
///
pub fn check_same_fields(vec: &[MatVariable]) -> bool {
    if vec.is_empty() {
        return false;
    }

    let first = vec.first().unwrap().fieldnames();

    vec.iter().map(|x| x.fieldnames() == first).into_iter().all(|x| x)
}
