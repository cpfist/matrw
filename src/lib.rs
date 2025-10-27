//! matrw is a pure Rust library for serializing and deserializing MATLAB MAT-files.
//!
//! # Highlights
//!
//! - Read and write of basic MATLAB array types (numeric, structure, cell, sparse).
//! - Support of MAT-file compression.
//! - Convence methods and macros for access and creation of data (see [Untyped interface](#untyped-interface)).
//! - Serde interface (see [Typed interface](#typed-interface)).
//!
//! # Introduction
//!
//! MAT-files store structured numerical data in a binary format. This library offers a [serde_json](https://docs.rs/serde_json)-like interface for ergonomically reading and writing MAT-file data.
//!
//! Currently, matrw supports serialization and deserialization of version 7 MAT-files. The parser currently handles the following data types:
//!
//! - [x] numeric arrays
//! - [x] structure arrays
//! - [x] cell arrays
//! - [x] sparse arrays
//! - [ ] MCOS/Handle/Java objects (not yet supported)
//!
//! # Untyped Interface
//!
//! The enum `MatVariable` is the Rust type representing a MATLAB variable. It has the
//! following variants. see [`MatVariable`] for details about each type.
//!
//! ```rust
//! # use matrw::*;
//! enum MatVariable {
//!     NumericArray(NumericArray),
//!     SparseArray(SparseArray),
//!     StructureArray(StructureArray),
//!     CellArray(CellArray),
//!     Structure(Structure),
//!     // ...
//! }
//! ```
//!
//! ## Writing MAT-files
//!
//! The macro [`matvar`] can be used to conveniently construct a [`MatVariable`] from various Rust types. Similarly, the macro [`matfile`] can be used to construct a [`MatFile`].
//!
//! ```standalone_crate
//! use matrw::{MatFile, MatVariable, matfile, matvar, save_matfile_v7};
//!
//! // scalar number
//! let a: MatVariable = matvar!(1);
//!
//! // Create a MAT-file with variables "a", "b", "c", ...
//! let mat: MatFile = matfile!(
//!     // Insert as variable "a" in MAT-file
//!     a: a,
//!     // Create also some other variables
//!     // ...
//!     // scalar character
//!     b: matvar!('c'),
//!     // empty value
//!     c: matvar!([]),
//!     // vector
//!     d: matvar!([1, 2, 3]),
//!     // character array
//!     e: matvar!("asd"),
//!     // 2x2 matrix
//!     f: matvar!([
//!             [1, 2],
//!             [3, 4],
//!         ]),
//!     // 2x2x2 array
//!     g: matvar!([
//!             [
//!                 [1, 2],
//!                 [3, 4],
//!             ],
//!             [
//!                 [5, 6],
//!                 [7, 8],
//!             ],
//!         ]),
//!     // scalar struct
//!     h: matvar!({
//!             f1: 42.,
//!             f2: "abc",
//!         }),
//!     // struct array
//!     i: matvar!([
//!             {
//!                 f1: 42.,
//!                 f2: "test",
//!                 f3: 1.,
//!             },
//!             {
//!                 f1: 43.,
//!                 f2: "test1",
//!                 f3: 2.,
//!             },
//!         ]),
//!     // empty struct
//!     j: matvar!({}),
//!     // cell array
//!     k: matvar!([
//!             "abc",
//!             42.,
//!             1,
//!         ])
//! );
//!
//! // Write MAT-file without using compression
//! let _ = save_matfile_v7("test.mat", mat, false);
//!
//! # let _ = std::fs::remove_file("test.mat");
//! ```
//!
//! ## Reading MAT-files
//!
//! A MAT-file is loaded from a file with the function [`load_matfile`].
//! Variables are accessed by indexing their name.
//!
//! A `MatVariable` must be converted into a Rust type for usage. See the [`MatVariable`]
//! documentation for conversion options for different types.
//!
//! ```standalone_crate
//! # use matrw::{matfile, matvar, save_matfile_v7};
//! #
//! # // Create a MAT-file with variables "a", "b", "c", ...
//! # let mat = matfile!(
//! #     // scalar number
//! #     a: matvar!(1),
//! #     // scalar character
//! #     b: matvar!('c'),
//! #     // empty value
//! #     c: matvar!([]),
//! #     // vector
//! #     d: matvar!([1, 2, 3]),
//! #     // character array
//! #     e: matvar!("asd"),
//! #     // 2x2 matrix
//! #     f: matvar!([
//! #             [1, 2],
//! #             [3, 4],
//! #         ]),
//! #     // 2x2x2 array
//! #     g: matvar!([
//! #             [
//! #                 [1, 2],
//! #                 [3, 4],
//! #             ],
//! #             [
//! #                 [5, 6],
//! #                 [7, 8],
//! #             ],
//! #         ]),
//! #     // scalar struct
//! #     h: matvar!({
//! #             f1: 42.,
//! #             f2: "abc",
//! #         }),
//! #     // struct array
//! #     i: matvar!([
//! #             {
//! #                 f1: 42.,
//! #                 f2: "test",
//! #                 f3: 1.,
//! #             },
//! #             {
//! #                 f1: 43.,
//! #                 f2: "test1",
//! #                 f3: 2.,
//! #             },
//! #         ]),
//! #     // empty struct
//! #     j: matvar!({}),
//! #     // cell array
//! #     k: matvar!([
//! #             "abc",
//! #             42.,
//! #             1,
//! #         ])
//! # );
//! #
//! # // Write MAT-file without using compression
//! # let _ = save_matfile_v7("test.mat", mat, false);
//! #
//! use matrw::{load_matfile, OwnedIndex};
//!
//! // Load MAT-file
//! let mat = load_matfile("test.mat").expect("Could not read file.");
//!
//! // Inspect data
//! // Access variable "a" and convert it to i32
//! assert_eq!(mat["a"].to_i32(), Some(1));
//! // Access variable "b" and convert element "2" to i32
//! assert_eq!(mat["d"].elem(2).to_i32(), Some(3));
//! // Access variable "f" and convert element "(1,1)" to i32
//! assert_eq!(mat["f"].elem([1,1]).to_i32(), Some(4));
//! // Access variable "g" and convert element "[1,0,1]" to i32
//! assert_eq!(mat["g"].elem([1,0,1]).to_i32(), Some(7));
//! // Access variable "h", then field "f1" and convert to f64
//! assert_eq!(mat["h"]["f1"].to_f64(), Some(42.));
//! // Access variable "i", then struct index "1" and convert to f64
//! assert_eq!(mat["i"][1]["f3"].to_f64(), Some(2.));
//! // Access variable "k", then cell index "1" and convert to f64
//! assert_eq!(mat["k"][1].to_f64(), Some(42.));
//!
//! # let _ = std::fs::remove_file("test.mat");
//! ```
//!
//! # Typed Interface
//!
//! The typed interface is used, when the structural content of a MAT-file is *known* at compile time. It is provided using the [`serde`] framework.
//!
//! ## Writing MAT-files
//!
//! Types implementing [`serde::Serialize`] can be serialized into [`MatFile`], using the function [`to_matfile`].
//!
//! ```standalone_crate
//! use matrw::{save_matfile_v7, to_matfile};
//! use serde::{Serialize};
//!
//! #[derive(Serialize)]
//! struct MyMat {
//!     a: i32,
//!     b: char,
//!     d: Vec<i32>,
//!     e: String,
//!     h: S,
//! }
//!
//! #[derive(Serialize)]
//! struct S {
//!     f1: f64,
//!     f2: String,
//! }
//!
//! let data = MyMat {
//!     a: 1,
//!     b: 'c',
//!     d: vec![1, 2, 3],
//!     e: "asd".to_string(),
//!     h: S {
//!         f1: 42.,
//!         f2: "abc".to_string()
//!     }
//! };
//!
//! let mat = to_matfile(data).expect("Cannot serialize data");
//! let _ = save_matfile_v7("test.mat", mat, false);
//!
//! # let _ = std::fs::remove_file("test.mat");
//! ```
//!
//! ## Reading MAT-files
//!
//! [`MatFile`] can be deserialized into a custom type implementing [`serde::Deserialize`], using the function [`from_matfile`].
//!
//! ```standalone_crate
//! use matrw::{save_matfile_v7, to_matfile, load_matfile, from_matfile};
//! use serde::{Deserialize};
//! # use serde::{Serialize};
//!
//! #[derive(Deserialize)]
//! # #[derive(Serialize)]
//! struct S {
//!     f1: f64,
//!     f2: String,
//! }
//!
//! #[derive(Deserialize)]
//! # #[derive(Serialize)]
//! struct MyMat {
//!     a: i32,
//!     b: char,
//!     d: Vec<i32>,
//!     e: String,
//!     h: S,
//! }
//!
//! # let data = MyMat {
//! #     a: 1,
//! #     b: 'c',
//! #     d: vec![1, 2, 3],
//! #     e: "asd".to_string(),
//! #     h: S {
//! #         f1: 42.,
//! #         f2: "abc".to_string()
//! #     }
//! # };
//! #
//! # let mat = to_matfile(data).expect("Cannot serialize data");
//! # let _ = save_matfile_v7("test.mat", mat, false);
//! #
//! // Load MAT-file
//! let matfile = load_matfile("test.mat").expect("Cannot not read file.");
//! let mat: MyMat = from_matfile(&matfile).expect("Cannot deserialize data");
//!
//! // Inspect data
//! assert_eq!(mat.a, 1);
//! assert_eq!(mat.d[2], 3);
//! assert_eq!(mat.e, "asd".to_string());
//! assert_eq!(mat.h.f1, 42.);
//!
//! # let _ = std::fs::remove_file("test.mat");
//! ```
//!

#[doc(hidden)]
pub mod interface;
#[doc(hidden)]
pub mod parser;

#[doc(hidden)]
pub mod __private {
    #[doc(hidden)]
    pub use indexmap::IndexMap;
}

#[doc(inline)]
pub use interface::{
    error::MatrwError,
    fileio::{load_matfile, load_matfile_from_u8, save_matfile_v7},
    matfile::MatFile,
    types::matlab_types::MatlabType,
    variable::MatVariable,
};

#[doc(hidden)]
pub use interface::variable::OwnedIndex;

#[doc(hidden)]
pub use interface::types::{
    cell_array::CellArray, numeric_array::NumericArray, sparse_array::SparseArray, structure::Structure,
    structure_array::StructureArray,
};

#[doc(hidden)]
pub use interface::types::{
    numeric_array::{check_same_dim, check_same_type},
    structure::check_same_fields,
};

#[doc(inline)]
pub use interface::serde::{de::from_matfile, ser::to_matfile};
