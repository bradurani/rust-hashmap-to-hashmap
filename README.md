# HashMap to HashMap

Adds a method .map to HashMap that maps to another HashMap

```rust

        let brad = Person {
            id: 1,
            first_name: "Brad",
            last_name: "Urani",
        };
        let susan = Person {
            id: 2,
            first_name: "Susan",
            last_name: "Urani",
        };
        let mut h: HashMap<i32, Person> = HashMap::new();
        h.insert(12, brad);
        h.insert(29, susan);

        let new_map = h.map(|k, v| v.id,
                            |k, v| k.to_string() + &"-" + &v.first_name + &" " + &v.last_name);
        // { 1: "12-Brad Urani", 2: "29-Susan Urani" }
```
