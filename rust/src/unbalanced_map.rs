use crate::traits::{Map, Set};
use crate::unbalanced_set::UnbalancedSet;

pub struct UnbalancedMap<K: 'static + PartialOrd + Clone, V: 'static + Clone> {
    set: UnbalancedSet<Assoc<K, V>>,
}

impl<K: PartialOrd + Clone, V: Clone> Map for UnbalancedMap<K, V> {
    type Key = K;
    type Value = V;

    fn empty() -> Self {
        UnbalancedMap {
            set: UnbalancedSet::empty(),
        }
    }

    fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    fn insert(&self, k: K, v: V) -> Self {
        UnbalancedMap {
            set: self.set.insert(Assoc::new(k, v)),
        }
    }

    fn lookup(&self, k: &impl PartialOrd<K>) -> Option<&V> {
        self.set.find(&Key(k)).map(|assoc| &assoc.val)
    }

    fn len(&self) -> usize {
        self.set.len()
    }
}

#[derive(Clone)]
struct Assoc<K, V> {
    key: K,
    val: V,
}

impl<K, V> Assoc<K, V> {
    pub fn new(key: K, val: V) -> Self {
        Assoc { key, val }
    }
}

impl<K: PartialEq, V> PartialEq for Assoc<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.key.eq(&other.key)
    }
}

impl<K: PartialOrd, V> PartialOrd for Assoc<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.key.partial_cmp(&other.key)
    }
}

struct Key<K>(K);

/*impl<K: PartialEq, V> PartialEq<Assoc<K, V>> for Key<&K> {
    fn eq(&self, other: &Assoc<K, V>) -> bool {
        self.0.eq(&other.key)
    }
}*/

/*impl<K: PartialOrd, V> PartialOrd<Assoc<K, V>> for Key<&K> {
    fn partial_cmp(&self, other: &Assoc<K, V>) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.key)
    }
}*/

/*impl<K: PartialEq, V> PartialEq<Assoc<K, V>> for Key<K> {
    fn eq(&self, other: &Assoc<K, V>) -> bool {
        self.0.eq(&other.key)
    }
}*/

/*impl<K: PartialOrd, V> PartialOrd<Assoc<K, V>> for Key<K> {
    fn partial_cmp(&self, other: &Assoc<K, V>) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.key)
    }
}*/

impl<K: PartialEq<L>, L: PartialEq, V> PartialEq<Assoc<L, V>> for Key<&K> {
    fn eq(&self, other: &Assoc<L, V>) -> bool {
        self.0.eq(&other.key)
    }
}

impl<K: PartialOrd<L>, L: PartialOrd, V> PartialOrd<Assoc<L, V>> for Key<&K> {
    fn partial_cmp(&self, other: &Assoc<L, V>) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.key)
    }
}

/*impl<K: PartialEq, V> PartialEq<Key<K>> for Assoc<K, V> {
    fn eq(&self, other: &Key<K>) -> bool {
        self.key.eq(&other.0)
    }
}

impl<K: PartialOrd, V> PartialOrd<Key<K>> for Assoc<K, V> {
    fn partial_cmp(&self, other: &Key<K>) -> Option<std::cmp::Ordering> {
        self.key.partial_cmp(&other.0)
    }
}

impl<K: PartialEq, V> PartialEq<Key<&K>> for Assoc<K, V> {
    fn eq(&self, other: &Key<&K>) -> bool {
        self.key.eq(&other.0)
    }
}

impl<K: PartialOrd, V> PartialOrd<Key<&K>> for Assoc<K, V> {
    fn partial_cmp(&self, other: &Key<&K>) -> Option<std::cmp::Ordering> {
        self.key.partial_cmp(&other.0)
    }
}*/
