//! Numeric arrays in MAT-files
//!
//! The readout of numeric array data needs special treatment due to downsizing mechanisms used
//! when saving MAT-files in MATLAB. Whenever suitable, numeric data is represented by the smallest
//! possible numeric data type. Example: When saving a double precision value `42.0` into a MATFILE,
//! the saving algorithm in MATLAB may choose to use an u8 type to represent the value in the
//! MAT-file.
//!

use binrw::*;

use crate::interface::types::matlab_types::MatlabType;
use crate::interface::types::numeric_array::NumericArray;
use crate::parser::v7::flags::{MatFileDataTypes, MatlabArrayTypes};
use crate::parser::v7::types::subelements::array_dimensions::ArrayDimensions;
use crate::parser::v7::types::subelements::array_flags::ArrayProps;
use crate::parser::v7::types::subelements::array_name::ArrayName;
use crate::parser::v7::types::subelements::array_numeric_data::array_data::ArrayData;

use super::subelements::array_flags::ArrayFlagBits;
use super::subelements::array_numeric_data::array_data::ArrayDataNew;
use super::subelements::array_numeric_data::array_data_value::ArrayDataValueVar;

#[binrw]
#[derive(Debug, Clone)]
#[br(assert(data_type == MatFileDataTypes::MiMATRIX &&
            props.array_class != MatlabArrayTypes::MxCELLCLASS &&
            props.array_class != MatlabArrayTypes::MxSTRUCTCLASS))]
pub struct NumericArray7 {
    #[brw(pad_size_to = 4)]
    data_type: MatFileDataTypes,
    #[bw(calc = self.size_data())]
    _num_bytes: u32,
    props: ArrayProps,
    #[brw(align_after = 8)]
    dimensions: ArrayDimensions,
    #[brw(align_after = 8)]
    name: ArrayName,
    #[br(args(props.array_class, props.array_flags.is_logical))]
    #[brw(align_after = 8)]
    value: ArrayData,
    #[br(if(props.array_flags.is_complex), args(props.array_class, props.array_flags.is_logical))]
    #[brw(align_after = 8)]
    value_cmp: Option<ArrayData>,
}

impl NumericArray7 {
    pub fn set_name(&mut self, name: &str) {
        self.name = ArrayName::new(name.to_string())
    }
    pub fn name(&self) -> String {
        self.name.name()
    }
    pub fn size_data(&self) -> u32 {
        let mut num_bytes = 0;
        num_bytes += self.props.size();
        num_bytes += self.dimensions.size();
        num_bytes += self.name.size();
        num_bytes += self.value.size();
        if self.value_cmp.is_some() {
            num_bytes += self.value.size();
        }

        num_bytes
    }
    pub fn size(&self) -> usize {
        self.size_data() as usize + 8
    }
    pub fn value(self) -> (String, Vec<usize>, ArrayDataValueVar, Option<ArrayDataValueVar>) {
        let name = self.name();
        let dim = self
            .dimensions
            .dim()
            .clone()
            .iter()
            .map(|&x| x as usize)
            .collect();
        let val = self.value.array_data_value_var();
        let val_cmp = self.value_cmp.map(|v| v.array_data_value_var());

        (name, dim, val, val_cmp)
    }
}

pub trait NumericArrayNew<T> {
    #[allow(clippy::new_ret_no_self)]
    fn new(dim: Vec<u32>, value: Vec<T>, value_cmp: Option<Vec<T>>) -> NumericArray7;
}

macro_rules! impl_NumericArrayNew {
    ($t1: ty, $t2: ident) => {
        impl NumericArrayNew<$t1> for NumericArray7 {
            fn new(dim: Vec<u32>, value: Vec<$t1>, value_cmp: Option<Vec<$t1>>) -> NumericArray7 {
                let name = "".to_string();

                let is_complex = if value_cmp.is_some() { true } else { false };

                let props = ArrayProps::new(
                    MatlabArrayTypes::$t2,
                    ArrayFlagBits::new(is_complex, false, false),
                    0,
                );
                let dimensions = ArrayDimensions::new(dim);
                let name = ArrayName::new(name);
                let value = ArrayData::new(value);
                let value_cmp = value_cmp.map(ArrayData::new);

                Self {
                    data_type: MatFileDataTypes::MiMATRIX,
                    props,
                    dimensions,
                    name,
                    value,
                    value_cmp,
                }
            }
        }
    };
}

impl_NumericArrayNew!(u8, MxUINT8CLASS);
impl_NumericArrayNew!(i8, MxINT8CLASS);
impl_NumericArrayNew!(u16, MxUINT16CLASS);
impl_NumericArrayNew!(i16, MxINT16CLASS);
impl_NumericArrayNew!(u32, MxUINT32CLASS);
impl_NumericArrayNew!(i32, MxINT32CLASS);
impl_NumericArrayNew!(u64, MxUINT64CLASS);
impl_NumericArrayNew!(i64, MxINT64CLASS);
impl_NumericArrayNew!(f32, MxSINGLECLASS);
impl_NumericArrayNew!(f64, MxDOUBLECLASS);
impl_NumericArrayNew!(char, MxCHARCLASS);

impl NumericArrayNew<bool> for NumericArray7 {
    fn new(dim: Vec<u32>, value: Vec<bool>, value_cmp: Option<Vec<bool>>) -> NumericArray7 {
        let name = "".to_string();

        let is_complex = value_cmp.is_some();

        let props = ArrayProps::new(
            MatlabArrayTypes::MxUINT8CLASS,
            ArrayFlagBits::new(is_complex, false, true),
            0,
        );
        let dimensions = ArrayDimensions::new(dim);
        let name = ArrayName::new(name);
        let value = ArrayData::new(value);
        let value_cmp = value_cmp.map(ArrayData::new);

        Self {
            data_type: MatFileDataTypes::MiMATRIX,
            props,
            dimensions,
            name,
            value,
            value_cmp,
        }
    }
}

impl std::fmt::Display for NumericArray7 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\": {}", self.name.name(), self.value)
    }
}

impl From<NumericArray> for NumericArray7 {
    fn from(value: NumericArray) -> Self {
        use MatlabType::*;

        let dim = value.dim.iter().map(|x| *x as u32).collect();

        if value.value.is_empty() {
            return Self::new(dim, Vec::<u8>::new(), None);
        }

        match (value.numeric_type(), value.is_complex()) {
            (U8(_), true) => Self::new(
                dim,
                value.value.inner::<u8>().unwrap(),
                Some(value.value_cmp.unwrap().inner::<u8>().unwrap()),
            ),
            (I8(_), true) => Self::new(
                dim,
                value.value.inner::<i8>().unwrap(),
                Some(value.value_cmp.unwrap().inner::<i8>().unwrap()),
            ),
            (U16(_), true) => Self::new(
                dim,
                value.value.inner::<u16>().unwrap(),
                Some(value.value_cmp.unwrap().inner::<u16>().unwrap()),
            ),
            (I16(_), true) => Self::new(
                dim,
                value.value.inner::<i16>().unwrap(),
                Some(value.value_cmp.unwrap().inner::<i16>().unwrap()),
            ),
            (U32(_), true) => Self::new(
                dim,
                value.value.inner::<u32>().unwrap(),
                Some(value.value_cmp.unwrap().inner::<u32>().unwrap()),
            ),
            (I32(_), true) => Self::new(
                dim,
                value.value.inner::<i32>().unwrap(),
                Some(value.value_cmp.unwrap().inner::<i32>().unwrap()),
            ),
            (U64(_), true) => Self::new(
                dim,
                value.value.inner::<u64>().unwrap(),
                Some(value.value_cmp.unwrap().inner::<u64>().unwrap()),
            ),
            (I64(_), true) => Self::new(
                dim,
                value.value.inner::<i64>().unwrap(),
                Some(value.value_cmp.unwrap().inner::<i64>().unwrap()),
            ),
            (F32(_), true) => Self::new(
                dim,
                value.value.inner::<f32>().unwrap(),
                Some(value.value_cmp.unwrap().inner::<f32>().unwrap()),
            ),
            (F64(_), true) => Self::new(
                dim,
                value.value.inner::<f64>().unwrap(),
                Some(value.value_cmp.unwrap().inner::<f64>().unwrap()),
            ),
            (U8(_), false) => Self::new(dim, value.value.inner::<u8>().unwrap(), None),
            (I8(_), false) => Self::new(dim, value.value.inner::<i8>().unwrap(), None),
            (U16(_), false) => Self::new(dim, value.value.inner::<u16>().unwrap(), None),
            (I16(_), false) => Self::new(dim, value.value.inner::<i16>().unwrap(), None),
            (U32(_), false) => Self::new(dim, value.value.inner::<u32>().unwrap(), None),
            (I32(_), false) => Self::new(dim, value.value.inner::<i32>().unwrap(), None),
            (U64(_), false) => Self::new(dim, value.value.inner::<u64>().unwrap(), None),
            (I64(_), false) => Self::new(dim, value.value.inner::<i64>().unwrap(), None),
            (F32(_), false) => Self::new(dim, value.value.inner::<f32>().unwrap(), None),
            (F64(_), false) => Self::new(dim, value.value.inner::<f64>().unwrap(), None),
            (UTF8(_), false) => Self::new(dim, value.value.inner::<char>().unwrap(), None),
            (UTF16(_), false) => Self::new(dim, value.value.inner::<char>().unwrap(), None),
            (BOOL(_), false) => Self::new(dim, value.value.inner::<bool>().unwrap(), None),
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use binrw::BinReaderExt;
    use binrw::io::Cursor; // A no_std reimplementation of std::io // extension traits for use with readers and writers

    /// Binary representation of a mat file containing a variable `var_uint8` with a single u8 value.
    /// To reproduce, in a MATLAB session with a clean workspace run
    /// `>> var_empty = [];`
    /// `>> save('example.mat', 'var_empty', '-nocompression');`
    ///
    const DATA_EMPTY: &[u8; 72] = &[
        0x0e, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x06,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x76, 0x61, 0x72,
        0x5f, 0x65, 0x6d, 0x70, 0x74, 0x79, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ];
    const VAR_EMPTY: &[f64; 0] = &[];

    #[test]
    fn deserialize_empty() {
        let mut bin = Cursor::new(DATA_EMPTY);
        let data = bin.read_le::<NumericArray7>().unwrap();
        println!("Deserialized data: {:#?}", &data);
        if let ArrayDataValueVar::ArrayValueF64(val) = data.value.array_data_value_var() {
            assert_eq!(val, VAR_EMPTY);
        }
    }

    #[test]
    fn serialize_empty() {
        let mut bin = Cursor::new(vec![]);
        let mut data = NumericArray7::new(vec![0, 0], VAR_EMPTY.to_vec(), None);
        data.set_name("var_empty");
        println!("data: {:#?}", &data);
        data.write_le(&mut bin).unwrap();

        let inner = bin.into_inner();
        println!("Orig bin: {:?}", DATA_EMPTY);
        println!("Ser  bin: {:?}", &inner);
        assert_eq!(inner, DATA_EMPTY);
    }

    /// Binary representation of a mat file containing a variable `var_uint8` with a single u8 value.
    /// To reproduce, in a MATLAB session with a clean workspace run
    /// `>> var_uint8 = uint8(99);`
    /// `>> save('example.mat', 'var_uint8');`
    ///
    const DATA_U8_1: &[u8; 72] = &[
        0x0e, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x09,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x01, 0x00,
        0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x76, 0x61, 0x72,
        0x5f, 0x75, 0x69, 0x6e, 0x74, 0x38, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x01, 0x00,
        0x63, 0x00, 0x00, 0x00,
    ];
    const VAR_U8_1: &[u8; 1] = &[99];

    #[test]
    fn deserialize_scalar_u8() {
        let mut bin = Cursor::new(DATA_U8_1);
        let data = bin.read_le::<NumericArray7>().unwrap();
        println!("Deserialized data: {:#?}", &data);
        if let ArrayDataValueVar::ArrayValueU8(val) = data.value.array_data_value_var() {
            assert_eq!(val, VAR_U8_1);
        }
    }

    #[test]
    fn serialize_scalar_u8() {
        let mut bin = Cursor::new(vec![]);
        let mut data = NumericArray7::new(vec![1, 1], VAR_U8_1.to_vec(), None);
        data.set_name("var_uint8");
        println!("data: {:#?}", &data);
        data.write_le(&mut bin).unwrap();

        let inner = bin.into_inner();
        println!("Orig bin: {:?}", DATA_U8_1);
        println!("Ser  bin: {:?}", &inner);
        assert!(inner == DATA_U8_1);
    }

    /// Binary representation of a mat file containing a variable `var_u8_cmp` with a single u8 value.
    /// To reproduce, in a MATLAB session with a clean workspace run
    /// `>> var_u8_cmp = uint8(9 + 1i);`
    /// `>> save('example.mat', 'var_u8_cmp');`
    ///
    const DATA_U8_2: [u8; 80] = [
        0x0e, 0x00, 0x00, 0x00, 0x48, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x09,
        0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x01, 0x00,
        0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x0a, 0x00, 0x00, 0x00, 0x76, 0x61, 0x72,
        0x5f, 0x75, 0x38, 0x5f, 0x63, 0x6d, 0x70, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x01, 0x00,
        0x09, 0x00, 0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00,
    ];
    const VAR_U8_2_REAL: &[u8; 1] = &[9];
    const VAR_U8_2_CMP: &[u8; 1] = &[1];

    #[test]
    fn deserialize_scalar_u8_cmp() {
        let mut bin = Cursor::new(&DATA_U8_2);
        let data = bin.read_le::<NumericArray7>().unwrap();
        println!("Deserialized data: {:#?}", data);
        if let ArrayDataValueVar::ArrayValueU8(val) = data.value.array_data_value_var() {
            assert_eq!(val, VAR_U8_2_REAL);
        }
        if let ArrayDataValueVar::ArrayValueU8(val) = data.value_cmp.unwrap().array_data_value_var() {
            assert_eq!(val, VAR_U8_2_CMP);
        }
    }

    #[test]
    fn serialize_scalar_u8_cmp() {
        let mut bin = Cursor::new(vec![]);
        let mut data = NumericArray7::new(vec![1, 1], VAR_U8_2_REAL.to_vec(), Some(VAR_U8_2_CMP.to_vec()));
        data.set_name("var_u8_cmp");
        println!("data: {:#?}", &data);
        data.write_le(&mut bin).unwrap();

        let inner = bin.into_inner();
        println!("Orig bin: {:?}", DATA_U8_2);
        println!("Ser  bin: {:?}", &inner);
        assert!(inner == DATA_U8_2);
    }

    /// Binary representation of a mat file containing a variable `arr_u8` with a u8 values.
    /// To reproduce, in a MATLAB session with a clean workspace run
    /// `>> arr_u8 = uint8([33,44,55]);`
    /// `>> save('example.mat', 'arr_u8');`
    ///
    const DATA_U8_3: [u8; 64] = [
        0x0e, 0x00, 0x00, 0x00, 0x38, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x09,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x01, 0x00,
        0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x61, 0x72, 0x72,
        0x5f, 0x75, 0x38, 0x00, 0x00, 0x02, 0x00, 0x03, 0x00, 0x21, 0x2c, 0x37, 0x00,
    ];
    const VAR_U8_3: &[u8; 3] = &[33, 44, 55];

    #[test]
    fn deserialize_array_u8_small() {
        let mut bin = Cursor::new(&DATA_U8_3);
        let data = bin.read_le::<NumericArray7>().unwrap();
        println!("Deserialized data: {:#?}", data);
        // let val: &Vec<u8> = data.as_vec_ref().unwrap();
        // assert!(val == VAR_U8_3);
    }

    #[test]
    fn serialize_array_u8_small() {
        let mut bin = Cursor::new(vec![]);
        let mut data = NumericArray7::new(vec![1, 3], VAR_U8_3.to_vec(), None);
        data.set_name("arr_u8");
        println!("data: {:#?}", &data);
        data.write_le(&mut bin).unwrap();

        let inner = bin.into_inner();
        println!("Orig bin: {:?}", DATA_U8_3);
        println!("Ser  bin: {:?}", &inner);
        assert!(inner == DATA_U8_3);
    }

    /// Binary representation of a mat file containing a variable `arr_u8` with u8 values.
    /// To reproduce, in a MATLAB session with a clean workspace run
    /// `>> arr_u8 = uint8([33,44,55,66,77]);`
    /// `>> save('example.mat', 'arr_u8');`
    ///
    const DATA_U8_4: [u8; 72] = [
        0x0e, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x09,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x01, 0x00,
        0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x61, 0x72, 0x72,
        0x5f, 0x75, 0x38, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x21, 0x2c, 0x37, 0x42,
        0x4d, 0x00, 0x00, 0x00,
    ];
    const VAR_U8_4: &[u8; 5] = &[33, 44, 55, 66, 77];

    #[test]
    fn deserialize_array_u8_normal() {
        let data = Cursor::new(&DATA_U8_4).read_le::<NumericArray7>().unwrap();
        println!("Deserialized data: {:#?}", data);

        // let val: &Vec<u8> = data.as_vec_ref().unwrap();
        // assert!(val == VAR_U8_4);
    }

    #[test]
    fn serialize_array_u8_normal() {
        let mut bin = Cursor::new(vec![]);
        let mut data = NumericArray7::new(vec![1, 5], VAR_U8_4.to_vec(), None);
        data.set_name("arr_u8");
        println!("data: {:#?}", &data);
        data.write_le(&mut bin).unwrap();

        let inner = bin.into_inner();
        println!("Orig bin: {:?}", DATA_U8_4);
        println!("Ser  bin: {:?}", &inner);
        assert!(inner == DATA_U8_4);
    }

    /// Binary representation of a mat file containing a variable `var_double` with a single double value.
    /// To reproduce, in a MATLAB session with a clean workspace run
    /// `>> var_double = double(99.9);`
    /// `>> save('example.mat', 'var_double');`
    ///
    const DATA_F64_1: [u8; 80] = [
        0x0e, 0x00, 0x00, 0x00, 0x48, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x06,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x01, 0x00,
        0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x0a, 0x00, 0x00, 0x00, 0x76, 0x61, 0x72,
        0x5f, 0x64, 0x6f, 0x75, 0x62, 0x6c, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x00, 0x00, 0x9a, 0x99, 0x99, 0x99, 0x99, 0xf9, 0x58, 0x40,
    ];
    const VAR_F64_1: &[f64; 1] = &[99.9];

    #[test]
    fn deserialize_scalar_f64() {
        let mut bin = Cursor::new(&DATA_F64_1);
        let data = bin.read_le::<NumericArray7>().unwrap();
        println!("Deserialized data: {:#?}", data);
    }

    #[test]
    fn serialize_scalar_f64() {
        let mut bin = Cursor::new(vec![]);
        let mut data = NumericArray7::new(vec![1, 1], VAR_F64_1.to_vec(), None);
        data.set_name("var_double");
        println!("data: {:#?}", &data);
        data.write_le(&mut bin).unwrap();

        let inner = bin.into_inner();
        println!("Orig bin: {:?}", DATA_F64_1);
        println!("Ser  bin: {:?}", &inner);
        assert!(inner == DATA_F64_1);
    }

    /// Binary representation of a mat file containing a variable `a` with a single double value.
    /// To reproduce, in a MATLAB session with a clean workspace run
    /// `>> a = [11,22,33,44,55,66,77,88,99];`
    /// `>> save('example.mat', 'a', '-nocompression');`
    ///
    const DATA_F64_2: [u8; 72] = [
        0x0e, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x06,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x01, 0x00,
        0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x61, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00,
        0x00, 0x09, 0x00, 0x00, 0x00, 0x0b, 0x16, 0x21, 0x2c, 0x37, 0x42, 0x4d, 0x58, 0x63, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ];
    const VAR_F64_2: &[f64; 9] = &[11., 22., 33., 44., 55., 66., 77., 88., 99.];

    #[test]
    fn deserialize_array_f64() {
        let mut bin = Cursor::new(&DATA_F64_2);
        let data = bin.read_le::<NumericArray7>().unwrap();
        println!("Deserialized data: {:#?}", data);
    }

    #[test]
    fn serialize_array_f64() {
        let mut bin = Cursor::new(vec![]);
        let mut data = NumericArray7::new(vec![1, 9], VAR_F64_2.to_vec(), None);
        data.set_name("a");
        println!("data: {:#?}", &data);
        data.write_le(&mut bin).unwrap();

        let inner = bin.into_inner();
        println!("Orig bin: {:?}", DATA_F64_2);
        println!("Ser  bin: {:?}", &inner);
        assert!(inner == DATA_F64_2);
    }

    /// Binary representation of a mat file containing a variable `a` with a single double value.
    /// To reproduce, in a MATLAB session with a clean workspace run
    /// `>> a = [false,true];`
    /// `>> save('example.mat', 'a', '-nocompression');`
    ///
    const DATA_BOOL: [u8; 56] = [
        0x0e, 0x00, 0x00, 0x00, 0x30, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x09,
        0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x01, 0x00,
        0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x61, 0x00, 0x00, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x00, 0x01, 0x00, 0x00,
    ];
    const VAR_BOOL: &[bool; 2] = &[false, true];

    #[test]
    fn deserialize_array_bool() {
        let mut bin = Cursor::new(&DATA_BOOL);
        let data = bin.read_le::<NumericArray7>().unwrap();
        println!("Deserialized data: {:#?}", data);
    }

    #[test]
    fn serialize_array_bool() {
        let mut bin = Cursor::new(vec![]);
        let mut data = NumericArray7::new(vec![1, 2], VAR_BOOL.to_vec(), None);
        data.set_name("a");
        println!("data: {:#?}", &data);
        data.write_le(&mut bin).unwrap();

        let inner = bin.into_inner();
        println!("Orig bin: {:?}", DATA_BOOL);
        println!("Ser  bin: {:?}", &inner);
        assert!(inner == DATA_BOOL);
    }
}
