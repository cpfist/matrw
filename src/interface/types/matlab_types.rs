//! Module defining [`MatlabType`]
//!
//! This module defines [`MatlabType`], a wrapper for data types allowed in MAT-files. It prevents
//! having a generic type parameter in [`crate::MatVariable`].

use std::fmt::Display;

/// Numeric types in MAT-files
#[derive(Debug, Clone, PartialEq)]
pub enum MatlabType {
    U8(Vec<u8>),
    I8(Vec<i8>),
    U16(Vec<u16>),
    I16(Vec<i16>),
    U32(Vec<u32>),
    I32(Vec<i32>),
    U64(Vec<u64>),
    I64(Vec<i64>),
    F32(Vec<f32>),
    F64(Vec<f64>),
    UTF8(Vec<char>),
    UTF16(Vec<char>),
    BOOL(Vec<bool>),
}

impl MatlabType {
    ///
    /// Construct a new empty [`MatlabType`].
    ///
    pub fn new() -> Self {
        Self::F64(Vec::new())
    }

    pub fn inner<T: MatlabTypeMarker>(self) -> Option<Vec<T>> {
        T::inner(self)
    }

    pub fn is_empty(&self) -> bool {
        match self {
            U8(items) => items.is_empty(),
            I8(items) => items.is_empty(),
            U16(items) => items.is_empty(),
            I16(items) => items.is_empty(),
            U32(items) => items.is_empty(),
            I32(items) => items.is_empty(),
            U64(items) => items.is_empty(),
            I64(items) => items.is_empty(),
            F32(items) => items.is_empty(),
            F64(items) => items.is_empty(),
            UTF8(items) => items.is_empty(),
            UTF16(items) => items.is_empty(),
            BOOL(items) => items.is_empty(),
        }
    }

    pub fn get<T: FromMatlabType>(&self, index: usize) -> Option<&T> {
        T::inner_ref(self).unwrap().get(index)
    }

    pub fn clone_at_index(&self, index: usize) -> MatlabType {
        match self {
            U8(items) => MatlabType::from(vec![items[index]]),
            I8(items) => MatlabType::from(vec![items[index]]),
            U16(items) => MatlabType::from(vec![items[index]]),
            I16(items) => MatlabType::from(vec![items[index]]),
            U32(items) => MatlabType::from(vec![items[index]]),
            I32(items) => MatlabType::from(vec![items[index]]),
            U64(items) => MatlabType::from(vec![items[index]]),
            I64(items) => MatlabType::from(vec![items[index]]),
            F32(items) => MatlabType::from(vec![items[index]]),
            F64(items) => MatlabType::from(vec![items[index]]),
            UTF8(items) => MatlabType::from(vec![items[index]]),
            UTF16(items) => MatlabType::from(vec![items[index]]),
            BOOL(items) => MatlabType::from(vec![items[index]]),
        }
    }

    pub fn row_vec_to_colmaj(value: MatlabType, n_rows: usize, n_cols: usize) -> MatlabType {
        match value {
            U8(items) => Self::row_vec_to_colmaj_interal(items, n_rows, n_cols),
            I8(items) => Self::row_vec_to_colmaj_interal(items, n_rows, n_cols),
            U16(items) => Self::row_vec_to_colmaj_interal(items, n_rows, n_cols),
            I16(items) => Self::row_vec_to_colmaj_interal(items, n_rows, n_cols),
            U32(items) => Self::row_vec_to_colmaj_interal(items, n_rows, n_cols),
            I32(items) => Self::row_vec_to_colmaj_interal(items, n_rows, n_cols),
            U64(items) => Self::row_vec_to_colmaj_interal(items, n_rows, n_cols),
            I64(items) => Self::row_vec_to_colmaj_interal(items, n_rows, n_cols),
            F32(items) => Self::row_vec_to_colmaj_interal(items, n_rows, n_cols),
            F64(items) => Self::row_vec_to_colmaj_interal(items, n_rows, n_cols),
            UTF8(items) => Self::row_vec_to_colmaj_interal(items, n_rows, n_cols),
            UTF16(items) => Self::row_vec_to_colmaj_interal(items, n_rows, n_cols),
            BOOL(items) => Self::row_vec_to_colmaj_interal(items, n_rows, n_cols),
        }
    }

    fn row_vec_to_colmaj_interal<T: MatlabTypeMarker>(value: Vec<T>, n_rows: usize, n_cols: usize) -> Self {
        let mut v = value.clone();
        for r in 0..n_rows {
            for c in 0..n_cols {
                v[c * n_rows + r] = value[r * n_cols + c];
            }
        }
        MatlabType::from(v)
    }

    pub fn to_sparse(self, n_rows: usize, n_cols: usize) -> (Vec<usize>, Vec<usize>, Self) {
        match self {
            U8(items) => Self::to_sparse_internal(items, n_rows, n_cols),
            I8(items) => Self::to_sparse_internal(items, n_rows, n_cols),
            U16(items) => Self::to_sparse_internal(items, n_rows, n_cols),
            I16(items) => Self::to_sparse_internal(items, n_rows, n_cols),
            U32(items) => Self::to_sparse_internal(items, n_rows, n_cols),
            I32(items) => Self::to_sparse_internal(items, n_rows, n_cols),
            U64(items) => Self::to_sparse_internal(items, n_rows, n_cols),
            I64(items) => Self::to_sparse_internal(items, n_rows, n_cols),
            F32(items) => Self::to_sparse_internal(items, n_rows, n_cols),
            F64(items) => Self::to_sparse_internal(items, n_rows, n_cols),
            UTF8(items) => Self::to_sparse_internal(items, n_rows, n_cols),
            UTF16(items) => Self::to_sparse_internal(items, n_rows, n_cols),
            BOOL(items) => Self::to_sparse_internal(items, n_rows, n_cols),
        }
    }

    fn to_sparse_internal<T: MatlabTypeMarker>(
        value: Vec<T>,
        n_rows: usize,
        n_cols: usize,
    ) -> (Vec<usize>, Vec<usize>, Self) {
        let mut ir = vec![];
        let mut jc = vec![0];
        let mut v = vec![];

        let stride = n_rows;
        for j in 0..n_cols {
            for i in 0..n_rows {
                let elem = &value[i + j * stride];
                if !elem.is_zero() {
                    v.push(*elem);
                    ir.push(i);
                }
            }
            jc.push(ir.len());
        }

        (ir, jc, MatlabType::from(v))
    }

    pub fn print(&self, f: &mut std::fmt::Formatter<'_>, index: usize) -> std::fmt::Result {
        match self {
            U8(items) => write!(f, "{:12.4}", items[index]),
            I8(items) => write!(f, "{:12.4}", items[index]),
            U16(items) => write!(f, "{:12.4}", items[index]),
            I16(items) => write!(f, "{:12.4}", items[index]),
            U32(items) => write!(f, "{:12.4}", items[index]),
            I32(items) => write!(f, "{:12.4}", items[index]),
            U64(items) => write!(f, "{:12.4}", items[index]),
            I64(items) => write!(f, "{:12.4}", items[index]),
            F32(items) => write!(f, "{:12.4}", items[index]),
            F64(items) => write!(f, "{:12.4}", items[index]),
            UTF8(items) => write!(f, "{:12.4}", items[index]),
            UTF16(items) => write!(f, "{:12.4}", items[index]),
            BOOL(items) => write!(f, "{:12.4}", items[index]),
        }
    }

    pub fn extend(&mut self, other: MatlabType) {
        match (self, other) {
            (U8(v), U8(items)) => v.extend(items),
            (I8(v), I8(items)) => v.extend(items),
            (U16(v), U16(items)) => v.extend(items),
            (I16(v), I16(items)) => v.extend(items),
            (U32(v), U32(items)) => v.extend(items),
            (I32(v), I32(items)) => v.extend(items),
            (U64(v), U64(items)) => v.extend(items),
            (I64(v), I64(items)) => v.extend(items),
            (F32(v), F32(items)) => v.extend(items),
            (F64(v), F64(items)) => v.extend(items),
            (UTF8(v), UTF8(items)) => v.extend(items),
            (UTF16(v), UTF16(items)) => v.extend(items),
            (BOOL(v), BOOL(items)) => v.extend(items),
            _ => panic!(),
        }
    }

    pub fn join(vec: Vec<Self>) -> Option<Self> {
        let mut out = match vec.first().unwrap() {
            U8(_) => MatlabType::U8(Vec::new()),
            I8(_) => MatlabType::I8(Vec::new()),
            U16(_) => MatlabType::U16(Vec::new()),
            I16(_) => MatlabType::I16(Vec::new()),
            U32(_) => MatlabType::U32(Vec::new()),
            I32(_) => MatlabType::I32(Vec::new()),
            U64(_) => MatlabType::U64(Vec::new()),
            I64(_) => MatlabType::I64(Vec::new()),
            F32(_) => MatlabType::F32(Vec::new()),
            F64(_) => MatlabType::F64(Vec::new()),
            UTF8(_) => MatlabType::UTF8(Vec::new()),
            UTF16(_) => MatlabType::UTF16(Vec::new()),
            BOOL(_) => MatlabType::BOOL(Vec::new()),
        };

        for v in vec {
            out.extend(v);
        }

        Some(out)
    }

    pub fn len(&self) -> usize {
        match self {
            U8(items) => items.len(),
            I8(items) => items.len(),
            U16(items) => items.len(),
            I16(items) => items.len(),
            U32(items) => items.len(),
            I32(items) => items.len(),
            U64(items) => items.len(),
            I64(items) => items.len(),
            F32(items) => items.len(),
            F64(items) => items.len(),
            UTF8(items) => items.len(),
            UTF16(items) => items.len(),
            BOOL(items) => items.len(),
        }
    }
}

// ============================================================================
// From
// ============================================================================

impl From<&str> for MatlabType {
    fn from(value: &str) -> Self {
        UTF8(value.chars().collect::<Vec<char>>())
    }
}

impl<T: MatlabTypeMarker> From<Vec<T>> for MatlabType {
    fn from(value: Vec<T>) -> Self {
        T::to_matlab_type(value)
    }
}

impl<T: MatlabTypeMarker> From<T> for MatlabType {
    fn from(value: T) -> Self {
        T::to_matlab_type(vec![value])
    }
}

impl Default for MatlabType {
    fn default() -> Self {
        Self::F64(Vec::new())
    }
}

pub trait IntoMatlabType {
    fn to_matlab_type(vec: Vec<Self>) -> MatlabType
    where
        Self: Sized;
}

pub trait FromMatlabType {
    fn inner(value: MatlabType) -> Option<Vec<Self>>
    where
        Self: Sized;
    fn inner_ref(value: &MatlabType) -> Option<&Vec<Self>>
    where
        Self: Sized;
}

pub trait Zero {
    fn is_zero(&self) -> bool;
}

macro_rules! impl_MatlabTypeMarker {
    ($t1: ty, $var: ident) => {
        impl IntoMatlabType for $t1 {
            fn to_matlab_type(vec: Vec<Self>) -> MatlabType {
                $var(vec)
            }
        }

        impl FromMatlabType for $t1 {
            fn inner(value: MatlabType) -> Option<Vec<Self>> {
                match value {
                    $var(v) => Some(v),
                    _ => None,
                }
            }
            fn inner_ref(value: &MatlabType) -> Option<&Vec<Self>> {
                match value {
                    $var(v) => Some(v),
                    _ => None,
                }
            }
        }
    };
}

macro_rules! impl_MatlabTypeMarkerZero {
    ($($t1:ty),*) => {
        $(
        impl Zero for $t1 {
            fn is_zero(&self) -> bool {
                *self == 0
            }
        }
        )*
    };
}

impl_MatlabTypeMarkerZero!(u8, i8, u16, i16, u32, i32, u64, i64);

impl Zero for f32 {
    fn is_zero(&self) -> bool {
        *self == 0.0
    }
}

impl Zero for f64 {
    fn is_zero(&self) -> bool {
        *self == 0.0
    }
}

impl Zero for char {
    fn is_zero(&self) -> bool {
        *self == char::from(0)
    }
}

impl Zero for bool {
    fn is_zero(&self) -> bool {
        !(*self)
    }
}

use MatlabType::*;
impl_MatlabTypeMarker!(u8, U8);
impl_MatlabTypeMarker!(i8, I8);
impl_MatlabTypeMarker!(u16, U16);
impl_MatlabTypeMarker!(i16, I16);
impl_MatlabTypeMarker!(u32, U32);
impl_MatlabTypeMarker!(i32, I32);
impl_MatlabTypeMarker!(u64, U64);
impl_MatlabTypeMarker!(i64, I64);
impl_MatlabTypeMarker!(f32, F32);
impl_MatlabTypeMarker!(f64, F64);
impl_MatlabTypeMarker!(char, UTF8);
impl_MatlabTypeMarker!(bool, BOOL);

pub trait MatlabTypeMarker: Copy + Display + FromMatlabType + IntoMatlabType + Zero {}
impl MatlabTypeMarker for u8 {}
impl MatlabTypeMarker for i8 {}
impl MatlabTypeMarker for u16 {}
impl MatlabTypeMarker for i16 {}
impl MatlabTypeMarker for u32 {}
impl MatlabTypeMarker for i32 {}
impl MatlabTypeMarker for u64 {}
impl MatlabTypeMarker for i64 {}
impl MatlabTypeMarker for f32 {}
impl MatlabTypeMarker for f64 {}
impl MatlabTypeMarker for char {}
impl MatlabTypeMarker for bool {}

pub trait MatlabTypeMarkerSparse: MatlabTypeMarker {}
impl MatlabTypeMarkerSparse for f64 {}
impl MatlabTypeMarkerSparse for bool {}
