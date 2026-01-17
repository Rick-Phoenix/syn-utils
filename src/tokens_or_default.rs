use crate::*;

pub type TokenStreamOr = TokensOr<TokenStream2>;

#[derive(Debug, Clone)]
pub struct TokensOr<T: ToTokens> {
  pub tokens: Option<T>,
  pub default_fn: fn(Span) -> TokenStream2,
  pub format_fn: fn(Span, &T) -> TokenStream2,
  pub span: Span,
}

impl<T: ToTokens> Default for TokensOr<T> {
  fn default() -> Self {
    Self {
      tokens: None,
      default_fn: |_| TokenStream2::new(),
      format_fn: |_, val| quote! { #val },
      span: Span::call_site(),
    }
  }
}

impl<T: ToTokens> ToTokens for TokensOr<T> {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    if let Some(inner) = &self.tokens {
      tokens.extend((self.format_fn)(self.span, inner));
    } else {
      tokens.extend((self.default_fn)(self.span));
    }
  }
}

impl<T: ToTokens> TokensOr<T> {
  #[inline]
  pub fn new(default: fn(Span) -> TokenStream2) -> Self {
    Self {
      tokens: None,
      default_fn: default,
      format_fn: |_, val| quote! { #val },
      span: Span::call_site(),
    }
  }

  #[must_use]
  #[inline]
  pub fn vec() -> Self {
    Self {
      tokens: None,
      default_fn: |span| quote_spanned! (span=> vec![]),
      format_fn: |_, val| quote! { #val },
      span: Span::call_site(),
    }
  }

  #[must_use]
  #[inline]
  pub fn option() -> Self {
    Self {
      tokens: None,
      default_fn: |span| quote_spanned! (span=> None),
      format_fn: |span, val| quote_spanned! (span=> Some(#val)),
      span: Span::call_site(),
    }
  }

  #[must_use]
  #[inline]
  pub fn with_formatter(mut self, format_fn: fn(Span, &T) -> TokenStream2) -> Self {
    self.format_fn = format_fn;
    self
  }

  #[must_use]
  #[inline]
  pub const fn with_span(mut self, span: Span) -> Self {
    self.span = span;
    self
  }

  #[inline]
  pub fn set(&mut self, tokens: T) {
    self.tokens = Some(tokens);
  }

  #[inline]
  pub fn maybe_set(&mut self, tokens: Option<T>) {
    self.tokens = tokens;
  }

  #[inline]
  pub const fn is_default(&self) -> bool {
    self.tokens.is_none()
  }
}
