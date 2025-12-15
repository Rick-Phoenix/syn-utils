use crate::*;

pub trait ExprExt {
  fn as_string(&self) -> syn::Result<String>;
  fn as_path(&self) -> syn::Result<&Path>;
  fn as_int<N>(&self) -> syn::Result<N>
  where
    N: FromStr,
    N::Err: Display;
  fn as_closure(&self) -> syn::Result<&ExprClosure>;
  fn as_call_or_closure(&self) -> syn::Result<CallOrClosure>;
  fn as_call(&self) -> syn::Result<&ExprCall>;
  fn as_path_or_closure(&self) -> syn::Result<PathOrClosure>;
}

impl ExprExt for Expr {
  fn as_path_or_closure(&self) -> syn::Result<PathOrClosure> {
    match self {
      Expr::Closure(closure) => Ok(PathOrClosure::Closure(closure.to_token_stream())),
      Expr::Path(expr_path) => Ok(PathOrClosure::Path(expr_path.path.to_token_stream())),
      _ => Err(error!(self, "Expected a path or a closure")),
    }
  }

  fn as_call(&self) -> syn::Result<&ExprCall> {
    if let Expr::Call(call) = self {
      Ok(call)
    } else {
      Err(error!(self, "Expected a function call"))
    }
  }
  fn as_string(&self) -> syn::Result<String> {
    if let Expr::Lit(expr_lit) = self && let Lit::Str(value) = &expr_lit.lit {
      Ok(value.value())
    } else {
      Err(error!(self, "Expected a string literal"))
    }
  }

  fn as_path(&self) -> syn::Result<&Path> {
    if let Expr::Path(expr_path) = self {
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
    if let Expr::Lit(expr_lit) = self && let Lit::Int(value) = &expr_lit.lit {
      Ok(value.base10_parse::<N>()?)
    } else {
      Err(error!(self, "Expected an integer literal"))
    }
  }

  fn as_closure(&self) -> syn::Result<&ExprClosure> {
    if let Expr::Closure(closure) = self {
      Ok(closure)
    } else {
      Err(error!(self, "Expected a closure"))
    }
  }

  fn as_call_or_closure(&self) -> syn::Result<CallOrClosure> {
    match self {
      Expr::Closure(closure) => Ok(CallOrClosure::Closure(closure.to_token_stream())),
      Expr::Call(call) => Ok(CallOrClosure::Call(call.to_token_stream())),
      _ => Err(error!(self, "Expected a function call or a closure")),
    }
  }
}
