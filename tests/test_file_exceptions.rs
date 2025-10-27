//! Test suite concerning the readout of MAT-file data via the public interface.

use matrw::*;

#[test]
/// Test error handling on wrong path
fn fail_on_wrong_file_path() {
    let result = load_matfile("tests/non-existing-file.mat");

    assert!(matches!(result, Err(MatrwError::IoError(_))))
}

#[test]
/// Test error handling on a corrupt MAT-file
fn fail_on_corrupt_mat_file() {
    let result = load_matfile("tests/example_v7_corrupt.mat");

    assert!(matches!(result, Err(MatrwError::BinrwError(_))))
}

#[test]
/// Temporary test to check if error is thrown on load of MAT-file Version 7.3
fn fail_on_mat_version_73() {
    let result = load_matfile("tests/example_v73.mat");

    assert!(matches!(result, Err(MatrwError::MatFile73Error)))
}

#[test]
/// Temporary test to check if error is thrown on load of MAT-file Version 7.3
fn run_example() {
    let result = load_matfile("tests/example_v7.mat");

    assert!(result.is_ok())
}
