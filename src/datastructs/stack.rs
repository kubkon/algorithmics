pub struct Stack<T> {
    top: Option<Box<Node<T>>>,
}

struct Node<T> {
    item: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { top: None }
    }

    pub fn push(&mut self, item: T) {
        let top = self.top.take();
        self.top = Some(Box::new(Node::new(item, top)))
    }

    pub fn pop(&mut self) -> Option<T> {
        let top = self.top.take();

        match top {
            Some(mut top) => {
                self.top = top.next.take();
                Some(top.item)
            }
            None => None,
        }
    }
}

impl<T> Node<T> {
    pub fn new(item: T, next: Option<Box<Node<T>>>) -> Node<T> {
        Node {
            item: item,
            next: next,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn empty() {
        let mut stack: Stack<i32> = Stack::new();

        assert!(stack.top.is_none());
        assert_eq!(stack.pop(), None);
    }

    #[test]
    pub fn push() {
        let mut stack: Stack<i32> = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        let top = &stack.top;
        assert!(top.is_some());
        assert_eq!(top.as_ref().unwrap().item, 3);

        let next = &top.as_ref().unwrap().next;
        assert!(next.is_some());
        assert_eq!(next.as_ref().unwrap().item, 2);

        let next = &next.as_ref().unwrap().next;
        assert!(next.is_some());
        assert_eq!(next.as_ref().unwrap().item, 1);
    }

    #[test]
    pub fn pop() {
        let mut stack: Stack<i32> = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }
}
