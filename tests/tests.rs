#![allow(dead_code)]

use runtime_struct_field_names_as_array::FieldNamesAsArray;

#[derive(FieldNamesAsArray)]
struct Parent<T> {
  parent1: Option<T>,
  parent2: T,
}

#[test]
fn test_with_parents() {
  #[derive(FieldNamesAsArray)]
  struct Test<T> {
    test1: T,
    test2: i64,
    test3: T,
    #[field_names_as_array(flatten)]
    test4: Parent<T>,
    #[field_names_as_array(flatten)]
    test5: Option<Parent<T>>,
  }

  assert_eq!(
    Test::<String>::field_names_as_array(),
    [
      "test1",
      "test2",
      "test3",
      "test4.parent1",
      "test4.parent2",
      "test5.parent1",
      "test5.parent2"
    ]
  );
}

#[test]
fn test_without_parents() {
  #[derive(FieldNamesAsArray)]
  struct Test<T> {
    test1: T,
    test2: i64,
    test3: T,
    test4: Parent<T>,
    test5: Option<Parent<T>>,
  }

  assert_eq!(
    Test::<String>::field_names_as_array(),
    ["test1", "test2", "test3", "test4", "test5"]
  );
}

#[test]
fn test_mixed() {
  #[derive(FieldNamesAsArray)]
  struct Test<T> {
    test1: T,
    test2: i64,
    test3: T,
    test4: Parent<T>,
    #[field_names_as_array(flatten)]
    test5: Option<Parent<T>>,
  }

  assert_eq!(
    Test::<String>::field_names_as_array(),
    [
      "test1",
      "test2",
      "test3",
      "test4",
      "test5.parent1",
      "test5.parent2",
    ]
  );
}
