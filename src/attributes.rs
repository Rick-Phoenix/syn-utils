use crate::*;

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

    let parser = Punctuated::<Meta, Token![,]>::parse_terminated;
    let args = attr.parse_args_with(parser)?;

    metas.extend(args);
  }

  Ok(metas)
}
