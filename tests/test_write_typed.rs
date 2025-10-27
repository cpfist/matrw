//! Test suite concerning the serde interface.

use serde::Serialize;

use matrw::*;

#[path = "shared_functions.rs"]
mod shared_functions;

use crate::shared_functions::TestFile;

#[test]
/// Serde serialize
fn serde_serialize_double() {
    #[derive(Serialize)]
    struct Example {
        var1: f64,
    }

    let e = Example { var1: 42.0 };
    let matfile = to_matfile(e).expect("Serializing failed");

    println!("{:#?}", matfile);
}

#[test]
/// Serde serialize
fn run_serde_serialize_double() {
    #[derive(Serialize, Debug)]
    struct S {
        f1: f64,
        f2: u64,
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
        n: S { f1: 42.0, f2: 43 },
    };

    let filepath = TestFile { path: "test.mat" };

    let matfile = to_matfile(e).expect("Serializing failed");

    println!("{:#?}", matfile);

    let _ = save_matfile_v7(filepath.path, matfile, false);
}
