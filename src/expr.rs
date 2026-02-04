use crate::*;

pub trait ExprExt {
  #[allow(private_interfaces)]
  const SEALED: Sealed;

  fn as_string(&self) -> syn::Result<String>;
  fn as_path(&self) -> syn::Result<&Path>;
  fn as_int<N>(&self) -> syn::Result<N>
  where
    N: FromStr,
    N::Err: Display;
  fn as_closure(&self) -> syn::Result<&ExprClosure>;
  fn as_closure_or_expr(&self) -> ClosureOrExpr;
  fn as_call(&self) -> syn::Result<&ExprCall>;
  fn as_path_or_closure(&self) -> syn::Result<PathOrClosure>;
  fn as_range(&self) -> syn::Result<&ExprRange>;
}

impl ExprExt for Expr {
  #[allow(private_interfaces)]
  const SEALED: Sealed = Sealed;

  fn as_range(&self) -> syn::Result<&ExprRange> {
    if let Self::Range(range) = &self {
      Ok(range)
    } else {
      Err(error!(self, "Expected a range expression"))
    }
  }

  fn as_path_or_closure(&self) -> syn::Result<PathOrClosure> {
    match self {
      Self::Closure(closure) => Ok(PathOrClosure::Closure(closure.to_token_stream())),
      Self::Path(expr_path) => Ok(PathOrClosure::Path(expr_path.path.clone())),
      _ => Err(error!(self, "Expected a path or a closure")),
    }
  }

  fn as_call(&self) -> syn::Result<&ExprCall> {
    if let Self::Call(call) = self {
      Ok(call)
    } else {
      Err(error!(self, "Expected a function call"))
    }
  }
  fn as_string(&self) -> syn::Result<String> {
    if let Self::Lit(expr_lit) = self
      && let Lit::Str(value) = &expr_lit.lit
    {
      Ok(value.value())
    } else {
      Err(error!(self, "Expected a string literal"))
    }
  }

  fn as_path(&self) -> syn::Result<&Path> {
    if let Self::Path(expr_path) = self {
      Ok(&expr_path.path)
    } else {
      Err(error!(self, "Expected a path"))
    }
  }

  fn as_int<N>(&self) -> syn::Result<N>
  where
    N: FromStr,
    N::Err: Display,
  {
    if let Self::Lit(expr_lit) = self
      && let Lit::Int(value) = &expr_lit.lit
    {
      Ok(value.base10_parse::<N>()?)
    } else {
      Err(error!(self, "Expected an integer literal"))
    }
  }

  fn as_closure(&self) -> syn::Result<&ExprClosure> {
    if let Self::Closure(closure) = self {
      Ok(closure)
    } else {
      Err(error!(self, "Expected a closure"))
    }
  }

  fn as_closure_or_expr(&self) -> ClosureOrExpr {
    match self {
      Self::Closure(closure) => ClosureOrExpr::Closure(closure.to_token_stream()),
      _ => ClosureOrExpr::Expr(self.to_token_stream()),
    }
  }
}
