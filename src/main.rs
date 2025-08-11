mod structs;
mod enums;
mod generics;
mod traits;

use enums::{enums as enum_mod, option, result};

fn main() {
    // structs::run();
    //enum_mod::run();
    //option::run();      // Chiama enums/option.rs
    result::run();      // Chiama enums/result.rs
    //result::run();
    //generics::run();
    //traits::run();
}