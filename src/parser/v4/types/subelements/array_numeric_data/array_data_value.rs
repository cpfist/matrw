use binrw::*;

use crate::parser::v4::flags::{MatFileDataTypes, MatFileMatrixTypes};

#[binrw]
#[derive(Debug, Clone)]
#[br(import(data_type: &MatFileDataTypes, data_size: usize))]
enum ArrayDataValueVarRaw {
    #[br(pre_assert(*data_type == MatFileDataTypes::MiDOUBLE))]
    ArrayValueF64(#[br(count = data_size)] Vec<f64>),
    #[br(pre_assert(*data_type == MatFileDataTypes::MiSINGLE))]
    ArrayValueF32(#[br(count = data_size)] Vec<f32>),
    #[br(pre_assert(*data_type == MatFileDataTypes::MiINT32))]
    ArrayValueI32(#[br(count = data_size)] Vec<i32>),
    #[br(pre_assert(*data_type == MatFileDataTypes::MiINT16))]
    ArrayValueI16(#[br(count = data_size)] Vec<i16>),
    #[br(pre_assert(*data_type == MatFileDataTypes::MiUINT16))]
    ArrayValueU16(#[br(count = data_size)] Vec<u16>),
    #[br(pre_assert(*data_type == MatFileDataTypes::MiUINT8))]
    ArrayValueU8(#[br(count = data_size)] Vec<u8>),
}

#[derive(Debug, Clone)]
pub enum ArrayDataValueVar {
    ArrayValueF64(Vec<f64>),
    ArrayValueF32(Vec<f32>),
    ArrayValueI32(Vec<i32>),
    ArrayValueI16(Vec<i16>),
    ArrayValueU16(Vec<u16>),
    ArrayValueU8(Vec<u8>),
    ArrayValueUTF8(Vec<char>),
}

#[parser(reader)]
pub fn parse_array_data(
    data_type: &MatFileDataTypes,
    matrix_type: &MatFileMatrixTypes,
    data_size: usize,
) -> BinResult<ArrayDataValueVar> {
    let data = reader.read_le_args::<ArrayDataValueVarRaw>((data_type, data_size))?;

    use {ArrayDataValueVarRaw::*, MatFileMatrixTypes::*};
    match (data, matrix_type) {
        (ArrayValueU8(v), Full) => Ok(ArrayDataValueVar::ArrayValueU8(v)),
        (ArrayValueU16(v), Full) => Ok(ArrayDataValueVar::ArrayValueU16(v)),
        (ArrayValueI16(v), Full) => Ok(ArrayDataValueVar::ArrayValueI16(v)),
        (ArrayValueI32(v), Full) => Ok(ArrayDataValueVar::ArrayValueI32(v)),
        (ArrayValueF32(v), Full) => Ok(ArrayDataValueVar::ArrayValueF32(v)),
        (ArrayValueF64(v), Full) => Ok(ArrayDataValueVar::ArrayValueF64(v)),
        (ArrayValueU8(v), Text) => Ok(ArrayDataValueVar::ArrayValueUTF8(
            String::from_utf8(v).unwrap().chars().collect(),
        )),
        (ArrayValueF64(v), Text) => {
            let v_conv = v.iter().map(|&x| x as u8).collect();
            Ok(ArrayDataValueVar::ArrayValueUTF8(
                String::from_utf8(v_conv).unwrap().chars().collect(),
            ))
        }
        _ => Err(Error::NoVariantMatch {
            pos: reader.stream_position()?,
        }),
    }
}

#[binrw::writer(writer, endian)]
pub fn write_array_data(value: &ArrayDataValueVar) -> BinResult<()> {
    use ArrayDataValueVar::*;
    let raw_data: ArrayDataValueVarRaw = match value {
        ArrayValueU8(v) => ArrayDataValueVarRaw::ArrayValueU8(v.to_owned()),
        ArrayValueU16(v) => ArrayDataValueVarRaw::ArrayValueU16(v.to_owned()),
        ArrayValueI16(v) => ArrayDataValueVarRaw::ArrayValueI16(v.to_owned()),
        ArrayValueI32(v) => ArrayDataValueVarRaw::ArrayValueI32(v.to_owned()),
        ArrayValueF32(v) => ArrayDataValueVarRaw::ArrayValueF32(v.to_owned()),
        ArrayValueF64(v) => ArrayDataValueVarRaw::ArrayValueF64(v.to_owned()),
        ArrayValueUTF8(v) => {
            ArrayDataValueVarRaw::ArrayValueU8(v.iter().flat_map(|c| c.to_string().into_bytes()).collect())
        }
    };

    raw_data.write_options(writer, endian, ())
}
