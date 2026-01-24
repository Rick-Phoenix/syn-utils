use crate::*;

pub trait FieldExt {
  #[allow(private_interfaces)]
  const SEALED: Sealed;

  fn require_ident(&self) -> syn::Result<&Ident>;
}

impl FieldExt for Field {
  #[allow(private_interfaces)]
  const SEALED: Sealed = Sealed;

  fn require_ident(&self) -> syn::Result<&Ident> {
    self
      .ident
      .as_ref()
      .ok_or_else(|| error!(self, "Expected a named field"))
  }
}
