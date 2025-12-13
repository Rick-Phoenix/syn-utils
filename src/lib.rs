#[macro_use]
mod macros;

mod expr_trait;
use std::{fmt::Display, str::FromStr};

pub use expr_trait::*;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::{
  parse::Parse, punctuated::Punctuated, Expr, ExprCall, ExprClosure, Ident, Lit, LitInt, LitStr,
  Path, Token,
};

pub fn new_ident(name: &str) -> Ident {
  Ident::new(name, Span::call_site())
}

#[derive(Default, Clone, Debug)]
pub struct ControlFlow {
  pub dummy: Option<TokenStream2>,
}

impl ControlFlow {
  pub fn new() -> Self {
    Self { dummy: None }
  }

  pub fn with_custom_dummy(dummy: &TokenStream2) -> Self {
    Self {
      dummy: Some(dummy.to_token_stream()),
    }
  }
}

pub trait MacroResult: Sized {
  type Output;

  fn unwrap_or_dummy(self, dummy: TokenStream2) -> Result<Self::Output, TokenStream2>;

  fn unwrap_or_unimplemented(self) -> Result<Self::Output, TokenStream2> {
    self.unwrap_or_dummy(quote! { unimplemented!() })
  }
}

impl<T> MacroResult for syn::Result<T> {
  type Output = T;

  fn unwrap_or_dummy(self, dummy: TokenStream2) -> Result<Self::Output, TokenStream2> {
    match self {
      Ok(o) => Ok(o),
      Err(e) => {
        let error = e.into_compile_error();

        Err(quote! {
          #error #dummy
        })
      }
    }
  }
}

#[derive(Debug, Clone)]
pub enum CallOrClosure {
  Call(ExprCall),
  Closure(ExprClosure),
}

impl ToTokens for CallOrClosure {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let output = match self {
      CallOrClosure::Call(call) => call.to_token_stream(),
      CallOrClosure::Closure(expr_closure) => expr_closure.to_token_stream(),
    };

    tokens.extend(output);
  }
}

pub struct PunctuatedItems<T: Parse> {
  pub inner: Vec<T>,
}

impl<T: Parse> Parse for PunctuatedItems<T> {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let inner = Punctuated::<T, Token![,]>::parse_terminated(input)?
      .into_iter()
      .collect();

    Ok(Self { inner })
  }
}

pub struct StringList {
  pub list: Vec<String>,
}

impl Parse for StringList {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let items = Punctuated::<LitStr, Token![,]>::parse_terminated(input)?;

    let list: Vec<String> = items
      .into_iter()
      .map(|lit_str| lit_str.value())
      .collect();

    Ok(Self { list })
  }
}

pub struct NumList {
  pub list: Vec<i32>,
}

impl Parse for NumList {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let items = Punctuated::<LitInt, Token![,]>::parse_terminated(input)?;

    let mut list: Vec<i32> = Vec::new();

    for item in items {
      list.push(item.base10_parse()?);
    }

    Ok(Self { list })
  }
}
