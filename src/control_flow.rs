use crate::*;

pub trait MacroResult: Sized {
  fn unwrap_or_unimplemented(self) -> TokenStream2;
  fn unwrap_or_compile_error(self) -> TokenStream2;
}

impl<T: ToTokens> MacroResult for syn::Result<T> {
  fn unwrap_or_compile_error(self) -> TokenStream2 {
    self.map_or_else(|err| err.into_compile_error(), |v| v.into_token_stream())
  }

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
