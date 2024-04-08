extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a represenatation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap(); // will panic if unwrap fails. this is intended as we want procedural macros to return TokenStream rather than Result

    // build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
      impl HelloMacro for #name {
        fn hello_macro() {
          println!("Hello, Macro! my name is {}!", stringify!(#name));
        }
      }
    };
    gen.into()
}
