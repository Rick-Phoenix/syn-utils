use crate::*;

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
