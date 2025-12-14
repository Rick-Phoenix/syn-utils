use syn::token::Comma;

use crate::*;

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

pub type PathsList = PunctuatedItems<Path>;
pub type IdentsList = PunctuatedItems<Path>;

impl<T: Parse> Parse for PunctuatedItems<T> {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let mut inner = Vec::new();

    while !input.is_empty() {
      inner.push(input.parse()?);

      if input.is_empty() {
        break;
      }
      let _comma: Comma = input.parse()?;
    }

    Ok(Self { inner })
  }
}

pub struct StringList {
  pub list: Vec<String>,
}

impl Parse for StringList {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let mut list: Vec<String> = Vec::new();

    while !input.is_empty() {
      list.push(input.parse::<LitStr>()?.value());

      if input.is_empty() {
        break;
      }
      let _comma: Comma = input.parse()?;
    }

    Ok(Self { list })
  }
}

pub struct NumList {
  pub list: Vec<i32>,
}

impl Parse for NumList {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let mut list: Vec<i32> = Vec::new();

    while !input.is_empty() {
      list.push(input.parse::<LitInt>()?.base10_parse()?);

      if input.is_empty() {
        break;
      }
      let _comma: Comma = input.parse()?;
    }

    Ok(Self { list })
  }
}
