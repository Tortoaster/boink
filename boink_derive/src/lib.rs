use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{
    parse_macro_input, parse_quote, spanned::Spanned, Data, DeriveInput, Fields, GenericParam,
    Generics, Index,
};

#[proc_macro_derive(Lerp)]
pub fn impl_interpolate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let construct = construct_self(&input.data);

    let expanded = quote! {
        impl #impl_generics boink::Lerp for #name #ty_generics #where_clause {
            fn lerp(&self, to: &Self, at: f64) -> Self {
                #construct
            }
        }
    };

    TokenStream::from(expanded)
}

fn construct_self(data: &Data) -> quote::__private::TokenStream {
    match data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                let recurse = fields.named.iter().map(|field| {
                    let name = &field.ident;
                    quote_spanned!(field.span()=> #name: self.#name.lerp(&to.#name, at))
                });
                quote!(Self { #(#recurse,)* })
            }
            Fields::Unnamed(fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(index, field)| {
                    let index = Index::from(index);
                    quote_spanned!(field.span()=> self.#index.lerp(&to.#index, at))
                });
                quote!(Self(#(#recurse),*))
            }
            Fields::Unit => quote!(Self),
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(type_param) = param {
            type_param.bounds.push(parse_quote!(boink::Lerp));
        }
    }

    generics
}
