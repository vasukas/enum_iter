use proc_macro::TokenStream;
use quote::quote;
use syn::*;

/// Generates static `iter()` method which returns iterator over all values of an enum, in order of declaration.
#[proc_macro_derive(EnumIter)]
pub fn derive_enum_iter(input: TokenStream) -> TokenStream {
    let item: DeriveInput = parse(input).unwrap();

    let mut values = Vec::new();

    match item.data {
        Data::Enum(item) => {
            for var in item.variants.iter() {
                match &var.fields {
                    Fields::Unit => (),
                    _ => panic!("EnumIter derive: variants with fields are not supported"),
                }
                let ident = &var.ident;
                values.push(quote! { Self::#ident, });
            }
        }
        _ => panic!("EnumIter derive: not used on enum"),
    };

    let enum_name = item.ident;
    quote! {
        impl #enum_name {
            /// Returns iterator over values
            pub fn iter() -> impl Iterator<Item = Self> {
                [#(#values)*].into_iter()
            }
        }
    }
    .into()
}
