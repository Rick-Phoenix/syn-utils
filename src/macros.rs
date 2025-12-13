#[macro_export]
macro_rules! bail {
  ($item:expr, $fmt:literal $(, $args:expr)* $(,)?) => {
    return Err(syn::Error::new_spanned(
      &$item,
      format!($fmt $(, $args)*)
    ))
  };
}

#[macro_export]
macro_rules! bail_with_span {
  ($span:expr, $fmt:literal $(, $args:expr)* $(,)?) => {
    return Err(syn::Error::new(
      $span,
      format!($fmt $(, $args)*)
    ))
  };
}

#[macro_export]
macro_rules! bail_call_site {
  ($fmt:literal $(, $args:expr)* $(,)?) => {
    return Err(syn::Error::new(
      proc_macro2::Span::call_site(),
      format!($fmt $(, $args)*)
    ))
  };
}

#[macro_export]
macro_rules! error {
  ($item:expr, $fmt:literal $(, $args:expr)* $(,)?) => {
    syn::Error::new_spanned(
      &$item,
      format!($fmt $(, $args)*)
    )
  };
}

#[macro_export]
macro_rules! error_with_span {
  ($span:expr, $fmt:literal $(, $args:expr)* $(,)?) => {
    syn::Error::new(
      $span,
      format!($fmt $(, $args)*)
    )
  };
}

#[macro_export]
macro_rules! error_call_site {
  ($fmt:literal $(, $args:expr)* $(,)?) => {
    syn::Error::new(
      proc_macro2::Span::call_site(),
      format!($fmt $(, $args)*)
    )
  };
}
