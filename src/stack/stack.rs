struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

struct Stack<T> {
    head: Option<Box<Node<T>>>,
    capacity: usize,
    size: usize
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { 
            head: None,
            capacity: usize::MAX,
            size: 0   
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Stack {
            head: None,
            capacity,
            size: 0
        }
    }

    pub fn push(&mut self, value: T) {
        if Self::is_full(&self) {
            panic!("Stack overflow!")
        }

        let new_node = Box::new(Node {
            value,
            next: self.head.take(), // Move onwnership of current head to the new node's next
        });
        self.head = Some(new_node);
        self.size += 1
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next; // Move head to the next node
            self.size -= 1;
            node.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_deref().map(|node| &node.value)
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size >= self.capacity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
        let mut stack = Stack::new();
        stack.push(10);
        stack.push(20);
        stack.push(30);

        // After pushing, the top element should be 30
        assert_eq!(stack.peek(), Some(&30));
        assert_eq!(stack.size, 3)
    }

    #[test]
    fn test_pop() {
        let mut stack = Stack::new();
        stack.push(10); 
        stack.push(20);
        stack.push(30);

        assert_eq!(stack.pop(), Some(30));
        assert_eq!(stack.pop(), Some(20));
        assert_eq!(stack.pop(), Some(10));
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.size, 0)
    }

    #[test]
    fn test_peek() {
        let mut stack = Stack::new();
        stack.push(5);
        stack.push(15);

        // Peek at the top element without removing it
        assert_eq!(stack.peek(), Some(&15));

        // Peek again to ensure it hasn't changed
        assert_eq!(stack.peek(), Some(&15));

        // Pop it and then check peek again
        stack.pop();
        assert_eq!(stack.peek(), Some(&5));
        assert_eq!(stack.size, 1)
    }

    #[test]
    fn test_empty_stack() {
        let mut stack: Stack<i32> = Stack::new();

        // Pop and peek on empty stack shoul return None
        assert_eq!(stack.peek(), None);
        assert_eq!(stack.pop(), None);
        assert!(stack.is_empty())
    }

    #[test]
    fn test_is_empty() {
        let mut stask: Stack<i32> = Stack::new();
        
        // Stack is empty when first created
        assert!(stask.is_empty());

        // Stack is not empty when element added
        stask.push(10);
        assert_eq!(stask.is_empty(), false);

        // Stack is empty when all elemets are poped out
        stask.pop();
        assert!(stask.is_empty());
    }

    #[test]
    fn test_with_capacity() {
        let stack: Stack<i32> = Stack::with_capacity(3);
        assert_eq!(stack.capacity, 3)
    
    }

    #[test]
    #[should_panic(expected = "Stack overflow!")]
    fn test_with_default_capacity_exceeded() {
        let mut stack = Stack::with_capacity(2);
        assert_eq!(stack.capacity, 2);
        for i in 0..3 {
            stack.push(i)
        }
    }

    #[test]
    fn test_is_full() {
        let mut stack = Stack::with_capacity(1);

        stack.push(10);
        assert!(stack.is_full());
    }
}