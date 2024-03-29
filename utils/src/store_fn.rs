use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
};
pub fn get_tuple_key_store<'a, K1, K2, V>(
    store: &'a HashMap<(K1, K2), V>,
    tuple_fist: &K1,
    tuple_seconde: &K2,
) -> Option<&'a V>
where
    K1: Hash + Eq + PartialEq,
    K2: Hash + Eq + PartialEq,
{
    for (k, v) in store {
        if tuple_fist == &k.0 && tuple_seconde == &k.1 {
            return Some(v);
        }
    }
    None
}

pub fn contains_to_kv_vec<K, V>(store: &HashMap<K, Vec<V>>, key: &K, value: &V) -> bool
where
    K: Hash + Eq,
    V: PartialEq,
{
    store
        .get(key)
        .map(|v| v.contains(value))
        .unwrap_or_default()
}
pub fn push_to_btree_vec<K, V>(store: &mut BTreeMap<K, Vec<V>>, key: K, value: V)
where
    K: Hash + Eq + Ord,
{
    if let Some(vec) = store.get_mut(&key) {
        vec.push(value);
        return;
    }
    store.insert(key, vec![value]);
}
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
    fn test_get_tuple_key_kv() {
        let mut store = HashMap::new();
        store.insert(("Test", "id"), 0);
        store.insert(("TestData", "account_id"), 10);
        assert_eq!(get_tuple_key_store(&store, &"Test", &"id"), Some(&0));
        assert_eq!(get_tuple_key_store(&store, &"Test", &"account_id"), None);
        assert_eq!(
            get_tuple_key_store(&store, &"TestData", &"account_id"),
            Some(&10)
        );
        assert_eq!(get_tuple_key_store(&store, &"TestData", &"id"), None);
        assert_eq!(get_tuple_key_store(&store, &"None", &"id"), None);
    }
    #[test]
    fn test_contains_to_kv_vec() {
        let mut store = HashMap::new();
        store.insert("test", vec!["value1"]);
        assert!(contains_to_kv_vec(&mut store, &"test", &"value1"));
        assert!(!contains_to_kv_vec(&mut store, &"test", &"value2"));
    }
    #[test]
    fn test_push_to_kv_vec() {
        let mut store = HashMap::new();
        store.insert("test", vec!["value1"]);
        push_to_kv_vec(&mut store, "test", "value2");
        assert_eq!(store.get("test").unwrap(), &vec!["value1", "value2"]);
    }
}
