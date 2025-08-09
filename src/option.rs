//% Option enum
/* enum Option<T> {
    None,
    Some(T)
} */

fn index_of(numbers: &[u32], target: u32) -> Option<usize> {
    let mut i = 0;
    for &n in numbers {
        if n == target {
            return Some(i);
        }
        i += 1;
    }
    None
}

struct Customer {
    age: Option<u32>,
    email: String,
}

struct GrocieryItem {
    name: String,
    quantity: i32,
}

fn find_quantity(name: &str) -> Option<i32> {
    let grocieries = vec![
        GrocieryItem {
            name: "apple".to_string(),
            quantity: 10,
        },
        GrocieryItem {
            name: "banana".to_string(),
            quantity: 15,
        },
        GrocieryItem {
            name: "orange".to_string(),
            quantity: 20,
        },
    ];

    for item in &grocieries {
        if item.name == name {
            return Some(item.quantity);
        }
    }
    None
}

pub fn run() {
    let number = Some(10);
    let boolean = Some(true);
    let nothing: Option<i32> = None;

    //, Handling Option enum
    let num_1 = Some(10);
    let num_2 = 5;

    //# match
    match num_1 {
        Some(num_1) => println!("num_1: {}", num_1),
        None => println!("num_1: None"),
    }

    //# if let
    if let Some(num_1) = num_1 {
        println!("num_1: {}", num_1);
    } else if let None = num_1 {
        println!("num_1: None");
    }

    println!("{}", num_1.unwrap() + num_2); // 15

    //# No values
    let dice_roll = 4;

    match dice_roll {
        1 => println!("1"),
        2 => println!("2"),
        3 => (),
        4 => (),
        5 => println!("5"),
        6 => println!("6"),
        _ => println!("Other"),
    }

    //# Es.3: methods
    let numbers = [1, 2, 3, 4, 5];
    let index = index_of(&numbers, 3);
    println!("Index of 3: {}", index.unwrap());

    let quantity = find_quantity("apple");
    println!("Quantity of apple: {}", quantity.unwrap());

    //# Es.4: Compounds data
    let customer1 = Customer {
        age: Some(30),
        email: "user1@email".to_string(),
    };

    let customer2 = Customer {
        age: None,
        email: "user2@email".to_string(),
    };

    match customer1.age {
        Some(age) => println!("Age: {}", age),
        None => println!("Age: None"),
    }

    match customer2.age {
        Some(age) => println!("Age: {}", age),
        None => println!("Age: None"),
    }

    //, Accesso sicuro ai dati
    let x: Option<i32> = Some(10);
    let y: i32 = 14;
    //let sum = x + y; //. Non sono lo stesso dato
    //println!("{}", sum);

    //* Metodi sicuri
    let sum = x.unwrap() + y;
    println!("{}", sum);

    let sum = x.unwrap_or(0) + y;
    println!("{}", sum);

    let sum = x.unwrap_or_else(|| 0) + y;
    println!("{}", sum);

    let sum = x.unwrap_or_default() + y;
    println!("{}", sum);
}
