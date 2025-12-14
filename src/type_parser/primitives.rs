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
      Int::ISize => quote! { isize },
      Int::I8 => quote! { i8 },
      Int::I16 => quote! { i16 },
      Int::I32 => quote! { i32 },
      Int::I64 => quote! { i64 },
      Int::I128 => quote! { i128 },
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
      Uint::USize => quote! { usize },
      Uint::U8 => quote! { u8 },
      Uint::U16 => quote! { u16 },
      Uint::U32 => quote! { u32 },
      Uint::U64 => quote! { u64 },
      Uint::U128 => quote! { u128 },
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
      Float::F32 => quote! { f32 },
      Float::F64 => quote! { f64 },
    };

    tokens.extend(output);
  }
}
