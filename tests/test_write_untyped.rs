//! Test suite concerning the writeout of MAT-file data via the public interface.

use matrw::*;

#[path = "shared_functions.rs"]
mod shared_functions;

use crate::shared_functions::TestFile;

#[test]
/// Create some data and write it to MAT-file
fn create_data_and_write() {
    let file = TestFile {
        path: "tests/my-matfile.mat",
    };

    let matfile = matfile!(
    a: matvar!([1., 2., 3.]),
    b: matvar!([4., 5., 6.]),
    );
    println!("{:#?}", &matfile);

    let _ = save_matfile_v7(file.path, matfile, false);
}

#[test]
/// Create some data and write it to MAT-file
fn create_data_and_write_compressed() {
    let file = TestFile {
        path: "tests/my-matfile.mat",
    };

    let matfile = matfile!(
    a: matvar!([1., 2., 3.]),
    b: matvar!([4., 5., 6.]),
    );
    println!("{:#?}", &matfile);

    let _ = save_matfile_v7(file.path, matfile, true);
}
