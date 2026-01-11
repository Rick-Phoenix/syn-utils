use crate::*;

pub trait OptionSpan {
  #[allow(private_interfaces)]
  const SEALED: Sealed;

  fn unwrap_or_call_site(self) -> Span;
}

impl OptionSpan for Option<Span> {
  #[allow(private_interfaces)]
  const SEALED: Sealed = Sealed;

  #[inline]
  fn unwrap_or_call_site(self) -> Span {
    self.unwrap_or_else(Span::call_site)
  }
}

pub trait MacroResult: Sized {
  #[allow(private_interfaces)]
  const SEALED: Sealed;

  fn unwrap_or_unimplemented(self) -> TokenStream2;
  fn unwrap_or_compile_error(self) -> TokenStream2;
}

impl<T: ToTokens> MacroResult for syn::Result<T> {
  #[allow(private_interfaces)]
  const SEALED: Sealed = Sealed;

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
