pub mod array {
    pub fn run() {
        let mut a1: [i32; 5] = [1, 2, 3, 4, 5];
        println!("{:?}", a1);
        println!("{}, {}, {}, {}, {}", a1[0], a1[1], a1[2], a1[3], a1[4]);

        for i in 0..a1.len() {
            println!("{}", a1[i]);
        }

        let a2 = [3; 5];
        println!("{:?}", a2);
    }
}
