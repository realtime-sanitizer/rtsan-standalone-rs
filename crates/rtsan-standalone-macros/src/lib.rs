extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// Enter real-time context in your function.
/// When in a real-time context, RTSan interceptors will error if realtime
/// violations are detected. Calls to this method are injected at the code
/// generation stage when RTSan is enabled.
///
/// # Example
///
/// ```
/// #[nonblocking]
/// fn process() {
///     let _ = vec![0.0; 256]; // oops
/// }
/// ```
#[proc_macro_attribute]
pub fn nonblocking(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let attrs = input.attrs;
    let vis = input.vis;
    let sig = input.sig;
    let block = input.block;

    let output = quote! {
        #(#attrs)*
        #vis #sig {
            let __guard = rtsan_standalone::ScopedSanitizeRealtime::default();
            #block
        }
    };
    TokenStream::from(output)
}

/// Allows the user to specify a function as not-real-time-safe.
///
/// # Example
///
/// ```
/// #[blocking]
/// fn my_blocking_function() {}
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
    let mut function_name = sig.ident.to_string();
    function_name.push('\0');

    // Generate the transformed function
    let output = quote! {
        #(#attrs)*
        #vis #sig {
            rtsan_standalone::notify_blocking_call(#function_name);
            // Directly execute and return the block
            #block
        }
    };

    TokenStream::from(output)
}

/// Disable all RTSan error reporting in an otherwise real-time context.
///
/// # Example
///
/// ```
/// #[no_sanitize_realtime]
/// fn process() {
///     let _ = vec![0.0; 256]; // ok!
/// }
/// ```
#[proc_macro_attribute]
pub fn no_sanitize_realtime(_attr: TokenStream, item: TokenStream) -> TokenStream {
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
            let __guard = rtsan_standalone::ScopedDisabler::default();
            #block
        }
    };
    TokenStream::from(output)
}
