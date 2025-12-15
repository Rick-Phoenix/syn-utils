use quote::ToTokens;
use syn::Type;
use syn_utils::TypeInfo;

fn get_inner(s: &str) -> String {
  let ty: Type = syn::parse_str(s).expect("Invalid Rust syntax");
  let info = TypeInfo::from_type(&ty).unwrap();

  info
    .inner()
    .to_token_stream()
    .to_string()
    .replace(" ", "")
}

#[test]
fn test_primitives_return_self() {
  assert_eq!(get_inner("i32"), "i32");
  assert_eq!(get_inner("String"), "String");
  assert_eq!(get_inner("bool"), "bool");
  assert_eq!(get_inner("f64"), "f64");
}

#[test]
fn test_paths_return_self() {
  assert_eq!(get_inner("MyStruct"), "MyStruct");
  assert_eq!(get_inner("crate::module::MyType"), "crate::module::MyType");
}

#[test]
fn test_wrappers() {
  assert_eq!(get_inner("::core::option::Option<i32>"), "i32");
  assert_eq!(get_inner("Vec<String>"), "String");
  assert_eq!(get_inner("Box<MyStruct>"), "MyStruct");
}

#[test]
fn test_nested_wrappers() {
  assert_eq!(get_inner("::core::option::Option<Vec<i32>>"), "Vec<i32>");
  assert_eq!(
    get_inner("Box<::core::option::Option<MyStruct>>"),
    "::core::option::Option<MyStruct>"
  );
  assert_eq!(get_inner("Vec<Vec<Vec<u8>>>"), "Vec<Vec<u8>>");
}

#[test]
fn test_arrays_and_slices() {
  assert_eq!(get_inner("[i32]"), "i32");
  assert_eq!(get_inner("[MyStruct; 16]"), "MyStruct");
  assert_eq!(get_inner("&[&[u8]]"), "&[u8]");
}

#[test]
fn test_hashmap_returns_self() {
  assert_eq!(
    get_inner("HashMap<::core::option::Option<MyKey>, Vec<MyValue>>"),
    "HashMap<::core::option::Option<MyKey>,Vec<MyValue>>"
  );
}

#[test]
fn test_complex_wrapper() {
  assert_eq!(
    get_inner("::core::option::Option<&mut [Box<MyStruct>]>"),
    "&mut[Box<MyStruct>]"
  );
}

fn get_path(s: &str) -> Option<String> {
  let ty: Type = syn::parse_str(s).expect("Invalid Rust syntax");
  let info = TypeInfo::from_type(&ty).unwrap();

  let path = info.as_path();

  path.map(|p| p.into_token_stream().to_string().replace(" ", ""))
}

#[test]
fn test_references_are_stripped() {
  assert_eq!(get_path("&MyStruct").unwrap(), "MyStruct");
  assert_eq!(get_path("&mut i32").unwrap(), "i32");
}

#[test]
fn test_invalid_paths() {
  assert!(get_path("(String, i32)").is_none());
  assert!(get_path("&[u8]").is_none());
  assert!(get_path("[u8]").is_none());
}
