use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(EntityComponent)]
pub fn derive_ecs_component(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = input.ident;
    let generics = input.generics;

    if generics.params.is_empty() {
        quote! {
            impl ecs::component::Component for #name {}
        }
        .into()
    } else {
        let where_clause = match generics.where_clause.as_ref() {
            Some(clause) => quote! {
                #clause
            },
            None => quote!(),
        };

        panic!(
            "{}",
            quote! {
                impl #generics ecs::component::Component for #name #generics
                #where_clause
                {}
            }
        )
    }
}
