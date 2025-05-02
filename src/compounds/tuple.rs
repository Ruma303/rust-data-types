pub mod tuple {

    pub fn run() {
        let t1: (i32, f64, bool) = (42, 6.12, false);
        println!("{:?}", t1);
        println!("{}, {}, {}", t1.0, t1.1, t1.2);
    }
}
