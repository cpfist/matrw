use binrw::*;

use crate::parser::v7::flags::MatFileDataTypes;

#[binrw]
#[derive(Debug, Clone)]
#[br(assert(data_type == MatFileDataTypes::MiMATRIX))]
pub struct Empty7 {
    #[brw(pad_size_to = 4)]
    data_type: MatFileDataTypes,
    num_bytes: u32,
}
