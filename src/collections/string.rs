pub mod string {
    // Strings are UTF-8 encoded, growable, and mutable

    pub fn run() {
        //, &str

        //, String

        let mut s = String::from("hello");
        s.push_str(", world!");
        println!("{}", s); // "hello, world!"

        //, &str
    }
}
