use binrw::*;
use std::fmt::Debug;

use super::array_data_value::*;
use super::parse_write::*;
use crate::parser::v7::flags::{MatFileDataTypes, MatlabArrayTypes};

#[binrw]
#[derive(Debug, Clone)]
#[br(import(arrtype: MatlabArrayTypes, is_logical: bool))]
pub struct ArrayDataSmall {
    #[brw(pad_size_to = 2)]
    data_type: MatFileDataTypes,
    pub data_size: u16,
    #[br(parse_with = parse_array_data, args(&data_type, data_size as u32, arrtype, is_logical))]
    #[bw(pad_size_to = 4, write_with = write_array_data)]
    pub value: ArrayDataValueVar,
}

impl ArrayDataSmall {
    pub fn new(data_type: MatFileDataTypes, data_size: u16, value: ArrayDataValueVar) -> Self {
        Self {
            data_type,
            data_size,
            value,
        }
    }
}

impl std::fmt::Display for ArrayDataSmall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

#[binrw]
#[derive(Debug, Clone)]
#[br(import(is_logical: bool))]
pub struct ArrayDataSparseSmall {
    #[brw(pad_size_to = 2)]
    data_type: MatFileDataTypes,
    pub data_size: u16,
    #[br(parse_with = parse_array_data_sparse, args(&data_type, data_size as u32, is_logical))]
    #[bw(pad_size_to = 4, write_with = write_array_data)]
    pub value: ArrayDataValueVar,
}

impl ArrayDataSparseSmall {
    pub fn new(data_type: MatFileDataTypes, data_size: u16, value: ArrayDataValueVar) -> Self {
        Self {
            data_type,
            data_size,
            value,
        }
    }
}
