mod primitives;
mod rust_type;
pub use rust_type::*;
mod type_info;

pub use primitives::*;
pub use type_info::*;

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
