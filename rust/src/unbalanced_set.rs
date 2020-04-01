use crate::shared_data::Share;
use crate::traits::Set;
use crate::binary_tree::BinaryTree;
use ref_cast::RefCast;

#[derive(Clone, PartialEq)]
#[derive(RefCast)]
#[repr(transparent)]
pub struct UnbalancedSet<T: 'static + PartialOrd> {
    tree: BinaryTree<T>,
}

impl<T: PartialOrd + Clone> UnbalancedSet<T> {
    pub fn new() -> Self {
        Self::empty()
    }

    fn from_tree(tree: BinaryTree<T>) -> Self {
        UnbalancedSet {
            tree
        }
    }

    pub(crate) fn find_impl<'a>(
        &'a self,
        x: &impl PartialOrd<T>,
        candidate: &'a T,
    ) -> Option<&'a T> {
        match &self.tree {
            BinaryTree::Empty => {
                if x == candidate {
                    Some(candidate)
                } else {
                    None
                }
            }
            BinaryTree::Node(tree) => {
                let (left, y, right) = &**tree;
                if x < y {
                    Self::ref_cast(left).find_impl(x, candidate)
                } else {
                    Self::ref_cast(right).find_impl(x, y)
                }
            }
        }
    }

    fn insert_impl(&self, x: T) -> Option<Self> {
        use BinaryTree::*;
        match &self.tree {
            Empty => Some(Self::from_tree(Node(Share::new((Empty, x, Empty))))),
            Node(tree) => {
                let (left, y, right) = &**tree;
                if x < *y {
                    Some(Self::from_tree(Node(Share::new((
                        Self::ref_cast(left).insert_impl(x)?.tree,
                        y.clone(),
                        right.clone(),
                    )))))
                } else if x == *y {
                    None
                } else {
                    Some(Self::from_tree(Node(Share::new((
                        left.clone(),
                        y.clone(),
                        Self::ref_cast(right).insert_impl(x)?.tree,
                    )))))
                }
            }
        }
    }
}

impl<T: PartialOrd + Clone> Set for UnbalancedSet<T> {
    type Element = T;

    fn empty() -> Self {
        Self::from_tree(BinaryTree::Empty)
    }

    fn is_empty(&self) -> bool {
        match &self.tree {
            BinaryTree::Empty => true,
            _ => false,
        }
    }

    fn contains(&self, x: &Self::Element) -> bool {
        self.find(x).is_some()
    }

    fn find(&self, x: &impl PartialOrd<Self::Element>) -> Option<&Self::Element> {
        match &self.tree {
            BinaryTree::Empty => None,
            BinaryTree::Node(tree) => {
                let (left, y, right) = &**tree;
                if x < y {
                    Self::ref_cast(left).find(x)
                } else if x == y {
                    Some(y)
                } else {
                    Self::ref_cast(right).find_impl(x, y)
                }
            }
        }
    }

    fn insert(&self, x: T) -> Self {
        self.insert_impl(x).unwrap_or_else(|| self.clone())
    }

    fn len(&self) -> usize {
        match &self.tree {
            BinaryTree::Empty => 0,
            BinaryTree::Node(tree) => {
                let (left, _, right) = &**tree;
                1 + Self::ref_cast(left).len() + Self::ref_cast(right).len()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_set_is_empty() {
        assert!(UnbalancedSet::<()>::empty().is_empty())
    }

    #[test]
    fn empty_set_has_zero_length() {
        assert_eq!(UnbalancedSet::<()>::empty().len(), 0)
    }

    #[test]
    fn empty_set_does_not_contain_item() {
        assert!(!UnbalancedSet::<()>::empty().contains(&()))
    }

    #[test]
    fn singleton_set_is_not_empty() {
        assert!(!UnbalancedSet::empty().insert(()).is_empty())
    }

    #[test]
    fn singleton_set_has_length_one() {
        assert_eq!(UnbalancedSet::empty().insert(()).len(), 1)
    }

    #[test]
    fn singleton_set_contains_same_item() {
        assert!(UnbalancedSet::empty().insert(42).contains(&42))
    }

    #[test]
    fn singleton_set_does_not_contain_other_item() {
        assert!(!UnbalancedSet::empty().insert(42).contains(&123))
    }

    #[test]
    fn balanced_tree() {
        let set = UnbalancedSet::from_iter(vec![5, 3, 7, 2, 4, 6, 8]);
        assert!(!set.is_empty());
        assert_eq!(set.len(), 7);
        assert!(!set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
        assert!(set.contains(&4));
        assert!(set.contains(&5));
        assert!(set.contains(&6));
        assert!(set.contains(&7));
        assert!(set.contains(&8));
        assert!(!set.contains(&9));
    }

    #[test]
    fn left_unbalanced_tree() {
        let set = UnbalancedSet::from_iter(vec![8, 7, 6, 5, 4, 3, 2]);
        assert!(!set.is_empty());
        assert_eq!(set.len(), 7);
        assert!(!set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
        assert!(set.contains(&4));
        assert!(set.contains(&5));
        assert!(set.contains(&6));
        assert!(set.contains(&7));
        assert!(set.contains(&8));
        assert!(!set.contains(&9));
    }

    #[test]
    fn right_unbalanced_tree() {
        let set = UnbalancedSet::from_iter(vec![2, 3, 4, 5, 6, 7, 8]);
        assert!(!set.is_empty());
        assert_eq!(set.len(), 7);
        assert!(!set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
        assert!(set.contains(&4));
        assert!(set.contains(&5));
        assert!(set.contains(&6));
        assert!(set.contains(&7));
        assert!(set.contains(&8));
        assert!(!set.contains(&9));
    }
}
