mod enums;
mod generics;
mod structs;
mod traits;

use std::{fmt::Error, os::unix};

use enums::{enums as enum_mod, option, result};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // structs::run();
    //enum_mod::run();
    //option::run();      // Chiama enums/option.rs
    let _ = result::run(); // Chiama enums/result.rs
    //result::run();
    //generics::run();
    //traits::run();

    let now = std::time::SystemTime::now();
    let elapsed = now.elapsed()?;
    println!("Time elapsed: {} ms", elapsed.as_millis());

    let unix_time = now.duration_since(std::time::UNIX_EPOCH)?.as_secs();
    println!("Unix time: {}", unix_time);

    let duration_since_epoch = std::time::Duration::from_secs(unix_time);
    println!("Duration since epoch: {}", duration_since_epoch.as_secs());

    // Simulazione di un un errore
    if unix_time % 2 == 0 {
        return Err("Errore: tempo unix è pari".into());
    }

    Ok(())
}
