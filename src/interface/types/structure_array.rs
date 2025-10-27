use std::fmt::Debug;

use indexmap::IndexMap;

use crate::interface::types::array::ArrayType;
use crate::interface::types::structure::Structure;
use crate::interface::variable::MatVariable;
use crate::parser::v7::types::structure_array::StructureArray7;
use crate::{MatrwError, impl_Array_for};

#[derive(Debug, Clone)]
pub struct StructureArray {
    pub dim: Vec<usize>,
    fieldnames: Vec<String>,
    pub value: Vec<MatVariable>,
}

impl_Array_for!(StructureArray);

impl StructureArray {
    pub fn new(
        dim: Vec<usize>,
        fieldnames: Vec<String>,
        value: Vec<MatVariable>,
    ) -> Result<Self, MatrwError> {
        if !dim.is_empty() {
            let elem_from_dim = dim.iter().product::<usize>() * fieldnames.len();
            let elem_provided = value.len();
            if elem_from_dim != elem_provided {
                return Err(MatrwError::TypeConstruction(format!(
                    "Specified dimension {} does not match number of elements {}.",
                    elem_from_dim, elem_provided
                )));
            }
        }

        let mut val = Vec::new();
        let mut v = value.into_iter();
        while v.len() != 0 {
            let mut map = IndexMap::new();
            for f in fieldnames.iter() {
                map.insert(f.to_string(), v.next().unwrap());
            }
            val.push(MatVariable::Structure(Structure::new(map)));
        }

        Ok(Self {
            dim,
            fieldnames,
            value: val,
        })
    }
    pub fn from_structures(dim: Vec<usize>, value: Vec<MatVariable>) -> Self {
        let mut val = Vec::new();
        for v in value.into_iter() {
            val.push(v);
        }
        let fieldnames = val[0].fieldnames().expect("Cannot read field names");

        Self {
            dim,
            fieldnames,
            value: val,
        }
    }
    pub fn fieldnames(&self) -> Vec<String> {
        self.fieldnames.clone()
    }
}

impl From<StructureArray7> for StructureArray {
    fn from(value: StructureArray7) -> Self {
        let dim: Vec<usize> = value.dim().clone().iter().map(|x| *x as usize).collect();
        let fieldnames = value.fieldnames();

        Self::new(
            dim,
            fieldnames,
            value.value().into_iter().map(|x| x.into()).collect(),
        )
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::{MatlabType, interface::types::numeric_array::NumericArray};

    use super::*;

    #[test]
    fn interface_struct_array_new() {
        let dim = vec![1, 1];
        let fieldnames = vec!["a".to_string()];
        let value = vec![MatVariable::NumericArray(
            NumericArray::new(vec![1, 1], MatlabType::from(vec![1.0f64]), None).unwrap(),
        )];

        let s = StructureArray::new(dim, fieldnames, value).unwrap();
        let v = &s.get_ref_multidim(&[0, 0]).unwrap()["a"];

        println!("{:#?}", v);
    }
    #[test]
    fn interface_struct_array_2new() {
        let dim = vec![1, 2];
        let fieldnames = vec!["a".to_string(), "b".to_string()];
        let value = vec![
            MatVariable::NumericArray(
                NumericArray::new(vec![1, 1], MatlabType::from(vec![1.0f64]), None).unwrap(),
            ),
            MatVariable::NumericArray(
                NumericArray::new(vec![1, 1], MatlabType::from(vec![2.0f64]), None).unwrap(),
            ),
            MatVariable::NumericArray(
                NumericArray::new(vec![1, 1], MatlabType::from(vec![3.0f64]), None).unwrap(),
            ),
            MatVariable::NumericArray(
                NumericArray::new(vec![1, 1], MatlabType::from(vec![4.0f64]), None).unwrap(),
            ),
        ];

        let s = StructureArray::new(dim, fieldnames, value).unwrap();
        let v = s.get_ref_multidim(&[0, 0]);
        println!("{:#?}", v);
        let v = s.get_ref_multidim(&[0, 0]);
        println!("{:#?}", v);
        let v = s.get_ref_multidim(&[0, 1]);
        println!("{:#?}", v);
        let v = s.get_ref_multidim(&[0, 1]);
        println!("{:#?}", v);
    }
    #[test]
    fn interface_struct_array_3new() {
        let dim = vec![1, 3];
        let fieldnames = vec!["a".to_string(), "b".to_string()];
        let value = vec![
            MatVariable::NumericArray(
                NumericArray::new(vec![1, 1], MatlabType::from(vec![1.0f64]), None).unwrap(),
            ),
            MatVariable::NumericArray(
                NumericArray::new(vec![1, 1], MatlabType::from(vec![2.0f64]), None).unwrap(),
            ),
            MatVariable::NumericArray(
                NumericArray::new(vec![1, 1], MatlabType::from(vec![3.0f64]), None).unwrap(),
            ),
            MatVariable::NumericArray(
                NumericArray::new(vec![1, 1], MatlabType::from(vec![4.0f64]), None).unwrap(),
            ),
            MatVariable::NumericArray(
                NumericArray::new(vec![1, 1], MatlabType::from(vec![5.0f64]), None).unwrap(),
            ),
            MatVariable::NumericArray(
                NumericArray::new(vec![1, 1], MatlabType::from(vec![6.0f64]), None).unwrap(),
            ),
        ];

        let s = StructureArray::new(dim, fieldnames, value).unwrap();
        let v = s.get_ref_multidim(&[0, 0]);
        println!("{:#?}", v);
        let v = s.get_ref_multidim(&[0, 0]);
        println!("{:#?}", v);
        let v = s.get_ref_multidim(&[0, 1]);
        println!("{:#?}", v);
        let v = s.get_ref_multidim(&[0, 1]);
        println!("{:#?}", v);
        let v = s.get_ref_multidim(&[0, 2]);
        println!("{:#?}", v);
        let v = s.get_ref_multidim(&[0, 2]);
        println!("{:#?}", v);
    }
}
