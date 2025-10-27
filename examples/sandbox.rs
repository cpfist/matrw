#![allow(dead_code, unused)]

use matrw::*;
use rand::rng;
use rand_pcg::Pcg64Mcg;
use serde::Deserialize;

fn main() {
    // example1();
    // example2();
    // example3();
    // example4();
    // example5();
    // example6();
    // example7();
    // example8();
    // example9();
    // example10();
    // example11();
    // example12();
    // example13();
    // example14();
    // example15();
    // example16();
    // example17();
    example18();
}

fn example1() {
    let m = matfile!(
       s: matvar!([
           // struct array
           [
               {
                   f1: 42.,
                   f2: "test",
                   f3: 1.,
               },
               {
                   f1: 43.,
                   f2: "test1",
                   f3: 2.,
               },
           ],
           // numeric array
           [1, 2, 3],
           // numeric array of chars
           "asd",
           // cell array of structs
           [
               {
                   a: 1.0
               },
               {
                   b: 2.0
               }
           ],
           // cell array of structs
           [
               {
                   c: 1.0
               },
               {
                   d: 2.0
               }
    ],
           // empty struct
           {
           },
           // Scalar
           1,
           //
           'c',
           [],
           [
               [1, 2],
               [3, 4],
           ],
           [
               [
                   [1, 2],
                   [3, 4],
               ],
               [
                   [5, 6],
                   [7, 8],
               ],
           ],
       ])
       );

    println!("{m:#?}");
}
fn example2() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];

    let m = matfile!(
    s: matvar!([v1, v2])
    );

    let _ = save_matfile_v7("test.mat", m, false);
}
fn example3() {
    let x1 = matvar!({a: 1.0});
    let x2 = matvar!(2.0);
    let x = MatVariable::CellArray(CellArray::new(vec![1, 2], vec![x1, x2]).unwrap());

    let m = matfile!(x: x);
    let _ = save_matfile_v7("test.mat", m, false);
}
fn example4() {
    let m = matfile!(
    a: matvar!(vec![1.0, 2.0, 3.0]),
    b: matvar!(vec!['a', 'b', 'c']),
    c: matvar!(false),
    );

    #[derive(Debug, Deserialize)]
    #[allow(dead_code)]
    struct A {
        a: Vec<f64>,
        b: String,
        c: bool,
    }

    let a: A = from_matfile(&m).unwrap();

    println!("a: {:#?}", a);
}
fn example5() {
    let _ = load_matfile("benches/large.mat").unwrap();
}
fn example6() {
    let a = matvar!([[1., 2.], [3., 4.],]);

    for i in a.iter() {
        println!("{}", i);
    }

    let x = a.to_u8();
}
fn example7() {
    // let a = matvar!([
    //     [1., 2.],
    //     [3., 4.],
    // ]);
    //
    let a = matvar!([[0., 0.], [0., 0.],]);

    // let a = matvar!([
    //     [false, false],
    //     [false, false],
    // ]);

    let s = a.to_sparse().unwrap();

    let m = matfile!(s: s);

    save_matfile_v7("test.mat", m, false);
}
fn example8() {
    let a = matvar!([[1., 0.], [0., 4.],]);
    let s = a.to_sparse().unwrap();

    let m = matfile!(s: matvar!({sp: s}));

    save_matfile_v7("test.mat", m, true);
}
fn example9() {
    use matrw::{matfile, matvar, save_matfile_v7};

    // Create some data
    let mat = matfile!(
        a: matvar!(42.), // Scalar
        b: matvar!([1., 2., 3.]), // Vector
        c: matvar!([[1, 2], [3, 4]]), // Matrix
        d: matvar!("Some text"), // String
    );

    save_matfile_v7("test.mat", mat, false);
}
fn example10() {
    use matrw::load_matfile;

    // Load a MAT-file
    let mat = load_matfile("test.mat").expect("Could not read file.");

    // Inspect file
    assert_eq!(mat["a"].to_f64(), Some(42.));

    assert_eq!(mat["b"].to_vec_f64(), Some(vec![1., 2., 3.]));

    assert_eq!(mat["b"].elem(0).to_f64(), Some(1.));
    assert_eq!(mat["b"].elem(1).to_f64(), Some(2.));
    assert_eq!(mat["b"].elem(2).to_f64(), Some(3.));

    assert_eq!(mat["c"].to_vec_i32(), Some(vec![1, 3, 2, 4]));

    assert_eq!(mat["c"].elem([0, 0]).to_i32(), Some(1));
    assert_eq!(mat["c"].elem([0, 1]).to_i32(), Some(2));
    assert_eq!(mat["c"].elem([1, 0]).to_i32(), Some(3));
    assert_eq!(mat["c"].elem([1, 1]).to_i32(), Some(4));
}
fn example11() {
    use rand::prelude::*;
    use rand_pcg::Pcg64;

    println!("Generating the data");

    let n = 15000 * 15000; // number of elements

    let mut rng = Pcg64Mcg::seed_from_u64(1);

    // Preallocate vector and fill in-place
    let mut data = vec![0.0f64; n];
    for x in &mut data {
        *x = rng.random();
    }

    println!("Putting into variable");

    let var = MatVariable::NumericArray(
        NumericArray::new(vec![15000, 15000], MatlabType::from(data), None).unwrap(),
    );

    println!("Putting into matfile");
    let mat = matfile!(var: var);

    println!("Now writing");
    save_matfile_v7("test.mat", mat, false);

    println!("Done");
}
fn example12() {
    let mat = matfile!(
        // scalar number
        a: matvar!(1),
        // scalar character
        b: matvar!('c'),
        // empty value
        c: matvar!([]),
        // vector
        d: matvar!([1, 2, 3]),
        // character array
        e: matvar!("asd"),
        // 2x2 matrix
        f: matvar!([
                [1, 2],
                [3, 4],
            ]),
        // 2x2x2 array
        g: matvar!([
                [
                    [1, 2],
                    [3, 4],
                ],
                [
                    [5, 6],
                    [7, 8],
                ],
            ]),
        // scalar struct
        h: matvar!({
                f1: 42.,
                f2: "abc",
            }),
        // struct array
        i: matvar!([
                {
                    f1: 42.,
                    f2: "test",
                    f3: 1.,
                },
                {
                    f1: 43.,
                    f2: "test1",
                    f3: 2.,
                },
            ]),
        // empty struct
        j: matvar!({}),
        // cell array
        k: matvar!([
                "abc",
                42.,
                1,
            ])
    );

    let _ = save_matfile_v7("test.mat", mat, false);
}
fn example13() {
    use matrw::load_matfile;

    // Load a MAT-file
    let mat = load_matfile("test.mat").expect("Cannot not read file.");

    // Inspect values
    assert_eq!(mat["a"].to_i32(), Some(1));
    assert_eq!(mat["d"].elem(2).to_i32(), Some(3));
    assert_eq!(mat["f"].elem([1, 1]).to_i32(), Some(4));
    assert_eq!(mat["g"].elem([1, 0, 1]).to_i32(), Some(7));
    assert_eq!(mat["h"]["f1"].to_f64(), Some(42.));
    assert_eq!(mat["i"][1]["f3"].to_f64(), Some(2.));
    assert_eq!(mat["k"][1].to_f64(), Some(42.));
}
fn example14() {
    use matrw::{save_matfile_v7, to_matfile};
    use serde::Serialize;

    #[derive(Serialize)]
    struct S {
        f1: f64,
        f2: String,
    }

    #[derive(Serialize)]
    struct MyMat {
        a: i32,
        b: char,
        d: Vec<i32>,
        e: String,
        h: S,
    }

    let data = MyMat {
        a: 1,
        b: 'c',
        d: vec![1, 2, 3],
        e: "asd".to_string(),
        h: S {
            f1: 42.,
            f2: "abc".to_string(),
        },
    };

    let mat = to_matfile(data).expect("Cannot serialize data");
    let _ = save_matfile_v7("test.mat", mat, false);
}
fn example15() {
    use matrw::{from_matfile, load_matfile};
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct S {
        f1: f64,
        f2: String,
    }

    #[derive(Deserialize)]
    struct MyMat {
        a: i32,
        b: char,
        d: Vec<i32>,
        e: String,
        h: S,
    }

    // Load a MAT-file
    let matfile = load_matfile("test.mat").expect("Cannot not read file.");
    let mat: MyMat = from_matfile(&matfile).expect("Cannot deserialize data");

    // Inspect values
    assert_eq!(mat.a, 1);
    assert_eq!(mat.d[2], 3);
    assert_eq!(mat.e, "asd".to_string());
    assert_eq!(mat.h.f1, 42.);
}
fn example16() {
    use matrw::{from_matfile, load_matfile, save_matfile_v7, to_matfile};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct S {
        f1: f64,
        f2: String,
    }

    #[derive(Serialize, Deserialize)]
    struct MyMat {
        a: i32,
        b: char,
        d: Vec<i32>,
        e: String,
        h: S,
    }

    let data = MyMat {
        a: 1,
        b: 'c',
        d: vec![1, 2, 3],
        e: "asd".to_string(),
        h: S {
            f1: 42.,
            f2: "abc".to_string(),
        },
    };

    let mat = to_matfile(data).expect("Cannot serialize data");
    let _ = save_matfile_v7("test.mat", mat, false);

    // Load a MAT-file
    let matfile = load_matfile("test.mat").expect("Cannot not read file.");
    let mat: MyMat = from_matfile(&matfile).expect("Cannot deserialize data");

    // Inspect values
    assert_eq!(mat.a, 1);
    assert_eq!(mat.d[2], 3);
    assert_eq!(mat.e, "asd".to_string());
    assert_eq!(mat.h.f1, 42.);
}
fn example17() {
    use matrw::{load_matfile, matfile, matvar, save_matfile_v7};

    // Create a MAT-file with variables "a", "b", "c", ...
    let mat = matfile!(
        // scalar number
        a: matvar!(1),
        // scalar character
        b: matvar!('c'),
        // empty value
        c: matvar!([]),
        // vector
        d: matvar!([1, 2, 3]),
        // character array
        e: matvar!("asd"),
        // 2x2 matrix
        f: matvar!([
                [1, 2],
                [3, 4],
            ]),
        // 2x2x2 array
        g: matvar!([
                [
                    [1, 2],
                    [3, 4],
                ],
                [
                    [5, 6],
                    [7, 8],
                ],
            ]),
        // scalar struct
        h: matvar!({
                f1: 42.,
                f2: "abc",
            }),
        // struct array
        i: matvar!([
                {
                    f1: 42.,
                    f2: "test",
                    f3: 1.,
                },
                {
                    f1: 43.,
                    f2: "test1",
                    f3: 2.,
                },
            ]),
        // empty struct
        j: matvar!({}),
        // cell array
        k: matvar!([
                "abc",
                42.,
                1,
            ])
    );

    // Write MAT-file without using compression
    let _ = save_matfile_v7("test.mat", mat, false);

    // ============================================================================

    // Load a MAT-file
    let mat = load_matfile("test.mat").expect("Could not read file.");

    // Inspect data
    // ------------
    // Access variable "a" and convert it to i32
    assert_eq!(mat["a"].to_i32(), Some(1));
    // Access variable "b" and convert element "2" to i32
    assert_eq!(mat["d"].elem(2).to_i32(), Some(3));
    // Access variable "f" and convert element "(1,1)" to i32
    assert_eq!(mat["f"].elem([1, 1]).to_i32(), Some(4));
    // Access variable "g" and convert element "[1,0,1]" to i32
    assert_eq!(mat["g"].elem([1, 0, 1]).to_i32(), Some(7));
    // Access variable "h", then field "f1" and convert to f64
    assert_eq!(mat["h"]["f1"].to_f64(), Some(42.));
    // Access variable "i", then struct index "1" and convert to f64
    assert_eq!(mat["i"][1]["f3"].to_f64(), Some(2.));
    // Access variable "k", then cell index "1" and convert to f64
    assert_eq!(mat["k"][1].to_f64(), Some(42.));
}
fn example18() {
    let a = matvar!([]);
    let m = matfile!(
        a: a
    );
}
