extern crate proc_macro;
use proc_macro::TokenStream;
//use syn::{parse_macro_input, Block};
//use syn::Expr::Block; // not used 
//use quote::quote;

use laststraw_core::testwindow;

/* 
#[proc_macro] 
pub fn apploop(input: TokenStream) -> TokenStream {
    let block: Block = parse_macro_input!(input as Block);
    let expanded = quote! {
        while true {
            #block
        }
    };
    TokenStream::from(expanded)
} 
*/

/* 
#[proc_macro_attribute]
pub fn asx(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input token stream as a Rust block
    let block: Block = parse_macro_input!(item as Block);

    // Generate the transformed token stream
    let expanded = quote! {
        while true {
            #block
        }
    };

    // Return the expanded token stream
    TokenStream::from(expanded)
}
*/

use quote::quote;
use syn::{parse_macro_input, Block};

/* 
#[proc_macro_attribute]
pub fn asx(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let block: Block = parse_macro_input!(item as Block);

    let expanded = quote! {
        fn _generated_block_wrapper() {
            loop {
                #block
            }
        }
        _generated_block_wrapper();
    };

    TokenStream::from(expanded)
}
*/

use syn::ItemFn;

/* 
#[proc_macro]
pub fn asx(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let block = input.block;

    let expanded = quote! {
        fn _generated_block_wrapper() {
            loop {
                #block
            }
        }
        _generated_block_wrapper();
    };

    TokenStream::from(expanded)
}
*/

#[proc_macro]
pub fn asx(input: TokenStream) -> TokenStream {
    let block = parse_macro_input!(input as Block);

    let expanded = quote! {
        fn _generated_block_wrapper() {
            loop {
                #block
            }
        }
        _generated_block_wrapper();
    };

    TokenStream::from(expanded)
}