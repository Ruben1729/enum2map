use enum2map::DeriveStyleKeys;

fn main() {
    // First define your enum that you want to convert into a map
    #[derive(Debug, PartialEq, Eq, Clone, DeriveStyleKeys)]
    pub enum TestValue {
        Padding(usize),
        Margin(String),
    }

    let mut map = TestValueMap::new();

    map.get(TestValueKey::Margin);
    map.get(TestValueKey::Padding);

    map.set(TestValue::Padding(10));
    map.set(TestValue::Margin("string test".to_string()));

    map.set_padding(50);
    map.set_margin("another test".to_string());
}