use std::cmp::Ord;

pub struct BinarySearchTree<T: Ord> {
    root: Option<Box<Node<T>>>,
}

struct Node<T: Ord> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    pub fn new() -> BinarySearchTree<T> {
        BinarySearchTree { root: None }
    }

    pub fn insert(&mut self, value: T) -> bool {
        let mut node = &mut self.root as *mut Option<Box<Node<T>>>;

        unsafe {
            while let Some(ref mut unwrapped) = *node {
                if unwrapped.value == value {
                    unwrapped.value = value;
                    return true;
                }

                if value < unwrapped.value {
                    node = &mut unwrapped.left;
                } else {
                    node = &mut unwrapped.right;
                }
            }

            *node = Some(Box::new(Node::new(value)));
        }

        false
    }

    pub fn find(&self, value: &T) -> Option<&T> {
        let mut node = &self.root;

        loop {
            let unwrapped = node.as_ref()?;

            if unwrapped.value == *value {
                return Some(&unwrapped.value);
            }

            if *value < unwrapped.value {
                node = &unwrapped.left;
            } else {
                node = &unwrapped.right;
            }
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        self.find(value).is_some()
    }
}

impl<T> Node<T>
where
    T: Ord,
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
        let tree: BinarySearchTree<i32> = BinarySearchTree::new();

        assert!(tree.root.is_none());
    }

    #[test]
    fn only_root_node() {
        let mut tree: BinarySearchTree<i32> = BinarySearchTree::new();
        tree.insert(5);

        assert!(tree.root.is_some());

        let root = tree.root.as_ref().unwrap();

        assert_eq!(root.value, 5);
        assert!(root.left.is_none());
        assert!(root.right.is_none());
    }

    #[test]
    fn insert() {
        let mut tree: BinarySearchTree<i32> = BinarySearchTree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(6);

        assert!(tree.root.is_some());

        let root = tree.root.as_ref().unwrap();

        assert_eq!(root.value, 5);
        assert!(root.left.is_some());

        let left = root.left.as_ref().unwrap();

        assert_eq!(left.value, 3);
        assert!(left.left.is_none());
        assert!(left.right.is_none());

        assert!(root.right.is_some());

        let right = root.right.as_ref().unwrap();

        assert_eq!(right.value, 6);
        assert!(right.left.is_none());
        assert!(right.right.is_none());
    }

    use std::cmp::Ordering;

    #[derive(Debug)]
    struct Test(i32, i32);

    impl PartialEq for Test {
        fn eq(&self, other: &Test) -> bool {
            self.0 == other.0
        }
    }

    impl PartialOrd for Test {
        fn partial_cmp(&self, other: &Test) -> Option<Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }

    impl Ord for Test {
        fn cmp(&self, other: &Test) -> Ordering {
            self.0.cmp(&other.0)
        }
    }

    impl Eq for Test {}

    #[test]
    fn insert_duplicate() {
        let mut tree: BinarySearchTree<Test> = BinarySearchTree::new();
        tree.insert(Test(4, 0));

        {
            assert!(tree.root.is_some());

            let root = tree.root.as_ref().unwrap();

            assert_eq!(root.value.0, 4);
            assert_eq!(root.value.1, 0);

            assert!(root.left.is_none());
            assert!(root.right.is_none());
        }

        tree.insert(Test(4, 1));

        {
            assert!(tree.root.is_some());

            let root = tree.root.as_ref().unwrap();

            assert_eq!(root.value.0, 4);
            assert_eq!(root.value.1, 1);

            assert!(root.left.is_none());
            assert!(root.right.is_none());
        }
    }

    #[test]
    fn find() {
        let mut tree: BinarySearchTree<i32> = BinarySearchTree::new();
        tree.insert(3);
        tree.insert(2);
        tree.insert(1);

        assert!(tree.find(&3).is_some());
        assert_eq!(tree.find(&3).unwrap(), &3);

        assert!(tree.find(&2).is_some());
        assert_eq!(tree.find(&2).unwrap(), &2);

        assert!(tree.find(&1).is_some());
        assert_eq!(tree.find(&1).unwrap(), &1);

        assert!(tree.find(&11).is_none());

        tree.insert(11);

        assert!(tree.find(&11).is_some());
        assert_eq!(tree.find(&11).unwrap(), &11);
    }
}
