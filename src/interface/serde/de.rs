//! Implementation of [`serde`] deserialization system for our types.

use crate::MatFile;
use crate::MatrwError;

use serde::Deserialize;

use crate::interface::variable::MatVariable;
use serde::{
    de::{Deserializer, IntoDeserializer, MapAccess, value::SeqDeserializer},
    forward_to_deserialize_any,
};

/// Deserialize [`MatFile`] into types which implement [`serde::Deserialize`].
///
/// Current supported deserializations:
/// - [x] [`MatVariable::NumericArray`] to scalar for all supported numeric types and `char`
/// - [x] [`MatVariable::NumericArray`] to `Vec<_>` for all supported numeric types and `char`
/// - [x] [`MatVariable::NumericArray`] to `String` for `char` data
/// - [x] [`MatVariable::Structure`] to `struct`
/// - [ ] [`MatVariable::StructureArray`]
/// - [ ] [`MatVariable::CellArray`]
/// ```
/// use matrw::{matfile, matvar, MatFile, from_matfile};
/// use serde::Deserialize;
///
/// // Create example data
/// let matfile = matfile!(
///     a: matvar!(vec![1.0, 2.0, 3.0]),
///     b: matvar!(42.),
///     c: matvar!(vec!['t', 'e', 's', 't']),
///     s: matvar!({
///         f1: 1.,
///         f2: 2.,
///     }),
/// );
///
/// // Define the sub-struct
/// #[derive(Deserialize)]
/// struct SubStruct {
///     f1: f64,
///     f2: f64,
/// }
///
/// // Create the structure of our MAT-file
/// #[derive(Deserialize)]
/// struct MyMatfile {
///     a: Vec<f64>,  // "a" into Vec<f64>
///     b: f64,       // "b" into f64
///     c: String,    // "c" into String
///     s: SubStruct, // "s" into SubStruct
/// }
///
/// // Deserialize the MAT-file into our structure
/// let m: MyMatfile = from_matfile(&matfile)
///         .expect("Failed to deserialize MAT-file");
///
/// // Variables accessible via struct members
/// let a: Vec<f64> = m.a;
/// let b: f64 = m.b;
/// let c: String = m.c;
/// let f1 = m.s.f1;
/// let f2 = m.s.f2;
/// ```
pub fn from_matfile<'a, T>(matfile: &'a MatFile) -> Result<T, MatrwError>
where
    T: Deserialize<'a>,
{
    let deserializer = MatFileDeserializer::new(matfile);
    T::deserialize(deserializer)
}

pub struct MatFileDeserializer<'de> {
    matfile: &'de MatFile,
}

impl<'de> MatFileDeserializer<'de> {
    pub fn new(matfile: &'de MatFile) -> Self {
        Self { matfile }
    }
}

impl<'de> Deserializer<'de> for MatFileDeserializer<'de> {
    type Error = MatrwError;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_map(MatFileMapAccess::new(&self, fields, 0))
    }

    forward_to_deserialize_any! {bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes byte_buf option unit unit_struct newtype_struct seq tuple tuple_struct map enum identifier ignored_any}
}

struct MatFileMapAccess<'a, 'de: 'a> {
    de: &'a MatFileDeserializer<'de>,
    fields: &'static [&'static str],
    id: usize,
}

impl<'a, 'de: 'a> MatFileMapAccess<'a, 'de> {
    fn new(de: &'a MatFileDeserializer<'de>, fields: &'static [&'static str], id: usize) -> Self {
        MatFileMapAccess { de, fields, id }
    }
}

#[allow(dead_code)]
impl<'a, 'de> MapAccess<'de> for MatFileMapAccess<'a, 'de> {
    type Error = MatrwError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if self.id < self.fields.len() {
            let key = self.fields[self.id];
            self.id += 1;
            seed.deserialize(key.into_deserializer()).map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        let key = self.fields[self.id - 1];
        let matvar = &self.de.matfile[key];

        match matvar {
            MatVariable::NumericArray(_) => seed.deserialize(MatVariableDeserializer { matvar }),
            MatVariable::Structure(_) => seed.deserialize(MatVariableDeserializer { matvar }),
            _ => unimplemented!(),
        }
    }
}

#[allow(dead_code)]
struct MatVariableDeserializer<'de> {
    matvar: &'de MatVariable,
}

impl<'de> Deserializer<'de> for MatVariableDeserializer<'de> {
    type Error = MatrwError;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.matvar {
            MatVariable::NumericArray(_) => {
                let vec_u8: Option<Vec<u8>> = self.matvar.to_vec_u8();
                let vec_i8: Option<Vec<i8>> = self.matvar.to_vec_i8();
                let vec_u16: Option<Vec<u16>> = self.matvar.to_vec_u16();
                let vec_i16: Option<Vec<i16>> = self.matvar.to_vec_i16();
                let vec_u32: Option<Vec<u32>> = self.matvar.to_vec_u32();
                let vec_i32: Option<Vec<i32>> = self.matvar.to_vec_i32();
                let vec_u64: Option<Vec<u64>> = self.matvar.to_vec_u64();
                let vec_i64: Option<Vec<i64>> = self.matvar.to_vec_i64();
                let vec_f32: Option<Vec<f32>> = self.matvar.to_vec_f32();
                let vec_f64: Option<Vec<f64>> = self.matvar.to_vec_f64();
                let vec_char: Option<Vec<char>> = self.matvar.to_vec_char();

                if let Some(value) = vec_u8 {
                    visitor.visit_seq(SeqDeserializer::new(value.into_iter()).into_deserializer())
                } else if let Some(value) = vec_i8 {
                    visitor.visit_seq(SeqDeserializer::new(value.into_iter()).into_deserializer())
                } else if let Some(value) = vec_u16 {
                    visitor.visit_seq(SeqDeserializer::new(value.into_iter()).into_deserializer())
                } else if let Some(value) = vec_i16 {
                    visitor.visit_seq(SeqDeserializer::new(value.into_iter()).into_deserializer())
                } else if let Some(value) = vec_u32 {
                    visitor.visit_seq(SeqDeserializer::new(value.into_iter()).into_deserializer())
                } else if let Some(value) = vec_i32 {
                    visitor.visit_seq(SeqDeserializer::new(value.into_iter()).into_deserializer())
                } else if let Some(value) = vec_u64 {
                    visitor.visit_seq(SeqDeserializer::new(value.into_iter()).into_deserializer())
                } else if let Some(value) = vec_i64 {
                    visitor.visit_seq(SeqDeserializer::new(value.into_iter()).into_deserializer())
                } else if let Some(value) = vec_f32 {
                    visitor.visit_seq(SeqDeserializer::new(value.into_iter()).into_deserializer())
                } else if let Some(value) = vec_f64 {
                    visitor.visit_seq(SeqDeserializer::new(value.into_iter()).into_deserializer())
                } else if let Some(value) = vec_char {
                    visitor.visit_seq(SeqDeserializer::new(value.into_iter()).into_deserializer())
                } else {
                    Err(MatrwError::SerdeError("Unknown numeric type".to_string()))
                }
            }
            _ => unimplemented!(),
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.matvar {
            MatVariable::Structure(_) => visitor.visit_map(MatVariableMapAccess::new(&self, fields, 0)),
            _ => unimplemented!(),
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.matvar {
            MatVariable::NumericArray(_) => {
                let vec: Option<u8> = self.matvar.to_u8();

                if let Some(value) = vec {
                    visitor.visit_u8(value)
                } else {
                    Err(MatrwError::SerdeError("Expected u8".to_string()))
                }
            }
            _ => unimplemented!(),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.matvar {
            MatVariable::NumericArray(_) => {
                let vec: Option<i8> = self.matvar.to_i8();

                if let Some(value) = vec {
                    visitor.visit_i8(value)
                } else {
                    Err(MatrwError::SerdeError("Expected i8".to_string()))
                }
            }
            _ => unimplemented!(),
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.matvar {
            MatVariable::NumericArray(_) => {
                let vec: Option<u16> = self.matvar.to_u16();

                if let Some(value) = vec {
                    visitor.visit_u16(value)
                } else {
                    Err(MatrwError::SerdeError("Expected u16".to_string()))
                }
            }
            _ => unimplemented!(),
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.matvar {
            MatVariable::NumericArray(_) => {
                let vec: Option<i16> = self.matvar.to_i16();

                if let Some(value) = vec {
                    visitor.visit_i16(value)
                } else {
                    Err(MatrwError::SerdeError("Expected i16".to_string()))
                }
            }
            _ => unimplemented!(),
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.matvar {
            MatVariable::NumericArray(_) => {
                let vec: Option<u32> = self.matvar.to_u32();

                if let Some(value) = vec {
                    visitor.visit_u32(value)
                } else {
                    Err(MatrwError::SerdeError("Expected u32".to_string()))
                }
            }
            _ => unimplemented!(),
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.matvar {
            MatVariable::NumericArray(_) => {
                let vec: Option<i32> = self.matvar.to_i32();

                if let Some(value) = vec {
                    visitor.visit_i32(value)
                } else {
                    Err(MatrwError::SerdeError("Expected i32".to_string()))
                }
            }
            _ => unimplemented!(),
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.matvar {
            MatVariable::NumericArray(_) => {
                let vec: Option<u64> = self.matvar.to_u64();

                if let Some(value) = vec {
                    visitor.visit_u64(value)
                } else {
                    Err(MatrwError::SerdeError("Expected u64".to_string()))
                }
            }
            _ => unimplemented!(),
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.matvar {
            MatVariable::NumericArray(_) => {
                let vec: Option<i64> = self.matvar.to_i64();

                if let Some(value) = vec {
                    visitor.visit_i64(value)
                } else {
                    Err(MatrwError::SerdeError("Expected i64".to_string()))
                }
            }
            _ => unimplemented!(),
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.matvar {
            MatVariable::NumericArray(_) => {
                let vec: Option<f32> = self.matvar.to_f32();

                if let Some(value) = vec {
                    visitor.visit_f32(value)
                } else {
                    Err(MatrwError::SerdeError("Expected f32".to_string()))
                }
            }
            _ => unimplemented!(),
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.matvar {
            MatVariable::NumericArray(_) => {
                let vec: Option<f64> = self.matvar.to_f64();

                if let Some(value) = vec {
                    visitor.visit_f64(value)
                } else {
                    Err(MatrwError::SerdeError("Expected f64".to_string()))
                }
            }
            _ => unimplemented!(),
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.matvar {
            MatVariable::NumericArray(_) => {
                let vec: Option<char> = self.matvar.to_char();

                if let Some(value) = vec {
                    visitor.visit_char(value)
                } else {
                    Err(MatrwError::SerdeError("Expected char".to_string()))
                }
            }
            _ => unimplemented!(),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.matvar {
            MatVariable::NumericArray(_) => {
                let vec: Option<bool> = self.matvar.to_bool();

                if let Some(value) = vec {
                    visitor.visit_bool(value)
                } else {
                    Err(MatrwError::SerdeError("Expected bool".to_string()))
                }
            }
            _ => unimplemented!(),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.matvar {
            MatVariable::NumericArray(_) => {
                let vec: Option<Vec<char>> = self.matvar.to_vec_char();

                if let Some(value) = vec {
                    let s: String = value.into_iter().collect();
                    visitor.visit_string(s)
                } else {
                    Err(MatrwError::SerdeError("Expected char".to_string()))
                }
            }
            _ => unimplemented!(),
        }
    }

    forward_to_deserialize_any! {str bytes byte_buf option unit unit_struct newtype_struct tuple tuple_struct map enum identifier ignored_any}
}

struct MatVariableMapAccess<'a, 'de: 'a> {
    de: &'a MatVariableDeserializer<'de>,
    fields: &'static [&'static str],
    id: usize,
}

impl<'a, 'de: 'a> MatVariableMapAccess<'a, 'de> {
    fn new(de: &'a MatVariableDeserializer<'de>, fields: &'static [&'static str], id: usize) -> Self {
        MatVariableMapAccess { de, fields, id }
    }
}

#[allow(dead_code)]
impl<'a, 'de> MapAccess<'de> for MatVariableMapAccess<'a, 'de> {
    type Error = MatrwError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if self.id < self.fields.len() {
            let key = self.fields[self.id];
            self.id += 1;
            seed.deserialize(key.into_deserializer()).map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        let key = self.fields[self.id - 1];
        let matvar = &self.de.matvar[key];

        match matvar {
            MatVariable::NumericArray(_) => seed.deserialize(MatVariableDeserializer { matvar }),
            MatVariable::Structure(_) => seed.deserialize(MatVariableDeserializer { matvar }),
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_matfile_from_u8;
    use serde::Deserialize;

    /// Binary representation of a MAT-file containing two variables 'a' and 'b'.
    /// To reproduce, in a MATLAB session with a clean workspace run
    /// `>> a = uint8(8);`
    /// `>> b = int8(-8);`
    /// `>> c = uint16(16);`
    /// `>> d = int16(-16);`
    /// `>> e = uint32(32);`
    /// `>> f = int32(-32);`
    /// `>> g = uint64(64);`
    /// `>> h = int64(-64);`
    /// `>> i = single(32);`
    /// `>> j = double(64);`
    /// `>> k = 'x';`
    /// `>> save('example.mat');`
    ///
    const MATFILE7_NUMERIC_VARS: [u8; 631] = [
        0x4d, 0x41, 0x54, 0x4c, 0x41, 0x42, 0x20, 0x35, 0x2e, 0x30, 0x20, 0x4d, 0x41, 0x54, 0x2d, 0x66, 0x69,
        0x6c, 0x65, 0x2c, 0x20, 0x50, 0x6c, 0x61, 0x74, 0x66, 0x6f, 0x72, 0x6d, 0x3a, 0x20, 0x47, 0x4c, 0x4e,
        0x58, 0x41, 0x36, 0x34, 0x2c, 0x20, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x20, 0x6f, 0x6e, 0x3a,
        0x20, 0x53, 0x75, 0x6e, 0x20, 0x4d, 0x61, 0x72, 0x20, 0x32, 0x33, 0x20, 0x31, 0x30, 0x3a, 0x30, 0x38,
        0x3a, 0x34, 0x32, 0x20, 0x32, 0x30, 0x32, 0x35, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
        0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
        0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x49, 0x4d, 0x0f, 0x00, 0x00, 0x00, 0x25, 0x00, 0x00, 0x00,
        0x78, 0x9c, 0xe3, 0x63, 0x60, 0x60, 0x30, 0x00, 0x62, 0x36, 0x20, 0xe6, 0x00, 0x62, 0x4e, 0x06, 0x08,
        0x60, 0x85, 0xf2, 0x19, 0xe1, 0x98, 0x91, 0x21, 0x11, 0x48, 0x33, 0x01, 0x69, 0x90, 0x38, 0x00, 0x17,
        0x7c, 0x00, 0xd3, 0x0f, 0x00, 0x00, 0x00, 0x21, 0x00, 0x00, 0x00, 0x78, 0x9c, 0xe3, 0x63, 0x60, 0x60,
        0x30, 0x00, 0x62, 0x36, 0x20, 0xe6, 0x80, 0x62, 0x10, 0x60, 0x85, 0xb2, 0x19, 0xe1, 0x98, 0x91, 0x21,
        0x09, 0x4a, 0xff, 0x00, 0xd2, 0x00, 0x1b, 0x18, 0x01, 0xc2, 0x0f, 0x00, 0x00, 0x00, 0x27, 0x00, 0x00,
        0x00, 0x78, 0x9c, 0xe3, 0x63, 0x60, 0x60, 0x30, 0x00, 0x62, 0x36, 0x20, 0xe6, 0x00, 0x62, 0x6e, 0x06,
        0x08, 0x60, 0x85, 0xf2, 0x19, 0xe1, 0x98, 0x91, 0x21, 0x19, 0x48, 0xb3, 0x30, 0x30, 0x31, 0x08, 0x00,
        0x69, 0x00, 0x18, 0x1a, 0x00, 0xe2, 0x0f, 0x00, 0x00, 0x00, 0x29, 0x00, 0x00, 0x00, 0x78, 0x9c, 0xe3,
        0x63, 0x60, 0x60, 0x30, 0x00, 0x62, 0x36, 0x20, 0xe6, 0x00, 0x62, 0x2e, 0x06, 0x08, 0x60, 0x85, 0xf2,
        0x19, 0xe1, 0x98, 0x91, 0x21, 0x05, 0x48, 0x33, 0x33, 0x30, 0x31, 0x7c, 0xf8, 0xcf, 0xc0, 0x00, 0x00,
        0x1e, 0x73, 0x02, 0xc0, 0x0f, 0x00, 0x00, 0x00, 0x26, 0x00, 0x00, 0x00, 0x78, 0x9c, 0xe3, 0x63, 0x60,
        0x60, 0x30, 0x00, 0x62, 0x36, 0x20, 0xe6, 0x00, 0x62, 0x5e, 0x06, 0x08, 0x60, 0x85, 0xf2, 0x19, 0xe1,
        0x98, 0x91, 0x21, 0x15, 0xac, 0x8e, 0x85, 0x41, 0x01, 0x48, 0x03, 0x00, 0x18, 0xde, 0x00, 0xfa, 0x0f,
        0x00, 0x00, 0x00, 0x27, 0x00, 0x00, 0x00, 0x78, 0x9c, 0xe3, 0x63, 0x60, 0x60, 0x30, 0x00, 0x62, 0x36,
        0x20, 0xe6, 0x00, 0x62, 0x1e, 0x06, 0x08, 0x60, 0x85, 0xf2, 0x19, 0xe1, 0x98, 0x91, 0x21, 0x0d, 0x2c,
        0xce, 0xc2, 0xf0, 0xe0, 0xff, 0xff, 0xff, 0x00, 0x21, 0xb4, 0x04, 0xb6, 0x0f, 0x00, 0x00, 0x00, 0x26,
        0x00, 0x00, 0x00, 0x78, 0x9c, 0xe3, 0x63, 0x60, 0x60, 0xb0, 0x00, 0x62, 0x36, 0x20, 0xe6, 0x00, 0x62,
        0x7e, 0x06, 0x08, 0x60, 0x85, 0xf2, 0x19, 0xe1, 0x98, 0x91, 0x21, 0x1d, 0x48, 0xf3, 0x42, 0xc5, 0x1d,
        0xa0, 0xea, 0x00, 0x24, 0x2e, 0x01, 0x31, 0x0f, 0x00, 0x00, 0x00, 0x27, 0x00, 0x00, 0x00, 0x78, 0x9c,
        0xe3, 0x63, 0x60, 0x60, 0xb0, 0x00, 0x62, 0x36, 0x20, 0xe6, 0x00, 0x62, 0x3e, 0x06, 0x08, 0x60, 0x85,
        0xf2, 0x19, 0xe1, 0x98, 0x91, 0x21, 0x03, 0x48, 0xf3, 0x40, 0xc5, 0x0f, 0xfc, 0x87, 0x00, 0x00, 0x43,
        0xe6, 0x08, 0xa9, 0x0f, 0x00, 0x00, 0x00, 0x25, 0x00, 0x00, 0x00, 0x78, 0x9c, 0xe3, 0x63, 0x60, 0x60,
        0x30, 0x00, 0x62, 0x36, 0x20, 0xe6, 0x00, 0x62, 0x76, 0x06, 0x08, 0x60, 0x85, 0xf2, 0x19, 0xe1, 0x98,
        0x91, 0x21, 0x13, 0x2c, 0xcf, 0x02, 0x92, 0x76, 0x02, 0x00, 0x17, 0xe8, 0x01, 0x1b, 0x0f, 0x00, 0x00,
        0x00, 0x24, 0x00, 0x00, 0x00, 0x78, 0x9c, 0xe3, 0x63, 0x60, 0x60, 0x30, 0x00, 0x62, 0x36, 0x20, 0xe6,
        0x80, 0xd2, 0x20, 0xc0, 0x0a, 0xe5, 0x33, 0xc2, 0x31, 0x23, 0x43, 0x16, 0x90, 0x66, 0x02, 0xd2, 0x0e,
        0x40, 0x1a, 0x00, 0x18, 0x50, 0x01, 0x11, 0x0f, 0x00, 0x00, 0x00, 0x26, 0x00, 0x00, 0x00, 0x78, 0x9c,
        0xe3, 0x63, 0x60, 0x60, 0x30, 0x00, 0x62, 0x36, 0x20, 0xe6, 0x00, 0x62, 0x16, 0x06, 0x08, 0x60, 0x85,
        0xf2, 0x19, 0xe1, 0x98, 0x91, 0x21, 0x1b, 0x48, 0x0b, 0x00, 0xe9, 0x0a, 0x20, 0x0d, 0x00, 0x19, 0x5c,
        0x01, 0x56,
    ];

    #[test]
    fn test_numeric_vars_vec() {
        #[allow(dead_code)]
        #[derive(Deserialize, Debug)]
        struct Example {
            a: Vec<u8>,
            b: Vec<i8>,
            c: Vec<u16>,
            d: Vec<i16>,
            e: Vec<u32>,
            f: Vec<i32>,
            g: Vec<u64>,
            h: Vec<i64>,
            i: Vec<f32>,
            j: Vec<f64>,
            k: Vec<char>,
        }

        let matfile = load_matfile_from_u8(&MATFILE7_NUMERIC_VARS).unwrap();
        let deserializer = MatFileDeserializer { matfile: &matfile };

        let s = Example::deserialize(deserializer);

        println!("{:#?}", s);
    }

    #[test]
    fn test_numeric_vars_scalar() {
        #[allow(dead_code)]
        #[derive(Deserialize, Debug)]
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
        }

        let matfile = load_matfile_from_u8(&MATFILE7_NUMERIC_VARS).unwrap();
        let deserializer = MatFileDeserializer { matfile: &matfile };
        println!("{:#?}", matfile);

        let s = Example::deserialize(deserializer);

        println!("{:#?}", s);
    }

    /// Binary representation of a MAT-file containing one variable 's'.
    /// To reproduce, in a MATLAB session with a clean workspace run
    /// `>> s.a = 42;`
    /// `>> s.b = pi;`
    /// `>> save('example.mat', 's');`
    ///
    const MATFILE7_STRUCT: [u8; 213] = [
        0x4d, 0x41, 0x54, 0x4c, 0x41, 0x42, 0x20, 0x35, 0x2e, 0x30, 0x20, 0x4d, 0x41, 0x54, 0x2d, 0x66, 0x69,
        0x6c, 0x65, 0x2c, 0x20, 0x50, 0x6c, 0x61, 0x74, 0x66, 0x6f, 0x72, 0x6d, 0x3a, 0x20, 0x47, 0x4c, 0x4e,
        0x58, 0x41, 0x36, 0x34, 0x2c, 0x20, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x20, 0x6f, 0x6e, 0x3a,
        0x20, 0x4d, 0x6f, 0x6e, 0x20, 0x4d, 0x61, 0x72, 0x20, 0x31, 0x30, 0x20, 0x30, 0x30, 0x3a, 0x32, 0x34,
        0x3a, 0x33, 0x36, 0x20, 0x32, 0x30, 0x32, 0x35, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
        0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
        0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x49, 0x4d, 0x0f, 0x00, 0x00, 0x00, 0x4d, 0x00, 0x00, 0x00,
        0x78, 0x9c, 0xe3, 0x63, 0x60, 0x60, 0xd8, 0x01, 0xc4, 0x6c, 0x40, 0xcc, 0x01, 0xc4, 0x4c, 0x0c, 0x10,
        0xc0, 0x0a, 0xe5, 0x33, 0xc2, 0x31, 0x23, 0x43, 0x31, 0x58, 0x9c, 0x85, 0x81, 0x19, 0x2a, 0x06, 0xd2,
        0x93, 0xc8, 0xc0, 0x90, 0x04, 0x52, 0xcf, 0x07, 0xc4, 0x06, 0x48, 0xe6, 0xb0, 0xe1, 0x34, 0x07, 0x02,
        0x98, 0x80, 0x2c, 0x2d, 0xa8, 0x3e, 0x0b, 0x12, 0xf4, 0x71, 0x42, 0xc5, 0x25, 0x74, 0x5d, 0x42, 0x7e,
        0x2b, 0x72, 0x3a, 0x00, 0x00, 0xad, 0xe1, 0x05, 0x7d,
    ];

    #[test]
    fn test_struct() {
        #[allow(dead_code)]
        #[derive(Deserialize, Debug)]
        struct S {
            a: Vec<f64>,
            b: Vec<f64>,
        }

        #[allow(dead_code)]
        #[derive(Deserialize, Debug)]
        struct Example {
            s: S,
        }

        let matfile = load_matfile_from_u8(&MATFILE7_STRUCT).unwrap();
        let deserializer = MatFileDeserializer { matfile: &matfile };

        let s = Example::deserialize(deserializer);

        println!("{:#?}", s);
    }
}
