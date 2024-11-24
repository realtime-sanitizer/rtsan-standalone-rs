extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn non_blocking(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input token stream as a function
    let input = parse_macro_input!(item as ItemFn);

    // Extract the function signature and body
    let sig = input.sig;
    let block = input.block;

    // Generate the transformed function
    let output = quote! {
        #sig {
            unsafe { rtsan_standalone_sys::__rtsan_realtime_enter() };

            // Wrap the block to potentially handle the return value
            let result = #block;

            unsafe { rtsan_standalone_sys::__rtsan_realtime_exit() };

            result
        }
    };

    TokenStream::from(output)
}

#[proc_macro_attribute]
pub fn blocking(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input token stream as a function
    let input = parse_macro_input!(item as ItemFn);

    // Extract the function signature and body
    let sig = input.sig;
    let block = input.block;
    let function_name = sig.ident.to_string();

    // Generate the transformed function
    let output = quote! {
        #sig {
            let c_string = std::ffi::CString::new(#function_name)
                .expect("String contained a null byte, which is not allowed in C strings.");
            unsafe { rtsan_standalone_sys::__rtsan_notify_blocking_call(c_string.as_ptr()) };

            // Directly execute and return the block
            #block
        }
    };

    TokenStream::from(output)
}

#[proc_macro_attribute]
pub fn no_sanitize(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input token stream as a function
    let input = parse_macro_input!(item as ItemFn);

    // Extract the function signature and body
    let sig = input.sig;
    let block = input.block;

    // Generate the transformed function
    let output = quote! {
        #sig {
            unsafe { rtsan_standalone_sys::__rtsan_disable() };

            // Wrap the block to potentially handle the return value
            let result = #block;

            unsafe { rtsan_standalone_sys::__rtsan_enable() };

            result
        }
    };

    TokenStream::from(output)
}
