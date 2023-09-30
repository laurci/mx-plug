#![no_std]

use mx_plug_shared::{arg, result};

#[arg]
pub struct Test1Args {
    pub a: i32,
    pub b: i32,
}

#[result]
pub struct Test1Result {
    pub sum: i32,
}
