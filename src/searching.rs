use std::cell::RefCell;
use std::rc::Rc;

pub struct BinarySearchTree<T: PartialOrd + Copy, U> {
    root: Option<Rc<RefCell<BTNode<T, U>>>>,
}

/// Binary search tree node
pub struct BTNode<T: PartialOrd + Copy, U> {
    key: T,
    value: U,
    left: Option<Rc<RefCell<BTNode<T, U>>>>,
    right: Option<Rc<RefCell<BTNode<T, U>>>>,
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
            root.borrow().n
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

    fn get_internal(node: &Option<Rc<RefCell<BTNode<T, U>>>>, key: T) -> Option<U> {
        if let Some(node) = node {
            let ref_node = node.borrow();
            if ref_node.key > key {
                Self::get_internal(&ref_node.left, key)
            } else if ref_node.key < key {
                Self::get_internal(&ref_node.right, key)
            } else {
                Some(ref_node.value.clone())
            }
        } else {
            None
        }
    }

    /// Insert key value pair into tree. If key already exists, update the corresponding value.
    pub fn put(&mut self, key: T, val: U) {
        BinarySearchTree::put_internal(&mut self.root, key, val);
    }

    fn put_internal(node: &mut Option<Rc<RefCell<BTNode<T, U>>>>, key: T, val: U) {
        if let Some(node) = node {
            let mut ref_node = node.borrow_mut();
            if ref_node.key > key {
                BinarySearchTree::put_internal(&mut ref_node.left, key, val);
            } else if ref_node.key < key {
                BinarySearchTree::put_internal(&mut ref_node.right, key, val);
            } else {
                ref_node.value = val;
            }
            ref_node.n = Self::node_size(&ref_node.left) + Self::node_size(&ref_node.right) + 1;
        } else {
            *node = Some(Rc::new(RefCell::new(BTNode::new(key, val, 1)))); //Some(Box::new(BTNode::new(key, val, 1)));
        }
    }
    fn node_size(node: &Option<Rc<RefCell<BTNode<T, U>>>>) -> usize {
        if let Some(node) = node {
            node.borrow().n
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
            Some(node) => Some(Self::min_internal(node.clone()).borrow().key),
        }
    }

    fn min_internal(node: Rc<RefCell<BTNode<T, U>>>) -> Rc<RefCell<BTNode<T, U>>> {
        match &node.borrow().left {
            None => node.clone(),
            Some(left) => Self::min_internal((*left).clone()),
        }
    }

    /// Get the key of the floor node of `key`, i.e. the largest key in the BST less than or equal to `key`.
    pub fn floor(&self, key: T) -> Option<T> {
        match &self.root {
            Some(rc_node) => match Self::floor_internal(Some((*rc_node).clone()), key) {
                Some(ref_node) => Some(ref_node.borrow().key),
                None => None,
            },
            None => None,
        }
        // match Self::floor_internal(self.root, key) {
        //     Some(boxed_node) => Some(boxed_node.borrow().key),
        //     None => None,
        // }
    }

    fn floor_internal(
        node: Option<Rc<RefCell<BTNode<T, U>>>>,
        key: T,
    ) -> Option<Rc<RefCell<BTNode<T, U>>>> {
        match node {
            Some(box_node) => {
                if key < box_node.borrow().key {
                    match &box_node.borrow().left {
                        Some(left) => Self::floor_internal(Some((*left).clone()), key),
                        None => None,
                    }
                } else if key > box_node.borrow().key {
                    let t = match &box_node.borrow().right {
                        Some(right) => Self::floor_internal(Some((*right).clone()), key),
                        None => None,
                    };
                    match t {
                        Some(_) => t,
                        None => Some(box_node),
                    }
                } else {
                    Some(box_node)
                }
            }
            None => None,
        }
    }


    /// Get the key of the node of rank `rank` (the key such that
    /// precisely *rank* other keys in the BST are smaller).
    pub fn key_of_rank(&self, rank: usize) -> Option<T> {
        match &self.root {
            Some(rc_node) => match Self::select_internal(Some((*rc_node).clone()), rank) {
                Some(ref_node) => Some(ref_node.borrow().key),
                None => None,
            },
            None => None,
        }
        // match Self::select_internal(self.root, rank) {
        //     Some(boxed_node) => Some(boxed_node.borrow().key),
        //     None => None,
        // }
    }

    fn select_internal(
        node: Option<Rc<RefCell<BTNode<T, U>>>>,
        rank: usize,
    ) -> Option<Rc<RefCell<BTNode<T, U>>>> {
        match node {
            Some(rc_node) => {
                let t = Self::node_size(&rc_node.borrow().left);
                if t > rank {
                    match &rc_node.borrow().left {
                        Some(left) => Self::select_internal(Some((left).clone()), rank),
                        None => None,
                    }
                // Self::select_internal((*(rc_node).clone()).borrow().left, rank)
                } else if t < rank {
                    match &rc_node.borrow().right {
                        Some(right) => Self::select_internal(Some((right).clone()), rank - t - 1),
                        None => None,
                    }
                // Self::select_internal(rc_node.borrow().right, rank - t - 1)
                } else {
                    Some(rc_node)
                }
            }
            None => None,
        }
    }

    /// Inverse method of `key_of_rank()`.
    pub fn rank_of_key(&self, key: T) -> usize {
        return Self::rank_internal(self.root.as_ref(), key);
    }

    fn rank_internal(node: Option<&Rc<RefCell<BTNode<T, U>>>>, key: T) -> usize {
        match node {
            Some(rc_node) => {
                let ref_node = rc_node.borrow();
                if key < ref_node.key {
                    Self::rank_internal(ref_node.left.as_ref(), key)
                } else if key > ref_node.key {
                    1 + Self::node_size(&ref_node.left)
                        + Self::rank_internal(ref_node.right.as_ref(), key)
                } else {
                    Self::node_size(&ref_node.left)
                }
            }
            None => 0,
        }
    }

    pub fn delete_min(&mut self) {
        match &self.root {
            Some(rc_node) => self.root = Self::delete_min_internal(Some((*rc_node).clone())),
            None => (),
        }
    }

    fn delete_min_internal(
        node: Option<Rc<RefCell<BTNode<T, U>>>>,
    ) -> Option<Rc<RefCell<BTNode<T, U>>>> {
        match node {
            Some(rc_node) => {
                if rc_node.borrow().left.is_none() {
                    match &rc_node.borrow().right {
                        Some(right) => Some(right.clone()),
                        None => None,
                    }
                } else {
                    let new_left = match &rc_node.borrow().left {
                        Some(left) => Self::delete_min_internal(Some(left.clone())),
                        None => None,
                    };
                    {
                        rc_node.borrow_mut().left = new_left;
                    }
                    let new_n = {
                        Self::node_size(&rc_node.borrow().left)
                            + Self::node_size(&rc_node.borrow().right)
                            + 1
                    };
                    rc_node.borrow_mut().n = new_n;
                    Some(rc_node)
                }
            }
            None => None,
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
        table.delete_min();
    }
}
