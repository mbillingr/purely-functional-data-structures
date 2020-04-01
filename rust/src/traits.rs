pub trait Set: Sized {
    type Element: PartialOrd;

    fn empty() -> Self;

    fn is_empty(&self) -> bool;
    fn contains(&self, x: &Self::Element) -> bool;
    fn find(&self, x: &impl PartialOrd<Self::Element>) -> Option<&Self::Element>;
    fn insert(&self, x: Self::Element) -> Self;

    fn len(&self) -> usize;

    fn from_iter(iter: impl IntoIterator<Item = Self::Element>) -> Self {
        let mut set = Self::empty();
        for x in iter {
            set = set.insert(x);
        }
        set
    }
}

pub trait Map: Sized {
    type Key: PartialOrd;
    type Value;

    fn empty() -> Self;

    fn is_empty(&self) -> bool;
    fn insert(&self, k: Self::Key, v: Self::Value) -> Self;
    fn lookup(&self, k: &impl PartialOrd<Self::Key>) -> Option<&Self::Value>;

    fn len(&self) -> usize;

    fn contains_key(&self, k: &impl PartialOrd<Self::Key>) -> bool {
        self.lookup(k).is_some()
    }

    fn from_iter(iter: impl IntoIterator<Item = (Self::Key, Self::Value)>) -> Self {
        let mut map = Self::empty();
        for (k, v) in iter {
            map = map.insert(k, v);
        }
        map
    }
}
