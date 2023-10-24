//! Attribute macro,implementing the From trait for your Enum.
//! 
//! ```rust
//! use from_num::from_num;
//! #[derive(Debug,PartialEq)]
//! #[from_num(i8,u64,usize)]
//! enum Planet {
//!     Mercury,
//!     Venus,
//!     Earth,
//!     Mars,
//!     Jupiter = 0b1000,
//!     Saturn,
//!     Uranus = 0xff,
//!     Neptune
//! }
//! pub fn get_from_number() {
//!     assert_eq!(Planet::Jupiter,Planet::from_i8(0b1000 as i8).unwrap());
//!     assert_eq!(Planet::Neptune,Planet::from_u64(256 as u64).unwrap());
//! }
//! ```

use proc_macro::{TokenStream, TokenTree};
use proc_macro2::Ident;
use quote::ToTokens;
use syn::{parse_macro_input, Data, DeriveInput};
extern crate proc_macro;

/// Implements the From trait for your Enum.
///
/// # Examples
/// 
/// ```rust
/// use from_num::from_num;
/// #[derive(Debug,PartialEq)]
/// #[from_num(i8,u64,usize)]
/// enum Planet {
///     Mercury,
///     Venus,
///     Earth,
///     Mars,
///     Jupiter = 0b1000,
///     Saturn,
///     Uranus = 0xff,
///     Neptune
/// }
/// pub fn get_from_number() {
///     assert_eq!(Planet::Jupiter,Planet::from_i8(0b1000 as i8).unwrap());
///     assert_eq!(Planet::Neptune,Planet::from_u64(256 as u64).unwrap());
/// }
/// ```
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
                    "{} => Ok(Self::{}),",
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
            r#"impl {0} {{ pub fn from_{2}(value:{2}) -> anyhow::Result<Self> {{ match value {{ {1}_ => Err(anyhow::anyhow!("[enum {0}] Failed convertion from {{}}",value)) }} }} }}"#,
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
