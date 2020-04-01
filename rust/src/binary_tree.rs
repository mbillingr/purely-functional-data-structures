use crate::shared_data::Share;

#[derive(Clone, PartialEq)]
pub enum BinaryTree<T: 'static> {
    Empty,
    Node(Share<(Self, T, Self)>),
}
