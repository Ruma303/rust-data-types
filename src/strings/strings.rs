pub fn run() {
    //, String

    //# Creazione String 1
    let mut mutable_string = String::from("hello");
    mutable_string.push_str(", world");
    mutable_string.push('!');
    println!("{}", mutable_string); // "hello, world!"

    //# Creazione String 2
    let mut new_string = String::new();
    new_string.push(char::from(97));
    println!("{}", new_string); // "a"

    //# Creazione String 3
    let from_literal = "hello".to_string();
    println!("{}", from_literal); // "hello"

    //, &str

    //# Creazione implicita
    let message1 = "String type: &'static str";
    println!("{}", message1); // "hello, world!"

    //# Creazione esplicita
    let message2: &str = "String type: &str";
    println!("{}", message2); // "hello, world!"

    //# Coercion
    let message3 = String::from("Changing from String to &str");
    println!("{}", message3.as_str()); // "hello, world!"

    //# Slicing
    let message4 = String::from("Changing from String to &str");
    println!("{}", &message4[0..8]); // "Changing"

    //, &String

    //# Creazione con riferimento immutabile
    let s = String::from("Esempio");
    let r: &String = &s; // riferimento immutabile

    //# Creazione con riferimento mutabile
    let mut s = String::from("Esempio");
    let r: &mut String = &mut s; // riferimento mutabile
    r.push_str(" aggiunto");
    println!("{}", r);

    //# Utilizzo
    let testo = String::from("Ferris");
    let len = lunghezza(&testo);
    println!("Lunghezza: {}", len);
}


fn lunghezza(s: &String) -> usize {
    s.len()
}