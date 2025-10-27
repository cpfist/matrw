//! Module defining enum [`MatVariable`], which describes different MAT-file variable types.

use paste::paste;
use std::fmt::{Debug, Display};
use std::ops;

use crate::interface::index::Index;
use crate::interface::types::array::ArrayType;
use crate::interface::types::cell_array::CellArray;
use crate::interface::types::compressed_array::CompressedArray;
use crate::interface::types::matlab_types::{MatlabType, MatlabTypeMarker};
use crate::interface::types::numeric_array::NumericArray;
use crate::interface::types::sparse_array::SparseArray;
use crate::interface::types::structure::Structure;
use crate::interface::types::structure_array::StructureArray;
use crate::parser::v7::types::compressed_array::CompressedArray7;
use crate::parser::v7::variable7::MatVariable7;

/// MAT-file variable wrapper
#[derive(Debug, Clone)]
pub enum MatVariable {
    ///
    /// Full (dense) numeric arrays of arbitrary dimensions. Can contain the numeric types (`i8`, `i16`, `i32`, `i64`, `u8`, `u16`, `u32`, `u64`, `f32`, `f64`) and the character type (`char`).
    ///
    /// # Examples
    ///
    /// 1x1 `f64` scalar
    /// ```
    /// # use matrw::matvar;
    /// // Matlab: x1 = 1.;
    /// let x1 = matvar!(1.);
    /// ```
    ///
    /// 1x1 `u8` scalar
    /// ```
    /// # use matrw::matvar;
    /// // Matlab: x1u8 = uint8(1);
    /// let x1u8 = matvar!(1_u8);
    /// ```
    ///
    /// 1x2 `f64` vector
    /// ```
    /// # use matrw::matvar;
    /// // Matlab: x2 = [1.0, 2.0];
    /// let x2 = matvar!([1.0, 2.0]);
    /// ```
    ///
    /// 2x2 `f64` matrix
    /// ```
    /// # use matrw::matvar;
    /// // Matlab: x3 = [1.0, 2.0; 3.0, 4.0];
    /// let x3 = matvar!([[1.0, 2.0], [3.0, 4.0]]);
    /// ```
    ///
    /// 2x2x2 `f64` array
    /// ```
    /// # use matrw::matvar;
    /// // Matlab:
    /// // x4(:,:,1) = [1.0, 2.0; 3.0, 4.0];
    /// // x4(:,:,2) = [10.0, 20.0; 30.0, 40.0];
    /// let x4 = matvar!([
    ///     [[1.0, 2.0], [3.0, 4.0]],
    ///     [[10.0, 20.0], [30.0, 40.0]],
    ///     ]);
    /// ```
    ///
    /// # Conversion
    ///
    /// `NumericArray` data can be explicitly converted
    /// into
    /// - scalar types,
    /// - [`Vec`] types,
    /// - [`nalgebra::DMatrix`] (requires feature `nalgebra`).
    ///
    /// For example, `f64` data can be converted by
    /// - [`MatVariable::to_f64`], to return the first value of the data as scalar,
    /// - [`MatVariable::to_vec_f64`], to return a clone of the variable data,
    /// - [`MatVariable::to_na_matrix()`], to return a `DMatrix`.
    ///
    /// ```
    /// # use matrw::{matvar};
    /// let a = matvar!(42.);
    ///
    /// let val: Vec<f64> = a.to_vec_f64().unwrap();
    ///
    /// // Variable "a" contains f64 data, so explicit conversion to f32 fails
    /// assert!(a.to_vec_f32().is_none());
    /// ```
    ///
    /// Equivalent methods exist for conversion of other Rust types.
    ///
    /// <div class="warning">
    /// Explicit conversion only works if the method used matches the data, e.g. MAT-file "double" data must be read using *vec_f64() methods, otherwise None is returned.
    /// </div>
    ///
    /// The type of a numeric variable, [`MatlabType`], can be determined with the method [`MatVariable::numeric_type`].
    /// ```
    /// # use matrw::{matvar};
    /// let a = matvar!(42.);
    ///
    /// assert!(matches!(a.numeric_type().unwrap(), matrw::MatlabType::F64(_)));
    /// ```
    ///
    /// # Indexing
    ///
    /// Indexing into [`MatVariable::NumericArray`] is done with [`OwnedIndex::elem`] which
    /// returns a dynamically constructed 1x1 [`MatVariable::NumericArray`] containing the element data.
    /// The index trait [`std::ops::Index`] can't be used here, because it can only return references.
    ///
    /// ```
    /// use matrw::{matfile, matvar, OwnedIndex};
    ///
    /// let a = matvar!([1, 2, 3, 4]);
    /// assert_eq!(a.elem(0).to_i32(), Some(1));
    /// assert_eq!(a.elem(1).to_i32(), Some(2));
    /// assert_eq!(a.elem(2).to_i32(), Some(3));
    /// assert_eq!(a.elem(3).to_i32(), Some(4));
    ///
    /// let b = matvar!([[1, 2], [3, 4]]);
    /// // Indexing via 2D tuple ...
    /// assert_eq!(b.elem([0,0]).to_i32(), Some(1));
    /// assert_eq!(b.elem([0,1]).to_i32(), Some(2));
    /// assert_eq!(b.elem([1,0]).to_i32(), Some(3));
    /// assert_eq!(b.elem([1,1]).to_i32(), Some(4));
    /// // ... or 1D column-major index
    /// assert_eq!(b.elem(0).to_i32(), Some(1));
    /// assert_eq!(b.elem(1).to_i32(), Some(3));
    /// assert_eq!(b.elem(2).to_i32(), Some(2));
    /// assert_eq!(b.elem(3).to_i32(), Some(4));
    /// ```
    ///
    NumericArray(NumericArray),
    ///
    /// Sparse arrays of dimension 2. Can contain numeric types `f64` or `bool`.
    ///
    /// # Examples
    ///
    /// 1x1 sparse `f64`
    /// ```
    /// # use matrw::matvar;
    /// // Matlab: x1 = sparse(1.);
    /// let x1 = matvar!(1.).to_sparse();
    /// ```
    ///
    /// 10x1 sparse `f64`
    /// ```
    /// # use matrw::matvar;
    /// // Matlab: x2 = sparse([1, 10], [1, 1], [42., 43.]);
    /// let x2 = matvar!([42., 0., 0., 0., 0., 0., 0., 0., 0., 43.]).to_sparse();
    /// ```
    ///
    /// # Indexing
    ///
    /// Indexing into [`MatVariable::SparseArray`] is done with [`OwnedIndex::elem`] which
    /// returns a dynamically constructed 1x1 [`MatVariable::NumericArray`] containing the element data.
    /// The index trait [`std::ops::Index`] can't be used here, because it can only return references.
    ///
    /// ```
    /// # use matrw::{matfile, matvar, OwnedIndex};
    /// #
    /// let b = matvar!([[1.0, 2.0], [3.0, 4.0]]).to_sparse().unwrap();
    ///
    /// // Indexing via 2D tuple
    /// assert_eq!(b.elem([0,0]).to_f64(), Some(1.0));
    /// assert_eq!(b.elem([0,1]).to_f64(), Some(2.0));
    /// assert_eq!(b.elem([1,0]).to_f64(), Some(3.0));
    /// assert_eq!(b.elem([1,1]).to_f64(), Some(4.0));
    /// ```
    ///
    SparseArray(SparseArray),
    ///
    /// Key-value structures in arrays of arbitrary dimensions.
    ///
    /// # Examples
    /// ```
    /// # use matrw::matvar;
    /// // Matlab:
    /// // s = [
    /// //      struct('a', 1., 'b', 2.),
    /// //      struct('a', 42., 'b', 43.)
    /// //      ];
    /// let s = matvar!([
    ///         { a: 1.0, b: 2.0 },
    ///         { a: 42.0, b: 43.0 },
    ///         ]);
    /// ```
    ///
    /// # Indexing
    ///
    /// ```
    /// # use matrw::matvar;
    /// #
    /// # let var = matvar!([
    /// #         {
    /// #             a: 1.0,
    /// #             b: 2.0,
    /// #         },
    /// #         {
    /// #             a: 42.0,
    /// #             b: 43.0,
    /// #         },
    /// #         ]);
    /// // Get field b of the second array element
    /// assert_eq!(var[1]["b"], matvar!(43.));
    /// ```
    ///
    StructureArray(StructureArray),
    ///
    /// Contains mixed MatVariable kinds in arrays of arbitrary dimensions.
    ///
    /// # Example
    ///
    /// ```
    /// // Matlab: c = { 'some text', struct('a', 42.0, 'b', 43.0) };
    /// # use matrw::matvar;
    /// #
    /// let c = matvar!([
    ///         "some text",
    ///         { a: 42.0, b: 43.0 },
    ///         ]);
    /// ```
    ///
    /// # Indexing
    ///
    /// ```
    /// # use matrw::matvar;
    /// #
    /// # let c = matvar!([
    /// #         "some text",
    /// #         { a: 42.0, b: 43.0 },
    /// #         ]);
    /// assert_eq!(c[0], matvar!("some text"));
    /// assert_eq!(c[1]["a"], matvar!(42.0));
    /// assert_eq!(c[1]["b"], matvar!(43.0));
    /// ```
    ///
    CellArray(CellArray),
    // ------------------------
    ///
    /// Support type describing scalar structure. Used in [`MatVariable::StructureArray`].
    ///
    /// # Example
    ///
    /// ```
    /// // Matlab: s = struct('a', 42.0, 'b', 43.0);
    /// # use matrw::matvar;
    /// #
    /// let s = matvar!(
    ///         { a: 42.0, b: 43.0 }
    ///         );
    /// ```
    ///
    /// # Indexing
    ///
    /// ```
    /// # use matrw::matvar;
    /// #
    /// let s = matvar!(
    ///         { a: 42.0, b: 43.0 }
    ///         );
    /// assert_eq!(s["a"], matvar!(42.0));
    /// assert_eq!(s["b"], matvar!(43.0));
    /// ```
    ///
    Structure(Structure),
    ///
    /// Null type used as return type for non-existing index
    ///
    /// # Example
    ///
    /// ```
    /// # use matrw::{matvar, MatVariable};
    /// #
    /// let s = matvar!(
    ///         { a: 42.0, b: 43.0 }
    ///         );
    /// assert_eq!(s["a"], matvar!(42.0));
    /// assert_eq!(s["b"], matvar!(43.0));
    /// // Index "c" does not exist
    /// assert!(matches!(s["c"], MatVariable::Null));
    /// ```
    ///
    ///
    Null,
    ///
    /// Wrapper type which applies compression on write when using option `compress=true` in function
    /// [`crate::save_matfile_v7`].
    ///
    /// <div class="warning">
    /// This type is used implicitly. It should not be used manually.
    /// </div>
    ///
    Compressed(CompressedArray),
    ///
    /// Support type used for description of unsupported types.
    ///
    Unsupported,
}

impl MatVariable {
    /// Get array dimensions.
    ///
    /// # Example
    ///
    /// ```
    /// # use matrw::matvar;
    /// let var = matvar!([[1.0, 2.0], [42.0, 43.0]]);
    ///
    /// assert_eq!(var.dim(), vec![2, 2]);
    /// ```
    ///
    pub fn dim(&self) -> Vec<usize> {
        match self {
            MatVariable::NumericArray(val) => val.dim.clone(),
            MatVariable::CellArray(val) => val.dim.clone(),
            MatVariable::Structure(_) => vec![1, 1],
            MatVariable::StructureArray(val) => val.dim.clone(),
            MatVariable::SparseArray(val) => val.dim.clone(),
            _ => unimplemented!(),
        }
    }

    /// If [`MatVariable`] is of type [`MatVariable::NumericArray`] or
    /// [`MatVariable::SparseArray`], return numeric type. Otherwise [`None`].
    ///
    /// # Example
    ///
    /// ```
    /// # use matrw::matvar;
    /// let var = matvar!([[1.0, 2.0], [42.0, 43.0]]);
    ///
    /// assert!(matches!(var.numeric_type().unwrap(), matrw::MatlabType::F64(_)));
    /// ```
    ///
    pub fn numeric_type(&self) -> Option<&MatlabType> {
        match self {
            MatVariable::NumericArray(val) => Some(val.numeric_type()),
            MatVariable::SparseArray(val) => Some(val.numeric_type()),
            _ => None,
        }
    }

    /// If [`MatVariable`] is of type [`MatVariable::Structure`] or
    /// [`MatVariable::StructureArray`], return field names. Otherwise [`None`].
    ///
    /// # Example
    ///
    /// ```
    /// # use matrw::matvar;
    /// let var = matvar!({ a: 1., b: 2. });
    ///
    /// assert_eq!(var.fieldnames(), Some(vec!["a".to_string(), "b".to_string()]));
    /// ```
    ///
    pub fn fieldnames(&self) -> Option<Vec<String>> {
        match self {
            MatVariable::Structure(val) => Some(val.fieldnames()),
            MatVariable::StructureArray(val) => Some(val.fieldnames()),
            _ => None,
        }
    }

    /// If [`MatVariable`] is of type [`MatVariable::NumericArray`] or
    /// [`MatVariable::SparseArray`], return if variable is complex. Otherwise [`None`].
    ///
    /// # Example
    ///
    /// ```
    /// # use matrw::matvar;
    /// let var = matvar!(1.0);
    ///
    /// assert_eq!(var.is_complex(), Some(false));
    /// ```
    ///
    pub fn is_complex(&self) -> Option<bool> {
        match self {
            MatVariable::NumericArray(val) => Some(val.is_complex()),
            MatVariable::SparseArray(val) => Some(val.is_complex()),
            _ => None,
        }
    }

    /// If [`MatVariable`] is of type [`MatVariable::NumericArray`],
    /// return real part as scalar value. Otherwise, returns [`None`].
    ///
    /// If `NumericArray` contains more than one value, the first value is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use matrw::matvar;
    /// let var = matvar!(1.0);
    ///
    /// assert_eq!(var.to_scalar(), Some(1.0));
    /// ```
    ///
    pub fn to_scalar<T: MatlabTypeMarker>(&self) -> Option<T> {
        match self {
            MatVariable::NumericArray(val) => val.real_to_scalar(),
            _ => None,
        }
    }

    /// If [`MatVariable`] is of type [`MatVariable::NumericArray`],
    /// return complex part as scalar value. Otherwise, returns [`None`].
    ///
    /// If `NumericArray` contains more than one value, the first value is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use matrw::matvar;
    /// let var = matvar!((1.0, 42.0));
    ///
    /// assert_eq!(var.comp_to_scalar(), Some(42.0));
    /// ```
    ///
    pub fn comp_to_scalar<T: MatlabTypeMarker>(&self) -> Option<T> {
        match self {
            MatVariable::NumericArray(val) => val.comp_to_scalar(),
            _ => None,
        }
    }

    /// If [`MatVariable`] is of type [`MatVariable::NumericArray`],
    /// return cloned inner real `Vec`. Otherwise, returns [`None`].
    ///
    /// # Example
    ///
    /// ```
    /// # use matrw::matvar;
    /// let var = matvar!([1.0, 2.0, 3.0]);
    ///
    /// assert_eq!(var.to_vec(), Some(vec![1.0, 2.0, 3.0]));
    /// ```
    ///
    pub fn to_vec<T: MatlabTypeMarker>(&self) -> Option<Vec<T>> {
        match self {
            MatVariable::NumericArray(val) => val.real_to_vec(),
            _ => None,
        }
    }

    /// If [`MatVariable`] is of type [`MatVariable::NumericArray`],
    /// return cloned inner complex `Vec`. Otherwise, returns [`None`].
    ///
    /// # Example
    ///
    /// ```
    /// # use matrw::matvar;
    /// let var = matvar!([(1.0, 42.), (2.0, 43.), (3.0, 44.)]);
    ///
    /// assert_eq!(var.comp_to_vec(), Some(vec![42.0, 43.0, 44.0]));
    /// ```
    ///
    pub fn comp_to_vec<T: MatlabTypeMarker>(&self) -> Option<Vec<T>> {
        match self {
            MatVariable::NumericArray(val) => val.comp_to_vec(),
            _ => None,
        }
    }

    /// If [`MatVariable`] is of type [`MatVariable::NumericArray`],
    /// return sparse transformation. Otherwise, returns [`None`].
    ///
    /// # Example
    ///
    /// ```
    /// # use matrw::{matvar, OwnedIndex};
    /// let var = matvar!([1.0, 2.0, 3.0]).to_sparse().unwrap();
    ///
    /// assert_eq!(var.elem(0), matvar!(1.0));
    /// assert_eq!(var.elem(1), matvar!(2.0));
    /// assert_eq!(var.elem(2), matvar!(3.0));
    /// ```
    ///
    pub fn to_sparse(self) -> Option<MatVariable> {
        match self {
            MatVariable::NumericArray(val) => val.to_sparse(),
            _ => None,
        }
    }

    /// Return iterator over all elements in column-major order.
    ///
    /// # Example
    ///
    /// ```
    /// # use matrw::{matvar, OwnedIndex};
    /// let var = matvar!([[1.0, 2.0], [42.0, 43.0]]);
    /// let mut var_iter = var.iter();
    ///
    /// assert_eq!(var_iter.next(), Some(matvar!(1.0)));
    /// assert_eq!(var_iter.next(), Some(matvar!(42.0)));
    /// assert_eq!(var_iter.next(), Some(matvar!(2.0)));
    /// assert_eq!(var_iter.next(), Some(matvar!(43.0)));
    /// ```
    ///
    pub fn iter(&self) -> MatVariableIterator<'_> {
        MatVariableIterator::new(self)
    }
}

macro_rules! impl_MatVariable_to {
    ($($ret: ty),*) => {
        paste! {
            $(
            //
            // to_<$ret>
            //
            #[doc = concat!("If [`MatVariable`] is of type [`MatVariable::NumericArray`], returns copied `", stringify!($ret),"`. Otherwise, returns [`None`].")]
            pub fn [<to_ $ret>](&self) -> Option<$ret> {
                match self {
                    MatVariable::NumericArray(val) if val.is_scalar() => val.real_to_scalar(),
                    _ => None,
                }
            }
            )*
        }
    };
}

macro_rules! impl_MatVariable_comp_to {
    ($($ret: ty),*) => {
        paste! {
            $(
            //
            // comp_to_<$ret>
            //
            #[doc = concat!("If [`MatVariable`] is of type [`MatVariable::NumericArray`], returns copied `", stringify!($ret),"`. Otherwise, returns [`None`].")]
            pub fn [<comp_to_ $ret>](&self) -> Option<$ret> {
                match self {
                    MatVariable::NumericArray(val) if val.is_scalar() => val.comp_to_scalar(),
                    _ => None,
                }
            }
            )*
        }
    };
}

macro_rules! impl_MatVariable_to_vec {
    ($($ret: ty),*) => {
        paste! {
            $(
            //
            // to_vec_<$ret>
            //
            #[doc = concat!("If [`MatVariable`] is of type [`MatVariable::NumericArray`], returns cloned `Vec<", stringify!($ret),">`. Otherwise, returns [`None`].")]
            pub fn [<to_vec_ $ret>](&self) -> Option<Vec<$ret>> {
                match self {
                    MatVariable::NumericArray(val) => val.real_to_vec::<$ret>(),
                    _ => None,
                }
            }
            )*
        }
    };
}

macro_rules! impl_MatVariable_comp_to_vec {
    ($($ret: ty),*) => {
        paste! {
            $(
            //
            // comp_to_vec_<$ret>
            //
            #[doc = concat!("If [`MatVariable`] is of type [`MatVariable::NumericArray`], returns complex part as cloned `Vec<", stringify!($ret),">`. Otherwise, returns [`None`].")]
            pub fn [<comp_to_vec_ $ret>](&self) -> Option<Vec<$ret>> {
                match self {
                    MatVariable::NumericArray(val) => val.comp_to_vec::<$ret>(),
                    _ => None,
                }
            }
            )*
        }
    };
}

impl MatVariable {
    impl_MatVariable_to!(u8, i8, u16, i16, u32, i32, u64, i64, f32, f64, char, bool);
    impl_MatVariable_comp_to!(u8, i8, u16, i16, u32, i32, u64, i64, f32, f64, char, bool);
    impl_MatVariable_to_vec!(u8, i8, u16, i16, u32, i32, u64, i64, f32, f64, char, bool);
    impl_MatVariable_comp_to_vec!(u8, i8, u16, i16, u32, i32, u64, i64, f32, f64, char, bool);
}

// ============================================================================
// Index
// ============================================================================

pub trait OwnedIndex<Idx> {
    type Output;
    fn elem(&self, index: Idx) -> Self::Output;
}

static NULL: MatVariable = MatVariable::Null;

impl<T> OwnedIndex<T> for MatVariable
where
    T: Index,
{
    type Output = MatVariable;

    fn elem(&self, index: T) -> Self::Output {
        index.index_into_clone(self).unwrap_or(NULL.clone())
    }
}

impl<T> ops::Index<T> for MatVariable
where
    T: Index,
{
    type Output = MatVariable;

    fn index(&self, index: T) -> &Self::Output {
        index.index_into_ref(self).unwrap_or(&NULL)
    }
}

// ============================================================================
// Iterator
// ============================================================================

pub struct MatVariableIterator<'a> {
    var: &'a MatVariable,
    count: usize,
}

impl<'a> MatVariableIterator<'a> {
    fn new(var: &'a MatVariable) -> Self {
        Self { var, count: 0 }
    }
}

impl<'a> Iterator for MatVariableIterator<'a> {
    type Item = MatVariable;

    fn next(&mut self) -> Option<Self::Item> {
        match self.var {
            MatVariable::NumericArray(v) => {
                let ret = if self.count < v.value.len() {
                    Some(v.get_clone_colmaj(self.count).unwrap())
                } else {
                    None
                };
                self.count += 1;
                ret
            }
            _ => todo!(),
        }
    }
}

pub struct MatVariableIntoIterator {
    var: MatVariable,
    count: usize,
}

impl Iterator for MatVariableIntoIterator {
    type Item = MatVariable;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.var {
            MatVariable::NumericArray(v) => {
                let ret = if self.count < v.value.len() {
                    Some(v.get_clone_colmaj(self.count).unwrap())
                } else {
                    None
                };
                self.count += 1;
                ret
            }
            _ => todo!(),
        }
    }
}

impl IntoIterator for MatVariable {
    type Item = MatVariable;
    type IntoIter = MatVariableIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        match &self {
            MatVariable::NumericArray(_) => MatVariableIntoIterator { var: self, count: 0 },
            _ => todo!(),
        }
    }
}

// ============================================================================
// From
// ============================================================================

/// Create a `MatVariable` from `&str`.
///
/// # Example
///
/// ```
/// # use matrw::MatVariable;
/// let s = MatVariable::from("test");
/// ```
impl From<&str> for MatVariable {
    fn from(value: &str) -> Self {
        MatVariable::NumericArray(NumericArray::from(value))
    }
}

/// Create a `MatVariable` from `Vec<T>`.
///
/// # Example
///
/// ```
/// # use matrw::MatVariable;
/// let s = MatVariable::from(vec![1.,2.,3.]);
/// ```
impl<T> From<Vec<T>> for MatVariable
where
    T: MatlabTypeMarker,
{
    fn from(value: Vec<T>) -> Self {
        MatVariable::NumericArray(
            NumericArray::new(vec![1, value.len()], MatlabType::from(value), None)
                .expect("Could not create NumericArray."),
        )
    }
}

/// Create a `MatVariable` from `Vec<(T, T)>`.
///
/// # Example
///
/// ```
/// # use matrw::MatVariable;
/// let s = MatVariable::from(vec![(1., 1.), (2., 1.), (3., 1.)]);
/// ```
impl<T> From<Vec<(T, T)>> for MatVariable
where
    T: MatlabTypeMarker,
{
    fn from(value: Vec<(T, T)>) -> Self {
        let real = value.iter().map(|x| x.0).collect::<Vec<T>>();
        let comp = value.iter().map(|x| x.1).collect::<Vec<T>>();

        MatVariable::NumericArray(
            NumericArray::new(
                vec![1, value.len()],
                MatlabType::from(real),
                Some(MatlabType::from(comp)),
            )
            .expect("Could not create NumericArray."),
        )
    }
}

/// Create a `MatVariable` from `Vec<(T, T)>`.
///
/// # Example
///
/// ```
/// # use matrw::MatVariable;
/// let s = MatVariable::from(1.);
/// ```
impl<T> From<T> for MatVariable
where
    T: MatlabTypeMarker,
{
    fn from(value: T) -> Self {
        MatVariable::NumericArray(
            NumericArray::new(vec![1, 1], MatlabType::from(vec![value]), None)
                .expect("Could not create NumericArray."),
        )
    }
}

/// Create a `MatVariable` from `Vec<(T, T)>`.
///
/// # Example
///
/// ```
/// # use matrw::MatVariable;
/// let s = MatVariable::from((1., 1.));
/// ```
impl<T> From<(T, T)> for MatVariable
where
    T: MatlabTypeMarker,
{
    fn from(value: (T, T)) -> Self {
        MatVariable::NumericArray(
            NumericArray::new(
                vec![1, 1],
                MatlabType::from(vec![value.0]),
                Some(MatlabType::from(vec![value.1])),
            )
            .expect("Could not create NumericArray."),
        )
    }
}

impl From<MatVariable7> for MatVariable {
    fn from(value: MatVariable7) -> Self {
        match value {
            MatVariable7::Compressed(v) => MatVariable::from(v),
            MatVariable7::Numeric(v) => MatVariable::NumericArray(NumericArray::from(v)),
            MatVariable7::Cell(v) => MatVariable::CellArray(CellArray::from(v)),
            MatVariable7::Structure(v) => MatVariable::Structure(Structure::from(v)),
            MatVariable7::StructureArray(v) => MatVariable::StructureArray(StructureArray::from(v)),
            MatVariable7::Sparse(v) => MatVariable::SparseArray(SparseArray::from(v)),
            MatVariable7::ObjectMCOS(_) => MatVariable::Unsupported,
            MatVariable7::ObjectHandle(_) => MatVariable::Unsupported,
            MatVariable7::Empty(_) => MatVariable::NumericArray(
                NumericArray::new(vec![0, 0], MatlabType::new(), None)
                    .expect("Could not create NumericArray."),
            ),
        }
    }
}

impl From<CompressedArray7> for MatVariable {
    fn from(value: CompressedArray7) -> Self {
        match value.value() {
            MatVariable7::Compressed(v) => MatVariable::from(v),
            MatVariable7::Numeric(v) => MatVariable::NumericArray(NumericArray::from(v)),
            MatVariable7::Cell(v) => MatVariable::CellArray(CellArray::from(v)),
            MatVariable7::Structure(v) => MatVariable::Structure(Structure::from(v)),
            MatVariable7::StructureArray(v) => MatVariable::StructureArray(StructureArray::from(v)),
            MatVariable7::Sparse(v) => MatVariable::SparseArray(SparseArray::from(v)),
            MatVariable7::ObjectMCOS(_) => MatVariable::Unsupported,
            MatVariable7::ObjectHandle(_) => MatVariable::Unsupported,
            MatVariable7::Empty(_) => MatVariable::NumericArray(
                NumericArray::new(vec![0, 0], MatlabType::new(), None)
                    .expect("Could not create NumericArray."),
            ),
        }
    }
}

// ============================================================================
// Other trait implementations
// ============================================================================

impl Display for MatVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatVariable::NumericArray(v) => write!(f, "{}", v),
            MatVariable::CellArray(_v) => todo!(),
            MatVariable::Structure(_v) => todo!(),
            MatVariable::StructureArray(_v) => todo!(),
            MatVariable::SparseArray(v) => write!(f, "{}", v),
            MatVariable::Null => todo!(),
            MatVariable::Compressed(_v) => todo!(),
            MatVariable::Unsupported => todo!(),
        }
    }
}

#[allow(unused)]
impl PartialEq for MatVariable {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::NumericArray(l0), Self::NumericArray(r0)) => l0 == r0,
            (Self::CellArray(l0), Self::CellArray(r0)) => todo!(),
            (Self::Structure(l0), Self::Structure(r0)) => todo!(),
            (Self::StructureArray(l0), Self::StructureArray(r0)) => todo!(),
            (Self::SparseArray(l0), Self::SparseArray(r0)) => todo!(),
            (Self::Compressed(l0), Self::Compressed(r0)) => todo!(),
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_variable_size() {
        println!("MatVariable size: {}", size_of::<MatVariable>());
    }
}
