extern crate proc_macro;

use proc_macro::{TokenStream};
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};
use std::iter::repeat;

#[proc_macro_derive(Discriminable)]
pub fn discriminable(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // The name of the original enum
    let ident = &input.ident;

    // The name of our discriminant enum
    let disc_ident = Ident::new(
            &format!("{}Discriminant", ident),
            Span::call_site());

    // Rip out the enum and throw an error if someone tried
    // to derive Discriminable for a non-Enum type.
    let en = match input.data {
        Data::Enum(en) => en,
        _ => panic!("Failed to derive Discriminable for {}. {0} is not an Enum.", &input.ident)
    };

    // Each type of variant requires a different matcher. 
    // We don't care about the internal values of the variant
    // when we map to the discriminant, but we have to find
    // the right way to tell Rust that we don't care.
    let catchalls = en.variants.iter().map(|v| {
        match v.fields {
            Fields::Named(..) => quote!({..}),
            Fields::Unnamed(..) => quote!((..)),
            Fields::Unit => quote!()
        }
    });

    // quote won't let us reuse the same iterable inside a repeat,
    // so we just create 2. I know it's gross.
    let variants: &Vec<Ident> = &en.variants.iter().map(|v| v.ident.clone()).collect();
    let variants_2: &Vec<Ident> = &en.variants.iter().map(|v| v.ident.clone()).collect();


    // Another quirk of quote is that inside repititions,
    // all variables must repeatable.
    // So here you go quote, eat your heart out.
    let disc_ident_rep = repeat(&disc_ident);
    let ident_rep = repeat(ident);

    let expanded = quote! {
        #[derive(Copy, Debug, Clone, PartialEq)]
        pub enum #disc_ident {
            #(#[allow(dead_code)] #variants),*
        }

        impl Discriminable for #ident {
            type Discriminant = #disc_ident;

            fn discriminate(&self) -> Self::Discriminant {
                match self {
                    #(#ident_rep::#variants#catchalls => #disc_ident_rep::#variants_2),*
                }
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
