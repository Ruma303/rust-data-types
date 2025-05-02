mod lib;

use crate::collections;
use crate::primitives;
use crate::customs;
use crate::compounds;

fn main() {
    primitives::run();
    customs::run();
    collections::run();
    compounds::run();
}