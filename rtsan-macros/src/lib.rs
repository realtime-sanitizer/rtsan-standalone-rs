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
            let result = (|| #block)();

            unsafe { rtsan_standalone_sys::__rtsan_realtime_exit() };

            result
        }
    };

    TokenStream::from(output)
}
