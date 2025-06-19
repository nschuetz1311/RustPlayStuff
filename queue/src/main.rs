use std::collections::VecDeque;

struct Queue {
    items: VecDeque<i32>,
}

impl Queue {
    fn new() -> Self {
        Queue {items: VecDeque::new()}
    }

    fn enqueue(&mut self, value: i32) {
        self.items.push_back(value);
    }

    fn dequeue(&mut self) -> Option<i32> {
        self.items.pop_front()
    }

    fn peek(&self) -> Option<&i32>  {
        self.items.front()
    }
}


fn main() {

    let mut queue = Queue::new();

    queue.enqueue(1);
    queue.enqueue(15);
    queue.enqueue(23);
    queue.enqueue(66);

    println!("Front of the queue: {:?}",queue.peek().unwrap());

    queue.dequeue();

    println!("After dequeue, front of the que is {:?}", queue.peek().unwrap());
}
