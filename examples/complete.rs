use enum2map::Enum2Map;

fn main() {
    // First define your enum that you want to convert into a map
    #[derive(Debug, PartialEq, Eq, Clone, Enum2Map)]
    pub enum TestValue {
        Padding(usize),
        Margin(String),
    }

    let mut map = TestValueMap::new();
    // Test generic getter
    println!("Getter with nothing in the map: {:?}", map.get(TestValueKey::Padding));
    println!("Getter with nothing in the map: {:?}", map.get(TestValueKey::Margin));

    // Default Values
    println!("Getter with default value: {:?}", map.get_or_default(TestValueKey::Padding));
    println!("Getter with default value: {:?}", map.get_or_default(TestValueKey::Margin));
    println!("Getter with default value {:?}", map.get_padding());
    println!("Getter with default value {:?}", map.get_margin());

    // Test generic setters and getters generated by the macro
    println!("Generic setter, should return None: {:?}", map.set(TestValue::Padding(10)));
    println!("Generic setter, should return None: {:?}", map.set(TestValue::Margin("string test".to_string())));
    println!("Generic setter, should return Some(Padding(10)): {:?}", map.set(TestValue::Padding(50)));
    println!("Generic setter, should return Some(Margin('string test')): {:?}", map.set(TestValue::Margin("another test".to_string())));

    println!("Generic getter, should return Some(Padding(50)): {:?}", map.get(TestValueKey::Padding));
    println!("Generic getter, shoudl return Some(Margin('another test')): {:?}", map.get(TestValueKey::Margin));

    // Test setters and getters generated by the macro
    println!("Setter, should return Some(50): {:?}", map.set_padding(70));
    println!("Setter, should return Some('another test'): {:?}", map.set_margin("final test".to_string()));
    println!("Getter should return 70: {:?}", map.get_padding());
    println!("Getter should return 'final test' {:?}", map.get_margin());
}
