struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>
}

struct Queue<T> {
    front: Option<Box<Node<T>>>,
    back: *mut Node<T>, // Raw pointer for the back (needed for efficient enqueue)
    size: usize
}

impl<T> Queue<T> {

    /// Constructs a new, empty `Queue<T>`.
    ///
    /// ## Examples
    ///
    /// ```
    /// #![allow(unused_mut)]
    /// let mut q: Queue<i32> = Queue::new();
    /// ```
    pub fn new() -> Self {
        Queue {
            front: None,
            back: std::ptr::null_mut(),
            size: 0
        }
    }

    /// Checks if th queue has no elements
    ///
    /// # Examples
    ///
    /// ```
    /// let mut q = Queue::new();
    /// assert!(q.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Adds an element at the end of the queue
    ///
    /// ## Examples
    ///
    /// ```
    /// let mut q = Queue::new();
    /// q.enqueue(10);
    /// ```
    pub fn enqueue(&mut self, value: T) {
        let new_node = Box::new(Node {
            value, 
            next: None
        });

        let raw_node = Box::into_raw(new_node); // Convert to a raw pointer

        if self.is_empty() {
            // First element: both from and back point to new node
            unsafe { self.front = Some(Box::from_raw(raw_node)); }
            self.back = raw_node;
        } else {
            // Update current back to point to new node
            unsafe {
                (*self.back).next = Some(Box::from_raw(raw_node));
            }
            self.back = raw_node;
        }
        
        self.size += 1;
    }

    /// Removes and returns the element at the front of the queue
    /// 
    /// ## Examples
    /// 
    /// ```
    /// let mut q = Queue::new()
    /// q.enqueue(10)
    /// 
    /// assert_eq!(queue.dequeue(), Some(10));
    /// assert!(queue.is_empty());
    /// ```
    pub fn dequeue(&mut self) -> Option<T> {
        self.front.take().map(|old_front| {
            self.front = old_front.next;
            
            if self.front.is_none() {
                self.back = std::ptr::null_mut();
            }
            
            self.size -= 1;
            old_front.value
        })
    }

    /// Returns the front element of the queue without removing it
    /// 
    /// ## Examples
    /// 
    /// ```
    /// let mut q = Queue::new()
    /// q.enqueue(10)
    /// 
    /// assert_eq!(queue.front(), Some(&5));
    /// ```
    pub fn front(&self) -> Option<&T> {
        self.front.as_deref().map(|node| &node.value)
    }


    /// Returns the element at the back of the queue without removing it 
    /// 
    /// ## Examples
    /// 
    /// ```
    /// let mut q = Queue::new()
    /// q.enqueue(10)
    /// 
    /// assert_eq!(queue.rear(), Some(&30));
    /// ```
    pub fn rear(&self) -> Option<&T> {
        unsafe { self.back.as_ref().map(|node| &node.value) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enqueue() {
        let mut queue = Queue::new();
        queue.enqueue(10);
        queue.enqueue(20);
        queue.enqueue(30);

        assert_eq!(queue.front(), Some(&10));
        assert_eq!(queue.size, 3);
    }

    #[test]
    fn test_dequeue() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), None); // Queue should be empty now
        assert!(queue.is_empty());
    }

    #[test]
    fn test_front() {
        let mut queue = Queue::new();
        queue.enqueue(5);
        queue.enqueue(15);

        // Peek at the top element without removing it
        assert_eq!(queue.front(), Some(&5));

        // Peek again to ensure it hasn't changed
        assert_eq!(queue.front(), Some(&5));

        // Pop it and then check peek again
        queue.dequeue();
        assert_eq!(queue.front(), Some(&15));
        assert_eq!(queue.size, 1)
    }

    #[test]
    fn test_is_empty() {
        let mut queue = Queue::new();

        assert!(queue.is_empty());
        queue.enqueue(5);
        assert!(!queue.is_empty());
        queue.dequeue();
        assert!(queue.is_empty())
    }

    #[test]
    fn test_rear() {
        let mut queue = Queue::new();
        queue.enqueue(10);
        queue.enqueue(20);
        queue.enqueue(30);

        assert_eq!(queue.rear(), Some(&30));
        assert_eq!(queue.size, 3);
    }
}
