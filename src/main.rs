mod hashmap;
mod hashset;
mod vector;
mod strings;

use strings::{self as mod_strings, operations};

fn main() {
    mod_strings::run();
    operations::run();
    //vector::run();
    //hashmap::run();
    //hashset::run();
}
