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

    pub fn push(&mut self, value: T) {
        if self.front.is_none() {
            return self.front = Some(Box::new(Node::new(value)));
        }

        let mut node = &mut self.front;
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

        assert!(list.front.is_some());
        assert_eq!(list.front.unwrap().item, 3);
    }
}
