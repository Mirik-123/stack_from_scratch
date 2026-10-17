# Stack Implementation in Rust

This is a Rust implementation of a stack data structure using manual memory management with `std::alloc`.

## Overview

The stack stores integer values (`i32`) and provides the following operations:
- `push(value)` - Add an element to the top of the stack
- `pop()` - Remove and return the top element
- `peek()` - Return the top element without removing it
- `is_empty()` - Check if the stack is empty
- `is_full()` - Check if the stack is full

## Features

- Manual memory allocation using `std::alloc::alloc`
- Fixed-size stack with capacity defined at creation
- Panic on stack overflow (push to full stack) and underflow (pop from empty stack)
