pub mod generic {

  pub fn run() {
    //% Generics
    let a1 = 10;
    let a2 = 20;
    let a3 = 30;

    println!("a1: {}, a2: {}, a3: {}", a1, a2, a3);

    let result = add(a1, a2);
    println!("Result of add: {}", result);

    let result = add(a1, a3);
    println!("Result of add: {}", result);

    let result = add(a2, a3);
    println!("Result of add: {}", result);
  }

  //% Funzione generica
  fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
  }
}