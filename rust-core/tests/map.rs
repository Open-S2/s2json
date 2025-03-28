extern crate alloc;

#[cfg(test)]
mod tests {
    use alloc::collections::BTreeMap;
    use s2json_core::*;

    #[test]
    fn test_map() {
        let mut map = Map::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);
        assert_eq!(map.len(), 3);
        assert_eq!(map.get(&"a").unwrap(), &1);
        assert_eq!(map.get(&"b").unwrap(), &2);
        assert_eq!(map.get(&"c").unwrap(), &3);
        assert_eq!(map.get(&"d"), None);
    }

    #[test]
    fn test_from() {
        let map = Map::from([("a", 1), ("b", 2), ("c", 3)]);
        assert_eq!(map.len(), 3);
        assert_eq!(map.get(&"a").unwrap(), &1);
        assert_eq!(map.get(&"b").unwrap(), &2);
        assert_eq!(map.get(&"c").unwrap(), &3);
        assert_eq!(map.get(&"d"), None);
    }

    #[test]
    fn from_btree_map() {
        let btree_map = BTreeMap::from([("a", 1), ("b", 2), ("c", 3)]);
        let map = Map::from(btree_map);
        assert_eq!(map.len(), 3);
        assert_eq!(map.get(&"a").unwrap(), &1);
        assert_eq!(map.get(&"b").unwrap(), &2);
        assert_eq!(map.get(&"c").unwrap(), &3);
        assert_eq!(map.get(&"d"), None);
    }
}
