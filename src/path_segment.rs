use crate::*;

pub trait PathSegmentExt {
  fn first_generic(&self) -> Option<GenericArgument> {
    self
      .generic_args()
      .and_then(|args| args.first().cloned())
  }
  fn first_two_generics(&self) -> Option<(GenericArgument, GenericArgument)> {
    self.generic_args().map(|args| {
      (
        args.first().cloned().unwrap(),
        args.get(1).cloned().unwrap(),
      )
    })
  }
  fn generic_args(&self) -> Option<Vec<GenericArgument>>;
  fn generic_args_mut(&mut self) -> Option<IterMut<'_, GenericArgument>>;
  fn parenthesized_args(&self) -> Option<&ParenthesizedGenericArguments>;
  fn parenthesized_args_mut(&mut self) -> Option<&mut ParenthesizedGenericArguments>;
}

impl PathSegmentExt for PathSegment {
  fn parenthesized_args_mut(&mut self) -> Option<&mut ParenthesizedGenericArguments> {
    match &mut self.arguments {
      PathArguments::None => None,
      PathArguments::AngleBracketed(_) => None,
      PathArguments::Parenthesized(par) => Some(par),
    }
  }

  fn parenthesized_args(&self) -> Option<&ParenthesizedGenericArguments> {
    match &self.arguments {
      PathArguments::None => None,
      PathArguments::AngleBracketed(_) => None,
      PathArguments::Parenthesized(par) => Some(par),
    }
  }

  fn generic_args_mut(&mut self) -> Option<IterMut<'_, GenericArgument>> {
    match &mut self.arguments {
      PathArguments::None => None,
      PathArguments::AngleBracketed(ab) => Some(ab.args.iter_mut()),
      PathArguments::Parenthesized(_) => None,
    }
  }

  fn generic_args(&self) -> Option<Vec<GenericArgument>> {
    match &self.arguments {
      PathArguments::None => None,
      PathArguments::AngleBracketed(ab) => Some(ab.args.clone().into_iter().collect()),
      PathArguments::Parenthesized(_) => None,
    }
  }
}
