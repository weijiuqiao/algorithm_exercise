pub struct BinarySearchTree<T: PartialOrd + Copy, U> {
    root: Option<Box<BTNode<T, U>>>,
}

/// Binary search tree node
pub struct BTNode<T: PartialOrd + Copy, U> {
    key: T,
    value: U,
    left: Option<Box<BTNode<T, U>>>,
    right: Option<Box<BTNode<T, U>>>,
    n: i32,
}

impl<T: PartialOrd + Copy, U: Clone> BTNode<T, U> {
    fn new(key: T, value: U, n: i32) -> BTNode<T, U> {
        BTNode {
            key,
            value,
            left: None,
            right: None,
            n,
        }
    }
}
impl<T: PartialOrd + Copy, U: Clone> BinarySearchTree<T, U> {
    /// Initializer.
    pub fn new() -> Self {
        BinarySearchTree { root: None }
    }

    /// Return size of the tree, ie. count of nodes.
    /// ```
    /// # use algorithm_exercise::*;
    /// let tree = BinarySearchTree::<i32, &str>::new();
    /// # //tree.put(2, "2");
    /// assert_eq!(tree.size(), 0);
    /// ```
    pub fn size(&self) -> i32 {
        if let Some(ref root) = self.root {
            root.n
        } else {
            0
        }
    }

    /// Get the value for `key`. Returns `None` if the tree doesn't have key `key`.
    /// ```
    /// # use algorithm_exercise::*;
    /// let mut tree = BinarySearchTree::new();
    /// tree.put("one",1);
    /// tree.put("two",2);
    /// assert_eq!(tree.get("two"), Some(2));
    /// assert_eq!(tree.get(""), None);
    /// ```
    pub fn get(&self, key: T) -> Option<U> {
        BinarySearchTree::get_internal(&self.root, key)
    }

    fn get_internal(node: &Option<Box<BTNode<T, U>>>, key: T) -> Option<U> {
        if let Some(ref node) = node {
            if node.key > key {
                BinarySearchTree::get_internal(&node.left, key)
            } else if node.key < key {
                BinarySearchTree::get_internal(&node.right, key)
            } else {
                Some(node.value.clone())
            }
        } else {
            None
        }
    }

    /// Insert key value pair into tree. If key already exists, update the corresponding value.
    pub fn put(&mut self, key: T, val: U) {
        BinarySearchTree::put_internal(&mut self.root, key, val);
    }

    fn put_internal(node: &mut Option<Box<BTNode<T, U>>>, key: T, val: U) {
        if let Some(ref mut node) = node {
            if node.key > key {
                BinarySearchTree::put_internal(&mut node.left, key, val);
            } else if node.key < key {
                BinarySearchTree::put_internal(&mut node.right, key, val);
            } else {
                node.value = val;
            }
            node.n = BinarySearchTree::node_size(&node.left)
                + BinarySearchTree::node_size(&node.right)
                + 1;
        } else {
            *node = Some(Box::new(BTNode::new(key, val, 1)));
        }
    }
    fn node_size(node: &Option<Box<BTNode<T, U>>>) -> i32 {
        if let Some(node) = node {
            node.n
        } else {
            0
        }
    }

    /// Get the key for minimal value.
    /// ```
    /// # use algorithm_exercise::*;
    /// let mut tree = BinarySearchTree::new();
    /// tree.put("two",2);
    /// tree.put("one",1);
    /// assert_eq!(tree.min(), Some("one"))
    /// ```
    pub fn min(&self) -> Option<T> {
        match &self.root {
            None => None,
            Some(node) => Some(BinarySearchTree::min_internal(node).key),
        }
    }

    fn min_internal(node: &Box<BTNode<T, U>>) -> &Box<BTNode<T, U>> {
        match &node.left {
            None => node,
            Some(left) => BinarySearchTree::min_internal(&left),
        }
    }

    /// Get the key of the floor node of `key`.
    pub fn floor(&self, key: T) -> Option<T> {
        match BinarySearchTree::floor_internal(&self.root, key) {
            Some(boxed_node) => Some(boxed_node.key),
            None => None,
        }
    }

    fn floor_internal(node: &Option<Box<BTNode<T, U>>>, key: T) -> &Option<Box<BTNode<T, U>>> {
        match node {
            Some(box_node) => {
                if key < box_node.key {
                    BinarySearchTree::floor_internal(&box_node.left, key)
                } else if key > box_node.key {
                    let t = BinarySearchTree::floor_internal(&box_node.right, key);
                    match t {
                        Some(_) => t,
                        None => node,
                    }
                } else {
                    node
                }
            }
            None => &None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut table = BinarySearchTree::new();
        table.put(3, "3");
        table.put(4, "4");
        table.put(2, "2");
        assert_eq!(table.size(), 3);
        assert_eq!(table.get(3), Some("3"));
        assert_eq!(table.get(1), None);
        assert_eq!(table.floor(1), None);
    }
}
