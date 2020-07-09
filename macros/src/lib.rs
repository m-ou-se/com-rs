use com_macros_support::co_class::expand_co_class;
use com_macros_support::com_interface::{expand_com_interfaces, expand_derive};
use com_macros_support::Interfaces;

extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{AttributeArgs, ItemStruct};

// All the Macro exports declared here. Delegates to respective crate for expansion.
#[proc_macro]
pub fn com_interface(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as Interfaces);

    expand_com_interfaces(input).into()
}

#[proc_macro_derive(VTable)]
pub fn derive_vtable(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as ItemStruct);
    expand_derive(input).into()
}

// Macro entry points.
#[proc_macro_attribute]
pub fn co_class(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as ItemStruct);
    let attr_args = syn::parse_macro_input!(attr as AttributeArgs);
    expand_co_class(&input, &attr_args).into()
}
