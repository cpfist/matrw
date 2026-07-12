use binrw::*;

use crate::parser::v4::flags::{Endian, MatFileDataTypes, MatFileMatrixTypes};

#[parser(reader)]
pub fn parse_endian(flags: u32) -> BinResult<Endian> {
    let flag = flags / 1000;
    match flag {
        0 => Ok(Endian::Little),
        1 => Ok(Endian::Big),
        _ => Err(Error::BadMagic {
            pos: reader.stream_position()?,
            found: Box::new(flag),
        }),
    }
}

#[parser(reader)]
pub fn parse_data_type(flags: u32) -> BinResult<MatFileDataTypes> {
    let flag = ((flags % 1000) % 100) / 10;
    match flag {
        0 => Ok(MatFileDataTypes::MiDOUBLE),
        1 => Ok(MatFileDataTypes::MiSINGLE),
        2 => Ok(MatFileDataTypes::MiINT32),
        3 => Ok(MatFileDataTypes::MiINT16),
        4 => Ok(MatFileDataTypes::MiUINT16),
        5 => Ok(MatFileDataTypes::MiUINT8),
        _ => Err(Error::BadMagic {
            pos: reader.stream_position()?,
            found: Box::new(flag),
        }),
    }
}

#[parser(reader)]
pub fn parse_matrix_type(flags: u32) -> BinResult<MatFileMatrixTypes> {
    let flag = ((flags % 1000) % 100) % 10;
    match flag {
        0 => Ok(MatFileMatrixTypes::Full),
        1 => Ok(MatFileMatrixTypes::Text),
        2 => Ok(MatFileMatrixTypes::Sparse),
        _ => Err(Error::BadMagic {
            pos: reader.stream_position()?,
            found: Box::new(flag),
        }),
    }
}
