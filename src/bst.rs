use std::cmp::PartialOrd;

pub struct BinarySearchTree<T: PartialOrd> {
    root: Option<Box<Node<T>>>,
}

struct Node<T: PartialOrd> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> BinarySearchTree<T>
where T: PartialOrd {
    pub fn new() -> BinarySearchTree<T> {
       BinarySearchTree { root: None }
    }

    pub fn insert(&mut self, value: T) {
        match self.root {
            Some(ref mut root) => root.insert(value),
            None => { self.root = Some(Box::new(Node::new(value))) },
        }
    }
}

impl<T> Node<T>
where T: PartialOrd {
    pub fn new(value: T) -> Node<T> {
        Node { value: value, left: None, right: None }
    }

    pub fn insert(&mut self, value: T) {
        if value == self.value {
            panic!("Value already exists in the tree!")
        }

        if value <= self.value {
            // go left
            match self.left {
                Some(ref mut left) => left.insert(value),
                None => { self.left = Some(Box::new(Node::new(value))) },
            }
        }
        else {
            // go right
            match self.right {
                Some(ref mut right) => right.insert(value),
                None => { self.right = Some(Box::new(Node::new(value))) },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let tree = BinarySearchTree::<i32>::new();

        assert!(tree.root.is_none());
    }

    #[test]
    fn only_root_node() {
        let mut tree = BinarySearchTree::<i32>::new();
        tree.insert(5);

        assert!(tree.root.is_some());
        
        if let Some(ref root) = tree.root {
            assert_eq!(root.value, 5);
            assert!(root.left.is_none());
            assert!(root.right.is_none());
        }
    }

    #[test]
    fn insert() {
        let mut tree = BinarySearchTree::<i32>::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(6);

        assert!(tree.root.is_some());
        
        if let Some(ref root) = tree.root {
            assert_eq!(root.value, 5);

            assert!(root.left.is_some());

            if let Some(ref left) = root.left {
                assert_eq!(left.value, 3);
                assert!(left.left.is_none());
                assert!(left.right.is_none());
            }

            assert!(root.right.is_some());

            if let Some(ref right) = root.right {
                assert_eq!(right.value, 6);
                assert!(right.left.is_none());
                assert!(right.right.is_none());
            }
        }
    }

    #[test]
    #[should_panic]
    fn insert_duplicate() {
        let mut tree = BinarySearchTree::<i32>::new();
        tree.insert(4);
        tree.insert(3);
        tree.insert(4);
    }
}
