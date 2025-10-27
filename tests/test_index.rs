//! Test suite concerning the readout of MAT-file data via the public interface.

use matrw::{interface::variable::OwnedIndex, *};

#[test]
fn index_vec_data() {
    let a = matvar!([1, 2, 3]);

    println!(" First: {}", a.elem(0));
    println!("Second: {}", a.elem(1));
    println!(" Third: {}", a.elem(2));
}
