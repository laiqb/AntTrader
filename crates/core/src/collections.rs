use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display},
    hash::Hash,
};

pub trait SetLike {
    //集合中存储的 Item
    type Item: Hash + Eq + Display + Clone;

    fn contains(&self, item: &Self::Item) -> bool;

    fn is_empty(&self) -> bool;
}

impl<T, S> SetLike for HashSet<T, S> 
where 
    T: Eq + Hash + Display + Clone,
    S: std::hash::BuildHasher,
{
    type Item = T;

    #[inline]
    fn contains(&self, v: &T) -> bool {
        Self::contains(self, v)
    }

    #[inline]
    fn is_empty(&self) -> bool {
        Self::is_empty(self)
    }
}

impl<T, S> SetLike for indexmap::IndexSet<T, S> 
where
    T: Eq + Hash + Display + Clone,
    S: std::hash::BuildHasher,
{
    type Item = T;

    #[inline]
    fn contains(&self, item: &T) -> bool {
        Self::contains(self, item)
    }
    
    #[inline]
    fn is_empty(&self) -> bool {
        Self::is_empty(self)
    }
}

impl<T, S> SetLike for ahash::AHashSet<T, S>
where 
    T: Eq + Hash + Display + Clone,
    S: std::hash::BuildHasher,
{
    type Item = T;

    #[inline]
    fn contains(&self, item: &T) -> bool {
        self.get(item).is_some()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait  MapLike {
    type Key: Hash + Eq + Display + Clone;

    type Value: Debug;

    fn contains_key(&self, key: &Self::Key) -> bool;

    fn is_empty(&self) -> bool;
}

impl<K, V, S> MapLike for HashMap<K, V, S> 
where
    K: Eq + Hash + Display + Clone,
    V: Debug,
    S: std::hash::BuildHasher,
{
    type Key = K;
    type Value = V;

    #[inline]
    fn contains_key(&self, key: &K) -> bool {
        self.contains_key(key)
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl <K, V, S> MapLike for ahash::AHashMap<K, V, S>
where 
    K: Eq + Hash + Display + Clone,
    V: Debug,
    S: std::hash::BuildHasher,
{
    type Key = K;
    type Value = V;

    #[inline]
    fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests{
    use rstest::*;
    use super::*;

    #[rstest]
    fn test_hashset_set_like(){
        let mut set: HashSet<String> = HashSet::new();
        set.insert("test".to_string());
        set.insert("value".to_string());

        assert!(set.contains(&"test".to_string()));
        assert!(!set.contains(&"missing".to_string()));
        assert!(!set.is_empty());

        let empty_set: HashSet<String> = HashSet::new();
        assert!(empty_set.is_empty());
    }
}