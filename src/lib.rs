extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Item, parse_macro_input};

pub fn build() {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("mmx") {
            println!("cargo::rustc-check-cfg=cfg(__MMX__)");
            println!("cargo::rustc-cfg=__MMX__");
        }
    }
}

#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn __MMX__(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Item);

    let expanded = quote! {
        #[cfg(__MMX__)]
        #input
    };

    expanded.into()
}
