use proc_macro::TokenStream;
use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::Comma,
    Result,
};

#[proc_macro_attribute]
pub fn arg(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as syn::Item);

    let _name = match &item {
        syn::Item::Struct(item) => &item.ident,
        syn::Item::Enum(item) => &item.ident,
        _ => panic!("Only structs and enums can be args"),
    };

    let item = quote::quote! {
        #[derive(mx_plug_shared::serialization::__serde::Serialize, mx_plug_shared::serialization::__serde::Deserialize, Debug, Clone)]
        #[serde(crate = "mx_plug_shared::serialization::__serde")]
        #item
    };

    item.into()
}

#[proc_macro_attribute]
pub fn result(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as syn::Item);

    let _name = match &item {
        syn::Item::Struct(item) => &item.ident,
        syn::Item::Enum(item) => &item.ident,
        _ => panic!("Only structs and enums can be results"),
    };

    let item = quote::quote! {
        #[derive(mx_plug_shared::serialization::__serde::Serialize, mx_plug_shared::serialization::__serde::Deserialize, Debug, Clone)]
        #[serde(crate = "mx_plug_shared::serialization::__serde")]
        #item
    };

    item.into()
}

struct PluginMacroInput {
    name: syn::LitStr,
    fns: Vec<syn::Ident>,
}

impl Parse for PluginMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        /* Example
         * plugin! {
         *    name = "demo",
         *    fns = [a, b, c]
         * }
         */

        let _name_ident: syn::Ident = input.parse()?;
        let _eq: syn::Token![=] = input.parse()?;
        let name_lit: syn::LitStr = input.parse()?;

        let _comma: syn::Token![,] = input.parse()?;

        let _fns_ident: syn::Ident = input.parse()?;
        let _eq: syn::Token![=] = input.parse()?;

        let content;
        bracketed!(content in input);
        let fns = Punctuated::<syn::Ident, Comma>::parse_terminated(&content)?;

        Ok(PluginMacroInput {
            name: name_lit,
            fns: fns.into_iter().collect(),
        })
    }
}

#[proc_macro]
pub fn plugin(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as PluginMacroInput);

    let name_str = item.name.token().to_string();
    let fn_names_str = item
        .fns
        .iter()
        .map(|func| {
            let func_str = func.to_string();
            let func_str = format!("\"{}\"", func_str);
            func_str
        })
        .collect::<Vec<_>>()
        .join(", ");

    let init_data = format!("{{ \"Name\": {name_str}, \"Methods\": [{fn_names_str}] }}");

    let match_arms = item
        .fns
        .iter()
        .map(|func| {
            let func_str = func.to_string();

            quote::quote! {
                #func_str => {
                    let args = mx_plug_core::shared::serialization::decode_from_ptr(args);
                    let result = #func(&call_ctx, args);
                    let result_ptr = mx_plug_core::shared::serialization::encode_to_ptr(&result);

                    result_ptr
                }
            }
        })
        .collect::<Vec<_>>();

    let item = quote::quote! {
        #[no_mangle]
        pub extern "C" fn mx_plug_init() -> *const std::ffi::c_char {
            let c_str = std::ffi::CString::new(#init_data).unwrap();

            let ptr = c_str.as_ptr();
            std::mem::forget(c_str);

            ptr
        }

        #[no_mangle]
        pub extern "C" fn mx_plug_call(call_ctx: *const u8, method_name: *const u8, args: *const u8) -> *const u8 {
            let method_name = mx_plug_core::shared::serialization::ptr_to_string(method_name);
            let call_ctx = mx_plug_core::shared::serialization::ptr_to_string(call_ctx);
            let call_ctx = mx_plug_core::context::decode_plugin_context_from_str(&call_ctx);

            match method_name.as_str() {
                #(#match_arms),*
                _ => {
                    panic!("Unknown method name: {}", method_name);
                }
            }
        }
    };

    item.into()
}
