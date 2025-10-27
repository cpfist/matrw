use binrw::*;
use std::fmt::Debug;

use super::array_data_value::*;
use super::parse_write::*;
use crate::parser::v7::flags::{MatFileDataTypes, MatlabArrayTypes};

#[binrw]
#[derive(Clone)]
#[br(import(arr_type: MatlabArrayTypes, is_logical: bool))]
pub struct ArrayDataNormal {
    #[brw(pad_size_to = 2)]
    data_type: MatFileDataTypes,
    #[br(assert(check == 0))]
    check: u16,
    pub data_size: u32,
    #[br(parse_with = parse_array_data, args(&data_type, data_size, arr_type, is_logical))]
    #[bw(write_with = write_array_data)]
    pub value: ArrayDataValueVar,
}

impl Debug for ArrayDataNormal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArrayDataNormal")
            .field("data_type", &self.data_type)
            .field("data_size", &self.data_size)
            .field("value", &self.value)
            .finish()
    }
}

impl ArrayDataNormal {
    pub fn new(data_type: MatFileDataTypes, data_size: u32, value: ArrayDataValueVar) -> Self {
        Self {
            data_type,
            check: 0u16,
            data_size,
            value,
        }
    }
}

impl std::fmt::Display for ArrayDataNormal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

#[derive(Debug, Clone)]
#[binrw]
#[br(import(is_logical: bool))]
pub struct ArrayDataSparseNormal {
    /// Data type tag
    #[brw(pad_size_to = 2)]
    pub data_type: MatFileDataTypes,
    #[br(assert(check == 0))]
    pub check: u16,
    /// Number of bytes tag
    pub data_size: u32,
    /// Array dimensions
    #[br(parse_with = parse_array_data_sparse, args(&data_type, data_size, is_logical))]
    #[bw(write_with = write_array_data)]
    pub value: ArrayDataValueVar,
}

impl ArrayDataSparseNormal {
    pub fn new(data_type: MatFileDataTypes, data_size: u32, value: ArrayDataValueVar) -> Self {
        Self {
            data_type,
            check: 0u16,
            data_size,
            value,
        }
    }
}
