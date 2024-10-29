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
    pub fn new() -> Self {
        Queue {
            front: None,
            back: std::ptr::null_mut(),
            size: 0
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

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

    pub fn peek(&self) -> Option<&T> {
        self.front.as_deref().map(|node| &node.value)
    }
}