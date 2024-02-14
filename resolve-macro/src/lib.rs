use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use darling::FromDeriveInput;
use resolve_core::RefType;

#[derive(FromDeriveInput)]
#[darling(attributes(resolve))]
struct Opts {
    reference_type: RefType,
}

#[proc_macro_derive(Resolvable, attributes(resolve))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let Opts { reference_type } = Opts::from_derive_input(&input).expect("Bad options");
    let DeriveInput { ident, .. } = input;

    let output = quote! {
        impl resolve_core::Resolvable for #ident {
            fn resolve(root: &impl resolve_core::ResolveRoot, path: &str) -> Result<Self, resolve_core::ResolveError> {
                root.resolve(#reference_type, path)
            }
        }
    };

    output.into()
}