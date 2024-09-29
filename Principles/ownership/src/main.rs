// Stack - A stack stores values in the order it gets them and removes the values in the opposite order (Last In First Out) - Like a stack of plates.
//       - All Data stored on the stack must have a known fixed size.
// Heap  - Data that without a known size at compile time is stores on the heap. The heap is less organized and when you put something on the heap.
//         You are requesting the space, and that would then return a pointer. Which is the address of that location.

// Pushing onto the stack is faster than allocating on the heap. So searching for data on the stack is also faster than searching on the heap.
// When you pass values into a function or have local variables inside a function, they are all pushed to the stack and then once the function is run they are popped off the stack

// Each value in Rust has a variable called it's owner. There can only be one owner at a time, and when the owner goes out of scope, the value will be dropped, also known as free.
fn main() {
    let _var: u32 = 1; // Created on the Stack
    let mut s = "hello".to_string(); // Created on the Heap
    s.push_str(", world");
}

// var is dropped, s is also dropped after function run