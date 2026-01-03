use crate::*;

pub trait ParseNestedMetaExt {
  fn is_path(&self) -> bool;
  fn is_name_value(&self) -> bool;
  fn is_list(&self) -> bool;
  fn meta_type(&self) -> MetaType;
  fn parse_value<T: Parse>(&self) -> syn::Result<T>;
  fn expr_value(&self) -> syn::Result<Expr>;
  fn parse_list<T: Parse>(&self) -> syn::Result<T>;
  fn ident(&self) -> syn::Result<&Ident>;
  fn ident_str(&self) -> syn::Result<String>;
  fn parse_inner_value<T, F>(&self, f: F) -> syn::Result<T>
  where
    F: FnMut(ParseNestedMeta) -> syn::Result<T>;
}

#[derive(Clone, Copy)]
pub enum MetaType {
  Path,
  List,
  NameValue,
}

impl ParseNestedMetaExt for ParseNestedMeta<'_> {
  fn ident_str(&self) -> syn::Result<String> {
    self.ident().map(|id| id.to_string())
  }

  fn ident(&self) -> syn::Result<&Ident> {
    self.path.require_ident()
  }

  fn parse_inner_value<T, F>(&self, mut f: F) -> syn::Result<T>
  where
    F: FnMut(ParseNestedMeta) -> syn::Result<T>,
  {
    let mut value: Option<T> = None;

    self.parse_nested_meta(|meta| {
      if value.is_some() {
        return Err(meta.error("duplicate value"));
      }

      value = Some(f(meta)?);
      Ok(())
    })?;

    let value = value.ok_or_else(|| self.error("Tried to parse empty nested meta"))?;

    Ok(value)
  }

  fn parse_list<T: Parse>(&self) -> syn::Result<T> {
    let content;
    syn::parenthesized!(content in self.input);
    content.parse::<T>()
  }

  fn expr_value(&self) -> syn::Result<Expr> {
    self.value()?.parse::<Expr>()
  }

  fn parse_value<T: Parse>(&self) -> syn::Result<T> {
    self.value()?.parse::<T>()
  }

  fn meta_type(&self) -> MetaType {
    if self.is_list() {
      MetaType::List
    } else if self.is_name_value() {
      MetaType::NameValue
    } else {
      MetaType::Path
    }
  }

  fn is_list(&self) -> bool {
    self.input.peek(token::Paren)
  }

  fn is_path(&self) -> bool {
    self.input.is_empty()
  }

  fn is_name_value(&self) -> bool {
    self.input.peek(Token![=])
  }
}

pub fn parse_filtered_attrs<F>(
  attrs: &[Attribute],
  allowed_idents: &[&str],
  mut f: F,
) -> syn::Result<()>
where
  F: FnMut(ParseNestedMeta) -> syn::Result<()>,
{
  for attr in attrs.iter().filter(|attr| {
    attr
      .path()
      .get_ident()
      .is_some_and(|i| allowed_idents.contains(&i.to_string().as_str()))
  }) {
    attr.parse_nested_meta(&mut f)?;
  }

  Ok(())
}

pub fn filter_attributes(attrs: &[Attribute], allowed_idents: &[&str]) -> syn::Result<Vec<Meta>> {
  let mut metas = Vec::new();

  for attr in attrs {
    let attr_ident = if let Some(ident) = attr.path().get_ident() {
      ident.to_string()
    } else {
      continue;
    };

    if !allowed_idents.contains(&attr_ident.as_str()) {
      continue;
    }

    let parser = |input: ParseStream| -> syn::Result<()> {
      while !input.is_empty() {
        let meta: Meta = input.parse()?;
        metas.push(meta);

        if input.is_empty() {
          break;
        }
        let _: Token![,] = input.parse()?;
      }
      Ok(())
    };

    attr.parse_args_with(parser)?;
  }

  Ok(metas)
}
