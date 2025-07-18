// Riferimenti immutabili

fn basic_slices() {
    let numeri = [10, 20, 30, 40, 50];

    let slice1: &[i32] = &numeri[1..4]; // [20, 30, 40]
    let slice2: &[i32] = &numeri[..3]; // [10, 20, 30]
    let slice3: &[i32] = &numeri[2..]; // [30, 40, 50]
    let slice4: &[i32] = &numeri[..]; // L'intero array [10, 20, 30, 40, 50]

    println!("Lunghezza array: {:?}", numeri.len());
    println!("Slice1: {:?}", slice1);
    println!("Slice2: {:?}", slice2);
    println!("Slice3: {:?}", slice3);
    println!("Slice4: {:?}", slice4);
}

fn sum(slice: &[i32]) -> i32{
    slice.iter().sum()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b',' {
            return &s[0..i];
        }
    }

    &s[..]
}

pub fn run() {

    // Riferimenti immutabili
    basic_slices();

    let a = [1, 2, 3, 4, 5];
    // Creazione slice da array dagli elementi 1 al 4 incluso
    let s1: &[i32] = &a[1..=4];
    println!("s1: {:?}", s1); // [2, 3, 4, 5]

    // Creazione slice da altri slice dagli elementi 1 al 3 escluso
    let s2: &[i32] = &s1[1..3];
    println!("s2: {:?}", s2); // [3, 4]

    // String litterals e string slices
    let string = "Hello, world!";
    let word2 = first_word(string);
    println!("{}", word2);

    let numeri = [10, 20, 30, 40, 50];
    let slice = &numeri[1..4]; // [20, 30, 40]
    println!("Somma della slice: {}", sum(slice)); // 90

    // Riferimenti mutabili
    let mut mutables = [10, 20, 30, 40, 50];
    let slice_mutable = &mut mutables[2..]; //
    slice_mutable[0] = 100;
    println!("Slice modificato: {:?}", slice_mutable); // [100, 40, 50]

    let slice_immutable = &mutables[2..];
    println!("Slice non modificato: {:?}", slice_immutable); // [40, 50]
}