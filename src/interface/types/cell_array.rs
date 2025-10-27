use crate::MatrwError;
use crate::impl_Array_for;
use crate::interface::types::array::ArrayType;
use crate::interface::variable::MatVariable;
use crate::parser::v7::types::cell_array::CellArray7;

#[derive(Debug, Clone)]
pub struct CellArray {
    pub dim: Vec<usize>,
    pub value: Vec<MatVariable>,
}

/// [`CellArray`] contains any kind of MatVariable in multidimensional arrays.
///
impl CellArray {
    pub fn new(dim: Vec<usize>, value: Vec<MatVariable>) -> Result<Self, MatrwError> {
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

        Ok(Self { dim, value })
    }
}

impl_Array_for!(CellArray);

impl From<CellArray7> for CellArray {
    fn from(value: CellArray7) -> Self {
        let dim = value.dim().into_iter().map(|x| x as usize).collect();
        let v = value.value().into_iter().map(|x| x.into()).collect();
        Self { dim, value: v }
    }
}
