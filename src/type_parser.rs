use std::rc::Rc;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ref {
  pub lifetime: Option<Lifetime>,
  pub kind: RefKind,
}

impl ToTokens for Ref {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let lifetime = &self.lifetime;
    let output = match &self.kind {
      RefKind::Ref => quote! { & #lifetime },
      RefKind::MutRef => quote! { & #lifetime mut },
    };

    tokens.extend(output);
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeInfo {
  pub reference: Option<Ref>,
  pub type_: Rc<RustType>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum RefKind {
  Ref,
  MutRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Array {
  pub len: Expr,
  pub inner: Rc<TypeInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RustType {
  Slice(Rc<TypeInfo>),
  Array(Rc<Array>),
  Tuple(Rc<[TypeInfo]>),
  Option(Rc<TypeInfo>),
  Box(Rc<TypeInfo>),
  Vec(Rc<TypeInfo>),
  HashMap((Rc<TypeInfo>, Rc<TypeInfo>)),
  Other(Rc<TypePath>),
}

impl RustType {
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
}

impl ToTokens for TypeInfo {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let ref_tokens = &self.reference;
    let type_tokens = &self.type_;

    tokens.extend(quote! {
      #ref_tokens #type_tokens
    });
  }
}

impl ToTokens for RustType {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let output = match self {
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
    };

    tokens.extend(output);
  }
}

impl From<TypeInfo> for Type {
  fn from(value: TypeInfo) -> Self {
    parse_quote!(#value)
  }
}

impl TypeInfo {
  pub fn as_type(&self) -> Type {
    parse_quote!(#self)
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
    if let Type::Reference(reference) = typ {
      let ownership = if reference.mutability.is_some() {
        RefKind::MutRef
      } else {
        RefKind::Ref
      };

      if let Type::Slice(slice) = &*reference.elem {
        return Ok(Self {
          reference: Some(Ref {
            lifetime: reference.lifetime.clone(),
            kind: ownership,
          }),
          type_: RustType::Slice(Self::from_type(&slice.elem)?.into()).into(),
        });
      } else {
        let mut ref_type = Self::from_type(&reference.elem)?;
        ref_type.reference = Some(Ref {
          lifetime: reference.lifetime.clone(),
          kind: ownership,
        });

        return Ok(ref_type);
      }
    }

    let output = match typ {
      Type::Slice(slice) => {
        let inner = Self::from_type(&slice.elem)?;

        Self {
          reference: None,
          type_: RustType::Slice(inner.into()).into(),
        }
      }
      Type::Array(TypeArray { elem, len, .. }) => {
        let inner = Self::from_type(elem)?;

        Self {
          reference: None,
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
          "HashMap" => {
            let (k, v) = last_segment.first_two_generics().unwrap();

            Self {
              reference: None,
              type_: RustType::HashMap((
                Self::from_type(k.as_type()?)?.into(),
                Self::from_type(v.as_type()?)?.into(),
              ))
              .into(),
            }
          }
          "Box" => {
            let inner = last_segment.first_generic().unwrap();

            Self {
              reference: None,

              type_: RustType::Box(Self::from_type(inner.as_type()?)?.into()).into(),
            }
          }
          "Vec" => {
            let inner = last_segment.first_generic().unwrap();

            Self {
              reference: None,

              type_: RustType::Vec(Self::from_type(inner.as_type()?)?.into()).into(),
            }
          }
          "Option" => {
            let inner = last_segment.first_generic().unwrap();

            Self {
              reference: None,

              type_: RustType::Option(Self::from_type(inner.as_type()?)?.into()).into(),
            }
          }
          _ => Self {
            reference: None,

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
    matches!(*self.type_, RustType::Slice(..))
  }

  /// Returns `true` if the rust type is [`Array`].
  ///
  /// [`Array`]: RustType::Array
  #[must_use]
  pub fn is_array(&self) -> bool {
    matches!(*self.type_, RustType::Array { .. })
  }

  /// Returns `true` if the rust type is [`Tuple`].
  ///
  /// [`Tuple`]: RustType::Tuple
  #[must_use]
  pub fn is_tuple(&self) -> bool {
    matches!(*self.type_, RustType::Tuple(..))
  }

  /// Returns `true` if the rust type is [`Option`].
  ///
  /// [`Option`]: RustType::Option
  #[must_use]
  pub fn is_option(&self) -> bool {
    matches!(*self.type_, RustType::Option(..))
  }

  /// Returns `true` if the rust type is [`Box`].
  ///
  /// [`Box`]: RustType::Box
  #[must_use]
  pub fn is_box(&self) -> bool {
    matches!(*self.type_, RustType::Box(..))
  }

  /// Returns `true` if the rust type is [`Vec`].
  ///
  /// [`Vec`]: RustType::Vec
  #[must_use]
  pub fn is_vec(&self) -> bool {
    matches!(*self.type_, RustType::Vec(..))
  }

  /// Returns `true` if the rust type is [`HashMap`].
  ///
  /// [`HashMap`]: RustType::HashMap
  #[must_use]
  pub fn is_hash_map(&self) -> bool {
    matches!(*self.type_, RustType::HashMap(..))
  }

  /// Returns `true` if the rust type is [`Other`].
  ///
  /// [`Other`]: RustType::Other
  #[must_use]
  pub fn is_other(&self) -> bool {
    matches!(*self.type_, RustType::Other(..))
  }
}

#[cfg(test)]
mod tests {
  use quote::ToTokens;
  use syn::Type;

  use super::*;

  fn assert_round_trip(input_str: &str) {
    let original_type: Type = syn::parse_str(input_str).expect("Invalid Rust syntax in test");

    let info = TypeInfo::from_type(&original_type).unwrap();

    let tokens = info.to_token_stream();

    let output_str = tokens.to_string();

    let normalized_input = quote!(#original_type).to_string();

    assert_eq!(
      output_str, normalized_input,
      "Round trip failed for: {}",
      input_str
    );
  }

  fn assert_is_option(ty: &str) {
    let original: Type = syn::parse_str(ty).unwrap();
    let info = TypeInfo::from_type(&original).unwrap();

    if let RustType::Option(_) = *info.type_ {
      // OK
    } else {
      panic!("Expected Option, got {:?}", info.type_);
    }
  }

  #[test]
  fn test_primitives() {
    assert_round_trip("i32");
    assert_round_trip("String");
    assert_round_trip("bool");
  }

  #[test]
  fn test_references() {
    assert_round_trip("&i32");
    assert_round_trip("&mut String");
    assert_round_trip("&'a [u8]");
    assert_round_trip("&'static mut Vec<i32>");
  }

  #[test]
  fn test_wrappers() {
    assert_round_trip("::core::option::Option<i32>");
    assert_round_trip("Vec<String>");
    assert_round_trip("Box<MyStruct>");
    assert_round_trip("HashMap<String, i32>");

    assert_is_option("::core::option::Option<i32>");
  }

  #[test]
  fn test_nested_complex() {
    assert_round_trip("::core::option::Option<Vec<Box<i32>>>");
    assert_round_trip("&mut HashMap<String, ::core::option::Option<Vec<u8>>>");
  }

  #[test]
  fn test_arrays_and_tuples() {
    assert_round_trip("[u8; 4]");
    assert_round_trip("(&str, i32, ::core::option::Option<bool>)");
  }

  #[test]
  fn test_slices() {
    assert_round_trip("[u8]");
    assert_round_trip("&[i32]");
  }

  fn get_info(s: &str) -> TypeInfo {
    let ty: Type =
      syn::parse_str(s).unwrap_or_else(|_| panic!("Failed to parse rust syntax: {}", s));
    TypeInfo::from_type(&ty).unwrap()
  }

  fn assert_inner_eq(info: &TypeInfo, expected: &str) {
    let tokens = info.to_token_stream().to_string();
    assert_eq!(tokens, expected, "Inner type mismatch");
  }

  #[test]
  fn test_wrappers_option() {
    let info = get_info("Option<i32>");

    if let RustType::Option(inner) = &*info.type_ {
      assert_inner_eq(inner, "i32");
    } else {
      panic!("Failed to parse Option. Got different variant.");
    }
  }

  #[test]
  fn test_wrappers_vec() {
    let info = get_info("Vec<String>");

    if let RustType::Vec(inner) = &*info.type_ {
      assert_inner_eq(inner, "String");
    } else {
      panic!("Failed to parse Vec.");
    }
  }

  #[test]
  fn test_wrappers_box() {
    let info = get_info("Box<MyStruct>");

    if let RustType::Box(inner) = &*info.type_ {
      assert_inner_eq(inner, "MyStruct");
    } else {
      panic!("Failed to parse Box.");
    }
  }

  #[test]
  fn test_hashmap() {
    let info = get_info("HashMap<String, i32>");

    if let RustType::HashMap((k, v)) = &*info.type_ {
      assert_inner_eq(k, "String");
      assert_inner_eq(v, "i32");
    } else {
      panic!("Failed to parse HashMap. Got type {:#?}", info.type_);
    }
  }

  #[test]
  fn test_slice() {
    let info = get_info("[u8]");

    if let RustType::Slice(inner) = &*info.type_ {
      assert_inner_eq(inner, "u8");
    } else {
      panic!("Failed to parse Slice.");
    }
  }

  #[test]
  fn test_array() {
    let info = get_info("[u8; 4]");

    if let RustType::Array(array) = &*info.type_ {
      let Array { len, inner } = array.as_ref();
      assert_inner_eq(inner, "u8");
      let len_str = quote!(#len).to_string();
      assert_eq!(len_str, "4");
    } else {
      panic!("Failed to parse Array.");
    }
  }

  #[test]
  fn test_tuple() {
    let info = get_info("(i32, bool, String)");

    if let RustType::Tuple(items) = &*info.type_ {
      assert_eq!(items.len(), 3);
      assert_inner_eq(&items[0], "i32");
      assert_inner_eq(&items[1], "bool");
      assert_inner_eq(&items[2], "String");
    } else {
      panic!("Failed to parse Tuple.");
    }
  }

  #[test]
  fn test_nested_wrappers() {
    let info = get_info("Option<Vec<Box<i32>>>");

    if let RustType::Option(l1) = &*info.type_
      && let RustType::Vec(l2) = &*l1.type_
        && let RustType::Box(l3) = &*l2.type_ {
          assert_inner_eq(l3, "i32");
          return;
        }
    panic!("Failed to parse deeply nested wrappers");
  }

  #[test]
  fn test_references_mut() {
    let info = get_info("&mut String");

    assert!(info.reference.is_some(), "Should be a reference");
    let ref_data = info.reference.unwrap();

    match ref_data.kind {
      RefKind::MutRef => {}
      _ => panic!("Expected MutRef"),
    }

    if let RustType::Other(path) = &*info.type_ {
      let path_str = quote!(#path).to_string();
      assert_eq!(path_str, "String");
    } else {
      panic!("Inner type should be Other(String)");
    }
  }

  #[test]
  fn test_references_const() {
    let info = get_info("&i32");

    let ref_data = info.reference.expect("Should be a reference");
    match ref_data.kind {
      RefKind::Ref => {}
      _ => panic!("Expected Ref (const)"),
    }
  }

  #[test]
  fn test_other_path() {
    let info = get_info("my_crate::MyType");

    if let RustType::Other(path) = &*info.type_ {
      let s = quote::quote!(#path).to_string().replace(" ", "");
      assert_eq!(s, "my_crate::MyType");
    } else {
      panic!("Should have parsed as Other/Path");
    }
  }
}
