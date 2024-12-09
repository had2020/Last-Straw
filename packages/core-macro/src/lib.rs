extern crate proc_macro;
use proc_macro::TokenStream;

use laststraw_core::testwindow;

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