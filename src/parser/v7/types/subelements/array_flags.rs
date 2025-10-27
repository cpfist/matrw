//! Module containing types for matching *Array Flag Subelements*.

use binrw::*;
use std::fmt::Debug;

use crate::parser::v7::flags::{MatFileDataTypes, MatlabArrayTypes};

fn parse_flags(is_complex: &bool, is_global: &bool, is_logical: &bool) -> u8 {
    let mut b = 0b00000000;
    if *is_complex {
        b |= 0b00001000;
    }
    if *is_global {
        b |= 0b00000100;
    }
    if *is_logical {
        b |= 0b00000010;
    }

    b
}

#[binrw]
#[derive(Clone)]
pub struct ArrayFlagBits {
    /// Temporary bit buffer
    #[bw(calc = parse_flags(is_complex, is_global, is_logical))]
    _raw: u8,
    /// Complex flag
    #[br(calc = (_raw & 0b00001000) != 0)]
    #[bw(ignore)]
    pub is_complex: bool,
    /// Global flag
    #[br(calc = (_raw & 0b00000100) != 0)]
    #[bw(ignore)]
    pub is_global: bool,
    /// Logical flag
    #[br(calc = (_raw & 0b00000010) != 0)]
    #[bw(ignore)]
    pub is_logical: bool,
}

impl ArrayFlagBits {
    pub fn new(is_complex: bool, is_global: bool, is_logical: bool) -> Self {
        Self {
            is_complex,
            is_global,
            is_logical,
        }
    }
}

impl Debug for ArrayFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArrayFlagBits")
            .field("is_complex", &self.is_complex)
            .field("is_global", &self.is_global)
            .field("is_logical", &self.is_logical)
            .finish()
    }
}

#[binrw]
#[derive(Debug, Clone)]
pub struct ArrayProps {
    /// Data type tag
    #[brw(pad_size_to = 4)]
    #[br(assert(data_type == MatFileDataTypes::MiUINT32))]
    data_type: MatFileDataTypes,
    /// Number of bytes tag
    #[br(assert(data_size == 8))]
    data_size: u32,
    /// Matlab type class
    pub array_class: MatlabArrayTypes,
    /// Array flags
    #[brw(pad_after = 2)]
    pub array_flags: ArrayFlagBits,
    /// Number of non-zero sparse values
    pub sparse_num: u32,
}

impl ArrayProps {
    pub fn new(array_class: MatlabArrayTypes, array_flags: ArrayFlagBits, sparse_num: u32) -> Self {
        Self {
            data_type: MatFileDataTypes::MiUINT32,
            data_size: 8u32,
            array_class,
            array_flags,
            sparse_num,
        }
    }
    pub fn size(&self) -> u32 {
        16
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use binrw::BinReaderExt;
    use binrw::io::Cursor; // A no_std reimplementation of std::io // extension traits for use with readers and writers

    #[test]
    fn flags_none() {
        // Deserialize
        let mut bin = Cursor::new(b"\x06\x00\x00\x00\x08\x00\x00\x00\x06\x00\x00\x00\x00\x00\x00\x00");
        let data = bin.read_le::<ArrayProps>().unwrap();
        println!("Deserialized data: {:#?}", &data);
        assert!(!data.array_flags.is_complex);
        assert!(!data.array_flags.is_global);
        assert!(!data.array_flags.is_logical);

        // Serialize
        let mut bin_new = Cursor::new(vec![]);
        let data_new = ArrayProps::new(
            data.array_class,
            ArrayFlagBits {
                is_complex: data.array_flags.is_complex,
                is_global: data.array_flags.is_global,
                is_logical: data.array_flags.is_logical,
            },
            0,
        );
        data_new.write_le(&mut bin_new).unwrap();
        println!("Deserialized data: {:?}", bin);
        println!("  Serialized data: {:?}", bin_new);
        assert!(bin_new.into_inner().to_vec() == bin.into_inner().to_vec());
    }

    #[test]
    fn flags_complex() {
        // Deserialize
        let mut bin = Cursor::new(b"\x06\x00\x00\x00\x08\x00\x00\x00\x06\x08\x00\x00\x00\x00\x00\x00");
        let data = bin.read_le::<ArrayProps>().unwrap();
        println!("Deserialized data: {:#?}", &data);
        assert!(data.array_flags.is_complex);
        assert!(!data.array_flags.is_global);
        assert!(!data.array_flags.is_logical);

        // Serialize
        let mut bin_new = Cursor::new(vec![]);
        let data_new = ArrayProps::new(
            data.array_class,
            ArrayFlagBits {
                is_complex: data.array_flags.is_complex,
                is_global: data.array_flags.is_global,
                is_logical: data.array_flags.is_logical,
            },
            0,
        );
        data_new.write_le(&mut bin_new).unwrap();
        println!("Deserialized data: {:?}", bin);
        println!("  Serialized data: {:?}", bin_new);
        assert!(bin_new.into_inner().to_vec() == bin.into_inner().to_vec());
    }

    #[test]
    fn flags_global() {
        // Deserialize
        let mut bin = Cursor::new(b"\x06\x00\x00\x00\x08\x00\x00\x00\x06\x04\x00\x00\x00\x00\x00\x00");
        let data = bin.read_le::<ArrayProps>().unwrap();
        println!("Deserialized data: {:#?}", &data);
        assert!(!data.array_flags.is_complex);
        assert!(data.array_flags.is_global);
        assert!(!data.array_flags.is_logical);

        // Serialize
        let mut bin_new = Cursor::new(vec![]);
        let data_new = ArrayProps::new(
            data.array_class,
            ArrayFlagBits {
                is_complex: data.array_flags.is_complex,
                is_global: data.array_flags.is_global,
                is_logical: data.array_flags.is_logical,
            },
            0,
        );
        data_new.write_le(&mut bin_new).unwrap();
        println!("Deserialized data: {:?}", bin);
        println!("  Serialized data: {:?}", bin_new);
        assert!(bin_new.into_inner().to_vec() == bin.into_inner().to_vec());
    }

    #[test]
    fn flags_logical() {
        // Deserialize
        let mut bin = Cursor::new(b"\x06\x00\x00\x00\x08\x00\x00\x00\x06\x02\x00\x00\x00\x00\x00\x00");
        let data = bin.read_le::<ArrayProps>().unwrap();
        println!("Deserialized data: {:#?}", &data);
        assert!(!data.array_flags.is_complex);
        assert!(!data.array_flags.is_global);
        assert!(data.array_flags.is_logical);

        // Serialize
        let mut bin_new = Cursor::new(vec![]);
        let data_new = ArrayProps::new(
            data.array_class,
            ArrayFlagBits {
                is_complex: data.array_flags.is_complex,
                is_global: data.array_flags.is_global,
                is_logical: data.array_flags.is_logical,
            },
            0,
        );
        data_new.write_le(&mut bin_new).unwrap();
        println!("Deserialized data: {:?}", bin);
        println!("  Serialized data: {:?}", bin_new);
        assert!(bin_new.into_inner().to_vec() == bin.into_inner().to_vec());
    }

    #[test]
    fn flags_complex_global() {
        // Deserialize
        let mut bin = Cursor::new(b"\x06\x00\x00\x00\x08\x00\x00\x00\x06\x0c\x00\x00\x00\x00\x00\x00");
        let data = bin.read_le::<ArrayProps>().unwrap();
        println!("Deserialized data: {:#?}", &data);
        assert!(data.array_flags.is_complex);
        assert!(data.array_flags.is_global);
        assert!(!data.array_flags.is_logical);

        // Serialize
        let mut bin_new = Cursor::new(vec![]);
        let data_new = ArrayProps::new(
            data.array_class,
            ArrayFlagBits {
                is_complex: data.array_flags.is_complex,
                is_global: data.array_flags.is_global,
                is_logical: data.array_flags.is_logical,
            },
            0,
        );
        data_new.write_le(&mut bin_new).unwrap();
        println!("Deserialized data: {:?}", bin);
        println!("  Serialized data: {:?}", bin_new);
        assert!(bin_new.into_inner().to_vec() == bin.into_inner().to_vec());
    }

    #[test]
    fn flags_complex_logical() {
        // Deserialize
        let mut bin = Cursor::new(b"\x06\x00\x00\x00\x08\x00\x00\x00\x06\x0a\x00\x00\x00\x00\x00\x00");
        let data = bin.read_le::<ArrayProps>().unwrap();
        println!("Deserialized data: {:#?}", &data);
        assert!(data.array_flags.is_complex);
        assert!(!data.array_flags.is_global);
        assert!(data.array_flags.is_logical);

        // Serialize
        let mut bin_new = Cursor::new(vec![]);
        let data_new = ArrayProps::new(
            data.array_class,
            ArrayFlagBits {
                is_complex: data.array_flags.is_complex,
                is_global: data.array_flags.is_global,
                is_logical: data.array_flags.is_logical,
            },
            0,
        );
        data_new.write_le(&mut bin_new).unwrap();
        println!("Deserialized data: {:?}", bin);
        println!("  Serialized data: {:?}", bin_new);
        assert!(bin_new.into_inner().to_vec() == bin.into_inner().to_vec());
    }

    #[test]
    fn flags_global_logical() {
        // Deserialize
        let mut bin = Cursor::new(b"\x06\x00\x00\x00\x08\x00\x00\x00\x06\x06\x00\x00\x00\x00\x00\x00");
        let data = bin.read_le::<ArrayProps>().unwrap();
        println!("Deserialized data: {:#?}", &data);
        assert!(!data.array_flags.is_complex);
        assert!(data.array_flags.is_global);
        assert!(data.array_flags.is_logical);

        // Serialize
        let mut bin_new = Cursor::new(vec![]);
        let data_new = ArrayProps::new(
            data.array_class,
            ArrayFlagBits {
                is_complex: data.array_flags.is_complex,
                is_global: data.array_flags.is_global,
                is_logical: data.array_flags.is_logical,
            },
            0,
        );
        data_new.write_le(&mut bin_new).unwrap();
        println!("Deserialized data: {:?}", bin);
        println!("  Serialized data: {:?}", bin_new);
        assert!(bin_new.into_inner().to_vec() == bin.into_inner().to_vec());
    }

    #[test]
    fn flags_complex_global_logical() {
        // Deserialize
        let mut bin = Cursor::new(b"\x06\x00\x00\x00\x08\x00\x00\x00\x06\x0e\x00\x00\x00\x00\x00\x00");
        let data = bin.read_le::<ArrayProps>().unwrap();
        println!("Deserialized data: {:#?}", &data);
        assert!(data.array_flags.is_complex);
        assert!(data.array_flags.is_global);
        assert!(data.array_flags.is_logical);

        // Serialize
        let mut bin_new = Cursor::new(vec![]);
        let data_new = ArrayProps::new(
            data.array_class,
            ArrayFlagBits {
                is_complex: data.array_flags.is_complex,
                is_global: data.array_flags.is_global,
                is_logical: data.array_flags.is_logical,
            },
            0,
        );
        data_new.write_le(&mut bin_new).unwrap();
        println!("Deserialized data: {:?}", bin);
        println!("  Serialized data: {:?}", bin_new);
        assert!(bin_new.into_inner().to_vec() == bin.into_inner().to_vec());
    }
}
