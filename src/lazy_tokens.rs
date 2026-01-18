use std::cell::OnceCell;

use proc_macro2::{Punct, Spacing, TokenTree};

use crate::*;

pub struct CrateName {
  name: &'static str,
}

impl CrateName {
  #[must_use]
  #[inline]
  pub const fn new(name: &'static str) -> Self {
    Self { name }
  }
}

impl ToTokens for CrateName {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let ident = new_ident(self.name);

    tokens.append(TokenTree::Punct(Punct::new(':', Spacing::Joint)));
    tokens.append(TokenTree::Punct(Punct::new(':', Spacing::Alone)));
    tokens.append(TokenTree::Ident(ident));
  }
}

pub struct LazyPath {
  segments: &'static [&'static str],
}

impl LazyPath {
  #[must_use]
  #[inline]
  pub const fn new(segments: &'static [&'static str]) -> Self {
    Self { segments }
  }
}

impl ToTokens for LazyPath {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    for segment in self.segments {
      tokens.append(TokenTree::Punct(Punct::new(':', Spacing::Joint)));
      tokens.append(TokenTree::Punct(Punct::new(':', Spacing::Alone)));
      tokens.append(TokenTree::Ident(new_ident(segment)));
    }
  }
}

pub struct LazyTokens<F> {
  init: F,
  tokens: OnceCell<TokenStream2>,
}

impl<F> LazyTokens<F>
where
  F: Fn() -> TokenStream2,
{
  #[inline]
  pub const fn new(init: F) -> Self {
    Self {
      init,
      tokens: OnceCell::new(),
    }
  }
}

impl<F> ToTokens for LazyTokens<F>
where
  F: Fn() -> TokenStream2,
{
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let cached_tokens = self.tokens.get_or_init(|| (self.init)());

    cached_tokens.to_tokens(tokens);
  }
}
