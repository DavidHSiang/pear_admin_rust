mod query_filters;

use proc_macro::TokenStream;
use query_filters::process_query_filters;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(QueryFilters, attributes(filter))]
pub fn query_filters_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    process_query_filters(input).into()
}
