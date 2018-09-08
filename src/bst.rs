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
where
    T: PartialOrd,
{
    pub fn new() -> BinarySearchTree<T> {
        BinarySearchTree { root: None }
    }

    pub fn insert(&mut self, value: T) {
        if self.root.is_none() {
            return self.root = Some(Box::new(Node::new(value)));
        }

        // traverse the tree, and insert
        let mut node = &mut self.root as *mut Option<Box<Node<T>>>;

        unsafe {
            while let Some(ref mut unwrapped) = *node {
                if unwrapped.value == value {
                    // duplicate found, so just swap the values
                    unwrapped.value = value;
                    return;
                }

                if value < unwrapped.value {
                    // go left
                    if unwrapped.left.is_none() {
                        unwrapped.left = Some(Box::new(Node::new(value)));
                        return;
                    }

                    node = &mut unwrapped.left;
                } else {
                    // go right
                    if unwrapped.right.is_none() {
                        unwrapped.right = Some(Box::new(Node::new(value)));
                        return;
                    }

                    node = &mut unwrapped.right;
                }
            }
        }
    }
}

impl<T> Node<T>
where
    T: PartialOrd,
{
    pub fn new(value: T) -> Node<T> {
        Node {
            value: value,
            left: None,
            right: None,
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
    fn insert_duplicate() {
        let mut tree = BinarySearchTree::<i32>::new();
        tree.insert(4);

        assert!(tree.root.is_some());

        if let Some(ref root) = tree.root {
            assert_eq!(root.value, 4);
            assert!(root.left.is_none());
            assert!(root.right.is_none());
        }

        tree.insert(4);

        assert!(tree.root.is_some());

        if let Some(ref root) = tree.root {
            assert_eq!(root.value, 4);
            assert!(root.left.is_none());
            assert!(root.right.is_none());
        }
    }
}
