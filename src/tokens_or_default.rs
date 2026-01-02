use crate::*;

#[derive(Debug, Clone)]
pub struct TokensOr<T: ToTokens> {
  pub tokens: Option<T>,
  pub default_fn: fn() -> TokenStream2,
  pub format_fn: fn(&T, &mut TokenStream2),
}

impl<T: ToTokens> ToTokens for TokensOr<T> {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    if let Some(inner) = &self.tokens {
      (self.format_fn)(inner, tokens);
    } else {
      tokens.extend((self.default_fn)());
    }
  }
}

impl<T: ToTokens> TokensOr<T> {
  pub fn new(default: fn() -> TokenStream2) -> Self {
    Self {
      tokens: None,
      default_fn: default,
      format_fn: |val, tokens| val.to_tokens(tokens),
    }
  }

  pub fn option() -> Self {
    Self {
      tokens: None,
      default_fn: || quote! { None },
      format_fn: |val, tokens| tokens.extend(quote! { Some(#val) }),
    }
  }

  pub fn custom(default_fn: fn() -> TokenStream2, format_fn: fn(&T, &mut TokenStream2)) -> Self {
    Self {
      tokens: None,
      default_fn,
      format_fn,
    }
  }

  pub fn with_formatter(mut self, format_fn: fn(&T, &mut TokenStream2)) -> Self {
    self.format_fn = format_fn;
    self
  }

  pub fn set(&mut self, tokens: T) {
    self.tokens = Some(tokens);
  }

  pub fn maybe_set(&mut self, tokens: Option<T>) {
    self.tokens = tokens;
  }

  pub fn is_default(&self) -> bool {
    self.tokens.is_none()
  }
}

#[derive(Debug, Clone)]
pub struct IterTokensOr<T: ToTokens> {
  pub items: Option<Vec<T>>,
  pub default_fn: fn() -> TokenStream2,
  pub format_fn: fn(&Vec<T>, &mut TokenStream2),
}

impl<T: ToTokens> IterTokensOr<T> {
  pub fn vec() -> Self {
    Self {
      items: None,
      default_fn: || quote! { vec![] },
      format_fn: |items, tokens| {
        tokens.extend(quote! { vec![ #(#items),* ] });
      },
    }
  }

  pub fn slice() -> Self {
    Self {
      items: None,
      default_fn: || quote! { &[] },
      format_fn: |items, tokens| {
        tokens.extend(quote! { &[ #(#items),* ] });
      },
    }
  }

  pub fn custom(default: fn() -> TokenStream2, formatter: fn(&Vec<T>, &mut TokenStream2)) -> Self {
    Self {
      items: None,
      default_fn: default,
      format_fn: formatter,
    }
  }

  pub fn with_formatter(mut self, format_fn: fn(&Vec<T>, &mut TokenStream2)) -> Self {
    self.format_fn = format_fn;
    self
  }

  pub fn set(&mut self, items: Vec<T>) {
    self.items = Some(items);
  }

  pub fn push(&mut self, item: T) {
    let items = self.items.get_or_insert_default();
    items.push(item);
  }

  pub fn extend(&mut self, new_items: impl IntoIterator<Item = T>) {
    let items = self.items.get_or_insert_default();
    items.extend(new_items);
  }

  pub fn maybe_set(&mut self, items: Option<Vec<T>>) {
    self.items = items;
  }

  pub fn is_default(&self) -> bool {
    self
      .items
      .as_ref()
      .is_none_or(|items| items.is_empty())
  }
}

impl<T: ToTokens> ToTokens for IterTokensOr<T> {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    if let Some(items) = &self.items {
      (self.format_fn)(items, tokens);
    } else {
      tokens.extend((self.default_fn)());
    }
  }
}
