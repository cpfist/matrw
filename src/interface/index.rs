use crate::interface::types::array::ArrayType;
use crate::interface::variable::MatVariable;

pub trait Index: private::Sealed {
    fn index_into_clone(&self, v: &MatVariable) -> Option<MatVariable>;
    fn index_into_ref<'a>(&self, v: &'a MatVariable) -> Option<&'a MatVariable>;
}

impl Index for usize {
    fn index_into_clone(&self, v: &MatVariable) -> Option<MatVariable> {
        match v {
            MatVariable::NumericArray(n) => n.get_clone_colmaj(*self),
            MatVariable::SparseArray(n) => n.get_clone_colmaj(*self),
            _ => None,
        }
    }
    fn index_into_ref<'a>(&self, v: &'a MatVariable) -> Option<&'a MatVariable> {
        match v {
            MatVariable::CellArray(n) => n.get_ref_colmaj(*self),
            MatVariable::StructureArray(n) => n.get_ref_colmaj(*self),
            _ => None,
        }
    }
}

macro_rules! array_index {
    ($c:expr, $($args:expr),*) => {

        impl Index for [usize; $c] {
            fn index_into_clone(&self, v: &MatVariable) -> Option<MatVariable> {
                match v {
                    MatVariable::NumericArray(n) => n.get_clone_multidim(&[$(self[$args]),*]),
                    MatVariable::SparseArray(n) => n.get_clone_multidim(&[$(self[$args]),*]),
                    _ => None,
                }
            }
            fn index_into_ref<'a>(&self, v: &'a MatVariable) -> Option<&'a MatVariable> {
                match v {
                    MatVariable::CellArray(n) => n.get_ref_multidim(&[$(self[$args]),*]),
                    MatVariable::StructureArray(n) => n.get_ref_multidim(&[$(self[$args]),*]),
                    _ => None,
                }
            }
        }

    }
}

array_index! {2, 0, 1}
array_index! {3, 0, 1, 2}
array_index! {4, 0, 1, 2, 3}
array_index! {5, 0, 1, 2, 3, 4}
array_index! {6, 0, 1, 2, 3, 4, 5}

impl Index for &[usize] {
    fn index_into_clone(&self, v: &MatVariable) -> Option<MatVariable> {
        match v {
            MatVariable::NumericArray(n) => n.get_clone_multidim(self),
            MatVariable::SparseArray(n) => n.get_clone_multidim(self),
            _ => None,
        }
    }
    fn index_into_ref<'a>(&self, v: &'a MatVariable) -> Option<&'a MatVariable> {
        match v {
            MatVariable::CellArray(n) => n.get_ref_multidim(self),
            MatVariable::StructureArray(n) => n.get_ref_multidim(self),
            _ => None,
        }
    }
}

impl Index for &str {
    fn index_into_clone(&self, _v: &MatVariable) -> Option<MatVariable> {
        todo!()
    }
    fn index_into_ref<'a>(&self, v: &'a MatVariable) -> Option<&'a MatVariable> {
        match v {
            MatVariable::Structure(n) => n.get(self),
            _ => None,
        }
    }
}

mod private {
    pub trait Sealed {}
    impl Sealed for usize {}
    impl Sealed for (usize, usize) {}
    impl Sealed for &[usize] {}
    impl Sealed for [usize; 2] {}
    impl Sealed for [usize; 3] {}
    impl Sealed for [usize; 4] {}
    impl Sealed for [usize; 5] {}
    impl Sealed for [usize; 6] {}
    impl Sealed for str {}
    impl Sealed for String {}
    impl<T> Sealed for &T where T: ?Sized + Sealed {}
}
