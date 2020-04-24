pub struct BinarySearchTree<T: PartialOrd + Copy, U> {
    root: Option<Box<BTNode<T, U>>>,
}

/// Binary search tree node
pub struct BTNode<T: PartialOrd + Copy, U> {
    key: T,
    value: U,
    left: Option<Box<BTNode<T, U>>>,
    right: Option<Box<BTNode<T, U>>>,
    n: usize,
}

impl<T: PartialOrd + Copy, U: Clone> BTNode<T, U> {
    fn new(key: T, value: U, n: usize) -> BTNode<T, U> {
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
    pub fn size(&self) -> usize {
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
        Self::get_internal(&self.root, key)
    }

    fn get_internal(node: &Option<Box<BTNode<T, U>>>, key: T) -> Option<U> {
        if let Some(ref node) = node {
            if node.key > key {
                Self::get_internal(&node.left, key)
            } else if node.key < key {
                Self::get_internal(&node.right, key)
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
            node.n = Self::node_size(node.left.as_ref()) + Self::node_size(node.right.as_ref()) + 1;
        } else {
            *node = Some(Box::new(BTNode::new(key, val, 1)));
        }
    }
    fn node_size(node: Option<&Box<BTNode<T, U>>>) -> usize {
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
            Some(node) => Some(Self::min_internal(node).key),
        }
    }

    fn min_internal(node: &Box<BTNode<T, U>>) -> &Box<BTNode<T, U>> {
        match &node.left {
            None => node,
            Some(left) => Self::min_internal(&left),
        }
    }

    /// Get the key of the floor node of `key`.
    pub fn floor(&self, key: T) -> Option<T> {
        match Self::floor_internal(self.root.as_ref(), key) {
            Some(boxed_node) => Some(boxed_node.key),
            None => None,
        }
    }

    fn floor_internal(node: Option<&Box<BTNode<T, U>>>, key: T) -> Option<&Box<BTNode<T, U>>> {
        match node {
            Some(box_node) => {
                if key < box_node.key {
                    Self::floor_internal(box_node.left.as_ref(), key)
                } else if key > box_node.key {
                    let t = Self::floor_internal(box_node.right.as_ref(), key);
                    match t {
                        Some(_) => t,
                        None => node,
                    }
                } else {
                    node
                }
            }
            None => None,
        }
    }

    /// Get the key of the node of rank `rank` (the key such that 
    /// precisely *rank* other keys in the BST are smaller).
    pub fn key_of_rank(&self, rank: usize) -> Option<T> {
        match Self::select_internal(self.root.as_ref(), rank) {
            Some(boxed_node) => Some(boxed_node.key),
            None => None,
        }
    }

    fn select_internal(node: Option<&Box<BTNode<T, U>>>, rank: usize) -> Option<&Box<BTNode<T, U>>> {
        match node {
            Some(box_node) => {
                let t = Self::node_size(box_node.left.as_ref());
                if t > rank { 
                    Self::select_internal(box_node.left.as_ref(), rank)
                } else if t < rank {
                    Self::select_internal(box_node.right.as_ref(), rank-t-1)
                } else {
                    node
                }
            },
            None => None
        }
    }

    /// Inverse method of `key_of_rank()`.
    pub fn rank_of_key(&self, key: T) -> usize {
        return Self::rank_internal(self.root.as_ref(), key)
    }

    fn rank_internal(node: Option<&Box<BTNode<T, U>>>, key:T) -> usize {
        match node {
            Some(box_node) => {
                if key < box_node.key {
                    Self::rank_internal(box_node.left.as_ref(), key)
                } else if key > box_node.key {
                    1 + Self::node_size(box_node.left.as_ref()) + Self::rank_internal(box_node.right.as_ref(), key)
                } else {
                    Self::node_size(box_node.left.as_ref())
                }
            },
            None => 0
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
        assert_eq!(table.key_of_rank(2), Some(4));
        assert_eq!(table.rank_of_key(4), 2);
    }
}
