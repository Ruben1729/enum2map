# Enum with data to HashMap

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

This crate provides a way to transform an enum with associated data into a hashmap. This idea came to me as I was working on my UI library. 

I wanted to implement styling in a very flexible way. Essentially, every style property is optional unless it's defined. This would have been extremely annoying to implement with a struct as all properties would have had to be an Option<T>. 

With enum2map, I can define an enum with associated data for each key and the crate will take care of turning it into a map with setters and getters for each thing.

```toml
[dependencies]
enum2map = "0.1"
```

# Examples
The way to use the crate is very simple. Define an enum with a bunch data associated to it and derive "DeriveStyleKeys".

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

# How it works

