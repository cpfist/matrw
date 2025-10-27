use binrw::*;
use std::fmt::{Debug, Display};
use std::mem;

use crate::parser::v7::flags::MatFileDataTypes;

#[binrw]
#[derive(Debug, Clone)]
#[br(import(data_type: &MatFileDataTypes, data_size: u32))]
pub enum ArrayDataValueVarRaw {
    #[br(pre_assert(*data_type == MatFileDataTypes::MiUINT8))]
    ArrayValueU8(
        #[br(count = data_size as usize / mem::size_of::<u8>())]
        #[bw(align_after = 8)]
        Vec<u8>,
    ),
    #[br(pre_assert(*data_type == MatFileDataTypes::MiINT8))]
    ArrayValueI8(
        #[br(count = data_size as usize / mem::size_of::<i8>())]
        #[bw(align_after = 8)]
        Vec<i8>,
    ),
    #[br(pre_assert(*data_type == MatFileDataTypes::MiUINT16))]
    ArrayValueU16(
        #[br(count = data_size as usize / mem::size_of::<u16>())]
        #[bw(align_after = 8)]
        Vec<u16>,
    ),
    #[br(pre_assert(*data_type == MatFileDataTypes::MiINT16))]
    ArrayValueI16(
        #[br(count = data_size as usize / mem::size_of::<i16>())]
        #[bw(align_after = 8)]
        Vec<i16>,
    ),
    #[br(pre_assert(*data_type == MatFileDataTypes::MiUINT32))]
    ArrayValueU32(
        #[br(count = data_size as usize / mem::size_of::<u32>())]
        #[bw(align_after = 8)]
        Vec<u32>,
    ),
    #[br(pre_assert(*data_type == MatFileDataTypes::MiINT32))]
    ArrayValueI32(
        #[br(count = data_size as usize / mem::size_of::<i32>())]
        #[bw(align_after = 8)]
        Vec<i32>,
    ),
    #[br(pre_assert(*data_type == MatFileDataTypes::MiUINT64))]
    ArrayValueU64(
        #[br(count = data_size as usize / mem::size_of::<u64>())]
        #[bw(align_after = 8)]
        Vec<u64>,
    ),
    #[br(pre_assert(*data_type == MatFileDataTypes::MiINT64))]
    ArrayValueI64(
        #[br(count = data_size as usize / mem::size_of::<i64>())]
        #[bw(align_after = 8)]
        Vec<i64>,
    ),
    #[br(pre_assert(*data_type == MatFileDataTypes::MiSINGLE))]
    ArrayValueF32(
        #[br(count = data_size as usize / mem::size_of::<f32>())]
        #[bw(align_after = 8)]
        Vec<f32>,
    ),
    #[br(pre_assert(*data_type == MatFileDataTypes::MiDOUBLE))]
    ArrayValueF64(
        #[br(count = data_size as usize / mem::size_of::<f64>())]
        #[bw(align_after = 8)]
        Vec<f64>,
    ),
    #[br(pre_assert(*data_type == MatFileDataTypes::MiUTF8))]
    ArrayValueUTF8(
        #[br(count = data_size as usize / mem::size_of::<u8>())]
        #[bw(align_after = 8)]
        Vec<u8>,
    ),
    #[br(pre_assert(*data_type == MatFileDataTypes::MiUTF16))]
    ArrayValueUTF16(
        #[br(count = data_size as usize / mem::size_of::<u16>())]
        #[bw(align_after = 8)]
        Vec<u16>,
    ),
    // ArrayValueEmpty,
}

#[derive(Debug, Clone)]
pub enum ArrayDataValueVar {
    ArrayValueU8(Vec<u8>),
    ArrayValueI8(Vec<i8>),
    ArrayValueU16(Vec<u16>),
    ArrayValueI16(Vec<i16>),
    ArrayValueU32(Vec<u32>),
    ArrayValueI32(Vec<i32>),
    ArrayValueU64(Vec<u64>),
    ArrayValueI64(Vec<i64>),
    ArrayValueF32(Vec<f32>),
    ArrayValueF64(Vec<f64>),
    ArrayValueUTF8(Vec<char>),
    ArrayValueUTF16(Vec<char>),
    ArrayValueBOOL(Vec<bool>),
}

impl Display for ArrayDataValueVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
