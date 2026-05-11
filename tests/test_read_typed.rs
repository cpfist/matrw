#![cfg(feature = "serde")]
//! Test suite concerning the readout of MAT-file data via the public interface.

use once_cell::sync::Lazy;
use serde::Deserialize;
use std::sync::Mutex;

use matrw::*;

static MATFILE: Lazy<Mutex<MatFile>> = Lazy::new(|| {
    // Use Octave to create a test MAT-file

    let path = "tests/example_v7.mat";
    Mutex::new(load_matfile(path).expect("Could not load file!"))
});

#[test]
/// Test read-only access of numeric data
fn single_var_f64() {
    let matfile = MATFILE.lock().unwrap();

    #[derive(Debug, Deserialize)]
    #[allow(dead_code)]
    struct Vars {
        a: f64,
    }

    let v: Vars = from_matfile(&matfile).unwrap();

    println!("The data {:#?}", v);
}

#[test]
/// Test read-only access of numeric data
fn single_var_string() {
    let matfile = MATFILE.lock().unwrap();

    #[derive(Debug, Deserialize)]
    #[allow(dead_code)]
    struct Vars {
        d: String,
    }

    let v: Vars = from_matfile(&matfile).unwrap();

    println!("The data {:#?}", v);
}

#[cfg(feature = "ndarray")]
mod ndarray_tests {
    use ndarray::{ArrayD, array};
    use serde::Serialize;
    use super::*;

    /// Deserialize a multi-dimensional MATLAB array into a ndarray::ArrayD
    #[test]
    fn deserialize_ndarray() {
        let arr = array![
                [
                    [
                        [0, 1, 2], 
                        [3, 4, 5]
                    ],
                    [
                        [6, 7, 8],
                        [9, 10, 11]
                    ]
                ],
                [
                    [
                        [0, 1, 2], 
                        [3, 4, 5]
                    ],
                    [
                        [6, 99, 8],
                        [9, 10, 11]
                    ]
                ],
        ];
        // println!("{arr}");
        
        #[derive(Debug, Deserialize, Serialize)]
        struct StructWithNDArray {
            arr: ArrayD<i32>,
        }

        let mat = matfile!(
            arr: matvar!([
                [
                    [
                        [0, 1, 2], 
                        [3, 4, 5]
                    ],
                    [
                        [6, 7, 8],
                        [9, 10, 11]
                    ]
                ],
                [
                    [
                        [0, 1, 2], 
                        [3, 4, 5]
                    ],
                    [
                        [6, 99, 8],
                        [9, 10, 11]
                    ]
                ],
            ]),
        );

        let s: StructWithNDArray = from_matfile(&mat).unwrap();
        // println!("{}", s.var);

        assert_eq!(arr.into_dyn(), s.arr)
    }

    /// Serialize a ndarray::ArrayD into a multi-dimensional MATLAB array
    #[test]
    fn serialize_ndarray() {
        let arr = array![
                [
                    [
                        [0, 1, 2], 
                        [3, 4, 5]
                    ],
                    [
                        [6, 7, 8],
                        [9, 10, 11]
                    ]
                ],
                [
                    [
                        [0, 1, 2], 
                        [3, 4, 5]
                    ],
                    [
                        [6, 99, 8],
                        [9, 10, 11]
                    ]
                ],
        ].into_dyn();
        // println!("{arr}");

        #[derive(Debug, Serialize)]
        struct StructWithNDArray {
            arr: ArrayD<i32>,
        }

        let s = StructWithNDArray { arr };

        let m = to_matfile(s).unwrap();
        println!("{:#?}", m);

        let _ = save_matfile_v7("test.mat", m, true);

        // assert_eq!(arr.into_dyn(), s.arr)
    }

}
