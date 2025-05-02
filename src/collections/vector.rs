pub mod vector {
    pub fn run() {
        //# Creazione di un vector
        let mut v1: Vec<i32> = Vec::new(); //? Creazione di un vector vuoto
        let mut v2 = vec![1, 2, 3, 4, 5]; //? Creazione di un vector con elementi

        println!("{:?}", v1);

        v2[2] = 10; //? Riassegnazione di un elemento

        let third: &i32 = &v2[2];
        println!("Il terzo elemento è {}", third);

        //# Operazioni di accesso
        match v2.get(2) {
            Some(third) => println!("Il terzo elemento è {}", third),
            None => println!("Non c'è un terzo elemento."),
        }

        v2.push(77);

        println!("{}", v2.len()); // 4

        let removed = v2.pop();
        println!("{:?}", removed); // Some(77)

        //# Iterazione di accesso
        for i in v2.iter() {
            println!("{}", i);
        } // 1, 2, 3, 4, 5

        //# Iterazione di accesso e modifica
        for i in v2.iter_mut() {
            *i += 50;
            println!("{}", i);
        } // 51, 52, 53, 54, 55
    }
}
