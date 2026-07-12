use binrw::*;

use crate::parser::v4::flags::{Endian, MatFileDataTypes, MatFileMatrixTypes};
use crate::parser::v4::types::parsing_util::{parse_endian, parse_data_type, parse_matrix_type};

#[binrw]
#[derive(Debug, Clone)]
pub struct MatVariableHeader4 {
    _flags: u32,
    #[br(parse_with = parse_endian, args(_flags))]
    #[bw(ignore)]
    pub endian: Endian,
    #[br(parse_with = parse_data_type, args(_flags))]
    #[bw(ignore)]
    pub data_type: MatFileDataTypes,
    #[br(parse_with = parse_matrix_type, args(_flags))]
    #[bw(ignore)]
    pub matrix_type: MatFileMatrixTypes,
    pub mrows: u32,
    pub ncols: u32,
    pub imagf: u32,
    pub namlen: u32,
}

impl MatVariableHeader4 {
    pub fn new(
        endian: Endian, 
        data_type: MatFileDataTypes, 
        matrix_type: MatFileMatrixTypes, 
        mrows: u32, 
        ncols: u32, 
        imagf: u32,
        namlen: u32,
    ) -> Self {
        let _flags = 
            1000 * endian as u32 + 
            10 * data_type as u32 +
            matrix_type as u32;

        Self {
            _flags,
            endian,
            data_type,
            matrix_type,
            mrows,
            ncols,
            imagf,
            namlen,
        }
    }
}
