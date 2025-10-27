//! Test suite concerning the writeout of MAT-file data via the public interface.

use once_cell::sync::Lazy;
use paste::paste;
use std::sync::Mutex;

use matrw::*;

#[path = "shared_functions.rs"]
mod shared_functions;

use crate::shared_functions::{MatlabCommand, TestFile};

macro_rules! skip_if_missing {
    ($tool:expr) => {
        if which::which($tool).is_err() {
            eprintln!("test skipped ({} not found)", $tool);
            return;
        }
    };
}

static MATFILE: Lazy<Mutex<MatFile>> = Lazy::new(|| {
    // Use Matlab to create a test MAT-file

    let filepath = TestFile {
        path: "tests/dynamic_matlab_testcases.mat",
    };

    let out = MatlabCommand::run("tests/dynamic_testcases.m");
    println!("error output: {}", out);

    Mutex::new(load_matfile(filepath.path).unwrap_or_default())
});

macro_rules! load_matlab_numeric_type {
    ($ty: ty, $var: expr, $ref: expr) => {
        paste! {
            #[test]
            fn [<load_matlab_ $var>]() {
                skip_if_missing!("matlab");

                let matfile = MATFILE.lock().unwrap();

                // Check variable names
                assert!(matfile.contains($var));

                // Check data content
                let data: Vec<$ty> = matfile[$var]
                    .[<to_vec_ $ty>]()
                    .expect("Could not access variable data");
                println!("Variable data:\n{:#?}", data);
                assert!(data == $ref);
            }
        }
    };
}

load_matlab_numeric_type!(u8, "numeric_var_u8", vec![1, 3, 2, 4]);
load_matlab_numeric_type!(i8, "numeric_var_i8", vec![-1, 3, 2, 4]);
load_matlab_numeric_type!(u16, "numeric_var_u16", vec![1, 3, 2, 4]);
load_matlab_numeric_type!(i16, "numeric_var_i16", vec![-1, 3, 2, 4]);
load_matlab_numeric_type!(u32, "numeric_var_u32", vec![1, 3, 2, 4]);
load_matlab_numeric_type!(i32, "numeric_var_i32", vec![-1, 3, 2, 4]);
load_matlab_numeric_type!(u64, "numeric_var_u64", vec![1, 3, 2, 4]);
load_matlab_numeric_type!(i64, "numeric_var_i64", vec![-1, 3, 2, 4]);
load_matlab_numeric_type!(f32, "numeric_var_f32", vec![-1., 3., 2., 4.]);
load_matlab_numeric_type!(f64, "numeric_var_f64", vec![-1., 3., 2., 4.]);
load_matlab_numeric_type!(char, "numeric_var_char", vec!['1', '3', '2', '4']);

#[test]
fn load_matlab_empty_struct() {
    skip_if_missing!("matlab");

    let matfile = MATFILE.lock().unwrap();

    // Check variable names
    assert!(matfile.contains("struct_var"));

    // Check data content
    let data = &matfile["struct_var"];
    println!("Variable data:\n{:#?}", data);
}

#[test]
fn load_matlab_struct_array_s() {
    skip_if_missing!("matlab");

    let matfile = MATFILE.lock().unwrap();

    let d = &matfile["S"];
    println!("the d: {:#?}", d);

    assert_eq!(d[[0, 0]]["id"].elem(0), matvar!(1.0));
    assert_eq!(d[[0, 0]]["name"], matvar!("Alice"));
    assert_eq!(d[[0, 0]]["age"].elem(0), matvar!(25.0));
    assert_eq!(d[[0, 0]]["score"].elem(0), matvar!(88.5));
    assert_eq!(d[[0, 0]]["passed"].elem(0), matvar!(true));

    assert_eq!(d[[0, 1]]["id"].elem(0), matvar!(2.0));
    assert_eq!(d[[0, 1]]["name"], matvar!("Bob"));
    assert_eq!(d[[0, 1]]["age"].elem(0), matvar!(34.0));
    assert_eq!(d[[0, 1]]["score"].elem(0), matvar!(72.3));
    assert_eq!(d[[0, 1]]["passed"].elem(0), matvar!(true));

    assert_eq!(d[[0, 2]]["id"].elem(0), matvar!(3.0));
    assert_eq!(d[[0, 2]]["name"], matvar!("Charlie"));
    assert_eq!(d[[0, 2]]["age"].elem(0), matvar!(29.0));
    assert_eq!(d[[0, 2]]["score"].elem(0), matvar!(91.7));
    assert_eq!(d[[0, 2]]["passed"].elem(0), matvar!(true));

    assert_eq!(d[[0, 3]]["id"].elem(0), matvar!(4.0));
    assert_eq!(d[[0, 3]]["name"], matvar!("Dana"));
    assert_eq!(d[[0, 3]]["age"].elem(0), matvar!(42.0));
    assert_eq!(d[[0, 3]]["score"].elem(0), matvar!(65.2));
    assert_eq!(d[[0, 3]]["passed"].elem(0), matvar!(false));
}

#[test]
fn load_matlab_function_handle() {
    skip_if_missing!("matlab");

    let matfile = MATFILE.lock().unwrap();

    // Check variable names
    assert!(matfile.contains("string_var"));

    // Check data content
    let data = &matfile["string_var"];
    println!("Variable data:\n{:#?}", data);

    assert!(matches!(data, MatVariable::Unsupported))
}
