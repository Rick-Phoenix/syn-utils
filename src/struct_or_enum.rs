use crate::*;

pub trait WithAttributes {
  fn attrs(&self) -> &[Attribute];
}

impl WithAttributes for Item {
  #[inline]
  fn attrs(&self) -> &[Attribute] {
    match self {
      Self::Enum(item_enum) => &item_enum.attrs,
      Self::Struct(item_struct) => &item_struct.attrs,
      _ => &[],
    }
  }
}
