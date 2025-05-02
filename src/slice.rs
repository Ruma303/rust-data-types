pub fn run() {
    let a = [1, 2, 3, 4, 5];
    let s1: &[i32] = &a[1..4];
    println!("{:?}", s1);
    let s2: &[i32] = &a[0..=4];

    // String litterals are string slices
    let s = "Hello, world!";
    let word2 = first_word(s);
    println!("{}", word2);
}

fn first_word(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  }

  &s[..]
}