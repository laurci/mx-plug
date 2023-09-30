use alloc::string::{String, ToString};
use serde::{de::DeserializeOwned, Serialize};

pub use serde as __serde;

pub fn serialize<T>(value: &T) -> alloc::vec::Vec<u8>
where
    T: Serialize,
{
    postcard::to_allocvec(value).unwrap()
}

pub fn deserialize<T>(bytes: &[u8]) -> T
where
    T: DeserializeOwned,
{
    postcard::from_bytes(bytes).unwrap()
}

pub fn encode_to_ptr<T>(value: &T) -> *const u8
where
    T: Serialize,
{
    let mut bytes = serialize(value);
    bytes.shrink_to_fit();

    let mut len_bytes = (bytes.len() as u64).to_le_bytes().to_vec();
    len_bytes.shrink_to_fit();

    let mut bytes = [len_bytes, bytes].concat();
    bytes.shrink_to_fit();

    let ptr = bytes.as_ptr();
    core::mem::forget(bytes);

    ptr
}

pub fn decode_from_ptr<T>(ptr: *const u8) -> T
where
    T: DeserializeOwned,
{
    let len_bytes = unsafe { core::slice::from_raw_parts(ptr, 8) };
    let len_bytes = len_bytes.to_vec();
    let len_bytes = u64::from_le_bytes(len_bytes.try_into().unwrap()) as usize;

    let bytes = unsafe { core::slice::from_raw_parts(ptr as *mut u8, 8 + len_bytes) };
    let bytes = bytes[8..].to_vec();

    deserialize(&bytes)
}

#[allow(unknown_lints)]
#[allow(forgetting_copy_types)]
pub fn string_to_ptr(s: &str) -> *const u8 {
    let ptr = s.as_ptr();
    core::mem::forget(ptr);

    ptr
}

pub fn ptr_to_string(ptr: *const u8) -> String {
    let c_str = unsafe { core::ffi::CStr::from_ptr(ptr as *const i8) };
    let string = c_str.to_str().unwrap().to_string();
    string
}
