use std::cell::RefCell;
use std::rc::Rc;

pub struct BinarySearchTree<T: PartialOrd + Copy, U> {
    root: Rc<RefCell<Option<BTNode<T, U>>>>,
}

/// Binary search tree node
pub struct BTNode<T: PartialOrd + Copy, U> {
    key: T,
    value: U,
    left: Rc<RefCell<Option<BTNode<T, U>>>>,
    right: Rc<RefCell<Option<BTNode<T, U>>>>,
    n: usize,
}

impl<T: PartialOrd + Copy, U: Clone> BTNode<T, U> {
    fn new(key: T, value: U, n: usize) -> BTNode<T, U> {
        BTNode {
            key,
            value,
            left: Rc::new(RefCell::new(None)),
            right: Rc::new(RefCell::new(None)),
            n,
        }
    }
}

impl<T: PartialOrd + Copy, U: Clone> BinarySearchTree<T, U> {
    /// Initializer.
    pub fn new() -> Self {
        BinarySearchTree {
            root: Rc::new(RefCell::new(None)),
        }
    }

    /// Return size of the tree, ie. count of nodes.
    /// ```
    /// # use algorithm_exercise::*;
    /// let tree = BinarySearchTree::<i32, &str>::new();
    /// # //tree.put(2, "2");
    /// assert_eq!(tree.size(), 0);
    /// ```
    pub fn size(&self) -> usize {
        if let Some(ref root) = *self.root.borrow() {
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

    fn get_internal(node: &Rc<RefCell<Option<BTNode<T, U>>>>, key: T) -> Option<U> {
        if let Some(ref node) = *node.borrow() {
            // let ref_node = node.borrow();
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
    pub fn put(&self, key: T, val: U) {
        BinarySearchTree::put_internal(&self.root, key, val);
    }

    fn put_internal(node: &Rc<RefCell<Option<BTNode<T, U>>>>, key: T, val: U) {
        let mut m = node.borrow_mut();
        if let Some(ref mut node) = *m {
            // let mut ref_node = node.borrow_mut();
            if node.key > key {
                BinarySearchTree::put_internal(&node.left, key, val);
            } else if node.key < key {
                BinarySearchTree::put_internal(&node.right, key, val);
            } else {
                node.value = val;
            }
            node.n = Self::node_size(&node.left) + Self::node_size(&node.right) + 1;
        } else {
            *m = Some(BTNode::new(key, val, 1));
        }
    }

    fn node_size(node: &Rc<RefCell<Option<BTNode<T, U>>>>) -> usize {
        if let Some(ref node) = *node.borrow() {
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
        // match *Self::min_internal(self.root.clone()).borrow() {
        //     Some(ref node) => Some(node.key),
        //     None => None,
        // }
        Self::min_internal(self.root.clone()) // Rc<RefCell<Option<Node>>>
            .borrow() // &Option<Node>
            .as_ref() // Option<&Node> (auto dereffed)
            .map(|x| x.key) // Option<T> (T copied)
    }

    fn min_internal(node: Rc<RefCell<Option<BTNode<T, U>>>>) -> Rc<RefCell<Option<BTNode<T, U>>>> {
        match *node.borrow() {
            None => node.clone(),
            Some(ref b_node) => match *b_node.left.borrow() {
                None => node.clone(),
                Some(_) => Self::min_internal(b_node.left.clone()),
            },
        }
    }
    /// Get the key of the floor node of `key`, i.e. the largest key in the BST less than or equal to `key`.
    pub fn floor(&self, key: T) -> Option<T> {
        Self::floor_internal(self.root.clone(), key) // Rc<RefCell<Option<Node>>>
            .borrow() // &Option<Node>
            .as_ref() // Option<&Node> (auto dereffed)
            .map(|x| x.key) // Option<T> (T copied)
    }

    fn floor_internal(
        node: Rc<RefCell<Option<BTNode<T, U>>>>,
        key: T,
    ) -> Rc<RefCell<Option<BTNode<T, U>>>> {
        match *node.borrow() {
            Some(ref b_node) => {
                if key < b_node.key {
                    Self::floor_internal(b_node.left.clone(), key)
                } else if key > b_node.key {
                    let floor_right = Self::floor_internal(b_node.right.clone(), key);
                    match *floor_right.clone().borrow() {
                        Some(_) => floor_right,
                        None => node.clone(),
                    }
                } else {
                    node.clone()
                }
            }
            None => node.clone(),
        }
    }

    /// Get the key of the node of rank `rank` (the key such that
    /// precisely *rank* number of other keys in the BST are smaller).
    pub fn key_of_rank(&self, rank: usize) -> Option<T> {
        Self::select_internal(self.root.clone(), rank)
            .borrow()
            .as_ref()
            .map(|x| x.key)
    }

    fn select_internal(
        node: Rc<RefCell<Option<BTNode<T, U>>>>,
        rank: usize,
    ) -> Rc<RefCell<Option<BTNode<T, U>>>> {
        match *node.borrow() {
            Some(ref b_node) => {
                let t = Self::node_size(&b_node.left);
                if t > rank {
                    Self::select_internal(b_node.left.clone(), rank)
                } else if t < rank {
                    Self::select_internal(b_node.right.clone(), rank - t - 1)
                } else {
                    node.clone()
                }
            }
            None => node.clone(),
        }
    }
    /// Inverse method of `key_of_rank()`.
    pub fn rank_of_key(&self, key: T) -> usize {
        return Self::rank_internal(self.root.clone(), key);
    }

    fn rank_internal(node: Rc<RefCell<Option<BTNode<T, U>>>>, key: T) -> usize {
        match *node.borrow() {
            Some(ref b_node) => {
                if key < b_node.key {
                    Self::rank_internal(b_node.left.clone(), key)
                } else if key > b_node.key {
                    1 + Self::node_size(&b_node.left)
                        + Self::rank_internal(b_node.right.clone(), key)
                } else {
                    Self::node_size(&b_node.left)
                }
            }
            None => 0,
        }
    }

    /// Delete the node with minimal value.
    pub fn delete_min(&mut self) {
        self.root = Self::delete_min_internal(self.root.clone())
    }

    fn delete_min_internal(
        node: Rc<RefCell<Option<BTNode<T, U>>>>,
    ) -> Rc<RefCell<Option<BTNode<T, U>>>> {
        match *node.borrow_mut() {
            Some(ref mut b_node) => {
                if b_node.left.borrow().is_none() {
                    b_node.right.clone()
                } else {
                    b_node.left = Self::delete_min_internal(b_node.left.clone());
                    b_node.n = Self::node_size(&b_node.left) + Self::node_size(&b_node.right) + 1;
                    node.clone()
                }
            }
            None => node.clone(),
        }
    }

    /// Delete the node of `Key`.
    pub fn delete(&mut self, key: T) {
        self.root = Self::delete_internal(self.root.clone(), key);
    }

    fn delete_internal(node: Rc<RefCell<Option<BTNode<T,U>>>>, key: T) -> Rc<RefCell<Option<BTNode<T,U>>>>{
        match *node.clone().borrow_mut() {
            Some(ref mut b_node) => {
                if key < b_node.key {
                    b_node.left = Self::delete_internal(b_node.left.clone(), key);
                } else if key > b_node.key {
                    b_node.right = Self::delete_internal(b_node.right.clone(), key);
                } else {
                    if b_node.right.borrow().is_none() {
                        return b_node.left.clone()
                    }
                    if b_node.left.borrow().is_none() {
                        return b_node.right.clone()
                    }
                    let new_node = Self::min_internal(b_node.right.clone());
                    let right = Self::delete_min_internal(b_node.right.clone());
                    if let Some(ref mut x) = *new_node.borrow_mut() {
                        x.right = right;
                        x.left = b_node.left.clone();
                        x.n = Self::node_size(&x.left) + Self::node_size(&x.right) + 1;
                        return new_node.clone()
                    } else {
                        unreachable!()
                    };

                }
                b_node.n = Self::node_size(&b_node.left) + Self::node_size(&b_node.right) + 1;
                node
            }
            None => node
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
        assert_eq!(table.floor(5), Some(4));
        assert_eq!(table.key_of_rank(2), Some(4));
        assert_eq!(table.rank_of_key(4), 2);
        // table.delete_min();
        table.delete(3);
        assert_eq!(table.size(), 2);
        assert_eq!(table.get(3), None);
        assert_eq!(table.floor(4), Some(4));
        assert_eq!(table.floor(3), Some(2));
    }
}
