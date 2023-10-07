use proc_macro::{TokenStream, TokenTree};
use proc_macro2::Ident;
use quote::ToTokens;
use syn::{parse_macro_input, Data, DeriveInput};
extern crate proc_macro;

#[proc_macro_attribute]
pub fn from_num(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let attrs = attrs
        .into_iter()
        .filter(|v| match v {
            TokenTree::Ident(_) => true,
            _ => false,
        })
        .collect::<Vec<_>>();
    let mut item_c = item.clone();
    let input = parse_macro_input!(item as DeriveInput);
    match input.data {
        Data::Enum(data) => {
            let mut arms = format!("");
            let mut count = 0;
            for var in data.variants.iter() {
                if let Some((_, expr)) = var.discriminant.clone() {
                    count = parse_with_prefix(&expr.into_token_stream().to_string());
                };
                arms += &format! {
                    "{} => Self::{},",
                    count,
                    var.ident.to_string()
                };
                count += 1;
            }
            let code = num_traits(&input.ident, &arms, &attrs);
            item_c.extend(code.parse::<TokenStream>().unwrap());
            item_c
        }
        _ => unimplemented!(),
    }
}

fn num_traits(name: &Ident, arms: &str, nums: &[TokenTree]) -> String {
    let mut code = String::new();
    for token in nums.iter() {
        code += &format! {
            r#"impl From<{2}> for {0} {{ fn from(value:{2}) -> Self {{ match value {{ {1}_ => panic!("Failed convertion from {{}}",value) }} }} }}"#,
            name,
            arms,
            token.to_string(),
        };
    }
    code
}

fn parse_with_prefix(s: &str) -> usize {
    let radix = match s {
        s if s.starts_with("0b") => 2,
        s if s.starts_with("0o") => 8,
        s if s.starts_with("0x") => 16,
        _ => {
            return usize::from_str_radix(s, 10).unwrap();
        }
    };
    usize::from_str_radix(&s[2..], radix).unwrap()
}
