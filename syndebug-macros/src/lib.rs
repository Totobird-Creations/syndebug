use proc_macro::TokenStream;
use proc_macro2::{
    Span as Span2,
    TokenStream as TokenStream2
};
use syn::{
    parse_macro_input,
    DeriveInput,
    Data,
    DataStruct,
    DataEnum,
    DataUnion,
    Fields,
    FieldsNamed,
    FieldsUnnamed,
    Field,
    Variant,
    Ident,
    Type
};
use quote::{
    quote,
    quote_spanned
};
use proc_macro_crate::{
    crate_name,
    FoundCrate
};

#[proc_macro_derive(SynDebug)]
pub fn syn_debug(item : TokenStream) -> TokenStream {
    let DeriveInput { ident : item_ident, generics, data, .. } = parse_macro_input!(item as DeriveInput);

    let found_crate = match (crate_name("syndebug")) {
        Ok(FoundCrate::Itself)     => quote!{ crate },
        Ok(FoundCrate::Name(crate_name)) => {
            let crate_ident = Ident::new(&crate_name, Span2::call_site());
            quote!( #crate_ident )
        },
        Err(_) => quote!{ ::syndebug }
    };
    let item_str = item_ident.to_string();
    let (impl_generics, type_generics, where_clause,) = generics.split_for_impl();

    let inner = match (data) {

        Data::Struct(DataStruct { fields, .. }) => {
            match (fields) {

                Fields::Named(FieldsNamed { named, .. }) => quote_fields_named(&found_crate,
                    named.into_iter().map(|Field { ident : field_ident, ty, .. }| {
                        let field_ident = field_ident.unwrap();
                        let access      = quote!{ &self.#field_ident };
                        (field_ident, ty, access,)
                    })
                ),

                Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => quote_fields_unnamed(&found_crate,
                    unnamed.into_iter().enumerate().map(|(i, Field { ty, .. },)| {
                        (ty, quote!{ self.#i },)
                    })
                ),

                Fields::Unit => quote!{ }

            }
        },

        Data::Enum(DataEnum { variants, .. }) => {
            if (variants.is_empty()) {
                quote!{ unsafe { ::core::hint::unreachable_unchecked(); } }
            } else {
                let variants = variants.into_iter().map(|Variant { ident : variant_ident, fields, .. }| {
                    let variant_str = variant_ident.to_string();
                    match (fields) {

                        Fields::Named(FieldsNamed { named, .. }) => {
                            let field_idents = named.iter().map(|Field { ident : field_ident, .. }| field_ident);
                            let fields       = quote_fields_named(&found_crate,
                                named.iter().map(|Field { ident : field_ident, ty, .. }| {
                                    let field_ident = field_ident.as_ref().unwrap();
                                    let access      = quote!{ #field_ident };
                                    (field_ident.clone(), ty.clone(), access,)
                                })
                            );
                            quote!{ Self::#variant_ident { #( #field_idents , )* } => {
                                write!(f, concat!("::", #variant_str))?;
                                #fields
                            } }
                        },

                        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                            let field_idents = (0..unnamed.len()).map(|i| Ident::new(&format!("f{i}"), Span2::call_site())).collect::<Vec<_>>();
                            let fields       = quote_fields_unnamed(&found_crate,
                                unnamed.iter().zip(field_idents.clone()).map(|(Field { ty, .. }, field_ident,)| {
                                    (ty.clone(), quote!{ #field_ident })
                                })
                            );
                            quote!{ Self::#variant_ident ( #( #field_idents , )* ) => {
                                write!(f, concat!("::", #variant_str))?;
                                #fields
                            } }
                        },

                        Fields::Unit => quote!{ Self::#variant_ident => {
                            write!(f, concat!("::", #variant_str))?;
                        } }

                    }
                });
                quote!{ match (self) { #( #variants , )* } }
            }
        },

        Data::Union(DataUnion { .. }) => quote_spanned!(Span2::call_site() => {
            compile_error!("SynDebug can not be implemented for unions");
        } )

    };

    quote!{
        impl #impl_generics #found_crate::SynDebug for #item_ident #type_generics
        #where_clause
        {
            fn fmt(&self, f : &mut ::core::fmt::Formatter<'_>, const_like : bool) -> ::core::fmt::Result {
                write!(f, #item_str)?;
                #inner
                Ok(())
            }
        }
    }.into()
}


fn quote_fields_named(found_crate : &TokenStream2, named : impl IntoIterator<Item = (Ident, Type, TokenStream2,)>) -> TokenStream2 {
    let fields = named.into_iter().map(|(field_ident, ty, access,)| {
        let field_str = field_ident.to_string();
        quote!{
            write!(f, concat!(#field_str, " : "))?;
            <#ty as #found_crate::SynDebug>::fmt(#access, f, const_like)?;
            write!(f, ", ")?;
        }
    });
    quote!{
        write!(f, " {{ ")?;
        #( #fields )*
        write!(f, "}}")?;
    }
}

fn quote_fields_unnamed(found_crate : &TokenStream2, unnamed : impl IntoIterator<Item = (Type, TokenStream2)>) -> TokenStream2 {
    let fields = unnamed.into_iter().map(|(ty, access,)| quote!{
        <#ty as #found_crate::SynDebug>::fmt(#access, f, const_like)?;
        write!(f, ", ")?;
    });
    quote!{
        write!(f, " ( ")?;
        #( #fields )*
        write!(f, ")")?;
    }
}
