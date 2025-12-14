use crate::*;

#[derive(Default, Clone, Debug)]
pub struct ControlFlow {
  pub dummy: Option<TokenStream2>,
}

impl ControlFlow {
  pub fn new() -> Self {
    Self { dummy: None }
  }

  pub fn with_custom_dummy(dummy: &TokenStream2) -> Self {
    Self {
      dummy: Some(dummy.to_token_stream()),
    }
  }
}

pub trait MacroError {
  fn append_unimplemented(self) -> TokenStream2;
}

impl MacroError for syn::Error {
  fn append_unimplemented(self) -> TokenStream2 {
    let error = self.into_compile_error();

    quote! { #error; unimplemented!() }
  }
}

pub trait MacroResult: Sized {
  fn unwrap_or_unimplemented(self) -> TokenStream2;
}

impl<T: ToTokens> MacroResult for syn::Result<T> {
  fn unwrap_or_unimplemented(self) -> TokenStream2 {
    self.map_or_else(
      |error| {
        let error = error.into_compile_error();

        quote! { #error; unimplemented!() }
      },
      |v| v.into_token_stream(),
    )
  }
}
