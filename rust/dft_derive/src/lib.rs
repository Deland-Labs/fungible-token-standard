mod func;

extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn standard_basic(_: TokenStream) -> TokenStream {
    func::basic_standard()
}