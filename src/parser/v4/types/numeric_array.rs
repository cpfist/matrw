use binrw::*;

use crate::NumericArray;
use crate::interface::types::matlab_types::MatlabType;
use crate::parser::v4::types::subelements::array_numeric_data::array_data::{ArrayData, ArrayDataNew};
use crate::parser::v4::types::subelements::array_numeric_data::array_data_value::ArrayDataValueVar;
use crate::parser::v4::flags::{MatFileDataTypes, MatFileMatrixTypes};
use crate::parser::v4::types::header::MatVariableHeader4;

#[binrw]
#[derive(Debug, Clone)]
pub struct NumericArray4 {
    pub header: MatVariableHeader4,
    #[br(count = header.namlen as usize)]
    pub name: Vec<u8>,
    #[br(args(&header.data_type, &header.matrix_type, (header.mrows * header.ncols) as usize))]
    pub real: ArrayData,
    #[br(if(header.imagf == 1), args(&header.data_type, &header.matrix_type, (header.mrows * header.ncols) as usize))]
    pub imag: Option<ArrayData>,
}

impl NumericArray4 {
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string().into_bytes();
        // append \0 byte
        self.name.push(0);

        self.header.namlen = self.name.len() as u32;
    }
    pub fn name(&self) -> String {
        let mut chars = self.name.clone();
        chars.pop();

        String::from_utf8(chars).unwrap()
    }
    pub fn value(self) -> (String, Vec<usize>, ArrayDataValueVar, Option<ArrayDataValueVar>) {
        let name = self.name();
        let dim = vec![self.header.mrows as usize, self.header.ncols as usize];
        let val = self.real.data;
        let val_cmp = self.imag.map(|v| v.data);

        (name, dim, val, val_cmp)
    }
}

pub trait NumericArrayNew<T> {
    #[allow(clippy::new_ret_no_self)]
    fn new(dim: Vec<u32>, value: Vec<T>, value_cmp: Option<Vec<T>>) -> NumericArray4;
}

macro_rules! impl_NumericArrayNew {
    ($t1: ty, $t2: ident, $t3: ident) => {
        impl NumericArrayNew<$t1> for NumericArray4 {
            fn new(dim: Vec<u32>, value: Vec<$t1>, value_cmp: Option<Vec<$t1>>) -> NumericArray4 {
                let real = ArrayData::new(value);
                let imag = value_cmp.map(ArrayData::new);

                let name = vec![0u8];

                let header = MatVariableHeader4::new(
                    crate::parser::v4::flags::Endian::Little,
                    MatFileDataTypes::$t2,
                    MatFileMatrixTypes::$t3,
                    dim[0],
                    dim[1],
                    imag.is_some() as u32,
                    (name.len() + 1) as u32,
                );

                Self {
                    header,
                    name,
                    real,
                    imag,
                }
            }
        }
    };
}

impl_NumericArrayNew!(u8, MiUINT8, Full);
impl_NumericArrayNew!(u16, MiUINT16, Full);
impl_NumericArrayNew!(i16, MiINT16, Full);
impl_NumericArrayNew!(i32, MiINT32, Full);
impl_NumericArrayNew!(f32, MiSINGLE, Full);
impl_NumericArrayNew!(f64, MiDOUBLE, Full);
impl_NumericArrayNew!(char, MiUINT8, Text);

impl From<NumericArray> for NumericArray4 {
    fn from(value: NumericArray) -> Self {
        use MatlabType::*;

        let dim = value.dim.iter().map(|x| *x as u32).collect();

        if value.value.is_empty() {
            return Self::new(dim, Vec::<f64>::new(), None);
        }

        match (value.numeric_type(), value.is_complex()) {
            (U8(_), true) => Self::new(
                dim,
                value.value.inner::<u8>().unwrap(),
                Some(value.value_cmp.unwrap().inner::<u8>().unwrap()),
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
            (I32(_), true) => Self::new(
                dim,
                value.value.inner::<i32>().unwrap(),
                Some(value.value_cmp.unwrap().inner::<i32>().unwrap()),
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
            (U16(_), false) => Self::new(dim, value.value.inner::<u16>().unwrap(), None),
            (I16(_), false) => Self::new(dim, value.value.inner::<i16>().unwrap(), None),
            (I32(_), false) => Self::new(dim, value.value.inner::<i32>().unwrap(), None),
            (F32(_), false) => Self::new(dim, value.value.inner::<f32>().unwrap(), None),
            (F64(_), false) => Self::new(dim, value.value.inner::<f64>().unwrap(), None),
            (UTF8(_), false) => Self::new(dim, value.value.inner::<char>().unwrap(), None),
            _ => unimplemented!(),
        }
    }
}
