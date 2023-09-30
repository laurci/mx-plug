#![no_std]

extern crate alloc;

pub mod serialization;

pub use mx_plug_meta as meta;
pub use mx_plug_meta::{arg, result};

#[derive(Debug, Clone, serialization::__serde::Serialize, serialization::__serde::Deserialize)]
#[serde(crate = "serialization::__serde")]
pub struct Void {}

impl Void {
    pub fn new() -> Self {
        Void {}
    }
}
