//! Module containing types for matching *Array Data Subelements*.

use binrw::*;
use std::fmt::Debug;

use super::array_data_normal::{ArrayDataNormal, ArrayDataSparseNormal};
use super::array_data_small::{ArrayDataSmall, ArrayDataSparseSmall};
use super::array_data_value::*;
use crate::parser::v7::flags::{MatFileDataTypes, MatlabArrayTypes};

#[binrw]
#[derive(Debug, Clone)]
#[br(import(arrtype: MatlabArrayTypes, is_logical: bool))]
pub enum ArrayData {
    DataNormal(#[br(args(arrtype, is_logical))] ArrayDataNormal),
    DataSmall(#[br(args(arrtype, is_logical))] ArrayDataSmall),
}

impl ArrayData {
    pub fn array_data_value_var(self) -> ArrayDataValueVar {
        match self {
            ArrayData::DataNormal(d) => d.value,
            ArrayData::DataSmall(d) => d.value,
        }
    }
}

pub trait ArrayDataNew<T> {
    #[allow(clippy::new_ret_no_self)]
    fn new(value: Vec<T>) -> ArrayData;
}

macro_rules! impl_ArrayDataNew {
    ($t1: ty, $t2: ident, $t3: ident, $t4: expr) => {
        impl ArrayDataNew<$t1> for ArrayData {
            fn new(value: Vec<$t1>) -> ArrayData {
                let nelem = value.len();

                let data_type = MatFileDataTypes::$t2;
                let data_size = std::mem::size_of::<$t1>() * nelem;
                let value_wrapped = ArrayDataValueVar::$t3(value);

                if nelem > $t4 {
                    ArrayData::DataNormal(ArrayDataNormal::new(
                        data_type,
                        data_size as u32,
                        value_wrapped,
                    ))
                } else {
                    ArrayData::DataSmall(ArrayDataSmall::new(
                        data_type,
                        data_size as u16,
                        value_wrapped,
                    ))
                }
            }
        }
    };
}

impl_ArrayDataNew!(u8, MiUINT8, ArrayValueU8, 4);
impl_ArrayDataNew!(i8, MiINT8, ArrayValueI8, 4);
impl_ArrayDataNew!(u16, MiUINT16, ArrayValueU16, 2);
impl_ArrayDataNew!(i16, MiINT16, ArrayValueI16, 2);
impl_ArrayDataNew!(u32, MiUINT32, ArrayValueU32, 1);
impl_ArrayDataNew!(i32, MiINT32, ArrayValueI32, 1);
impl_ArrayDataNew!(u64, MiUINT64, ArrayValueU64, 0);
impl_ArrayDataNew!(i64, MiINT64, ArrayValueI64, 0);

impl ArrayDataNew<bool> for ArrayData {
    fn new(value: Vec<bool>) -> ArrayData {
        let nelem = value.len();

        let data_type = MatFileDataTypes::MiUINT8;
        let data_size = std::mem::size_of::<bool>() * nelem;
        let value_wrapped = ArrayDataValueVar::ArrayValueBOOL(value);

        if nelem > 4 {
            ArrayData::DataNormal(ArrayDataNormal::new(data_type, data_size as u32, value_wrapped))
        } else {
            ArrayData::DataSmall(ArrayDataSmall::new(data_type, data_size as u16, value_wrapped))
        }
    }
}

impl ArrayDataNew<char> for ArrayData {
    fn new(value: Vec<char>) -> ArrayData {
        let v: Vec<char> = value.into_iter().filter(|c| c.is_ascii()).collect();

        let nelem = v.len();

        let data_type = MatFileDataTypes::MiUTF8;
        let data_size = std::mem::size_of::<u8>() * nelem;
        let value_wrapped = ArrayDataValueVar::ArrayValueUTF8(v);

        if nelem > 4 {
            ArrayData::DataNormal(ArrayDataNormal::new(data_type, data_size as u32, value_wrapped))
        } else {
            ArrayData::DataSmall(ArrayDataSmall::new(data_type, data_size as u16, value_wrapped))
        }
    }
}

impl ArrayDataNew<f32> for ArrayData {
    fn new(value: Vec<f32>) -> ArrayData {
        let nelem = value.len();

        let data_type = MatFileDataTypes::MiSINGLE;
        let data_size = std::mem::size_of::<f32>() * nelem;
        let value_wrapped = ArrayDataValueVar::ArrayValueF32(value);

        if nelem > 1 {
            ArrayData::DataNormal(ArrayDataNormal::new(data_type, data_size as u32, value_wrapped))
        } else {
            ArrayData::DataSmall(ArrayDataSmall::new(data_type, data_size as u16, value_wrapped))
        }
    }
}

impl ArrayDataNew<f64> for ArrayData {
    fn new(value: Vec<f64>) -> ArrayData {
        let mut can_be_u8 = true;
        let mut can_be_i8 = true;
        let mut can_be_u16 = true;
        let mut can_be_i16 = true;
        let mut can_be_u32 = true;
        let mut can_be_i32 = true;

        // Test for each f64 array element, if it can be represented by a 
        // smaller integer type.
        for e in &value {
            if e.fract() != 0.0 {
                // Check if any element has fractional part
                can_be_u8 = false;
                can_be_i8 = false;
                can_be_u16 = false;
                can_be_i16 = false;
                can_be_u32 = false;
                can_be_i32 = false;

                break;
            }

            // Check u8 bounds
            if can_be_u8 && (*e < u8::MIN as f64 || *e > u8::MAX as f64) {
                can_be_u8 = false;
            }
            // Check i8 bounds
            if can_be_i8 && (*e < i8::MIN as f64 || *e > i8::MAX as f64) {
                can_be_i8 = false;
            }
            // Check u16 bounds
            if can_be_u16 && (*e < u16::MIN as f64 || *e > u16::MAX as f64) {
                can_be_u16 = false;
            }
            // Check i16 bounds
            if can_be_i16 && (*e < i16::MIN as f64 || *e > i16::MAX as f64) {
                can_be_i16 = false;
            }
            // Check u32 bounds
            if can_be_u32 && (*e < u32::MIN as f64 || *e > u32::MAX as f64) {
                can_be_u32 = false;
            }
            // Check i32 bounds
            if can_be_i32 && (*e < i32::MIN as f64 || *e > i32::MAX as f64) {
                can_be_i32 = false;
            }

            // Early exit if no integer possible
            if !can_be_u8 && !can_be_i8 && !can_be_u16 && !can_be_i16 && !can_be_u32 && !can_be_i32 {
                break;
            }
        }

        let (data_type, value_wrapped, data_size, is_normal) = if can_be_u8 {
            let data_type = MatFileDataTypes::MiUINT8;
            let value_new: Vec<u8> = value.iter().map(|&x| x as u8).collect();
            let nelem = value_new.len();
            let data_size = std::mem::size_of::<u8>() * nelem;
            let is_normal = nelem > 4;
            (
                data_type,
                ArrayDataValueVar::ArrayValueU8(value_new),
                data_size,
                is_normal,
            )
        } else if can_be_i8 {
            let data_type = MatFileDataTypes::MiINT8;
            let value_new: Vec<i8> = value.iter().map(|&x| x as i8).collect();
            let nelem = value_new.len();
            let data_size = std::mem::size_of::<u8>() * nelem;
            let is_normal = nelem > 4;
            (
                data_type,
                ArrayDataValueVar::ArrayValueI8(value_new),
                data_size,
                is_normal,
            )
        } else if can_be_u16 {
            let data_type = MatFileDataTypes::MiUINT16;
            let value_new: Vec<u16> = value.iter().map(|&x| x as u16).collect();
            let nelem = value_new.len();
            let data_size = std::mem::size_of::<u16>() * nelem;
            let is_normal = nelem > 2;
            (
                data_type,
                ArrayDataValueVar::ArrayValueU16(value_new),
                data_size,
                is_normal,
            )
        } else if can_be_i16 {
            let data_type = MatFileDataTypes::MiINT16;
            let value_new: Vec<i16> = value.iter().map(|&x| x as i16).collect();
            let nelem = value_new.len();
            let data_size = std::mem::size_of::<i16>() * nelem;
            let is_normal = nelem > 2;
            (
                data_type,
                ArrayDataValueVar::ArrayValueI16(value_new),
                data_size,
                is_normal,
            )
        } else if can_be_u32 {
            let data_type = MatFileDataTypes::MiUINT32;
            let value_new: Vec<u32> = value.iter().map(|&x| x as u32).collect();
            let nelem = value_new.len();
            let data_size = std::mem::size_of::<u32>() * nelem;
            let is_normal = nelem > 1;
            (
                data_type,
                ArrayDataValueVar::ArrayValueU32(value_new),
                data_size,
                is_normal,
            )
        } else if can_be_i32 {
            let data_type = MatFileDataTypes::MiINT32;
            let value_new: Vec<i32> = value.iter().map(|&x| x as i32).collect();
            let nelem = value_new.len();
            let data_size = std::mem::size_of::<i32>() * nelem;
            let is_normal = nelem > 1;
            (
                data_type,
                ArrayDataValueVar::ArrayValueI32(value_new),
                data_size,
                is_normal,
            )
        } else {
            let data_type = MatFileDataTypes::MiDOUBLE;
            let nelem = value.len();
            let data_size = std::mem::size_of::<f64>() * nelem;
            let is_normal = nelem > 0;
            (
                data_type,
                ArrayDataValueVar::ArrayValueF64(value),
                data_size,
                is_normal,
            )
        };

        if is_normal {
            ArrayData::DataNormal(ArrayDataNormal::new(data_type, data_size as u32, value_wrapped))
        } else {
            ArrayData::DataSmall(ArrayDataSmall::new(data_type, data_size as u16, value_wrapped))
        }
    }
}

impl ArrayData {
    pub fn size(&self) -> u32 {
        match self {
            ArrayData::DataNormal(v) => {
                let padding = if (v.data_size % 8) == 0 {
                    0
                } else {
                    8 - v.data_size % 8
                };
                8 + v.data_size + padding
            }
            ArrayData::DataSmall(v) => {
                let padding = if v.data_size == 0 {
                    4
                } else if (v.data_size % 4) == 0 {
                    0
                } else {
                    4 - v.data_size % 4
                };
                (4 + v.data_size + padding) as u32
            }
        }
    }
}

impl std::fmt::Display for ArrayData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArrayData::DataNormal(data) => {
                write!(f, "{}", data)
            }
            ArrayData::DataSmall(data) => {
                write!(f, "{}", data)
            }
        }
    }
}

#[binrw]
#[derive(Debug, Clone)]
#[br(import(is_logical: bool))]
pub enum ArrayDataSparse {
    DataNormal(#[br(args(is_logical))] ArrayDataSparseNormal),
    DataSmall(#[br(args(is_logical))] ArrayDataSparseSmall),
}

impl ArrayDataSparse {
    pub fn array_data_value_var(self) -> ArrayDataValueVar {
        match self {
            ArrayDataSparse::DataNormal(d) => d.value,
            ArrayDataSparse::DataSmall(d) => d.value,
        }
    }
    pub fn size(&self) -> u32 {
        match self {
            ArrayDataSparse::DataNormal(v) => {
                let padding = if (v.data_size % 8) == 0 {
                    0
                } else {
                    8 - v.data_size % 8
                };
                8 + v.data_size + padding
            }
            ArrayDataSparse::DataSmall(v) => {
                let padding = if v.data_size == 0 {
                    4
                } else if (v.data_size % 4) == 0 {
                    0
                } else {
                    4 - v.data_size % 4
                };
                (4 + v.data_size + padding) as u32
            }
        }
    }
}

pub trait ArrayDataSparseNew<T> {
    #[allow(clippy::new_ret_no_self)]
    fn new(value: Vec<T>) -> ArrayDataSparse;
}

macro_rules! impl_ArrayDataSparseNew {
    ($t1: ty, $t2: ident, $t3: ident, $t4: expr) => {
        impl ArrayDataSparseNew<$t1> for ArrayDataSparse {
            fn new(value: Vec<$t1>) -> ArrayDataSparse {
                let nelem = value.len();

                let data_type = MatFileDataTypes::$t2;
                let data_size = std::mem::size_of::<$t1>() * nelem;
                let value_wrapped = ArrayDataValueVar::$t3(value);

                if nelem > $t4 {
                    ArrayDataSparse::DataNormal(ArrayDataSparseNormal::new(
                        data_type,
                        data_size as u32,
                        value_wrapped,
                    ))
                } else {
                    ArrayDataSparse::DataSmall(ArrayDataSparseSmall::new(
                        data_type,
                        data_size as u16,
                        value_wrapped,
                    ))
                }
            }
        }
    };
}

impl_ArrayDataSparseNew!(u8, MiUINT8, ArrayValueU8, 4);

impl ArrayDataSparseNew<f64> for ArrayDataSparse {
    fn new(value: Vec<f64>) -> ArrayDataSparse {
        let data_type = MatFileDataTypes::MiDOUBLE;
        let nelem = value.len();
        let data_size = std::mem::size_of::<f64>() * nelem;
        let value_wrapped = ArrayDataValueVar::ArrayValueF64(value);

        ArrayDataSparse::DataNormal(ArrayDataSparseNormal::new(
            data_type,
            data_size as u32,
            value_wrapped,
        ))
    }
}

impl ArrayDataSparseNew<bool> for ArrayDataSparse {
    fn new(value: Vec<bool>) -> ArrayDataSparse {
        let nelem = value.len();

        let data_type = MatFileDataTypes::MiUINT8;
        let data_size = std::mem::size_of::<bool>() * nelem;
        let value_wrapped = ArrayDataValueVar::ArrayValueBOOL(value);

        if nelem > 4 {
            ArrayDataSparse::DataNormal(ArrayDataSparseNormal::new(
                data_type,
                data_size as u32,
                value_wrapped,
            ))
        } else {
            ArrayDataSparse::DataSmall(ArrayDataSparseSmall::new(
                data_type,
                data_size as u16,
                value_wrapped,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use binrw::BinReaderExt;
    use binrw::io::Cursor; // A no_std reimplementation of std::io // extension traits for use with readers and writers

    /*
     *
     * u8
     *
     */

    /// (Part of) binary representation of a MAT-file containing a variable with a empty single (double) value.
    /// To reproduce, in a MATLAB session with a clean workspace run
    /// `>> a = [];
    /// `>> save('example.mat', 'a', '-nocompression');`
    ///
    const DATA0: &[u8; 8] = b"\x02\x00\x00\x00\x00\x00\x00\x00";
    const A0: &[f64; 0] = &[];

    #[test]
    fn deserialize_data_u8_empty() {
        let mut bin = Cursor::new(DATA0);
        let data = bin
            .read_le_args::<ArrayData>((MatlabArrayTypes::MxDOUBLECLASS, false))
            .unwrap();
        println!("Deserialized data: {:#?}", &data);

        if let ArrayData::DataNormal(v) = data {
            if let ArrayDataValueVar::ArrayValueF64(val) = v.value {
                assert!(val == A0);
            } else {
                panic!("Not f64")
            }
        } else {
            panic!("No DataSmall")
        }
    }

    #[test]
    fn serialize_data_u8_empty() {
        let mut bin = Cursor::new(vec![]);
        let data = ArrayData::new(A0.to_vec());
        println!("Serialized data: {:#?}", &data);
        data.write_le(&mut bin).unwrap();

        println!("Orig bin: {:?}", DATA0);
        println!("Ser  bin: {:?}", bin);
        assert!(bin.into_inner() == DATA0);
    }

    /// (Part of) binary representation of a MAT-file containing a variable with a u8 values.
    /// To reproduce, in a MATLAB session with a clean workspace run
    /// `>> a = [55, 66, 77, 88];
    /// `>> save('example.mat', 'a', '-nocompression');`
    ///
    const DATA2: &[u8; 8] = b"\x02\x00\x04\x00\x37\x42\x4d\x58";
    const A2: &[u8; 4] = &[55, 66, 77, 88];

    #[test]
    fn deserialize_data_u8_array_1_4() {
        let mut bin = Cursor::new(DATA2);
        let data = bin
            .read_le_args::<ArrayData>((MatlabArrayTypes::MxUINT8CLASS, false))
            .unwrap();
        println!("Deserialized data: {:#?}", &data);

        if let ArrayData::DataSmall(v) = data {
            if let ArrayDataValueVar::ArrayValueU8(val) = v.value {
                assert!(val == A2);
            } else {
                panic!("Not u8")
            }
        } else {
            panic!("No DataSmall")
        }
    }

    #[test]
    fn serialize_data_u8_array_1_4() {
        let mut bin = Cursor::new(vec![]);
        let data = ArrayData::new(A2.to_vec());
        println!("data: {:#?}", &data);
        data.write_le(&mut bin).unwrap();

        println!("Orig bin: {:?}", DATA2);
        println!("Ser  bin: {:?}", bin);
        assert!(bin.into_inner() == DATA2);
    }

    /// (Part of) binary representation of a MAT-file containing a variable with a u8 values.
    /// To reproduce, in a MATLAB session with a clean workspace run
    /// `>> a = [11, 22, 33, 44, 55, 66, 77, 88, 99];
    /// `>> save('example.mat', 'a', '-nocompression');`
    ///
    const DATA3: &[u8; 24] =
        b"\x02\x00\x00\x00\x09\x00\x00\x00\x0b\x16\x21\x2c\x37\x42\x4d\x58\x63\x00\x00\x00\x00\x00\x00\x00";
    const A3: &[u8; 9] = &[11, 22, 33, 44, 55, 66, 77, 88, 99];

    #[test]
    fn deserialize_data_u8_array_1_9() {
        let mut bin = Cursor::new(DATA3);
        let data = bin
            .read_le_args::<ArrayData>((MatlabArrayTypes::MxUINT8CLASS, false))
            .unwrap();
        println!("Deserialized data: {:#?}", &data);

        if let ArrayData::DataNormal(v) = data {
            if let ArrayDataValueVar::ArrayValueU8(val) = v.value {
                assert!(val == A3);
            } else {
                panic!("Not u8")
            }
        } else {
            panic!("No DataNormal")
        }
    }

    #[test]
    fn serialize_data_u8_array_1_9() {
        let mut bin = Cursor::new(vec![]);
        let data = ArrayData::new(A3.to_vec());
        println!("data: {:#?}", &data);
        data.write_le(&mut bin).unwrap();

        println!("Orig bin: {:?}", DATA3);
        println!("Ser  bin: {:?}", bin);
        assert!(bin.into_inner() == DATA3);
    }

    /*
     *
     * u16
     *
     */

    /// (Part of) binary representation of a MAT-file containing a variable with a u16 values.
    /// To reproduce, in a MATLAB session with a clean workspace run
    /// `>> a = uint16([55, 66]);
    /// `>> save('example.mat', 'a', '-nocompression');`
    ///
    const DATA_U16_1: &[u8; 8] = b"\x04\x00\x04\x00\x37\x00\x42\x00";
    const VAR_U16_1: &[u16; 2] = &[55, 66];

    #[test]
    fn deserialize_data_u16_array_1_2() {
        let mut bin = Cursor::new(DATA_U16_1);
        let data = bin
            .read_le_args::<ArrayData>((MatlabArrayTypes::MxUINT16CLASS, false))
            .unwrap();
        println!("Deserialized data: {:#?}", &data);

        if let ArrayData::DataSmall(v) = data {
            if let ArrayDataValueVar::ArrayValueU16(val) = v.value {
                assert!(val == VAR_U16_1);
            } else {
                panic!("Not u16")
            }
        } else {
            panic!("No DataNormal")
        }
    }

    #[test]
    fn serialize_data_u16_array_1_2() {
        let mut bin = Cursor::new(vec![]);
        let data = ArrayData::new(VAR_U16_1.to_vec());
        println!("data: {:#?}", &data);
        data.write_le(&mut bin).unwrap();

        println!("Orig bin: {:?}", DATA_U16_1);
        println!("Ser  bin: {:?}", bin);
        assert!(bin.into_inner() == DATA_U16_1);
    }

    /// (Part of) binary representation of a MAT-file containing a variable with a u16 values.
    /// To reproduce, in a MATLAB session with a clean workspace run
    /// `>> a = uint16([55, 66, 77, 88]);
    /// `>> save('example.mat', 'a', '-nocompression');`
    ///
    const DATA_U16_2: &[u8; 16] = b"\x04\x00\x00\x00\x08\x00\x00\x00\x37\x00\x42\x00\x4d\x00\x58\x00";
    const VAR_U16_2: &[u16; 4] = &[55, 66, 77, 88];

    #[test]
    fn deserialize_data_u16_array_1_4() {
        let mut bin = Cursor::new(DATA_U16_2);
        let data = bin
            .read_le_args::<ArrayData>((MatlabArrayTypes::MxUINT16CLASS, false))
            .unwrap();
        println!("Deserialized data: {:#?}", &data);

        if let ArrayData::DataNormal(v) = data {
            if let ArrayDataValueVar::ArrayValueU16(val) = v.value {
                assert!(val == VAR_U16_2);
            } else {
                panic!("Not u16")
            }
        } else {
            panic!("No DataNormal")
        }
    }

    #[test]
    fn serialize_data_u16_array_1_4() {
        let mut bin = Cursor::new(vec![]);
        let data = ArrayData::new(VAR_U16_2.to_vec());
        println!("data: {:#?}", &data);
        data.write_le(&mut bin).unwrap();

        println!("Orig bin: {:?}", DATA_U16_2);
        println!("Ser  bin: {:?}", bin);
        assert!(bin.into_inner() == DATA_U16_2);
    }

    /*
     *
     * u32
     *
     */

    /// (Part of) binary representation of a MAT-file containing a variable with a u32 values.
    /// To reproduce, in a MATLAB session with a clean workspace run
    /// `>> a = uint32([55]);
    /// `>> save('example.mat', 'a', '-nocompression');`
    ///
    const DATA_U32_1: &[u8; 8] = b"\x06\x00\x04\x00\x37\x00\x00\x00";
    const VAR_U32_1: &[u32; 1] = &[55];

    #[test]
    fn deserialize_data_u32_array_1_1() {
        let mut bin = Cursor::new(DATA_U32_1);
        let data = bin
            .read_le_args::<ArrayData>((MatlabArrayTypes::MxUINT32CLASS, false))
            .unwrap();
        println!("Deserialized data: {:#?}", &data);

        if let ArrayData::DataSmall(v) = data {
            if let ArrayDataValueVar::ArrayValueU32(val) = v.value {
                assert!(val == VAR_U32_1);
            } else {
                panic!("Not u32")
            }
        } else {
            panic!("No DataNormal")
        }
    }

    #[test]
    fn serialize_data_u32_array_1_1() {
        let mut bin = Cursor::new(vec![]);
        let data = ArrayData::new(VAR_U32_1.to_vec());
        println!("data: {:#?}", &data);
        data.write_le(&mut bin).unwrap();

        println!("Orig bin: {:?}", DATA_U32_1);
        println!("Ser  bin: {:?}", bin);
        assert!(bin.into_inner() == DATA_U32_1);
    }

    /*
     *
     * f64
     *
     */

    /// (Part of) binary representation of a MAT-file containing a variable with a single double value.
    /// To reproduce, in a MATLAB session with a clean workspace run
    /// `>> a = 99.99;
    /// `>> save('example.mat', 'a', '-nocompression');`
    ///
    const DATA1: &[u8; 16] = b"\x09\x00\x00\x00\x08\x00\x00\x00\x8f\xc2\xf5\x28\x5c\xff\x58\x40";
    const A1: &[f64; 1] = &[99.99];

    #[test]
    fn deserialize_data_double_1_1() {
        let mut bin = Cursor::new(DATA1);
        let data = bin
            .read_le_args::<ArrayData>((MatlabArrayTypes::MxDOUBLECLASS, false))
            .unwrap();
        println!("Deserialized data: {:#?}", &data);
        // let val: &Vec<f64> = data.as_vec_ref().unwrap();
        // assert!(val == &vec![99.99]);

        if let ArrayData::DataNormal(v) = data {
            if let ArrayDataValueVar::ArrayValueF64(val) = v.value {
                assert!(val == A1);
            } else {
                panic!("Not u16")
            }
        } else {
            panic!("No DataNormal")
        }
    }

    #[test]
    fn serialize_data_double_1_1() {
        let mut bin = Cursor::new(vec![]);
        let data = ArrayData::new(A1.to_vec());
        println!("data: {:#?}", &data);
        data.write_le(&mut bin).unwrap();

        println!("Orig bin: {:?}", DATA0);
        println!("Ser  bin: {:?}", bin);
        assert!(bin.into_inner() == DATA1);
    }

    /*
     *
     * logical
     *
     */

    /// (Part of) binary representation of a MAT-file containing a variable with a single logical value.
    /// To reproduce, in a MATLAB session with a clean workspace run
    /// `>> a = true;
    /// `>> save('example.mat', 'a', '-nocompression');`
    ///
    const DATA4: &[u8; 8] = b"\x02\x00\x01\x00\x01\x00\x00\x00";
    const A4: &[bool; 1] = &[true];

    #[test]
    fn deserialize_data_logical_1_1() {
        let mut bin = Cursor::new(DATA4);
        let data = bin
            .read_le_args::<ArrayData>((MatlabArrayTypes::MxUINT8CLASS, true))
            .unwrap();
        println!("Deserialized data: {:#?}", &data);

        if let ArrayData::DataSmall(v) = data {
            if let ArrayDataValueVar::ArrayValueBOOL(val) = v.value {
                assert!(val == A4);
            } else {
                panic!("Not u16")
            }
        } else {
            panic!("No DataNormal")
        }
    }

    #[test]
    fn serialize_data_logical_1_1() {
        let mut bin = Cursor::new(vec![]);
        let data = ArrayData::new(A4.to_vec());
        println!("data: {:#?}", &data);
        data.write_le(&mut bin).unwrap();

        println!("Orig bin: {:?}", DATA4);
        println!("Ser  bin: {:?}", bin);
        assert!(bin.into_inner() == DATA4);
    }

    /*
     *
     * char
     *
     */

    /// (Part of) binary representation of a MAT-file containing a variable with char values.
    /// To reproduce, in a MATLAB session with a clean workspace run
    /// `>> a = 'abc';
    /// `>> save('example.mat', 'a', '-nocompression');`
    ///
    const DATA_CHAR_1: &[u8; 8] = b"\x10\x00\x03\x00\x61\x62\x63\x00";
    const VAR_CHAR_1: &[char; 3] = &['a', 'b', 'c'];

    #[test]
    fn deserialize_data_char_1_3() {
        let mut bin = Cursor::new(DATA_CHAR_1);
        let data = bin
            .read_le_args::<ArrayData>((MatlabArrayTypes::MxCHARCLASS, false))
            .unwrap();
        println!("Deserialized data: {:#?}", &data);
        // let val: &Vec<char> = data.as_vec_ref().unwrap();
        // assert!(val == VAR_CHAR_1);

        if let ArrayData::DataSmall(v) = data {
            if let ArrayDataValueVar::ArrayValueUTF8(val) = v.value {
                assert!(val == VAR_CHAR_1);
            } else {
                panic!("Not char")
            }
        } else {
            panic!("No DataNormal")
        }
    }

    #[test]
    fn serialize_data_char_1_3() {
        let mut bin = Cursor::new(vec![]);
        let data = ArrayData::new(VAR_CHAR_1.to_vec());
        println!("data: {:#?}", &data);
        data.write_le(&mut bin).unwrap();

        println!("Orig bin: {:?}", DATA_CHAR_1);
        println!("Ser  bin: {:?}", bin);
        assert!(bin.into_inner() == DATA_CHAR_1);
    }

    #[test]
    fn serialize_non_ascii() {
        let mut bin = Cursor::new(vec![]);
        let data = ArrayData::new(['a', '✔'].to_vec());
        println!("data: {:#?}", &data);
        data.write_le(&mut bin).unwrap();
        println!("Ser  bin: {:?}", bin);
    }
}
