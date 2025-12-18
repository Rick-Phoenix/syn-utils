use syn::{token::Comma, RangeLimits};

use crate::*;

#[derive(Debug, Clone)]
pub struct ClosedRangeList {
  pub list: Vec<Range<i32>>,
}

impl Parse for ClosedRangeList {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let mut ranges: Vec<Range<i32>> = Vec::new();

    while !input.is_empty() {
      let item: Expr = input.parse()?;

      if let Expr::Range(range_expr) = &item {
        let start = if let Some(start_expr) = &range_expr.start {
          start_expr.as_int::<i32>()?
        } else {
          return Err(input.error("Expected a defined start for this range"));
        };

        if let Some(end_expr) = &range_expr.end {
          let mut end = end_expr.as_int::<i32>()?;

          if let RangeLimits::Closed(_) = &range_expr.limits {
            end += 1;
          }

          ranges.push(start..end)
        } else {
          return Err(input.error("Expected a closed range"));
        }
      } else if let Expr::Lit(lit) = &item && let Lit::Int(lit_int) = &lit.lit {
        let num = lit_int.base10_parse::<i32>()?;

        ranges.push(num..num + 1);
      } else {
        return Err(error!(
          item,
          "Expected a range (e.g. `1..5`, `10..=15`) or a single number"
        ));
      }

      if input.is_empty() {
        break;
      }

      let _comma: Comma = input.parse()?;
    }

    ranges.sort_by_key(|range| range.start);

    Ok(Self { list: ranges })
  }
}

#[derive(Debug, Clone)]
pub enum GenericRange {
  Open(RangeFrom<i32>),
  Closed(Range<i32>),
}

#[derive(Debug, Clone)]
pub struct GenericRangeList {
  pub list: Vec<GenericRange>,
}

impl Parse for GenericRangeList {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let mut ranges: Vec<GenericRange> = Vec::new();

    while !input.is_empty() {
      let item: Expr = input.parse()?;

      if let Expr::Range(range_expr) = &item {
        let start = if let Some(start_expr) = &range_expr.start {
          start_expr.as_int::<i32>()?
        } else {
          return Err(input.error("Expected a defined start for this range"));
        };

        if let Some(end_expr) = &range_expr.end {
          let mut end = end_expr.as_int::<i32>()?;

          if let RangeLimits::Closed(_) = &range_expr.limits {
            end += 1;
          }
          ranges.push(GenericRange::Closed(start..end))
        } else {
          ranges.push(GenericRange::Open(start..))
        }
      } else if let Expr::Lit(lit) = &item && let Lit::Int(lit_int) = &lit.lit {
        let num = lit_int.base10_parse::<i32>()?;

        ranges.push(GenericRange::Closed(num..num + 1));
      } else {
        return Err(error!(
          item,
          "Expected a range (e.g. `1..5`, `10..=15`) or a single number"
        ));
      }

      if input.is_empty() {
        break;
      }

      let _comma: Comma = input.parse()?;
    }

    Ok(Self { list: ranges })
  }
}

#[derive(Debug, Clone)]
pub enum PathOrClosure {
  Path(TokenStream2),
  Closure(TokenStream2),
}

impl ToTokens for PathOrClosure {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    match self {
      PathOrClosure::Path(path) => path.to_tokens(tokens),
      PathOrClosure::Closure(expr_closure) => expr_closure.to_tokens(tokens),
    }
  }
}

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
  pub list: Vec<T>,
}

pub type PathList = PunctuatedItems<Path>;
pub type IdentList = PunctuatedItems<Ident>;

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

    Ok(Self { list: inner })
  }
}

impl<T: Parse + ToTokens> ToTokens for PunctuatedItems<T> {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let list = &self.list;

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
