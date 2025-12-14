use crate::*;

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
