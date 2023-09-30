#![no_std]

extern crate alloc;

use alloc::string::String;
use mx_plug_bridge::Void;
use mx_plugin_demo_shared::{Test1Args, Test1Result};

pub fn test1(a: i32, b: i32) -> i32 {
    let result: Test1Result = mx_plug_bridge::call_plugin_fn("demo", "test_1", &Test1Args { a, b });
    return result.sum;
}

pub fn test2(a: u8) -> u8 {
    return mx_plug_bridge::call_plugin_fn("demo", "test_2", &a);
}

pub fn test3() -> String {
    return mx_plug_bridge::call_plugin_fn("demo", "test_3", &Void::new());
}
