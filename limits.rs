macro_rules! show_limits {
    ($($x:ident) *) => {
        $(println!("{} min: {}, max: {}", stringify!($x), std::$x::MIN, std::$x::MAX);)*
    };
}

fn main() {
    show_limits!(u8 i8 u16 i16 u32 i32 u64 i64 u128 i128 f32 f64);
}
