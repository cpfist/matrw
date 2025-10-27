//! Test suite concerning the readout of MAT-file data via the public interface.

use once_cell::sync::Lazy;
use std::sync::Mutex;

use matrw::*;

static MATFILE: Lazy<Mutex<MatFile>> = Lazy::new(|| {
    // Use Octave to create a test MAT-file

    let path = "tests/example_v7.mat";
    Mutex::new(load_matfile(path).expect("Could not load file!"))
});

#[test]
/// Print all variables in MAT-file
fn print_all_variables() {
    let matfile = MATFILE.lock().unwrap();

    let var_names = matfile.iter().map(|(k, _)| k.clone()).collect::<Vec<String>>();

    println!("Variable names: {:#?}", var_names);
}

#[test]
/// Test read-only access of numeric data
fn access_numeric_data_as_vec_ref() {
    let matfile = MATFILE.lock().unwrap();

    let v = &matfile["a"];

    println!("The data {:#?}", v);
}

#[test]
/// Test read-only access of numeric data
fn access_numeric_data_as_vec_ref2() {
    let mut matfile = MATFILE.lock().unwrap();

    let v = matfile.take("d").unwrap();

    println!("The data {:#?}", v);
}

#[test]
/// Test read-only access of complex numeric data
fn access_numeric_data_as_vec_ref_complex() {
    let matfile = MATFILE.lock().unwrap();

    let c_real: Vec<f64> = matfile["c"].to_vec_f64().unwrap();
    let c_comp: Vec<f64> = matfile["c"].comp_to_vec_f64().unwrap();

    println!("The data \nreal: {:#?}\ncomplex: {:#?}", c_real, c_comp);
}

// #[test]
// /// Test clone access of numeric data
// fn access_numeric_data_as_mat_ref() {
//     let mut matfile = MATFILE.lock().unwrap();
//
//     let m: NumericArray = matfile
//         .take("A")
//         .unwrap()
//         .try_into()
//         .unwrap();
//
//     println!("{}", m)
// }

#[test]
/// Test clone access of struct data
fn access_struct_data() {
    let matfile = MATFILE.lock().unwrap();

    let n = matfile["s"]["field1"].elem([0, 0]).to_f64().unwrap();
    println!("{}", n)
}

// #[test]
// /// Test clone access of struct data
// fn access_numeric_data_take() {
//     let mut matfile = MATFILE.lock().unwrap();
//
//     let v: Vec<f64> = matfile["a"].take_vec_f64().unwrap();
//
//     println!("{:?}", v)
// }
//
// #[test]
// /// Test clone access of struct data
// fn access_numeric_data_into_trait() {
//     let matfile = MATFILE.lock().unwrap();
//
//     let v: Vec<u8> = (&matfile["a"]).into();
//
//     println!("{:?}", v)
// }

#[test]
fn access_struct_array() {
    let matfile = MATFILE.lock().unwrap();

    let v = &matfile["S"];
    println!("S: {:#?}", v);
}
