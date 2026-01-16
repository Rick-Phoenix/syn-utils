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

pub trait OptionTokens: Sized {
  #[allow(private_interfaces)]
  const SEALED: Sealed;

  fn unwrap_or_unimplemented(self) -> TokenStream2;
}

impl<T: ToTokens> OptionTokens for Option<T> {
  #[allow(private_interfaces)]
  const SEALED: Sealed = Sealed;

  fn unwrap_or_unimplemented(self) -> TokenStream2 {
    self.map_or_else(|| quote! { unimplemented!() }, |i| i.to_token_stream())
  }
}

pub trait MacroResult<T>: Sized {
  #[allow(private_interfaces)]
  const SEALED: Sealed;

  fn unwrap_or_unimplemented(self) -> TokenStream2
  where
    T: ToTokens;
  fn unwrap_or_compile_error(self) -> TokenStream2
  where
    T: ToTokens;
  fn unwrap_or_default_and_push_error(self, errors: &mut Vec<syn::Error>) -> T
  where
    T: Default;
}

impl<T> MacroResult<T> for syn::Result<T> {
  #[allow(private_interfaces)]
  const SEALED: Sealed = Sealed;

  fn unwrap_or_default_and_push_error(self, errors: &mut Vec<syn::Error>) -> T
  where
    T: Default,
  {
    self.unwrap_or_else(|e| {
      errors.push(e);
      T::default()
    })
  }

  fn unwrap_or_compile_error(self) -> TokenStream2
  where
    T: ToTokens,
  {
    self.map_or_else(|err| err.into_compile_error(), |v| v.into_token_stream())
  }

  fn unwrap_or_unimplemented(self) -> TokenStream2
  where
    T: ToTokens,
  {
    self.map_or_else(
      |error| {
        let error = error.into_compile_error();

        quote! { #error; unimplemented!() }
      },
      |v| v.into_token_stream(),
    )
  }
}
