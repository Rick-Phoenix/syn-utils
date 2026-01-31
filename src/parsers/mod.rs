use core::slice;
use std::vec;

use syn::{RangeLimits, token::Comma};

use crate::*;

pub fn parse_bracketed<T: Parse>(input: ParseStream) -> syn::Result<T> {
  let content;
  syn::bracketed!(content in input);
  content.parse::<T>()
}

pub fn parse_braced<T: Parse>(input: ParseStream) -> syn::Result<T> {
  let content;
  syn::braced!(content in input);
  content.parse::<T>()
}

#[derive(Debug, Clone)]
pub struct ParsedStr {
  pub str: String,
  pub span: Span,
}

impl Default for ParsedStr {
  #[inline]
  fn default() -> Self {
    Self {
      str: String::new(),
      span: Span::call_site(),
    }
  }
}

impl ParsedStr {
  #[must_use]
  #[inline]
  pub fn with_default_span(str: String) -> Self {
    Self {
      str,
      span: Span::call_site(),
    }
  }
}

impl Display for ParsedStr {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.str)
  }
}

impl Deref for ParsedStr {
  type Target = str;
  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.str
  }
}

impl Parse for ParsedStr {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let lit: LitStr = input.parse()?;
    let span = lit.span();

    Ok(Self {
      str: lit.value(),
      span,
    })
  }
}

impl ToTokens for ParsedStr {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let mut lit = proc_macro2::Literal::string(&self.str);
    lit.set_span(self.span);

    lit.to_tokens(tokens);
  }
}

#[derive(Debug, Clone, Copy)]
pub struct ParsedNum {
  pub num: i32,
  pub span: Span,
}

impl Default for ParsedNum {
  #[inline]
  fn default() -> Self {
    Self {
      num: 0,
      span: Span::call_site(),
    }
  }
}

impl ParsedNum {
  #[must_use]
  #[inline]
  pub fn with_default_span(num: i32) -> Self {
    Self {
      num,
      span: Span::call_site(),
    }
  }
}

impl Display for ParsedNum {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.num)
  }
}

impl Deref for ParsedNum {
  type Target = i32;
  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.num
  }
}

impl Parse for ParsedNum {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let lit: LitInt = input.parse()?;
    let span = lit.span();

    Ok(Self {
      num: lit.base10_parse()?,
      span,
    })
  }
}

impl ToTokens for ParsedNum {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let mut literal = proc_macro2::Literal::i32_unsuffixed(self.num);
    literal.set_span(self.span);

    literal.to_tokens(tokens);
  }
}

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
      } else if let Expr::Lit(lit) = &item
        && let Lit::Int(lit_int) = &lit.lit
      {
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
      } else if let Expr::Lit(lit) = &item
        && let Lit::Int(lit_int) = &lit.lit
      {
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
      Self::Path(path) => path.to_tokens(tokens),
      Self::Closure(expr_closure) => expr_closure.to_tokens(tokens),
    }
  }
}

#[derive(Debug, Clone)]
pub enum ClosureOrExpr {
  Closure(TokenStream2),
  Expr(TokenStream2),
}

impl Parse for ClosureOrExpr {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let expr: Expr = input.parse()?;

    let output = match expr {
      Expr::Closure(closure) => Self::Closure(closure.into_token_stream()),
      _ => Self::Expr(expr.into_token_stream()),
    };

    Ok(output)
  }
}

impl ToTokens for ClosureOrExpr {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    match self {
      Self::Expr(call) => call.to_tokens(tokens),
      Self::Closure(expr_closure) => expr_closure.to_tokens(tokens),
    }
  }
}

pub struct PunctuatedItems<T: Parse + ToTokens> {
  pub list: Vec<T>,
}

impl<T: Parse + ToTokens> Default for PunctuatedItems<T> {
  #[inline]
  fn default() -> Self {
    Self { list: vec![] }
  }
}

impl<T: Parse + ToTokens> DerefMut for PunctuatedItems<T> {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.list
  }
}

impl<T: Parse + ToTokens> Deref for PunctuatedItems<T> {
  type Target = [T];

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.list
  }
}

impl<T: Parse + ToTokens> PunctuatedItems<T> {
  #[inline]
  pub fn iter(&self) -> slice::Iter<'_, T> {
    self.into_iter()
  }

  #[inline]
  pub fn iter_mut(&mut self) -> slice::IterMut<'_, T> {
    self.into_iter()
  }
}

impl<T: Parse + ToTokens> IntoIterator for PunctuatedItems<T> {
  type IntoIter = vec::IntoIter<T>;
  type Item = T;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.list.into_iter()
  }
}

impl<'a, T: Parse + ToTokens> IntoIterator for &'a PunctuatedItems<T> {
  type IntoIter = slice::Iter<'a, T>;
  type Item = &'a T;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.list.iter()
  }
}

impl<'a, T: Parse + ToTokens> IntoIterator for &'a mut PunctuatedItems<T> {
  type IntoIter = slice::IterMut<'a, T>;
  type Item = &'a mut T;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.list.iter_mut()
  }
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
