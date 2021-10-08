mod func;

extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn standard_basic(_: TokenStream) -> TokenStream {
    func::basic_standard()
}

#[proc_macro]
pub fn standard_ext_burnable(_: TokenStream) -> TokenStream {
    func::standard_ext_burnable()
}

#[proc_macro]
pub fn standard_ext_mintable(_: TokenStream) -> TokenStream {
    func::standard_ext_mintable()
}
