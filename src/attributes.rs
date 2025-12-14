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
