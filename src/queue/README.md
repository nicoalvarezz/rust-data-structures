A **Queue** is a linear data structure that follows the **First In, First Out (FIFO)** principle. This means that elements are inserted at the back (or "rear") and removed from the front, ensuring that the element that has been in the queue the longest is the first to be removed.

## Basic Operations
1. **Enqueue**: Insert an element at the end (rear) of the queue.
2. **Dequeue**: Remove and return the element at the front of the queue.
3. **Front**: Return the front element without removing it.
4. **Rear**: Return the element at the rear without removing it.
5. **IsEmpty**: Check if the queue contains any elements.

Each operation typically has the following time complexities:

- **Enqueue**: O(1)
- **Dequeue**: O(1)
- **Front / Rear**: O(1)
- **IsEmpty**: O(1)

---

## Linked List Queue
In this project, the queue is implemented using a **singly linked list** to allow dynamic resizing. This approach ensures efficient operations by maintaining two pointers:

- **Front pointer**: Points to the first element in the queue, allowing quick access for dequeuing.
- **Rear pointer**: Points to the last element in the queue, allowing quick access for enqueuing.

### Key Properties
- The front pointer helps with efficient O(1) dequeue operations.
- The rear pointer enables O(1) enqueue operations.
- This structure adapts easily to various queue lengths, as there’s no fixed capacity.