# Enum with data to HashMap

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
    #[derive(Debug, PartialEq, Eq, Clone, DeriveStyleKeys)]
    pub enum TestValue {
        Padding(usize),
        Margin(String),
    }
```

This will generate getters and setters for each enum value as well as a generic getter and setter provided a key.

# How it works


