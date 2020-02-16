pub struct BinarySearchTree<'a> {
    pub head: Option<&'a BST>,
}

struct BST<'a> {
    value: usize,
    left: Option<&'a BST>,
    right: Option<&'a BST>,
}

impl<'a> BinarySearchTree {
    pub fn new() -> Self {
        BinarySearchTree { head: None }
    }
}
