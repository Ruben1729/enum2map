use enum2map::Enum2Map;

// First define your enum that you want to convert into a map
#[derive(Debug, PartialEq, Eq, Clone, Enum2Map)]
pub enum TestValue {
    Padding(usize),
    Margin(String),
}

#[test]
fn complete_test() {
    let mut map = TestValueMap::new();
    // Test generic getter
    assert_eq!(map.get(TestValueKey::Padding), None);
    assert_eq!(map.get(TestValueKey::Margin), None);

    // Default Values
    assert_eq!(map.get_or_default(TestValueKey::Padding), TestValue::Padding(0));
    assert_eq!(map.get_or_default(TestValueKey::Margin), TestValue::Margin("".to_string()));
    assert_eq!(map.get_padding(), 0);
    assert_eq!(map.get_margin(), "".to_string());

    // Test generic setters and getters generated by the macro
    assert_eq!(map.set(TestValue::Padding(10)), None);
    assert_eq!(map.set(TestValue::Margin("string test".to_string())), None);
    assert_eq!(map.set(TestValue::Padding(50)), Some(TestValue::Padding(10)));
    assert_eq!(map.set(TestValue::Margin("another test".to_string())), Some(TestValue::Margin("string test".to_string())));

    assert_eq!(map.get(TestValueKey::Padding).unwrap(), &TestValue::Padding(50));
    assert_eq!(
        map.get(TestValueKey::Margin).unwrap(),
        &TestValue::Margin("another test".to_string())
    );

    map.set_padding(50);
    map.set_margin("another test".to_string());

    // Test setters and getters generated by the macro
    // assert_eq!(map.set_padding(50), Some(TestValue::Padding(50)));
    // assert_eq!(map.set_margin("another test".to_string()), Some(TestValue::Margin("another_test".to_string())));
    assert_eq!(map.get_padding(), 50);
    assert_eq!(map.get_margin(), "another test".to_string());
}

#[test]
#[should_panic(expected = "Unexpected condition: Didn't find type String for Margin")]
fn panic_test() {
    let mut map = TestValueMap::new();

    // Should throw a panic if the developer tries inserting a key with the wrong value type
    map.values.insert(TestValueKey::Margin, TestValue::Padding(10));
    map.get_margin();
}
