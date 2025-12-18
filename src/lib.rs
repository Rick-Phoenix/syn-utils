#[macro_use]
mod macros;

mod field_or_variant;
pub use field_or_variant::*;
mod expr;
mod generic_args;
mod parsers;
mod path;
mod path_segment;
mod type_ext;
mod type_parser;
use std::{fmt::Display, rc::Rc, str::FromStr};

pub use parsers::*;

mod control_flow;
pub use control_flow::*;
mod attributes;
pub use attributes::*;
mod field;
pub use field::*;
mod enum_variant;
use std::ops::{Range, RangeFrom};

pub use enum_variant::*;
pub use expr::*;
pub use generic_args::*;
pub use path::*;
pub use path_segment::*;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, quote_spanned, ToTokens};
use syn::{
  parse::{Parse, ParseStream},
  parse_quote, parse_quote_spanned,
  punctuated::Punctuated,
  spanned::Spanned,
  token, Attribute, Expr, ExprCall, ExprClosure, ExprRange, Field, Fields, GenericArgument, Ident,
  Lifetime, Lit, LitInt, LitStr, Meta, Path, PathArguments, PathSegment, Token, Type, TypeArray,
  TypePath, Variant,
};
pub use type_ext::*;
pub use type_parser::*;

pub fn new_ident(name: &str) -> Ident {
  Ident::new(name, Span::call_site())
}
