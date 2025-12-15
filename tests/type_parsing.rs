use quote::{quote, ToTokens};
use syn::Type;
use syn_utils::{Array, RefKind, RustType, TypeInfo};

fn assert_round_trip(input_str: &str) {
  let original_type: Type = syn::parse_str(input_str).expect("Invalid Rust syntax in test");

  let info = TypeInfo::from_type(&original_type).unwrap();

  let tokens = info.to_token_stream();

  let output_str = tokens.to_string();

  let normalized_input = quote!(#original_type).to_string();

  assert_eq!(
    output_str, normalized_input,
    "Round trip failed for: {}",
    input_str
  );
}

fn assert_is_option(ty: &str) {
  let original: Type = syn::parse_str(ty).unwrap();
  let info = TypeInfo::from_type(&original).unwrap();

  if let RustType::Option(_) = *info.type_ {
    // OK
  } else {
    panic!("Expected Option, got {:?}", info.type_);
  }
}

#[test]
fn test_primitives() {
  assert_round_trip("i32");
  assert_round_trip("String");
  assert_round_trip("bool");
}

#[test]
fn test_references() {
  assert_round_trip("&i32");
  assert_round_trip("&mut String");
  assert_round_trip("&'a [u8]");
  assert_round_trip("&'static mut Vec<i32>");
}

#[test]
fn test_wrappers() {
  assert_round_trip("::core::option::Option<i32>");
  assert_round_trip("Vec<String>");
  assert_round_trip("Box<MyStruct>");
  assert_round_trip("HashMap<String, i32>");

  assert_is_option("::core::option::Option<i32>");
}

#[test]
fn test_nested_complex() {
  assert_round_trip("::core::option::Option<Vec<Box<i32>>>");
  assert_round_trip("&mut HashMap<String, ::core::option::Option<Vec<u8>>>");
}

#[test]
fn test_arrays_and_tuples() {
  assert_round_trip("[u8; 4]");
  assert_round_trip("(&str, i32, ::core::option::Option<bool>)");
}

#[test]
fn test_slices() {
  assert_round_trip("[u8]");
  assert_round_trip("&[i32]");
}

fn get_info(s: &str) -> TypeInfo {
  let ty: Type = syn::parse_str(s).unwrap_or_else(|_| panic!("Failed to parse rust syntax: {}", s));
  TypeInfo::from_type(&ty).unwrap()
}

fn assert_inner_eq(info: &TypeInfo, expected: &str) {
  let tokens = info.to_token_stream().to_string();
  assert_eq!(tokens, expected, "Inner type mismatch");
}

#[test]
fn test_wrappers_option() {
  let info = get_info("Option<i32>");

  if let RustType::Option(inner) = &*info.type_ {
    assert_inner_eq(inner, "i32");
  } else {
    panic!("Failed to parse Option. Got different variant.");
  }
}

#[test]
fn test_wrappers_vec() {
  let info = get_info("Vec<String>");

  if let RustType::Vec(inner) = &*info.type_ {
    assert_inner_eq(inner, "String");
  } else {
    panic!("Failed to parse Vec.");
  }
}

#[test]
fn test_wrappers_box() {
  let info = get_info("Box<MyStruct>");

  if let RustType::Box(inner) = &*info.type_ {
    assert_inner_eq(inner, "MyStruct");
  } else {
    panic!("Failed to parse Box.");
  }
}

#[test]
fn test_hashmap() {
  let info = get_info("HashMap<String, i32>");

  if let RustType::HashMap((k, v)) = &*info.type_ {
    assert_inner_eq(k, "String");
    assert_inner_eq(v, "i32");
  } else {
    panic!("Failed to parse HashMap. Got type {:#?}", info.type_);
  }
}

#[test]
fn test_slice() {
  let info = get_info("[u8]");

  if let RustType::Slice(inner) = &*info.type_ {
    assert_inner_eq(inner, "u8");
  } else {
    panic!("Failed to parse Slice.");
  }
}

#[test]
fn test_array() {
  let info = get_info("[u8; 4]");

  if let RustType::Array(array) = &*info.type_ {
    let Array { len, inner } = array.as_ref();
    assert_inner_eq(inner, "u8");
    let len_str = quote!(#len).to_string();
    assert_eq!(len_str, "4");
  } else {
    panic!("Failed to parse Array.");
  }
}

#[test]
fn test_tuple() {
  let info = get_info("(i32, bool, String)");

  if let RustType::Tuple(items) = &*info.type_ {
    assert_eq!(items.len(), 3);
    assert_inner_eq(&items[0], "i32");
    assert_inner_eq(&items[1], "bool");
    assert_inner_eq(&items[2], "String");
  } else {
    panic!("Failed to parse Tuple.");
  }
}

#[test]
fn test_nested_wrappers() {
  let info = get_info("Option<Vec<Box<i32>>>");

  if let RustType::Option(l1) = &*info.type_
      && let RustType::Vec(l2) = &*l1.type_
        && let RustType::Box(l3) = &*l2.type_ {
          assert_inner_eq(l3, "i32");
          return;
        }
  panic!("Failed to parse deeply nested wrappers");
}

#[test]
fn test_references_mut() {
  let info = get_info("&mut String");

  assert!(info.reference.is_some(), "Should be a reference");
  let ref_data = info.reference.unwrap();

  match ref_data.kind {
    RefKind::MutRef => {}
    _ => panic!("Expected MutRef"),
  }

  if let RustType::Other(path) = &*info.type_ {
    let path_str = quote!(#path).to_string();
    assert_eq!(path_str, "String");
  } else {
    panic!("Inner type should be Other(String)");
  }
}

#[test]
fn test_references_const() {
  let info = get_info("&i32");

  let ref_data = info.reference.expect("Should be a reference");
  match ref_data.kind {
    RefKind::Ref => {}
    _ => panic!("Expected Ref (const)"),
  }
}

#[test]
fn test_other_path() {
  let info = get_info("my_crate::MyType");

  if let RustType::Other(path) = &*info.type_ {
    let s = quote::quote!(#path).to_string().replace(" ", "");
    assert_eq!(s, "my_crate::MyType");
  } else {
    panic!("Should have parsed as Other/Path");
  }
}
