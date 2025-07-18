pub fn run() {
    // Array e tipizzazione
    let mut a1: [i32; 5] = [1, 2, 3, 4, 5];

    // Accedere
    println!("{:?}", a1);
    println!("{}, {}, {}, {}, {}", a1[0], a1[1], a1[2], a1[3], a1[4]);

    let a2 = [3; 5];
    println!("{:?}", a2);

    // Iterare
    for i in 0..a1.len() {
        println!("{}", a1[i]);
    }

    let primes = [2, 3, 5, 7, 11, 13, 17, 19];
    for prime in primes {
        for i in 2..prime {
            assert_ne!(prime % i, 0);
        }
    }

    let numeri = [10, 20, 30, 40, 50];
    numeri.iter().for_each(|&n| println!("{}", n));

    let mut mutables = [10, 20, 30, 40, 50];
    mutables.iter_mut().for_each(|n| *n *= 2);
    println!("{:?}", numeri);
}
