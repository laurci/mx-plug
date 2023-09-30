#![no_std]

extern crate alloc;

pub use mx_plug_shared as shared;
use shared::serialization;
pub use shared::Void;

extern "C" {
    fn mxPlugCall(
        plugin_name: *const u8,
        plugin_name_len: i32,
        method_name: *const u8,
        method_name_len: i32,
        args: *const u8,
    ) -> *const u8;
}

pub fn call_plugin_fn<TInput, TOutput>(plugin_name: &str, fn_name: &str, args: &TInput) -> TOutput
where
    TInput: serialization::__serde::Serialize,
    TOutput: serialization::__serde::de::DeserializeOwned,
{
    let data = serialization::encode_to_ptr(&args);

    let result = unsafe {
        mxPlugCall(
            serialization::string_to_ptr(plugin_name),
            plugin_name.len() as i32,
            serialization::string_to_ptr(fn_name),
            fn_name.len() as i32,
            data,
        )
    };

    return serialization::decode_from_ptr(result);
}
