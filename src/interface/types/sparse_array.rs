use std::fmt::{Debug, Display};
use std::ops::Deref;

use crate::MatrwError;
use crate::interface::types::array::{ArrayType, ensure_matching_complex_size};
use crate::interface::types::matlab_types::MatlabType;
use crate::interface::types::numeric_array::NumericArray;
use crate::interface::variable::MatVariable;
use crate::parser::v7::types::sparse_array::SparseArray7;
use crate::parser::v7::types::subelements::array_numeric_data::array_data_value::ArrayDataValueVar;

/// Sparse array
///
/// The [`SparseArray`] type represents sparse arrays in compressed sparse column (CSC) format.
///
#[derive(Debug, Clone)]
pub struct SparseArray {
    pub dim: Vec<usize>,
    pub ir: Vec<usize>,
    pub jc: Vec<usize>,
    #[allow(dead_code)]
    null_type: Box<MatVariable>,
    pub value: MatlabType,
    pub value_cmp: Option<MatlabType>,
}

impl ArrayType for SparseArray {
    /// Get the dimension of the array
    fn dim(&self) -> &Vec<usize> {
        &self.dim
    }

    /// Get a borrowed value from a column-major index
    fn get_ref_colmaj(&self, _index: usize) -> Option<&MatVariable> {
        unimplemented!("It is not possible to receive SparseArray as reference.")
    }

    /// Get a cloned value from a multi-dimensional index
    fn get_clone_multidim(&self, idx: &[usize]) -> Option<MatVariable> {
        if idx[0] >= self.dim[0] {
            return None;
        }
        if idx[1] >= self.dim[1] {
            return None;
        }

        let colmaj_idx = self.column_index(idx);

        if let Some(v) = colmaj_idx {
            self.get_clone_colmaj(v)
        } else {
            Some(self.null_type.deref().clone())
        }
    }

    /// Get a cloned value from a column-major index
    fn get_clone_colmaj(&self, index: usize) -> Option<MatVariable> {
        if self.is_complex() {
            Some(MatVariable::NumericArray(
                NumericArray::new(
                    vec![1, 1],
                    self.value.clone_at_index(index),
                    self.value_cmp.as_ref().map(|x| x.clone_at_index(index)),
                )
                .unwrap(),
            ))
        } else {
            Some(MatVariable::NumericArray(
                NumericArray::new(vec![1, 1], self.value.clone_at_index(index), None).unwrap(),
            ))
        }
    }

    /// Get column-major index from multi-dimensional index
    fn column_index(&self, idx: &[usize]) -> Option<usize> {
        let jc = self.jc[idx[1]];
        let nc = self.jc[idx[1] + 1] - jc;

        if let Some(l) = (jc..jc + nc).find(|&f| self.ir[f] == idx[0]) {
            return Some(l);
        }

        None
    }
}

impl SparseArray {
    pub fn new(
        dim_i: usize,
        dim_j: usize,
        ir: Vec<usize>,
        jc: Vec<usize>,
        value: MatlabType,
        value_cmp: Option<MatlabType>,
    ) -> Result<Self, MatrwError> {
        // Plausibility checks
        if let Some(cmp) = &value_cmp {
            ensure_matching_complex_size(value.len(), cmp.len())?;
        }

        let max_elem = dim_i * dim_j;
        if value.len() > max_elem {
            return Err(MatrwError::TypeConstruction(format!(
                "More value elements ({}) than allowed by dimension ({}).",
                value.len(),
                max_elem
            )));
        }

        if ir.len() != value.len() {
            return Err(MatrwError::TypeConstruction(format!(
                "Specified ir size {} does not match number of elements {}.",
                ir.len(),
                value.len()
            )));
        }

        let is_comp = value_cmp.is_some();
        let null_type = match value {
            MatlabType::BOOL(_) if !is_comp => MatVariable::from(false),
            MatlabType::BOOL(_) if is_comp => MatVariable::from((false, false)),
            MatlabType::F64(_) if !is_comp => MatVariable::from(0.0),
            MatlabType::F64(_) if is_comp => MatVariable::from((0.0, 0.0)),
            _ => {
                return Err(MatrwError::TypeConstruction(
                    "Sparse matrix can only be of type bool or f64".to_string(),
                ));
            }
        };

        Ok(Self {
            dim: vec![dim_i, dim_j],
            ir,
            jc,
            null_type: Box::new(null_type),
            value,
            value_cmp,
        })
    }

    pub fn is_complex(&self) -> bool {
        self.value_cmp.is_some()
    }

    pub fn numeric_type(&self) -> &MatlabType {
        &self.value
    }
}

impl From<SparseArray7> for SparseArray {
    fn from(value: SparseArray7) -> Self {
        use ArrayDataValueVar::*;

        let (_name, dim, ir, jc, val, val_cmp) = value.value();
        let dim_i = dim[0];
        let dim_j = dim[1];

        let value = match val {
            ArrayValueU8(v) => MatlabType::U8(v),
            ArrayValueI8(v) => MatlabType::I8(v),
            ArrayValueU16(v) => MatlabType::U16(v),
            ArrayValueI16(v) => MatlabType::I16(v),
            ArrayValueU32(v) => MatlabType::U32(v),
            ArrayValueI32(v) => MatlabType::I32(v),
            ArrayValueU64(v) => MatlabType::U64(v),
            ArrayValueI64(v) => MatlabType::I64(v),
            ArrayValueF32(v) => MatlabType::F32(v),
            ArrayValueF64(v) => MatlabType::F64(v),
            ArrayValueUTF8(v) => MatlabType::UTF8(v),
            ArrayValueUTF16(v) => MatlabType::UTF16(v),
            ArrayValueBOOL(v) => MatlabType::BOOL(v),
        };

        let value_cmp = match val_cmp {
            Some(ArrayValueU8(v)) => Some(MatlabType::U8(v)),
            Some(ArrayValueI8(v)) => Some(MatlabType::I8(v)),
            Some(ArrayValueU16(v)) => Some(MatlabType::U16(v)),
            Some(ArrayValueI16(v)) => Some(MatlabType::I16(v)),
            Some(ArrayValueU32(v)) => Some(MatlabType::U32(v)),
            Some(ArrayValueI32(v)) => Some(MatlabType::I32(v)),
            Some(ArrayValueU64(v)) => Some(MatlabType::U64(v)),
            Some(ArrayValueI64(v)) => Some(MatlabType::I64(v)),
            Some(ArrayValueF32(v)) => Some(MatlabType::F32(v)),
            Some(ArrayValueF64(v)) => Some(MatlabType::F64(v)),
            Some(ArrayValueUTF8(v)) => Some(MatlabType::UTF8(v)),
            Some(ArrayValueUTF16(v)) => Some(MatlabType::UTF16(v)),
            Some(ArrayValueBOOL(v)) => Some(MatlabType::BOOL(v)),
            None => None,
        };

        Self::new(dim_i, dim_j, ir, jc, value, value_cmp).unwrap()
    }
}

impl Display for SparseArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;

        let max_width = self.value.max_width();

        for j in 0..self.jc.len() - 1 {
            let jc = self.jc[j];
            let nc = self.jc[j + 1] - jc;

            if nc == 0 {
                continue;
            }

            for l in jc..jc + nc {
                let ir = self.ir[l];

                write!(f, "   ({},{})\t", ir, j)?;
                self.value.print(f, l, false, max_width)?;
                self.value_cmp.as_ref().map(|v| v.print(f, l, true, max_width));
                writeln!(f)?;
            }
        }

        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use crate::OwnedIndex;

    use super::*;

    #[test]
    fn sparse_index_0x0() {
        let dim_i = 0;
        let dim_j = 0;
        let ir = vec![];
        let jc = vec![0];
        let a = MatlabType::from(Vec::<f64>::new());
        let m = MatVariable::SparseArray(SparseArray::new(dim_i, dim_j, ir, jc, a, None).unwrap());

        println!("m: {}", m);

        assert_eq!(m.elem([0, 0]).to_f64(), None);
    }
    #[test]
    fn sparse_index_1x1() {
        let dim_i = 1;
        let dim_j = 1;
        let ir = vec![0];
        let jc = vec![0, 1];
        let a = MatlabType::from(vec![9.0]);
        let m = MatVariable::SparseArray(SparseArray::new(dim_i, dim_j, ir, jc, a, None).unwrap());

        println!("m: {}", m);

        assert_eq!(m.elem([0, 0]).to_f64().unwrap(), 9.0);
    }
    #[test]
    fn sparse_index_2x2() {
        let dim_i = 2;
        let dim_j = 2;
        let ir = vec![0, 1, 0, 1];
        let jc = vec![0, 2, 4];
        let a = MatlabType::from(vec![1.0, 2.0, 3.0, 4.0]);
        let m = MatVariable::SparseArray(SparseArray::new(dim_i, dim_j, ir, jc, a, None).unwrap());

        println!("m: {}", m);

        assert_eq!(m.elem([0, 0]).to_f64().unwrap(), 1.0);
        assert_eq!(m.elem([1, 0]).to_f64().unwrap(), 2.0);
        assert_eq!(m.elem([0, 1]).to_f64().unwrap(), 3.0);
        assert_eq!(m.elem([1, 1]).to_f64().unwrap(), 4.0);
    }
    #[test]
    fn sparse_index_5x5() {
        let dim_i = 5;
        let dim_j = 5;
        let ir = vec![0, 0, 0, 0, 0];
        let jc = vec![0, 1, 2, 3, 4, 5];
        let a = MatlabType::from(vec![2.5, 5.0, 7.5, 10.0, 12.5]);
        let m = MatVariable::SparseArray(SparseArray::new(dim_i, dim_j, ir, jc, a, None).unwrap());

        println!("m: {}", m);

        assert_eq!(m.elem([0, 0]).to_f64().unwrap(), 2.5);
        assert_eq!(m.elem([0, 1]).to_f64().unwrap(), 5.0);
        assert_eq!(m.elem([0, 2]).to_f64().unwrap(), 7.5);
        assert_eq!(m.elem([0, 3]).to_f64().unwrap(), 10.0);
        assert_eq!(m.elem([0, 4]).to_f64().unwrap(), 12.5);

        assert_eq!(m.elem([1, 0]).to_f64().unwrap(), 0.0);
        assert!(matches!(m.elem([5, 0]), MatVariable::Null));
    }
    #[test]
    fn sparse_index_3x3() {
        let dim_i = 3;
        let dim_j = 3;
        let ir = vec![1];
        let jc = vec![0, 0, 1, 1];
        let a = MatlabType::from(vec![1.0]);
        let m = MatVariable::SparseArray(SparseArray::new(dim_i, dim_j, ir, jc, a, None).unwrap());

        println!("m: {}", m);

        assert_eq!(m.elem([1, 1]).to_f64().unwrap(), 1.0);
    }
}
