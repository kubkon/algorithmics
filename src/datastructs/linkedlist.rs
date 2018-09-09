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

    pub fn push(&mut self, item: T) {
        let mut node = &mut self.front as *mut Option<Box<Node<T>>>;

        unsafe {
            loop {
                match *node {
                    Some(ref mut n) => node = &mut n.next,
                    None => {
                        return *node = Some(Box::new(Node::new(item)));
                    }
                }
            }
        }
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
            Some(ref n) => Some(&n.item),
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
    fn push() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push(3);
        list.push(5);
        list.push(3);
    }
}
