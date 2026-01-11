use crate::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Int {
  ISize,
  I8,
  I16,
  I32,
  I64,
  I128,
}

impl ToTokens for Int {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let output = match self {
      Self::ISize => quote! { isize },
      Self::I8 => quote! { i8 },
      Self::I16 => quote! { i16 },
      Self::I32 => quote! { i32 },
      Self::I64 => quote! { i64 },
      Self::I128 => quote! { i128 },
    };

    tokens.extend(output);
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Uint {
  USize,
  U8,
  U16,
  U32,
  U64,
  U128,
}

impl ToTokens for Uint {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let output = match self {
      Self::USize => quote! { usize },
      Self::U8 => quote! { u8 },
      Self::U16 => quote! { u16 },
      Self::U32 => quote! { u32 },
      Self::U64 => quote! { u64 },
      Self::U128 => quote! { u128 },
    };

    tokens.extend(output);
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Float {
  F32,
  F64,
}

impl ToTokens for Float {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let output = match self {
      Self::F32 => quote! { f32 },
      Self::F64 => quote! { f64 },
    };

    tokens.extend(output);
  }
}
