use std::{collections::HashMap, hash::Hash};

pub fn push_to_kv_vec<K, V>(store: &mut HashMap<K, Vec<V>>, key: K, value: V)
where
    K: Hash + Eq,
{
    if let Some(vec) = store.get_mut(&key) {
        vec.push(value);
        return;
    }
    store.insert(key, vec![value]);
}
#[cfg(test)]
mod test_store_fn {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_push_to_kv_vec() {
        let mut store = HashMap::new();
        store.insert("test", vec!["value1"]);
        push_to_kv_vec(&mut store, "test", "value2");
        assert_eq!(store.get("test").unwrap(), &vec!["value1", "value2"]);
    }
}
