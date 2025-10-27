use binrw::*;

use super::subelements::array_numeric_data::array_data_value::ArrayDataValueVar;
use crate::interface::types::matlab_types::MatlabType;
use crate::interface::types::sparse_array::SparseArray;
use crate::parser::v7::flags::{MatFileDataTypes, MatlabArrayTypes};
use crate::parser::v7::types::subelements::array_dimensions::ArrayDimensions;
use crate::parser::v7::types::subelements::array_flags::ArrayFlagBits;
use crate::parser::v7::types::subelements::array_flags::ArrayProps;
use crate::parser::v7::types::subelements::array_name::ArrayName;
use crate::parser::v7::types::subelements::array_numeric_data::array_data::{
    ArrayDataSparse, ArrayDataSparseNew,
};

#[binrw]
#[derive(Debug, Clone)]
#[br(assert(data_type == MatFileDataTypes::MiMATRIX &&
            props.array_class == MatlabArrayTypes::MxSPARSECLASS))]
pub struct SparseArray7 {
    #[brw(pad_size_to = 4)]
    data_type: MatFileDataTypes,
    #[bw(calc = self.size_data())]
    _num_bytes: u32,
    props: ArrayProps,
    #[brw(align_after = 8)]
    dimensions: ArrayDimensions,
    #[brw(align_after = 8)]
    name: ArrayName,
    #[brw(align_after = 8)]
    ir: ArrayDimensions,
    #[brw(align_after = 8)]
    jc: ArrayDimensions,
    #[br(args(props.array_flags.is_logical))]
    #[brw(align_after = 8)]
    value: ArrayDataSparse,
    #[br(if(props.array_flags.is_complex), args(props.array_flags.is_logical))]
    #[brw(align_after = 8)]
    value_cmp: Option<ArrayDataSparse>,
}

impl SparseArray7 {
    pub fn set_name(&mut self, name: &str) {
        self.name = ArrayName::new(name.to_string())
    }
    pub fn name(&self) -> String {
        self.name.name()
    }
    pub fn dim(&self) -> Vec<u32> {
        self.dimensions.dim().clone()
    }
    pub fn size_data(&self) -> u32 {
        let mut num_bytes = 0;
        num_bytes += self.props.size();
        num_bytes += self.dimensions.size();
        num_bytes += self.ir.size();
        num_bytes += self.jc.size();
        num_bytes += self.name.size();
        num_bytes += self.value.size();
        if self.value_cmp.is_some() {
            num_bytes += self.value.size();
        }
        if self.dimensions.is_empty() {
            num_bytes += 4
        }

        num_bytes
    }
    pub fn size(&self) -> usize {
        self.size_data() as usize + 8
    }
    pub fn value(
        self,
    ) -> (
        String,
        Vec<usize>,
        Vec<usize>,
        Vec<usize>,
        ArrayDataValueVar,
        Option<ArrayDataValueVar>,
    ) {
        let name = self.name();
        let dim = self
            .dimensions
            .dim()
            .clone()
            .iter()
            .map(|&x| x as usize)
            .collect();
        let ir = self.ir.dim().clone().iter().map(|&x| x as usize).collect();
        let jc = self.jc.dim().clone().iter().map(|&x| x as usize).collect();
        let val = self.value.array_data_value_var();
        let val_cmp = self.value_cmp.map(|v| v.array_data_value_var());

        (name, dim, ir, jc, val, val_cmp)
    }
}

pub trait SparseArrayNew<T> {
    #[allow(clippy::new_ret_no_self)]
    fn new(
        name: String,
        dim: Vec<u32>,
        dim_ir: Vec<u32>,
        dim_jc: Vec<u32>,
        value: Vec<T>,
        value_cmp: Option<Vec<T>>,
    ) -> SparseArray7;
}

macro_rules! impl_SparseArrayNew {
    ($t1: ty, $t2: ident) => {
        impl SparseArrayNew<$t1> for SparseArray7 {
            fn new(
                name: String,
                dim: Vec<u32>,
                dim_ir: Vec<u32>,
                dim_jc: Vec<u32>,
                value: Vec<$t1>,
                value_cmp: Option<Vec<$t1>>,
            ) -> Self {
                let is_complex = if value_cmp.is_some() { true } else { false };
                let empty_sparse = value.is_empty();

                let len = if !empty_sparse { value.len() as u32 } else { 1 };

                let props = ArrayProps::new(
                    MatlabArrayTypes::MxSPARSECLASS,
                    ArrayFlagBits::new(is_complex, false, false),
                    len,
                );

                let dimensions = ArrayDimensions::new(dim);
                let name = ArrayName::new(name);
                let ir = ArrayDimensions::new(dim_ir);
                let jc = ArrayDimensions::new(dim_jc);
                let value = ArrayDataSparse::new(value);
                let value_cmp = value_cmp.map(|v| ArrayDataSparse::new(v));

                Self {
                    data_type: MatFileDataTypes::MiMATRIX,
                    props,
                    dimensions,
                    name,
                    ir,
                    jc,
                    value,
                    value_cmp,
                }
            }
        }
    };
}

impl_SparseArrayNew!(u8, MxUINT8CLASS);
impl_SparseArrayNew!(f64, MxDOUBLECLASS);

impl SparseArrayNew<bool> for SparseArray7 {
    fn new(
        name: String,
        dim: Vec<u32>,
        dim_ir: Vec<u32>,
        dim_jc: Vec<u32>,
        value: Vec<bool>,
        value_cmp: Option<Vec<bool>>,
    ) -> SparseArray7 {
        let is_complex = value_cmp.is_some();
        let empty_sparse = value.is_empty();

        let len = if !empty_sparse { value.len() as u32 } else { 1 };

        let props = ArrayProps::new(
            MatlabArrayTypes::MxSPARSECLASS,
            ArrayFlagBits::new(is_complex, false, true),
            len,
        );

        let dimensions = ArrayDimensions::new(dim);
        let name = ArrayName::new(name);
        let ir = ArrayDimensions::new(dim_ir);
        let jc = ArrayDimensions::new(dim_jc);
        let value = ArrayDataSparse::new(value);
        let value_cmp = value_cmp.map(ArrayDataSparse::new);

        Self {
            data_type: MatFileDataTypes::MiMATRIX,
            props,
            dimensions,
            name,
            ir,
            jc,
            value,
            value_cmp,
        }
    }
}

impl From<SparseArray> for SparseArray7 {
    fn from(value: SparseArray) -> Self {
        use MatlabType::*;

        let dim = value.dim.iter().map(|x| *x as u32).collect();
        let ir = value.ir.iter().map(|x| *x as u32).collect();
        let jc = value.jc.iter().map(|x| *x as u32).collect();

        match (value.numeric_type(), value.is_complex()) {
            (F64(_), true) => Self::new(
                "".to_string(),
                dim,
                ir,
                jc,
                value.value.inner::<f64>().unwrap(),
                Some(value.value_cmp.unwrap().inner::<f64>().unwrap()),
            ),
            (F64(_), false) => Self::new(
                "".to_string(),
                dim,
                ir,
                jc,
                value.value.inner::<f64>().unwrap(),
                None,
            ),
            (BOOL(_), false) => Self::new(
                "".to_string(),
                dim,
                ir,
                jc,
                value.value.inner::<bool>().unwrap(),
                None,
            ),
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
    /// `>> a = sparse([1], [1], [1], 1, 1);`
    /// `>> save('example.mat', 'a');`
    ///
    const DATA_U8_1: &[u8; 88] = &[
        0x0e, 0x00, 0x00, 0x00, 0x50, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x05,
        0x10, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x01, 0x00,
        0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x61, 0x00, 0x00, 0x00, 0x05, 0x00, 0x04,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0xf0, 0x3f,
    ];

    #[test]
    fn sparse_array_scalar_double() {
        let mut bin = Cursor::new(DATA_U8_1);
        let data = bin.read_le::<SparseArray7>().unwrap();
        println!("Deserialized data: {:#?}", &data);

        let (name, dim, ir, jc, val, val_cmp) = data.value();

        assert_eq!(name, "a");
        assert_eq!(dim, vec![1, 1]);
        assert_eq!(ir, vec![0]);
        assert_eq!(jc, vec![0, 1]);
        assert!(matches!(val, ArrayDataValueVar::ArrayValueF64(_)));
        if let ArrayDataValueVar::ArrayValueF64(v) = val {
            assert_eq!(v, vec![1.0]);
        }
        assert!(val_cmp.is_none());
    }

    /// Binary representation of a mat file containing a variable `var_uint8` with a single u8 value.
    /// To reproduce, in a MATLAB session with a clean workspace run
    /// `>> a = sparse([3], [3], [true], 3, 3);`
    /// `>> save('example.mat', 'a');`
    ///
    const DATA_BOOL: &[u8; 88] = &[
        0x0e, 0x00, 0x00, 0x00, 0x50, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x05,
        0x12, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x03, 0x00,
        0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x61, 0x00, 0x00, 0x00, 0x05, 0x00, 0x04,
        0x00, 0x02, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x01,
        0x00, 0x00, 0x00,
    ];

    #[test]
    fn sparse_array_scalar_bool() {
        let mut bin = Cursor::new(DATA_BOOL);
        let data = bin.read_le::<SparseArray7>().unwrap();
        println!("Deserialized data: {:#?}", &data);

        let (name, dim, ir, jc, val, val_cmp) = data.value();

        assert_eq!(name, "a");
        assert_eq!(dim, vec![3, 3]);
        assert_eq!(ir, vec![2]);
        assert_eq!(jc, vec![0, 0, 0, 1]);
        assert!(matches!(val, ArrayDataValueVar::ArrayValueBOOL(_)));
        if let ArrayDataValueVar::ArrayValueBOOL(v) = val {
            assert_eq!(v, vec![true]);
        }
        assert!(val_cmp.is_none());
    }
}
