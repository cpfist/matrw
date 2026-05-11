#![cfg(feature = "ndarray")]

use matrw::*;
use ndarray::*;

#[test]
fn ndarray_arr_to_matvar() {
    
    let arr = array![[1, 2, 3, 4]];
    let var = matvar!(arr.clone());

    assert_eq!(arr[[0,0]], var.elem(0).to_i32().unwrap());
    assert_eq!(arr[[0,1]], var.elem(1).to_i32().unwrap());
    assert_eq!(arr[[0,2]], var.elem(2).to_i32().unwrap());
    assert_eq!(arr[[0,3]], var.elem(3).to_i32().unwrap());
}


