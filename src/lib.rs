#[macro_use]
mod macros;

mod expr;
mod generic_args;
mod parsers;
mod path;
mod path_segment;
mod type_ext;
mod type_parser;
use std::{fmt::Display, str::FromStr};

pub use parsers::*;

mod control_flow;
pub use control_flow::*;
mod attributes;
pub use attributes::*;
mod field;
pub use field::*;
mod enum_variant;
pub use enum_variant::*;
pub use expr::*;
pub use generic_args::*;
pub use path::*;
pub use path_segment::*;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::{
  parse::{Parse, ParseStream},
  parse_quote,
  punctuated::Punctuated,
  spanned::Spanned,
  token, Attribute, Expr, ExprCall, ExprClosure, Field, Fields, GenericArgument, Ident, Lifetime,
  Lit, LitInt, LitStr, Meta, Path, PathArguments, PathSegment, Token, Type, TypeArray, TypePath,
  Variant,
};
pub use type_ext::*;
pub use type_parser::*;

pub fn new_ident(name: &str) -> Ident {
  Ident::new(name, Span::call_site())
}
