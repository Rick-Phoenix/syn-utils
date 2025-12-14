use crate::*;

pub trait AsNamedField {
  fn require_ident(&self) -> syn::Result<&Ident>;
}

impl AsNamedField for Field {
  fn require_ident(&self) -> syn::Result<&Ident> {
    self
      .ident
      .as_ref()
      .ok_or(error!(self, "Expected a named field"))
  }
}
