use crate::*;

pub type TokenStreamOr = TokensOr<TokenStream2>;

#[derive(Debug, Clone)]
pub struct TokensOr<T: ToTokens> {
  pub tokens: Option<T>,
  pub default_fn: fn(Span) -> TokenStream2,
  pub format_fn: fn(Span, &T) -> TokenStream2,
  pub span: Span,
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

  #[inline]
  pub fn new_spanned(span: Span, default: fn(Span) -> TokenStream2) -> Self {
    Self {
      tokens: None,
      default_fn: default,
      format_fn: |_, val| quote! { #val },
      span,
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
  pub fn option_spanned(span: Span) -> Self {
    Self {
      tokens: None,
      default_fn: |span| quote_spanned! (span=> None),
      format_fn: |span, val| quote_spanned! (span=> Some(#val)),
      span,
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

#[derive(Debug, Clone)]
pub struct IterTokensOr<T: ToTokens> {
  pub items: Vec<T>,
  pub default_fn: fn(Span) -> TokenStream2,
  pub format_fn: fn(Span, &[T]) -> TokenStream2,
  pub span: Span,
}

pub type IterTokenStreamOr = IterTokensOr<TokenStream2>;

impl<T: ToTokens> IterTokensOr<T> {
  #[must_use]
  #[inline]
  pub const fn with_span(mut self, span: Span) -> Self {
    self.span = span;
    self
  }

  #[must_use]
  #[inline]
  pub fn vec_spanned(span: Span) -> Self {
    Self {
      items: Vec::new(),
      default_fn: |span| quote_spanned! (span=> vec![]),
      format_fn: |span, items| quote_spanned! (span=> vec![ #(#items),* ]),
      span,
    }
  }

  #[must_use]
  #[inline]
  pub fn vec() -> Self {
    Self {
      items: Vec::new(),
      default_fn: |span| quote_spanned! (span=> vec![]),
      format_fn: |span, items| quote_spanned! (span=> vec![ #(#items),* ]),
      span: Span::call_site(),
    }
  }

  #[must_use]
  #[inline]
  pub fn slice_spanned(span: Span) -> Self {
    Self {
      items: Vec::new(),
      default_fn: |span| quote_spanned! (span=> &[]),
      format_fn: |span, items| quote_spanned! (span=> &[ #(#items),* ]),
      span,
    }
  }

  #[must_use]
  #[inline]
  pub fn slice() -> Self {
    Self {
      items: Vec::new(),
      default_fn: |span| quote_spanned! (span=> &[]),
      format_fn: |span, items| quote_spanned! (span=> &[ #(#items),* ]),
      span: Span::call_site(),
    }
  }

  #[inline]
  pub fn new(
    default_fn: fn(Span) -> TokenStream2,
    formatter: fn(Span, &[T]) -> TokenStream2,
  ) -> Self {
    Self {
      items: Vec::new(),
      default_fn,
      format_fn: formatter,
      span: Span::call_site(),
    }
  }

  #[must_use]
  #[inline]
  pub fn with_formatter(mut self, format_fn: fn(Span, &[T]) -> TokenStream2) -> Self {
    self.format_fn = format_fn;
    self
  }

  #[inline]
  pub fn set(&mut self, items: Vec<T>) {
    self.items = items;
  }

  #[inline]
  pub fn push(&mut self, item: T) {
    self.items.push(item);
  }

  #[inline]
  pub fn extend(&mut self, new_items: impl IntoIterator<Item = T>) {
    self.items.extend(new_items);
  }

  #[inline]
  pub fn maybe_set(&mut self, items: Option<Vec<T>>) {
    if let Some(items) = items {
      self.items = items;
    }
  }

  #[must_use]
  #[inline]
  pub const fn is_empty(&self) -> bool {
    self.items.is_empty()
  }
}

impl<T: ToTokens> ToTokens for IterTokensOr<T> {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    if self.items.is_empty() {
      tokens.extend((self.default_fn)(self.span));
    } else {
      tokens.extend((self.format_fn)(self.span, &self.items));
    }
  }
}
