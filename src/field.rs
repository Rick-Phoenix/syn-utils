use crate::*;

pub trait FieldExt {
  fn require_ident(&self) -> syn::Result<&Ident>;
}

impl FieldExt for Field {
  fn require_ident(&self) -> syn::Result<&Ident> {
    self
      .ident
      .as_ref()
      .ok_or(error!(self, "Expected a named field"))
  }
}
