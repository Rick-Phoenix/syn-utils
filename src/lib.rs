use syn::{
  spanned::Spanned, GenericArgument, ParenthesizedGenericArguments, PathArguments, PathSegment,
  Type,
};
#[macro_use]
mod macros;

mod expr_trait;
use std::{fmt::Display, str::FromStr};

pub use expr_trait::*;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::{
  parse::Parse,
  punctuated::{Iter, IterMut, Punctuated},
  Attribute, Expr, ExprCall, ExprClosure, Field, Fields, Ident, Lit, LitInt, LitStr, Meta, Path,
  Token, Variant,
};

pub trait GenericArgumentExt {
  fn as_type(&self) -> syn::Result<&Type>;
  fn as_type_mut(&mut self) -> syn::Result<&mut Type>;
}

impl GenericArgumentExt for GenericArgument {
  fn as_type(&self) -> syn::Result<&Type> {
    if let GenericArgument::Type(ty) = self {
      Ok(ty)
    } else {
      bail!(self, "Expected this generic argument to be a type");
    }
  }

  fn as_type_mut(&mut self) -> syn::Result<&mut Type> {
    if let GenericArgument::Type(ty) = self {
      Ok(ty)
    } else {
      let reborrow = &self;
      bail!(reborrow, "Expected this generic argument to be a type");
    }
  }
}

pub trait PathSegmentExt {
  fn generic_args(&self) -> Option<Iter<'_, GenericArgument>>;
  fn generic_args_mut(&mut self) -> Option<IterMut<'_, GenericArgument>>;
  fn parenthesized_args(&self) -> Option<&ParenthesizedGenericArguments>;
  fn parenthesized_args_mut(&mut self) -> Option<&mut ParenthesizedGenericArguments>;
}

impl PathSegmentExt for PathSegment {
  fn parenthesized_args_mut(&mut self) -> Option<&mut ParenthesizedGenericArguments> {
    match &mut self.arguments {
      PathArguments::None => None,
      PathArguments::AngleBracketed(_) => None,
      PathArguments::Parenthesized(par) => Some(par),
    }
  }

  fn parenthesized_args(&self) -> Option<&ParenthesizedGenericArguments> {
    match &self.arguments {
      PathArguments::None => None,
      PathArguments::AngleBracketed(_) => None,
      PathArguments::Parenthesized(par) => Some(par),
    }
  }

  fn generic_args_mut(&mut self) -> Option<IterMut<'_, GenericArgument>> {
    match &mut self.arguments {
      PathArguments::None => None,
      PathArguments::AngleBracketed(ab) => Some(ab.args.iter_mut()),
      PathArguments::Parenthesized(_) => None,
    }
  }

  fn generic_args(&self) -> Option<Iter<'_, GenericArgument>> {
    match &self.arguments {
      PathArguments::None => None,
      PathArguments::AngleBracketed(ab) => Some(ab.args.iter()),
      PathArguments::Parenthesized(_) => None,
    }
  }
}

pub trait PathExt {
  fn last_segment(&self) -> &PathSegment;
  fn last_segment_mut(&mut self) -> &mut PathSegment;
}

impl PathExt for Path {
  fn last_segment(&self) -> &PathSegment {
    self.segments.last().unwrap()
  }

  fn last_segment_mut(&mut self) -> &mut PathSegment {
    self.segments.last_mut().unwrap()
  }
}

pub trait TypeExt {
  fn as_path(&self) -> syn::Result<&Path>;
  fn as_path_mut(&mut self) -> syn::Result<&mut Path>;
}

impl TypeExt for Type {
  fn as_path(&self) -> syn::Result<&Path> {
    if let Type::Path(path) = self {
      Ok(&path.path)
    } else {
      bail!(self, "Expected a type path");
    }
  }

  fn as_path_mut(&mut self) -> syn::Result<&mut Path> {
    if let Type::Path(path) = self {
      Ok(&mut path.path)
    } else {
      bail!(self, "Expected a type path");
    }
  }
}

pub trait EnumVariant {
  fn is_single_tuple(&self) -> bool;
  fn typ(&self) -> syn::Result<&Type>;
  fn path(&self) -> syn::Result<&Path>;
  fn type_mut(&mut self) -> syn::Result<&mut Type>;
  fn path_mut(&mut self) -> syn::Result<&mut Path>;
  fn is_unit(&self) -> bool;
  fn named_fields(&self) -> syn::Result<Iter<'_, Field>>;
  fn named_fields_mut(&mut self) -> syn::Result<IterMut<'_, Field>>;
  fn unnamed_fields(&self) -> syn::Result<Iter<'_, Field>>;
  fn unnamed_fields_mut(&mut self) -> syn::Result<IterMut<'_, Field>>;
}

impl EnumVariant for Variant {
  fn is_single_tuple(&self) -> bool {
    if let Fields::Unnamed(fields) = &self.fields && fields.unnamed.len() == 1 {
      true
    } else {
      false
    }
  }

  fn path_mut(&mut self) -> syn::Result<&mut Path> {
    let span = self.span();

    if let Fields::Unnamed(fields) = &mut self.fields && fields.unnamed.len() == 1 {
      Ok(fields.unnamed.last_mut().unwrap().ty.as_path_mut()?)
    } else {
      bail_with_span!(span, "Expected this variant to have a single unnamed field");
    }
  }

  fn type_mut(&mut self) -> syn::Result<&mut Type> {
    let span = self.span();

    if let Fields::Unnamed(fields) = &mut self.fields && fields.unnamed.len() == 1 {
      Ok(&mut fields.unnamed.last_mut().unwrap().ty)
    } else {
      bail_with_span!(span, "Expected this variant to have a single unnamed field");
    }
  }

  fn path(&self) -> syn::Result<&Path> {
    if let Fields::Unnamed(fields) = &self.fields && fields.unnamed.len() == 1 {
      Ok(fields.unnamed.last().unwrap().ty.as_path()?)
    } else {
      bail!(self, "Expected this variant to have a single unnamed field");
    }
  }

  fn typ(&self) -> syn::Result<&Type> {
    if let Fields::Unnamed(fields) = &self.fields && fields.unnamed.len() == 1 {
      Ok(&fields.unnamed.last().unwrap().ty)
    } else {
      bail!(self, "Expected this variant to have a single unnamed field");
    }
  }

  fn is_unit(&self) -> bool {
    matches!(self.fields, Fields::Unit)
  }

  fn named_fields(&self) -> syn::Result<Iter<'_, Field>> {
    if let Fields::Named(fields) = &self.fields {
      Ok(fields.named.iter())
    } else {
      bail!(self, "Expected this variant to have named fields");
    }
  }

  fn named_fields_mut(&mut self) -> syn::Result<IterMut<'_, Field>> {
    let span = self.span();

    if let Fields::Named(fields) = &mut self.fields {
      Ok(fields.named.iter_mut())
    } else {
      bail_with_span!(span, "Expected this variant to have named fields");
    }
  }

  fn unnamed_fields(&self) -> syn::Result<Iter<'_, Field>> {
    if let Fields::Unnamed(fields) = &self.fields {
      Ok(fields.unnamed.iter())
    } else {
      bail!(self, "Expected this variant to have unnamed fields");
    }
  }

  fn unnamed_fields_mut(&mut self) -> syn::Result<IterMut<'_, Field>> {
    let span = self.span();

    if let Fields::Unnamed(fields) = &mut self.fields {
      Ok(fields.unnamed.iter_mut())
    } else {
      bail_with_span!(span, "Expected this variant to have unnamed fields");
    }
  }
}

pub trait AsNamedField {
  fn ident(&self) -> syn::Result<&Ident>;
}

impl AsNamedField for Field {
  fn ident(&self) -> syn::Result<&Ident> {
    self
      .ident
      .as_ref()
      .ok_or(error!(self, "Expected a named field"))
  }
}

pub fn filter_attributes(attrs: &[Attribute], allowed_idents: &[&str]) -> syn::Result<Vec<Meta>> {
  let mut metas = Vec::new();

  for attr in attrs {
    let attr_ident = if let Some(ident) = attr.path().get_ident() {
      ident.to_string()
    } else {
      continue;
    };

    if !allowed_idents.contains(&attr_ident.as_str()) {
      continue;
    }

    let parser = Punctuated::<Meta, Token![,]>::parse_terminated;
    let args = attr.parse_args_with(parser)?;

    metas.extend(args);
  }

  Ok(metas)
}

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
