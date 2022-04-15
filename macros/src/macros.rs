// Proc macro
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::parse::Parser;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[proc_macro_attribute]
pub fn entity(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as syn::DeriveInput);
    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {           
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    fields
                        .named
                        .push(syn::Field::parse_named.parse2(quote! { pub attachement_id: Option<EntityId> }).unwrap());
                }
                syn::Fields::Unnamed(fields) => {
                    panic!("entity macro requires named struct parameters!")
                }
                _ => {
                    panic!("entity macro requires struct to have at least one named parameter! (for now)")
                }
            }              
            
            let mut stream: TokenStream = quote! {
                #ast
            }.into();

            // Combine with Entity trait implementation
            stream.extend(implement_entity_trait(&ast));

            return stream;
        }
        _ => panic!("`add_field` has to be used with structs "),
    }
}

fn implement_entity_trait(ast: &syn::DeriveInput) -> TokenStream {
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
