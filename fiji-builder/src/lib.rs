use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use proc_macro_error::abort_call_site;
use quote::quote;
use syn::{
    ext::IdentExt, parse_macro_input, DataStruct, DeriveInput, PathArguments::AngleBracketed, Type,
};

fn is_option(ty: &Type) -> bool {
    if let syn::Type::Path(tp) = ty {
        if let Some(segment) = tp.path.segments.first() {
            if let AngleBracketed(args) = &segment.arguments {
                if !args.args.is_empty() {
                    if segment.ident == "Option" {
                        return true;
                    }
                }
            }
        }
    }
    false
}

// TODO: Add required option to buildable
#[proc_macro_derive(Builder, attributes(buildable))]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        data,
        generics,
        ..
    } = parse_macro_input!(input);
    let (impl_generics, type_generics, where_class) = generics.split_for_impl();
    let builder_struct_ident = Ident::new(&format!("{}Builder", ident.unraw()), Span::call_site());

    if let syn::Data::Struct(DataStruct { ref fields, .. }) = data {
        let filtered_fields = fields
            .iter()
            .filter(|f| f.attrs.iter().any(|attr| attr.path.is_ident("buildable")));

        let builder_fields = filtered_fields.clone().map(|f| {
            let name = f.ident.clone().unwrap();
            let ty = f.ty.clone();
            quote!(
                pub #name: Option<#ty>,
            )
        });

        let builder_init = filtered_fields.clone().map(|f| {
            let name = f.ident.clone().unwrap();
            quote!(
                #name: None,
            )
        });

        let builder_methods = filtered_fields.clone().map(|f| {
            let name = f.ident.clone().unwrap();
            let ty = f.ty.clone();
            quote!(
                pub fn #name(mut self, #name: #ty) -> Self {
                    self.#name = Some(#name);
                    self
                }
            )
        });

        let build_fields = fields.iter().filter_map(|f| {
            let name = f.ident.clone().unwrap();
            let ty = f.ty.clone();
            if f.attrs.iter().any(|attr| attr.path.is_ident("buildable")) {
                // Check for other generic possibilities
                if is_option(&ty) {
                    return Some(quote!(
                        #name: self.#name.unwrap_or(None),
                    ));
                }
                // TODO: Look for required parameter
                return Some(quote!(
                    #name: self.#name.unwrap_or(#ty::default()),
                ));
            }
            None
        });

        quote!(
            pub struct #impl_generics #builder_struct_ident #type_generics #where_class {
                #(#builder_fields)*
            }

            impl #impl_generics #builder_struct_ident #type_generics #where_class {
                pub fn new() -> Self {
                    Self {
                        #(#builder_init)*
                    }
                }

                #(#builder_methods)*

                pub fn build(self) -> #ident {
                    #ident {
                        #(#build_fields)*
                        ..#ident::default()
                    }
                }
            }

        )
        .into()
    } else {
        abort_call_site!("Builders can only be created for structs! Not enums and unions")
    }
}
