/// Construct a `MatVariable`.
///
/// ```
/// # use matrw::matvar;
/// #
/// // Construct a scalar (real) value
/// let var = matvar!(1.0);
/// ```
///
/// Convenience macro to construct a new [`crate::MatVariable`]. The macro accepts different patterns allowing the construction of different variants of `MatVariable`, see section [Examples](#examples).
///
/// The design of this macro is strongly inspired by the macro [`serde_json::json`](https://docs.rs/serde_json/1/serde_json/macro.json.html).
///
/// # Panics
///
/// Panics occur when something goes wrong on construction of a underlying `MatVariable` variants.
///
/// # Examples
/// ```
/// # use matrw::matvar;
/// #
/// // Construct a scalar complex value
/// let var = matvar!((1.0, 1.0));
/// ```
/// ```
/// # use matrw::matvar;
/// #
/// // Construct a row vector
/// let var = matvar!([1., 2., 3.]);
/// ```
/// ```
/// # use matrw::matvar;
/// #
/// // Construct a 2x3 matrix from row vectors
/// let var = matvar!([
///             [1., 2., 3.],
///             [4., 5., 6.],
///         ]);
///
/// // Construct the same 2x3 matrix, but differently
/// let v1 = vec![1., 2., 3.];
/// let v2 = vec![4. ,5., 6.];
/// let var = matvar!([v1, v2]);
/// ```
/// ```
/// # use matrw::matvar;
/// #
/// // Construct a multidimensional 2x3x2 numeric array from row vectors
/// let var = matvar!([
///         [
///             [1., 2., 3.],
///             [4., 5., 6.],
///         ],
///         [
///             [11., 12., 13.],
///             [14., 15., 16.],
///         ],
///         ]);
/// ```
/// ```
/// # use matrw::matvar;
/// #
/// // Construct a structure
/// let var = matvar!({
///             a: 1.0,
///             b: 2.0,
///         });
/// ```
/// ```
/// # use matrw::matvar;
/// #
/// // Construct a 1x2 structure array
/// let var = matvar!([
///         {
///             a: 1.0,
///             b: 2.0,
///         },
///         {
///             a: 42.0,
///             b: 43.0,
///         },
///         ]);
/// ```
/// ```
/// # use matrw::matvar;
/// #
/// // Construct a 1x2 cell array
/// let var = matvar!([
///         "some text",
///         {
///             a: 42.0,
///             b: 43.0,
///         },
///         ]);
/// ```
///
#[macro_export]
macro_rules! matvar {
    ($($matvar:tt)+) => {
        $crate::matvar_internal!($($matvar)+)
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! matvar_internal {
    // -------------
    // Array parsing
    // -------------

    // Next element is an expression followed by comma.
    (@array [$($elems:expr,)*] $next:tt, $($rest:tt)*) => {{
        $crate::matvar_internal!(@array [$($elems,)* $crate::matvar_internal!($next),] $($rest)*)
    }};

    // Last element is an expression with no trailing comma.
    (@array [$($elems:expr,)*] $last:tt) => {{
        $crate::matvar_internal!(@array [$($elems,)* $crate::matvar_internal!($last)])
    }};

    // Comma after the most recent element.
    (@array [$($elems:expr),*] , $($rest:tt)*) => {
        $crate::matvar_internal!(@array [$($elems,)*] $($rest)*)
    };

    // Done with trailing comma.
    (@array [$($elems:expr,)*]) => {{
        let v = vec![$(($elems),)*];
        if v.iter().all(|x| matches!(x, $crate::MatVariable::NumericArray(_))) && $crate::check_same_dim(&v) && $crate::check_same_type(&v) {
            $crate::MatVariable::NumericArray($crate::NumericArray::from_nested_matvar(vec![1, v.len()], v).unwrap())
        } else if v.iter().all(|x| matches!(x, $crate::MatVariable::Structure(_))) && $crate::check_same_fields(&v) {
            $crate::MatVariable::StructureArray($crate::StructureArray::from_structures(vec![1, v.len()], v))
        } else {
            $crate::MatVariable::CellArray($crate::CellArray::new(vec![1, v.len()], v).unwrap())
        }
    }};

    // Done without trailing comma.
    (@array [$($elems:expr),*]) => {{
        let v = vec![$(($elems)),*];
        if v.iter().all(|x| matches!(x, $crate::MatVariable::NumericArray(_))) && $crate::check_same_dim(&v) && $crate::check_same_type(&v) {
            $crate::MatVariable::NumericArray($crate::NumericArray::from_nested_matvar(vec![1, v.len()], v).unwrap())
        } else if v.iter().all(|x| matches!(x, $crate::MatVariable::Structure(_))) && $crate::check_same_fields(&v) {
            $crate::MatVariable::StructureArray($crate::StructureArray::from_structures(vec![1, v.len()], v))
        } else  {
            $crate::MatVariable::CellArray($crate::CellArray::new(vec![1, v.len()], v).unwrap())
        }
    }};

    // -----------------
    // Structure parsing
    // -----------------

    (@structure $structure:ident () () ()) => {};

    // Insert the current entry followed by trailing comma.
    (@structure $structure:ident ($key:ident) ($value:expr) , $($rest:tt)*) => {
        let _ = $structure.insert(stringify!($key).into(), $value);
        $crate::matvar_internal!(@structure $structure () ($($rest)*) ($($rest)*));
    };

    // Insert the last entry without trailing comma.
    (@structure $structure:ident ($key:ident) ($value:expr)) => {
        let _ = $structure.insert(stringify!($key).into(), $value);
    };

    // Next value is an array.
    (@structure $structure:ident ($key:ident) (: [$($array:tt)*] $($rest:tt)*) $copy:tt) => {
        $crate::matvar_internal!(@structure $structure ($key) ($crate::matvar_internal!([$($array)*])) $($rest)*);
    };

    // Next value is a map.
    (@structure $structure:ident ($key:ident) (: {$($map:tt)*} $($rest:tt)*) $copy:tt) => {
        $crate::matvar_internal!(@structure $structure ($key) ($crate::matvar_internal!({$($map)*})) $($rest)*);
    };

    // Next value is an expression followed by comma.
    (@structure $structure:ident ($key:ident) (: $value:expr , $($rest:tt)*) $copy:tt) => {
        $crate::matvar_internal!(@structure $structure ($key) ($crate::matvar_internal!($value)) , $($rest)*);
    };

    // Last value is an expression with no trailing comma.
    (@structure $structure:ident ($key:ident) (: $value:expr) $copy:tt) => {
        $crate::matvar_internal!(@structure $structure ($key) ($crate::matvar_internal!($value)));
    };

    // Munch a token into the current key.
    (@structure $structure:ident () ($tt:ident $($rest:tt)*) $copy:tt) => {
        $crate::matvar_internal!(@structure $structure ($tt) ($($rest)*) ($($rest)*));
    };

    // ----------
    // Main cases
    // ----------

    // Match an empty array
    ([]) => {
        $crate::MatVariable::NumericArray($crate::NumericArray::from_nested_matvar(vec![0, 0], vec![]).unwrap())
    };

    // Match an array
    ([ $($tt:tt)+ ]) => {{
        $crate::matvar_internal!(@array [] $($tt)+)
    }};

    // Match an empty Structure
    ({}) => {{
        $crate::MatVariable::Structure($crate::Structure::new($crate::__private::IndexMap::new()))
    }};

    // Match a Structure
    ({ $($tt:tt)+ }) => {
        $crate::MatVariable::Structure($crate::Structure::new({
            let mut structure = $crate::__private::IndexMap::new();
            $crate::matvar_internal!(@structure structure () ($($tt)+) ($($tt)+));
            structure
        }))
    };

    // Any Serialize type: numbers, strings, struct literals, variables etc.
    // Must be below every other rule.
    ($other:expr) => {{
        $crate::MatVariable::from($other)
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn macro_test_1() {
        let v = matvar!([true, true, true, false]);
        println!("v = {:#?}", v);

        let x = v[[0, 0]].to_bool();
        println!("{:?}", x)
    }

    #[test]
    fn macro_test_2() {
        let v = matvar!([1.0]);
        println!("v = {:#?}", v);

        let x = v[[0, 0]].to_f64();
        println!("{:?}", x)
    }

    #[test]
    fn macro_test_3() {
        let v = matvar!([]);
        println!("v = {:#?}", v);
    }

    #[test]
    fn macro_test_4() {
        let v = matvar!({});
        println!("v = {:#?}", v);
    }

    #[test]
    fn macro_test_5() {
        let v = matvar!({
            code: 200,
            success: true,
            payload: {
                features: [
                    1.0,
                    2.0,
                ],
                homepage: false
            }
        });

        println!("v = {:#?}", v);
    }

    #[test]
    fn macro_test_6() {
        let v = matvar!([
            { f1: 1.0 },
            { f1: 2.0 }
        ]);

        println!("v = {:#?}", v);
    }

    #[test]
    fn macro_test_7() {
        let v = matvar!([
            { f1: [1.0, 2.2] },
            { f2: 2.0 }
        ]);

        println!("v = {:#?}", v);
    }

    #[test]
    fn macro_test_8() {
        let v = matvar!("test");

        println!("v = {:#?}", v);
    }

    #[test]
    fn macro_test_9() {
        let v = matvar!([1, 2, 3, 4]);
        println!("v = {:#?}", v);
    }

    #[test]
    fn macro_test_10() {
        let v = matvar!([[1, 2], [3, 4]]);
        println!("v = {:#?}", v);
    }

    #[test]
    fn macro_test_11() {
        let v = matvar!([[[1, 2], [3, 4]], [[5, 6], [7, 8]]]);
        println!("v = {:#?}", v);
    }

    #[test]
    fn macro_test_12() {
        let v = matvar!([
            [[[1, 2], [3, 4]], [[5, 6], [7, 8]]],
            [[[11, 12], [13, 14]], [[15, 16], [17, 18]]]
        ]);
        println!("v = {:#?}", v);
    }

    #[test]
    fn macro_test_13() {
        let v = matvar!([(1.0, 42.), (2.0, 43.), (3.0, 44.)]);
        println!("v = {:#?}", v);
    }
}

///
/// Construct a [`crate::MatFile`] from a key-`MatVariable`-pair.
///
/// ```
/// use matrw::{matfile, matvar};
///
/// let var = matvar!(1);
///
/// // Create a MAT-file with variables "a"
/// let mat = matfile!(
///     // Insert Rust variable "var" as "a" in MAT-file
///     a: var,
///     // Insert another variable "b"
///     b: matvar!(42.),
/// );
/// ```
///
/// # Panics
///
/// #### Invalid variable names
///
/// Panics may occur, when an invalid variable name is used. Since the macro pattern asks for
/// idents, most requirements for a valid name are checked by the Rust compiler. The exception are
/// idents with leading underscores and the use of keywords, see also [`crate::MatFile::insert`].
///
/// #### Nested `matvar` call
///
/// Panics can also occur from nested [`matvar`] calls.
///
#[macro_export]
macro_rules! matfile {
    ($($matfile:tt)+) => {
        $crate::matfile_internal!($($matfile)+)
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! matfile_internal {
    (@variable $mat:ident $($name:ident: $var:expr,)*) => {{
        $(
        let varname = stringify!($name);
        $mat.insert(varname, $var);
        )*
        $mat
    }};

    (@variable $mat:ident $($name:ident: $var:expr),*) => {{
        $(
        let varname = stringify!($name);
        $mat.insert(varname, $var);
        )*
        $mat
    }};

    () => {
        $crate::MatFile::new()
    };

    ( $($tt:tt)+ ) => {{
        let mut m = $crate::MatFile::new();
        $crate::matfile_internal!(@variable m $($tt)+)
    }};

}

#[cfg(test)]
mod matfile_tests {
    #[test]
    fn matfile_1() {
        let f = matfile!(
        var1: matvar!(1.0),
        var2: matvar!(2),
        );
        println!("{:#?}", f)
    }
}
