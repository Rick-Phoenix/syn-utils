use crate::*;

pub trait GenericArgumentExt {
  #[allow(private_interfaces)]
  const SEALED: Sealed;

  fn as_type(&self) -> syn::Result<&Type>;
  fn as_type_mut(&mut self) -> syn::Result<&mut Type>;
}

impl GenericArgumentExt for GenericArgument {
  #[allow(private_interfaces)]
  const SEALED: Sealed = Sealed;

  fn as_type(&self) -> syn::Result<&Type> {
    if let Self::Type(ty) = self {
      Ok(ty)
    } else {
      bail!(self, "Expected this generic argument to be a type");
    }
  }

  fn as_type_mut(&mut self) -> syn::Result<&mut Type> {
    if let Self::Type(ty) = self {
      Ok(ty)
    } else {
      bail!(self, "Expected this generic argument to be a type");
    }
  }
}
