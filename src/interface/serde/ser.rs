//! Implementation of [`serde`] serialization system for our types.

use crate::MatlabType;
use crate::interface::error::MatrwError;
use crate::interface::matfile::MatFile;
use crate::interface::types::numeric_array::NumericArray;
use crate::interface::types::structure::Structure;
use crate::interface::variable::MatVariable;
use indexmap::IndexMap;
use serde::ser::Impossible;
use serde::ser::Serialize;
use serde::ser::SerializeSeq;
use serde::ser::SerializeStruct;
use serde::ser::Serializer;

/// Serialize [`MatFile`] from types which implement [`serde::Serialize`]
///
/// Current supported serializations:
/// - [x] [`MatVariable::NumericArray`] from scalar for all supported numeric types and `char`
/// - [x] [`MatVariable::NumericArray`] from `Vec<_>` for all supported numeric types and `char`
/// - [x] [`MatVariable::NumericArray`] from `String` for `char` data
/// - [x] [`MatVariable::Structure`] from `struct`
/// - [ ] [`MatVariable::StructureArray`]
/// - [ ] [`MatVariable::CellArray`]
/// ```
/// use matrw::{matfile, matvar, MatFile, to_matfile};
/// use serde::Serialize;
///
/// #[derive(Serialize, Debug)]
/// struct S2 {
///     v: Vec<f64>,
/// }
///
/// #[derive(Serialize, Debug)]
/// struct S {
///     f1: f64,
///     f2: u64,
///     s: S2,
/// }
///
/// #[derive(Serialize, Debug)]
/// struct Example {
///     a: u8,
///     b: i8,
///     c: u16,
///     d: i16,
///     e: u32,
///     f: i32,
///     g: u64,
///     h: i64,
///     i: f32,
///     j: f64,
///     k: char,
///     l: (),
///     m: String,
///     n: S,
///     o: Vec<i32>,
///     p: Vec<f32>,
/// }
///
/// let e = Example {
///     a: 8,
///     b: -8,
///     c: 16,
///     d: -16,
///     e: 32,
///     f: -32,
///     g: 64,
///     h: -64,
///     i: 32.0,
///     j: 64.0,
///     k: 'x',
///     l: (),
///     m: "test".to_string(),
///     n: S {
///         f1: 42.0,
///         f2: 43,
///         s: S2 { v: vec![1., 2., 3.] },
///     },
///     o: vec![0, 1, 2, 3, 4],
///     p: vec![0., 1., 2., 3., 4.],
/// };
///
/// let matfile = to_matfile(e);
/// ```
pub fn to_matfile<T>(t: T) -> Result<MatFile, MatrwError>
where
    T: Serialize,
{
    let serializer = MatFileSerializer::new();
    t.serialize(serializer)
}

pub struct MatFileSerializer {
    pub matfile: MatFile,
}

impl MatFileSerializer {
    pub fn new() -> Self {
        Self {
            matfile: MatFile::new(),
        }
    }
}

impl Default for MatFileSerializer {
    fn default() -> Self {
        Self::new()
    }
}

impl Serializer for MatFileSerializer {
    type Ok = MatFile;
    type Error = MatrwError;

    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Self;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(MatrwError::SerdeError(
            "serialize_bool is not supported by MatFileSerializer".to_string(),
        ))
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Err(MatrwError::SerdeError(
            "serialize_i8 is not supported by MatFileSerializer".to_string(),
        ))
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        Err(MatrwError::SerdeError(
            "serialize_i16 is not supported by MatFileSerializer".to_string(),
        ))
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        Err(MatrwError::SerdeError(
            "serialize_i32 is not supported by MatFileSerializer".to_string(),
        ))
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Err(MatrwError::SerdeError(
            "serialize_i64 is not supported by MatFileSerializer".to_string(),
        ))
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        Err(MatrwError::SerdeError(
            "serialize_u8 is not supported by MatFileSerializer".to_string(),
        ))
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        Err(MatrwError::SerdeError(
            "serialize_u16 is not supported by MatFileSerializer".to_string(),
        ))
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        Err(MatrwError::SerdeError(
            "serialize_u32 is not supported by MatFileSerializer".to_string(),
        ))
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Err(MatrwError::SerdeError(
            "serialize_u64 is not supported by MatFileSerializer".to_string(),
        ))
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(MatrwError::SerdeError(
            "serialize_f32 is not supported by MatFileSerializer".to_string(),
        ))
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(MatrwError::SerdeError(
            "serialize_f64 is not supported by MatFileSerializer".to_string(),
        ))
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Err(MatrwError::SerdeError(
            "serialize_char is not supported by MatFileSerializer".to_string(),
        ))
    }

    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
        Err(MatrwError::SerdeError(
            "serialize_str is not supported by MatFileSerializer".to_string(),
        ))
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(MatrwError::SerdeError(
            "serialize_bytes is not supported by MatFileSerializer".to_string(),
        ))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(MatrwError::SerdeError(
            "serialize_none is not supported by MatFileSerializer".to_string(),
        ))
    }

    fn serialize_some<T>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        Err(MatrwError::SerdeError(
            "serialize_some is not supported by MatFileSerializer".to_string(),
        ))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(MatrwError::SerdeError(
            "serialize_none is not supported by MatFileSerializer".to_string(),
        ))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        todo!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}

impl SerializeStruct for MatFileSerializer {
    type Ok = MatFile;
    type Error = MatrwError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        let serializer = MatVariableSerializer {};

        let matvar = value.serialize(serializer)?;
        self.matfile.insert(key, matvar);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.matfile)
    }
}

struct MatVariableSerializer {}

impl Serializer for MatVariableSerializer {
    type Ok = MatVariable;
    type Error = MatrwError;

    type SerializeSeq = MatVariableSeqSerializer;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = MatVariableStructSerializer;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(MatVariable::NumericArray(NumericArray::new(
            vec![1, 1],
            MatlabType::from(vec![v]),
            None,
        )?))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(MatVariable::NumericArray(NumericArray::new(
            vec![1, 1],
            MatlabType::from(vec![v]),
            None,
        )?))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(MatVariable::NumericArray(NumericArray::new(
            vec![1, 1],
            MatlabType::from(vec![v]),
            None,
        )?))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(MatVariable::NumericArray(NumericArray::new(
            vec![1, 1],
            MatlabType::from(vec![v]),
            None,
        )?))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(MatVariable::NumericArray(NumericArray::new(
            vec![1, 1],
            MatlabType::from(vec![v]),
            None,
        )?))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(MatVariable::NumericArray(NumericArray::new(
            vec![1, 1],
            MatlabType::from(vec![v]),
            None,
        )?))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(MatVariable::NumericArray(NumericArray::new(
            vec![1, 1],
            MatlabType::from(vec![v]),
            None,
        )?))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(MatVariable::NumericArray(NumericArray::new(
            vec![1, 1],
            MatlabType::from(vec![v]),
            None,
        )?))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(MatVariable::NumericArray(NumericArray::new(
            vec![1, 1],
            MatlabType::from(vec![v]),
            None,
        )?))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(MatVariable::NumericArray(NumericArray::new(
            vec![1, 1],
            MatlabType::from(vec![v]),
            None,
        )?))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(MatVariable::NumericArray(NumericArray::new(
            vec![1, 1],
            MatlabType::from(vec![v]),
            None,
        )?))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        let vec: Vec<char> = v.chars().collect();
        Ok(MatVariable::NumericArray(NumericArray::new(
            vec![1, vec.len()],
            MatlabType::from(vec),
            None,
        )?))
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(MatrwError::SerdeError(
            "serialize_bytes is not supported by MatVariableSerializer".to_string(),
        ))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(MatrwError::SerdeError(
            "serialize_none is not supported by MatVariableSerializer".to_string(),
        ))
    }

    fn serialize_some<T>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        Err(MatrwError::SerdeError(
            "serialize_some is not supported by MatVariableSerializer".to_string(),
        ))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(MatVariable::NumericArray(NumericArray::new(
            vec![0, 0],
            MatlabType::from(Vec::<f64>::new()),
            None,
        )?))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        todo!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(MatVariableSeqSerializer::new())
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(MatVariableStructSerializer { map: IndexMap::new() })
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}

struct MatVariableSeqSerializer {
    ty: MatVariableSeqVariants,
    vec_u8: Vec<u8>,
    vec_i8: Vec<i8>,
    vec_u16: Vec<u16>,
    vec_i16: Vec<i16>,
    vec_u32: Vec<u32>,
    vec_i32: Vec<i32>,
    vec_u64: Vec<u64>,
    vec_i64: Vec<i64>,
    vec_f32: Vec<f32>,
    vec_f64: Vec<f64>,
    vec_char: Vec<char>,
}

impl MatVariableSeqSerializer {
    fn new() -> Self {
        Self {
            ty: MatVariableSeqVariants::None,
            vec_u8: Vec::new(),
            vec_i8: Vec::new(),
            vec_u16: Vec::new(),
            vec_i16: Vec::new(),
            vec_u32: Vec::new(),
            vec_i32: Vec::new(),
            vec_u64: Vec::new(),
            vec_i64: Vec::new(),
            vec_f32: Vec::new(),
            vec_f64: Vec::new(),
            vec_char: Vec::new(),
        }
    }
}

impl SerializeSeq for MatVariableSeqSerializer {
    type Ok = MatVariable;
    type Error = MatrwError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        let serializer = ValueSerializer;

        let val = value.serialize(serializer).expect("Value serialization failed");

        match val {
            MatVariableSeqVariants::None => unimplemented!(),
            MatVariableSeqVariants::U8(v) => self.vec_u8.push(v),
            MatVariableSeqVariants::I8(v) => self.vec_i8.push(v),
            MatVariableSeqVariants::U16(v) => self.vec_u16.push(v),
            MatVariableSeqVariants::I16(v) => self.vec_i16.push(v),
            MatVariableSeqVariants::U32(v) => self.vec_u32.push(v),
            MatVariableSeqVariants::I32(v) => self.vec_i32.push(v),
            MatVariableSeqVariants::U64(v) => self.vec_u64.push(v),
            MatVariableSeqVariants::I64(v) => self.vec_i64.push(v),
            MatVariableSeqVariants::F32(v) => self.vec_f32.push(v),
            MatVariableSeqVariants::F64(v) => self.vec_f64.push(v),
            MatVariableSeqVariants::Char(v) => self.vec_char.push(v),
        }

        self.ty = val;

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self.ty {
            MatVariableSeqVariants::None => unimplemented!(),
            MatVariableSeqVariants::U8(_) => Ok(MatVariable::NumericArray(NumericArray::new(
                vec![1, self.vec_u8.len()],
                MatlabType::from(self.vec_u8),
                None,
            )?)),
            MatVariableSeqVariants::I8(_) => Ok(MatVariable::NumericArray(NumericArray::new(
                vec![1, self.vec_i8.len()],
                MatlabType::from(self.vec_i8),
                None,
            )?)),
            MatVariableSeqVariants::U16(_) => Ok(MatVariable::NumericArray(NumericArray::new(
                vec![1, self.vec_u16.len()],
                MatlabType::from(self.vec_u16),
                None,
            )?)),
            MatVariableSeqVariants::I16(_) => Ok(MatVariable::NumericArray(NumericArray::new(
                vec![1, self.vec_i16.len()],
                MatlabType::from(self.vec_i16),
                None,
            )?)),
            MatVariableSeqVariants::U32(_) => Ok(MatVariable::NumericArray(NumericArray::new(
                vec![1, self.vec_u32.len()],
                MatlabType::from(self.vec_u32),
                None,
            )?)),
            MatVariableSeqVariants::I32(_) => Ok(MatVariable::NumericArray(NumericArray::new(
                vec![1, self.vec_i32.len()],
                MatlabType::from(self.vec_i32),
                None,
            )?)),
            MatVariableSeqVariants::U64(_) => Ok(MatVariable::NumericArray(NumericArray::new(
                vec![1, self.vec_u64.len()],
                MatlabType::from(self.vec_u64),
                None,
            )?)),
            MatVariableSeqVariants::I64(_) => Ok(MatVariable::NumericArray(NumericArray::new(
                vec![1, self.vec_i64.len()],
                MatlabType::from(self.vec_i64),
                None,
            )?)),
            MatVariableSeqVariants::F32(_) => Ok(MatVariable::NumericArray(NumericArray::new(
                vec![1, self.vec_f32.len()],
                MatlabType::from(self.vec_f32),
                None,
            )?)),
            MatVariableSeqVariants::F64(_) => Ok(MatVariable::NumericArray(NumericArray::new(
                vec![1, self.vec_f64.len()],
                MatlabType::from(self.vec_f64),
                None,
            )?)),
            MatVariableSeqVariants::Char(_) => Ok(MatVariable::NumericArray(NumericArray::new(
                vec![1, self.vec_char.len()],
                MatlabType::from(self.vec_char),
                None,
            )?)),
        }
    }
}

enum MatVariableSeqVariants {
    None,
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    F32(f32),
    F64(f64),
    Char(char),
}

struct ValueSerializer;

impl Serializer for ValueSerializer {
    type Ok = MatVariableSeqVariants;
    type Error = MatrwError;

    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(MatVariableSeqVariants::I8(v))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(MatVariableSeqVariants::I16(v))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(MatVariableSeqVariants::I32(v))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(MatVariableSeqVariants::I64(v))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(MatVariableSeqVariants::U8(v))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(MatVariableSeqVariants::U16(v))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(MatVariableSeqVariants::U32(v))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(MatVariableSeqVariants::U64(v))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(MatVariableSeqVariants::F32(v))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(MatVariableSeqVariants::F64(v))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(MatVariableSeqVariants::Char(v))
    }

    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_some<T>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        todo!()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        todo!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}

struct MatVariableStructSerializer {
    map: IndexMap<String, MatVariable>,
}

impl SerializeStruct for MatVariableStructSerializer {
    type Ok = MatVariable;
    type Error = MatrwError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        let serializer = MatVariableSerializer {};

        let matvar = value.serialize(serializer)?;
        self.map.insert(key.to_string(), matvar);

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(MatVariable::Structure(Structure { value: self.map }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[test]
    fn serde_serialize_mixed() {
        #[derive(Serialize, Debug)]
        struct S2 {
            v: Vec<f64>,
        }

        #[derive(Serialize, Debug)]
        struct S {
            f1: f64,
            f2: u64,
            s: S2,
        }

        #[derive(Serialize, Debug)]
        struct Example {
            a: u8,
            b: i8,
            c: u16,
            d: i16,
            e: u32,
            f: i32,
            g: u64,
            h: i64,
            i: f32,
            j: f64,
            k: char,
            l: (),
            m: String,
            n: S,
            o: Vec<i32>,
            p: Vec<f32>,
        }

        let e = Example {
            a: 8,
            b: -8,
            c: 16,
            d: -16,
            e: 32,
            f: -32,
            g: 64,
            h: -64,
            i: 32.0,
            j: 64.0,
            k: 'x',
            l: (),
            m: "test".to_string(),
            n: S {
                f1: 42.0,
                f2: 43,
                s: S2 { v: vec![1., 2., 3.] },
            },
            o: vec![0, 1, 2, 3, 4],
            p: vec![0., 1., 2., 3., 4.],
        };

        let serializer = MatFileSerializer::new();
        let matfile = e.serialize(serializer);

        println!("{:#?}", matfile)
    }
}
