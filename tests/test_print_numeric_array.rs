use matrw::*;
use paste::paste;

macro_rules! print_type {
    ($t:ty, $v:expr, $vmin:expr, $vmax:expr) => {
        paste! {
        #[test]
        fn [<print_numeric_array_ $t>]() {
            let little = $v;
            let min = $vmin;
            let max = $vmax;

            let m = matvar!([
                [little, little, little],
                [little, little, max],
                [little, max, little],
                [little, max, max],
                [max, little, little],
                [max, little, max],
                [max, max, little],
                [max, max, max],
                [min, min, min],
            ]);
            println!("{m}");
        }
        }
    };
}

print_type!(u8, 1u8, u8::MIN, u8::MAX);
print_type!(i8, 1i8, i8::MIN, i8::MAX);
print_type!(u16, 1u16, u16::MIN, u16::MAX);
print_type!(i16, 1i16, i16::MIN, i16::MAX);
print_type!(u32, 1u32, u32::MIN, u32::MAX);
print_type!(i32, 1i32, i32::MIN, i32::MAX);
print_type!(u64, 1u64, u64::MIN, u64::MAX);
print_type!(i64, 1i64, i64::MIN, i64::MAX);
print_type!(f32, 1f32, f32::MIN, f32::MAX);
print_type!(f64, 1f64, f64::MIN, f64::MAX);
print_type!(char, 'a', 'b', 'c');
print_type!(bool, false, false, true);

macro_rules! print_type_complex {
    ($t:ty, $v:expr, $vmin:expr, $vmax:expr) => {
        paste! {
        #[test]
        fn [<print_complex_numeric_array_ $t>]() {
            let little = $v;
            let min = $vmin;
            let max = $vmax;

            let m = matvar!([
                [(little, little), (little, little), (min, min)],
                [(max, max), (min, max), (little, little)]
            ]);
            println!("{m}");
        }
        }
    };
}

print_type_complex!(u8, 1u8, u8::MIN, u8::MAX);
print_type_complex!(i8, 1i8, i8::MIN, i8::MAX);
print_type_complex!(u16, 1u16, u16::MIN, u16::MAX);
print_type_complex!(i16, 1i16, i16::MIN, i16::MAX);
print_type_complex!(u32, 1u32, u32::MIN, u32::MAX);
print_type_complex!(i32, 1i32, i32::MIN, i32::MAX);
print_type_complex!(u64, 1u64, u64::MIN, u64::MAX);
print_type_complex!(i64, 1i64, i64::MIN, i64::MAX);
print_type_complex!(f32, 1f32, f32::MIN, f32::MAX);
print_type_complex!(f64, 1f64, f64::MIN, f64::MAX);

