use crate::*;

pub trait PathSegmentExt {
  #[allow(private_interfaces)]
  const SEALED: Sealed;

  fn first_generic(&self) -> Option<&GenericArgument>;
  fn first_generic_mut(&mut self) -> Option<&mut GenericArgument>;
  fn last_generic(&self) -> Option<&GenericArgument>;
  fn last_generic_mut(&mut self) -> Option<&mut GenericArgument>;
  fn first_two_generics(&self) -> Option<(&GenericArgument, &GenericArgument)>;
  fn generic_args(&self) -> Option<&Punctuated<GenericArgument, Token![,]>>;
  fn generic_args_mut(&mut self) -> Option<&mut Punctuated<GenericArgument, Token![,]>>;
}

impl PathSegmentExt for PathSegment {
  #[allow(private_interfaces)]
  const SEALED: Sealed = Sealed;

  #[inline]
  fn first_generic(&self) -> Option<&GenericArgument> {
    match &self.arguments {
      PathArguments::AngleBracketed(args) => args.args.first(),
      PathArguments::None | PathArguments::Parenthesized(_) => None,
    }
  }

  #[inline]
  fn first_generic_mut(&mut self) -> Option<&mut GenericArgument> {
    match &mut self.arguments {
      PathArguments::AngleBracketed(args) => args.args.first_mut(),
      PathArguments::None | PathArguments::Parenthesized(_) => None,
    }
  }

  #[inline]
  fn last_generic(&self) -> Option<&GenericArgument> {
    match &self.arguments {
      PathArguments::AngleBracketed(args) => args.args.last(),
      PathArguments::None | PathArguments::Parenthesized(_) => None,
    }
  }

  #[inline]
  fn last_generic_mut(&mut self) -> Option<&mut GenericArgument> {
    match &mut self.arguments {
      PathArguments::AngleBracketed(args) => args.args.last_mut(),
      PathArguments::None | PathArguments::Parenthesized(_) => None,
    }
  }

  #[inline]
  fn first_two_generics(&self) -> Option<(&GenericArgument, &GenericArgument)> {
    match &self.arguments {
      PathArguments::AngleBracketed(args) => {
        if args.args.len() > 1 {
          Some((args.args.first().unwrap(), args.args.get(1).unwrap()))
        } else {
          None
        }
      }
      PathArguments::None | PathArguments::Parenthesized(_) => None,
    }
  }

  #[inline]
  fn generic_args_mut(&mut self) -> Option<&mut Punctuated<GenericArgument, Token![,]>> {
    match &mut self.arguments {
      PathArguments::AngleBracketed(ab) => Some(&mut ab.args),
      PathArguments::None | PathArguments::Parenthesized(_) => None,
    }
  }

  #[inline]
  fn generic_args(&self) -> Option<&Punctuated<GenericArgument, Token![,]>> {
    match &self.arguments {
      PathArguments::AngleBracketed(ab) => Some(&ab.args),
      PathArguments::None | PathArguments::Parenthesized(_) => None,
    }
  }
}
