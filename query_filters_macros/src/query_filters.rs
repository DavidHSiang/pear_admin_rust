use darling::{ast::Data, util::parse_expr, FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Expr};

pub fn process_query_filters(input: DeriveInput) -> TokenStream {
    // println!("{:#?}", input);
    let QueryParamsInfo {
        ident: struct_name,
        data: Data::Struct(data),
    } = QueryParamsInfo::from_derive_input(&input).expect("failed to parse input")
    else {
        panic!("failed to parse input");
    };
    // println!("ident: {:?}", struct_name);
    // println!("{:#?}", data);
    let filters = data.fields.iter().map(|field| {
        let field_ident = field.ident.as_ref().expect("field must have an identifier");
        let field_with = field
            .with
            .as_ref()
            .expect("field must have a with attribute");

        quote! {
            if let Some(ref value) = self.#field_ident {
                query = query.filter(#field_with(value.clone()));
            }
        }
    });

    quote! {
        impl query_filters_traits::QueryFiltersTrait for #struct_name {
            fn apply_filters<E: sea_orm::EntityTrait>(&self, mut query: sea_orm::query::Select<E>) -> sea_orm::query::Select<E> {
                #(#filters)*
                query
            }
        }
    }
}

#[derive(Debug, FromDeriveInput)]
struct QueryParamsInfo {
    ident: syn::Ident,
    data: Data<(), FieldsInfo>,
}

#[derive(Debug, FromField)]
#[darling(attributes(filter))]
struct FieldsInfo {
    ident: Option<syn::Ident>,
    #[darling(with = parse_expr::preserve_str_literal, map = Some)]
    with: Option<Expr>,
}
