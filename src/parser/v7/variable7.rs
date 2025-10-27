use crate::{
    interface::variable::MatVariable,
    parser::v7::types::{
        cell_array::CellArray7,
        compressed_array::CompressedArray7,
        empty::Empty7,
        numeric_array::NumericArray7,
        object::{ObjectHandle7, ObjectMCOS7},
        sparse_array::SparseArray7,
        structure::Structure7,
        structure_array::StructureArray7,
    },
};

use binrw::binrw;

use super::types::numeric_array::NumericArrayNew;

#[binrw]
#[derive(Debug, Clone)]
pub enum MatVariable7 {
    ObjectMCOS(ObjectMCOS7),
    ObjectHandle(ObjectHandle7),
    //
    Compressed(CompressedArray7),
    Numeric(NumericArray7),
    Cell(CellArray7),
    Structure(Structure7),
    StructureArray(StructureArray7),
    Sparse(SparseArray7),
    Empty(Empty7),
}

impl MatVariable7 {
    pub fn set_name(&mut self, name: &str) {
        match self {
            MatVariable7::Numeric(val) => val.set_name(name),
            MatVariable7::Compressed(val) => val.set_name(name),
            MatVariable7::Structure(val) => val.set_name(name),
            MatVariable7::StructureArray(val) => val.set_name(name),
            MatVariable7::Cell(val) => val.set_name(name),
            MatVariable7::Sparse(val) => val.set_name(name),
            _ => unimplemented!(),
        };
    }
    pub fn name(&self) -> String {
        match self {
            MatVariable7::Numeric(val) => val.name(),
            MatVariable7::Compressed(val) => val.name(),
            MatVariable7::Structure(val) => val.name(),
            MatVariable7::StructureArray(val) => val.name(),
            MatVariable7::Cell(val) => val.name(),
            MatVariable7::ObjectMCOS(val) => val.name(),
            MatVariable7::ObjectHandle(val) => val.name(),
            MatVariable7::Sparse(val) => val.name(),
            _ => unimplemented!("{:#?}", self),
        }
    }
    pub fn size(&self) -> usize {
        match self {
            MatVariable7::Compressed(_) => unimplemented!(),
            MatVariable7::Numeric(val) => val.size(),
            MatVariable7::Structure(val) => val.size(),
            MatVariable7::StructureArray(val) => val.size(),
            MatVariable7::Cell(val) => val.size(),
            MatVariable7::Sparse(val) => val.size(),
            _ => unimplemented!(),
        }
    }
}

impl From<MatVariable> for MatVariable7 {
    fn from(value: MatVariable) -> Self {
        match value {
            MatVariable::Compressed(v) => MatVariable7::Compressed(CompressedArray7::from(v)),
            MatVariable::NumericArray(v) => MatVariable7::Numeric(NumericArray7::from(v)),
            MatVariable::CellArray(v) => MatVariable7::Cell(CellArray7::from(v)),
            MatVariable::Structure(v) => MatVariable7::Structure(Structure7::from(v)),
            MatVariable::StructureArray(v) => MatVariable7::StructureArray(StructureArray7::from(v)),
            MatVariable::SparseArray(v) => MatVariable7::Sparse(SparseArray7::from(v)),
            MatVariable::Unsupported => {
                MatVariable7::Numeric(NumericArray7::new(vec![1, 1], Vec::<f64>::new(), None))
            }
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    // use std::f64::consts::PI;
    //
    // use super::*;
    // use crate::parser::v7::types::numeric_array::NumericArrayNew;
    // use binrw::io::Cursor; // A no_std reimplementation of std::io
    // use binrw::BinReaderExt; // extension traits for use with readers and writers
    // use binrw::BinWrite;
    //
    // /// Binary representation of a mat file containing a variable `s` with a struct.
    // /// To reproduce, in a MATLAB session with a clean workspace run
    // /// `>> s.a = 42;`
    // /// `>> s.b = pi;`
    // /// `>> save('example.mat', 's', '-nocompression');`
    // ///
    // const DATA_STRUCTURE_1: [u8; 192] = [
    //     0x0e, 0x00, 0x00, 0x00, 0xb8, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x02,
    //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x01, 0x00,
    //     0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x73, 0x00, 0x00, 0x00, 0x05, 0x00, 0x04,
    //     0x00, 0x03, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x61, 0x00, 0x00, 0x62,
    //     0x00, 0x00, 0x00, 0x00, 0x0e, 0x00, 0x00, 0x00, 0x30, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x08,
    //     0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x08, 0x00,
    //     0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //     0x00, 0x02, 0x00, 0x01, 0x00, 0x2a, 0x00, 0x00, 0x00, 0x0e, 0x00, 0x00, 0x00, 0x38, 0x00, 0x00, 0x00,
    //     0x06, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05,
    //     0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00,
    //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x18, 0x2d, 0x44,
    //     0x54, 0xfb, 0x21, 0x09, 0x40,
    // ];
    //
    // #[test]
    // fn deserialize_variable7_structure1() {
    //     let mut bin = Cursor::new(&DATA_STRUCTURE_1);
    //     let data: MatVariable = bin.read_le::<MatVariable7>().unwrap().into();
    //     println!("data: {:#?}", &data);
    // }
    //
    // #[test]
    // fn serialize_variable7_structure1() {
    //     let mut bin = Cursor::new(vec![]);
    //     let s = vec![
    //         MatVariable7::Numeric(NumericArray7::new("".to_string(), vec![1, 1], vec![42.], None)),
    //         MatVariable7::Numeric(NumericArray7::new("".to_string(), vec![1, 1], vec![PI], None)),
    //     ];
    //
    //     let data = MatVariable7::Structure(Structure7::new(
    //         "s".to_string(),
    //         vec!["a".to_string(), "b".to_string()],
    //         s,
    //     ));
    //     println!("data: {:#?}", &data);
    //     data.write_le(&mut bin).unwrap();
    //
    //     let inner = bin.into_inner();
    //     println!("Orig bin: {:?}", DATA_STRUCTURE_1);
    //     println!("Ser  bin: {:?}", &inner);
    //     assert!(inner == DATA_STRUCTURE_1);
    // }
    //
    // /// Binary representation of a mat file containing a variable `a` with a scalar.
    // /// To reproduce, in a MATLAB session with a clean workspace run
    // /// `>> a = 32;`
    // /// `>> save('example.mat', 'a', '-nocompression');`
    // ///
    // const DATA_U32_1: [u8; 56] = [
    //     0x0e, 0x00, 0x00, 0x00, 0x30, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x0d,
    //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x01, 0x00,
    //     0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x61, 0x00, 0x00, 0x00, 0x06, 0x00, 0x04,
    //     0x00, 0x20, 0x00, 0x00, 0x00,
    // ];
    //
    // #[test]
    // fn deserialize_variable7_u32_1() {
    //     let mut bin = Cursor::new(&DATA_U32_1);
    //     let data: MatVariable = bin.read_le::<MatVariable7>().unwrap().into();
    //     println!("data: {:#?}", &data);
    // }
}
