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
  pub fn new(default: fn(Span) -> TokenStream2) -> Self {
    Self {
      tokens: None,
      default_fn: default,
      format_fn: |span, val| quote_spanned! {span=> #val},
      span: Span::call_site(),
    }
  }

  pub fn new_spanned(span: Span, default: fn(Span) -> TokenStream2) -> Self {
    Self {
      tokens: None,
      default_fn: default,
      format_fn: |span, val| quote_spanned! {span=> #val},
      span,
    }
  }

  #[must_use]
  pub fn vec() -> Self {
    Self {
      tokens: None,
      default_fn: |span| quote_spanned! {span=> vec![] },
      format_fn: |span, val| {
        quote_spanned! {span=> #val }
      },
      span: Span::call_site(),
    }
  }

  #[must_use]
  pub fn option() -> Self {
    Self {
      tokens: None,
      default_fn: |span| quote_spanned! {span=> None },
      format_fn: |span, val| quote_spanned! {span=> Some(#val) },
      span: Span::call_site(),
    }
  }

  #[must_use]
  pub fn option_spanned(span: Span) -> Self {
    Self {
      tokens: None,
      default_fn: |span| quote_spanned! {span=> None },
      format_fn: |span, val| quote_spanned! {span=> Some(#val) },
      span,
    }
  }

  #[must_use]
  pub fn with_formatter(mut self, format_fn: fn(Span, &T) -> TokenStream2) -> Self {
    self.format_fn = format_fn;
    self
  }

  #[must_use]
  pub const fn with_span(mut self, span: Span) -> Self {
    self.span = span;
    self
  }

  pub fn set(&mut self, tokens: T) {
    self.tokens = Some(tokens);
  }

  pub fn maybe_set(&mut self, tokens: Option<T>) {
    self.tokens = tokens;
  }

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
  pub const fn with_span(mut self, span: Span) -> Self {
    self.span = span;
    self
  }

  #[must_use]
  pub fn vec_spanned(span: Span) -> Self {
    Self {
      items: Vec::new(),
      default_fn: |span| quote_spanned! {span=> vec![] },
      format_fn: |span, items| {
        quote_spanned! {span=> vec![ #(#items),* ] }
      },
      span,
    }
  }

  #[must_use]
  pub fn vec() -> Self {
    Self {
      items: Vec::new(),
      default_fn: |span| quote_spanned! {span=> vec![] },
      format_fn: |span, items| {
        quote_spanned! {span=> vec![ #(#items),* ] }
      },
      span: Span::call_site(),
    }
  }

  #[must_use]
  pub fn slice_spanned(span: Span) -> Self {
    Self {
      items: Vec::new(),
      default_fn: |span| quote_spanned! {span=> &[] },
      format_fn: |span, items| {
        quote_spanned! {span=> &[ #(#items),* ] }
      },
      span,
    }
  }

  #[must_use]
  pub fn slice() -> Self {
    Self {
      items: Vec::new(),
      default_fn: |span| quote_spanned! {span=> &[] },
      format_fn: |span, items| {
        quote_spanned! {span=> &[ #(#items),* ] }
      },
      span: Span::call_site(),
    }
  }

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
  pub fn with_formatter(mut self, format_fn: fn(Span, &[T]) -> TokenStream2) -> Self {
    self.format_fn = format_fn;
    self
  }

  pub fn set(&mut self, items: Vec<T>) {
    self.items = items;
  }

  pub fn push(&mut self, item: T) {
    self.items.push(item);
  }

  pub fn extend(&mut self, new_items: impl IntoIterator<Item = T>) {
    self.items.extend(new_items);
  }

  pub fn maybe_set(&mut self, items: Option<Vec<T>>) {
    if let Some(items) = items {
      self.items = items;
    }
  }

  #[must_use]
  pub const fn is_empty(&self) -> bool {
    self.items.is_empty()
  }
}

impl<T: ToTokens> ToTokens for IterTokensOr<T> {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    if self.items.is_empty() {
      tokens.extend((self.default_fn)(self.span));
    } else {
      (self.format_fn)(self.span, &self.items);
    }
  }
}
