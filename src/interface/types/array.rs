use crate::interface::variable::MatVariable;

/// Trait methods that array types share
pub trait ArrayType {
    /// Get the dimension of the array
    fn dim(&self) -> &Vec<usize>;

    /// Get a borrowed value from a column-major index
    fn get_ref_colmaj(&self, index: usize) -> Option<&MatVariable>;

    /// Get a cloned value from a column-major index
    fn get_clone_colmaj(&self, index: usize) -> Option<MatVariable>;

    /// Get a borrowed value from a multi-dimensional index
    fn get_ref_multidim(&self, idx: &[usize]) -> Option<&MatVariable> {
        self.get_ref_colmaj(self.column_index(idx)?)
    }

    /// Get a cloned value from a multi-dimensional index
    fn get_clone_multidim(&self, idx: &[usize]) -> Option<MatVariable> {
        self.get_clone_colmaj(self.column_index(idx)?)
    }

    /// Get column-major index from multi-dimensional index
    fn column_index(&self, idx: &[usize]) -> Option<usize> {
        // The index must have the same size as dimension
        if self.dim().len() != idx.len() {
            return None;
        }
        // Index components cannot fall out of range
        for (i, v) in idx.iter().enumerate() {
            if self.dim()[i] <= *v {
                return None;
            }
        }

        let mut v_idx = 0;
        let mut stride = 1;

        for (i, dim) in idx.iter().zip(self.dim().iter()) {
            v_idx += i * stride;
            stride *= dim;
        }

        Some(v_idx)
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! impl_Array_for {
    ($type:ty) => {
        impl ArrayType for $type {
            fn dim(&self) -> &Vec<usize> {
                &self.dim
            }

            fn get_ref_colmaj(&self, index: usize) -> Option<&MatVariable> {
                self.value.get(index)
            }

            fn get_clone_colmaj(&self, _index: usize) -> Option<MatVariable> {
                unimplemented!()
            }
        }
    };
}
