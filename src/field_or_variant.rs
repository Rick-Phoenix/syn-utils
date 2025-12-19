use crate::*;

#[derive(PartialEq, Debug)]
pub enum FieldOrVariant<'a> {
  Field(&'a mut Field),
  Variant(&'a mut Variant),
}

impl<'a> FieldOrVariant<'a> {
  pub fn ident(&self) -> syn::Result<&Ident> {
    match self {
      FieldOrVariant::Field(field) => field.require_ident(),
      FieldOrVariant::Variant(variant) => Ok(&variant.ident),
    }
  }

  pub fn get_type(&self) -> syn::Result<&Type> {
    let output = match self {
      FieldOrVariant::Field(field) => &field.ty,
      FieldOrVariant::Variant(variant) => variant.type_()?,
    };

    Ok(output)
  }

  pub fn type_mut(&mut self) -> syn::Result<&mut Type> {
    let output = match self {
      FieldOrVariant::Field(field) => &mut field.ty,
      FieldOrVariant::Variant(variant) => variant.type_mut()?,
    };

    Ok(output)
  }

  pub fn inject_attr(&mut self, attr: Attribute) {
    match self {
      FieldOrVariant::Field(field) => field.attrs.push(attr),
      FieldOrVariant::Variant(variant) => variant.attrs.push(attr),
    }
  }

  pub fn change_type(&mut self, ty: Type) -> syn::Result<()> {
    let src_type = self.type_mut()?;

    *src_type = ty;

    Ok(())
  }

  /// Returns `true` if the field or variant is [`Field`].
  ///
  /// [`Field`]: FieldOrVariant::Field
  #[must_use]
  pub fn is_field(&self) -> bool {
    matches!(self, Self::Field(..))
  }

  /// Returns `true` if the field or variant is [`Variant`].
  ///
  /// [`Variant`]: FieldOrVariant::Variant
  #[must_use]
  pub fn is_variant(&self) -> bool {
    matches!(self, Self::Variant(..))
  }

  pub fn as_field(&self) -> Option<&&'a mut Field> {
    if let Self::Field(v) = self {
      Some(v)
    } else {
      None
    }
  }

  pub fn as_variant(&self) -> Option<&&'a mut Variant> {
    if let Self::Variant(v) = self {
      Some(v)
    } else {
      None
    }
  }
}
