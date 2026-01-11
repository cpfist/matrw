use std::u8;

use matrw::*;

#[path = "shared_functions.rs"]
mod shared_functions;

use crate::shared_functions::TestFile;

macro_rules! compression_test_runner {
    ($path:expr, $num:expr) => {
        // write test data
        let m = matfile!(a: matvar!($num));
        let _ = save_matfile_v7($path, m, false);

        // read data in again
        let m = load_matfile($path).unwrap();

        assert_eq!(m["a"], matvar!($num));
    };
}

/// Test the compession in the v7 write module.
/// When suitable 
#[test]
fn compression() {
    let file = TestFile {
        path: "tests/compression.mat",
    };

    compression_test_runner!(file.path, u8::MAX as f64);
    compression_test_runner!(file.path, i8::MAX as f64);
    compression_test_runner!(file.path, u16::MAX as f64);
    compression_test_runner!(file.path, i16::MAX as f64);
    compression_test_runner!(file.path, u32::MAX as f64);
    compression_test_runner!(file.path, i32::MAX as f64);
    compression_test_runner!(file.path, f32::MAX as f64);
    compression_test_runner!(file.path, f64::MIN);
    compression_test_runner!(file.path, f64::MAX);
}

