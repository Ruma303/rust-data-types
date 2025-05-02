pub fn run() {
    let a = [1, 2, 3, 4, 5];
    let s1: &[i32] = &a[1..4];
    println!("{:?}", s1);
    let s2: &[i32] = &a[0..=4];
}
