//% Result enum
/* enum Result<T, E> {
    Ok(T),
    Err(E)
} */

use std::fs::File;
use std::io::{self, Error, ErrorKind, Read};

fn validate_email(email: &str) -> Result<(), Error> {
    if email.contains('@') {
        Ok(())
    } else {
        Err(Error::new(ErrorKind::InvalidInput, "Invalid email"))
    }
}

fn read_file() -> Result<String, io::Error> {
    let mut file = File::open("data.txt")?; // Se Err, ritorna subito Err
    let mut content = String::new();
    file.read_to_string(&mut content)?; // Se Err, ritorna subito Err
    Ok(content)
}

pub fn run() -> Result<(), io::Error> {
    //% Unrecoverable errors
    let my_vec = vec![1, 2, 3];
    // let not_existing_element: i32 = my_vec[10];

    //% Recoverable errors
    let email = "email@email.com";
    match validate_email(email) {
        Ok(()) => println!("Email validated."),
        // oppure:
        // Ok(_) => println!("Email validated."),
        // Ok(..) => println!("Email validated."),
        Err(e) => println!("Error: {}", e),
    }

    //, Try operator
    let data = read_file()?; // Propaga errori
    println!("{}", data);

    Ok(())
}