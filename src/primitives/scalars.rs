pub mod scalars {
    pub fn run() {
        //, Signed integers
        let signed_integer: i8 = 10; // i18, i16, i32, i64, i128

        //, Unsigned integers
        let unsigned_integer: u8 = 66; // u8, u16, u32, u64, u128

        //, Platform specific integers
        let arch_1: isize = 10;
        let arch_2: usize = 10;

        //, Floating-point numbers
        let float_32: f32 = 50.99;
        let float_64: f64 = 60.88;

        //, Booleans
        let boolean: bool = true;

        //, Characters
        let character: char = 'a';
        let emoji: char = '\u{1F600}';
        println!("{}", emoji); // 😀

        //, Type aliasing
        type Kilometers = i32;
        let distance: Kilometers = 5;
        println!("{}", distance);
    }
}
