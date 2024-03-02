struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct Stack<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
    size: usize,
}

#[derive(Debug)]
pub struct StackOverflowError;
#[derive(Debug)]
pub struct StackEmptyError;

impl<T> Stack<T> {
    pub fn new(size: usize) -> Self {
        Stack {
            head: None,
            len: 0,
            size,
        }
    }

    pub fn get_len(&self) -> usize {
        self.len
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            Some(node) => Some(&node.value),
            None => None,
        }
    }

    pub fn pop(&mut self) -> Result<T, StackEmptyError> {
        if self.len == 0 {
            return Err(StackEmptyError);
        }
        let head_node = self.head.take().unwrap();
        self.head = head_node.next;
        self.len -= 1;
        Ok(head_node.value)
    }

    pub fn push(&mut self, value: T) -> Result<(), StackOverflowError> {
        if self.len == self.size {
            return Err(StackOverflowError);
        }
        let old_head = self.head.take();
        self.head = Some(Box::new(Node {
            value,
            next: old_head,
        }));
        self.len += 1;
        Ok(())
    }
}
