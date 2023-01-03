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
        let where_clause = &generics.where_clause.as_ref();

        match where_clause {
            Some(clause) => quote! {
                impl #generics ecs::component::Component for #name #generics
                #clause
                {}
            }
            .into(),
            None => quote! {
                impl #generics ecs::component::Component for #name #generics {}
            }
            .into(),
        }
    }
}
