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
