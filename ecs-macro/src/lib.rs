use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Component)]
pub fn derive_ecs_component(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = input.ident;

    quote! {
        impl Container for #name {
            fn get_attr<T: std::any::Any>(&self) -> Vec<&T> {
                let mut attrs = Vec::<&T>::new();
                self.attrs
                    .iter()
                    .map(|attr| attr.downcast_ref::<T>())
                    .flatten()
                    .for_each(|attr| attrs.push(attr));

                attrs
            }

            fn get<T: std::any::Any>(&self) -> Option<&T> {
                self.attrs
                    .iter()
                    .map(|attr| attr.downcast_ref::<T>())
                    .flatten()
                    .next()
            }

            fn with_attr<T: Any>(&mut self, attr: T) {
                self.attrs.push(Box::new(attr));
            }

            fn with<T: Any>(mut self, attr: T) -> Self {
                self.attrs.push(Box::new(attr));
                self
            }
        }
    }
    .into()
}
