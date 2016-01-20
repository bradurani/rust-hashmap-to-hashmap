use std::collections::HashMap;
use std::hash::Hash;

pub trait Map<PK: Hash+Eq, PV, NK: Hash+Eq, NV, FK: Fn(&PK, &PV)->NK, FV: Fn(&PK, &PV)->NV> {
    fn map(self, key_func: FK, value_func: FV) -> HashMap<NK, NV>;
}

impl <PK: Hash+Eq, PV, NK: Hash+Eq, NV, FK: Fn(&PK, &PV)->NK, FV: Fn(&PK, &PV)->NV> Map<PK, PV, NK, NV, FK, FV> for HashMap<PK, PV> {
    fn map(self, key_func: FK, value_func: FV) -> HashMap<NK, NV> {
        let mut new_map: HashMap<NK, NV> = HashMap::new();
        for (key, value) in self {
            new_map.insert(key_func(&key, &value), value_func(&key, &value));
        }
        new_map
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    struct Person<'s> {
        id: i32,
        first_name: &'s str,
        last_name: &'s str,
    }

    #[test]
    fn it_works() {
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

        let mut expected = HashMap::new();
        expected.insert(1, "Brad Urani");
        expected.insert(2, "Susan Urani");

        let new_map = h.map(|_, v| v.id,
                            |_, v| v.first_name.to_string() + &" " + v.last_name);
        assert_eq!(new_map.get(&1).unwrap(), expected.get(&1).unwrap());
        assert_eq!(new_map.get(&2).unwrap(), expected.get(&2).unwrap());
    }

}
