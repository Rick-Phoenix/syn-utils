use crate::*;

pub trait WithAttributes {
  #[allow(private_interfaces)]
  const SEALED: Sealed;

  fn attrs(&self) -> &[Attribute];
}

impl WithAttributes for Item {
  #[allow(private_interfaces)]
  const SEALED: Sealed = Sealed;

  #[inline]
  fn attrs(&self) -> &[Attribute] {
    match self {
      Self::Enum(item_enum) => &item_enum.attrs,
      Self::Struct(item_struct) => &item_struct.attrs,
      _ => &[],
    }
  }
}
