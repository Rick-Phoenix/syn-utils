use crate::*;

pub trait EnumVariant {
  fn has_single_item(&self) -> bool;
  fn is_unit(&self) -> bool;
  fn type_(&self) -> syn::Result<&Type>;
  fn type_mut(&mut self) -> syn::Result<&mut Type>;
  fn type_path(&self) -> syn::Result<&Path>;
  fn type_path_mut(&mut self) -> syn::Result<&mut Path>;
  fn named_fields(&self) -> syn::Result<&Punctuated<Field, Token![,]>>;
  fn named_fields_mut(&mut self) -> syn::Result<&mut Punctuated<Field, Token![,]>>;
  fn unnamed_fields(&self) -> syn::Result<&Punctuated<Field, Token![,]>>;
  fn unnamed_fields_mut(&mut self) -> syn::Result<&mut Punctuated<Field, Token![,]>>;
}

impl EnumVariant for Variant {
  fn has_single_item(&self) -> bool {
    if let Fields::Unnamed(fields) = &self.fields && fields.unnamed.len() == 1 {
      true
    } else {
      false
    }
  }

  fn type_path_mut(&mut self) -> syn::Result<&mut Path> {
    let span = self.span();

    if let Fields::Unnamed(fields) = &mut self.fields && fields.unnamed.len() == 1 {
      Ok(fields.unnamed.last_mut().unwrap().ty.as_path_mut()?)
    } else {
      bail_with_span!(span, "Expected this variant to have a single unnamed field");
    }
  }

  /// Returns a mutable ref to the type of the enum variant, if the variant contains only a single unnamed field.
  fn type_mut(&mut self) -> syn::Result<&mut Type> {
    let span = self.span();

    if let Fields::Unnamed(fields) = &mut self.fields && fields.unnamed.len() == 1 {
      Ok(&mut fields.unnamed.last_mut().unwrap().ty)
    } else {
      bail_with_span!(span, "Expected this variant to have a single unnamed field");
    }
  }

  fn type_path(&self) -> syn::Result<&Path> {
    if let Fields::Unnamed(fields) = &self.fields && fields.unnamed.len() == 1 {
      Ok(fields.unnamed.last().unwrap().ty.as_path()?)
    } else {
      bail!(self, "Expected this variant to have a single unnamed field");
    }
  }

  /// Returns the type of the enum variant, if the variant contains only a single unnamed field.
  fn type_(&self) -> syn::Result<&Type> {
    if let Fields::Unnamed(fields) = &self.fields && fields.unnamed.len() == 1 {
      Ok(&fields.unnamed.last().unwrap().ty)
    } else {
      bail!(self, "Expected this variant to have a single unnamed field");
    }
  }

  fn is_unit(&self) -> bool {
    matches!(self.fields, Fields::Unit)
  }

  fn named_fields(&self) -> syn::Result<&Punctuated<Field, Token![,]>> {
    if let Fields::Named(fields) = &self.fields {
      Ok(&fields.named)
    } else {
      bail!(self, "Expected this variant to have named fields");
    }
  }

  fn named_fields_mut(&mut self) -> syn::Result<&mut Punctuated<Field, Token![,]>> {
    let span = self.span();

    if let Fields::Named(fields) = &mut self.fields {
      Ok(&mut fields.named)
    } else {
      bail_with_span!(span, "Expected this variant to have named fields");
    }
  }

  fn unnamed_fields(&self) -> syn::Result<&Punctuated<Field, token::Comma>> {
    if let Fields::Unnamed(fields) = &self.fields {
      Ok(&fields.unnamed)
    } else {
      bail!(self, "Expected this variant to have unnamed fields");
    }
  }

  fn unnamed_fields_mut(&mut self) -> syn::Result<&mut Punctuated<Field, Token![,]>> {
    let span = self.span();

    if let Fields::Unnamed(fields) = &mut self.fields {
      Ok(&mut fields.unnamed)
    } else {
      bail_with_span!(span, "Expected this variant to have unnamed fields");
    }
  }
}
