//! Module numeric_array
//!
//! This module defines the struct [`NumericArray`] which describes multidimensional dense arrays of complex numeric data.
//!

use std::fmt::{Debug, Display};
use std::mem::discriminant;

use crate::MatrwError;
use crate::interface::types::array::ArrayType;
use crate::interface::types::matlab_types::{MatlabType, MatlabTypeMarker};
use crate::interface::types::sparse_array::SparseArray;
use crate::interface::variable::MatVariable;
use crate::parser::v7::types::numeric_array::NumericArray7;
use crate::parser::v7::types::subelements::array_numeric_data::array_data_value::ArrayDataValueVar;

/// Contains vectors, matrices or multidimensional arrays of complex numeric data.
///
/// Examples
/// ```
/// use matrw::{NumericArray, MatlabType, MatVariable};
///
/// let data = vec![1, 2, 3];
/// let m = NumericArray::new(
///     vec![1, 3],
///     MatlabType::from(data),
///     None,
/// );
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct NumericArray {
    pub dim: Vec<usize>,
    pub value: MatlabType,
    pub value_cmp: Option<MatlabType>,
}

impl ArrayType for NumericArray {
    /// Get the dimension of the array
    fn dim(&self) -> &Vec<usize> {
        &self.dim
    }

    /// Get a borrowed value from a column-major index
    fn get_ref_colmaj(&self, _index: usize) -> Option<&MatVariable> {
        unimplemented!("It is not possible to receive NumericArray as reference.")
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
                .ok()?,
            ))
        } else {
            Some(MatVariable::NumericArray(
                NumericArray::new(vec![1, 1], self.value.clone_at_index(index), None).ok()?,
            ))
        }
    }
}

impl NumericArray {
    /// Constructs a new `NumericArray`.
    ///
    /// `NumericArray` can be constructed in two ways:
    /// - The first way uses `Vec<`[`MatVariable::NumericArray`]`>` data
    ///   to represent arbitrary numeric arrays. The data is interpreted in column major order.
    ///   The vector `dim` specifies the dimensions of the array.
    /// - The second way uses `Vec<`[`MatVariable::NumericArray`]`>` data to representing
    ///   row vectors of a 2D matrix. This is internally converted into the column major
    ///   representation. This method allows the convenient macro syntax in [`crate::matvar`].
    ///
    /// Example
    /// ```
    /// use matrw::{NumericArray, MatlabType, MatVariable};
    ///
    /// // Constructs a new `NumericArray` from `Vec<NumericScalar>`.
    /// // ---------------------------------------------------------
    /// let a = vec![1, 2, 3];
    /// let m = NumericArray::new(
    ///     vec![],
    ///     MatlabType::from(a),
    ///     None,
    /// );
    /// ```
    /// ```
    /// use matrw::{NumericArray, MatlabType, MatVariable};
    ///
    /// // Constructs a new `NumericArray` from `Vec<NumericArray>`.
    /// // --------------------------------------------------------
    /// let data1 = vec![1, 2, 3];
    /// let arr1 = MatVariable::NumericArray(NumericArray::new(
    ///     vec![],
    ///     MatlabType::from(data1),
    ///     None,
    /// ).unwrap());
    ///
    /// let data2 = vec![4, 5, 6];
    /// let arr2 = MatVariable::NumericArray(NumericArray::new(
    ///     vec![],
    ///     MatlabType::from(data2),
    ///     None,
    /// ).unwrap());
    ///
    /// let v = NumericArray::from_nested_matvar(vec![], vec![arr1, arr2]).unwrap();
    /// assert_eq!(v.dim, vec![2,3]);
    ///
    /// ```
    pub fn new(
        dim: Vec<usize>,
        value: MatlabType,
        value_cmp: Option<MatlabType>,
    ) -> Result<Self, MatrwError> {
        // Assert that dimensions match to number of values
        if !dim.is_empty() {
            let elem_from_dim = dim.iter().product::<usize>();
            let elem_provided = value.len();
            if elem_from_dim != elem_provided {
                return Err(MatrwError::TypeConstruction(format!(
                    "Specified dimension {} does not match number of elements {}.",
                    elem_from_dim, elem_provided
                )));
            }
        }

        let dim = if dim.is_empty() || dim.len() == 1 {
            // Normalize the dimension vector. Even 1D arrays are treated as 2D matrices in
            // MAT-files.
            vec![1, value.len()]
        } else {
            dim
        };

        Ok(Self {
            dim,
            value,
            value_cmp,
        })
    }

    pub fn from_nested_matvar(dim: Vec<usize>, value: Vec<MatVariable>) -> Result<Self, MatrwError> {
        // Return the trivial empty array
        if value.is_empty() {
            return Self::new(dim, MatlabType::new(), None);
        }

        let first_dim = value.first().unwrap().dim();

        if first_dim == vec![1, 1] {
            // If the vector `value` contains only numeric scalars, we create
            //  - a 1D array, if an empty or 1D `dim` is given,
            //  - an arbitrary multidimensional array, in every other case.
            //

            // Assert all elements have the same numeric type
            let first = discriminant(value.first().unwrap().numeric_type().unwrap());
            if !value
                .iter()
                .all(|x| discriminant(x.numeric_type().unwrap()) == first)
            {
                return Err(MatrwError::TypeConstruction(
                    "All elements must be of same numeric type.".to_string(),
                ));
            }

            let dim = if dim.is_empty() || dim.len() == 1 {
                // Normalize the dimension vector. Even 1D arrays are treated as 2D matrices in
                // MAT-files.
                vec![1, value.len()]
            } else {
                dim
            };

            let mut value_new = vec![];
            for v in value.iter() {
                match v {
                    MatVariable::NumericArray(x) => value_new.push(x.value.clone()),
                    _ => panic!(),
                }
            }

            let value_new = MatlabType::join(value_new).unwrap();

            let value_comp_new = if value.first().unwrap().is_complex().unwrap() {
                let mut value_comp_new = vec![];

                for v in value.iter() {
                    match v {
                        MatVariable::NumericArray(x) => {
                            value_comp_new.push(x.value_cmp.as_ref().unwrap().clone())
                        }
                        _ => panic!(),
                    }
                }

                Some(MatlabType::join(value_comp_new).unwrap())
            } else {
                None
            };

            Self::new(dim, value_new, value_comp_new)
        } else {
            // Check if input is 1D
            let is_row_vec = first_dim[0] == 1;
            let is_col_vec = first_dim[1] == 1;

            let (new_dim, new_value, new_value_cmp) = if is_row_vec {
                nested_row_vecs_to_colmaj_array(value)?
            } else if is_col_vec {
                nested_col_vecs_to_colmaj_array(value)?
            } else {
                flatten_higher_dim_nested_array(value)?
            };

            Self::new(new_dim, new_value, new_value_cmp)
        }
    }

    /// Move out real data into `Vec<T>`
    ///
    /// ```
    /// use matrw::{NumericArray, MatlabType, MatVariable};
    ///
    /// let a: Vec<f64> = vec![1.0, 2.0, 3.0];
    /// let m = NumericArray::new(
    ///     vec![1, 3],
    ///     MatlabType::from(a),
    ///     None,
    /// ).unwrap();
    ///
    /// let m_data: Vec<f64> = m.real_to_vec().unwrap();
    /// assert_eq!(m_data[0], 1.0);
    /// assert_eq!(m_data[1], 2.0);
    /// assert_eq!(m_data[2], 3.0);
    /// ```
    pub fn real_to_vec<T: MatlabTypeMarker>(&self) -> Option<Vec<T>> {
        self.value.clone().inner()
    }
    pub fn real_to_scalar<T: MatlabTypeMarker>(&self) -> Option<T> {
        Some(*self.value.get(0).unwrap())
    }
    pub fn is_scalar(&self) -> bool {
        self.dim.iter().product::<usize>() == 1
    }

    /// Move out complex data into `Vec<T>`
    ///
    /// ```
    /// use matrw::{NumericArray, MatlabType, MatVariable};
    ///
    /// let a: Vec<f64> = vec![1.0, 2.0, 3.0];
    /// let b: Vec<f64> = vec![1.0, 1.0, 1.0];
    /// let m = NumericArray::new(
    ///     vec![1, 3],
    ///     MatlabType::from(a),
    ///     Some(MatlabType::from(b)),
    /// ).unwrap();
    ///
    /// let m_data: Vec<f64> = m.comp_to_vec().unwrap();
    /// assert_eq!(m_data[0], 1.0);
    /// assert_eq!(m_data[1], 1.0);
    /// assert_eq!(m_data[2], 1.0);
    /// ```
    pub fn comp_to_vec<T: MatlabTypeMarker>(&self) -> Option<Vec<T>> {
        self.value_cmp.as_ref().map(|x| x.clone().inner().unwrap())
    }
    pub fn comp_to_scalar<T: MatlabTypeMarker>(&self) -> Option<T> {
        Some(*self.value_cmp.as_ref().map(|x| x.get(0).unwrap()).unwrap())
    }

    /// Convert to sparse matrix
    ///
    /// ```
    /// use matrw::{NumericArray, MatlabType, MatVariable};
    ///
    /// let data = vec![1.0, 2.0, 3.0];
    /// let m = NumericArray::new(
    ///     vec![1, 3],
    ///     MatlabType::from(data),
    ///     None,
    /// ).unwrap();
    ///
    /// let m_sparse = m.to_sparse().unwrap();
    ///
    /// matches!(m_sparse, MatVariable::SparseArray(_));
    /// ```
    pub fn to_sparse(self) -> Option<MatVariable> {
        if self.dim.len() > 2 {
            return None;
        }

        let is_comp = self.is_complex();
        let n_rows = self.dim[0];
        let n_cols = self.dim[1];
        let (ir, jc, data) = self.value.to_sparse(n_rows, n_cols);

        Some(MatVariable::SparseArray(
            SparseArray::new(self.dim.clone(), ir, jc, is_comp, data, None).unwrap(),
        ))
    }

    pub fn numeric_type(&self) -> &MatlabType {
        &self.value
    }

    pub fn is_complex(&self) -> bool {
        self.value_cmp.is_some()
    }
}

impl From<NumericArray7> for NumericArray {
    fn from(value: NumericArray7) -> Self {
        use ArrayDataValueVar::*;

        let (_name, dim, val, val_cmp) = value.value();

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

        let value_cmp = if val_cmp.is_some() {
            let tmp = match val_cmp {
                Some(ArrayValueU8(v)) => MatlabType::U8(v),
                Some(ArrayValueI8(v)) => MatlabType::I8(v),
                Some(ArrayValueU16(v)) => MatlabType::U16(v),
                Some(ArrayValueI16(v)) => MatlabType::I16(v),
                Some(ArrayValueU32(v)) => MatlabType::U32(v),
                Some(ArrayValueI32(v)) => MatlabType::I32(v),
                Some(ArrayValueU64(v)) => MatlabType::U64(v),
                Some(ArrayValueI64(v)) => MatlabType::I64(v),
                Some(ArrayValueF32(v)) => MatlabType::F32(v),
                Some(ArrayValueF64(v)) => MatlabType::F64(v),
                Some(ArrayValueUTF8(v)) => MatlabType::UTF8(v),
                Some(ArrayValueUTF16(v)) => MatlabType::UTF16(v),
                Some(ArrayValueBOOL(v)) => MatlabType::BOOL(v),
                _ => panic!("This should not happen"),
            };
            Some(tmp)
        } else {
            None
        };

        Self::new(dim, value, value_cmp).expect("Could not create NumericArray.")
    }
}

/// Constructs a new `NumericArray` from `&str`.
///
/// ```
/// use matrw::{NumericArray, MatlabType, MatVariable};
///
/// let m = NumericArray::from("test");
/// ```
impl From<&str> for NumericArray {
    fn from(value: &str) -> Self {
        let value = MatlabType::from(value);
        Self::new(vec![1, value.len()], value, None).expect("Could not create NumericArray")
    }
}

impl Display for NumericArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // If NumericArray is empty
        if self.dim.is_empty() {
            writeln!(f)?;
            write!(f, "[]")?;
            return writeln!(f);
        }

        //
        let idx_ref: Vec<usize> = Vec::from(&self.dim[2..]);
        let mut idx: Vec<usize> = vec![0; idx_ref.len()];

        let mut global_index = 0;
        let len = self.dim.iter().product::<usize>();

        writeln!(f)?;
        write!(f, "Dimensions: (")?;
        for (i, v) in self.dim.iter().enumerate() {
            if i < self.dim.len() - 1 {
                write!(f, "{},", v)?;
            } else {
                write!(f, "{}", v)?;
            }
        }
        writeln!(f, ")")?;

        // Calculate format
        let max_width = self.value.max_width();

        loop {
            writeln!(f)?;

            if self.dim.len() > 2 {
                write!(f, "(:,:,")?;
                for (i, v) in idx.iter().enumerate() {
                    if i < idx.len() - 1 {
                        write!(f, "{},", v)?;
                    } else {
                        write!(f, "{}", v)?;
                    }
                }
                writeln!(f, ")")?;
                writeln!(f)?;
            }

            for r in 0..self.dim[0] {
                for c in 0..self.dim[1] {
                    let idx = global_index + c*self.dim[0] + r;
                    self.value.print(f, idx, false, max_width)?;
                    self.value_cmp.as_ref().map(|v| v.print(f, idx, true, max_width));
                }
                writeln!(f)?;
            }
            global_index += self.dim[0] * self.dim[1];

            for i in 0..idx_ref.len() {
                if idx[i] < idx_ref[i] - 1 {
                    idx[i] += 1;
                    break;
                }
                if i < idx_ref.len() - 1 {
                    idx[i] = 0;
                }
            }

            if global_index == len {
                break;
            }
        }
        write!(f, "")
    }
}

/// Convert a row vectors into a column-major representation
///
///
fn nested_row_vecs_to_colmaj_array(rows: Vec<MatVariable>) -> Result<(Vec<usize>, MatlabType, Option<MatlabType>), MatrwError> {
    // Assert that all nested arrays have the same dimension and complex property
    let dim = rows.first().unwrap().dim();
    let is_complex = rows.first().unwrap().is_complex().unwrap();
    for row in &rows {
        if row.dim() != dim {
            return Err(MatrwError::TypeConstruction(
                "All row vectors must have the same dimensions.".to_string(),
            ));
        }
        if row.is_complex().unwrap() != is_complex {
            return Err(MatrwError::TypeConstruction(
                "All row vectors must have the same dimensions.".to_string(),
            ));
        }
    }

    let n_cols = rows[0].dim().iter().product();
    let n_rows = rows.len();

    let rows_vec = rows
        .iter()
        .map(|x| match x {
            MatVariable::NumericArray(v) => v.value.clone(),
            _ => panic!(),
        })
        .collect::<Vec<MatlabType>>();

    let value = MatlabType::join(rows_vec).unwrap();
    let value = MatlabType::row_vec_to_colmaj(value, n_rows, n_cols);

    let value_cmp = if is_complex {
        let rows_vec = rows
            .iter()
            .map(|x| match x {
                MatVariable::NumericArray(v) => v.value_cmp.as_ref().unwrap().clone(),
                _ => panic!(),
            })
            .collect::<Vec<MatlabType>>();

        let value = MatlabType::join(rows_vec).unwrap();
        let value = MatlabType::row_vec_to_colmaj(value, n_rows, n_cols);

        Some(value)
    } else {
        None
    };

    let dim = vec![n_rows, n_cols];

    Ok((dim, value, value_cmp))
}

/// Convert a col vectors into a column-major representation
///
///
fn nested_col_vecs_to_colmaj_array(cols: Vec<MatVariable>) -> Result<(Vec<usize>, MatlabType, Option<MatlabType>), MatrwError> {
    // Assert that all nested arrays have the same dimension and complex property
    let dim = cols.first().unwrap().dim();
    let is_complex = cols.first().unwrap().is_complex().unwrap();
    for col in &cols {
        if col.dim() != dim {
            return Err(MatrwError::TypeConstruction(
                "All row vectors must have the same dimensions.".to_string(),
            ));
        }
        if col.is_complex().unwrap() != is_complex {
            return Err(MatrwError::TypeConstruction(
                "All row vectors must have the same dimensions.".to_string(),
            ));
        }
    }

    let n_rows = cols[0].dim().iter().product();
    let n_cols = cols.len();

    let cols_vec = cols
        .iter()
        .map(|x| match x {
            MatVariable::NumericArray(v) => v.value.clone(),
            _ => panic!(),
        })
        .collect::<Vec<MatlabType>>();
    let value = MatlabType::join(cols_vec).unwrap();

    let value_cmp = if is_complex {
        let cols_vec = cols
            .iter()
            .map(|x| match x {
                MatVariable::NumericArray(v) => v.value_cmp.as_ref().unwrap().clone(),
                _ => panic!(),
            })
            .collect::<Vec<MatlabType>>();

        let value = MatlabType::join(cols_vec).unwrap();
        let value = MatlabType::row_vec_to_colmaj(value, n_rows, n_cols);

        Some(value)
    } else {
        None
    };


    let dim = vec![n_rows, n_cols];

    Ok((dim, value, value_cmp))
}

/// Flatten a nested array into a column-major representation
///
///
fn flatten_higher_dim_nested_array(value: Vec<MatVariable>) -> Result<(Vec<usize>, MatlabType, Option<MatlabType>), MatrwError> {
    // Assert that all nested arrays have the same dimension and complex property
    let dim = value.first().unwrap().dim();
    let is_complex = value.first().unwrap().is_complex().unwrap();
    for row in &value {
        if row.dim() != dim {
            return Err(MatrwError::TypeConstruction(
                "All row vectors must have the same dimensions.".to_string(),
            ));
        }
        if row.is_complex().unwrap() != is_complex {
            return Err(MatrwError::TypeConstruction(
                "All row vectors must have the same dimensions.".to_string(),
            ));
        }
    }

    let new_dim = vec![value[0].dim(), vec![value.len()]]
        .into_iter()
        .flatten()
        .collect();

    let new_value = value
        .iter()
        .map(|x| match x {
            MatVariable::NumericArray(v) => v.value.clone(),
            _ => panic!(),
        })
        .collect::<Vec<MatlabType>>();
    let new_value = MatlabType::join(new_value).unwrap();

    let new_value_cmp = if is_complex {
        let new_value_cmp = value
            .iter()
            .map(|x| match x {
                MatVariable::NumericArray(v) => v.value_cmp.as_ref().unwrap().clone(),
                _ => panic!(),
            })
            .collect::<Vec<MatlabType>>();
        Some(MatlabType::join(new_value_cmp).unwrap())
    } else {
        None
    };

    Ok((new_dim, new_value, new_value_cmp))
}

/// Check of every `NumericArray` has the same dimension
///
pub fn check_same_dim(vec: &[MatVariable]) -> bool {
    if vec.is_empty() {
        return false;
    }

    let first = vec.first().unwrap().dim();

    vec.iter().map(|x| x.dim() == first).into_iter().all(|x| x)
}

/// Check of every `NumericArray` has the same type
///
pub fn check_same_type(vec: &[MatVariable]) -> bool {
    if vec.is_empty() {
        return false;
    }

    let _first = vec.first().unwrap().numeric_type();

    vec.iter()
        .map(|x| matches!(x.numeric_type(), _first))
        .into_iter()
        .all(|x| x)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::OwnedIndex;

    #[test]
    fn assert_wrong_dim() {
        let a: Vec<f64> = vec![1.0, 2.0, 3.0];
        let m = NumericArray::new(vec![1, 4], MatlabType::from(a), None);

        assert!(matches!(m.expect_err(""), MatrwError::TypeConstruction(_)));
    }
    #[test]
    fn assert_mixed_dim() {
        let matrix_row1_raw = vec![1.0, 2.0];
        let matrix_row2_raw = vec![3.0];
        let matrix_row1 = MatVariable::NumericArray(
            NumericArray::new(vec![], MatlabType::from(matrix_row1_raw), None).unwrap(),
        );
        let matrix_row2 = MatVariable::NumericArray(
            NumericArray::new(vec![], MatlabType::from(matrix_row2_raw), None).unwrap(),
        );
        let matrix = NumericArray::from_nested_matvar(vec![], vec![matrix_row1, matrix_row2]);

        assert!(matches!(matrix.expect_err(""), MatrwError::TypeConstruction(_)));
    }
    #[test]
    fn numeric_array_view_1x9_real() {
        let a: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let m = NumericArray::new(vec![1, 9], MatlabType::from(a), None).unwrap();

        println!("{m}");
    }
    #[test]
    fn view_9x1_real() {
        let a: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let m = NumericArray::new(vec![9, 1], MatlabType::from(a), None).unwrap();

        println!("{m}");
    }
    #[test]
    fn view_3x3_real() {
        let a: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let m = NumericArray::new(vec![3, 3], MatlabType::from(a), None).unwrap();

        println!("{m}");
    }
    #[test]
    fn view_3x3x2x2_complex() {
        let a: Vec<f64> = vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 18.0,
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 18.0,
        ];
        let m = NumericArray::new(
            vec![3, 3, 2, 2],
            MatlabType::from(a.clone()),
            Some(MatlabType::from(a)),
        )
        .unwrap();

        println!("{m}");
    }
    #[test]
    fn numeric_array_view_3x3_complex() {
        let a: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let b: Vec<f64> = vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
        let m = NumericArray::new(vec![3, 3], MatlabType::from(a), Some(MatlabType::from(b))).unwrap();

        println!("{m}");
    }
    #[test]
    fn view_2x2x2_real_nested() {
        let matrix1_row1_raw = vec![1.0, 2.0];
        let matrix1_row2_raw = vec![3.0, 4.0];
        let matrix1_row1 = MatVariable::NumericArray(
            NumericArray::new(vec![], MatlabType::from(matrix1_row1_raw), None).unwrap(),
        );
        let matrix1_row2 = MatVariable::NumericArray(
            NumericArray::new(vec![], MatlabType::from(matrix1_row2_raw), None).unwrap(),
        );
        let matrix2_row1_raw = vec![5.0, 6.0];
        let matrix2_row2_raw = vec![7.0, 8.0];
        let matrix2_row1 = MatVariable::NumericArray(
            NumericArray::new(vec![], MatlabType::from(matrix2_row1_raw), None).unwrap(),
        );
        let matrix2_row2 = MatVariable::NumericArray(
            NumericArray::new(vec![], MatlabType::from(matrix2_row2_raw), None).unwrap(),
        );

        let matrix1 = MatVariable::NumericArray(
            NumericArray::from_nested_matvar(vec![], vec![matrix1_row1, matrix1_row2]).unwrap(),
        );
        let matrix2 = MatVariable::NumericArray(
            NumericArray::from_nested_matvar(vec![], vec![matrix2_row1, matrix2_row2]).unwrap(),
        );

        let m = MatVariable::NumericArray(
            NumericArray::from_nested_matvar(vec![], vec![matrix1, matrix2]).unwrap(),
        );

        assert_eq!(m.elem([0, 0, 0]).to_f64(), Some(1.0));
        assert_eq!(m.elem([0, 1, 0]).to_f64(), Some(2.0));
        assert_eq!(m.elem([1, 0, 0]).to_f64(), Some(3.0));
        assert_eq!(m.elem([1, 1, 0]).to_f64(), Some(4.0));
        assert_eq!(m.elem([0, 0, 1]).to_f64(), Some(5.0));
        assert_eq!(m.elem([0, 1, 1]).to_f64(), Some(6.0));
        assert_eq!(m.elem([1, 0, 1]).to_f64(), Some(7.0));
        assert_eq!(m.elem([1, 1, 1]).to_f64(), Some(8.0));

        println!("{m}");
    }
    #[test]
    fn view_9x2_real_nested() {
        let col1_raw: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let col2_raw: Vec<f64> = vec![11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0];
        let col1 = MatVariable::NumericArray(
            NumericArray::new(vec![9, 1], MatlabType::from(col1_raw), None).unwrap(),
        );
        let col2 = MatVariable::NumericArray(
            NumericArray::new(vec![9, 1], MatlabType::from(col2_raw), None).unwrap(),
        );

        let m =
            MatVariable::NumericArray(NumericArray::from_nested_matvar(vec![], vec![col1, col2]).unwrap());

        println!("{m}");

        assert_eq!(m.elem([0, 0]).to_f64(), Some(1.0));
        assert_eq!(m.elem([1, 0]).to_f64(), Some(2.0));
        assert_eq!(m.elem([2, 0]).to_f64(), Some(3.0));
        assert_eq!(m.elem([3, 0]).to_f64(), Some(4.0));
        assert_eq!(m.elem([4, 0]).to_f64(), Some(5.0));
        assert_eq!(m.elem([5, 0]).to_f64(), Some(6.0));
        assert_eq!(m.elem([6, 0]).to_f64(), Some(7.0));
        assert_eq!(m.elem([7, 0]).to_f64(), Some(8.0));
        assert_eq!(m.elem([8, 0]).to_f64(), Some(9.0));
        assert_eq!(m.elem([0, 1]).to_f64(), Some(11.0));
        assert_eq!(m.elem([1, 1]).to_f64(), Some(12.0));
        assert_eq!(m.elem([2, 1]).to_f64(), Some(13.0));
        assert_eq!(m.elem([3, 1]).to_f64(), Some(14.0));
        assert_eq!(m.elem([4, 1]).to_f64(), Some(15.0));
        assert_eq!(m.elem([5, 1]).to_f64(), Some(16.0));
        assert_eq!(m.elem([6, 1]).to_f64(), Some(17.0));
        assert_eq!(m.elem([7, 1]).to_f64(), Some(18.0));
        assert_eq!(m.elem([8, 1]).to_f64(), Some(19.0));
    }
    #[test]
    fn empty_array() {
        let m = NumericArray::new(vec![], MatlabType::new(), None).unwrap();

        println!("{m}");
    }
    #[test]
    fn numeric_array_view_1x9_real_no_size_hint() {
        let a: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let m = NumericArray::new(vec![], MatlabType::from(a), None).unwrap();

        println!("{m}");
    }
    #[test]
    fn to_sparse() {
        let a: Vec<f64> = vec![1.0, 0.0, 4.0, 0.0, 0.0, 3.0, 5.0, 7.0, 2.0, 0.0, 6.0, 0.0];
        let m = NumericArray::new(vec![4, 3], MatlabType::from(a), None).unwrap();

        let m_sparse = m.to_sparse().unwrap();

        assert_eq!(m_sparse.elem([0, 0]).to_f64(), Some(1.0));
        assert_eq!(m_sparse.elem([1, 0]).to_f64(), Some(0.0));
        assert_eq!(m_sparse.elem([2, 0]).to_f64(), Some(4.0));
        assert_eq!(m_sparse.elem([3, 0]).to_f64(), Some(0.0));
        assert_eq!(m_sparse.elem([0, 1]).to_f64(), Some(0.0));
        assert_eq!(m_sparse.elem([1, 1]).to_f64(), Some(3.0));
        assert_eq!(m_sparse.elem([2, 1]).to_f64(), Some(5.0));
        assert_eq!(m_sparse.elem([3, 1]).to_f64(), Some(7.0));
        assert_eq!(m_sparse.elem([0, 2]).to_f64(), Some(2.0));
        assert_eq!(m_sparse.elem([1, 2]).to_f64(), Some(0.0));
        assert_eq!(m_sparse.elem([2, 2]).to_f64(), Some(6.0));
        assert_eq!(m_sparse.elem([3, 2]).to_f64(), Some(0.0));
    }
}
