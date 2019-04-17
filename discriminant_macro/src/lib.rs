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

    let ident = &input.ident;

    // The name of our discriminant enum
    let disc_ident = Ident::new(
            &format!("{}Discriminant", ident),
            Span::call_site());

    let en = match input.data {
        Data::Enum(en) => en,
        _ => panic!("Failed to derive Discriminant for {}. {0} is not an Enum.", &input.ident)
    };

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

    let disc_ident_rep = repeat(&disc_ident);
    let ident_rep = repeat(ident);

    // Build the output, possibly using quasi-quotation
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
    println!("{}", expanded);
    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
