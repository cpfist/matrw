use std::ops::Index;

use indexmap::IndexMap;
use indexmap::map::{Iter, IterMut};

use crate::interface::helper::is_valid_variable_name;
use crate::interface::variable::MatVariable;
use crate::parser::v7::matfile7::MatFile7;

///
/// MAT-file container
///
/// Stores MAT-file variables using a variable name as unique key.
///
#[derive(Debug)]
pub struct MatFile {
    data: IndexMap<String, MatVariable>,
}

impl MatFile {
    ///
    /// Create empty `MatFile`.
    ///
    pub fn new() -> Self {
        Self {
            data: IndexMap::new(),
        }
    }

    ///
    /// Insert a `MatVariable` called `name`.
    ///
    /// A valid MATLAB variable
    /// - has a length between 1 and 64 characters,
    /// - starts with a ascii alphabetic character,
    /// - contains only ascii alpha-numeric or underscore characters,
    /// - does not match any reserved keyword.
    ///
    /// See also [here](https://www.mathworks.com/help/matlab/matlab_prog/variable-names.html).
    ///
    /// # Panics
    ///
    /// Panics, if `name` does not meet the criteria for a valid MATLAB variable
    /// name.
    ///
    pub fn insert(&mut self, name: &str, value: MatVariable) {
        if !is_valid_variable_name(name) {
            panic!("Invalid variable name");
        }
        self.data.insert(name.to_string(), value);
    }

    ///
    /// Take variable out of container.
    ///
    /// Returns `MatVariable` stored under `name`. If not existing, returns `None`.
    ///
    pub fn take(&mut self, name: &str) -> Option<MatVariable> {
        self.data.shift_remove(name)
    }

    ///
    /// Return if variable `name` exists.
    ///
    pub fn contains(&self, name: &str) -> bool {
        self.data.contains_key(name)
    }

    ///
    /// Return iterator over variables.
    ///
    pub fn iter(&self) -> Iter<'_, String, MatVariable> {
        self.data.iter()
    }

    ///
    /// Return mutable iterator over variables.
    ///
    pub fn iter_mut(&mut self) -> IterMut<'_, String, MatVariable> {
        self.data.iter_mut()
    }
}

impl IntoIterator for MatFile {
    type Item = (String, MatVariable);
    type IntoIter = indexmap::map::IntoIter<String, MatVariable>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl Default for MatFile {
    fn default() -> Self {
        Self::new()
    }
}

impl Index<&str> for MatFile {
    type Output = MatVariable;

    fn index(&self, index: &str) -> &Self::Output {
        self.data.get(index).unwrap_or(&MatVariable::Null)
    }
}

impl From<MatFile7> for MatFile {
    fn from(value: MatFile7) -> Self {
        let mut matfile = MatFile {
            data: IndexMap::new(),
        };

        for (key, value) in value.data.into_iter() {
            matfile.data.insert(key.clone(), MatVariable::from(value));
        }

        matfile
    }
}

#[cfg(test)]
mod tests {
    use binrw::Endian;

    use crate::OwnedIndex;

    use super::*;

    #[test]
    fn false_index() {
        let mat = MatFile::new();
        assert_eq!(mat["some_index"], MatVariable::Null);
    }

    use binrw::*;
    use std::fs::File;
    use std::io::{BufReader, Seek};

    #[test]
    #[ignore]
    fn parse_large_file_into() {
        let f = File::open("tests/large.mat").unwrap();
        let mut reader = BufReader::new(f);
        let _ = reader.seek(std::io::SeekFrom::Current(128));
        let m = MatFile::from(reader.read_type::<MatFile7>(Endian::Little).unwrap());

        println!(
            "m(15000,15000) = {}",
            m["A"].elem([14999, 14999]).to_f64().unwrap()
        )
    }
}
