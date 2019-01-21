use std::ops::AddAssign;

fn sum<T: AddAssign + Copy + Default>(input: &[T]) -> T {
    let mut total: T = T::default();
    for value in input {
        total += *value;
    }
    total
}

macro_rules! print_items {
    ($($x:expr) *) => {
        $(println!("item is {}", $x);)*
    };
}

macro_rules! check_types {
    ($x:ty) => {
        let data: Vec<$x> =  vec![1, 2, 3];
        let total = sum(&data);
        let expected: $x = 6;
        assert_eq!(total, expected);
        println!("checked sum for {}", stringify!($x));
    };
    ($($x:ty) *) => {
        $(check_types!($x);)*
    };
}

macro_rules! patterns {
    (input ($x:expr) expected ($y:expr)) => {
        println!("input is {:?}", $x);
        println!("expected {:?}", $x);
        assert_eq!($x, $y);
        println!("check successfully passed");
    };
}

fn main() {
    print_items!(1);
    print_items!(42);
    print_items!(3 2 5 7);
    check_types!(u8);
    check_types!(u16);
    check_types!(u32);
    check_types!(u64);
    check_types!(u128);
    check_types!(i8 i16 i32 i64 i128);
    patterns! {
        input (vec![1, 2])
        expected (vec![1, 2])
    };
}
