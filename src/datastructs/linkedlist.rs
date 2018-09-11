pub struct LinkedList<T> {
    front: Option<Box<Node<T>>>,
}

struct Node<T> {
    item: T,
    next: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { front: None }
    }

    pub fn push_back(&mut self, item: T) {
        let mut node = &mut self.front as *mut Option<Box<Node<T>>>;

        unsafe {
            while let Some(ref mut n) = *node {
                node = &mut n.next;
            }

            *node = Some(Box::new(Node::new(item)))
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let front = self.front.take();
        if let Some(mut v) = front {
            self.front = v.next.take();
            return Some(v.item);
        }
        None
    }

    pub fn iter(&self) -> LinkedListIter<T> {
        LinkedListIter { node: &self.front }
    }
}

impl<T> Node<T> {
    pub fn new(item: T) -> Self {
        Node {
            item: item,
            next: None,
        }
    }
}

pub struct LinkedListIter<'a, T: 'a> {
    node: &'a Option<Box<Node<T>>>,
}

impl<'a, T: 'a> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        match self.node {
            Some(ref n) => {
                self.node = &n.next;
                Some(&n.item)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let list: LinkedList<i32> = LinkedList::new();

        assert!(list.front.is_none());
    }

    #[test]
    fn push_back() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_back(3);
        list.push_back(5);
        list.push_back(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&3));
    }
}
