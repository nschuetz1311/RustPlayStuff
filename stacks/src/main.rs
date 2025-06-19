struct Stack {
    items: Vec<i32>,
}

impl Stack {
    fn new() -> Self {
        Stack {items: Vec::new()}
    }

    // push an element onto the top of the Stack
    fn push(&mut self, value: i32) {
        self.items.push(value);
    }

    // pop an element from the top of the stack
    fn pop(&mut self) -> Option<i32> {
        self.items.pop()
    }

    // displays the top element without removing it
    fn peek(&self) -> Option<&i32> {
        self.items.last()
    }
}


fn main() {

    // A stack is a datastructure that follows tha LIFO principle

    // push and pop
    let mut stack = Stack::new();


    stack.push(1);
    stack.push(5);
    stack.push(8);

    println!("Top of the stack: {:?}", stack.peek().unwrap());

    stack.pop();

    println!("Top of the stack after pop: {:?}", stack.peek().unwrap());
}
