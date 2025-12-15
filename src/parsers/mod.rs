use syn::token::Comma;

use crate::*;

#[derive(Debug, Clone)]
pub enum CallOrClosure {
  Call(TokenStream2),
  Closure(TokenStream2),
}

impl ToTokens for CallOrClosure {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    match self {
      CallOrClosure::Call(call) => call.to_tokens(tokens),
      CallOrClosure::Closure(expr_closure) => expr_closure.to_tokens(tokens),
    }
  }
}

pub struct PunctuatedItems<T: Parse + ToTokens> {
  pub inner: Vec<T>,
}

pub type PathsList = PunctuatedItems<Path>;
pub type IdentsList = PunctuatedItems<Path>;

impl<T: Parse + ToTokens> Parse for PunctuatedItems<T> {
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

impl<T: Parse + ToTokens> ToTokens for PunctuatedItems<T> {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let list = &self.inner;

    let output = quote! { #(#list),* };

    tokens.extend(output)
  }
}

pub struct StringList {
  pub list: Vec<String>,
}

impl ToTokens for StringList {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let list = &self.list;

    let output = quote! { #(#list),* };

    tokens.extend(output)
  }
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

impl ToTokens for NumList {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let list = self
      .list
      .iter()
      .map(|n| proc_macro2::Literal::i32_unsuffixed(*n));

    let output = quote! { #(#list),* };

    tokens.extend(output)
  }
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
