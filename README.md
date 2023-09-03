# Enum with data to HashMap

[<img alt="github" src="https://img.shields.io/badge/github-ruben1729/enum2map-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/Ruben1729/enum2map)
[<img alt="crates.io" src="https://img.shields.io/crates/v/enum2map.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/enum2map)

This crate provides a way to transform an enum with associated data into a hashmap. This idea came to me as I was working on my UI library. 

I wanted to implement styling in a very flexible way. Essentially, every style property is optional unless it's defined. This would have been extremely annoying to implement with a struct as all properties would have had to be an Option<T>. 

With enum2map, I can define an enum with associated data for each key and the crate will take care of turning it into a map with setters and getters for each thing.

```toml
[dependencies]
enum2map = "0.1"
```

## Examples
The way to use the crate is very simple. Define an enum with a bunch data associated to it and derive "Enum2Map".

```rust
#[derive(Debug, PartialEq, Eq, Clone, Enum2Map)]
pub enum TestValue {
    Padding(usize),
    Margin(String),
}
```

This will generate the generic getters and setters:

```rust
let mut map = TestValueMap::new();

map.get(TestValueKey::Margin);
map.get(TestValueKey::Padding);

map.set(TestValue::Padding(10));
map.set(TestValue::Margin("string test".to_string()));
```

As well as the getters and setters for each key:

```rust
map.set_padding(50);
map.set_margin("another test".to_string());

map.get_padding();
map.get_margin();
```

If the value is not set, the macro will generate a default value from ``Default::default()``.

## How it works
It works by taking every enum value and then using the name to generate the keys and it's getters/setters. For example, this code:

```rust
#[derive(Debug, PartialEq, Eq, Clone, Enum2Map)]
pub enum TestValue {
    Padding(usize),
    Margin(String),
}
```

Will generate:

```rust
pub enum TestValueKey {
    Padding,
    Margin,
}
pub struct TestValueMap {
    pub values: std::collections::HashMap<TestValueKey, TestValue>,
}
impl TestValueMap {
    pub fn new() -> Self {
        Self {
            values: std::collections::HashMap::new(),
        }
    }
    pub fn insert(&mut self, value: TestValue) {
        match value {
            TestValue::Padding(val) => {
                self.values.insert(TestValueKey::Padding, TestValue::Padding(val));
            }
            TestValue::Margin(val) => {
                self.values.insert(TestValueKey::Margin, TestValue::Margin(val));
            }
        }
    }
    pub fn get(&self, key: TestValueKey) -> Option<&TestValue> {
        self.values.get(&key)
    }
    pub fn get_or_default(&self, key: TestValueKey) -> TestValue {
        match self.values.get(&key) {
            Some(value) => value.clone(),
            None => {
                match key {
                    TestValueKey::Padding => TestValue::Padding(Default::default()),
                    TestValueKey::Margin => TestValue::Margin(Default::default()),
                }
            }
        }
    }
    pub fn set(&mut self, value: TestValue) {
        match value {
            TestValue::Padding(val) => {
                self.values.insert(TestValueKey::Padding, TestValue::Padding(val));
            }
            TestValue::Margin(val) => {
                self.values.insert(TestValueKey::Margin, TestValue::Margin(val));
            }
        }
    }
    pub fn get_padding(&self) -> usize {
        match self.values.get(&TestValueKey::Padding) {
            Some(TestValue::Padding(value)) => value.clone(),
            None => Default::default(),
            _ => {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "Unexpected condition: Didn\'t find type {0} for {1}",
                        "usize",
                        "Padding",
                    ),
                );
            }
        }
    }
    pub fn get_margin(&self) -> String {
        match self.values.get(&TestValueKey::Margin) {
            Some(TestValue::Margin(value)) => value.clone(),
            None => Default::default(),
            _ => {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "Unexpected condition: Didn\'t find type {0} for {1}",
                        "String",
                        "Margin",
                    ),
                );
            }
        }
    }
    pub fn set_padding(&mut self, val: usize) {
        self.values.insert(TestValueKey::Padding, TestValue::Padding(val));
    }
    pub fn set_margin(&mut self, val: String) {
        self.values.insert(TestValueKey::Margin, TestValue::Margin(val));
    }
}

```

## Future Work

- [x] Better error handling.
- [ ] Handle cases where the enum has no associated data.
- [ ] Better documentation.
