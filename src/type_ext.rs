use crate::*;

pub trait TypeExt {
  #[allow(private_interfaces)]
  const SEALED: Sealed;

  fn as_path(&self) -> syn::Result<&Path>;
  fn as_path_mut(&mut self) -> syn::Result<&mut Path>;
}

impl TypeExt for Type {
  #[allow(private_interfaces)]
  const SEALED: Sealed = Sealed;

  fn as_path(&self) -> syn::Result<&Path> {
    if let Self::Path(path) = self {
      Ok(&path.path)
    } else {
      bail!(self, "Expected a type path");
    }
  }

  fn as_path_mut(&mut self) -> syn::Result<&mut Path> {
    if let Self::Path(path) = self {
      Ok(&mut path.path)
    } else {
      bail!(self, "Expected a type path");
    }
  }
}
