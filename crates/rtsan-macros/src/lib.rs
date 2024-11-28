extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// Enables the sanitizer in your function.
///
/// # Example
///
/// ```
/// #[rtsan::non_blocking]
/// fn process() {
///     // this works as it is real-time safe
///     let _ = [0.0; 256];
/// }
/// ```
#[proc_macro_attribute]
pub fn non_blocking(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input token stream as a function
    let input = parse_macro_input!(item as ItemFn);

    // Extract the function signature and body
    let attrs = input.attrs; // Attributes, including doc comments
    let vis = input.vis; // Visibility modifier
    let sig = input.sig; // Function signature (includes name, generics, and return type)
    let block = input.block; // Function body

    // Generate the transformed function
    let output = quote! {
        #(#attrs)*
        #vis #sig {
            rtsan::realtime_enter();

            // Wrap the block to potentially handle the return value
            let result = #block;

            rtsan::realtime_exit();

            result
        }
    };

    TokenStream::from(output)
}

/// Marks a function as not real-time safe.
///
/// # Example
///
/// ```
/// #[rtsan::blocking]
/// fn process() {
///     // this will fail although it is real-time safe
///     let _ = [0.0; 256];
/// }
/// ```
#[proc_macro_attribute]
pub fn blocking(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input token stream as a function
    let input = parse_macro_input!(item as ItemFn);

    // Extract components of the function
    let attrs = input.attrs; // Attributes, including doc comments
    let vis = input.vis; // Visibility modifier
    let sig = input.sig; // Function signature (includes name, generics, and return type)
    let block = input.block; // Function body
    let function_name = sig.ident.to_string();

    // Generate the transformed function
    let output = quote! {
        #(#attrs)*
        #vis #sig {
            rtsan::notify_blocking_call(#function_name);
            // Directly execute and return the block
            #block
        }
    };

    TokenStream::from(output)
}

/// Disables the sanitizer for this function.
///
/// # Example
///
/// ```
/// #[rtsan::no_sanitize]
/// fn process() {
///     // this won't fail
///     let _ = vec![0.0; 256];
/// }
/// ```
#[proc_macro_attribute]
pub fn no_sanitize(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input token stream as a function
    let input = parse_macro_input!(item as ItemFn);

    // Extract the function signature and body
    let attrs = input.attrs; // Attributes, including doc comments
    let vis = input.vis; // Visibility modifier
    let sig = input.sig; // Function signature (includes name, generics, and return type)
    let block = input.block; // Function body

    // Generate the transformed function
    let output = quote! {
        #(#attrs)*
        #vis #sig {
            rtsan::disable();

            // Wrap the block to potentially handle the return value
            let result = #block;

            rtsan::enable();

            result
        }
    };

    TokenStream::from(output)
}
