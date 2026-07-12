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
