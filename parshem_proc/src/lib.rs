#![allow(unused_parens)]


use proc_macro::TokenStream;

use parshem_shared;


#[proc_macro]
pub fn generate_rule(stream : TokenStream) -> TokenStream {
    let tree = parshem_shared::generate_rule(stream.to_string());
    return tree.gen_snippet().parse().unwrap();
}
