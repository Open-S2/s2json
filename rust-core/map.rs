use alloc::collections::BTreeMap;
use core::ops::{Deref, DerefMut};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A BTreeMap wrapper
#[derive(Clone, Debug, PartialEq, Default, PartialOrd, Eq)]
pub struct Map<K: Ord, V> {
    map: BTreeMap<K, V>,
}
impl<K: Ord, V> Map<K, V> {
    /// Creates an empty `Map<K, V>`
    pub fn new() -> Map<K, V> {
        Map { map: BTreeMap::new() }
    }
}
// Automatically expose BTreeMap's methods
impl<K: Ord, V> Deref for Map<K, V> {
    type Target = BTreeMap<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}
impl<K: Ord, V> DerefMut for Map<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

impl<K: Ord, V> From<BTreeMap<K, V>> for Map<K, V> {
    fn from(map: BTreeMap<K, V>) -> Self {
        Self { map }
    }
}
// Enable `Map<K, V>::from([(_, _), ...])` (same as BTreeMap::from)
impl<K: Ord, V, const N: usize> From<[(K, V); N]> for Map<K, V> {
    fn from(arr: [(K, V); N]) -> Self {
        Self { map: BTreeMap::from(arr) }
    }
}
// Custom serialization
impl<K: Ord + Serialize, V: Serialize> Serialize for Map<K, V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.map.serialize(serializer) // Serialize only the inner BTreeMap
    }
}
// Custom deserialization
impl<'de, K: Ord + Deserialize<'de>, V: Deserialize<'de>> Deserialize<'de> for Map<K, V> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map = BTreeMap::deserialize(deserializer)?;
        Ok(Self { map }) // Deserialize directly into the inner BTreeMap
    }
}
