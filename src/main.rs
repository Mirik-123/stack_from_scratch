/* 
* Size: number of bytes
* Alignment: memory address requirements for the data type
* Layout: rust's allocation system uses both size and alignment to discribe a memory block
* NOTE:: A fixed size block has a predeterminded capacity. What happens when incomming data exceeds it ?
* Reject? Truncate? Separate overflow strategy?
*/
use std::alloc::{alloc, Layout};

#[derive(Debug)]
struct Stack {
    capacity: usize,
    top: isize,
    ptr: *mut i32,
}

impl Stack{
    pub fn new(capacity: usize) -> Self{
        assert!(capacity > 0, "Capacity must be greater than zero");
        let top = -1;
        let layout = Layout::array::<i32>(capacity).expect("Invalid memory layout");
        let ptr : *mut i32 = unsafe{ alloc(layout) as *mut i32};
        if ptr.is_null() {
            std::alloc::handle_alloc_error(layout);
        }
        return  Self{
            capacity,
            top,
            ptr,
        }
    }  

    pub fn push(&mut self, value: i32){
        if self.is_full() {
            panic!("Stack Overflow");
        }
        self.top += 1;
        unsafe{
            self.ptr.add(usize::try_from(self.top).unwrap()).write(value);
        }
    }

    pub fn pop(&mut self) -> i32 {
        if self.is_empty() {
            panic!("Stack is empty");
        }
        self.top -= 1;
        unsafe {
            return self.ptr.add(usize::try_from(self.top).unwrap()).read();
        }
    }

    pub fn peek(&self) -> i32{
        if self.is_empty(){
            panic!("Stack is empty");
        }
        unsafe{
             return self.ptr.add(usize::try_from(self.top).unwrap()).read();
        }
    }

    pub fn is_empty(&self) -> bool{
        return self.top == -1;
    }

    pub fn is_full(&self) -> bool{
        return !self.is_empty() && usize::try_from(self.top).unwrap() == self.capacity;
    }
}

fn main() { 
    const CAPACITY : usize = 20;  // max nummer of elements will be 20
    let mut stack = Stack::new(CAPACITY);

    stack.push(5);
    stack.push(7);
    stack.push(9);
    stack.push(3);
    stack.push(4);

    println!("{:?}", stack);
    println!("Top element: {}", stack.peek());
    println!("Popped: {}", stack.pop());
    println!("Popped: {}", stack.pop());
    println!("Is empty: {}", stack.is_empty());
}
