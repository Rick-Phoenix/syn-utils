use crate::*;

pub trait ParseExpr {
  fn parse_string(&self) -> syn::Result<String>;
  fn parse_path(&self) -> syn::Result<&Path>;
  fn parse_int<N>(&self) -> syn::Result<N>
  where
    N: FromStr,
    N::Err: Display;
  fn parse_closure(&self) -> syn::Result<&ExprClosure>;
  fn parse_call_or_closure(self) -> syn::Result<CallOrClosure>;
  fn parse_call(&self) -> syn::Result<&ExprCall>;
}

impl ParseExpr for Expr {
  fn parse_call(&self) -> syn::Result<&ExprCall> {
    if let Expr::Call(call) = self {
      Ok(call)
    } else {
      Err(error!(self, "Expected a function call"))
    }
  }
  fn parse_string(&self) -> syn::Result<String> {
    if let Expr::Lit(expr_lit) = self && let Lit::Str(value) = &expr_lit.lit {
      Ok(value.value())
    } else {
      Err(error!(self, "Expected a string literal"))
    }
  }

  fn parse_path(&self) -> syn::Result<&Path> {
    if let Expr::Path(expr_path) = self {
      Ok(&expr_path.path)
    } else {
      Err(error!(self, "Expected a path"))
    }
  }

  fn parse_int<N>(&self) -> syn::Result<N>
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

  fn parse_closure(&self) -> syn::Result<&ExprClosure> {
    if let Expr::Closure(closure) = self {
      Ok(closure)
    } else {
      Err(error!(self, "Expected a closure"))
    }
  }

  fn parse_call_or_closure(self) -> syn::Result<CallOrClosure> {
    match self {
      Expr::Closure(closure) => Ok(CallOrClosure::Closure(closure)),
      Expr::Call(call) => Ok(CallOrClosure::Call(call)),
      _ => Err(error!(self, "Expected a path or a closure")),
    }
  }
}
