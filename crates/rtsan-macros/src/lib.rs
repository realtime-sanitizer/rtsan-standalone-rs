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
/// #[rtsan::nonblocking]
/// fn process() {
///     let _ = vec![0.0; 256]; // oops
/// }
/// ```
#[proc_macro_attribute]
pub fn nonblocking(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Check for a feature flag at compile time
    if cfg!(feature = "enable") {
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
    } else {
        // If the feature is not enabled, return the original function unchanged
        item
    }
}

/// Allows the user to specify a function as not-real-time-safe.
///
/// # Example
///
/// ```
/// #[rtsan::blocking]
/// fn my_blocking_function() {}
/// ```
#[proc_macro_attribute]
pub fn blocking(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Check for a feature flag at compile time
    if cfg!(feature = "enable") {
        // Parse the input token stream as a function
        let input = parse_macro_input!(item as ItemFn);

        // Extract components of the function
        let attrs = input.attrs; // Attributes, including doc comments
        let vis = input.vis; // Visibility modifier
        let sig = input.sig; // Function signature (includes name, generics, and return type)
        let block = input.block; // Function body
        let mut function_name = sig.ident.to_string();
        function_name.push_str("\0");

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
    } else {
        // If the feature is not enabled, return the original function unchanged
        item
    }
}

/// Disable all RTSan error reporting in an otherwise real-time context.
///
/// # Example
///
/// ```
/// #[rtsan::no_enable]
/// fn process() {
///     let _ = vec![0.0; 256]; // ok!
/// }
/// ```
#[proc_macro_attribute]
pub fn no_sanitize(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Check for a feature flag at compile time
    if cfg!(feature = "enable") {
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
    } else {
        // If the feature is not enabled, return the original function unchanged
        item
    }
}
