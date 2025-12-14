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
