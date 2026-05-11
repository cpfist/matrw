#![cfg(feature = "ndarray")]

/// Convert a MATLAB column-major data storage to ndarray row-major
///
pub fn mat_col_to_ndarray_row<T: Copy + std::fmt::Debug>(data: &[T], dims: &[usize]) -> Vec<T> {
    let total_size: usize = dims.iter().product();
    assert_eq!(data.len(), total_size, "Data length does not match dimensions.");

    let mut result = vec![data[0]; total_size];

    let idx_ref: Vec<usize> = Vec::from(&dims[2..]);
    let mut idx: Vec<usize> = vec![0; idx_ref.len()];

    let mut global_index = 0;
    let len = dims.iter().product::<usize>();

    let mut count = 0;
    loop {
        for r in 0..dims[0] {
            for c in 0..dims[1] {
                let idx_r = global_index + c*dims[0] + r;
                result[count] = data[idx_r];
                count += 1;
            }
        }
        global_index += dims[0] * dims[1];

        for i in 0..idx_ref.len() {
            if idx[i] < idx_ref[i] - 1 {
                idx[i] += 1;
                break;
            }
            if i < idx_ref.len() - 1 {
                idx[i] = 0;
            }
        }

        if global_index == len {
            break;
        }
    }

    result
}

/// Convert array dimension description from MATLAB to ndarray format
///
pub fn mat_dim_to_ndarray_dim(dim: &[usize]) -> Vec<usize> {
    // Switch 2d and higher dim indices
    let idx_2d = vec![dim[0], dim[1]];
    let idx_higher = dim[2..].to_vec().iter().cloned().rev().collect::<Vec<_>>();

    [idx_higher, idx_2d].into_iter().flatten().collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matvar;
    use ndarray::ArrayD;

    #[test]
    fn test_col_to_row_major() {
        let m = matvar!([
            [
                [0, 1, 2], 
                [3, 4, 5]
            ],
            [
                [6, 7, 8],
                [9, 10, 11]
            ]
        ]);
        let dims = m.dim();
        let data = m.to_vec_i32().unwrap();

        let dims_conv = mat_dim_to_ndarray_dim(&dims);
        let conv = mat_col_to_ndarray_row(&data, &dims);

        let arr = ArrayD::from_shape_vec(dims_conv, conv).unwrap();

        let arr_cmp = ArrayD::<i32>::from_shape_vec(
            vec![2, 2, 3], 
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
            )
            .unwrap();

        assert_eq!(arr, arr_cmp);
    }
}
