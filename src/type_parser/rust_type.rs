use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RustType {
  Slice(Rc<TypeInfo>),
  Array(Rc<Array>),
  Tuple(Rc<[TypeInfo]>),
  Option(Rc<TypeInfo>),
  Box(Rc<TypeInfo>),
  Vec(Rc<TypeInfo>),
  HashMap((Rc<TypeInfo>, Rc<TypeInfo>)),
  String,
  Int(Int),
  Uint(Uint),
  Float(Float),
  Bool,
  Other(Rc<TypePath>),
}

impl ToTokens for RustType {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let output = match self {
      RustType::Bool => quote! { bool },
      RustType::String => quote! { String },
      RustType::Slice(ty) => quote! { [#ty] },
      RustType::Array(array) => {
        let Array { len, inner } = array.as_ref();
        quote! { [#inner; #len] }
      }
      RustType::Tuple(types) => quote! { (#(#types),*) },
      RustType::Option(ty) => quote! { ::core::option::Option<#ty> },
      RustType::Box(ty) => quote! { Box<#ty> },
      RustType::Vec(ty) => quote! { Vec<#ty> },
      RustType::HashMap((k, v)) => quote! { HashMap<#k, #v> },
      RustType::Other(path) => quote! { #path },
      RustType::Int(int) => int.to_token_stream(),
      RustType::Uint(uint) => uint.to_token_stream(),
      RustType::Float(float) => float.to_token_stream(),
    };

    tokens.extend(output);
  }
}

impl RustType {
  pub fn as_path(&self) -> Option<Path> {
    match self {
      RustType::Tuple(_) | RustType::Slice(_) | RustType::Array(_) => return None,
      _ => {}
    };

    let tokens = self.to_token_stream();
    Some(parse_quote!(#tokens))
  }

  #[must_use]
  pub fn is_num(&self) -> bool {
    matches!(self, Self::Int(_) | Self::Float(_) | Self::Uint(_))
  }

  #[must_use]
  pub fn is_primitive(&self) -> bool {
    matches!(
      self,
      Self::Int(_) | Self::Float(_) | Self::Uint(_) | Self::Bool | Self::String
    )
  }

  /// Returns `true` if the rust type is [`Slice`].
  ///
  /// [`Slice`]: RustType::Slice
  #[must_use]
  pub fn is_slice(&self) -> bool {
    matches!(self, Self::Slice(..))
  }

  /// Returns `true` if the rust type is [`Array`].
  ///
  /// [`Array`]: RustType::Array
  #[must_use]
  pub fn is_array(&self) -> bool {
    matches!(self, Self::Array { .. })
  }

  /// Returns `true` if the rust type is [`Tuple`].
  ///
  /// [`Tuple`]: RustType::Tuple
  #[must_use]
  pub fn is_tuple(&self) -> bool {
    matches!(self, Self::Tuple(..))
  }

  /// Returns `true` if the rust type is [`Option`].
  ///
  /// [`Option`]: RustType::Option
  #[must_use]
  pub fn is_option(&self) -> bool {
    matches!(self, Self::Option(..))
  }

  /// Returns `true` if the rust type is [`Box`].
  ///
  /// [`Box`]: RustType::Box
  #[must_use]
  pub fn is_box(&self) -> bool {
    matches!(self, Self::Box(..))
  }

  /// Returns `true` if the rust type is [`Vec`].
  ///
  /// [`Vec`]: RustType::Vec
  #[must_use]
  pub fn is_vec(&self) -> bool {
    matches!(self, Self::Vec(..))
  }

  /// Returns `true` if the rust type is [`HashMap`].
  ///
  /// [`HashMap`]: RustType::HashMap
  #[must_use]
  pub fn is_hash_map(&self) -> bool {
    matches!(self, Self::HashMap(..))
  }

  /// Returns `true` if the rust type is [`Other`].
  ///
  /// [`Other`]: RustType::Other
  #[must_use]
  pub fn is_other(&self) -> bool {
    matches!(self, Self::Other(..))
  }

  pub fn as_option(&self) -> Option<&TypeInfo> {
    if let Self::Option(v) = self {
      Some(v)
    } else {
      None
    }
  }

  pub fn as_slice(&self) -> Option<&TypeInfo> {
    if let Self::Slice(v) = self {
      Some(v)
    } else {
      None
    }
  }

  pub fn as_tuple(&self) -> Option<&[TypeInfo]> {
    if let Self::Tuple(v) = self {
      Some(v.as_ref())
    } else {
      None
    }
  }

  pub fn as_box(&self) -> Option<&TypeInfo> {
    if let Self::Box(v) = self {
      Some(v)
    } else {
      None
    }
  }

  pub fn as_vec(&self) -> Option<&TypeInfo> {
    if let Self::Vec(v) = self {
      Some(v)
    } else {
      None
    }
  }

  pub fn as_hash_map(&self) -> Option<&(Rc<TypeInfo>, Rc<TypeInfo>)> {
    if let Self::HashMap(v) = self {
      Some(v)
    } else {
      None
    }
  }

  pub fn as_other(&self) -> Option<&TypePath> {
    if let Self::Other(v) = self {
      Some(v)
    } else {
      None
    }
  }

  pub fn as_array(&self) -> Option<&Array> {
    if let Self::Array(v) = self {
      Some(v)
    } else {
      None
    }
  }

  /// Returns `true` if the rust type is [`Bool`].
  ///
  /// [`Bool`]: RustType::Bool
  #[must_use]
  pub fn is_bool(&self) -> bool {
    matches!(self, Self::Bool)
  }

  /// Returns `true` if the rust type is [`String`].
  ///
  /// [`String`]: RustType::String
  #[must_use]
  pub fn is_string(&self) -> bool {
    matches!(self, Self::String)
  }

  /// Returns `true` if the rust type is [`Int`].
  ///
  /// [`Int`]: RustType::Int
  #[must_use]
  pub fn is_int(&self) -> bool {
    matches!(self, Self::Int(..))
  }

  /// Returns `true` if the rust type is [`Uint`].
  ///
  /// [`Uint`]: RustType::Uint
  #[must_use]
  pub fn is_uint(&self) -> bool {
    matches!(self, Self::Uint(..))
  }

  /// Returns `true` if the rust type is [`Float`].
  ///
  /// [`Float`]: RustType::Float
  #[must_use]
  pub fn is_float(&self) -> bool {
    matches!(self, Self::Float(..))
  }
}
