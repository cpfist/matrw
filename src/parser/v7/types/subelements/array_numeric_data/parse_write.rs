use binrw::*;

use super::array_data_value::*;
use crate::parser::v7::flags::{MatFileDataTypes, MatlabArrayTypes};

#[parser(reader)]
pub fn parse_array_data(
    data_type: &MatFileDataTypes,
    data_size: u32,
    arr_type: MatlabArrayTypes,
    is_logical: bool,
) -> BinResult<ArrayDataValueVar> {
    let data = reader.read_le_args::<ArrayDataValueVarRaw>((data_type, data_size))?;

    use {ArrayDataValueVarRaw::*, MatlabArrayTypes::*};
    match (data, arr_type) {
        // u8
        (ArrayValueU8(v), MxUINT8CLASS) => {
            if is_logical {
                Ok(ArrayDataValueVar::ArrayValueBOOL(
                    v.iter().map(|&x| x != 0).collect(),
                ))
            } else {
                Ok(ArrayDataValueVar::ArrayValueU8(v))
            }
        }
        (ArrayValueU8(v), MxINT8CLASS) => Ok(ArrayDataValueVar::ArrayValueI8(
            v.iter().map(|&x| x as i8).collect(),
        )),
        (ArrayValueU8(v), MxUINT16CLASS) => Ok(ArrayDataValueVar::ArrayValueU16(
            v.iter().map(|&x| x as u16).collect(),
        )),
        (ArrayValueU8(v), MxINT16CLASS) => Ok(ArrayDataValueVar::ArrayValueI16(
            v.iter().map(|&x| x as i16).collect(),
        )),
        (ArrayValueU8(v), MxUINT32CLASS) => Ok(ArrayDataValueVar::ArrayValueU32(
            v.iter().map(|&x| x as u32).collect(),
        )),
        (ArrayValueU8(v), MxINT32CLASS) => Ok(ArrayDataValueVar::ArrayValueI32(
            v.iter().map(|&x| x as i32).collect(),
        )),
        (ArrayValueU8(v), MxUINT64CLASS) => Ok(ArrayDataValueVar::ArrayValueU64(
            v.iter().map(|&x| x as u64).collect(),
        )),
        (ArrayValueU8(v), MxINT64CLASS) => Ok(ArrayDataValueVar::ArrayValueI64(
            v.iter().map(|&x| x as i64).collect(),
        )),
        (ArrayValueU8(v), MxSINGLECLASS) => Ok(ArrayDataValueVar::ArrayValueF32(
            v.iter().map(|&x| x as f32).collect(),
        )),
        (ArrayValueU8(v), MxDOUBLECLASS) => Ok(ArrayDataValueVar::ArrayValueF64(
            v.iter().map(|&x| x as f64).collect(),
        )),
        // i8
        (ArrayValueI8(v), MxUINT8CLASS) => Ok(ArrayDataValueVar::ArrayValueU8(
            v.iter().map(|&x| x as u8).collect(),
        )),
        (ArrayValueI8(v), MxINT8CLASS) => Ok(ArrayDataValueVar::ArrayValueI8(v)),
        (ArrayValueI8(v), MxUINT16CLASS) => Ok(ArrayDataValueVar::ArrayValueU16(
            v.iter().map(|&x| x as u16).collect(),
        )),
        (ArrayValueI8(v), MxINT16CLASS) => Ok(ArrayDataValueVar::ArrayValueI16(
            v.iter().map(|&x| x as i16).collect(),
        )),
        (ArrayValueI8(v), MxUINT32CLASS) => Ok(ArrayDataValueVar::ArrayValueU32(
            v.iter().map(|&x| x as u32).collect(),
        )),
        (ArrayValueI8(v), MxINT32CLASS) => Ok(ArrayDataValueVar::ArrayValueI32(
            v.iter().map(|&x| x as i32).collect(),
        )),
        (ArrayValueI8(v), MxUINT64CLASS) => Ok(ArrayDataValueVar::ArrayValueU64(
            v.iter().map(|&x| x as u64).collect(),
        )),
        (ArrayValueI8(v), MxINT64CLASS) => Ok(ArrayDataValueVar::ArrayValueI64(
            v.iter().map(|&x| x as i64).collect(),
        )),
        (ArrayValueI8(v), MxSINGLECLASS) => Ok(ArrayDataValueVar::ArrayValueF32(
            v.iter().map(|&x| x as f32).collect(),
        )),
        (ArrayValueI8(v), MxDOUBLECLASS) => Ok(ArrayDataValueVar::ArrayValueF64(
            v.iter().map(|&x| x as f64).collect(),
        )),
        // u16
        (ArrayValueU16(v), MxUINT8CLASS) => Ok(ArrayDataValueVar::ArrayValueU8(
            v.iter().map(|&x| x as u8).collect(),
        )),
        (ArrayValueU16(v), MxINT8CLASS) => Ok(ArrayDataValueVar::ArrayValueI8(
            v.iter().map(|&x| x as i8).collect(),
        )),
        (ArrayValueU16(v), MxUINT16CLASS) => Ok(ArrayDataValueVar::ArrayValueU16(v)),
        (ArrayValueU16(v), MxINT16CLASS) => Ok(ArrayDataValueVar::ArrayValueI16(
            v.iter().map(|&x| x as i16).collect(),
        )),
        (ArrayValueU16(v), MxUINT32CLASS) => Ok(ArrayDataValueVar::ArrayValueU32(
            v.iter().map(|&x| x as u32).collect(),
        )),
        (ArrayValueU16(v), MxINT32CLASS) => Ok(ArrayDataValueVar::ArrayValueI32(
            v.iter().map(|&x| x as i32).collect(),
        )),
        (ArrayValueU16(v), MxUINT64CLASS) => Ok(ArrayDataValueVar::ArrayValueU64(
            v.iter().map(|&x| x as u64).collect(),
        )),
        (ArrayValueU16(v), MxINT64CLASS) => Ok(ArrayDataValueVar::ArrayValueI64(
            v.iter().map(|&x| x as i64).collect(),
        )),
        (ArrayValueU16(v), MxSINGLECLASS) => Ok(ArrayDataValueVar::ArrayValueF32(
            v.iter().map(|&x| x as f32).collect(),
        )),
        (ArrayValueU16(v), MxDOUBLECLASS) => Ok(ArrayDataValueVar::ArrayValueF64(
            v.iter().map(|&x| x as f64).collect(),
        )),
        // i16
        (ArrayValueI16(v), MxUINT8CLASS) => Ok(ArrayDataValueVar::ArrayValueU8(
            v.iter().map(|&x| x as u8).collect(),
        )),
        (ArrayValueI16(v), MxINT8CLASS) => Ok(ArrayDataValueVar::ArrayValueI8(
            v.iter().map(|&x| x as i8).collect(),
        )),
        (ArrayValueI16(v), MxUINT16CLASS) => Ok(ArrayDataValueVar::ArrayValueU16(
            v.iter().map(|&x| x as u16).collect(),
        )),
        (ArrayValueI16(v), MxINT16CLASS) => Ok(ArrayDataValueVar::ArrayValueI16(v)),
        (ArrayValueI16(v), MxUINT32CLASS) => Ok(ArrayDataValueVar::ArrayValueU32(
            v.iter().map(|&x| x as u32).collect(),
        )),
        (ArrayValueI16(v), MxINT32CLASS) => Ok(ArrayDataValueVar::ArrayValueI32(
            v.iter().map(|&x| x as i32).collect(),
        )),
        (ArrayValueI16(v), MxUINT64CLASS) => Ok(ArrayDataValueVar::ArrayValueU64(
            v.iter().map(|&x| x as u64).collect(),
        )),
        (ArrayValueI16(v), MxINT64CLASS) => Ok(ArrayDataValueVar::ArrayValueI64(
            v.iter().map(|&x| x as i64).collect(),
        )),
        (ArrayValueI16(v), MxSINGLECLASS) => Ok(ArrayDataValueVar::ArrayValueF32(
            v.iter().map(|&x| x as f32).collect(),
        )),
        (ArrayValueI16(v), MxDOUBLECLASS) => Ok(ArrayDataValueVar::ArrayValueF64(
            v.iter().map(|&x| x as f64).collect(),
        )),
        // u32
        (ArrayValueU32(v), MxUINT8CLASS) => Ok(ArrayDataValueVar::ArrayValueU8(
            v.iter().map(|&x| x as u8).collect(),
        )),
        (ArrayValueU32(v), MxINT8CLASS) => Ok(ArrayDataValueVar::ArrayValueI8(
            v.iter().map(|&x| x as i8).collect(),
        )),
        (ArrayValueU32(v), MxUINT16CLASS) => Ok(ArrayDataValueVar::ArrayValueU16(
            v.iter().map(|&x| x as u16).collect(),
        )),
        (ArrayValueU32(v), MxINT16CLASS) => Ok(ArrayDataValueVar::ArrayValueI16(
            v.iter().map(|&x| x as i16).collect(),
        )),
        (ArrayValueU32(v), MxUINT32CLASS) => Ok(ArrayDataValueVar::ArrayValueU32(v)),
        (ArrayValueU32(v), MxINT32CLASS) => Ok(ArrayDataValueVar::ArrayValueI32(
            v.iter().map(|&x| x as i32).collect(),
        )),
        (ArrayValueU32(v), MxUINT64CLASS) => Ok(ArrayDataValueVar::ArrayValueU64(
            v.iter().map(|&x| x as u64).collect(),
        )),
        (ArrayValueU32(v), MxINT64CLASS) => Ok(ArrayDataValueVar::ArrayValueI64(
            v.iter().map(|&x| x as i64).collect(),
        )),
        (ArrayValueU32(v), MxSINGLECLASS) => Ok(ArrayDataValueVar::ArrayValueF32(
            v.iter().map(|&x| x as f32).collect(),
        )),
        (ArrayValueU32(v), MxDOUBLECLASS) => Ok(ArrayDataValueVar::ArrayValueF64(
            v.iter().map(|&x| x as f64).collect(),
        )),
        // i32
        (ArrayValueI32(v), MxUINT8CLASS) => Ok(ArrayDataValueVar::ArrayValueU8(
            v.iter().map(|&x| x as u8).collect(),
        )),
        (ArrayValueI32(v), MxINT8CLASS) => Ok(ArrayDataValueVar::ArrayValueI8(
            v.iter().map(|&x| x as i8).collect(),
        )),
        (ArrayValueI32(v), MxUINT16CLASS) => Ok(ArrayDataValueVar::ArrayValueU16(
            v.iter().map(|&x| x as u16).collect(),
        )),
        (ArrayValueI32(v), MxINT16CLASS) => Ok(ArrayDataValueVar::ArrayValueI16(
            v.iter().map(|&x| x as i16).collect(),
        )),
        (ArrayValueI32(v), MxUINT32CLASS) => Ok(ArrayDataValueVar::ArrayValueU32(
            v.iter().map(|&x| x as u32).collect(),
        )),
        (ArrayValueI32(v), MxINT32CLASS) => Ok(ArrayDataValueVar::ArrayValueI32(v)),
        (ArrayValueI32(v), MxUINT64CLASS) => Ok(ArrayDataValueVar::ArrayValueU64(
            v.iter().map(|&x| x as u64).collect(),
        )),
        (ArrayValueI32(v), MxINT64CLASS) => Ok(ArrayDataValueVar::ArrayValueI64(
            v.iter().map(|&x| x as i64).collect(),
        )),
        (ArrayValueI32(v), MxSINGLECLASS) => Ok(ArrayDataValueVar::ArrayValueF32(
            v.iter().map(|&x| x as f32).collect(),
        )),
        (ArrayValueI32(v), MxDOUBLECLASS) => Ok(ArrayDataValueVar::ArrayValueF64(
            v.iter().map(|&x| x as f64).collect(),
        )),
        // u64
        (ArrayValueU64(v), MxUINT8CLASS) => Ok(ArrayDataValueVar::ArrayValueU8(
            v.iter().map(|&x| x as u8).collect(),
        )),
        (ArrayValueU64(v), MxINT8CLASS) => Ok(ArrayDataValueVar::ArrayValueI8(
            v.iter().map(|&x| x as i8).collect(),
        )),
        (ArrayValueU64(v), MxUINT16CLASS) => Ok(ArrayDataValueVar::ArrayValueU16(
            v.iter().map(|&x| x as u16).collect(),
        )),
        (ArrayValueU64(v), MxINT16CLASS) => Ok(ArrayDataValueVar::ArrayValueI16(
            v.iter().map(|&x| x as i16).collect(),
        )),
        (ArrayValueU64(v), MxUINT32CLASS) => Ok(ArrayDataValueVar::ArrayValueU32(
            v.iter().map(|&x| x as u32).collect(),
        )),
        (ArrayValueU64(v), MxINT32CLASS) => Ok(ArrayDataValueVar::ArrayValueI32(
            v.iter().map(|&x| x as i32).collect(),
        )),
        (ArrayValueU64(v), MxUINT64CLASS) => Ok(ArrayDataValueVar::ArrayValueU64(v)),
        (ArrayValueU64(v), MxINT64CLASS) => Ok(ArrayDataValueVar::ArrayValueI64(
            v.iter().map(|&x| x as i64).collect(),
        )),
        (ArrayValueU64(v), MxSINGLECLASS) => Ok(ArrayDataValueVar::ArrayValueF32(
            v.iter().map(|&x| x as f32).collect(),
        )),
        (ArrayValueU64(v), MxDOUBLECLASS) => Ok(ArrayDataValueVar::ArrayValueF64(
            v.iter().map(|&x| x as f64).collect(),
        )),
        // i64
        (ArrayValueI64(v), MxUINT8CLASS) => Ok(ArrayDataValueVar::ArrayValueU8(
            v.iter().map(|&x| x as u8).collect(),
        )),
        (ArrayValueI64(v), MxINT8CLASS) => Ok(ArrayDataValueVar::ArrayValueI8(
            v.iter().map(|&x| x as i8).collect(),
        )),
        (ArrayValueI64(v), MxUINT16CLASS) => Ok(ArrayDataValueVar::ArrayValueU16(
            v.iter().map(|&x| x as u16).collect(),
        )),
        (ArrayValueI64(v), MxINT16CLASS) => Ok(ArrayDataValueVar::ArrayValueI16(
            v.iter().map(|&x| x as i16).collect(),
        )),
        (ArrayValueI64(v), MxUINT32CLASS) => Ok(ArrayDataValueVar::ArrayValueU32(
            v.iter().map(|&x| x as u32).collect(),
        )),
        (ArrayValueI64(v), MxINT32CLASS) => Ok(ArrayDataValueVar::ArrayValueI32(
            v.iter().map(|&x| x as i32).collect(),
        )),
        (ArrayValueI64(v), MxUINT64CLASS) => Ok(ArrayDataValueVar::ArrayValueU64(
            v.iter().map(|&x| x as u64).collect(),
        )),
        (ArrayValueI64(v), MxINT64CLASS) => Ok(ArrayDataValueVar::ArrayValueI64(v)),
        (ArrayValueI64(v), MxSINGLECLASS) => Ok(ArrayDataValueVar::ArrayValueF32(
            v.iter().map(|&x| x as f32).collect(),
        )),
        (ArrayValueI64(v), MxDOUBLECLASS) => Ok(ArrayDataValueVar::ArrayValueF64(
            v.iter().map(|&x| x as f64).collect(),
        )),
        // f32
        (ArrayValueF32(v), MxUINT8CLASS) => Ok(ArrayDataValueVar::ArrayValueU8(
            v.iter().map(|&x| x as u8).collect(),
        )),
        (ArrayValueF32(v), MxINT8CLASS) => Ok(ArrayDataValueVar::ArrayValueI8(
            v.iter().map(|&x| x as i8).collect(),
        )),
        (ArrayValueF32(v), MxUINT16CLASS) => Ok(ArrayDataValueVar::ArrayValueU16(
            v.iter().map(|&x| x as u16).collect(),
        )),
        (ArrayValueF32(v), MxINT16CLASS) => Ok(ArrayDataValueVar::ArrayValueI16(
            v.iter().map(|&x| x as i16).collect(),
        )),
        (ArrayValueF32(v), MxUINT32CLASS) => Ok(ArrayDataValueVar::ArrayValueU32(
            v.iter().map(|&x| x as u32).collect(),
        )),
        (ArrayValueF32(v), MxINT32CLASS) => Ok(ArrayDataValueVar::ArrayValueI32(
            v.iter().map(|&x| x as i32).collect(),
        )),
        (ArrayValueF32(v), MxUINT64CLASS) => Ok(ArrayDataValueVar::ArrayValueU64(
            v.iter().map(|&x| x as u64).collect(),
        )),
        (ArrayValueF32(v), MxINT64CLASS) => Ok(ArrayDataValueVar::ArrayValueI64(
            v.iter().map(|&x| x as i64).collect(),
        )),
        (ArrayValueF32(v), MxSINGLECLASS) => Ok(ArrayDataValueVar::ArrayValueF32(v)),
        (ArrayValueF32(v), MxDOUBLECLASS) => Ok(ArrayDataValueVar::ArrayValueF64(
            v.iter().map(|&x| x as f64).collect(),
        )),
        // f64
        (ArrayValueF64(v), MxUINT8CLASS) => Ok(ArrayDataValueVar::ArrayValueU8(
            v.iter().map(|&x| x as u8).collect(),
        )),
        (ArrayValueF64(v), MxINT8CLASS) => Ok(ArrayDataValueVar::ArrayValueI8(
            v.iter().map(|&x| x as i8).collect(),
        )),
        (ArrayValueF64(v), MxUINT16CLASS) => Ok(ArrayDataValueVar::ArrayValueU16(
            v.iter().map(|&x| x as u16).collect(),
        )),
        (ArrayValueF64(v), MxINT16CLASS) => Ok(ArrayDataValueVar::ArrayValueI16(
            v.iter().map(|&x| x as i16).collect(),
        )),
        (ArrayValueF64(v), MxUINT32CLASS) => Ok(ArrayDataValueVar::ArrayValueU32(
            v.iter().map(|&x| x as u32).collect(),
        )),
        (ArrayValueF64(v), MxINT32CLASS) => Ok(ArrayDataValueVar::ArrayValueI32(
            v.iter().map(|&x| x as i32).collect(),
        )),
        (ArrayValueF64(v), MxUINT64CLASS) => Ok(ArrayDataValueVar::ArrayValueU64(
            v.iter().map(|&x| x as u64).collect(),
        )),
        (ArrayValueF64(v), MxINT64CLASS) => Ok(ArrayDataValueVar::ArrayValueI64(
            v.iter().map(|&x| x as i64).collect(),
        )),
        (ArrayValueF64(v), MxSINGLECLASS) => Ok(ArrayDataValueVar::ArrayValueF32(
            v.iter().map(|&x| x as f32).collect(),
        )),
        (ArrayValueF64(v), MxDOUBLECLASS) => Ok(ArrayDataValueVar::ArrayValueF64(v)),
        // utf8
        (ArrayValueUTF8(v), MxCHARCLASS) => Ok(ArrayDataValueVar::ArrayValueUTF8(
            String::from_utf8(v).unwrap().chars().collect(),
        )),
        // utf16
        (ArrayValueUTF16(v), MxCHARCLASS) => Ok(ArrayDataValueVar::ArrayValueUTF16(
            String::from_utf16(&v).unwrap().chars().collect(),
        )),
        //
        _ => Err(Error::NoVariantMatch {
            pos: reader.stream_position()?,
        }),
    }
}

#[parser(reader)]
pub fn parse_array_data_sparse(
    data_type: &MatFileDataTypes,
    data_size: u32,
    is_logical: bool,
) -> BinResult<ArrayDataValueVar> {
    let data = reader.read_le_args::<ArrayDataValueVarRaw>((data_type, data_size))?;

    use ArrayDataValueVarRaw::*;
    match data {
        ArrayValueU8(v) => {
            if is_logical {
                Ok(ArrayDataValueVar::ArrayValueBOOL(
                    v.iter().map(|&x| x != 0).collect(),
                ))
            } else {
                Ok(ArrayDataValueVar::ArrayValueU8(v))
            }
        }
        ArrayValueF64(v) => Ok(ArrayDataValueVar::ArrayValueF64(v)),
        _ => panic!(),
    }
}

#[binrw::writer(writer, endian)]
pub fn write_array_data(value: &ArrayDataValueVar) -> BinResult<()> {
    use ArrayDataValueVar::*;
    let raw_data: ArrayDataValueVarRaw = match value {
        ArrayValueU8(v) => ArrayDataValueVarRaw::ArrayValueU8(v.to_owned()),
        ArrayValueI8(v) => ArrayDataValueVarRaw::ArrayValueI8(v.to_owned()),
        ArrayValueU16(v) => ArrayDataValueVarRaw::ArrayValueU16(v.to_owned()),
        ArrayValueI16(v) => ArrayDataValueVarRaw::ArrayValueI16(v.to_owned()),
        ArrayValueU32(v) => ArrayDataValueVarRaw::ArrayValueU32(v.to_owned()),
        ArrayValueI32(v) => ArrayDataValueVarRaw::ArrayValueI32(v.to_owned()),
        ArrayValueU64(v) => ArrayDataValueVarRaw::ArrayValueU64(v.to_owned()),
        ArrayValueI64(v) => ArrayDataValueVarRaw::ArrayValueI64(v.to_owned()),
        ArrayValueF32(v) => ArrayDataValueVarRaw::ArrayValueF32(v.to_owned()),
        ArrayValueF64(v) => ArrayDataValueVarRaw::ArrayValueF64(v.to_owned()),
        ArrayValueUTF8(v) => {
            ArrayDataValueVarRaw::ArrayValueU8(v.iter().flat_map(|c| c.to_string().into_bytes()).collect())
        }
        ArrayValueUTF16(v) => ArrayDataValueVarRaw::ArrayValueU16(
            v.iter()
                .flat_map(|c| c.to_string().into_bytes())
                .collect::<Vec<u8>>()
                .iter()
                .map(|x| *x as u16)
                .collect(),
        ),
        ArrayValueBOOL(v) => ArrayDataValueVarRaw::ArrayValueU8(v.iter().map(|x| *x as u8).collect()),
    };

    raw_data.write_options(writer, endian, ())
}
