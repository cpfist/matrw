//! Module containing types for matching *Array Name Subelements*.

use binrw::*;
use std::fmt::Debug;

use crate::parser::v7::flags::MatFileDataTypes;

#[binrw]
#[derive(Debug, Clone)]
pub enum ArrayName {
    Empty(ArrayNameEmpty),
    Normal(ArrayNameNormal),
    Small(ArrayNameSmall),
}

impl ArrayName {
    pub fn new(name: String) -> Self {
        let nelem = name.len();
        if nelem == 0 {
            Self::Empty(ArrayNameEmpty::new())
        } else if nelem < 5 {
            Self::Small(ArrayNameSmall::new(name))
        } else {
            Self::Normal(ArrayNameNormal::new(name))
        }
    }
    pub fn name(&self) -> String {
        match self {
            ArrayName::Empty(_) => "".to_string(),
            ArrayName::Normal(x) => String::from_utf8(x.chars.clone()).unwrap(),
            ArrayName::Small(x) => String::from_utf8(x.chars.clone()).unwrap(),
        }
    }
    pub fn size(&self) -> u32 {
        match self {
            ArrayName::Empty(_) => 8,
            ArrayName::Normal(s) => {
                let padding = if (s.data_size % 8) == 0 {
                    0
                } else {
                    8 - s.data_size % 8
                };

                8 + s.data_size + padding
            }
            ArrayName::Small(s) => {
                let padding = if (s.data_size % 4) == 0 {
                    0
                } else {
                    4 - s.data_size % 4
                };
                (4 + s.data_size + padding) as u32
            }
        }
    }
}

/// Struct matching *Matrix* name whith empty content.
#[binrw]
#[derive(Clone)]
pub struct ArrayNameEmpty {
    /// Temporary field to distinguish from *Small Data Element Format*
    #[brw(restore_position)]
    #[br(assert(check == 1))]
    check: u64,
    /// Data type tag
    #[br(assert(data_type == MatFileDataTypes::MiINT8))]
    data_type: MatFileDataTypes,
}

impl ArrayNameEmpty {
    pub fn new() -> Self {
        Self {
            check: 1,
            data_type: MatFileDataTypes::MiINT8,
        }
    }
}

impl Debug for ArrayNameEmpty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArrayNameEmpty")
            .field("data_type", &self.data_type)
            .field("chars", &"")
            .finish()
    }
}

impl Default for ArrayNameEmpty {
    fn default() -> Self {
        Self::new()
    }
}

/// Struct matching *Matrix* name in *Data Element Format*.
#[binrw]
#[derive(Clone)]
pub struct ArrayNameNormal {
    /// Data type tag
    #[br(assert(data_type == MatFileDataTypes::MiINT8))]
    #[brw(pad_size_to = 2)]
    data_type: MatFileDataTypes,
    /// Temporary field to distinguish from *Small Data Element Format*
    #[br(assert(check == 0))]
    check: u16,
    /// Number of bytes tag
    data_size: u32,
    /// Data
    #[br(count = data_size)]
    #[brw(pad_size_to = 8)]
    chars: Vec<u8>,
}

impl ArrayNameNormal {
    pub fn new(name: String) -> Self {
        let chars = name.into_bytes();

        Self {
            data_type: MatFileDataTypes::MiINT8,
            check: 0u16,
            data_size: chars.len() as u32,
            chars,
        }
    }
}

impl Debug for ArrayNameNormal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = String::from_utf8(self.chars.clone()).unwrap();
        f.debug_struct("ArrayNameNormal")
            .field("data_type", &self.data_type)
            .field("data_size", &self.data_size)
            .field("chars", &s)
            .finish()
    }
}

/// Struct matching *Matrix* name in *Small Data Element Format*.
#[binrw]
#[derive(Clone)]
pub struct ArrayNameSmall {
    /// Data type tag
    #[br(assert(data_type == MatFileDataTypes::MiINT8))]
    #[brw(pad_size_to = 2)]
    data_type: MatFileDataTypes,
    /// Number of bytes tag
    data_size: u16,
    /// Data
    #[br(count = data_size as u32)]
    #[brw(pad_size_to = 4)]
    chars: Vec<u8>,
}

impl ArrayNameSmall {
    pub fn new(name: String) -> Self {
        let chars = name.into_bytes();

        Self {
            data_type: MatFileDataTypes::MiINT8,
            data_size: chars.len() as u16,
            chars,
        }
    }
}

impl Debug for ArrayNameSmall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = String::from_utf8(self.chars.clone()).unwrap();
        f.debug_struct("ArrayNameSmall")
            .field("data_type", &self.data_type)
            .field("data_size", &self.data_size)
            .field("chars", &s)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use binrw::BinReaderExt;
    use binrw::io::Cursor; // A no_std reimplementation of std::io // extension traits for use with readers and writers

    #[test]
    fn parse_small_name() {
        // Deserialize
        let mut bin = Cursor::new(b"\x01\x00\x03\x00\x61\x62\x63\x00");
        let data = bin.read_le::<ArrayName>().unwrap();
        println!("Deserialized data: {:#?}", &data);
        assert!(data.name() == "abc");

        // Serialize
        let mut bin_new = Cursor::new(vec![]);
        let data_new = ArrayName::Small(ArrayNameSmall::new(data.name()));
        data_new.write_le(&mut bin_new).unwrap();
        println!("Deserialized data: {:?}", bin);
        println!("  Serialized data: {:?}", bin_new);
        assert!(bin_new.into_inner().to_vec() == bin.into_inner().to_vec());
    }

    #[test]
    fn parse_small_empty_name() {
        // Deserialize
        let mut bin = Cursor::new(b"\x01\x00\x00\x00\x00\x00\x00\x00");
        let data = bin.read_le::<ArrayName>().unwrap();
        println!("Deserialized data: {:#?}", &data);
        assert!(data.name() == "");

        // Serialize
        let mut bin_new = Cursor::new(vec![]);
        let data_new = ArrayName::Empty(ArrayNameEmpty::new());
        data_new.write_le(&mut bin_new).unwrap();
        println!("Deserialized data: {:?}", bin);
        println!("  Serialized data: {:?}", bin_new);
        assert!(bin_new.into_inner().to_vec() == bin.into_inner().to_vec());
    }

    #[test]
    fn parse_normal_name() {
        // Deserialize
        let mut bin = Cursor::new(b"\x01\x00\x00\x00\x06\x00\x00\x00\x61\x62\x63\x64\x65\x66\x00\x00");
        let data = bin.read_le::<ArrayName>().unwrap();
        println!("Deserialized data: {:#?}", &data);
        assert!(data.name() == "abcdef");

        // Serialize
        let mut bin_new = Cursor::new(vec![]);
        let data_new = ArrayName::Normal(ArrayNameNormal::new(data.name()));
        data_new.write_le(&mut bin_new).unwrap();
        println!("Deserialized data: {:?}", bin);
        println!("  Serialized data: {:?}", bin_new);
        assert!(bin_new.into_inner().to_vec() == bin.into_inner().to_vec());
    }
}
