use std::hash::{Hash, Hasher};

use crate::*;

#[derive(Debug, Clone)]
pub struct TypeInfo {
  pub reference: Option<Ref>,
  pub type_: Rc<RustType>,
  pub(crate) span: Span,
}

impl PartialEq for TypeInfo {
  fn eq(&self, other: &Self) -> bool {
    self.reference == other.reference && self.type_ == other.type_
  }
}

impl Eq for TypeInfo {}

impl Hash for TypeInfo {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.reference.hash(state);
    self.type_.hash(state);
  }
}

impl ToTokens for TypeInfo {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let ref_tokens = &self.reference;
    let type_tokens = &self.type_;
    let span = self.span;

    tokens.extend(quote_spanned! {span=>
      #ref_tokens #type_tokens
    });
  }
}

impl From<TypeInfo> for Type {
  fn from(value: TypeInfo) -> Self {
    value.as_type()
  }
}

impl TypeInfo {
  pub fn inner(&self) -> &Self {
    match self.type_.as_ref() {
      RustType::Slice(ty) => ty,
      RustType::Array(array) => &array.inner,
      RustType::Option(ty) => ty,
      RustType::Box(ty) => ty,
      RustType::Vec(ty) => ty,
      RustType::Tuple(_) => self,
      RustType::HashMap(_) => self,
      RustType::String => self,
      RustType::Int(_) => self,
      RustType::Uint(_) => self,
      RustType::Float(_) => self,
      RustType::Bool => self,
      RustType::Other(_) => self,
      RustType::Bytes => self,
    }
  }

  pub fn as_path(&self) -> Option<Path> {
    self.type_.as_path()
  }

  pub fn require_path(&self) -> syn::Result<Path> {
    self
      .as_path()
      .ok_or(error!(self, "Expected a type path"))
  }

  pub fn as_type(&self) -> Type {
    parse_quote_spanned! {self.span=>
      #self
    }
  }

  pub fn is_mut_ref(&self) -> bool {
    self
      .reference
      .as_ref()
      .is_some_and(|r| matches!(r.kind, RefKind::MutRef))
  }

  pub fn is_ref(&self) -> bool {
    self
      .reference
      .as_ref()
      .is_some_and(|r| matches!(r.kind, RefKind::Ref))
  }

  pub fn is_owned(&self) -> bool {
    self.reference.is_none()
  }

  pub fn from_type(typ: &Type) -> syn::Result<Self> {
    if let Type::Reference(ty_reference) = typ {
      let ref_kind = if ty_reference.mutability.is_some() {
        RefKind::MutRef
      } else {
        RefKind::Ref
      };

      let reference = Some(Ref {
        lifetime: ty_reference.lifetime.clone(),
        kind: ref_kind,
      });

      if let Type::Slice(slice) = &*ty_reference.elem {
        return Ok(Self {
          reference,
          type_: RustType::Slice(Self::from_type(&slice.elem)?.into()).into(),
          span: typ.span(),
        });
      } else {
        let mut ref_type = Self::from_type(&ty_reference.elem)?;
        ref_type.reference = reference;

        return Ok(ref_type);
      }
    }

    let output = match typ {
      Type::Slice(slice) => {
        let inner = Self::from_type(&slice.elem)?;

        Self {
          reference: None,
          type_: RustType::Slice(inner.into()).into(),
          span: typ.span(),
        }
      }
      Type::Array(TypeArray { elem, len, .. }) => {
        let inner = Self::from_type(elem)?;

        Self {
          reference: None,
          span: typ.span(),
          type_: RustType::Array(
            Array {
              len: len.clone(),
              inner: inner.into(),
            }
            .into(),
          )
          .into(),
        }
      }
      Type::Path(path) => {
        let last_segment = path.path.last_segment();

        let last_segment_ident = last_segment.ident.to_string();

        match last_segment_ident.as_str() {
          "i32" => Self {
            reference: None,
            type_: RustType::Int(Int::I32).into(),
            span: typ.span(),
          },
          "u32" => Self {
            reference: None,
            type_: RustType::Uint(Uint::U32).into(),
            span: typ.span(),
          },
          "f32" => Self {
            reference: None,
            type_: RustType::Float(Float::F32).into(),
            span: typ.span(),
          },
          "Bytes" => Self {
            reference: None,
            type_: RustType::Bytes.into(),
            span: typ.span(),
          },
          "HashMap" => {
            let (k, v) = last_segment.first_two_generics().unwrap();

            Self {
              reference: None,
              type_: RustType::HashMap((
                Self::from_type(k.as_type()?)?.into(),
                Self::from_type(v.as_type()?)?.into(),
              ))
              .into(),
              span: typ.span(),
            }
          }
          "Box" => {
            let inner = last_segment.first_generic().unwrap();

            Self {
              reference: None,
              span: typ.span(),
              type_: RustType::Box(Self::from_type(inner.as_type()?)?.into()).into(),
            }
          }
          "Vec" => {
            let inner = last_segment.first_generic().unwrap();

            Self {
              reference: None,
              span: typ.span(),
              type_: RustType::Vec(Self::from_type(inner.as_type()?)?.into()).into(),
            }
          }
          "Option" => {
            let inner = last_segment.first_generic().unwrap();

            Self {
              reference: None,
              span: typ.span(),
              type_: RustType::Option(Self::from_type(inner.as_type()?)?.into()).into(),
            }
          }
          _ => Self {
            reference: None,
            span: typ.span(),
            type_: RustType::Other(path.clone().into()).into(),
          },
        }
      }
      Type::Tuple(tuple) => {
        let types: Vec<Self> = tuple
          .elems
          .iter()
          .map(Self::from_type)
          .collect::<syn::Result<Vec<Self>>>()?;

        let type_enum = RustType::Tuple(types.into());

        Self {
          reference: None,
          span: typ.span(),
          type_: type_enum.into(),
        }
      }

      _ => bail!(
        typ,
        "Unsupported type {}",
        typ.to_token_stream().to_string()
      ),
    };

    Ok(output)
  }

  /// Returns `true` if the rust type is [`Slice`].
  ///
  /// [`Slice`]: RustType::Slice
  #[must_use]
  pub fn is_slice(&self) -> bool {
    self.type_.is_slice()
  }

  /// Returns `true` if the rust type is [`Array`].
  ///
  /// [`Array`]: RustType::Array
  #[must_use]
  pub fn is_array(&self) -> bool {
    self.type_.is_array()
  }

  /// Returns `true` if the rust type is [`Tuple`].
  ///
  /// [`Tuple`]: RustType::Tuple
  #[must_use]
  pub fn is_tuple(&self) -> bool {
    self.type_.is_tuple()
  }

  /// Returns `true` if the rust type is [`Option`].
  ///
  /// [`Option`]: RustType::Option
  #[must_use]
  pub fn is_option(&self) -> bool {
    self.type_.is_option()
  }

  /// Returns `true` if the rust type is [`Box`].
  ///
  /// [`Box`]: RustType::Box
  #[must_use]
  pub fn is_box(&self) -> bool {
    self.type_.is_box()
  }

  /// Returns `true` if the rust type is [`Vec`].
  ///
  /// [`Vec`]: RustType::Vec
  #[must_use]
  pub fn is_vec(&self) -> bool {
    self.type_.is_vec()
  }

  /// Returns `true` if the rust type is [`HashMap`].
  ///
  /// [`HashMap`]: RustType::HashMap
  #[must_use]
  pub fn is_hash_map(&self) -> bool {
    self.type_.is_hash_map()
  }

  /// Returns `true` if the rust type is [`Other`].
  ///
  /// [`Other`]: RustType::Other
  #[must_use]
  pub fn is_other(&self) -> bool {
    self.type_.is_other()
  }

  /// Returns `true` if the rust type is [`Bool`].
  ///
  /// [`Bool`]: RustType::Bool
  #[must_use]
  pub fn is_bool(&self) -> bool {
    self.type_.is_bool()
  }

  /// Returns `true` if the rust type is [`String`].
  ///
  /// [`String`]: RustType::String
  #[must_use]
  pub fn is_string(&self) -> bool {
    self.type_.is_string()
  }

  /// Returns `true` if the rust type is [`Int`].
  ///
  /// [`Int`]: RustType::Int
  #[must_use]
  pub fn is_int(&self) -> bool {
    self.type_.is_int()
  }

  /// Returns `true` if the rust type is [`Uint`].
  ///
  /// [`Uint`]: RustType::Uint
  #[must_use]
  pub fn is_uint(&self) -> bool {
    self.type_.is_uint()
  }

  /// Returns `true` if the rust type is [`Float`].
  ///
  /// [`Float`]: RustType::Float
  #[must_use]
  pub fn is_float(&self) -> bool {
    self.type_.is_float()
  }

  #[must_use]
  pub fn is_num(&self) -> bool {
    self.type_.is_num()
  }

  #[must_use]
  pub fn is_primitive(&self) -> bool {
    self.type_.is_primitive()
  }

  /// Returns `true` if the rust type is [`Bytes`].
  ///
  /// [`Bytes`]: RustType::Bytes
  #[must_use]
  pub fn is_bytes(&self) -> bool {
    self.type_.is_bytes()
  }
}
