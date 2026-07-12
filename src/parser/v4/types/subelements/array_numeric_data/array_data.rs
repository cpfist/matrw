use binrw::*;

use super::array_data_value::{ArrayDataValueVar, parse_array_data, write_array_data};
use crate::parser::v4::flags::{MatFileDataTypes, MatFileMatrixTypes};

#[binrw]
#[derive(Debug, Clone)]
#[br(import(data_type: &MatFileDataTypes, matrix_type: &MatFileMatrixTypes, size: usize))]
pub struct ArrayData {
    #[br(parse_with = parse_array_data, args(data_type, matrix_type, size))]
    #[bw(write_with = write_array_data)]
    pub data: ArrayDataValueVar,
}

// impl ArrayData {
//     pub fn new(value: Vec<f64>) -> Self {
//         Self {
//             data: ArrayDataValueVar::ArrayValueF64(value)
//         }
//     }
// }

pub trait ArrayDataNew<T> {
    #[allow(clippy::new_ret_no_self)]
    fn new(value: Vec<T>) -> ArrayData;
}

macro_rules! impl_ArrayDataNew {
    ($t1: ty, $t3: ident) => {
        impl ArrayDataNew<$t1> for ArrayData {
            fn new(value: Vec<$t1>) -> ArrayData {
                Self {
                    data: ArrayDataValueVar::$t3(value),
                }
            }
        }
    };
}

impl_ArrayDataNew!(u8, ArrayValueU8);
impl_ArrayDataNew!(u16, ArrayValueU16);
impl_ArrayDataNew!(i16, ArrayValueI16);
impl_ArrayDataNew!(i32, ArrayValueI32);
impl_ArrayDataNew!(f32, ArrayValueF32);
impl_ArrayDataNew!(f64, ArrayValueF64);
impl_ArrayDataNew!(char, ArrayValueUTF8);
