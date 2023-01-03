use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(EntityComponent)]
pub fn derive_ecs_component(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = input.ident;

    quote! {
        impl ecs::component::Component for #name {}
    }
    .into()
}
