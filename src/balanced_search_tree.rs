use std::cell::RefCell;
use std::rc::Rc;
static RED: bool = true;
static BLACK: bool = false;

struct Node<K: PartialOrd + Copy, V: Clone> {
    key: K,
    value: V,
    left: Rc<RefCell<Option<Node<K,V>>>>,
    right: Rc<RefCell<Option<Node<K,V>>>>,
    n: usize,
    color: bool,
}

pub struct BalancedSearchTree<K: PartialOrd + Copy, V: Clone> {
    root: Rc<RefCell<Option<Node<K, V>>>>,
}

impl<K: PartialOrd + Copy, V: Clone> Node<K, V> {
    fn new(key: K, value: V, n: usize, color: bool) -> Self {
        Node {
            key,
            value,
            n,
            color,
            left: Rc::new(RefCell::new(None)),
            right: Rc::new(RefCell::new(None)),
        }
    }
}

impl<K: PartialOrd + Copy, V: Clone> BalancedSearchTree<K, V> {
    pub fn new() -> Self {
        BalancedSearchTree { root: Rc::new(RefCell::new(None)) }
    }

    fn is_red(node: &Rc<RefCell<Option<Node<K, V>>>>) -> bool {
        match *node.borrow_mut() {
            Some(ref node) => node.color == RED,
            None => false,
        }
    }

    fn rotate_left(
        node: &Rc<RefCell<Option<Node<K, V>>>>,
    ) -> Rc<RefCell<Option<Node<K, V>>>> {
        match *node.borrow_mut() {
            Some(ref mut h) => {
                let x = h.right.clone();
                h.right = match *x.borrow() {
                    None => Rc::new(RefCell::new(None)),
                    Some(ref x) => x.left.clone()
                };
                if let Some(ref mut x) = *x.borrow_mut() {
                    x.color = h.color;
                    h.color = RED;
                    x.n = h.n;
                    h.n = 1 + Self::size(&h.left) + Self::size(&h.right);
                    x.left = node.clone();
                }
                x
            },
            None => node.clone()
        }
       
    }

    fn rotate_right(
        node: &Rc<RefCell<Option<Node<K, V>>>>,
    ) -> Rc<RefCell<Option<Node<K, V>>>> {
        match *node.borrow_mut() {
            Some(ref mut h) => {
                let x = h.left.clone();
                h.left = match *x.borrow() {
                    None => Rc::new(RefCell::new(None)),
                    Some(ref x) => x.right.clone()
                };
                if let Some(ref mut x) = *x.borrow_mut() {
                    x.color = h.color;
                    h.color = RED;
                    x.n = h.n;
                    h.n = 1 + Self::size(&h.left) + Self::size(&h.right);
                    x.right = node.clone();
                }
                x
            },
            None => node.clone()
        }
       
    }

    fn size(rc_node: &Rc<RefCell<Option<Node<K, V>>>>) -> usize {
        match *rc_node.borrow() {
            Some(ref x) => x.n,
            None => 0,
        }
    }

    fn flip_color(node: &Rc<RefCell<Option<Node<K, V>>>> ) {
        match *node.borrow_mut() {
            Some(ref mut node) => {
                node.color = RED;
                if let Some(ref mut left) = *node.left.borrow_mut() {
                    left.color = BLACK;
                }
                if let Some(ref mut right) = *node.right.borrow_mut() {
                    right.color = BLACK;
                }
            },
            None => ()
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        self.root = Self::put_internal(self.root.clone(), key, value);
        match *self.root.borrow_mut(){
            Some(ref mut h) => h.color = BLACK,
            None => unreachable!()
        }
    }

    fn put_internal(
        node: Rc<RefCell<Option<Node<K, V>>>>,
        key: K,
        value: V,
    ) -> Rc<RefCell<Option<Node<K, V>>>> {
        match *node.borrow_mut() {
            Some(ref mut h) => {
                if key < h.key {
                    h.left = Self::put_internal(h.left.clone(), key, value)
                } else if key > h.key {
                    h.right = Self::put_internal(h.right.clone(), key, value)
                } else {
                    h.value = value
                }

                let mut return_node = node.clone();
                if Self::is_red(&h.right) && !Self::is_red(&h.left) {
                    return_node = Self::rotate_left(&node)
                }
                if Self::is_red(&h.left) && Self::is_red(&(&h.left).borrow().as_ref().unwrap().left) {
                    return_node = Self::rotate_right(&node)
                }
                if Self::is_red(&h.left) && Self::is_red(&h.right) {
                    Self::flip_color(&node)
                }
                h.n = Self::size(&h.left) + Self::size(&h.right) + 1;

                return_node
            },
            None => Rc::new(RefCell::new(Some(Node::new(key, value, 1,RED))))
        }
    }
   
}
