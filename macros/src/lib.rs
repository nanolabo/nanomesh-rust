// Proc macro
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[proc_macro_derive(Entity)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let mut s = DefaultHasher::new();
    name.hash(&mut s);
    let id = s.finish();

    let gen = quote! {
        impl Entity for #name {
            fn get_id() -> u64 {
                #id
            }
            fn get_attachement_id(&self) -> Option<EntityId> {
                self.attachement_id
            }
            fn set_attachement_id(&mut self, attachement_id: EntityId) {
                self.attachement_id = Some(attachement_id);
            }
        }
    };
    gen.into()
}