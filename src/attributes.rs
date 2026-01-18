use crate::*;

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
